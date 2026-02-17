use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::Path;
use tokio::process::Command;
use tracing::{debug, warn};

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
}

/// Run ffprobe on a file and extract stream/format info.
pub async fn probe_file(ffprobe_path: &str, file_path: &Path) -> Result<ProbeResult> {
    let output = Command::new(ffprobe_path)
        .args([
            "-v", "quiet",
            "-print_format", "json",
            "-show_format",
            "-show_streams",
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
        f.duration.as_deref().and_then(|d| {
            d.parse::<f64>().ok().map(|s| (s * 1000.0) as u64)
        })
    });

    let bitrate_kbps = json.format.as_ref().and_then(|f| {
        f.bit_rate.as_deref().and_then(|b| {
            b.parse::<u64>().ok().map(|bps| (bps / 1000) as u32)
        })
    });

    // Find the first video stream
    let video_stream = json.streams.iter().find(|s| s.codec_type.as_deref() == Some("video"));
    let video_codec = video_stream.and_then(|s| s.codec_name.clone());
    let width = video_stream.and_then(|s| s.width);
    let height = video_stream.and_then(|s| s.height);

    // Find the first audio stream
    let audio_stream = json.streams.iter().find(|s| s.codec_type.as_deref() == Some("audio"));
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
                bit_depth: s.bits_per_raw_sample.as_deref().and_then(|b| b.parse().ok()),
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

    debug!(
        "Probed {}: container={:?} video={:?} audio={:?} {}x{} duration={}ms streams={}",
        file_path.display(),
        container_format,
        video_codec,
        audio_codec,
        width.unwrap_or(0),
        height.unwrap_or(0),
        duration_ms.unwrap_or(0),
        streams.len(),
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
    })
}

#[derive(Deserialize)]
struct FfprobeOutput {
    #[serde(default)]
    streams: Vec<FfprobeStream>,
    format: Option<FfprobeFormat>,
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
