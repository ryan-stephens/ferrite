use anyhow::{anyhow, Result};
use dashmap::DashMap;
use ferrite_transcode::hwaccel::EncoderProfile;
use ferrite_transcode::variants::QualityVariant;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;
use tokio::process::Child;
use tokio::sync::{Mutex, Semaphore};
use tracing::{debug, info, warn};

// ---------------------------------------------------------------------------
// Session
// ---------------------------------------------------------------------------

pub struct HlsSession {
    pub session_id: String,
    pub media_id: String,
    pub output_dir: PathBuf,
    pub segment_duration: u64,
    ffmpeg_handle: Mutex<Option<Child>>,
    pub created_at: Instant,
    /// Epoch-millis timestamp of last access (playlist or segment request).
    /// AtomicU64 avoids async Mutex contention on the hot path.
    last_accessed_epoch_ms: std::sync::atomic::AtomicU64,
    /// Epoch-millis timestamp of last segment/init file served.
    /// Used to kill FFmpeg promptly when the client stops consuming (e.g. paused).
    last_segment_request_epoch_ms: std::sync::atomic::AtomicU64,
    /// Set to true when FFmpeg stderr indicates a fatal error (corrupt file, disk full, etc.).
    /// Causes get_segment() to short-circuit instead of waiting the full 30s timeout.
    pub ffmpeg_failed: std::sync::atomic::AtomicBool,
    /// Cached count of .m4s segment files on disk, updated when segments are served.
    /// Avoids synchronous std::fs::read_dir on every buffered_secs() call.
    segment_count: std::sync::atomic::AtomicU64,
    pub duration_secs: Option<f64>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub bitrate_kbps: Option<u32>,
    /// The time offset (in seconds) where FFmpeg started transcoding.
    /// Segments in this session represent media time starting at this offset.
    pub start_secs: f64,
    /// Quality variant label (e.g. "1080p", "720p"). None for legacy single-variant.
    pub variant_label: Option<String>,
    /// Bandwidth in bps for this variant (for master playlist).
    pub bandwidth_bps: u64,
    /// RFC6381 video codec string for master playlist CODECS tag.
    pub video_codec_rfc6381: String,
    /// RFC6381 audio codec string for master playlist CODECS tag.
    pub audio_codec_rfc6381: String,
    /// True when video was copied (-c:v copy) rather than re-encoded.
    /// With copy, fmp4 segments retain original file PTS so the frontend
    /// must NOT add an offset (videoRef.currentTime is already absolute).
    pub video_copied: bool,
    /// True when this session was created as a single-variant for fast initial TTFF
    /// and should be promoted to the full ABR ladder on the next master playlist poll.
    /// Set to false for seek-created sessions (which should stay single-variant).
    pub awaiting_promotion: std::sync::atomic::AtomicBool,
}

/// RFC6381 video codec emitted by the HLS pipeline.
/// Video is currently always H.264 when transcoded, and passthrough is only
/// enabled for H.264 sources.
fn output_video_codec_rfc6381(source_video_codec: Option<&str>, video_copied: bool) -> String {
    if video_copied
        && source_video_codec
            .map(|c| c.eq_ignore_ascii_case("h264") || c.eq_ignore_ascii_case("avc1"))
            .unwrap_or(false)
    {
        return "avc1.64001f".to_string();
    }
    "avc1.64001f".to_string()
}

/// RFC6381 audio codec emitted by the HLS pipeline.
/// Unsupported source codecs are transcoded to AAC-LC stereo.
fn output_audio_codec_rfc6381(source_audio_codec: Option<&str>) -> String {
    let Some(codec) = source_audio_codec else {
        return "mp4a.40.2".to_string();
    };

    if !ferrite_transcode::audio::can_passthrough(codec) {
        return "mp4a.40.2".to_string();
    }

    match codec.to_lowercase().as_str() {
        "aac" => "mp4a.40.2".to_string(),
        "mp3" => "mp4a.40.34".to_string(),
        "opus" => "opus".to_string(),
        "flac" => "flac".to_string(),
        "alac" => "alac".to_string(),
        _ => "mp4a.40.2".to_string(),
    }
}

/// Current wall-clock time as milliseconds since UNIX epoch.
fn epoch_ms_now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

impl HlsSession {
    /// Mark the session as recently accessed (lock-free).
    pub fn touch(&self) {
        self.last_accessed_epoch_ms
            .store(epoch_ms_now(), std::sync::atomic::Ordering::Release);
    }

    /// Seconds since last access (lock-free).
    pub fn idle_secs(&self) -> u64 {
        let last = self
            .last_accessed_epoch_ms
            .load(std::sync::atomic::Ordering::Acquire);
        epoch_ms_now().saturating_sub(last) / 1000
    }

    /// Seconds since a segment or init file was last served to the client (lock-free).
    pub fn segment_idle_secs(&self) -> u64 {
        let last = self
            .last_segment_request_epoch_ms
            .load(std::sync::atomic::Ordering::Acquire);
        epoch_ms_now().saturating_sub(last) / 1000
    }

    /// Record that a segment was just served (lock-free).
    fn touch_segment(&self) {
        let now = epoch_ms_now();
        self.last_accessed_epoch_ms
            .store(now, std::sync::atomic::Ordering::Release);
        self.last_segment_request_epoch_ms
            .store(now, std::sync::atomic::Ordering::Release);
    }

    /// How many seconds of content have been transcoded so far (based on cached segment count).
    /// Used to decide whether a seek target is already within the buffered range.
    pub fn buffered_secs(&self) -> f64 {
        let count = self
            .segment_count
            .load(std::sync::atomic::Ordering::Acquire);
        count as f64 * self.segment_duration as f64
    }

    /// Scan the output directory and update the cached segment count.
    /// Called after a segment is confirmed ready (listed in playlist).
    pub fn refresh_segment_count(&self) {
        let mut count = 0u64;
        if let Ok(rd) = std::fs::read_dir(&self.output_dir) {
            for entry in rd.flatten() {
                let name = entry.file_name();
                let s = name.to_string_lossy();
                if s.starts_with("seg_") && s.ends_with(".m4s") {
                    count += 1;
                }
            }
        }
        self.segment_count
            .store(count, std::sync::atomic::Ordering::Release);
    }

    /// Parse the playlist from disk to determine the actual available segment range.
    /// Returns `(available_start_secs, available_end_secs)` relative to the media timeline.
    /// Uses segment filenames (seg_NNN.m4s) rather than EXTINF count because the playlist
    /// is a sliding window (hls_list_size + delete_segments) — the count is capped but
    /// segment numbers keep incrementing.
    pub async fn playlist_available_range(&self) -> Option<(f64, f64)> {
        let playlist_path = self.output_dir.join("playlist.m3u8");
        let content = tokio::fs::read_to_string(&playlist_path).await.ok()?;

        let mut min_seg: Option<u64> = None;
        let mut max_seg: Option<u64> = None;

        for line in content.lines() {
            let line = line.trim();
            if let Some(num_str) = line
                .strip_prefix("seg_")
                .and_then(|s| s.strip_suffix(".m4s"))
            {
                if let Ok(n) = num_str.parse::<u64>() {
                    min_seg = Some(min_seg.map_or(n, |m: u64| m.min(n)));
                    max_seg = Some(max_seg.map_or(n, |m: u64| m.max(n)));
                }
            }
        }

        let seg_dur = self.segment_duration as f64;
        Some((
            self.start_secs + min_seg? as f64 * seg_dur,
            self.start_secs + (max_seg? + 1) as f64 * seg_dur,
        ))
    }

