pub mod chapter_repo;
pub mod collection_repo;
pub mod preference_repo;
pub mod media_repo;
pub mod library_repo;
pub mod movie_repo;
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

/// Create and initialize the SQLite connection pool.
/// `max_connections` controls the pool size (default 16 in config).
pub async fn create_pool(db_path: &Path, max_connections: u32) -> Result<SqlitePool> {
    let db_url = format!("sqlite:{}?mode=rwc", db_path.display());
    let options = SqliteConnectOptions::from_str(&db_url)?
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        .create_if_missing(true)
        // NORMAL is safe with WAL mode and significantly faster than FULL for writes
        .pragma("synchronous", "NORMAL")
        // 20MB page cache (negative = KiB) — reduces disk reads for repeated queries
        .pragma("cache_size", "-20000")
        // Wait up to 30s for write locks — scans can hold locks for 10s+ under
        // concurrent enrichment + subtitle extraction load
        .pragma("busy_timeout", "30000")
        // 64MB WAL file limit — prevents unbounded WAL growth
        .pragma("journal_size_limit", "67108864")
        // Store temp tables in memory for faster intermediate query results
        .pragma("temp_store", "MEMORY")
        // Enable foreign key enforcement
        .pragma("foreign_keys", "ON")
        // 256MB memory-mapped I/O — avoids read() syscalls for repeated page reads,
        // giving 10-30% faster read throughput on large databases
        .pragma("mmap_size", "268435456");

    let pool = SqlitePoolOptions::new()
        .max_connections(max_connections)
        .connect_with(options)
        .await?;

    info!("Database connected at {}", db_path.display());
    run_migrations(&pool).await?;
    Ok(pool)
}

/// Run SQL migrations from the migrations/ directory.
/// Uses sqlx::migrate!() which embeds migration files at compile time.
/// Tracks which migrations have been applied in a `_sqlx_migrations` table,
/// so new migrations are applied automatically on startup.
async fn run_migrations(pool: &SqlitePool) -> Result<()> {
    // Pre-migration fixup: SQLite doesn't support ADD COLUMN IF NOT EXISTS.
    // If a previous run already added the column but the migration wasn't
    // recorded (e.g. dev environment), we need to handle this gracefully.
    fix_duplicate_column_migrations(pool).await;

    sqlx::migrate!("../../migrations")
        .run(pool)
        .await?;
    info!("Database migrations applied");
    Ok(())
}

/// Check for columns that already exist but whose migrations haven't been
/// recorded yet. For each such case, manually insert the migration record
/// so SQLx skips the ALTER TABLE that would otherwise fail.
async fn fix_duplicate_column_migrations(pool: &SqlitePool) {
    // Only run if the _sqlx_migrations table exists (i.e. not a fresh DB)
    let has_table: bool = sqlx::query_scalar(
        "SELECT COUNT(*) > 0 FROM sqlite_master WHERE type='table' AND name='_sqlx_migrations'"
    )
    .fetch_one(pool)
    .await
    .unwrap_or(false);

    if !has_table {
        return;
    }

    // Migration 012: ALTER TABLE tv_shows ADD COLUMN normalized_title
    let col_exists: bool = sqlx::query_scalar(
        "SELECT COUNT(*) > 0 FROM pragma_table_info('tv_shows') WHERE name='normalized_title'"
    )
    .fetch_one(pool)
    .await
    .unwrap_or(false);

    if !col_exists {
        return;
    }

    let migration_recorded: bool = sqlx::query_scalar(
        "SELECT COUNT(*) > 0 FROM _sqlx_migrations WHERE version = 12"
    )
    .fetch_one(pool)
    .await
    .unwrap_or(false);

    if migration_recorded {
        return;
    }

    // Column exists but migration not recorded — insert the record so SQLx
    // skips this migration instead of failing on duplicate column.
    info!("Pre-migration fixup: normalized_title column already exists, marking migration 012 as applied");
    let migrator = sqlx::migrate!("../../migrations");
    if let Some(m) = migrator.iter().find(|m| m.version == 12) {
        let _ = sqlx::query(
            "INSERT INTO _sqlx_migrations (version, description, installed_on, success, checksum, execution_time) \
             VALUES (?, ?, CURRENT_TIMESTAMP, TRUE, ?, 0)"
        )
        .bind(m.version)
        .bind(&*m.description)
        .bind(&*m.checksum)
        .execute(pool)
        .await;
    }
}
