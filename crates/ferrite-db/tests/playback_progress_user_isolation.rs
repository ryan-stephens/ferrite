use ferrite_db::movie_repo::{self, MediaQuery};
use ferrite_db::{create_pool, progress_repo, tv_repo};
use sqlx::SqlitePool;
use uuid::Uuid;

async fn new_test_pool() -> SqlitePool {
    let db_path = std::env::temp_dir().join(format!("ferrite-db-test-{}.sqlite", Uuid::new_v4()));
    create_pool(&db_path, 4)
        .await
        .expect("failed to create test db pool")
}

async fn seed_library_and_users(pool: &SqlitePool) -> (String, String, String) {
    let library_id = Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO libraries (id, name, path, library_type) VALUES (?, 'Test Library', '/tmp', 'movie')",
    )
    .bind(&library_id)
    .execute(pool)
    .await
    .expect("failed to insert library");

    let user_a = Uuid::new_v4().to_string();
    let user_b = Uuid::new_v4().to_string();

    sqlx::query(
        "INSERT INTO users (id, username, password_hash) VALUES (?, ?, '$2b$12$placeholder')",
    )
    .bind(&user_a)
    .bind(format!("user-a-{}", &user_a[..8]))
    .execute(pool)
    .await
    .expect("failed to insert user A");

    sqlx::query(
        "INSERT INTO users (id, username, password_hash) VALUES (?, ?, '$2b$12$placeholder')",
    )
    .bind(&user_b)
    .bind(format!("user-b-{}", &user_b[..8]))
    .execute(pool)
    .await
    .expect("failed to insert user B");

    (library_id, user_a, user_b)
}

async fn seed_movie_media(pool: &SqlitePool, library_id: &str) -> String {
    let media_id = Uuid::new_v4().to_string();
    let file_path = format!("/tmp/{}.mkv", media_id);

    sqlx::query(
        "INSERT INTO media_items (id, library_id, media_type, file_path, file_size, title) \
         VALUES (?, ?, 'movie', ?, 1024, 'Test Movie')",
    )
    .bind(&media_id)
    .bind(library_id)
    .bind(&file_path)
    .execute(pool)
    .await
    .expect("failed to insert media item");

    sqlx::query("INSERT INTO movies (media_item_id, title, year) VALUES (?, 'Test Movie', 2024)")
        .bind(&media_id)
        .execute(pool)
        .await
        .expect("failed to insert movie row");

    media_id
}

async fn seed_episode_media(pool: &SqlitePool, library_id: &str) -> (String, String) {
    let show_id = Uuid::new_v4().to_string();
    let season_id = Uuid::new_v4().to_string();
    let media_id = Uuid::new_v4().to_string();
    let file_path = format!("/tmp/{}.mkv", media_id);

    sqlx::query("INSERT INTO tv_shows (id, library_id, title) VALUES (?, ?, 'Isolation Show')")
        .bind(&show_id)
        .bind(library_id)
        .execute(pool)
        .await
        .expect("failed to insert tv show");

    sqlx::query("INSERT INTO seasons (id, tv_show_id, season_number) VALUES (?, ?, 1)")
        .bind(&season_id)
        .bind(&show_id)
        .execute(pool)
        .await
        .expect("failed to insert season");

    sqlx::query(
        "INSERT INTO media_items (id, library_id, media_type, file_path, file_size, title) \
         VALUES (?, ?, 'episode', ?, 2048, 'Episode 1')",
    )
    .bind(&media_id)
    .bind(library_id)
    .bind(&file_path)
    .execute(pool)
    .await
    .expect("failed to insert episode media item");

    sqlx::query("INSERT INTO episodes (media_item_id, season_id, episode_number) VALUES (?, ?, 1)")
        .bind(&media_id)
        .bind(&season_id)
        .execute(pool)
        .await
        .expect("failed to insert episode row");

    (season_id, media_id)
}

