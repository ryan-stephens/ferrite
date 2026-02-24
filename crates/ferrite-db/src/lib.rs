pub mod chapter_repo;
pub mod collection_repo;
pub mod keyframe_repo;
pub mod library_repo;
pub mod media_repo;
pub mod movie_repo;
pub mod preference_repo;
pub mod progress_repo;
pub mod stream_repo;
pub mod subtitle_repo;
pub mod tv_repo;
pub mod user_repo;
pub mod webhook_repo;

use anyhow::Result;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;
use std::path::Path;
use std::str::FromStr;
use tracing::info;

#[derive(Clone, Debug)]
pub struct Database {
    pub read: SqlitePool,
    pub write: SqlitePool,
}

/// Create and initialize the SQLite connection pools.
/// `max_connections` controls the reader pool size (default 16 in config).
/// The writer pool is always size 1 to serialize writes and prevent `database is locked`.
pub async fn create_pools(db_path: &Path, max_connections: u32) -> Result<Database> {
    let db_url = format!("sqlite:{}?mode=rwc", db_path.display());

    let options = SqliteConnectOptions::from_str(&db_url)?
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        .create_if_missing(true)
        // NORMAL is safe with WAL mode and significantly faster than FULL for writes
        .pragma("synchronous", "NORMAL")
        // 20MB page cache (negative = KiB) — reduces disk reads for repeated queries
        .pragma("cache_size", "-20000")
        // Wait up to 5s for write locks instead of failing immediately
        .pragma("busy_timeout", "5000")
        // 64MB WAL file limit — prevents unbounded WAL growth
        .pragma("journal_size_limit", "67108864")
        // Store temp tables in memory for faster intermediate query results
        .pragma("temp_store", "MEMORY")
        // Enable foreign key enforcement
        .pragma("foreign_keys", "ON")
        // 30GB memory-mapped I/O (per optimization plan)
        .pragma("mmap_size", "30000000000");

    let read_pool = SqlitePoolOptions::new()
        .max_connections(max_connections)
        .connect_with(options.clone())
        .await?;

    let write_pool = SqlitePoolOptions::new()
        .max_connections(1) // STRICTLY 1 connection for writing
        .connect_with(options)
        .await?;

    info!("Database connected at {}", db_path.display());

    // Run migrations using the write pool
    run_migrations(&write_pool).await?;

    Ok(Database {
        read: read_pool,
        write: write_pool,
    })
}

/// Run SQL migrations from the migrations/ directory.
/// Uses sqlx::migrate!() which embeds migration files at compile time.
/// Tracks which migrations have been applied in a `_sqlx_migrations` table,
/// so new migrations are applied automatically on startup.
async fn run_migrations(pool: &SqlitePool) -> Result<()> {
    sqlx::migrate!("../../migrations").run(pool).await?;
    info!("Database migrations applied");
    Ok(())
}
