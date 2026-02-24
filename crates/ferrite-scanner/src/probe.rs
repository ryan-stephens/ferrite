use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::Path;
use tokio::process::Command;
use tracing::{debug, warn};

const KEYFRAME_INDEX_MIN_GAP_MS: u64 = 2_000;

/// A single stream (video, audio, or subtitle) extracted from ffprobe.
#[derive(Debug, Clone)]
pub struct StreamInfo {
    pub index: u32,
    pub stream_type: String,
    pub codec_name: Option<String>,
    pub codec_long_name: Option<String>,
    pub profile: Option<String>,
    pub language: Option<String>,
    pub title: Option<String>,
    pub is_default: bool,
    pub is_forced: bool,
    // Video-specific
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub frame_rate: Option<String>,
    pub pixel_format: Option<String>,
    pub bit_depth: Option<u32>,
    pub color_space: Option<String>,
    pub color_transfer: Option<String>,
    pub color_primaries: Option<String>,
    // Audio-specific
    pub channels: Option<u32>,
    pub channel_layout: Option<String>,
    pub sample_rate: Option<u32>,
    // Common
    pub bitrate_bps: Option<u64>,
}

/// A single chapter extracted from ffprobe.
#[derive(Debug, Clone)]
pub struct ChapterInfo {
    pub chapter_index: u32,
    pub title: Option<String>,
    pub start_time_ms: u64,
    pub end_time_ms: u64,
}

/// Media info extracted from ffprobe.
#[derive(Debug, Clone)]
pub struct ProbeResult {
    pub container_format: Option<String>,
    pub duration_ms: Option<u64>,
    pub bitrate_kbps: Option<u32>,
    pub video_codec: Option<String>,
    pub audio_codec: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    /// All individual streams (video, audio, subtitle) discovered in the file.
    pub streams: Vec<StreamInfo>,
    /// Chapter markers embedded in the container.
    pub chapters: Vec<ChapterInfo>,
    /// Coarse keyframe seek map persisted for fast runtime seeks.
    /// `None` means keyframe extraction failed; `Some(vec![])` means no keyframes found.
    pub keyframe_index_ms: Option<Vec<u64>>,
}

/// Run ffprobe on a file and extract stream/format info.
pub async fn probe_file(ffprobe_path: &str, file_path: &Path) -> Result<ProbeResult> {
    let output = Command::new(ffprobe_path)
        .args([
            "-v",
            "quiet",
            "-print_format",
            "json",
            "-show_format",
            "-show_streams",
            "-show_chapters",
        ])
        .arg(file_path)
        .output()
        .await
        .with_context(|| format!("Failed to run ffprobe on {}", file_path.display()))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        warn!("ffprobe failed for {}: {}", file_path.display(), stderr);
        return Ok(ProbeResult {
            container_format: None,
            duration_ms: None,
            bitrate_kbps: None,
            video_codec: None,
            audio_codec: None,
            width: None,
            height: None,
            streams: Vec::new(),
            chapters: Vec::new(),
            keyframe_index_ms: None,
        });
    }

    let json: FfprobeOutput = serde_json::from_slice(&output.stdout)
        .with_context(|| format!("Failed to parse ffprobe JSON for {}", file_path.display()))?;

    let container_format = json.format.as_ref().and_then(|f| {
        f.format_name.as_deref().map(|s| {
            // ffprobe returns comma-separated list like "matroska,webm" — take the first
            s.split(',').next().unwrap_or(s).to_string()
        })
    });

    let duration_ms = json.format.as_ref().and_then(|f| {
        f.duration
            .as_deref()
            .and_then(|d| d.parse::<f64>().ok().map(|s| (s * 1000.0) as u64))
    });

    let bitrate_kbps = json.format.as_ref().and_then(|f| {
        f.bit_rate
            .as_deref()
            .and_then(|b| b.parse::<u64>().ok().map(|bps| (bps / 1000) as u32))
    });

    // Find the first video stream
    let video_stream = json
        .streams
        .iter()
        .find(|s| s.codec_type.as_deref() == Some("video"));
    let video_codec = video_stream.and_then(|s| s.codec_name.clone());
    let width = video_stream.and_then(|s| s.width);
    let height = video_stream.and_then(|s| s.height);

    // Find the first audio stream
    let audio_stream = json
        .streams
        .iter()
        .find(|s| s.codec_type.as_deref() == Some("audio"));
    let audio_codec = audio_stream.and_then(|s| s.codec_name.clone());

    // Build detailed StreamInfo for every stream
    let streams: Vec<StreamInfo> = json
        .streams
        .iter()
        .filter_map(|s| {
            let stream_type = s.codec_type.as_deref()?;
            // Only capture video, audio, and subtitle streams
            if !matches!(stream_type, "video" | "audio" | "subtitle") {
                return None;
            }

            let disposition = s.disposition.as_ref();
            let tags = s.tags.as_ref();

            Some(StreamInfo {
                index: s.index.unwrap_or(0),
                stream_type: stream_type.to_string(),
                codec_name: s.codec_name.clone(),
                codec_long_name: s.codec_long_name.clone(),
                profile: s.profile.clone(),
                language: tags.and_then(|t| t.language.clone()),
                title: tags.and_then(|t| t.title.clone()),
                is_default: disposition.and_then(|d| d.default).unwrap_or(0) != 0,
                is_forced: disposition.and_then(|d| d.forced).unwrap_or(0) != 0,
                width: s.width,
                height: s.height,
                frame_rate: s.r_frame_rate.clone(),
                pixel_format: s.pix_fmt.clone(),
                bit_depth: s
                    .bits_per_raw_sample
                    .as_deref()
                    .and_then(|b| b.parse().ok()),
                color_space: s.color_space.clone(),
                color_transfer: s.color_transfer.clone(),
                color_primaries: s.color_primaries.clone(),
                channels: s.channels,
                channel_layout: s.channel_layout.clone(),
                sample_rate: s.sample_rate.as_deref().and_then(|r| r.parse().ok()),
                bitrate_bps: s.bit_rate.as_deref().and_then(|b| b.parse().ok()),
            })
        })
        .collect();

    // Keyframe indexing is deferred to on-demand (lazy) probing at seek time.
    // This avoids the expensive full-file read during scanning.
    let keyframe_index_ms = None;

    // Parse chapter markers
    let chapters: Vec<ChapterInfo> = json
        .chapters
        .iter()
        .enumerate()
        .filter_map(|(i, c)| {
            let start_ms = c
                .start_time
                .as_deref()
                .and_then(|s| s.parse::<f64>().ok())
                .map(|s| (s * 1000.0) as u64)?;
            let end_ms = c
                .end_time
                .as_deref()
                .and_then(|s| s.parse::<f64>().ok())
                .map(|s| (s * 1000.0) as u64)
                .unwrap_or(start_ms);
            Some(ChapterInfo {
                chapter_index: i as u32,
                title: c.tags.as_ref().and_then(|t| t.title.clone()),
                start_time_ms: start_ms,
                end_time_ms: end_ms,
            })
        })
        .collect();

    debug!(
        "Probed {}: container={:?} video={:?} audio={:?} {}x{} duration={}ms streams={} chapters={} keyframes={}",
        file_path.display(),
        container_format,
        video_codec,
        audio_codec,
        width.unwrap_or(0),
        height.unwrap_or(0),
        duration_ms.unwrap_or(0),
        streams.len(),
        chapters.len(),
        keyframe_index_ms.as_ref().map_or(0, Vec::len),
    );

    Ok(ProbeResult {
        container_format,
        duration_ms,
        bitrate_kbps,
        video_codec,
        audio_codec,
        width,
        height,
        streams,
        chapters,
        keyframe_index_ms,
    })
}