    /// Kill the FFmpeg process without destroying the session or its files.
    /// Called when the client stops consuming segments (paused) to free resources.
    pub async fn kill_ffmpeg(&self) {
        info!("kill_ffmpeg called for session {}", self.session_id);
        if let Some(mut child) = self.ffmpeg_handle.lock().await.take() {
            info!(
                "kill_ffmpeg took child handle for session {}",
                self.session_id
            );
            // On Unix: send SIGTERM so FFmpeg can flush write buffers and close
            // the playlist cleanly, then wait up to 2 seconds for it to exit,
            // then send SIGKILL if it's still alive.
            // On Windows: kill immediately (no SIGTERM concept).
            #[cfg(unix)]
            {
                if let Some(pid) = child.id() {
                    info!(
                        "kill_ffmpeg sending SIGTERM to pid {} for session {}",
                        pid, self.session_id
                    );
                    // SAFETY: pid is a valid process ID from a child we own.
                    unsafe { libc::kill(pid as libc::pid_t, libc::SIGTERM) };

                    // Wait up to 2 seconds for graceful exit
                    let wait_fut = child.wait();
                    if let Ok(Ok(status)) =
                        tokio::time::timeout(std::time::Duration::from_secs(2), wait_fut).await
                    {
                        info!(
                            "FFmpeg exited gracefully for session {} with status {}",
                            self.session_id, status
                        );
                        return;
                    }

                    // Force kill if it didn't exit
                    info!(
                        "FFmpeg did not exit gracefully, sending SIGKILL for session {}",
                        self.session_id
                    );
                    let _ = child.kill().await;
                    let _ = child.wait().await;
                    return;
                } else {
                    info!(
                        "kill_ffmpeg child.id() was None, process already exited for session {}",
                        self.session_id
                    );
                }
            }
            // Windows / no pid: kill immediately.
            info!(
                "kill_ffmpeg killing immediately for session {}",
                self.session_id
            );
            let _ = child.kill().await;
            let _ = child.wait().await;
        } else {
            info!(
                "kill_ffmpeg handle was already None for session {}",
                self.session_id
            );
        }
    }

