use anyhow::{anyhow, Result};
use std::path::{Path, PathBuf};
use tracing::{debug, info, warn};

/// Configuration for thumbnail sprite sheet generation.
#[derive(Debug, Clone)]
pub struct ThumbnailConfig {
    /// Interval between thumbnails in seconds
    pub interval_secs: u32,
    /// Width of each individual thumbnail in pixels (height auto-scaled)
    pub thumb_width: u32,
    /// Number of columns in the sprite grid
    pub columns: u32,
}

impl Default for ThumbnailConfig {
    fn default() -> Self {
        Self {
            interval_secs: 10,
            thumb_width: 160,
            columns: 10,
        }
    }
}

/// Result of sprite sheet generation.
#[derive(Debug, Clone)]
pub struct SpriteSheetResult {
    /// Path to the generated sprite sheet image (JPEG)
    pub image_path: PathBuf,
    /// Path to the generated WebVTT file
    pub vtt_path: PathBuf,
    /// Total number of thumbnails generated
    pub thumb_count: u32,
    /// Number of columns in the grid
    pub columns: u32,
    /// Number of rows in the grid
    pub rows: u32,
    /// Width of each thumbnail
    pub thumb_width: u32,
    /// Height of each thumbnail
    pub thumb_height: u32,
    /// Interval between thumbnails in seconds
    pub interval_secs: u32,
}

/// Generate a thumbnail sprite sheet and WebVTT file for a video.
///
/// Uses FFmpeg to extract frames at regular intervals, then tiles them
/// into a single JPEG sprite sheet. Also generates a WebVTT file that
/// maps time ranges to sprite coordinates for scrubber preview.
///
/// Returns `None` if the video has no duration or generation fails.
pub async fn generate_sprite_sheet(
    ffmpeg_path: &str,
    video_path: &Path,
    output_dir: &Path,
    media_id: &str,
    duration_secs: f64,
    config: &ThumbnailConfig,
) -> Result<SpriteSheetResult> {
    if duration_secs <= 0.0 {
        return Err(anyhow!("Video has no duration"));
    }

    tokio::fs::create_dir_all(output_dir).await?;

    let thumb_count = (duration_secs / config.interval_secs as f64).ceil() as u32;
    if thumb_count == 0 {
        return Err(anyhow!("Video too short for thumbnails"));
    }

    let columns = config.columns.min(thumb_count);
    let rows = (thumb_count + columns - 1) / columns;

    let sprite_path = output_dir.join(format!("{}_sprites.jpg", media_id));
    let vtt_path = output_dir.join(format!("{}_sprites.vtt", media_id));

    // FFmpeg command: extract frames at interval, scale to thumb width, tile into grid
    // fps=1/interval extracts one frame per interval
    // scale=width:-1 maintains aspect ratio
    // tile=columns×rows creates the sprite sheet
    let filter = format!(
        "fps=1/{},scale={}:-1,tile={}x{}",
        config.interval_secs, config.thumb_width, columns, rows
    );

    let video_str = video_path.to_string_lossy().to_string();
    let sprite_str = sprite_path.to_string_lossy().to_string();

    let args = vec![
        "-hide_banner",
        "-nostdin",
        "-i",
        &video_str,
        "-frames:v",
        "1",
        "-vf",
        &filter,
        "-q:v",
        "5",
        "-y",
        &sprite_str,
    ];

    debug!("Generating sprite sheet: ffmpeg {}", args.join(" "));

    let output = tokio::process::Command::new(ffmpeg_path)
        .args(&args)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::piped())
        .output()
        .await
        .map_err(|e| anyhow!("Failed to spawn ffmpeg for thumbnails: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        warn!("FFmpeg thumbnail generation failed: {}", stderr);
        return Err(anyhow!("FFmpeg thumbnail generation failed"));
    }

    if !sprite_path.exists() {
        return Err(anyhow!("Sprite sheet was not created"));
    }

    // Probe the actual sprite sheet dimensions to compute per-thumb height
    let (sprite_w, sprite_h) = probe_image_dimensions(ffmpeg_path, &sprite_path).await?;
    let thumb_width = sprite_w / columns;
    let thumb_height = sprite_h / rows;

    info!(
        "Generated sprite sheet for {}: {}x{} grid ({}x{} per thumb, {} total)",
        media_id, columns, rows, thumb_width, thumb_height, thumb_count
    );

    // Generate WebVTT file
    let vtt_content = generate_vtt(
        media_id,
        thumb_count,
        columns,
        thumb_width,
        thumb_height,
        config.interval_secs,
        duration_secs,
    );
    tokio::fs::write(&vtt_path, &vtt_content).await?;

    Ok(SpriteSheetResult {
        image_path: sprite_path,
        vtt_path,
        thumb_count,
        columns,
        rows,
        thumb_width,
        thumb_height,
        interval_secs: config.interval_secs,
    })
}

