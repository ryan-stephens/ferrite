use ferrite_db::create_pool;
use ferrite_db::movie_repo::{self, MediaQuery};
use sqlx::SqlitePool;
use std::time::{Duration, Instant};
use uuid::Uuid;

async fn new_test_pool() -> SqlitePool {
    let db_path =
        std::env::temp_dir().join(format!("ferrite-db-fts-test-{}.sqlite", Uuid::new_v4()));
    create_pool(&db_path, 4)
        .await
        .expect("failed to create test db pool")
}

async fn seed_library(pool: &SqlitePool) -> String {
    let library_id = Uuid::new_v4().to_string();

    sqlx::query(
        "INSERT INTO libraries (id, name, path, library_type) VALUES (?, 'FTS Library', '/tmp', 'movie')",
    )
    .bind(&library_id)
    .execute(pool)
    .await
    .expect("failed to insert library");

    library_id
}

async fn seed_movie(pool: &SqlitePool, library_id: &str, title: &str, overview: &str) -> String {
    let media_id = Uuid::new_v4().to_string();

    sqlx::query(
        "INSERT INTO media_items (id, library_id, media_type, file_path, file_size, title) \
         VALUES (?, ?, 'movie', ?, 1234, ?)",
    )
    .bind(&media_id)
    .bind(library_id)
    .bind(format!("/tmp/{}.mkv", media_id))
    .bind(title)
    .execute(pool)
    .await
    .expect("failed to insert media item");

    sqlx::query(
        "INSERT INTO movies (media_item_id, title, overview, genres) VALUES (?, ?, ?, 'Drama')",
    )
    .bind(&media_id)
    .bind(title)
    .bind(overview)
    .execute(pool)
    .await
    .expect("failed to insert movie metadata");

    media_id
}

fn query_for<'a>(library_id: &'a str, search: &'a str) -> MediaQuery<'a> {
    MediaQuery {
        library_id: Some(library_id),
        search: Some(search),
        genre: None,
        sort_by: None,
        sort_dir: None,
        page: 1,
        per_page: 20,
    }
}

#[tokio::test]
async fn list_media_uses_fts_for_search_queries() {
    let pool = new_test_pool().await;
    let library_id = seed_library(&pool).await;
    let media_id = seed_movie(
        &pool,
        &library_id,
        "Ocean Depth",
        "A story about abyssal life and bioluminescence",
    )
    .await;

    let query = query_for(&library_id, "bioluminescence");
    let rows = movie_repo::list_movies_with_media(&pool, &query, None)
        .await
        .expect("list query failed");
    let total = movie_repo::count_movies_with_media(&pool, &query)
        .await
        .expect("count query failed");

    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].id, media_id);
    assert_eq!(total, 1);
}

#[tokio::test]
async fn list_media_default_sort_matches_bm25_relevance() {
    let pool = new_test_pool().await;
    let library_id = seed_library(&pool).await;
    let top = seed_movie(
        &pool,
        &library_id,
        "Ocean Abyss",
        "A focused documentary on abyss ecosystems",
    )
    .await;
    let _other = seed_movie(
        &pool,
        &library_id,
        "Ocean Expedition",
        "An ocean journey that eventually descends into an abyss",
    )
    .await;

    let fts_query = "ocean* AND abyss*";
    let expected_top: (String,) = sqlx::query_as(
        r#"
        SELECT media_item_id
        FROM media_fts
        WHERE media_fts MATCH ?
        ORDER BY bm25(media_fts) ASC
        LIMIT 1
        "#,
    )
    .bind(fts_query)
    .fetch_one(&pool)
    .await
    .expect("failed to compute expected bm25 ordering");

    let query = query_for(&library_id, "ocean abyss");
    let rows = movie_repo::list_movies_with_media(&pool, &query, None)
        .await
        .expect("list query failed");

    assert!(rows.len() >= 2);
    assert_eq!(expected_top.0, top);
    assert_eq!(rows[0].id, expected_top.0);
}

#[tokio::test]
async fn list_media_search_completes_within_reasonable_time_budget() {
    let pool = new_test_pool().await;
    let library_id = seed_library(&pool).await;

    for i in 0..400 {
        let title = format!("Latency Seed Movie {}", i);
        let overview = if i % 25 == 0 {
            format!("Contains benchmarking token {}", i)
        } else {
            "Background overview text".to_string()
        };
        let _ = seed_movie(&pool, &library_id, &title, &overview).await;
    }

    let query = query_for(&library_id, "benchmarking token");
    let started = Instant::now();

    let rows = movie_repo::list_movies_with_media(&pool, &query, None)
        .await
        .expect("list query failed");
    let total = movie_repo::count_movies_with_media(&pool, &query)
        .await
        .expect("count query failed");

    let elapsed = started.elapsed();

    assert!(!rows.is_empty());
    assert!(total > 0);
    assert!(
        elapsed < Duration::from_secs(3),
        "search query exceeded latency budget: {:?}",
        elapsed
    );
}

#[tokio::test]
async fn list_media_falls_back_to_like_when_fts_is_missing() {
    let pool = new_test_pool().await;
    let library_id = seed_library(&pool).await;
    let media_id = seed_movie(
        &pool,
        &library_id,
        "Fallback Search",
        "FTS table intentionally removed",
    )
    .await;

    sqlx::query("DROP TABLE media_fts")
        .execute(&pool)
        .await
        .expect("failed to drop media_fts for fallback test");

    let query = query_for(&library_id, "Fallback");
    let rows = movie_repo::list_movies_with_media(&pool, &query, None)
        .await
        .expect("fallback list query failed");
    let total = movie_repo::count_movies_with_media(&pool, &query)
        .await
        .expect("fallback count query failed");

    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].id, media_id);
    assert_eq!(total, 1);
}