/// Probe keyframe positions from a media file using ffprobe.
///
/// This runs `ffprobe -skip_frame nokey` which reads the entire video file
/// to extract keyframe timestamps. The result is a coarse, deduplicated
/// keyframe map with a minimum gap of 2s between entries.
///
/// Called lazily on first seek (not during scan) to avoid blocking the scan
/// pipeline with expensive full-file reads.
pub async fn probe_keyframe_index(ffprobe_path: &str, file_path: &Path) -> Option<Vec<u64>> {
    let output = Command::new(ffprobe_path)
        .args([
            "-v",
            "error",
            "-select_streams",
            "v:0",
            "-skip_frame",
            "nokey",
            "-show_entries",
            "frame=pts_time",
            "-of",
            "csv=print_section=0",
        ])
        .arg(file_path)
        .output()
        .await
        .ok()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        warn!(
            "ffprobe keyframe index extraction failed for {}: {}",
            file_path.display(),
            stderr
        );
        return None;
    }

    let mut keyframes_ms = Vec::new();
    let mut last_kept_ms: Option<u64> = None;
    let text = String::from_utf8_lossy(&output.stdout);

    for line in text.lines() {
        let pts_secs = match parse_pts_time(line) {
            Some(v) => v,
            None => continue,
        };
        let pts_ms = (pts_secs * 1000.0).round() as u64;
        let keep = last_kept_ms
            .map(|last| pts_ms >= last + KEYFRAME_INDEX_MIN_GAP_MS)
            .unwrap_or(true);
        if keep {
            keyframes_ms.push(pts_ms);
            last_kept_ms = Some(pts_ms);
        }
    }

    Some(keyframes_ms)
}

fn parse_pts_time(line: &str) -> Option<f64> {
    line.split(',')
        .find_map(|token| token.trim().parse::<f64>().ok())
        .filter(|v| *v >= 0.0)
}

#[derive(Deserialize)]
struct FfprobeOutput {
    #[serde(default)]
    streams: Vec<FfprobeStream>,
    format: Option<FfprobeFormat>,
    #[serde(default)]
    chapters: Vec<FfprobeChapter>,
}

#[derive(Deserialize)]
struct FfprobeChapter {
    start_time: Option<String>,
    end_time: Option<String>,
    tags: Option<FfprobeTags>,
}

#[derive(Deserialize)]
struct FfprobeStream {
    index: Option<u32>,
    codec_name: Option<String>,
    codec_long_name: Option<String>,
    codec_type: Option<String>,
    profile: Option<String>,
    width: Option<u32>,
    height: Option<u32>,
    r_frame_rate: Option<String>,
    pix_fmt: Option<String>,
    bits_per_raw_sample: Option<String>,
    color_space: Option<String>,
    color_transfer: Option<String>,
    color_primaries: Option<String>,
    channels: Option<u32>,
    channel_layout: Option<String>,
    sample_rate: Option<String>,
    bit_rate: Option<String>,
    disposition: Option<FfprobeDisposition>,
    tags: Option<FfprobeTags>,
}

#[derive(Deserialize)]
struct FfprobeDisposition {
    default: Option<i32>,
    forced: Option<i32>,
}

#[derive(Deserialize)]
struct FfprobeTags {
    language: Option<String>,
    title: Option<String>,
}

#[derive(Deserialize)]
struct FfprobeFormat {
    format_name: Option<String>,
    duration: Option<String>,
    bit_rate: Option<String>,
}
