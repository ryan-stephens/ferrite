use axum::body::Body;
use axum::http::{header, StatusCode};
use axum::response::Response;
use std::path::Path;
use std::pin::Pin;
use std::process::Stdio;
use std::task::{Context, Poll};
use tokio::io::AsyncRead;
use tokio::process::{Child, Command};
use tokio_util::io::ReaderStream;
use ferrite_transcode::hwaccel::EncoderProfile;
use tracing::{debug, info, warn};

/// Wraps an FFmpeg child process stdout so that when the stream is dropped
/// (e.g. client disconnects), the FFmpeg process is killed immediately.
/// This prevents orphaned FFmpeg processes from consuming CPU/memory.
struct ChildGuardReader {
    inner: tokio::process::ChildStdout,
    child: Option<Child>,
}

impl ChildGuardReader {
    fn new(stdout: tokio::process::ChildStdout, child: Child) -> Self {
        Self {
            inner: stdout,
            child: Some(child),
        }
    }
}

impl AsyncRead for ChildGuardReader {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.inner).poll_read(cx, buf)
    }
}

impl Drop for ChildGuardReader {
    fn drop(&mut self) {
        if let Some(mut child) = self.child.take() {
            // Try to kill the process. If it already exited, this is a no-op.
            match child.try_wait() {
                Ok(Some(_)) => {
                    debug!("FFmpeg process already exited");
                }
                _ => {
                    info!("Killing FFmpeg process (client disconnected or stream ended)");
                    let _ = child.start_kill();
                }
            }
        }
    }
}

