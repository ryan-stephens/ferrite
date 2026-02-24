use anyhow::Result;
use sqlx::{SqliteConnection, SqlitePool};
use uuid::Uuid;

/// Probe data from ffprobe, used during scanning.
#[derive(Debug, Default)]
pub struct MediaProbeData {
    pub container_format: Option<String>,
    pub video_codec: Option<String>,
    pub audio_codec: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub duration_ms: Option<u64>,
    pub bitrate_kbps: Option<u32>,
}

#[allow(clippy::too_many_arguments)]
pub async fn insert_media_item(
    executor: &mut SqliteConnection,
    library_id: &Uuid,
    media_type: &str,
    file_path: &str,
    file_size: u64,
    title: Option<&str>,
    year: Option<i32>,
    probe: Option<&MediaProbeData>,
) -> Result<String> {
    let id = Uuid::new_v4().to_string();
    let empty = MediaProbeData::default();
    let p = probe.unwrap_or(&empty);

    // Use RETURNING id to get the actual row id in a single query.
    // On INSERT the new id is returned; on conflict the existing row's id is returned
    // because the DO UPDATE triggers RETURNING on the updated row.
    let actual_id: (String,) = sqlx::query_as(
        r#"INSERT INTO media_items (id, library_id, media_type, file_path, file_size, title, year,
             container_format, video_codec, audio_codec, width, height, duration_ms, bitrate_kbps)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
           ON CONFLICT(file_path) DO UPDATE SET
             file_size = excluded.file_size,
             title = excluded.title,
             year = excluded.year,
             container_format = excluded.container_format,
             video_codec = excluded.video_codec,
             audio_codec = excluded.audio_codec,
             width = excluded.width,
             height = excluded.height,
             duration_ms = excluded.duration_ms,
             bitrate_kbps = excluded.bitrate_kbps,
             updated_at = datetime('now')
           RETURNING id"#,
    )
    .bind(&id)
    .bind(library_id.to_string())
    .bind(media_type)
    .bind(file_path)
    .bind(file_size as i64)
    .bind(title)
    .bind(year)
    .bind(&p.container_format)
    .bind(&p.video_codec)
    .bind(&p.audio_codec)
    .bind(p.width.map(|v| v as i64))
    .bind(p.height.map(|v| v as i64))
    .bind(p.duration_ms.map(|v| v as i64))
    .bind(p.bitrate_kbps.map(|v| v as i64))
    .fetch_one(executor)
    .await?;

    Ok(actual_id.0)
}

/// Load all (file_path, file_size) pairs for a library for delta scan.
/// Used to skip re-probing files that haven't changed since the last scan.
pub async fn get_all_file_sizes(
    pool: &SqlitePool,
    library_id: &str,
) -> Result<std::collections::HashMap<String, u64>> {
    let rows: Vec<(String, i64)> =
        sqlx::query_as("SELECT file_path, file_size FROM media_items WHERE library_id = ?")
            .bind(library_id)
            .fetch_all(pool)
            .await?;
    Ok(rows.into_iter().map(|(p, s)| (p, s as u64)).collect())
}

/// Look up a media item's ID by its file path. Used after a batched insert to
/// retrieve the ID for subtitle extraction (which runs outside the transaction).
pub async fn get_media_item_id_by_path(
    pool: &SqlitePool,
    file_path: &str,
) -> Result<Option<String>> {
    let row: Option<(String,)> = sqlx::query_as("SELECT id FROM media_items WHERE file_path = ?")
        .bind(file_path)
        .fetch_optional(pool)
        .await?;
    Ok(row.map(|r| r.0))
}

