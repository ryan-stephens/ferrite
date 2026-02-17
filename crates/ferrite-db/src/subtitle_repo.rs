use anyhow::Result;
use sqlx::SqlitePool;

/// Data for an external subtitle file to insert into the database.
#[derive(Debug)]
pub struct SubtitleInsert {
    pub file_path: String,
    pub format: String,
    pub language: Option<String>,
    pub title: Option<String>,
    pub is_forced: bool,
    pub is_sdh: bool,
    pub file_size: u64,
}

/// Replace all external subtitles for a media item (delete old, insert new).
/// Called during scanning when a media file's sibling subtitles are re-discovered.
pub async fn replace_subtitles(
    pool: &SqlitePool,
    media_item_id: &str,
    subtitles: &[SubtitleInsert],
) -> Result<()> {
    sqlx::query("DELETE FROM external_subtitles WHERE media_item_id = ?")
        .bind(media_item_id)
        .execute(pool)
        .await?;

    for s in subtitles {
        sqlx::query(
            r#"INSERT INTO external_subtitles
                (media_item_id, file_path, format, language, title, is_forced, is_sdh, file_size)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(media_item_id)
        .bind(&s.file_path)
        .bind(&s.format)
        .bind(&s.language)
        .bind(&s.title)
        .bind(s.is_forced as i32)
        .bind(s.is_sdh as i32)
        .bind(s.file_size as i64)
        .execute(pool)
        .await?;
    }

    Ok(())
}

/// Row type for querying external subtitles.
#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
pub struct ExternalSubtitleRow {
    pub id: i64,
    pub media_item_id: String,
    pub file_path: String,
    pub format: String,
    pub language: Option<String>,
    pub title: Option<String>,
    pub is_forced: i64,
    pub is_sdh: i64,
    pub file_size: i64,
}

/// Get all external subtitles for a media item.
pub async fn get_subtitles(
    pool: &SqlitePool,
    media_item_id: &str,
) -> Result<Vec<ExternalSubtitleRow>> {
    let rows = sqlx::query_as::<_, ExternalSubtitleRow>(
        "SELECT * FROM external_subtitles WHERE media_item_id = ? ORDER BY language, title",
    )
    .bind(media_item_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

/// Get a single subtitle by ID.
pub async fn get_subtitle_by_id(
    pool: &SqlitePool,
    id: i64,
) -> Result<Option<ExternalSubtitleRow>> {
    let row = sqlx::query_as::<_, ExternalSubtitleRow>(
        "SELECT * FROM external_subtitles WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}
