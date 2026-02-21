use anyhow::Result;
use sqlx::{SqliteConnection, SqlitePool};

/// Data for a single media stream to insert into the database.
#[derive(Debug)]
pub struct StreamInsert {
    pub stream_index: u32,
    pub stream_type: String,
    pub codec_name: Option<String>,
    pub codec_long_name: Option<String>,
    pub profile: Option<String>,
    pub language: Option<String>,
    pub title: Option<String>,
    pub is_default: bool,
    pub is_forced: bool,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub frame_rate: Option<String>,
    pub pixel_format: Option<String>,
    pub bit_depth: Option<u32>,
    pub color_space: Option<String>,
    pub color_transfer: Option<String>,
    pub color_primaries: Option<String>,
    pub channels: Option<u32>,
    pub channel_layout: Option<String>,
    pub sample_rate: Option<u32>,
    pub bitrate_bps: Option<u64>,
}

/// Replace all streams for a media item (delete old, insert new).
/// Called during scanning when a file is re-probed.
/// Accepts `&mut SqliteConnection` so it can run inside a transaction.
pub async fn replace_streams(
    executor: &mut SqliteConnection,
    media_item_id: &str,
    streams: &[StreamInsert],
) -> Result<()> {
    // Delete existing streams for this media item
    sqlx::query("DELETE FROM media_streams WHERE media_item_id = ?")
        .bind(media_item_id)
        .execute(&mut *executor)
        .await?;

    // Insert all new streams
    for s in streams {
        sqlx::query(
            r#"INSERT INTO media_streams (
                media_item_id, stream_index, stream_type, codec_name, codec_long_name,
                profile, language, title, is_default, is_forced,
                width, height, frame_rate, pixel_format, bit_depth,
                color_space, color_transfer, color_primaries,
                channels, channel_layout, sample_rate, bitrate_bps
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(media_item_id)
        .bind(s.stream_index as i64)
        .bind(&s.stream_type)
        .bind(&s.codec_name)
        .bind(&s.codec_long_name)
        .bind(&s.profile)
        .bind(&s.language)
        .bind(&s.title)
        .bind(s.is_default as i32)
        .bind(s.is_forced as i32)
        .bind(s.width.map(|v| v as i64))
        .bind(s.height.map(|v| v as i64))
        .bind(&s.frame_rate)
        .bind(&s.pixel_format)
        .bind(s.bit_depth.map(|v| v as i64))
        .bind(&s.color_space)
        .bind(&s.color_transfer)
        .bind(&s.color_primaries)
        .bind(s.channels.map(|v| v as i64))
        .bind(&s.channel_layout)
        .bind(s.sample_rate.map(|v| v as i64))
        .bind(s.bitrate_bps.map(|v| v as i64))
        .execute(&mut *executor)
        .await?;
    }

    Ok(())
}

/// Row type for querying media streams.
#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
pub struct MediaStreamRow {
    pub id: i64,
    pub media_item_id: String,
    pub stream_index: i64,
    pub stream_type: String,
    pub codec_name: Option<String>,
    pub codec_long_name: Option<String>,
    pub profile: Option<String>,
    pub language: Option<String>,
    pub title: Option<String>,
    pub is_default: i64,
    pub is_forced: i64,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub frame_rate: Option<String>,
    pub pixel_format: Option<String>,
    pub bit_depth: Option<i64>,
    pub color_space: Option<String>,
    pub color_transfer: Option<String>,
    pub color_primaries: Option<String>,
    pub channels: Option<i64>,
    pub channel_layout: Option<String>,
    pub sample_rate: Option<i64>,
    pub bitrate_bps: Option<i64>,
}

/// Get all streams for a media item, ordered by stream index.
pub async fn get_streams(pool: &SqlitePool, media_item_id: &str) -> Result<Vec<MediaStreamRow>> {
    let rows = sqlx::query_as::<_, MediaStreamRow>(
        "SELECT * FROM media_streams WHERE media_item_id = ? ORDER BY stream_index",
    )
    .bind(media_item_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

/// All video stream metadata needed for transcoding decisions, fetched in one query.
#[derive(Debug, Clone)]
pub struct VideoMeta {
    pub pixel_format: Option<String>,
    pub frame_rate: Option<String>,
    pub color_space: Option<String>,
    pub color_transfer: Option<String>,
    pub color_primaries: Option<String>,
}

/// Fetch all video stream metadata for a media item in a single DB round-trip.
/// Replaces the three separate `get_video_pixel_format`, `get_video_frame_rate`,
/// and `get_video_color_metadata` calls that were previously made sequentially.
pub async fn get_video_meta(pool: &SqlitePool, media_item_id: &str) -> Result<Option<VideoMeta>> {
    let row: Option<(
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
    )> = sqlx::query_as(
        "SELECT pixel_format, frame_rate, color_space, color_transfer, color_primaries \
             FROM media_streams \
             WHERE media_item_id = ? AND stream_type = 'video' \
             ORDER BY stream_index LIMIT 1",
    )
    .bind(media_item_id)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|r| VideoMeta {
        pixel_format: r.0,
        frame_rate: r.1,
        color_space: r.2,
        color_transfer: r.3,
        color_primaries: r.4,
    }))
}

/// Get the pixel format of the first video stream for a media item.
/// Returns `None` if no video stream exists or pixel_format is not set.
pub async fn get_video_pixel_format(
    pool: &SqlitePool,
    media_item_id: &str,
) -> Result<Option<String>> {
    let row: Option<(Option<String>,)> = sqlx::query_as(
        "SELECT pixel_format FROM media_streams WHERE media_item_id = ? AND stream_type = 'video' ORDER BY stream_index LIMIT 1",
    )
    .bind(media_item_id)
    .fetch_optional(pool)
    .await?;
    Ok(row.and_then(|r| r.0))
}

/// Get the frame rate of the first video stream for a media item.
/// Returns the raw string (e.g. "24000/1001", "30/1") or `None`.
pub async fn get_video_frame_rate(
    pool: &SqlitePool,
    media_item_id: &str,
) -> Result<Option<String>> {
    let row: Option<(Option<String>,)> = sqlx::query_as(
        "SELECT frame_rate FROM media_streams WHERE media_item_id = ? AND stream_type = 'video' ORDER BY stream_index LIMIT 1",
    )
    .bind(media_item_id)
    .fetch_optional(pool)
    .await?;
    Ok(row.and_then(|r| r.0))
}

/// Color metadata for the first video stream of a media item.
#[derive(Debug, Clone)]
pub struct VideoColorMeta {
    pub color_space: Option<String>,
    pub color_transfer: Option<String>,
    pub color_primaries: Option<String>,
}

/// Get color metadata (color_space, color_transfer, color_primaries) for the first video stream.
pub async fn get_video_color_metadata(
    pool: &SqlitePool,
    media_item_id: &str,
) -> Result<Option<VideoColorMeta>> {
    let row: Option<(Option<String>, Option<String>, Option<String>)> = sqlx::query_as(
        "SELECT color_space, color_transfer, color_primaries FROM media_streams WHERE media_item_id = ? AND stream_type = 'video' ORDER BY stream_index LIMIT 1",
    )
    .bind(media_item_id)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|r| VideoColorMeta {
        color_space: r.0,
        color_transfer: r.1,
        color_primaries: r.2,
    }))
}
