use anyhow::{anyhow, Result};
use dashmap::DashMap;
use ferrite_transcode::hwaccel::EncoderProfile;
use ferrite_transcode::variants::QualityVariant;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;
use tokio::process::Child;
use tokio::sync::Mutex;
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
    last_accessed: Mutex<Instant>,
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
}

impl HlsSession {
    pub async fn touch(&self) {
        *self.last_accessed.lock().await = Instant::now();
    }

    pub async fn idle_secs(&self) -> u64 {
        self.last_accessed.lock().await.elapsed().as_secs()
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
                    Ok(None) => true,   // Still running
                    Err(_) => false,     // Error checking — assume dead
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Session Manager
// ---------------------------------------------------------------------------

pub struct HlsSessionManager {
    sessions: DashMap<String, Arc<HlsSession>>,
    /// Map from media_id → session_id for reuse (legacy single-variant)
    media_sessions: DashMap<String, String>,
    /// Map from media_id → list of session_ids for multi-variant ABR
    media_variant_sessions: DashMap<String, Vec<String>>,
    cache_dir: PathBuf,
    ffmpeg_path: String,
    segment_duration: u64,
    session_timeout_secs: u64,
    encoder: EncoderProfile,
}

impl HlsSessionManager {
    pub fn new(
        cache_dir: PathBuf,
        ffmpeg_path: String,
        segment_duration: u64,
        session_timeout_secs: u64,
        encoder: EncoderProfile,
    ) -> Self {
        Self {
            sessions: DashMap::new(),
            media_sessions: DashMap::new(),
            media_variant_sessions: DashMap::new(),
            cache_dir,
            ffmpeg_path,
            segment_duration,
            session_timeout_secs,
            encoder,
        }
    }

    /// Get or create an HLS session for a media item.
    /// Returns the session. Creates FFmpeg process if new.
    /// `start_secs` is the time offset to start transcoding from (0.0 = beginning).
    /// If a session already exists for this media with the same start offset, reuse it.
    pub async fn get_or_create_session(
        &self,
        media_id: &str,
        file_path: &Path,
        duration_secs: Option<f64>,
        width: Option<u32>,
        height: Option<u32>,
        bitrate_kbps: Option<u32>,
        start_secs: f64,
        subtitle_path: Option<&Path>,
        pixel_format: Option<&str>,
        audio_stream_index: Option<u32>,
        frame_rate: Option<&str>,
        audio_codec: Option<&str>,
        video_codec: Option<&str>,
        color_transfer: Option<&str>,
        color_primaries: Option<&str>,
    ) -> Result<Arc<HlsSession>> {
        // Check for existing session for this media
        if let Some(sid) = self.media_sessions.get(media_id) {
            if let Some(session) = self.sessions.get(sid.value()) {
                // Reuse if the start offset is close enough (within one segment)
                let offset_diff = (session.start_secs - start_secs).abs();
                if offset_diff < self.segment_duration as f64 {
                    session.touch().await;
                    return Ok(session.clone());
                }
                // Different start position — destroy old session first
                let old_sid = sid.clone();
                drop(sid);
                self.destroy_session(&old_sid).await;
            }
        }

        self.create_session(media_id, file_path, duration_secs, width, height, bitrate_kbps, start_secs, subtitle_path, None, pixel_format, audio_stream_index, frame_rate, audio_codec, video_codec, color_transfer, color_primaries).await
    }

    /// Always create a fresh HLS session (destroys any existing session for this media).
    /// Used for seeking to a new position.
    /// If `variant` is provided, FFmpeg will scale and constrain bitrate to that quality level.
    pub async fn create_session(
        &self,
        media_id: &str,
        file_path: &Path,
        duration_secs: Option<f64>,
        width: Option<u32>,
        height: Option<u32>,
        bitrate_kbps: Option<u32>,
        start_secs: f64,
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
        let child = self.spawn_ffmpeg(file_path, &output_dir, start_secs, subtitle_path, variant, height, pixel_format, audio_stream_index, frame_rate, audio_codec, video_codec, color_transfer, color_primaries).await?;

        let (session_w, session_h, session_bw) = match variant {
            Some(v) => (Some(v.width), Some(v.height), v.bandwidth_bps),
            None => (
                width,
                height,
                bitrate_kbps.map(|k| (k as u64) * 1000).unwrap_or(5_000_000),
            ),
        };

        let session = Arc::new(HlsSession {
            session_id: session_id.clone(),
            media_id: media_id.to_string(),
            output_dir: output_dir.clone(),
            segment_duration: self.segment_duration,
            ffmpeg_handle: Mutex::new(Some(child)),
            created_at: Instant::now(),
            last_accessed: Mutex::new(Instant::now()),
            duration_secs,
            width: session_w,
            height: session_h,
            bitrate_kbps: variant.map(|v| v.video_bitrate_kbps).or(bitrate_kbps),
            start_secs,
            variant_label,
            bandwidth_bps: session_bw,
        });

        self.sessions.insert(session_id.clone(), session.clone());
        if variant.is_none() {
            self.media_sessions
                .insert(media_id.to_string(), session_id.clone());
        }

        // Wait for first segment to appear (up to 15 seconds)
        let playlist_path = output_dir.join("playlist.m3u8");
        let mut ready = false;
        for _ in 0..150 {
            if playlist_path.exists() {
                if let Ok(content) = tokio::fs::read_to_string(&playlist_path).await {
                    if content.contains("#EXTINF:") {
                        ready = true;
                        break;
                    }
                }
            }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
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

        Ok(session)
    }

    /// Create multiple HLS sessions for adaptive bitrate streaming.
    /// Spawns one FFmpeg process per quality variant. Returns all sessions.
    pub async fn create_variant_sessions(
        &self,
        media_id: &str,
        file_path: &Path,
        duration_secs: Option<f64>,
        source_width: Option<u32>,
        source_height: Option<u32>,
        source_bitrate_kbps: Option<u32>,
        start_secs: f64,
        subtitle_path: Option<&Path>,
        pixel_format: Option<&str>,
        audio_stream_index: Option<u32>,
        frame_rate: Option<&str>,
        audio_codec: Option<&str>,
        video_codec: Option<&str>,
        color_transfer: Option<&str>,
        color_primaries: Option<&str>,
    ) -> Result<Vec<Arc<HlsSession>>> {
        // Destroy any existing variant sessions for this media
        self.destroy_media_sessions(media_id).await;

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

        for variant in &variants {
            let session = self
                .create_session(
                    media_id,
                    file_path,
                    duration_secs,
                    source_width,
                    source_height,
                    source_bitrate_kbps,
                    start_secs,
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

        self.media_variant_sessions
            .insert(media_id.to_string(), session_ids);

        Ok(sessions)
    }

    /// Create a single HLS session at the highest quality for fast seeking.
    /// Only spawns ONE FFmpeg process instead of one per variant, dramatically
    /// reducing seek latency. Returns a single-element Vec for API compatibility.
    pub async fn create_single_variant_session(
        &self,
        media_id: &str,
        file_path: &Path,
        duration_secs: Option<f64>,
        source_width: Option<u32>,
        source_height: Option<u32>,
        source_bitrate_kbps: Option<u32>,
        start_secs: f64,
        subtitle_path: Option<&Path>,
        pixel_format: Option<&str>,
        audio_stream_index: Option<u32>,
        frame_rate: Option<&str>,
        audio_codec: Option<&str>,
        video_codec: Option<&str>,
        color_transfer: Option<&str>,
        color_primaries: Option<&str>,
    ) -> Result<Vec<Arc<HlsSession>>> {
        // Destroy any existing sessions for this media
        self.destroy_media_sessions(media_id).await;

        let variants = ferrite_transcode::variants::select_variants(source_width, source_height);
        // Use only the highest quality variant (first in the list)
        let variant = variants.first().ok_or_else(|| anyhow!("No quality variants available"))?;

        info!(
            "Creating single HLS session for seek: media {} at {:.1}s variant={}",
            media_id, start_secs, variant.label,
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

        let session_ids = vec![session.session_id.clone()];
        self.media_variant_sessions
            .insert(media_id.to_string(), session_ids);

        Ok(vec![session])
    }

    /// Destroy all sessions (single + variant) for a media item.
    pub async fn destroy_media_sessions(&self, media_id: &str) {
        // Destroy single-variant session
        if let Some(sid) = self.media_sessions.get(media_id) {
            let old_sid = sid.clone();
            drop(sid);
            self.destroy_session(&old_sid).await;
        }
        // Destroy all variant sessions
        if let Some((_, sids)) = self.media_variant_sessions.remove(media_id) {
            for sid in sids {
                self.destroy_session(&sid).await;
            }
        }
    }

    /// Get all variant sessions for a media item (for master playlist generation).
    pub fn get_variant_sessions(&self, media_id: &str) -> Vec<Arc<HlsSession>> {
        if let Some(sids) = self.media_variant_sessions.get(media_id) {
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
    async fn spawn_ffmpeg(
        &self,
        file_path: &Path,
        output_dir: &Path,
        start_secs: f64,
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
    ) -> Result<Child> {
        // Only fall back to software encoding when we actually need CPU-side
        // frame access (subtitle burn-in or resolution scaling).
        // If the variant matches the source resolution, no scale filter is needed
        // and we can keep using the hardware encoder.
        let needs_scaling = variant.map_or(false, |v| {
            let src_h = source_height.unwrap_or(1080);
            v.height != src_h
        });
        let needs_software = (subtitle_path.is_some() || needs_scaling) && self.encoder.is_hardware();
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

        let mut args: Vec<String> = vec![
            "-hide_banner".into(),
            "-nostdin".into(),
        ];

        // HW-accelerated decoding args (before -i).
        // Only use HW decoding when we also have a scale filter, because
        // `-hwaccel_output_format cuda` keeps frames in GPU memory and the
        // HLS muxer can't consume them without an explicit hwdownload step.
        // CPU decode + NVENC encode is still very fast and avoids this issue.
        if needs_scaling && !needs_software {
            args.extend(effective_encoder.hw_input_args());
        }

        // Seek before input for fast seeking (demuxer-level)
        if start_secs > 0.5 {
            args.push("-ss".into());
            args.push(format!("{:.3}", start_secs));
        }

        let audio_map = format!("0:a:{}", audio_stream_index.unwrap_or(0));
        args.extend([
            "-i".into(),
            file_path.to_string_lossy().to_string(),
            // Map first video + selected audio stream
            "-map".into(),
            "0:v:0".into(),
            "-map".into(),
            audio_map,
        ]);

        // Build video filter chain: HDR tone-mapping + subtitle burn-in + scaling
        let mut vf_parts: Vec<String> = Vec::new();

        // High bit-depth handling: detect whether content is true HDR or just
        // 10-bit SDR, and apply the appropriate filter.
        let is_high_bit = pixel_format
            .map(|pf| ferrite_transcode::tonemap::is_high_bit_depth(pf))
            .unwrap_or(false);
        let needs_tonemap = is_high_bit
            && ferrite_transcode::tonemap::is_true_hdr(color_transfer, color_primaries);
        if needs_tonemap {
            info!("True HDR detected (pix={}, transfer={:?}, primaries={:?}), applying tone-mapping",
                pixel_format.unwrap_or("unknown"), color_transfer, color_primaries);
            vf_parts.push(ferrite_transcode::tonemap::tonemap_filter());
        } else if is_high_bit {
            info!("10-bit SDR detected (pix={}, transfer={:?}), applying bit-depth conversion only",
                pixel_format.unwrap_or("unknown"), color_transfer);
            vf_parts.push(ferrite_transcode::tonemap::bit_depth_filter());
        }

        if let Some(sub_path) = subtitle_path {
            let sub_path_escaped = crate::transcode::escape_ffmpeg_filter_path(&sub_path.to_string_lossy());
            vf_parts.push(format!("subtitles={}", sub_path_escaped));
            info!("HLS subtitle burn-in enabled: {}", sub_path.display());
        }

        if needs_scaling {
            if let Some(v) = variant {
                // Scale to variant resolution, maintaining aspect ratio.
                // -2 ensures even dimensions (required by H.264).
                vf_parts.push(format!("scale=-2:{}", v.height));
            }
        }

        if !vf_parts.is_empty() {
            args.extend(["-vf".into(), vf_parts.join(",")]);
        }

        // Determine if we can copy the video stream instead of re-encoding.
        // Video copy is possible when:
        // 1. Source is already H.264 (browser-compatible in HLS/fMP4)
        // 2. No video filters are needed (no scaling, subtitles, or tone-mapping)
        let video_is_h264 = video_codec
            .map(|c| c.to_lowercase() == "h264")
            .unwrap_or(false);
        let can_copy_video = video_is_h264 && vf_parts.is_empty();

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
            let fps = frame_rate
                .and_then(|fr| parse_frame_rate(fr))
                .unwrap_or(24.0);
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
            .map(|c| ferrite_transcode::audio::can_passthrough(c))
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
            "-hls_segment_type".into(),
            "fmp4".into(),
            "-hls_fmp4_init_filename".into(),
            "init.mp4".into(),
            "-hls_segment_filename".into(),
            "seg_%03d.m4s".into(),
            "-hls_flags".into(),
            "independent_segments+append_list".into(),
            "-hls_playlist_type".into(),
            "event".into(),
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

        // Spawn a task to log FFmpeg stderr
        if let Some(stderr) = child.stderr.take() {
            let session_id = output_dir
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            tokio::spawn(async move {
                use tokio::io::AsyncBufReadExt;
                let reader = tokio::io::BufReader::new(stderr);
                let mut lines = reader.lines();
                while let Ok(Some(line)) = lines.next_line().await {
                    // Log all FFmpeg output at warn level for debugging
                    warn!("ffmpeg HLS [{}]: {}", session_id, line);
                }
            });
        }

        Ok(child)
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

            let name = session
                .variant_label
                .as_deref()
                .unwrap_or("native");

            let variant_url = format!(
                "/api/stream/{}/hls/{}/playlist.m3u8{}",
                media_id, session.session_id, token_suffix
            );

            playlist.push_str(&format!(
                "\n#EXT-X-STREAM-INF:BANDWIDTH={},NAME=\"{}\"{}\n{}\n",
                bandwidth, name, resolution, variant_url
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
        session.touch().await;

        let playlist_path = session.output_dir.join("playlist.m3u8");
        let raw = tokio::fs::read_to_string(&playlist_path)
            .await
            .map_err(|e| anyhow!("Failed to read playlist: {}", e))?;

        let base_url = format!(
            "/api/stream/{}/hls/{}",
            media_id, session.session_id
        );
        let token_suffix = token
            .map(|t| format!("?token={}", percent_encode(t)))
            .unwrap_or_default();

        let rewritten = rewrite_playlist(&raw, &base_url, &token_suffix);
        Ok(rewritten)
    }

    /// Read a segment file from disk.
    ///
    /// For `init.mp4`, serves immediately once the file exists (it's written once
    /// before any segments). For `.m4s` segments, waits until the segment filename
    /// appears in the playlist's `#EXTINF` entries — this means FFmpeg has finished
    /// writing that segment and moved on to the next one, preventing partial reads.
    pub async fn get_segment(
        &self,
        session: &HlsSession,
        filename: &str,
    ) -> Result<Option<Vec<u8>>> {
        session.touch().await;

        // Validate filename to prevent path traversal
        if filename.contains("..") || filename.contains('/') || filename.contains('\\') {
            return Err(anyhow!("Invalid segment filename"));
        }

        let path = session.output_dir.join(filename);

        // init.mp4 is written once before any segments — serve as soon as it exists.
        if filename == "init.mp4" {
            for _ in 0..60 {
                if path.exists() {
                    let data = tokio::fs::read(&path).await?;
                    return Ok(Some(data));
                }
                // Bail early if FFmpeg has crashed
                if !session.is_ffmpeg_alive().await {
                    warn!("FFmpeg exited before init.mp4 was written for session {}", session.session_id);
                    return Ok(None);
                }
                tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            }
            return Ok(None);
        }

        // For .m4s segments, wait until FFmpeg has listed the segment in the playlist.
        // A segment appearing in a #EXTINF entry means FFmpeg has finished writing it
        // and moved on, so it's safe to read without getting a partial/truncated file.
        let playlist_path = session.output_dir.join("playlist.m3u8");

        // Poll up to 30 seconds (60 × 500ms)
        for _ in 0..60 {
            if let Ok(playlist) = tokio::fs::read_to_string(&playlist_path).await {
                if segment_listed_in_playlist(&playlist, filename) {
                    // Segment is finalized in the playlist — safe to read
                    if let Ok(data) = tokio::fs::read(&path).await {
                        return Ok(Some(data));
                    }
                }
            }
            // Bail early if FFmpeg has crashed — no point waiting for segments
            // that will never be written.
            if !session.is_ffmpeg_alive().await {
                // One final check: FFmpeg may have written this segment before dying
                if let Ok(playlist) = tokio::fs::read_to_string(&playlist_path).await {
                    if segment_listed_in_playlist(&playlist, filename) {
                        if let Ok(data) = tokio::fs::read(&path).await {
                            return Ok(Some(data));
                        }
                    }
                }
                warn!(
                    "FFmpeg exited before segment '{}' was ready for session {}",
                    filename, session.session_id
                );
                return Ok(None);
            }
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }

        // Final check after timeout
        if let Ok(playlist) = tokio::fs::read_to_string(&playlist_path).await {
            if segment_listed_in_playlist(&playlist, filename) {
                let data = tokio::fs::read(&path).await?;
                return Ok(Some(data));
            }
        }

        Ok(None)
    }

    /// Get a session by ID.
    pub fn get_session(&self, session_id: &str) -> Option<Arc<HlsSession>> {
        self.sessions.get(session_id).map(|s| s.clone())
    }

    /// Destroy a session: kill FFmpeg, remove files.
    pub async fn destroy_session(&self, session_id: &str) {
        if let Some((_, session)) = self.sessions.remove(session_id) {
            // Remove media → session mapping
            self.media_sessions
                .remove(&session.media_id);

            // Kill FFmpeg
            if let Some(mut child) = session.ffmpeg_handle.lock().await.take() {
                let _ = child.kill().await;
                debug!("Killed FFmpeg for HLS session {}", session_id);
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

            info!("Destroyed HLS session {}", session_id);
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

    /// Background cleanup loop — removes expired sessions.
    pub async fn cleanup_loop(self: Arc<Self>) {
        let mut interval =
            tokio::time::interval(std::time::Duration::from_secs(60));

        loop {
            interval.tick().await;

            let mut expired = Vec::new();
            for entry in self.sessions.iter() {
                if entry.value().idle_secs().await > self.session_timeout_secs {
                    expired.push(entry.key().clone());
                }
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
        if d > 0.0 { Some(n / d) } else { None }
    } else {
        fr.trim().parse().ok()
    }
}

/// Minimal percent-encoding for token in URL.
fn percent_encode(input: &str) -> String {
    percent_encoding::utf8_percent_encode(
        input,
        percent_encoding::NON_ALPHANUMERIC,
    )
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