pub async fn get_media_item(pool: &SqlitePool, id: &str) -> Result<Option<MediaItemRow>> {
    let row = sqlx::query_as::<_, MediaItemRow>(
        r#"SELECT mi.*,
                  e.episode_number,
                  e.title AS episode_title,
                  s.season_number,
                  ts.title AS show_title
           FROM media_items mi
           LEFT JOIN episodes e ON e.media_item_id = mi.id
           LEFT JOIN seasons s ON s.id = e.season_id
           LEFT JOIN tv_shows ts ON ts.id = s.tv_show_id
           WHERE mi.id = ?"#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn list_media_items(
    pool: &SqlitePool,
    library_id: Option<&str>,
    page: u32,
    per_page: u32,
) -> Result<Vec<MediaItemRow>> {
    let offset = (page.saturating_sub(1)) * per_page;

    let rows = if let Some(lib_id) = library_id {
        sqlx::query_as::<_, MediaItemRow>(
            r#"SELECT mi.*,
                      e.episode_number,
                      e.title AS episode_title,
                      s.season_number,
                      ts.title AS show_title
               FROM media_items mi
               LEFT JOIN episodes e ON e.media_item_id = mi.id
               LEFT JOIN seasons s ON s.id = e.season_id
               LEFT JOIN tv_shows ts ON ts.id = s.tv_show_id
               WHERE mi.library_id = ?
               ORDER BY mi.title ASC, mi.file_path ASC
               LIMIT ? OFFSET ?"#,
        )
        .bind(lib_id)
        .bind(per_page as i64)
        .bind(offset as i64)
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query_as::<_, MediaItemRow>(
            r#"SELECT mi.*,
                      e.episode_number,
                      e.title AS episode_title,
                      s.season_number,
                      ts.title AS show_title
               FROM media_items mi
               LEFT JOIN episodes e ON e.media_item_id = mi.id
               LEFT JOIN seasons s ON s.id = e.season_id
               LEFT JOIN tv_shows ts ON ts.id = s.tv_show_id
               ORDER BY mi.title ASC, mi.file_path ASC
               LIMIT ? OFFSET ?"#,
        )
        .bind(per_page as i64)
        .bind(offset as i64)
        .fetch_all(pool)
        .await?
    };

    Ok(rows)
}

pub async fn count_media_items(pool: &SqlitePool, library_id: Option<&str>) -> Result<i64> {
    let count: (i64,) = if let Some(lib_id) = library_id {
        sqlx::query_as("SELECT COUNT(*) FROM media_items WHERE library_id = ?")
            .bind(lib_id)
            .fetch_one(pool)
            .await?
    } else {
        sqlx::query_as("SELECT COUNT(*) FROM media_items")
            .fetch_one(pool)
            .await?
    };
    Ok(count.0)
}

/// Return all media item IDs belonging to a library (used for cache cleanup before deletion).
pub async fn list_media_item_ids_for_library(
    pool: &SqlitePool,
    library_id: &str,
) -> Result<Vec<String>> {
    let rows: Vec<(String,)> = sqlx::query_as("SELECT id FROM media_items WHERE library_id = ?")
        .bind(library_id)
        .fetch_all(pool)
        .await?;
    Ok(rows.into_iter().map(|(id,)| id).collect())
}

pub async fn delete_media_items_for_library(pool: &SqlitePool, library_id: &str) -> Result<()> {
    sqlx::query("DELETE FROM media_items WHERE library_id = ?")
        .bind(library_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete_media_item_by_path(pool: &SqlitePool, file_path: &str) -> Result<u64> {
    let result = sqlx::query("DELETE FROM media_items WHERE file_path = ?")
        .bind(file_path)
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}

pub async fn delete_media_items_by_path_prefix(
    pool: &SqlitePool,
    path_prefix: &str,
) -> Result<u64> {
    let normalized = path_prefix.trim_end_matches(['\\', '/']);
    let escaped = normalized
        .replace('\\', "\\\\")
        .replace('%', "\\%")
        .replace('_', "\\_");
    let like_backslash = format!("{}\\\\%", escaped);
    let like_slash = format!("{}/%", escaped);

    let result = sqlx::query(
        "DELETE FROM media_items WHERE file_path = ? OR file_path LIKE ? ESCAPE '\\' OR file_path LIKE ? ESCAPE '\\'",
    )
    .bind(normalized)
    .bind(like_backslash)
    .bind(like_slash)
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
pub struct MediaItemRow {
    pub id: String,
    pub library_id: String,
    pub media_type: String,
    pub file_path: String,
    pub file_size: i64,
    pub file_hash: Option<String>,
    pub duration_ms: Option<i64>,
    pub container_format: Option<String>,
    pub video_codec: Option<String>,
    pub audio_codec: Option<String>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub bitrate_kbps: Option<i64>,
    pub title: Option<String>,
    pub year: Option<i64>,
    pub added_at: String,
    pub updated_at: String,
    /// Episode number (null for non-episodes)
    pub episode_number: Option<i64>,
    /// Episode title from the episodes table (null for non-episodes)
    pub episode_title: Option<String>,
    /// Season number (null for non-episodes)
    pub season_number: Option<i64>,
    /// Show title (null for non-episodes)
    pub show_title: Option<String>,
}
