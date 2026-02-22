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
/// All operations run in a single transaction to avoid per-row write lock contention
/// when many concurrent subtitle extractions are running simultaneously.
///
/// Deduplication: when both an external subtitle file and an extracted embedded
/// subtitle share the same (language, is_forced, is_sdh) combination, the external
/// file is preferred (typically higher quality). Embedded subtitles whose file_path
/// starts with the subtitle cache directory prefix are considered "extracted".
pub async fn replace_subtitles(
    pool: &SqlitePool,
    media_item_id: &str,
    subtitles: &[SubtitleInsert],
) -> Result<()> {
    // Deduplicate: prefer external subtitles over extracted embedded ones.
    // An extracted subtitle has a cache-dir path (contains "/subtitle_cache/" or "\\subtitle_cache\\").
    let is_extracted = |s: &SubtitleInsert| -> bool {
        s.file_path.contains("/subtitle_cache/") || s.file_path.contains("\\subtitle_cache\\")
    };

    // Build a set of (language, is_forced, is_sdh) keys from external (non-extracted) subs
    let mut external_keys = std::collections::HashSet::new();
    for s in subtitles {
        if !is_extracted(s) {
            let key = (
                s.language.as_deref().unwrap_or("").to_lowercase(),
                s.is_forced,
                s.is_sdh,
            );
            external_keys.insert(key);
        }
    }

    // Filter: keep all external subs, and only keep extracted subs that don't
    // duplicate an external sub's language+flags combination.
    let deduped: Vec<&SubtitleInsert> = subtitles
        .iter()
        .filter(|s| {
            if !is_extracted(s) {
                return true; // always keep external
            }
            let key = (
                s.language.as_deref().unwrap_or("").to_lowercase(),
                s.is_forced,
                s.is_sdh,
            );
            !external_keys.contains(&key)
        })
        .collect();

    let mut tx = pool.begin().await?;

    sqlx::query("DELETE FROM external_subtitles WHERE media_item_id = ?")
        .bind(media_item_id)
        .execute(&mut *tx)
        .await?;

    for s in &deduped {
        sqlx::query(
            r#"INSERT OR IGNORE INTO external_subtitles
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
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
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
pub async fn get_subtitle_by_id(pool: &SqlitePool, id: i64) -> Result<Option<ExternalSubtitleRow>> {
    let row =
        sqlx::query_as::<_, ExternalSubtitleRow>("SELECT * FROM external_subtitles WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await?;
    Ok(row)
}
