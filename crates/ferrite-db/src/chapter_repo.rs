use anyhow::Result;
use sqlx::{SqliteConnection, SqlitePool};

/// Data for a single chapter to insert into the database.
#[derive(Debug)]
pub struct ChapterInsert {
    pub chapter_index: u32,
    pub title: Option<String>,
    pub start_time_ms: u64,
    pub end_time_ms: u64,
}

/// Row type returned when querying chapters.
#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
pub struct ChapterRow {
    pub id: i64,
    pub media_item_id: String,
    pub chapter_index: i64,
    pub title: Option<String>,
    pub start_time_ms: i64,
    pub end_time_ms: i64,
}

/// Replace all chapters for a media item (delete old, insert new).
/// Accepts `&mut SqliteConnection` so it can run inside a transaction.
pub async fn replace_chapters(
    executor: &mut SqliteConnection,
    media_item_id: &str,
    chapters: &[ChapterInsert],
) -> Result<()> {
    sqlx::query("DELETE FROM chapters WHERE media_item_id = ?")
        .bind(media_item_id)
        .execute(&mut *executor)
        .await?;

    for c in chapters {
        sqlx::query(
            r#"INSERT INTO chapters (media_item_id, chapter_index, title, start_time_ms, end_time_ms)
               VALUES (?, ?, ?, ?, ?)"#,
        )
        .bind(media_item_id)
        .bind(c.chapter_index as i64)
        .bind(&c.title)
        .bind(c.start_time_ms as i64)
        .bind(c.end_time_ms as i64)
        .execute(&mut *executor)
        .await?;
    }

    Ok(())
}

/// Get all chapters for a media item, ordered by start time.
pub async fn get_chapters(pool: &SqlitePool, media_item_id: &str) -> Result<Vec<ChapterRow>> {
    let rows = sqlx::query_as::<_, ChapterRow>(
        "SELECT * FROM chapters WHERE media_item_id = ? ORDER BY chapter_index",
    )
    .bind(media_item_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}