/// Probe image dimensions using ffprobe.
async fn probe_image_dimensions(ffmpeg_path: &str, image_path: &Path) -> Result<(u32, u32)> {
    // Derive ffprobe path from ffmpeg path
    let ffprobe_path = ffmpeg_path.replace("ffmpeg", "ffprobe");

    let output = tokio::process::Command::new(&ffprobe_path)
        .args([
            "-v", "error",
            "-select_streams", "v:0",
            "-show_entries", "stream=width,height",
            "-of", "csv=p=0:s=x",
        ])
        .arg(image_path)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .output()
        .await
        .map_err(|e| anyhow!("Failed to probe image dimensions: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let parts: Vec<&str> = stdout.trim().split('x').collect();
    if parts.len() == 2 {
        let w: u32 = parts[0].parse().unwrap_or(0);
        let h: u32 = parts[1].parse().unwrap_or(0);
        if w > 0 && h > 0 {
            return Ok((w, h));
        }
    }

    // Fallback: estimate from thumb_width and 16:9 aspect ratio
    Err(anyhow!("Could not probe image dimensions from: {}", stdout.trim()))
}

/// Generate a WebVTT file mapping time ranges to sprite sheet coordinates.
///
/// Format follows the de-facto standard used by JW Player, Video.js, etc:
/// ```text
/// WEBVTT
///
/// 00:00:00.000 --> 00:00:10.000
/// sprites.jpg#xywh=0,0,160,90
///
/// 00:00:10.000 --> 00:00:20.000
/// sprites.jpg#xywh=160,0,160,90
/// ```
fn generate_vtt(
    media_id: &str,
    thumb_count: u32,
    columns: u32,
    thumb_width: u32,
    thumb_height: u32,
    interval_secs: u32,
    duration_secs: f64,
) -> String {
    let mut vtt = String::from("WEBVTT\n\n");
    let sprite_filename = format!("{}_sprites.jpg", media_id);

    for i in 0..thumb_count {
        let start_secs = i * interval_secs;
        let end_secs = ((i + 1) * interval_secs).min(duration_secs.ceil() as u32);

        let col = i % columns;
        let row = i / columns;
        let x = col * thumb_width;
        let y = row * thumb_height;

        vtt.push_str(&format!(
            "{} --> {}\n{}\n\n",
            format_vtt_time(start_secs),
            format_vtt_time(end_secs),
            format!("{}#xywh={},{},{},{}", sprite_filename, x, y, thumb_width, thumb_height),
        ));
    }

    vtt
}

/// Format seconds as VTT timestamp: HH:MM:SS.mmm
fn format_vtt_time(secs: u32) -> String {
    let h = secs / 3600;
    let m = (secs % 3600) / 60;
    let s = secs % 60;
    format!("{:02}:{:02}:{:02}.000", h, m, s)
}

/// Check if a sprite sheet already exists for a media item.
pub fn sprite_sheet_exists(output_dir: &Path, media_id: &str) -> bool {
    let sprite_path = output_dir.join(format!("{}_sprites.jpg", media_id));
    let vtt_path = output_dir.join(format!("{}_sprites.vtt", media_id));
    sprite_path.exists() && vtt_path.exists()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_vtt_time() {
        assert_eq!(format_vtt_time(0), "00:00:00.000");
        assert_eq!(format_vtt_time(65), "00:01:05.000");
        assert_eq!(format_vtt_time(3661), "01:01:01.000");
    }

    #[test]
    fn test_generate_vtt_basic() {
        let vtt = generate_vtt("test-id", 4, 2, 160, 90, 10, 40.0);
        assert!(vtt.starts_with("WEBVTT\n\n"));
        assert!(vtt.contains("00:00:00.000 --> 00:00:10.000"));
        assert!(vtt.contains("test-id_sprites.jpg#xywh=0,0,160,90"));
        assert!(vtt.contains("00:00:10.000 --> 00:00:20.000"));
        assert!(vtt.contains("test-id_sprites.jpg#xywh=160,0,160,90"));
        // Second row
        assert!(vtt.contains("00:00:20.000 --> 00:00:30.000"));
        assert!(vtt.contains("test-id_sprites.jpg#xywh=0,90,160,90"));
    }

    #[test]
    fn test_generate_vtt_single_thumb() {
        let vtt = generate_vtt("short", 1, 1, 160, 90, 10, 5.0);
        assert!(vtt.contains("00:00:00.000 --> 00:00:05.000"));
        assert!(vtt.contains("short_sprites.jpg#xywh=0,0,160,90"));
    }

    #[test]
    fn test_default_config() {
        let cfg = ThumbnailConfig::default();
        assert_eq!(cfg.interval_secs, 10);
        assert_eq!(cfg.thumb_width, 160);
        assert_eq!(cfg.columns, 10);
    }

    #[test]
    fn test_sprite_sheet_exists_false() {
        let dir = std::env::temp_dir();
        assert!(!sprite_sheet_exists(&dir, "nonexistent-media-id-12345"));
    }
}