/// Probe the nearest keyframe position before the requested seek time.
/// Returns the actual keyframe timestamp so we can report it to the frontend.
/// Uses ffprobe to read keyframe timestamps from the video stream.
pub async fn find_keyframe_before(
    ffprobe_path: &str,
    file_path: &Path,
    target_secs: f64,
) -> Option<f64> {
    // Use ffprobe to find keyframes near the target. We read packets around the
    // target time and look for the last keyframe at or before the target.
    let output = Command::new(ffprobe_path)
        .args([
            "-hide_banner",
            "-loglevel", "error",
            "-read_intervals", &format!("{}%+5", (target_secs - 15.0).max(0.0)),
            "-select_streams", "v:0",
            "-show_packets",
            "-show_entries", "packet=pts_time,flags",
            "-of", "csv=print_section=0",
        ])
        .arg(file_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .await
        .ok()?;

    let text = String::from_utf8_lossy(&output.stdout);
    let mut last_keyframe: Option<f64> = None;

    for line in text.lines() {
        // Format: pts_time,flags  e.g. "120.120,K__"
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() >= 2 {
            if let Ok(pts) = parts[0].parse::<f64>() {
                if parts[1].starts_with('K') && pts <= target_secs + 0.5 {
                    last_keyframe = Some(pts);
                }
            }
        }
    }

    last_keyframe
}

/// Serve a media file by remuxing to MP4 with all streams copied (zero re-encoding).
/// Both video and audio codecs are browser-compatible — only the container needs changing
/// (e.g. MKV → fMP4). This is the fastest transcode path: no CPU cost for codec work.
///
/// `duration_secs` is the known total duration from ffprobe — FFmpeg uses it to write
/// proper duration metadata into the fragmented MP4 header.
///
/// `start_secs` allows seeking — FFmpeg will fast-seek to this position.
#[allow(clippy::too_many_arguments)]
pub async fn serve_remux(
    ffmpeg_path: &str,
    ffprobe_path: &str,
    file_path: &Path,
    duration_secs: Option<f64>,
    start_secs: Option<f64>,
    subtitle_path: Option<&Path>,
    encoder: &EncoderProfile,
    audio_stream_index: Option<u32>,
    video_codec: Option<&str>,
) -> Result<Response, StatusCode> {
    // If subtitle burn-in is requested, we need to re-encode video — fall back to audio transcode
    if subtitle_path.is_some() {
        return serve_audio_transcode(
            ffmpeg_path, ffprobe_path, file_path, duration_secs, start_secs, subtitle_path, encoder, audio_stream_index,
        ).await;
    }

    if !file_path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    let start = start_secs.unwrap_or(0.0);

    let actual_start = if start > 0.1 {
        match find_keyframe_before(ffprobe_path, file_path, start).await {
            Some(kf_time) => {
                info!(
                    "Remux seek to {:.1}s → snapped to keyframe at {:.3}s (delta {:.1}s)",
                    start, kf_time, start - kf_time
                );
                kf_time
            }
            None => {
                info!("No keyframe found near {:.1}s, using requested time", start);
                start
            }
        }
    } else {
        start
    };

    info!(
        "Remux: {} (video copy + audio copy, start={:.1}s, duration={:?}s)",
        file_path.display(),
        actual_start,
        duration_secs,
    );

    let mut args: Vec<String> = vec![
        "-hide_banner".into(),
        "-nostdin".into(),
    ];

    if actual_start > 0.1 {
        args.push("-ss".into());
        args.push(format!("{:.3}", actual_start));
        args.push("-noaccurate_seek".into());
    }

    args.push("-i".into());
    args.push(file_path.to_string_lossy().to_string());

    if actual_start > 0.1 {
        args.push("-avoid_negative_ts".into());
        args.push("make_zero".into());
    }

    // Map first video and selected audio stream — copy both without re-encoding
    let audio_map = format!("0:a:{}", audio_stream_index.unwrap_or(0));
    args.extend([
        "-map".into(), "0:v:0".into(),
        "-map".into(), audio_map,
        "-c:v".into(), "copy".into(),
        "-c:a".into(), "copy".into(),
    ]);

    // Choose output container based on video codec:
    // VP9/VP8 → WebM (VP9 doesn't work in MP4 for browser playback)
    // Everything else → fragmented MP4
    let use_webm = video_codec
        .map(|c| matches!(c.to_lowercase().as_str(), "vp9" | "vp8"))
        .unwrap_or(false);

    if use_webm {
        args.extend(["-f".into(), "webm".into()]);
    } else {
        args.extend([
            "-movflags".into(),
            "frag_keyframe+empty_moov+default_base_moof".into(),
            "-f".into(),
            "mp4".into(),
        ]);
    }
    args.push("pipe:1".into());

    debug!("ffmpeg remux args: {:?}", args);

    let mut child = Command::new(ffmpeg_path)
        .args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| {
            warn!("Failed to spawn ffmpeg for remux: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let stdout = child.stdout.take().ok_or_else(|| {
        warn!("Failed to capture ffmpeg stdout");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let guarded = ChildGuardReader::new(stdout, child);
    let stream = ReaderStream::new(guarded);
    let body = Body::from_stream(stream);

    let content_type = if use_webm { "video/webm" } else { "video/mp4" };
    let mut builder = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, content_type)
        .header(header::TRANSFER_ENCODING, "chunked")
        .header("X-Seek-Actual", format!("{:.3}", actual_start));

    if let Some(dur) = duration_secs {
        let remaining = dur - actual_start;
        builder = builder
            .header("X-Content-Duration", format!("{:.3}", remaining))
            .header("X-Total-Duration", format!("{:.3}", dur));
    }

    builder = builder.header(
        header::ACCESS_CONTROL_EXPOSE_HEADERS,
        "X-Seek-Actual, X-Content-Duration, X-Total-Duration",
    );

    Ok(builder.body(body).unwrap())
}

/// Serve a media file by remuxing to MP4 with audio transcoded to AAC.
/// Video stream is copied without re-encoding (fast). Audio is transcoded to AAC stereo.
/// The output is streamed directly to the client via FFmpeg stdout pipe.
///
/// `duration_secs` is the known total duration from ffprobe — FFmpeg uses it to write
/// proper duration metadata into the fragmented MP4 header so the browser shows a
/// correct timeline/scrubber.
///
/// `start_secs` allows seeking — FFmpeg will fast-seek to this position before encoding.
#[allow(clippy::too_many_arguments)]
pub async fn serve_audio_transcode(
    ffmpeg_path: &str,
    ffprobe_path: &str,
    file_path: &Path,
    duration_secs: Option<f64>,
    start_secs: Option<f64>,
    subtitle_path: Option<&Path>,
    _encoder: &EncoderProfile,
    audio_stream_index: Option<u32>,
) -> Result<Response, StatusCode> {
    if !file_path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    let start = start_secs.unwrap_or(0.0);

    // For seeks, find the actual keyframe position so we start cleanly.
    // This avoids the "frozen video" problem: with -c:v copy, FFmpeg outputs
    // from the nearest keyframe but the browser can't render partial GOPs.
    // By seeking exactly to a keyframe, the first frame is immediately renderable.
    let actual_start = if start > 0.1 {
        match find_keyframe_before(ffprobe_path, file_path, start).await {
            Some(kf_time) => {
                info!(
                    "Seek to {:.1}s → snapped to keyframe at {:.3}s (delta {:.1}s)",
                    start, kf_time, start - kf_time
                );
                kf_time
            }
            None => {
                info!("No keyframe found near {:.1}s, using requested time", start);
                start
            }
        }
    } else {
        start
    };

    info!(
        "Audio transcode: {} (video copy + audio->AAC, start={:.1}s, duration={:?}s)",
        file_path.display(),
        actual_start,
        duration_secs,
    );

    // Build ffmpeg args dynamically
    let mut args: Vec<String> = vec![
        "-hide_banner".into(),
        "-nostdin".into(),
    ];

    // Seek strategy: fast-seek to keyframe position (before -i).
    //
    // Since we pre-calculated the exact keyframe position via ffprobe,
    // fast-seek will land right on it. Both video and audio start from
    // the same keyframe, so there's no A/V desync. And since the first
    // frame is a keyframe, the browser can render immediately — no freeze.
    //
    // -noaccurate_seek prevents FFmpeg from trying to trim to an exact
    // sub-keyframe position (which would cause the frozen frames issue
    // with -c:v copy).
    //
    // -avoid_negative_ts make_zero ensures output timestamps start at 0.
    if actual_start > 0.1 {
        args.push("-ss".into());
        args.push(format!("{:.3}", actual_start));
        args.push("-noaccurate_seek".into());
    }

    args.push("-i".into());
    args.push(file_path.to_string_lossy().to_string());

    // Ensure clean timestamps from the seek point
    if actual_start > 0.1 {
        args.push("-avoid_negative_ts".into());
        args.push("make_zero".into());
    }

    // Map first video and selected audio stream
    let audio_map = format!("0:a:{}", audio_stream_index.unwrap_or(0));
    args.extend([
        "-map".into(), "0:v:0".into(),
        "-map".into(), audio_map,
    ]);

    // Video: copy (no re-encoding) unless subtitle burn-in is requested
    if let Some(sub_path) = subtitle_path {
        // Subtitle burn-in requires video re-encoding (can't filter with -c:v copy).
        // Use the subtitles filter to render text onto the video frames.
        // Note: subtitle filter requires CPU-side frames, so always use software encoder here.
        let sub_path_escaped = escape_ffmpeg_filter_path(&sub_path.to_string_lossy());
        args.extend([
            "-vf".into(),
            format!("subtitles={}", sub_path_escaped),
        ]);
        args.extend(EncoderProfile::software().video_encode_args());
        info!("Subtitle burn-in enabled (software encode): {}", sub_path.display());
    } else {
        args.extend(["-c:v".into(), "copy".into()]);
    }

    // Audio: transcode to AAC stereo
    args.extend([
        "-c:a".into(), "aac".into(),
        "-b:a".into(), "192k".into(),
        "-ac".into(), "2".into(),
    ]);

    // Fragmented MP4 for streaming — empty_moov allows playback before the
    // entire file is written, frag_keyframe creates fragments at keyframes
    args.extend([
        "-movflags".into(),
        "frag_keyframe+empty_moov+default_base_moof".into(),
    ]);

    args.extend(["-f".into(), "mp4".into()]);
    args.push("pipe:1".into());

    debug!("ffmpeg args: {:?}", args);

    let mut child = Command::new(ffmpeg_path)
        .args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| {
            warn!("Failed to spawn ffmpeg: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let stdout = child.stdout.take().ok_or_else(|| {
        warn!("Failed to capture ffmpeg stdout");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Wrap stdout + child in ChildGuardReader so FFmpeg is killed on client disconnect
    let guarded = ChildGuardReader::new(stdout, child);
    let stream = ReaderStream::new(guarded);
    let body = Body::from_stream(stream);

    let mut builder = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "video/mp4")
        .header(header::TRANSFER_ENCODING, "chunked")
        // Tell the frontend the actual seek position (may differ from requested)
        // so it can display the correct time on the scrubber.
        .header("X-Seek-Actual", format!("{:.3}", actual_start));

    // Pass duration as a custom header so the frontend can set it on the video element.
    // The browser can't read it from the fMP4 stream because empty_moov doesn't include
    // a duration in the initial moov atom. The frontend reads this header and sets
    // video.duration manually via a MediaSource workaround, or shows a custom timeline.
    if let Some(dur) = duration_secs {
        let remaining = dur - actual_start;
        builder = builder
            .header("X-Content-Duration", format!("{:.3}", remaining))
            .header("X-Total-Duration", format!("{:.3}", dur));
    }

    // Expose custom headers to the browser JS (CORS-safe)
    builder = builder.header(
        header::ACCESS_CONTROL_EXPOSE_HEADERS,
        "X-Seek-Actual, X-Content-Duration, X-Total-Duration",
    );

    Ok(builder.body(body).unwrap())
}

/// Serve a media file with full video + audio transcode.
/// Video is re-encoded to H.264 (for HEVC/AV1/other incompatible codecs).
/// Audio is transcoded to AAC stereo. Output is streamed as fragmented MP4.
///
/// This is significantly more CPU-intensive than audio-only transcode since
/// every video frame must be decoded and re-encoded.
#[allow(clippy::too_many_arguments)]
pub async fn serve_full_transcode(
    ffmpeg_path: &str,
    ffprobe_path: &str,
    file_path: &Path,
    duration_secs: Option<f64>,
    start_secs: Option<f64>,
    subtitle_path: Option<&Path>,
    encoder: &EncoderProfile,
    pixel_format: Option<&str>,
    audio_stream_index: Option<u32>,
    color_transfer: Option<&str>,
    color_primaries: Option<&str>,
) -> Result<Response, StatusCode> {
    if !file_path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    let start = start_secs.unwrap_or(0.0);

    // For seeks, find the actual keyframe position so we start cleanly.
    let actual_start = if start > 0.1 {
        match find_keyframe_before(ffprobe_path, file_path, start).await {
            Some(kf_time) => {
                info!(
                    "Full transcode seek to {:.1}s → snapped to keyframe at {:.3}s",
                    start, kf_time
                );
                kf_time
            }
            None => {
                info!("No keyframe found near {:.1}s, using requested time", start);
                start
            }
        }
    } else {
        start
    };

    info!(
        "Full transcode: {} (video→H.264 + audio→AAC, start={:.1}s, duration={:?}s)",
        file_path.display(),
        actual_start,
        duration_secs,
    );

    // Subtitle burn-in requires CPU-side frame access, so fall back to software.
    let effective_encoder = if subtitle_path.is_some() && encoder.is_hardware() {
        info!("Subtitle burn-in active — falling back to software encoder");
        EncoderProfile::software()
    } else {
        encoder.clone()
    };

    let mut args: Vec<String> = vec![
        "-hide_banner".into(),
        "-nostdin".into(),
    ];

    // HW-accelerated decoding args (before -i)
    if subtitle_path.is_none() {
        args.extend(effective_encoder.hw_input_args().iter().cloned());
    }

    // Fast-seek before input
    if actual_start > 0.1 {
        args.push("-ss".into());
        args.push(format!("{:.3}", actual_start));
    }

    args.push("-i".into());
    args.push(file_path.to_string_lossy().to_string());

    if actual_start > 0.1 {
        args.push("-avoid_negative_ts".into());
        args.push("make_zero".into());
    }

    // Map first video and selected audio stream
    let audio_map = format!("0:a:{}", audio_stream_index.unwrap_or(0));
    args.extend([
        "-map".into(), "0:v:0".into(),
        "-map".into(), audio_map,
    ]);

    // Build video filter chain: HDR tone-mapping + subtitle burn-in
    let mut vf_parts: Vec<String> = Vec::new();

    let is_high_bit = pixel_format
        .map(ferrite_transcode::tonemap::is_high_bit_depth)
        .unwrap_or(false);
    let needs_tonemap = is_high_bit
        && ferrite_transcode::tonemap::is_true_hdr(color_transfer, color_primaries);
    if needs_tonemap {
        info!("True HDR detected (pix={}, transfer={:?}, primaries={:?}), applying tone-mapping (full transcode)",
            pixel_format.unwrap_or("unknown"), color_transfer, color_primaries);
        vf_parts.push(ferrite_transcode::tonemap::tonemap_filter());
    } else if is_high_bit {
        info!("10-bit SDR detected (pix={}, transfer={:?}), applying bit-depth conversion only (full transcode)",
            pixel_format.unwrap_or("unknown"), color_transfer);
        vf_parts.push(ferrite_transcode::tonemap::bit_depth_filter());
    }

    if let Some(sub_path) = subtitle_path {
        let sub_path_escaped = escape_ffmpeg_filter_path(&sub_path.to_string_lossy());
        vf_parts.push(format!("subtitles={}", sub_path_escaped));
        info!("Subtitle burn-in enabled (full transcode): {}", sub_path.display());
    }

    if !vf_parts.is_empty() {
        args.extend(["-vf".into(), vf_parts.join(",")]);
    }

    // Video encoder args — skip -pix_fmt when a filter already handles format conversion
    // (both tone-mapping and bit-depth conversion output yuv420p via the filter chain)
    if is_high_bit {
        args.extend(effective_encoder.video_encode_args_no_pix_fmt());
    } else {
        args.extend(effective_encoder.video_encode_args());
    }

    // Audio: transcode to AAC stereo
    args.extend([
        "-c:a".into(), "aac".into(),
        "-b:a".into(), "192k".into(),
        "-ac".into(), "2".into(),
    ]);

    // Fragmented MP4 for streaming
    args.extend([
        "-movflags".into(),
        "frag_keyframe+empty_moov+default_base_moof".into(),
    ]);

    args.extend(["-f".into(), "mp4".into()]);
    args.push("pipe:1".into());

    debug!("ffmpeg full transcode args: {:?}", args);

    let mut child = Command::new(ffmpeg_path)
        .args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| {
            warn!("Failed to spawn ffmpeg for full transcode: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let stdout = child.stdout.take().ok_or_else(|| {
        warn!("Failed to capture ffmpeg stdout");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Wrap stdout + child in ChildGuardReader so FFmpeg is killed on client disconnect
    let guarded = ChildGuardReader::new(stdout, child);
    let stream = ReaderStream::new(guarded);
    let body = Body::from_stream(stream);

    let mut builder = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "video/mp4")
        .header(header::TRANSFER_ENCODING, "chunked")
        .header("X-Seek-Actual", format!("{:.3}", actual_start));

    if let Some(dur) = duration_secs {
        let remaining = dur - actual_start;
        builder = builder
            .header("X-Content-Duration", format!("{:.3}", remaining))
            .header("X-Total-Duration", format!("{:.3}", dur));
    }

    builder = builder.header(
        header::ACCESS_CONTROL_EXPOSE_HEADERS,
        "X-Seek-Actual, X-Content-Duration, X-Total-Duration",
    );

    Ok(builder.body(body).unwrap())
}

/// Escape a file path for use in FFmpeg's `-vf subtitles=` filter.
/// FFmpeg filter syntax uses `:`, `\`, `'`, and `[` as special characters.
/// On Windows, paths contain `\` and `:` which must be escaped.
pub fn escape_ffmpeg_filter_path(path: &str) -> String {
    let mut escaped = String::with_capacity(path.len() * 2);
    for ch in path.chars() {
        match ch {
            // FFmpeg filter path special characters that need escaping
            ':' | '\\' | '\'' | '[' | ']' => {
                escaped.push('\\');
                escaped.push(ch);
            }
            _ => escaped.push(ch),
        }
    }
    // Wrap in single quotes for safety
    format!("'{}'", escaped)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_unix_path() {
        let result = escape_ffmpeg_filter_path("/media/movies/file.srt");
        assert_eq!(result, "'/media/movies/file.srt'");
    }

    #[test]
    fn test_escape_windows_path() {
        let result = escape_ffmpeg_filter_path("C:\\Users\\ryan\\Videos\\sub.srt");
        assert_eq!(result, "'C\\:\\\\Users\\\\ryan\\\\Videos\\\\sub.srt'");
    }

    #[test]
    fn test_escape_brackets_and_quotes() {
        let result = escape_ffmpeg_filter_path("/media/[Special]/file's.srt");
        assert_eq!(result, "'/media/\\[Special\\]/file\\'s.srt'");
    }
}
