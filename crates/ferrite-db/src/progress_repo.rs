use anyhow::Result;
use sqlx::SqlitePool;
use uuid::Uuid;

/// A row from the playback_progress table.
#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
pub struct ProgressRow {
    pub id: String,
    pub media_item_id: String,
    pub user_id: Option<String>,
    pub position_ms: i64,
    pub completed: i64,
    pub last_played_at: Option<String>,
    pub play_count: i64,
}

/// Upsert playback progress for a media item and user.
/// Creates the row if it doesn't exist, otherwise updates position and timestamp.
/// `user_id` is optional for backward compatibility (API key access without a user context).
pub async fn upsert_progress(
    pool: &SqlitePool,
    media_item_id: &str,
    user_id: Option<&str>,
    position_ms: i64,
) -> Result<()> {
    let id = Uuid::new_v4().to_string();

    // The original schema has UNIQUE(media_item_id) and migration 005 added
    // UNIQUE(user_id, media_item_id). When user_id is NULL the single-column
    // constraint fires, so we must target the right conflict column.
    match user_id {
        Some(uid) => {
            sqlx::query(
                r#"
                INSERT INTO playback_progress (id, media_item_id, user_id, position_ms, completed, last_played_at, play_count)
                VALUES (?, ?, ?, ?, 0, datetime('now'), 0)
                ON CONFLICT(user_id, media_item_id) DO UPDATE SET
                    position_ms = excluded.position_ms,
                    completed = 0,
                    last_played_at = datetime('now')
                "#,
            )
            .bind(&id)
            .bind(media_item_id)
            .bind(uid)
            .bind(position_ms)
            .execute(pool)
            .await?;
        }
        None => {
            sqlx::query(
                r#"
                INSERT INTO playback_progress (id, media_item_id, user_id, position_ms, completed, last_played_at, play_count)
                VALUES (?, ?, NULL, ?, 0, datetime('now'), 0)
                ON CONFLICT(media_item_id) DO UPDATE SET
                    position_ms = excluded.position_ms,
                    completed = 0,
                    last_played_at = datetime('now')
                "#,
            )
            .bind(&id)
            .bind(media_item_id)
            .bind(position_ms)
            .execute(pool)
            .await?;
        }
    }

    Ok(())
}

/// Mark a media item as completed (finished watching).
/// Increments play_count and sets completed flag.
pub async fn mark_completed(
    pool: &SqlitePool,
    media_item_id: &str,
    user_id: Option<&str>,
) -> Result<()> {
    let id = Uuid::new_v4().to_string();

    match user_id {
        Some(uid) => {
            sqlx::query(
                r#"
                INSERT INTO playback_progress (id, media_item_id, user_id, position_ms, completed, last_played_at, play_count)
                VALUES (?, ?, ?, 0, 1, datetime('now'), 1)
                ON CONFLICT(user_id, media_item_id) DO UPDATE SET
                    completed = 1,
                    play_count = playback_progress.play_count + 1,
                    last_played_at = datetime('now')
                "#,
            )
            .bind(&id)
            .bind(media_item_id)
            .bind(uid)
            .execute(pool)
            .await?;
        }
        None => {
            sqlx::query(
                r#"
                INSERT INTO playback_progress (id, media_item_id, user_id, position_ms, completed, last_played_at, play_count)
                VALUES (?, ?, NULL, 0, 1, datetime('now'), 1)
                ON CONFLICT(media_item_id) DO UPDATE SET
                    completed = 1,
                    play_count = playback_progress.play_count + 1,
                    last_played_at = datetime('now')
                "#,
            )
            .bind(&id)
            .bind(media_item_id)
            .execute(pool)
            .await?;
        }
    }

    Ok(())
}

/// Reset playback progress for a media item — clears position and completed flag.
pub async fn reset_progress(
    pool: &SqlitePool,
    media_item_id: &str,
    user_id: Option<&str>,
) -> Result<()> {
    match user_id {
        Some(uid) => {
            sqlx::query(
                r#"
                UPDATE playback_progress
                SET position_ms = 0, completed = 0
                WHERE media_item_id = ? AND user_id = ?
                "#,
            )
            .bind(media_item_id)
            .bind(uid)
            .execute(pool)
            .await?;
        }
        None => {
            sqlx::query(
                r#"
                UPDATE playback_progress
                SET position_ms = 0, completed = 0
                WHERE media_item_id = ? AND user_id IS NULL
                "#,
            )
            .bind(media_item_id)
            .execute(pool)
            .await?;
        }
    }
    Ok(())
}

/// Get playback progress for a single media item and user.
pub async fn get_progress(
    pool: &SqlitePool,
    media_item_id: &str,
    user_id: Option<&str>,
) -> Result<Option<ProgressRow>> {
    let row = match user_id {
        Some(uid) => {
            sqlx::query_as::<_, ProgressRow>(
                "SELECT * FROM playback_progress WHERE media_item_id = ? AND user_id = ?",
            )
            .bind(media_item_id)
            .bind(uid)
            .fetch_optional(pool)
            .await?
        }
        None => {
            sqlx::query_as::<_, ProgressRow>(
                "SELECT * FROM playback_progress WHERE media_item_id = ? AND user_id IS NULL",
            )
            .bind(media_item_id)
            .fetch_optional(pool)
            .await?
        }
    };

    Ok(row)
}

/// Get recently played items for a user, ordered by last_played_at descending.
pub async fn get_recently_played(
    pool: &SqlitePool,
    user_id: Option<&str>,
    limit: i64,
) -> Result<Vec<ProgressRow>> {
    let rows = match user_id {
        Some(uid) => {
            sqlx::query_as::<_, ProgressRow>(
                r#"
                SELECT * FROM playback_progress
                WHERE last_played_at IS NOT NULL AND user_id = ?
                ORDER BY last_played_at DESC
                LIMIT ?
                "#,
            )
            .bind(uid)
            .bind(limit)
            .fetch_all(pool)
            .await?
        }
        None => {
            sqlx::query_as::<_, ProgressRow>(
                r#"
                SELECT * FROM playback_progress
                WHERE last_played_at IS NOT NULL AND user_id IS NULL
                ORDER BY last_played_at DESC
                LIMIT ?
                "#,
            )
            .bind(limit)
            .fetch_all(pool)
            .await?
        }
    };

    Ok(rows)
}