    /// Check if the FFmpeg process is still running.
    /// Returns `false` if FFmpeg has exited (crashed or finished encoding).
    pub async fn is_ffmpeg_alive(&self) -> bool {
        let mut guard = self.ffmpeg_handle.lock().await;
        match guard.as_mut() {
            None => false,
            Some(child) => {
                // try_wait returns Ok(Some(status)) if exited, Ok(None) if still running
                match child.try_wait() {
                    Ok(Some(_status)) => {
                        // Process has exited — take it out so we don't check again
                        *guard = None;
                        false
                    }
                    Ok(None) => true, // Still running
                    Err(_) => false,  // Error checking — assume dead
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Session Manager
// ---------------------------------------------------------------------------

/// Snapshot of a single active HLS session for the admin dashboard.
pub struct ActiveSessionInfo {
    pub session_id: String,
    pub media_id: String,
    pub variant_label: Option<String>,
    pub start_secs: f64,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub bitrate_kbps: Option<u32>,
    pub idle_secs: u64,
    pub age_secs: u64,
}

pub struct HlsSessionManager {
    sessions: DashMap<String, Arc<HlsSession>>,
    /// Map from media_id → session_id for reuse (legacy single-variant)
    media_sessions: DashMap<String, String>,
    /// Map from media_id → list of session_ids for multi-variant ABR
    media_variant_sessions: DashMap<String, Vec<String>>,
    /// Per-owner-key semaphore (capacity 1) to prevent concurrent session creation
    /// races for the same owner key, which could orphan FFmpeg processes.
    creation_locks: DashMap<String, Arc<Semaphore>>,
    cache_dir: PathBuf,
    ffmpeg_path: String,
    segment_duration: u64,
    playlist_window_segments: u32,
    session_timeout_secs: u64,
    /// Seconds of no segment requests before FFmpeg is killed (client paused).
    ffmpeg_idle_secs: u64,
    encoder: EncoderProfile,
}

impl HlsSessionManager {
    /// Build an ownership key for session maps.
    /// If a playback session id is provided, sessions are isolated per playback;
    /// otherwise falls back to media-level ownership for backward compatibility.
    pub fn owner_key(media_id: &str, playback_session_id: Option<&str>) -> String {
        match playback_session_id {
            Some(pid) if !pid.is_empty() => format!("{}::{}", media_id, pid),
            _ => media_id.to_string(),
        }
    }

    pub fn new(
        cache_dir: PathBuf,
        ffmpeg_path: String,
        segment_duration: u64,
        playlist_window_segments: u32,
        session_timeout_secs: u64,
        ffmpeg_idle_secs: u64,
        encoder: EncoderProfile,
    ) -> Self {
        Self {
            sessions: DashMap::new(),
            media_sessions: DashMap::new(),
            media_variant_sessions: DashMap::new(),
            creation_locks: DashMap::new(),
            cache_dir,
            ffmpeg_path,
            segment_duration,
            playlist_window_segments,
            session_timeout_secs,
            ffmpeg_idle_secs,
            encoder,
        }
    }

    /// Get or create an HLS session for a media item.
    /// Returns the session. Creates FFmpeg process if new.
    /// `start_secs` is the time offset to start transcoding from (0.0 = beginning).
    /// If a session already exists for this media with the same start offset, reuse it.
    #[allow(clippy::too_many_arguments)]
    pub async fn get_or_create_session(
        &self,
        media_id: &str,
        file_path: &Path,
        duration_secs: Option<f64>,
        width: Option<u32>,
        height: Option<u32>,
        bitrate_kbps: Option<u32>,
        start_secs: f64,
        requested_secs: f64,
        subtitle_path: Option<&Path>,
        pixel_format: Option<&str>,
        audio_stream_index: Option<u32>,
        frame_rate: Option<&str>,
        audio_codec: Option<&str>,
        video_codec: Option<&str>,
        color_transfer: Option<&str>,
        color_primaries: Option<&str>,
    ) -> Result<Arc<HlsSession>> {
        // Acquire per-media creation lock to prevent concurrent requests for the same
        // media from each spawning a separate FFmpeg process, where the second would
        // overwrite the first in media_sessions, orphaning the first FFmpeg process.
        let lock = self
            .creation_locks
            .entry(media_id.to_string())
            .or_insert_with(|| Arc::new(Semaphore::new(1)))
            .clone();
        let _guard = lock
            .acquire()
            .await
            .map_err(|e| anyhow!("Lock error: {}", e))?;

        // Check for existing session for this media
        if let Some(sid) = self.media_sessions.get(media_id) {
            if let Some(session) = self.sessions.get(sid.value()) {
                // Reuse if the start offset is close enough (within one segment)
                let offset_diff = (session.start_secs - start_secs).abs();
                if offset_diff < self.segment_duration as f64 {
                    session.touch();
                    return Ok(session.clone());
                }
                // Different start position — destroy old session first
                let old_sid = sid.clone();
                drop(session);
                drop(sid);
                self.destroy_session(&old_sid).await;
            }
        }

        self.create_session(
            media_id,
            file_path,
            duration_secs,
            width,
            height,
            bitrate_kbps,
            start_secs,
            requested_secs,
            subtitle_path,
            None,
            pixel_format,
            audio_stream_index,
            frame_rate,
            audio_codec,
            video_codec,
            color_transfer,
            color_primaries,
        )
        .await
    }

    /// Always create a fresh HLS session (destroys any existing session for this media).
    /// Used for seeking to a new position.
    /// If `variant` is provided, FFmpeg will scale and constrain bitrate to that quality level.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_session(
        &self,
        media_id: &str,
        file_path: &Path,
        duration_secs: Option<f64>,
        width: Option<u32>,
        height: Option<u32>,
        bitrate_kbps: Option<u32>,
        start_secs: f64,
        requested_secs: f64,
        subtitle_path: Option<&Path>,
        variant: Option<&QualityVariant>,
        pixel_format: Option<&str>,
        audio_stream_index: Option<u32>,
        frame_rate: Option<&str>,
        audio_codec: Option<&str>,
        video_codec: Option<&str>,
        color_transfer: Option<&str>,
        color_primaries: Option<&str>,
    ) -> Result<Arc<HlsSession>> {
        // Destroy any existing session for this media (single-variant path)
        if variant.is_none() {
            if let Some(sid) = self.media_sessions.get(media_id) {
                let old_sid = sid.clone();
                drop(sid);
                self.destroy_session(&old_sid).await;
            }
        }

        // Create new session
        let session_id = uuid::Uuid::new_v4().to_string();
        let output_dir = self.cache_dir.join(&session_id);
        tokio::fs::create_dir_all(&output_dir).await?;

        let variant_label = variant.map(|v| v.label.clone());
        info!(
            "Creating HLS session {} for media {} at {:.1}s variant={} ({})",
            session_id,
            media_id,
            start_secs,
            variant_label.as_deref().unwrap_or("native"),
            file_path.display()
        );

        // Spawn FFmpeg (with -ss if starting from a non-zero position)
        let (child, stderr, video_copied) = self
            .spawn_ffmpeg(
                file_path,
                &output_dir,
                start_secs,
                requested_secs,
                subtitle_path,
                variant,
                height,
                pixel_format,
                audio_stream_index,
                frame_rate,
                audio_codec,
                video_codec,
                color_transfer,
                color_primaries,
            )
            .await?;

        let (session_w, session_h, session_bw) = match variant {
            Some(v) => (Some(v.width), Some(v.height), v.bandwidth_bps),
            None => (
                width,
                height,
                bitrate_kbps.map(|k| (k as u64) * 1000).unwrap_or(5_000_000),
            ),
        };

        // When video is copied (-c:v copy), there is no post-input -ss trim so
        // the stream actually starts from the keyframe position, not the precise
        // requested time. The frontend uses start_secs as hlsStartOffset to
        // compute actualTime(), so it must match where the video truly begins.
        let effective_start = if video_copied {
            start_secs
        } else {
            requested_secs
        };

        let video_codec_rfc6381 = output_video_codec_rfc6381(video_codec, video_copied);
        let audio_codec_rfc6381 = output_audio_codec_rfc6381(audio_codec);

        let now_epoch = epoch_ms_now();
        let session = Arc::new(HlsSession {
            session_id: session_id.clone(),
            media_id: media_id.to_string(),
            output_dir: output_dir.clone(),
            segment_duration: self.segment_duration,
            ffmpeg_handle: Mutex::new(Some(child)),
            created_at: Instant::now(),
            last_accessed_epoch_ms: std::sync::atomic::AtomicU64::new(now_epoch),
            last_segment_request_epoch_ms: std::sync::atomic::AtomicU64::new(now_epoch),
            ffmpeg_failed: std::sync::atomic::AtomicBool::new(false),
            segment_count: std::sync::atomic::AtomicU64::new(0),
            duration_secs,
            width: session_w,
            height: session_h,
            bitrate_kbps: variant.map(|v| v.video_bitrate_kbps).or(bitrate_kbps),
            start_secs: effective_start,
            variant_label,
            bandwidth_bps: session_bw,
            video_codec_rfc6381,
            audio_codec_rfc6381,
            video_copied,
            awaiting_promotion: std::sync::atomic::AtomicBool::new(false),
        });

        // Wire the stderr reader to the session's ffmpeg_failed flag.
        // This must happen after session construction so we can clone the Arc.
        if let Some(stderr) = stderr {
            let session_id_log = session_id.clone();
            let session_arc = session.clone();
            tokio::spawn(async move {
                use tokio::io::AsyncBufReadExt;
                let reader = tokio::io::BufReader::new(stderr);
                let mut lines = reader.lines();
                while let Ok(Some(line)) = lines.next_line().await {
                    warn!("ffmpeg HLS [{}]: {}", session_id_log, line);
                    if is_ffmpeg_fatal_error(&line) {
                        warn!(
                            "ffmpeg HLS [{}]: fatal error detected, marking session failed",
                            session_id_log
                        );
                        session_arc
                            .ffmpeg_failed
                            .store(true, std::sync::atomic::Ordering::Release);
                    }
                }
            });
        }

        self.sessions.insert(session_id.clone(), session.clone());
        if variant.is_none() {
            self.media_sessions
                .insert(media_id.to_string(), session_id.clone());
        }

        Self::wait_for_first_segment(&session).await;

        Ok(session)
    }

    /// Wait for the first HLS segment to appear in a session's playlist.
    /// Uses adaptive backoff: 20ms×25 (0.5s), 50ms×40 (2s), 250ms×40 (10s) ≈ 12.5s total.
    /// The aggressive initial tier (20ms) minimizes latency for video-copy sessions
    /// that typically produce the first segment in <300ms.
    async fn wait_for_first_segment(session: &HlsSession) {
        let playlist_path = session.output_dir.join("playlist.m3u8");
        let mut ready = false;
        let poll_schedule: &[(u64, u32)] = &[(20, 25), (50, 40), (250, 40)];
        'outer: for &(interval_ms, count) in poll_schedule {
            for _ in 0..count {
                if playlist_path.exists() {
                    if let Ok(content) = tokio::fs::read_to_string(&playlist_path).await {
                        if content.contains("#EXTINF:") {
                            ready = true;
                            break 'outer;
                        }
                    }
                }
                tokio::time::sleep(std::time::Duration::from_millis(interval_ms)).await;
            }
        }

        if !ready {
            warn!(
                "HLS session {} timed out waiting for first segment",
                session.session_id
            );
        } else {
            info!(
                "HLS session {} ready (first segment generated in {:.1}s)",
                session.session_id,
                session.created_at.elapsed().as_secs_f64()
            );
        }
    }

    /// Same as `create_session` but does NOT wait for the first segment.
    /// Used by `create_variant_sessions_owned` to spawn all FFmpeg processes
    /// first, then wait for all segments in parallel.
    #[allow(clippy::too_many_arguments)]
    async fn create_session_no_wait(
        &self,
        media_id: &str,
        file_path: &Path,
        duration_secs: Option<f64>,
        width: Option<u32>,
        height: Option<u32>,
        bitrate_kbps: Option<u32>,
        start_secs: f64,
        requested_secs: f64,
        subtitle_path: Option<&Path>,
        variant: Option<&QualityVariant>,
        pixel_format: Option<&str>,
        audio_stream_index: Option<u32>,
        frame_rate: Option<&str>,
        audio_codec: Option<&str>,
        video_codec: Option<&str>,
        color_transfer: Option<&str>,
        color_primaries: Option<&str>,
    ) -> Result<Arc<HlsSession>> {
        // Destroy any existing session for this media (single-variant path)
        if variant.is_none() {
            if let Some(sid) = self.media_sessions.get(media_id) {
                let old_sid = sid.clone();
                drop(sid);
                self.destroy_session(&old_sid).await;
            }
        }

        // Create new session
        let session_id = uuid::Uuid::new_v4().to_string();
        let output_dir = self.cache_dir.join(&session_id);
        tokio::fs::create_dir_all(&output_dir).await?;

        let variant_label = variant.map(|v| v.label.clone());
        info!(
            "Creating HLS session {} for media {} at {:.1}s variant={} ({})",
            session_id,
            media_id,
            start_secs,
            variant_label.as_deref().unwrap_or("native"),
            file_path.display()
        );

        // Spawn FFmpeg (with -ss if starting from a non-zero position)
        let (child, stderr, video_copied) = self
            .spawn_ffmpeg(
                file_path,
                &output_dir,
                start_secs,
                requested_secs,
                subtitle_path,
                variant,
                height,
                pixel_format,
                audio_stream_index,
                frame_rate,
                audio_codec,
                video_codec,
                color_transfer,
                color_primaries,
            )
            .await?;

        let (session_w, session_h, session_bw) = match variant {
            Some(v) => (Some(v.width), Some(v.height), v.bandwidth_bps),
            None => (
                width,
                height,
                bitrate_kbps.map(|k| (k as u64) * 1000).unwrap_or(5_000_000),
            ),
        };

        let effective_start = if video_copied {
            start_secs
        } else {
            requested_secs
        };

        let video_codec_rfc6381 = output_video_codec_rfc6381(video_codec, video_copied);
        let audio_codec_rfc6381 = output_audio_codec_rfc6381(audio_codec);

        let now_epoch = epoch_ms_now();
        let session = Arc::new(HlsSession {
            session_id: session_id.clone(),
            media_id: media_id.to_string(),
            output_dir: output_dir.clone(),
            segment_duration: self.segment_duration,
            ffmpeg_handle: Mutex::new(Some(child)),
            created_at: Instant::now(),
            last_accessed_epoch_ms: std::sync::atomic::AtomicU64::new(now_epoch),
            last_segment_request_epoch_ms: std::sync::atomic::AtomicU64::new(now_epoch),
            ffmpeg_failed: std::sync::atomic::AtomicBool::new(false),
            segment_count: std::sync::atomic::AtomicU64::new(0),
            duration_secs,
            width: session_w,
            height: session_h,
            bitrate_kbps: variant.map(|v| v.video_bitrate_kbps).or(bitrate_kbps),
            start_secs: effective_start,
            variant_label,
            bandwidth_bps: session_bw,
            video_codec_rfc6381,
            audio_codec_rfc6381,
            video_copied,
            awaiting_promotion: std::sync::atomic::AtomicBool::new(false),
        });

        // Wire the stderr reader to the session's ffmpeg_failed flag.
        if let Some(stderr) = stderr {
            let session_id_log = session_id.clone();
            let session_arc = session.clone();
            tokio::spawn(async move {
                use tokio::io::AsyncBufReadExt;
                let reader = tokio::io::BufReader::new(stderr);
                let mut lines = reader.lines();
                while let Ok(Some(line)) = lines.next_line().await {
                    warn!("ffmpeg HLS [{}]: {}", session_id_log, line);
                    if is_ffmpeg_fatal_error(&line) {
                        warn!(
                            "ffmpeg HLS [{}]: fatal error detected, marking session failed",
                            session_id_log
                        );
                        session_arc
                            .ffmpeg_failed
                            .store(true, std::sync::atomic::Ordering::Release);
                    }
                }
            });
        }

        self.sessions.insert(session_id.clone(), session.clone());
        if variant.is_none() {
            self.media_sessions
                .insert(media_id.to_string(), session_id.clone());
        }

        Ok(session)
    }

    /// Create multiple HLS sessions for adaptive bitrate streaming.
    /// Spawns one FFmpeg process per quality variant. Returns all sessions.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_variant_sessions(
        &self,
        media_id: &str,
        file_path: &Path,
        duration_secs: Option<f64>,
        source_width: Option<u32>,
        source_height: Option<u32>,
        source_bitrate_kbps: Option<u32>,
        start_secs: f64,
        requested_secs: f64,
        subtitle_path: Option<&Path>,
        pixel_format: Option<&str>,
        audio_stream_index: Option<u32>,
        frame_rate: Option<&str>,
        audio_codec: Option<&str>,
        video_codec: Option<&str>,
        color_transfer: Option<&str>,
        color_primaries: Option<&str>,
    ) -> Result<Vec<Arc<HlsSession>>> {
        self.create_variant_sessions_owned(
            media_id,
            media_id,
            file_path,
            duration_secs,
            source_width,
            source_height,
            source_bitrate_kbps,
            start_secs,
            requested_secs,
            subtitle_path,
            pixel_format,
            audio_stream_index,
            frame_rate,
            audio_codec,
            video_codec,
            color_transfer,
            color_primaries,
        )
        .await
    }

    /// Same as `create_variant_sessions` but keyed by explicit ownership key
    /// (e.g. media_id::playback_session_id) to avoid cross-client interference.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_variant_sessions_owned(
        &self,
        owner_key: &str,
        media_id: &str,
        file_path: &Path,
        duration_secs: Option<f64>,
        source_width: Option<u32>,
        source_height: Option<u32>,
        source_bitrate_kbps: Option<u32>,
        start_secs: f64,
        requested_secs: f64,
        subtitle_path: Option<&Path>,
        pixel_format: Option<&str>,
        audio_stream_index: Option<u32>,
        frame_rate: Option<&str>,
        audio_codec: Option<&str>,
        video_codec: Option<&str>,
        color_transfer: Option<&str>,
        color_primaries: Option<&str>,
    ) -> Result<Vec<Arc<HlsSession>>> {
        // Serialize creates for this ownership key so concurrent calls don't
        // destroy each other's session mapping and orphan FFmpeg processes.
        let lock = self
            .creation_locks
            .entry(owner_key.to_string())
            .or_insert_with(|| Arc::new(Semaphore::new(1)))
            .clone();
        let _guard = lock
            .acquire()
            .await
            .map_err(|e| anyhow!("Lock error: {}", e))?;

        // Destroy any existing variant sessions for this ownership key
        self.destroy_owner_sessions(owner_key).await;

        let variants = ferrite_transcode::variants::select_variants(source_width, source_height);
        info!(
            "Creating {} ABR variant sessions for media {} (source={}x{})",
            variants.len(),
            media_id,
            source_width.unwrap_or(0),
            source_height.unwrap_or(0),
        );

        let mut sessions = Vec::with_capacity(variants.len());
        let mut session_ids = Vec::with_capacity(variants.len());

        // Phase 1: Spawn all FFmpeg processes without waiting for segments.
        for variant in &variants {
            let session = self
                .create_session_no_wait(
                    media_id,
                    file_path,
                    duration_secs,
                    source_width,
                    source_height,
                    source_bitrate_kbps,
                    start_secs,
                    requested_secs,
                    subtitle_path,
                    Some(variant),
                    pixel_format,
                    audio_stream_index,
                    frame_rate,
                    audio_codec,
                    video_codec,
                    color_transfer,
                    color_primaries,
                )
                .await?;
            session_ids.push(session.session_id.clone());
            sessions.push(session);
        }

        // Phase 2: Wait for all first segments in parallel.
        // This reduces ABR startup from N×(spawn+wait) to N×spawn + max(wait).
        let wait_futures: Vec<_> = sessions
            .iter()
            .map(|s| Self::wait_for_first_segment(s))
            .collect();
        futures::future::join_all(wait_futures).await;

        self.media_variant_sessions
            .insert(owner_key.to_string(), session_ids);

        Ok(sessions)
    }

    /// Create a single HLS session at the highest quality for fast seeking.
    /// Only spawns ONE FFmpeg process instead of one per variant, dramatically
    /// reducing seek latency. Returns a single-element Vec for API compatibility.
    /// If `awaiting_promotion` is true, the session will be promoted to the full
    /// ABR ladder on the next master playlist poll.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_single_variant_session(
        &self,
        media_id: &str,
        file_path: &Path,
        duration_secs: Option<f64>,
        source_width: Option<u32>,
        source_height: Option<u32>,
        source_bitrate_kbps: Option<u32>,
        start_secs: f64,
        requested_secs: f64,
        subtitle_path: Option<&Path>,
        pixel_format: Option<&str>,
        audio_stream_index: Option<u32>,
        frame_rate: Option<&str>,
        audio_codec: Option<&str>,
        video_codec: Option<&str>,
        color_transfer: Option<&str>,
        color_primaries: Option<&str>,
        awaiting_promotion: bool,
    ) -> Result<Vec<Arc<HlsSession>>> {
        self.create_single_variant_session_owned(
            media_id,
            media_id,
            file_path,
            duration_secs,
            source_width,
            source_height,
            source_bitrate_kbps,
            start_secs,
            requested_secs,
            subtitle_path,
            pixel_format,
            audio_stream_index,
            frame_rate,
            audio_codec,
            video_codec,
            color_transfer,
            color_primaries,
            awaiting_promotion,
        )
        .await
    }

