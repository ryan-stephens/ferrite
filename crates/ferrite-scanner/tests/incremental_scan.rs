use ferrite_db::create_pools;
use ferrite_scanner::scan_library_incremental;
use sqlx::SqlitePool;
use std::path::Path;
use tokio::fs;
use uuid::Uuid;

async fn new_test_pool() -> SqlitePool {
    let db_path =
        std::env::temp_dir().join(format!("ferrite-scanner-test-{}.sqlite", Uuid::new_v4()));
    let pools = create_pools(&db_path, 4)
        .await
        .expect("failed to create test db pool");
    pools.read
}

async fn seed_library(pool: &SqlitePool, library_path: &Path) -> String {
    let library_id = Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO libraries (id, name, path, library_type) VALUES (?, 'Incremental Test', ?, 'movie')",
    )
    .bind(&library_id)
    .bind(library_path.to_string_lossy().to_string())
    .execute(pool)
    .await
    .expect("failed to insert library");
    library_id
}

async fn insert_media_item(pool: &SqlitePool, library_id: &str, file_path: &Path, title: &str) {
    let media_id = Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO media_items (id, library_id, media_type, file_path, file_size, title) \
         VALUES (?, ?, 'movie', ?, 1234, ?)",
    )
    .bind(media_id)
    .bind(library_id)
    .bind(file_path.to_string_lossy().to_string())
    .bind(title)
    .execute(pool)
    .await
    .expect("failed to insert media item");
}

#[tokio::test]
async fn incremental_scan_removes_media_for_deleted_directory_prefix() {
    let pool = new_test_pool().await;

    let library_root = std::env::temp_dir().join(format!("ferrite-lib-{}", Uuid::new_v4()));
    fs::create_dir_all(&library_root)
        .await
        .expect("failed to create library root");

    let removed_dir = library_root.join("removed_show");
    let keep_dir = library_root.join("keep_show");
    fs::create_dir_all(removed_dir.join("nested"))
        .await
        .expect("failed to create removed dir");
    fs::create_dir_all(&keep_dir)
        .await
        .expect("failed to create keep dir");

    let library_id = seed_library(&pool, &library_root).await;

    insert_media_item(
        &pool,
        &library_id,
        &removed_dir.join("episode1.mkv"),
        "Removed Episode 1",
    )
    .await;
    insert_media_item(
        &pool,
        &library_id,
        &removed_dir.join("nested").join("episode2.mkv"),
        "Removed Episode 2",
    )
    .await;
    insert_media_item(
        &pool,
        &library_id,
        &keep_dir.join("keep.mkv"),
        "Keep Episode",
    )
    .await;

    fs::remove_dir_all(&removed_dir)
        .await
        .expect("failed to remove directory before incremental scan");

    let touched = scan_library_incremental(
        &pool,
        &library_id,
        "missing-ffprobe",
        "missing-ffmpeg",
        2,
        &library_root.join("subtitle-cache"),
        std::slice::from_ref(&removed_dir),
    )
    .await
    .expect("incremental scan failed");

    assert_eq!(touched, 2);

    let remaining: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM media_items WHERE library_id = ?")
        .bind(&library_id)
        .fetch_one(&pool)
        .await
        .expect("failed to count remaining media rows");

    assert_eq!(remaining.0, 1);

    let _ = fs::remove_dir_all(&library_root).await;
}

#[tokio::test]
async fn incremental_scan_indexes_media_for_changed_subdirectory_events() {
    let pool = new_test_pool().await;

    let library_root = std::env::temp_dir().join(format!("ferrite-lib-{}", Uuid::new_v4()));
    let incoming_dir = library_root.join("incoming");
    fs::create_dir_all(&incoming_dir)
        .await
        .expect("failed to create incoming dir");
    fs::write(incoming_dir.join("new_file.mkv"), b"test")
        .await
        .expect("failed to create sample media file");

    let library_id = seed_library(&pool, &library_root).await;

    let touched = scan_library_incremental(
        &pool,
        &library_id,
        "missing-ffprobe",
        "missing-ffmpeg",
        2,
        &library_root.join("subtitle-cache"),
        std::slice::from_ref(&incoming_dir),
    )
    .await
    .expect("incremental scan failed");

    assert_eq!(touched, 1);

    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM media_items WHERE library_id = ?")
        .bind(&library_id)
        .fetch_one(&pool)
        .await
        .expect("failed to count media rows");

    assert_eq!(count.0, 1);

    let _ = fs::remove_dir_all(&library_root).await;
}