#[tokio::test]
async fn progress_upsert_allows_distinct_rows_per_user_for_same_media() {
    let pool = new_test_pool().await;
    let (library_id, user_a, user_b) = seed_library_and_users(&pool).await;
    let media_id = seed_movie_media(&pool, &library_id).await;

    progress_repo::upsert_progress(&pool, &media_id, Some(&user_a), 11_000)
        .await
        .expect("failed to upsert progress for user A");
    progress_repo::upsert_progress(&pool, &media_id, Some(&user_b), 22_000)
        .await
        .expect("failed to upsert progress for user B");

    let a = progress_repo::get_progress(&pool, &media_id, Some(&user_a))
        .await
        .expect("failed to fetch user A progress")
        .expect("missing progress row for user A");
    let b = progress_repo::get_progress(&pool, &media_id, Some(&user_b))
        .await
        .expect("failed to fetch user B progress")
        .expect("missing progress row for user B");

    assert_eq!(a.position_ms, 11_000);
    assert_eq!(b.position_ms, 22_000);

    let count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM playback_progress WHERE media_item_id = ? AND user_id IS NOT NULL",
    )
    .bind(&media_id)
    .fetch_one(&pool)
    .await
    .expect("failed to count playback progress rows");

    assert_eq!(count.0, 2);
}

#[tokio::test]
async fn movie_queries_join_progress_for_requesting_user_only() {
    let pool = new_test_pool().await;
    let (library_id, user_a, user_b) = seed_library_and_users(&pool).await;
    let media_id = seed_movie_media(&pool, &library_id).await;

    progress_repo::upsert_progress(&pool, &media_id, Some(&user_a), 30_000)
        .await
        .expect("failed to upsert user A movie progress");
    progress_repo::upsert_progress(&pool, &media_id, Some(&user_b), 45_000)
        .await
        .expect("failed to upsert user B movie progress");

    let row_a = movie_repo::get_movie_with_media(&pool, &media_id, Some(&user_a))
        .await
        .expect("movie query failed for user A")
        .expect("missing movie row for user A");
    let row_b = movie_repo::get_movie_with_media(&pool, &media_id, Some(&user_b))
        .await
        .expect("movie query failed for user B")
        .expect("missing movie row for user B");

    assert_eq!(row_a.position_ms, Some(30_000));
    assert_eq!(row_b.position_ms, Some(45_000));

    let query = MediaQuery {
        library_id: Some(&library_id),
        search: None,
        genre: None,
        sort_by: None,
        sort_dir: None,
        page: 1,
        per_page: 20,
    };

    let listed = movie_repo::list_movies_with_media(&pool, &query, Some(&user_a))
        .await
        .expect("failed to list movies");

    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].position_ms, Some(30_000));
}

#[tokio::test]
async fn episode_queries_join_progress_for_requesting_user_only() {
    let pool = new_test_pool().await;
    let (library_id, user_a, user_b) = seed_library_and_users(&pool).await;
    let (season_id, episode_media_id) = seed_episode_media(&pool, &library_id).await;

    progress_repo::upsert_progress(&pool, &episode_media_id, Some(&user_a), 5_000)
        .await
        .expect("failed to upsert episode progress for user A");
    progress_repo::upsert_progress(&pool, &episode_media_id, Some(&user_b), 12_000)
        .await
        .expect("failed to upsert episode progress for user B");

    let rows_a = tv_repo::list_episodes(&pool, &season_id, Some(&user_a))
        .await
        .expect("failed to list episodes for user A");
    let rows_b = tv_repo::list_episodes(&pool, &season_id, Some(&user_b))
        .await
        .expect("failed to list episodes for user B");

    assert_eq!(rows_a.len(), 1);
    assert_eq!(rows_b.len(), 1);
    assert_eq!(rows_a[0].position_ms, Some(5_000));
    assert_eq!(rows_b[0].position_ms, Some(12_000));
}