    /// Same as `create_single_variant_session` but keyed by explicit ownership key
    /// (e.g. media_id::playback_session_id) to avoid cross-client interference.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_single_variant_session_owned(
        &self,
        owner_key: &str,
        media_id: &str,
        file_path: &Path,
        duration_secs: Option<f64>,
        source_width: Option<u32>,
        source_height: Option<u32>,
        source_bitrate_kbps: Option<u32>,
        start_secs: f64,
        requested_secs: f64,
        subtitle_path: Option<&Path>,
        pixel_format: Option<&str>,
        audio_stream_index: Option<u32>,
        frame_rate: Option<&str>,
        audio_codec: Option<&str>,
        video_codec: Option<&str>,
        color_transfer: Option<&str>,
        color_primaries: Option<&str>,
        awaiting_promotion: bool,
    ) -> Result<Vec<Arc<HlsSession>>> {
        // Serialize creates for this ownership key so concurrent calls don't
        // destroy each other's session mapping and orphan FFmpeg processes.
        let lock = self
            .creation_locks
            .entry(owner_key.to_string())
            .or_insert_with(|| Arc::new(Semaphore::new(1)))
            .clone();
        let _guard = lock
            .acquire()
            .await
            .map_err(|e| anyhow!("Lock error: {}", e))?;

        // Destroy any existing sessions for this media
        self.destroy_owner_sessions(owner_key).await;

        let variants = ferrite_transcode::variants::select_variants(source_width, source_height);
        // Use only the highest quality variant (first in the list)
        let variant = variants
            .first()
            .ok_or_else(|| anyhow!("No quality variants available"))?;

        info!(
            "Creating single HLS session for media {} at {:.1}s variant={} awaiting_promotion={}",
            media_id, start_secs, variant.label, awaiting_promotion,
        );

        let session = self
            .create_session(
                media_id,
                file_path,
                duration_secs,
                source_width,
                source_height,
                source_bitrate_kbps,
                start_secs,
                requested_secs,
                subtitle_path,
                Some(variant),
                pixel_format,
                audio_stream_index,
                frame_rate,
                audio_codec,
                video_codec,
                color_transfer,
                color_primaries,
            )
            .await?;

        // Mark the session for ABR ladder promotion if requested
        session
            .awaiting_promotion
            .store(awaiting_promotion, std::sync::atomic::Ordering::Release);

        let session_ids = vec![session.session_id.clone()];
        self.media_variant_sessions
            .insert(owner_key.to_string(), session_ids);

        Ok(vec![session])
    }

    /// Destroy all sessions (single + variant) for a media item.
    pub async fn destroy_media_sessions(&self, media_id: &str) {
        self.destroy_owner_sessions(media_id).await;
    }

    /// Destroy all sessions for an ownership key (media_id fallback, or media_id::playback_session_id).
    /// Destroys sessions concurrently to minimize cleanup latency.
    pub async fn destroy_owner_sessions(&self, owner_key: &str) {
        let mut session_ids = Vec::new();

        // Collect single-variant session ID
        if let Some(sid) = self.media_sessions.get(owner_key) {
            session_ids.push(sid.clone());
        }
        // Collect all variant session IDs
        if let Some(sids) = self.media_variant_sessions.get(owner_key) {
            session_ids.extend(sids.iter().cloned());
        }

        if session_ids.is_empty() {
            return;
        }

        // Destroy all sessions concurrently
        let futs: Vec<_> = session_ids
            .iter()
            .map(|sid| self.destroy_session(sid))
            .collect();
        futures::future::join_all(futs).await;
    }

    /// Return a snapshot of all currently active HLS sessions.
    pub fn list_active_sessions(&self) -> Vec<ActiveSessionInfo> {
        let mut result = Vec::new();
        for entry in self.sessions.iter() {
            let s = entry.value();
            result.push(ActiveSessionInfo {
                session_id: s.session_id.clone(),
                media_id: s.media_id.clone(),
                variant_label: s.variant_label.clone(),
                start_secs: s.start_secs,
                width: s.width,
                height: s.height,
                bitrate_kbps: s.bitrate_kbps,
                idle_secs: s.idle_secs(),
                age_secs: s.created_at.elapsed().as_secs(),
            });
        }
        result
    }

    /// Get all variant sessions for a media item (for master playlist generation).
    pub fn get_variant_sessions(&self, media_id: &str) -> Vec<Arc<HlsSession>> {
        self.get_variant_sessions_owned(media_id)
    }

    /// Get all variant sessions for an ownership key.
    pub fn get_variant_sessions_owned(&self, owner_key: &str) -> Vec<Arc<HlsSession>> {
        if let Some(sids) = self.media_variant_sessions.get(owner_key) {
            sids.iter()
                .filter_map(|sid| self.sessions.get(sid).map(|s| s.clone()))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Spawn FFmpeg with HLS output.
    /// If `start_secs > 0`, uses `-ss` before `-i` for fast input seeking.
    /// If `subtitle_path` is provided, burns subtitles into the video via `-vf subtitles=`.
    /// If `variant` is provided, scales video and constrains bitrate to that quality level.
    /// `source_height` is used to determine if the variant actually needs scaling.
    #[allow(clippy::too_many_arguments)]
    async fn spawn_ffmpeg(
        &self,
        file_path: &Path,
        output_dir: &Path,
        start_secs: f64,
        requested_secs: f64,
        subtitle_path: Option<&Path>,
        variant: Option<&QualityVariant>,
        source_height: Option<u32>,
        pixel_format: Option<&str>,
        audio_stream_index: Option<u32>,
        frame_rate: Option<&str>,
        audio_codec: Option<&str>,
        video_codec: Option<&str>,
        color_transfer: Option<&str>,
        color_primaries: Option<&str>,
    ) -> Result<(Child, Option<tokio::process::ChildStderr>, bool)> {
        // Only fall back to software encoding when we actually need CPU-side
        // frame access (subtitle burn-in or resolution scaling).
        // If the variant matches the source resolution, no scale filter is needed
        // and we can keep using the hardware encoder.
        let needs_scaling = variant.is_some_and(|v| {
            let src_h = source_height.unwrap_or(1080);
            v.height != src_h
        });
        let needs_software =
            (subtitle_path.is_some() || needs_scaling) && self.encoder.is_hardware();
        let effective_encoder = if needs_software {
            if subtitle_path.is_some() {
                info!("HLS subtitle burn-in active — falling back to software encoder");
            }
            if needs_scaling {
                info!("HLS variant scaling active — falling back to software encoder");
            }
            EncoderProfile::software()
        } else {
            self.encoder.clone()
        };

        // ---------------------------------------------------------------
        // Pre-compute video filter chain and copy decision BEFORE building
        // the seek args, because -c:v copy requires different seek flags.
        // ---------------------------------------------------------------
        let mut vf_parts: Vec<String> = Vec::new();

        let is_high_bit = pixel_format
            .map(ferrite_transcode::tonemap::is_high_bit_depth)
            .unwrap_or(false);
        let needs_tonemap =
            is_high_bit && ferrite_transcode::tonemap::is_true_hdr(color_transfer, color_primaries);
        if needs_tonemap {
            info!(
                "True HDR detected (pix={}, transfer={:?}, primaries={:?}), applying tone-mapping",
                pixel_format.unwrap_or("unknown"),
                color_transfer,
                color_primaries
            );
            vf_parts.push(ferrite_transcode::tonemap::tonemap_filter());
        } else if is_high_bit {
            info!(
                "10-bit SDR detected (pix={}, transfer={:?}), applying bit-depth conversion only",
                pixel_format.unwrap_or("unknown"),
                color_transfer
            );
            vf_parts.push(ferrite_transcode::tonemap::bit_depth_filter());
        }

        if let Some(sub_path) = subtitle_path {
            let sub_path_escaped =
                crate::transcode::escape_ffmpeg_filter_path(&sub_path.to_string_lossy());
            if start_secs > 0.5 {
                // Pre-input -ss resets PTS to 0, but the subtitles filter reads
                // the external file using absolute timestamps. We must shift PTS
                // back to the original file time so the filter renders the correct
                // subtitles, then shift back to 0-based PTS for the encoder.
                vf_parts.push(format!("setpts=PTS+{:.3}/TB", start_secs));
                vf_parts.push(format!("subtitles={}", sub_path_escaped));
                vf_parts.push(format!("setpts=PTS-{:.3}/TB", start_secs));
            } else {
                vf_parts.push(format!("subtitles={}", sub_path_escaped));
            }
            info!("HLS subtitle burn-in enabled: {}", sub_path.display());
        }

        if needs_scaling {
            if let Some(v) = variant {
                vf_parts.push(format!("scale=-2:{}", v.height));
            }
        }

        let video_is_h264 = video_codec
            .map(|c| c.to_lowercase() == "h264")
            .unwrap_or(false);
        // Video copy is now permitted on seek because we align the seek target
        // to a keyframe in the API layer, removing the need for a precise post-input trim.
        let can_copy_video = video_is_h264 && vf_parts.is_empty();

        let has_software_filters = !vf_parts.is_empty();

        // ---------------------------------------------------------------
        // Build FFmpeg args
        // ---------------------------------------------------------------
        let mut args: Vec<String> = vec![
            "-hide_banner".into(),
            "-loglevel".into(),
            "error".into(),
            "-nostdin".into(),
        ];

        // HW-accelerated decoding args (before -i).
        if needs_scaling && !needs_software {
            args.extend(
                effective_encoder
                    .hw_input_args(has_software_filters)
                    .iter()
                    .cloned(),
            );
        }

        // Seek before input for fast seeking (demuxer-level)
        if start_secs > 0.5 {
            args.push("-ss".into());
            args.push(format!("{:.3}", start_secs));
        }

        // With -c:v copy (only at start=0), use -noaccurate_seek so both
        // audio and video start from the same keyframe position.
        if can_copy_video && start_secs > 0.5 {
            args.push("-noaccurate_seek".into());
        }

        let audio_map = format!("0:a:{}", audio_stream_index.unwrap_or(0));
        args.extend(["-i".into(), file_path.to_string_lossy().to_string()]);

        // Precise seek after input (only when re-encoding): decode from the
        // keyframe but trim output to the exact requested time.
        // Always add post-input -ss when seeking, even if the delta is tiny —
        // without it, FFmpeg's HLS muxer starts the first segment from the
        // nearest keyframe the demuxer landed on, which can be 10-20s before
        // the target depending on GOP size.
        if !can_copy_video && requested_secs > 0.5 {
            let precise_delta = (requested_secs - start_secs).max(0.0);
            args.extend(["-ss".into(), format!("{:.3}", precise_delta)]);
        }

        args.extend(["-map".into(), "0:v:0".into(), "-map".into(), audio_map]);

        if !vf_parts.is_empty() {
            args.extend(["-vf".into(), vf_parts.join(",")]);
        }

        if can_copy_video {
            info!("HLS video passthrough: source is H.264 with no filters, using -c:v copy");
            args.extend(["-c:v".into(), "copy".into()]);
        } else {
            // Video: H.264 transcode (using selected encoder profile).
            // When a filter handles format conversion (tone-mapping or bit-depth),
            // skip the redundant -pix_fmt flag to avoid conflicts.
            if is_high_bit {
                args.extend(effective_encoder.video_encode_args_no_pix_fmt());
            } else {
                args.extend(effective_encoder.video_encode_args());
            }

            // If variant specifies a target bitrate AND we're actually scaling,
            // override CRF with constrained bitrate. Skip for native resolution
            // to avoid conflicting with NVENC's own rate control (vbr + cq).
            if needs_scaling {
                if let Some(v) = variant {
                    args.extend([
                        "-b:v".into(),
                        format!("{}k", v.video_bitrate_kbps),
                        "-maxrate".into(),
                        format!("{}k", (v.video_bitrate_kbps as f64 * 1.5) as u32),
                        "-bufsize".into(),
                        format!("{}k", v.video_bitrate_kbps * 2),
                    ]);
                }
            }
        }

        let audio_bitrate = variant
            .map(|v| format!("{}k", v.audio_bitrate_kbps))
            .unwrap_or_else(|| "192k".into());

        // Force keyframes at segment boundaries (only when re-encoding video).
        // -force_key_frames "expr:..." only works with libx264, NOT with hardware
        // encoders (NVENC/QSV/VAAPI). Use -g (GOP size in frames) instead, which
        // is universally supported. Calculate from actual frame rate when available.
        // With -c:v copy, GOP args are meaningless — the source keyframes are preserved.
        if !can_copy_video {
            let fps = frame_rate.and_then(parse_frame_rate).unwrap_or(24.0);
            let gop_size = (self.segment_duration as f64 * fps).round() as u32;
            args.extend([
                "-g".into(),
                gop_size.to_string(),
                "-keyint_min".into(),
                gop_size.to_string(),
            ]);
        }

        // Audio: passthrough if the source codec is browser-compatible, otherwise re-encode to AAC stereo
        let can_passthrough = audio_codec
            .map(ferrite_transcode::audio::can_passthrough)
            .unwrap_or(false);
        if can_passthrough {
            args.extend(["-c:a".into(), "copy".into()]);
        } else {
            args.extend([
                "-c:a".into(),
                "aac".into(),
                "-b:a".into(),
                audio_bitrate,
                "-ac".into(),
                "2".into(),
            ]);
        }

        args.extend([
            // HLS output
            "-f".into(),
            "hls".into(),
            "-hls_time".into(),
            self.segment_duration.to_string(),
            "-hls_list_size".into(),
            if can_copy_video {
                "0".into() // Unlimited: keep all segments in playlist for copy mode
            } else {
                self.playlist_window_segments.to_string()
            },
            "-hls_segment_type".into(),
            "fmp4".into(),
            "-hls_fmp4_init_filename".into(),
            "init.mp4".into(),
            "-hls_segment_filename".into(),
            "seg_%03d.m4s".into(),
            "-hls_flags".into(),
            if can_copy_video {
                // Video-copy remuxes at disk speed (much faster than real-time).
                // Keep all segments so the player can fetch from the start.
                "independent_segments+temp_file".into()
            } else {
                // Real-time transcode: sliding window keeps disk usage bounded.
                "independent_segments+delete_segments+temp_file".into()
            },
            "playlist.m3u8".into(),
        ]);

        info!("HLS ffmpeg args: {:?}", args);

        let mut child = tokio::process::Command::new(&self.ffmpeg_path)
            .args(&args)
            .current_dir(output_dir)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| anyhow!("Failed to spawn ffmpeg for HLS: {}", e))?;

        // Return stderr to the caller so it can be wired to the session's
        // ffmpeg_failed flag after the session Arc is constructed.
        let stderr = child.stderr.take();
        Ok((child, stderr, can_copy_video))
    }

    /// Generate master playlist pointing to one or more variants.
    /// If `sessions` contains multiple entries, generates an adaptive bitrate master playlist.
    /// Falls back to single-variant if only one session is provided.
    pub fn generate_master_playlist(
        &self,
        sessions: &[Arc<HlsSession>],
        media_id: &str,
        token: Option<&str>,
    ) -> String {
        let token_suffix = token
            .map(|t| format!("?token={}", percent_encode(t)))
            .unwrap_or_default();

        let mut playlist = String::from("#EXTM3U\n#EXT-X-VERSION:7\n");

        for session in sessions {
            let bandwidth = session.bandwidth_bps;

            let resolution = match (session.width, session.height) {
                (Some(w), Some(h)) => format!(",RESOLUTION={}x{}", w, h),
                _ => String::new(),
            };

            let name = session.variant_label.as_deref().unwrap_or("native");
            let codecs = format!(
                ",CODECS=\"{},{}\"",
                session.video_codec_rfc6381, session.audio_codec_rfc6381
            );

            let variant_url = format!(
                "/api/stream/{}/hls/{}/playlist.m3u8{}",
                media_id, session.session_id, token_suffix
            );

            playlist.push_str(&format!(
                "\n#EXT-X-STREAM-INF:BANDWIDTH={},NAME=\"{}\"{}{}\n{}\n",
                bandwidth, name, resolution, codecs, variant_url
            ));
        }

        playlist
    }

    /// Read the variant playlist from disk and rewrite URLs to absolute API paths.
    pub async fn get_variant_playlist(
        &self,
        session: &HlsSession,
        media_id: &str,
        token: Option<&str>,
    ) -> Result<String> {
        session.touch();

        let playlist_path = session.output_dir.join("playlist.m3u8");
        let raw = tokio::fs::read_to_string(&playlist_path)
            .await
            .map_err(|e| anyhow!("Failed to read playlist: {}", e))?;

        let base_url = format!("/api/stream/{}/hls/{}", media_id, session.session_id);
        let token_suffix = token
            .map(|t| format!("?token={}", percent_encode(t)))
            .unwrap_or_default();

        let rewritten = rewrite_playlist(&raw, &base_url, &token_suffix);
        Ok(rewritten)
    }

    /// Wait for a segment file to be finalized by FFmpeg and return its path.
    ///
    /// For `init.mp4`, waits until the file exists (written once before any segments).
    /// For `.m4s` segments, waits until the filename appears in the playlist's `#EXTINF`
    /// entries — this means FFmpeg has finished writing that segment and moved on,
    /// preventing partial reads.
    ///
    /// Returns `Ok(Some(PathBuf))` when the segment is ready to be read, `Ok(None)` if
    /// FFmpeg exited or timed out, or `Err` for invalid filenames.
    pub async fn wait_for_segment(
        &self,
        session: &HlsSession,
        filename: &str,
    ) -> Result<Option<std::path::PathBuf>> {
        session.touch_segment();

        // Validate filename to prevent path traversal
        if filename.contains("..") || filename.contains('/') || filename.contains('\\') {
            return Err(anyhow!("Invalid segment filename"));
        }

        let path = session.output_dir.join(filename);

        // init.mp4 is written once before any segments — serve as soon as it exists.
        // Poll at 100ms (not 500ms) because FFmpeg writes init.mp4 almost immediately
        // and this is on the critical path for time-to-first-frame.
        if filename == "init.mp4" {
            for _ in 0..300 {
                if path.exists() {
                    return Ok(Some(path));
                }
                // Bail early if FFmpeg has crashed
                if !session.is_ffmpeg_alive().await {
                    warn!(
                        "FFmpeg exited before init.mp4 was written for session {}",
                        session.session_id
                    );
                    return Ok(None);
                }
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            }
            return Ok(None);
        }

        // For .m4s segments, wait until FFmpeg has listed the segment in the playlist.
        // A segment appearing in a #EXTINF entry means FFmpeg has finished writing it
        // and moved on, so it's safe to read without getting a partial/truncated file.
        let playlist_path = session.output_dir.join("playlist.m3u8");
        let mut last_playlist_mtime: Option<std::time::SystemTime> = None;

        // Adaptive polling: fast at first (most segments arrive within 500ms),
        // fall back to slower polling for segments that take longer (e.g. HDR transcode).
        // Total timeout: 10×50ms + 10×100ms + 116×250ms = 500ms + 1000ms + 29000ms = 30.5s
        let poll_schedule: &[(u64, u32)] = &[
            (50, 10),   //  0 – 500ms:  poll every  50ms
            (100, 10),  //  0.5 – 1.5s: poll every 100ms
            (250, 116), //  1.5 – 30.5s: poll every 250ms
        ];

        // Only re-read playlist when it changed on disk to reduce I/O churn.
        for &(interval_ms, count) in poll_schedule {
            for _ in 0..count {
                if let Ok(meta) = tokio::fs::metadata(&playlist_path).await {
                    if let Ok(mtime) = meta.modified() {
                        let changed = match last_playlist_mtime {
                            Some(prev) => mtime != prev,
                            None => true,
                        };
                        if changed {
                            last_playlist_mtime = Some(mtime);
                            if let Ok(playlist) = tokio::fs::read_to_string(&playlist_path).await {
                                if segment_listed_in_playlist(&playlist, filename) {
                                    session.refresh_segment_count();
                                    return Ok(Some(path));
                                }
                            }
                        }
                    }
                }
                // Short-circuit if FFmpeg reported a fatal error via stderr
                // (e.g. corrupt file, permission denied, disk full).
                if session
                    .ffmpeg_failed
                    .load(std::sync::atomic::Ordering::Acquire)
                {
                    warn!(
                        "FFmpeg fatal error detected for session {}, aborting segment wait for '{}'",
                        session.session_id, filename
                    );
                    return Ok(None);
                }
                // Bail early if FFmpeg has crashed — no point waiting for segments
                // that will never be written.
                if !session.is_ffmpeg_alive().await {
                    // One final check: FFmpeg may have written this segment before dying
                    if let Ok(playlist) = tokio::fs::read_to_string(&playlist_path).await {
                        if segment_listed_in_playlist(&playlist, filename) {
                            return Ok(Some(path));
                        }
                    }
                    warn!(
                        "FFmpeg exited before segment '{}' was ready for session {}",
                        filename, session.session_id
                    );
                    return Ok(None);
                }
                tokio::time::sleep(std::time::Duration::from_millis(interval_ms)).await;
            }
        }

        // Final check after timeout
        if let Ok(playlist) = tokio::fs::read_to_string(&playlist_path).await {
            if segment_listed_in_playlist(&playlist, filename) {
                return Ok(Some(path));
            }
        }

        Ok(None)
    }

    /// Get a session by ID.
    pub fn get_session(&self, session_id: &str) -> Option<Arc<HlsSession>> {
        self.sessions.get(session_id).map(|s| s.clone())
    }

    /// Get the current session for a media item (if any).
    /// Checks variant sessions first (used by create_single_variant_session),
    /// then falls back to legacy single-variant media_sessions.
    pub fn get_session_for_media(&self, media_id: &str) -> Option<Arc<HlsSession>> {
        self.get_session_for_owner(media_id)
    }

    /// Get the current session for an ownership key.
    pub fn get_session_for_owner(&self, owner_key: &str) -> Option<Arc<HlsSession>> {
        // Check variant sessions first (create_single_variant_session stores here)
        if let Some(sids) = self.media_variant_sessions.get(owner_key) {
            if let Some(first_sid) = sids.first() {
                if let Some(session) = self.sessions.get(first_sid) {
                    return Some(session.clone());
                }
            }
        }
        // Fallback to legacy single-variant map
        let sid = self.media_sessions.get(owner_key)?;
        self.sessions.get(sid.value()).map(|s| s.clone())
    }

    /// Destroy a session: kill FFmpeg, remove files.
    pub async fn destroy_session(&self, session_id: &str) {
        if let Some((_, session)) = self.sessions.remove(session_id) {
            // Remove this session ID from all owner maps.
            self.media_sessions.retain(|_, sid| sid != session_id);
            self.media_variant_sessions.retain(|_, sids| {
                sids.retain(|sid| sid != session_id);
                !sids.is_empty()
            });

            info!("Initiated teardown for HLS session {}", session_id);

            let session_id_owned = session_id.to_string();
            tokio::spawn(async move {
                // Kill FFmpeg (SIGTERM → 2s → SIGKILL on Unix, immediate kill on Windows)
                session.kill_ffmpeg().await;

                // Wait a moment for FFmpeg to actually flush its buffers and exit
                // before we yank the directory out from under it. Otherwise, FFmpeg
                // will complain about "No such file or directory" in its final log lines.
                // On Windows, the kill is immediate, but on Unix, the process may need
                // a brief window to process the SIGTERM cleanly.
                for _ in 0..10 {
                    if !session.is_ffmpeg_alive().await {
                        break;
                    }
                    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                }

                // Remove output directory
                if session.output_dir.exists() {
                    if let Err(e) = tokio::fs::remove_dir_all(&session.output_dir).await {
                        warn!(
                            "Failed to remove HLS output dir {}: {}",
                            session.output_dir.display(),
                            e
                        );
                    }
                }

                debug!("Cleaned up HLS session files for {}", session_id_owned);
            });
        }
    }

    /// Destroy all active sessions (used during graceful shutdown).
    /// Kills all FFmpeg processes and removes output directories.
    pub async fn destroy_all_sessions(&self) {
        let session_ids: Vec<String> = self
            .sessions
            .iter()
            .map(|entry| entry.key().clone())
            .collect();

        if session_ids.is_empty() {
            return;
        }

        info!(
            "Destroying {} active HLS session(s) for shutdown...",
            session_ids.len()
        );

        for session_id in session_ids {
            self.destroy_session(&session_id).await;
        }

        info!("All HLS sessions destroyed");
    }

    /// Background cleanup loop — kills idle FFmpeg processes promptly and removes expired sessions.
    pub async fn cleanup_loop(self: Arc<Self>) {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(15));

        loop {
            interval.tick().await;

            let mut expired = Vec::new();
            let mut ffmpeg_to_kill: Vec<Arc<HlsSession>> = Vec::new();

            for entry in self.sessions.iter() {
                let session = entry.value();
                if session.idle_secs() > self.session_timeout_secs {
                    expired.push(entry.key().clone());
                } else if session.is_ffmpeg_alive().await
                    && session.segment_idle_secs() > self.ffmpeg_idle_secs
                {
                    ffmpeg_to_kill.push(session.clone());
                }
            }

            for session in ffmpeg_to_kill {
                info!(
                    "Killing idle FFmpeg for session {} (no segments requested for {}s)",
                    session.session_id, self.ffmpeg_idle_secs
                );
                session.kill_ffmpeg().await;
            }

            for session_id in expired {
                info!("Cleaning up expired HLS session: {}", session_id);
                self.destroy_session(&session_id).await;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Playlist rewriting
// ---------------------------------------------------------------------------

/// Rewrite relative filenames in an HLS playlist to absolute API paths with auth token.
fn rewrite_playlist(raw: &str, base_url: &str, token_suffix: &str) -> String {
    let mut result = String::with_capacity(raw.len() * 2);

    for line in raw.lines() {
        if line.starts_with("#EXT-X-MAP:") {
            // Rewrite URI="init.mp4" → URI="/api/stream/.../init.mp4?token=..."
            if let Some(start) = line.find("URI=\"") {
                let prefix = &line[..start + 5]; // up to and including URI="
                let rest = &line[start + 5..];
                if let Some(end) = rest.find('"') {
                    let filename = &rest[..end];
                    let after = &rest[end..];
                    result.push_str(prefix);
                    result.push_str(base_url);
                    result.push('/');
                    result.push_str(filename);
                    result.push_str(token_suffix);
                    result.push_str(after);
                    result.push('\n');
                    continue;
                }
            }
            result.push_str(line);
            result.push('\n');
        } else if !line.starts_with('#') && !line.is_empty() {
            // Segment filename line, e.g. "seg_000.m4s"
            result.push_str(base_url);
            result.push('/');
            result.push_str(line.trim());
            result.push_str(token_suffix);
            result.push('\n');
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }

    result
}

/// Check if a segment filename appears in the playlist's #EXTINF entries.
/// A line sequence like:
///   #EXTINF:6.006,
///   seg_003.m4s
/// means FFmpeg has finished writing seg_003.m4s.
fn segment_listed_in_playlist(playlist: &str, filename: &str) -> bool {
    let mut prev_was_extinf = false;
    for line in playlist.lines() {
        if line.starts_with("#EXTINF:") {
            prev_was_extinf = true;
        } else if prev_was_extinf {
            if line.trim() == filename {
                return true;
            }
            prev_was_extinf = false;
        } else {
            prev_was_extinf = false;
        }
    }
    false
}

/// Parse a frame rate string from ffprobe (e.g. "24000/1001", "30/1", "25") into fps.
fn parse_frame_rate(fr: &str) -> Option<f64> {
    if let Some((num, den)) = fr.split_once('/') {
        let n: f64 = num.trim().parse().ok()?;
        let d: f64 = den.trim().parse().ok()?;
        if d > 0.0 {
            Some(n / d)
        } else {
            None
        }
    } else {
        fr.trim().parse().ok()
    }
}

/// Detect known fatal FFmpeg error patterns in a stderr line.
/// Returns true if the error is unrecoverable and the segment will never be written.
/// Used to short-circuit the 30s segment polling timeout.
fn is_ffmpeg_fatal_error(line: &str) -> bool {
    let lower = line.to_lowercase();
    lower.contains("no such file or directory")
        || lower.contains("permission denied")
        || lower.contains("disk quota exceeded")
        || lower.contains("no space left on device")
        || lower.contains("invalid data found when processing input")
        || lower.contains("moov atom not found")
        || lower.contains("end of file")
        || (lower.contains("error") && lower.contains("opening") && lower.contains("for reading"))
}

/// Minimal percent-encoding for token in URL.
fn percent_encode(input: &str) -> String {
    percent_encoding::utf8_percent_encode(input, percent_encoding::NON_ALPHANUMERIC).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicBool;

    #[test]
    fn parse_frame_rate_fraction() {
        let fps = parse_frame_rate("24000/1001").unwrap();
        assert!((fps - 23.976).abs() < 0.01);
    }

    #[test]
    fn parse_frame_rate_integer_fraction() {
        let fps = parse_frame_rate("30/1").unwrap();
        assert!((fps - 30.0).abs() < 0.001);
    }

    #[test]
    fn parse_frame_rate_plain_number() {
        let fps = parse_frame_rate("25").unwrap();
        assert!((fps - 25.0).abs() < 0.001);
    }

    #[test]
    fn parse_frame_rate_50fps() {
        let fps = parse_frame_rate("50000/1001").unwrap();
        assert!((fps - 49.95).abs() < 0.01);
    }

    #[test]
    fn parse_frame_rate_zero_denominator() {
        assert!(parse_frame_rate("30/0").is_none());
    }

    #[test]
    fn parse_frame_rate_invalid() {
        assert!(parse_frame_rate("abc").is_none());
    }

    #[test]
    fn output_audio_codec_defaults_to_aac() {
        assert_eq!(output_audio_codec_rfc6381(None), "mp4a.40.2");
        assert_eq!(output_audio_codec_rfc6381(Some("dts")), "mp4a.40.2");
    }

    #[test]
    fn output_audio_codec_passthrough_mappings() {
        assert_eq!(output_audio_codec_rfc6381(Some("aac")), "mp4a.40.2");
        assert_eq!(output_audio_codec_rfc6381(Some("mp3")), "mp4a.40.34");
        assert_eq!(output_audio_codec_rfc6381(Some("opus")), "opus");
    }

    fn test_temp_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "ferrite-hls-tests-{}-{}",
            name,
            uuid::Uuid::new_v4()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn make_test_session(media_id: &str, session_id: &str, output_dir: PathBuf) -> Arc<HlsSession> {
        let now_epoch = epoch_ms_now();
        Arc::new(HlsSession {
            session_id: session_id.to_string(),
            media_id: media_id.to_string(),
            output_dir,
            segment_duration: 2,
            ffmpeg_handle: Mutex::new(None),
            created_at: Instant::now(),
            last_accessed_epoch_ms: std::sync::atomic::AtomicU64::new(now_epoch),
            last_segment_request_epoch_ms: std::sync::atomic::AtomicU64::new(now_epoch),
            ffmpeg_failed: AtomicBool::new(false),
            segment_count: std::sync::atomic::AtomicU64::new(0),
            duration_secs: Some(60.0),
            width: Some(1920),
            height: Some(1080),
            bitrate_kbps: Some(5000),
            start_secs: 0.0,
            variant_label: Some("1080p".to_string()),
            bandwidth_bps: 5_000_000,
            video_codec_rfc6381: "avc1.64001f".to_string(),
            audio_codec_rfc6381: "mp4a.40.2".to_string(),
            video_copied: false,
            awaiting_promotion: AtomicBool::new(false),
        })
    }

    #[tokio::test]
    async fn destroy_media_sessions_isolated_per_media() {
        let root = test_temp_dir("isolation");
        let manager = Arc::new(HlsSessionManager::new(
            root.clone(),
            "ffmpeg".to_string(),
            2,
            30,
            30,
            30,
            EncoderProfile::software(),
        ));

        let session_a = make_test_session("media-a", "sid-a", root.join("sid-a"));
        let session_b = make_test_session("media-b", "sid-b", root.join("sid-b"));
        std::fs::create_dir_all(&session_a.output_dir).unwrap();
        std::fs::create_dir_all(&session_b.output_dir).unwrap();

        manager
            .sessions
            .insert(session_a.session_id.clone(), session_a.clone());
        manager
            .sessions
            .insert(session_b.session_id.clone(), session_b.clone());
        manager
            .media_variant_sessions
            .insert("media-a".to_string(), vec![session_a.session_id.clone()]);
        manager
            .media_variant_sessions
            .insert("media-b".to_string(), vec![session_b.session_id.clone()]);

        manager.destroy_media_sessions("media-a").await;

        assert!(manager.get_session("sid-a").is_none());
        assert!(manager.get_session("sid-b").is_some());

        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn master_playlist_includes_codecs_metadata() {
        let root = test_temp_dir("master-codecs");
        let manager = HlsSessionManager::new(
            root.clone(),
            "ffmpeg".to_string(),
            2,
            30,
            30,
            30,
            EncoderProfile::software(),
        );

        let session = make_test_session("media-codec", "sid-codec", root.join("sid-codec"));
        let playlist = manager.generate_master_playlist(&[session], "media-codec", None);

        assert!(playlist.contains("#EXT-X-STREAM-INF:"));
        assert!(playlist.contains("CODECS=\"avc1.64001f,mp4a.40.2\""));

        let _ = std::fs::remove_dir_all(root);
    }

    #[tokio::test]
    async fn get_session_for_media_prefers_variant_after_seek() {
        let root = test_temp_dir("seek-select");
        let manager = Arc::new(HlsSessionManager::new(
            root.clone(),
            "ffmpeg".to_string(),
            2,
            30,
            30,
            30,
            EncoderProfile::software(),
        ));

        let legacy = make_test_session("media-x", "sid-legacy", root.join("sid-legacy"));
        let seeked = make_test_session("media-x", "sid-seeked", root.join("sid-seeked"));
        std::fs::create_dir_all(&legacy.output_dir).unwrap();
        std::fs::create_dir_all(&seeked.output_dir).unwrap();

        manager
            .sessions
            .insert(legacy.session_id.clone(), legacy.clone());
        manager
            .sessions
            .insert(seeked.session_id.clone(), seeked.clone());
        manager
            .media_sessions
            .insert("media-x".to_string(), legacy.session_id.clone());
        manager
            .media_variant_sessions
            .insert("media-x".to_string(), vec![seeked.session_id.clone()]);

        let selected = manager
            .get_session_for_media("media-x")
            .expect("session should exist");
        assert_eq!(selected.session_id, "sid-seeked");

        let _ = std::fs::remove_dir_all(root);
    }
}
