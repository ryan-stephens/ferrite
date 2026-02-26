use crate::error::ApiError;
use crate::state::AppState;
use axum::body::Body;
use axum::extract::{Path, Query, State};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use dashmap::DashMap;
use ferrite_core::config::HlsSegmentMimeMode;
use ferrite_db::{keyframe_repo, media_repo, stream_repo, subtitle_repo};
use ferrite_stream::compat::{self, StreamStrategy};
use ferrite_stream::{direct, transcode};
use serde::Deserialize;
use sqlx::SqlitePool;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::{Mutex, OwnedSemaphorePermit};
use tokio_util::io::ReaderStream;
use tracing::{debug, info, warn};
use uuid::Uuid;

/// Per-media lock to prevent duplicate concurrent keyframe probes.
/// When the first seek for a media item triggers a lazy probe, this lock
/// ensures that concurrent seeks for the same media wait rather than
/// spawning redundant ffprobe processes.
static KEYFRAME_PROBE_LOCKS: std::sync::LazyLock<DashMap<String, Arc<Mutex<()>>>> =
    std::sync::LazyLock::new(DashMap::new);

#[derive(Deserialize, Default)]
pub struct StreamQuery {
    /// Seek position in seconds (for transcoded streams)
    pub start: Option<f64>,
    /// External subtitle ID to burn into the video stream
    pub subtitle_id: Option<i64>,
    /// Audio stream index to select (0-based within audio streams). Defaults to first audio stream.
    pub audio_stream: Option<u32>,
    /// Optional explicit capability profile override (e.g. web-chrome, safari-ios, android, tvos, roku).
    pub client_profile: Option<String>,
    /// Seek behavior: fast (default, index-backed) or precise (ffprobe-backed).
    #[serde(default)]
    pub seek_mode: SeekMode,
}

#[derive(Deserialize)]
pub struct HlsPlaybackSessionQuery {
    pub playback_session_id: Option<String>,
}

#[derive(Deserialize)]
pub struct KeyframeQuery {
    pub time: f64,
    /// Seek behavior: fast (default, index-backed) or precise (ffprobe-backed).
    #[serde(default)]
    pub seek_mode: SeekMode,
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub enum SeekMode {
    #[default]
    Fast,
    Precise,
}

impl SeekMode {
    fn as_str(self) -> &'static str {
        match self {
            SeekMode::Fast => "fast",
            SeekMode::Precise => "precise",
        }
    }
}

fn build_hls_master_url(media_id: &str, requested_start: Option<f64>, suffix: &str) -> String {
    let base = format!("/api/stream/{media_id}/hls/master.m3u8");
    match (requested_start, suffix.is_empty()) {
        (Some(start), false) => format!("{base}?start={start:.3}{suffix}"),
        (Some(start), true) => format!("{base}?start={start:.3}"),
        (None, false) => format!("{base}?{}", suffix.trim_start_matches('&')),
        (None, true) => base,
    }
}

fn require_playback_session_id(playback_session_id: Option<&str>) -> Result<&str, ApiError> {
    playback_session_id
        .map(str::trim)
        .filter(|sid| !sid.is_empty())
        .ok_or_else(|| ApiError::bad_request("playback_session_id is required"))
}

fn can_reuse_seek_session(
    requested_start: f64,
    session_start: f64,
    buffered_end: f64,
    ffmpeg_alive: bool,
) -> bool {
    ffmpeg_alive && requested_start >= session_start && requested_start < buffered_end
}

fn strategy_label(strategy: &StreamStrategy) -> &'static str {
    match strategy {
        StreamStrategy::DirectPlay => "direct",
        StreamStrategy::Remux => "remux",
        StreamStrategy::AudioTranscode => "audio-transcode",
        StreamStrategy::FullTranscode => "full-transcode",
    }
}

fn header_str<'a>(headers: &'a HeaderMap, name: &str) -> Option<&'a str> {
    headers.get(name).and_then(|v| v.to_str().ok())
}

fn resolve_profile_override<'a>(
    query_profile: Option<&'a str>,
    headers: &'a HeaderMap,
) -> Option<&'a str> {
    query_profile.or_else(|| header_str(headers, "x-ferrite-client-profile"))
}

fn extract_bearer_token(headers: &HeaderMap) -> Option<String> {
    headers
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .map(str::to_string)
}

fn resolve_hls_token(query_token: Option<&str>, headers: &HeaderMap) -> Option<String> {
    query_token
        .map(str::to_string)
        .or_else(|| extract_bearer_token(headers))
}

fn build_seek_master_url_suffix(
    token: Option<&str>,
    playback_session_id: Option<&str>,
    seek_mode: SeekMode,
) -> String {
    let mut params: Vec<String> = Vec::new();

    if let Some(t) = token {
        params.push(format!(
            "token={}",
            percent_encoding::utf8_percent_encode(t, percent_encoding::NON_ALPHANUMERIC)
        ));
    }

    if let Some(sid) = playback_session_id {
        params.push(format!(
            "playback_session_id={}",
            percent_encoding::utf8_percent_encode(sid, percent_encoding::NON_ALPHANUMERIC)
        ));
    }

    if matches!(seek_mode, SeekMode::Precise) {
        params.push("seek_mode=precise".to_string());
    }

    if params.is_empty() {
        String::new()
    } else {
        format!("&{}", params.join("&"))
    }
}

async fn resolve_seek_start(
    state: &AppState,
    media_id: &str,
    file_path: &std::path::Path,
    requested_start: f64,
    seek_mode: SeekMode,
) -> (f64, &'static str, f64) {
    if requested_start <= 0.5 {
        return (requested_start, "none", 0.0);
    }

    let t0 = Instant::now();
    match seek_mode {
        SeekMode::Fast => {
            let resolved = match keyframe_repo::find_keyframe_before(
                &state.db.read,
                media_id,
                requested_start,
            )
            .await
            {
                Ok(Some(kf)) => (kf, "index"),
                Ok(None) => {
                    // No keyframes cached yet — spawn background probe so future seeks are fast.
                    // Fall back to requested_start for this immediate request to avoid blocking.
                    // FFmpeg will still seek to the nearest keyframe, but the reported start_secs
                    // might be slightly inaccurate until the index is built.
                    let state_clone = state.clone();
                    let media_id_clone = media_id.to_string();
                    let file_path_clone = file_path.to_path_buf();
                    tokio::spawn(async move {
                        lazy_probe_keyframes(&state_clone, &media_id_clone, &file_path_clone).await;
                    });

                    (requested_start, "requested-fallback")
                }
                Err(e) => {
                    warn!(
                        "Keyframe index lookup failed for media {} at {:.3}s: {}",
                        media_id, requested_start, e
                    );
                    (requested_start, "requested")
                }
            };
            (resolved.0, resolved.1, t0.elapsed().as_secs_f64() * 1000.0)
        }
        SeekMode::Precise => {
            let ffprobe_path = &state.config.transcode.ffprobe_path;
            let kf = transcode::find_keyframe_before(ffprobe_path, file_path, requested_start)
                .await
                .unwrap_or(requested_start);
            (kf, "ffprobe", t0.elapsed().as_secs_f64() * 1000.0)
        }
    }
}

/// Lazily probe and cache keyframe positions for a media item.
///
/// Uses a per-media lock to prevent duplicate concurrent probes when multiple
/// seeks arrive simultaneously for the same un-indexed media.
/// Returns `true` if keyframes were successfully probed and cached.
async fn lazy_probe_keyframes(
    state: &AppState,
    media_id: &str,
    file_path: &std::path::Path,
) -> bool {
    let lock = KEYFRAME_PROBE_LOCKS
        .entry(media_id.to_string())
        .or_insert_with(|| Arc::new(Mutex::new(())))
        .clone();

    let _guard = lock.lock().await;

    // Double-check: another task may have populated keyframes while we waited.
    match keyframe_repo::has_keyframes(&state.db.read, media_id).await {
        Ok(true) => return true,
        Ok(false) => {}
        Err(e) => {
            warn!("Keyframe existence check failed for {}: {}", media_id, e);
            return false;
        }
    }

    let ffprobe_path = &state.config.transcode.ffprobe_path;
    debug!(
        "Lazy-probing keyframes for media {} ({})",
        media_id,
        file_path.display()
    );

    let keyframes_ms =
        match ferrite_scanner::probe::probe_keyframe_index(ffprobe_path, file_path).await {
            Some(kfs) if !kfs.is_empty() => kfs,
            Some(_) => {
                debug!("No keyframes found for media {}", media_id);
                return false;
            }
            None => {
                warn!("Keyframe probe failed for media {}", media_id);
                return false;
            }
        };

    debug!(
        "Lazy-probed {} keyframes for media {}",
        keyframes_ms.len(),
        media_id
    );

    if let Err(e) =
        keyframe_repo::replace_keyframes_pool(&state.db.write, media_id, &keyframes_ms).await
    {
        warn!(
            "Failed to cache lazy-probed keyframes for {}: {}",
            media_id, e
        );
        return false;
    }

    true
}

async fn acquire_transcode_permit(
    state: &AppState,
    media_id: &str,
    operation: &'static str,
) -> Result<OwnedSemaphorePermit, ApiError> {
    let wait_started = Instant::now();
    let timeout =
        std::time::Duration::from_secs(state.config.transcode.transcode_queue_timeout_secs);
    match tokio::time::timeout(timeout, state.transcode_semaphore.clone().acquire_owned()).await {
        Ok(Ok(permit)) => {
            let wait_ms = wait_started.elapsed().as_secs_f64() * 1000.0;
            state.playback_metrics.record_timing(
                "transcode_queue_wait_ms",
                &[("operation", operation)],
                wait_ms,
            );
            info!(
                "Transcode permit acquired: op={} media_id={} wait_ms={:.1}",
                operation, media_id, wait_ms
            );
            Ok(permit)
        }
        Ok(Err(_)) => Err(ApiError::service_unavailable("Transcode queue unavailable")),
        Err(_) => {
            warn!(
                "Transcode queue timeout: op={} media_id={} timeout_secs={}",
                operation, media_id, state.config.transcode.transcode_queue_timeout_secs
            );
            Err(ApiError::service_unavailable(
                "Transcode queue timeout, try again later",
            ))
        }
    }
}

pub async fn stream_media(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(query): Query<StreamQuery>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let item = match media_repo::get_media_item(&state.db.read, &id).await {
        Ok(Some(item)) => item,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let file_path = std::path::Path::new(&item.file_path);

    // Determine streaming strategy per client capability profile.
    let explicit_profile = resolve_profile_override(query.client_profile.as_deref(), &headers);
    let client_profile = compat::resolve_client_profile(
        explicit_profile,
        header_str(&headers, "user-agent"),
        header_str(&headers, "sec-ch-ua-platform"),
    );
    let strategy = compat::determine_strategy_for_profile(
        client_profile,
        item.container_format.as_deref(),
        item.video_codec.as_deref(),
        item.audio_codec.as_deref(),
    );

    info!(
        "Stream {}: strategy={:?} profile={} (container={:?}, video={:?}, audio={:?})",
        item.title.as_deref().unwrap_or("unknown"),
        strategy,
        client_profile.as_str(),
        item.container_format,
        item.video_codec,
        item.audio_codec,
    );

    let duration_secs = item.duration_ms.map(|ms| ms as f64 / 1000.0);
    let startup_started = Instant::now();
    let strategy_metric = strategy_label(&strategy);
    let requested_start = query.start.unwrap_or(0.0);
    let pre_resolved_start_secs =
        if requested_start > 0.5 && !matches!(strategy, StreamStrategy::DirectPlay) {
            let (resolved_start_secs, _, _) =
                resolve_seek_start(&state, &id, file_path, requested_start, query.seek_mode).await;
            Some(resolved_start_secs)
        } else {
            None
        };

    let response = match strategy {
        StreamStrategy::DirectPlay => match direct::serve_file(file_path, &headers).await {
            Ok(response) => response.into_response(),
            Err(status) => status.into_response(),
        },
        StreamStrategy::Remux => {
            let _permit = match acquire_transcode_permit(&state, &id, "remux").await {
                Ok(permit) => permit,
                Err(err) => return err.into_response(),
            };
            let ffmpeg_path = &state.config.transcode.ffmpeg_path;
            let ffprobe_path = &state.config.transcode.ffprobe_path;
            let sub_path = resolve_subtitle_path(&state.db.read, query.subtitle_id).await;
            match transcode::serve_remux(
                ffmpeg_path,
                ffprobe_path,
                file_path,
                duration_secs,
                query.start,
                pre_resolved_start_secs,
                sub_path.as_deref(),
                &state.encoder_profile,
                query.audio_stream,
                item.video_codec.as_deref(),
            )
            .await
            {
                Ok(response) => response.into_response(),
                Err(status) => status.into_response(),
            }
        }
        StreamStrategy::AudioTranscode => {
            let _permit = match acquire_transcode_permit(&state, &id, "audio-transcode").await {
                Ok(permit) => permit,
                Err(err) => return err.into_response(),
            };
            let ffmpeg_path = &state.config.transcode.ffmpeg_path;
            let ffprobe_path = &state.config.transcode.ffprobe_path;
            let sub_path = resolve_subtitle_path(&state.db.read, query.subtitle_id).await;
            match transcode::serve_audio_transcode(
                ffmpeg_path,
                ffprobe_path,
                file_path,
                duration_secs,
                query.start,
                pre_resolved_start_secs,
                sub_path.as_deref(),
                &state.encoder_profile,
                query.audio_stream,
            )
            .await
            {
                Ok(response) => response.into_response(),
                Err(status) => status.into_response(),
            }
        }
        StreamStrategy::FullTranscode => {
            let _permit = match acquire_transcode_permit(&state, &id, "full-transcode").await {
                Ok(permit) => permit,
                Err(err) => return err.into_response(),
            };
            let ffmpeg_path = &state.config.transcode.ffmpeg_path;
            let ffprobe_path = &state.config.transcode.ffprobe_path;
            let sub_path = resolve_subtitle_path(&state.db.read, query.subtitle_id).await;
            let pixel_format = stream_repo::get_video_pixel_format(&state.db.read, &id)
                .await
                .unwrap_or(None);
            let color_meta = stream_repo::get_video_color_metadata(&state.db.read, &id)
                .await
                .unwrap_or(None);
            let color_transfer = color_meta.as_ref().and_then(|m| m.color_transfer.clone());
            let color_primaries = color_meta.as_ref().and_then(|m| m.color_primaries.clone());
            match transcode::serve_full_transcode(
                ffmpeg_path,
                ffprobe_path,
                file_path,
                duration_secs,
                query.start,
                pre_resolved_start_secs,
                sub_path.as_deref(),
                &state.encoder_profile,
                pixel_format.as_deref(),
                query.audio_stream,
                color_transfer.as_deref(),
                color_primaries.as_deref(),
            )
            .await
            {
                Ok(response) => response.into_response(),
                Err(status) => status.into_response(),
            }
        }
    };

    state.playback_metrics.record_timing(
        "playback_ttff_ms",
        &[
            ("path", "stream"),
            ("stream", strategy_metric),
            ("client_profile", client_profile.as_str()),
        ],
        startup_started.elapsed().as_secs_f64() * 1000.0,
    );

    response
}

/// Lightweight endpoint to find the nearest keyframe before a given time.
/// Used by the frontend to know the actual seek position before starting
/// the stream, so it can display the correct time on the scrubber.
pub async fn find_keyframe(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(query): Query<KeyframeQuery>,
) -> Result<impl IntoResponse, ApiError> {
    let t0 = Instant::now();

    let item = media_repo::get_media_item(&state.db.read, &id)
        .await?
        .ok_or_else(|| ApiError::not_found(format!("Media item '{id}' not found")))?;
    let db_ms = t0.elapsed().as_secs_f64() * 1000.0;

    let file_path = std::path::Path::new(&item.file_path);
    let requested = query.time.max(0.0);
    let (keyframe, source, seek_lookup_ms) =
        resolve_seek_start(&state, &id, file_path, requested, query.seek_mode).await;
    let total_ms = t0.elapsed().as_secs_f64() * 1000.0;

    info!(
        "Keyframe lookup for {}: requested={:.1}s keyframe={:.3}s mode={} source={} db={:.0}ms seek={:.0}ms total={:.0}ms",
        id,
        requested,
        keyframe,
        query.seek_mode.as_str(),
        source,
        db_ms,
        seek_lookup_ms,
        total_ms
    );

    let timing = format!(
        "db;dur={:.1}, seek;dur={:.1};desc=\"{}\", total;dur={:.1}",
        db_ms, seek_lookup_ms, source, total_ms
    );

    let mut headers = HeaderMap::new();
    headers.insert("Server-Timing", timing.parse().unwrap());

    Ok((
        headers,
        Json(serde_json::json!({
            "requested": requested,
            "keyframe": keyframe,
            "seek_mode": query.seek_mode.as_str(),
            "source": source,
            "timing_ms": { "db": db_ms, "seek": seek_lookup_ms, "total": total_ms },
        })),
    ))
}

// ---------------------------------------------------------------------------
// HLS Streaming Endpoints
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
pub struct HlsQuery {
    pub token: Option<String>,
    /// Optional playback session ownership key to isolate sessions per active player.
    pub playback_session_id: Option<String>,
    /// Start time in seconds for seeking. FFmpeg will begin transcoding from this point.
    pub start: Option<f64>,
    /// External subtitle ID to burn into the video stream
    pub subtitle_id: Option<i64>,
    /// Audio stream index to select (0-based within audio streams). Defaults to first audio stream.
    pub audio_stream: Option<u32>,
    /// Seek behavior: fast (default, index-backed) or precise (ffprobe-backed).
    #[serde(default)]
    pub seek_mode: SeekMode,
}

/// Resolve a subtitle_id to a file path on disk.
async fn resolve_subtitle_path(
    pool: &SqlitePool,
    subtitle_id: Option<i64>,
) -> Option<std::path::PathBuf> {
    let id = subtitle_id?;
    match subtitle_repo::get_subtitle_by_id(pool, id).await {
        Ok(Some(sub)) => {
            let path = std::path::PathBuf::from(&sub.file_path);
            if path.exists() {
                info!(
                    "Subtitle burn-in requested: id={}, path={}",
                    id, sub.file_path
                );
                Some(path)
            } else {
                warn!("Subtitle file not found on disk: {}", sub.file_path);
                None
            }
        }
        Ok(None) => {
            warn!("Subtitle id={} not found in database", id);
            None
        }
        Err(e) => {
            warn!("Failed to look up subtitle id={}: {}", id, e);
            None
        }
    }
}

/// GET /api/stream/{id}/hls/master.m3u8
/// Creates an HLS session (or reuses an existing one) and returns the master playlist.
pub async fn hls_master_playlist(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(query): Query<HlsQuery>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, ApiError> {
    let t0 = Instant::now();

    let item = media_repo::get_media_item(&state.db.read, &id)
        .await?
        .ok_or_else(|| ApiError::not_found(format!("Media item '{id}' not found")))?;
    let db_ms = t0.elapsed().as_secs_f64() * 1000.0;

    let file_path = std::path::Path::new(&item.file_path);
    let duration_secs = item.duration_ms.map(|ms| ms as f64 / 1000.0);

    // Extract auth token from query or Authorization header for playlist URL rewriting
    let token = resolve_hls_token(query.token.as_deref(), &headers);

    let requested_start = query.start.unwrap_or(0.0);
    let owner_key = ferrite_stream::hls::HlsSessionManager::owner_key(
        &id,
        query.playback_session_id.as_deref(),
    );

    let (start_secs, seek_source, seek_lookup_ms) =
        resolve_seek_start(&state, &id, file_path, requested_start, query.seek_mode).await;

    let sub_path = resolve_subtitle_path(&state.db.read, query.subtitle_id).await;

    // Fetch all video stream metadata in a single DB round-trip
    let video_meta = stream_repo::get_video_meta(&state.db.read, &id)
        .await
        .unwrap_or(None);
    let pixel_format = video_meta.as_ref().and_then(|m| m.pixel_format.clone());
    let frame_rate = video_meta.as_ref().and_then(|m| m.frame_rate.clone());
    let color_transfer = video_meta.as_ref().and_then(|m| m.color_transfer.clone());
    let color_primaries = video_meta.as_ref().and_then(|m| m.color_primaries.clone());

    // Check if we already have variant sessions for this media/playback owner.
    let t1 = Instant::now();
    let existing_variants = state.hls_sessions.get_variant_sessions_owned(&owner_key);
    let mut reused = !existing_variants.is_empty();
    // Promote a single-variant session (from initial playback) to full ABR ladder.
    // Only promotes if the session has the awaiting_promotion flag set (initial play path).
    // Seek-created sessions have awaiting_promotion=false and stay single-variant.
    let should_promote_ladder = reused
        && existing_variants.len() == 1
        && existing_variants[0]
            .awaiting_promotion
            .load(std::sync::atomic::Ordering::Acquire);
    let sessions = if should_promote_ladder {
        let _permit = acquire_transcode_permit(&state, &id, "hls-master").await?;
        reused = false;

        let create_result = state
            .hls_sessions
            .create_variant_sessions_owned(
                &owner_key,
                &id,
                file_path,
                duration_secs,
                item.width.map(|w| w as u32),
                item.height.map(|h| h as u32),
                item.bitrate_kbps.map(|b| b as u32),
                start_secs,
                requested_start,
                sub_path.as_deref(),
                pixel_format.as_deref(),
                query.audio_stream,
                frame_rate.as_deref(),
                item.audio_codec.as_deref(),
                item.video_codec.as_deref(),
                color_transfer.as_deref(),
                color_primaries.as_deref(),
            )
            .await;

        create_result.map_err(|e| {
            warn!("Failed to create HLS session for {}: {}", id, e);
            ApiError::internal(e.to_string())
        })?
    } else if reused {
        // Reuse existing variant sessions (touch them to keep alive)
        for s in &existing_variants {
            s.touch();
        }
        existing_variants
    } else {
        let _permit = acquire_transcode_permit(&state, &id, "hls-master").await?;

        // Start with a single variant at the highest quality for fastest TTFF.
        // The player re-polls master.m3u8 within a few seconds; the
        // should_promote_ladder check will then spawn the full ABR ladder.
        let create_result = state
            .hls_sessions
            .create_single_variant_session_owned(
                &owner_key,
                &id,
                file_path,
                duration_secs,
                item.width.map(|w| w as u32),
                item.height.map(|h| h as u32),
                item.bitrate_kbps.map(|b| b as u32),
                start_secs,
                requested_start,
                sub_path.as_deref(),
                pixel_format.as_deref(),
                query.audio_stream,
                frame_rate.as_deref(),
                item.audio_codec.as_deref(),
                item.video_codec.as_deref(),
                color_transfer.as_deref(),
                color_primaries.as_deref(),
                true, // awaiting_promotion = true for initial play
            )
            .await;

        create_result.map_err(|e| {
            warn!("Failed to create HLS session for {}: {}", id, e);
            ApiError::internal(e.to_string())
        })?
    };
    let session_ms = t1.elapsed().as_secs_f64() * 1000.0;

    let playlist = state
        .hls_sessions
        .generate_master_playlist(&sessions, &id, token.as_deref());

    let start = sessions.first().map(|s| s.start_secs).unwrap_or(0.0);
    let total_ms = t0.elapsed().as_secs_f64() * 1000.0;

    state.playback_metrics.record_timing(
        "playback_ttff_ms",
        &[("path", "hls_master"), ("stream", "hls")],
        total_ms,
    );

    info!(
        "HLS master playlist for {}: variants={} reused={} mode={} seek_source={} db={:.0}ms seek={:.0}ms session={:.0}ms total={:.0}ms",
        id,
        sessions.len(),
        reused,
        query.seek_mode.as_str(),
        seek_source,
        db_ms,
        seek_lookup_ms,
        session_ms,
        total_ms
    );

    let timing = format!(
        "db;dur={:.1}, seek;dur={:.1};desc=\"{}\", session;dur={:.1};desc=\"{}\", total;dur={:.1}",
        db_ms,
        seek_lookup_ms,
        seek_source,
        session_ms,
        if reused { "reused" } else { "created" },
        total_ms
    );

    // Build response with custom headers so the frontend knows the actual
    // media time and session IDs immediately (before MANIFEST_PARSED fires).
    let session_ids: Vec<String> = sessions.iter().map(|s| s.session_id.clone()).collect();
    let mut resp_headers = HeaderMap::new();
    resp_headers.insert(
        header::CONTENT_TYPE,
        "application/vnd.apple.mpegurl".parse().unwrap(),
    );
    resp_headers.insert(header::CACHE_CONTROL, "no-store".parse().unwrap());
    let video_copied = sessions.first().map(|s| s.video_copied).unwrap_or(false);
    resp_headers.insert("x-hls-start-secs", format!("{:.3}", start).parse().unwrap()); resp_headers.insert("x-hls-requested-start", format!("{:.3}", requested_start).parse().unwrap());
    resp_headers.insert(
        "x-hls-video-copied",
        if video_copied { "1" } else { "0" }.parse().unwrap(),
    );
    resp_headers.insert("x-hls-session-ids", session_ids.join(",").parse().unwrap());
    resp_headers.insert("Server-Timing", timing.parse().unwrap());

    Ok((StatusCode::OK, resp_headers, playlist))
}

/// POST /api/stream/{id}/hls/session/start
/// Returns an explicit playback session id and a canonical HLS master URL.
pub async fn hls_session_start(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(query): Query<HlsQuery>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, ApiError> {
    media_repo::get_media_item(&state.db.read, &id)
        .await?
        .ok_or_else(|| ApiError::not_found(format!("Media item '{id}' not found")))?;

    let playback_session_id = query
        .playback_session_id
        .as_deref()
        .map(str::trim)
        .filter(|sid| !sid.is_empty())
        .map(str::to_string)
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    let token = resolve_hls_token(query.token.as_deref(), &headers);
    let master_url_suffix = build_seek_master_url_suffix(
        token.as_deref(),
        Some(playback_session_id.as_str()),
        query.seek_mode,
    );
    let master_url = build_hls_master_url(&id, query.start, &master_url_suffix);

    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({
            "playback_session_id": playback_session_id,
            "master_url": master_url,
        })),
    ))
}

/// POST /api/stream/{id}/hls/session/heartbeat
/// Touches active sessions for a playback owner so idle cleanup doesn't reap them.
pub async fn hls_session_heartbeat(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(query): Query<HlsPlaybackSessionQuery>,
) -> Result<impl IntoResponse, ApiError> {
    let playback_session_id = require_playback_session_id(query.playback_session_id.as_deref())?;
    let owner_key =
        ferrite_stream::hls::HlsSessionManager::owner_key(&id, Some(playback_session_id));

    let mut touched = 0usize;
    let sessions = state.hls_sessions.get_variant_sessions_owned(&owner_key);
    if sessions.is_empty() {
        if let Some(session) = state.hls_sessions.get_session_for_owner(&owner_key) {
            session.touch();
            touched = 1;
        }
    } else {
        for session in sessions {
            session.touch();
            touched += 1;
        }
    }

    Ok(Json(serde_json::json!({
        "playback_session_id": playback_session_id,
        "active_sessions": touched,
    })))
}

/// GET /api/stream/{id}/hls/{session_id}/playlist.m3u8
/// Returns the variant playlist with URLs rewritten to absolute API paths.
pub async fn hls_variant_playlist(
    State(state): State<AppState>,
    Path((id, session_id)): Path<(String, String)>,
    Query(query): Query<HlsQuery>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, ApiError> {
    let session = state
        .hls_sessions
        .get_session(&session_id)
        .ok_or_else(|| ApiError::not_found(format!("HLS session '{session_id}' not found")))?;

    let token = resolve_hls_token(query.token.as_deref(), &headers);

    let playlist = state
        .hls_sessions
        .get_variant_playlist(&session, &id, token.as_deref())
        .await
        .map_err(|e| {
            warn!(
                "Failed to read HLS playlist for session {}: {}",
                session_id, e
            );
            ApiError::internal(e.to_string())
        })?;

    let mut resp_headers = HeaderMap::new();
    resp_headers.insert(
        header::CONTENT_TYPE,
        "application/vnd.apple.mpegurl".parse().unwrap(),
    );
    resp_headers.insert(header::CACHE_CONTROL, "no-store".parse().unwrap());

    Ok((StatusCode::OK, resp_headers, playlist))
}

fn hls_segment_content_type(filename: &str, mode: HlsSegmentMimeMode) -> &'static str {
    if filename.ends_with(".mp4") {
        return "video/mp4";
    }

    if filename.ends_with(".m4s") {
        return match mode {
            HlsSegmentMimeMode::VideoMp4 => "video/mp4",
            HlsSegmentMimeMode::VideoIsoSegment => "video/iso.segment",
        };
    }

    "application/octet-stream"
}

/// GET /api/stream/{id}/hls/{session_id}/{filename}
/// Serves an HLS segment (init.mp4 or seg_NNN.m4s) from disk.
/// Segments are streamed directly from disk to the response body without
/// buffering the entire file in memory, reducing peak memory usage and
/// improving time-to-first-byte for large segments (2-6MB each).
pub async fn hls_segment(
    State(state): State<AppState>,
    Path((_id, session_id, filename)): Path<(String, String, String)>,
) -> Result<impl IntoResponse, ApiError> {
    let t0 = Instant::now();

    let session = state
        .hls_sessions
        .get_session(&session_id)
        .ok_or_else(|| ApiError::not_found(format!("HLS session '{session_id}' not found")))?;

    // Wait for the segment to be ready (polls playlist until FFmpeg finalizes it)
    let wait_started = Instant::now();
    let path = state
        .hls_sessions
        .wait_for_segment(&session, &filename)
        .await
        .map_err(|e| {
            warn!(
                "Failed to wait for HLS segment {} for session {}: {}",
                filename, session_id, e
            );
            ApiError::internal(e.to_string())
        })?
        .ok_or_else(|| ApiError::not_found(format!("HLS segment '{filename}' not found")))?;
    let wait_ms = wait_started.elapsed().as_secs_f64() * 1000.0;

    state.playback_metrics.record_timing(
        "hls_segment_wait_ms",
        &[("path", "hls_segment")],
        wait_ms,
    );

    let total_ms = t0.elapsed().as_secs_f64() * 1000.0;

    // Open the file and stream it directly to the response — no full-file buffering.
    let file = tokio::fs::File::open(&path).await.map_err(|e| {
        warn!("Failed to open HLS segment file {:?}: {}", path, e);
        ApiError::internal(e.to_string())
    })?;
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    let content_type =
        hls_segment_content_type(&filename, state.config.transcode.hls_segment_mime_mode);

    let timing = format!("total;dur={:.1}", total_ms);
    let mut resp_headers = HeaderMap::new();
    resp_headers.insert(header::CONTENT_TYPE, content_type.parse().unwrap());
    resp_headers.insert(header::CACHE_CONTROL, "max-age=3600".parse().unwrap());
    resp_headers.insert("Server-Timing", timing.parse().unwrap());

    Ok((StatusCode::OK, resp_headers, body))
}

/// POST /api/stream/{id}/hls/seek?start=123.456
/// Seeks to the specified time. If the requested time is within the already-buffered
/// range of the current session, reuses it (returns reused=true) so the frontend can
/// seek within the existing HLS.js instance without spawning a new FFmpeg process.
/// Otherwise destroys the old session and creates a new one.
pub async fn hls_seek(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(query): Query<HlsQuery>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, ApiError> {
    let t0 = Instant::now();
    let requested_start = query.start.unwrap_or(0.0);
    let owner_key = ferrite_stream::hls::HlsSessionManager::owner_key(
        &id,
        query.playback_session_id.as_deref(),
    );

    let token = resolve_hls_token(query.token.as_deref(), &headers);
    let master_url_suffix = build_seek_master_url_suffix(
        token.as_deref(),
        query.playback_session_id.as_deref(),
        query.seek_mode,
    );

    // Fast path: if the current session already covers the requested time, reuse it.
    // This avoids destroying and recreating an FFmpeg process for seeks that land
    // within the already-transcoded buffer (e.g. sequential +10s presses).
    if let Some(existing) = state.hls_sessions.get_session_for_owner(&owner_key) {
        let ffmpeg_alive = existing.is_ffmpeg_alive().await;
        // Read the playlist to get the true available window on disk, accounting
        // for both the encode-ahead (upper bound) and segment deletion (lower bound).
        let reuse = if let Some((available_start, available_end)) =
            existing.playlist_available_range().await
        {
            can_reuse_seek_session(
                requested_start,
                available_start,
                available_end,
                ffmpeg_alive,
            )
        } else {
            // Playlist unreadable — fall back to cached estimate
            let buffered_end = existing.start_secs + existing.buffered_secs();
            can_reuse_seek_session(
                requested_start,
                existing.start_secs,
                buffered_end,
                ffmpeg_alive,
            )
        };

        if reuse {
            existing.touch();
            let total_ms = t0.elapsed().as_secs_f64() * 1000.0;
            info!(
                "HLS seek for {} reused session {} (target={:.1}s) total={:.0}ms",
                id, existing.session_id, requested_start, total_ms
            );
            state.playback_metrics.record_timing(
                "seek_latency_ms",
                &[("path", "hls_seek"), ("mode", "reused")],
                total_ms,
            );
            return Ok(Json(serde_json::json!({
                "session_id": existing.session_id,
                "start_secs": existing.start_secs, "requested_start": requested_start,
                "video_copied": existing.video_copied,
                "reused": true,
                "variant_count": 1,
                "master_url": build_hls_master_url(&id, Some(requested_start), &master_url_suffix),
                "timing_ms": { "db": 0, "session": 0, "total": total_ms },
            })));
        }
    }

    let item = media_repo::get_media_item(&state.db.read, &id)
        .await?
        .ok_or_else(|| ApiError::not_found(format!("Media item '{id}' not found")))?;
    let db_ms = t0.elapsed().as_secs_f64() * 1000.0;

    let file_path = std::path::Path::new(&item.file_path);
    let duration_secs = item.duration_ms.map(|ms| ms as f64 / 1000.0);

    let (start_secs, seek_source, seek_lookup_ms) =
        resolve_seek_start(&state, &id, file_path, requested_start, query.seek_mode).await;

    let sub_path = resolve_subtitle_path(&state.db.read, query.subtitle_id).await;

    // Fetch all video stream metadata in a single DB round-trip
    let video_meta = stream_repo::get_video_meta(&state.db.read, &id)
        .await
        .unwrap_or(None);
    let pixel_format = video_meta.as_ref().and_then(|m| m.pixel_format.clone());
    let frame_rate = video_meta.as_ref().and_then(|m| m.frame_rate.clone());
    let color_transfer = video_meta.as_ref().and_then(|m| m.color_transfer.clone());
    let color_primaries = video_meta.as_ref().and_then(|m| m.color_primaries.clone());

    let _permit = acquire_transcode_permit(&state, &id, "hls-seek").await?;

    // Create a single variant session for fast seeking (1 FFmpeg process instead of N).
    let t1 = Instant::now();
    let sessions = state
        .hls_sessions
        .create_single_variant_session_owned(
            &owner_key,
            &id,
            file_path,
            duration_secs,
            item.width.map(|w| w as u32),
            item.height.map(|h| h as u32),
            item.bitrate_kbps.map(|b| b as u32),
            start_secs,
            requested_start,
            sub_path.as_deref(),
            pixel_format.as_deref(),
            query.audio_stream,
            frame_rate.as_deref(),
            item.audio_codec.as_deref(),
            item.video_codec.as_deref(),
            color_transfer.as_deref(),
            color_primaries.as_deref(),
            false, // awaiting_promotion = false for seek
        )
        .await
        .map_err(|e| {
            warn!("Failed to create HLS seek session for {}: {}", id, e);
            ApiError::internal(e.to_string())
        })?;
    let session_ms = t1.elapsed().as_secs_f64() * 1000.0;
    let total_ms = t0.elapsed().as_secs_f64() * 1000.0;

    state.playback_metrics.record_timing(
        "seek_latency_ms",
        &[("path", "hls_seek"), ("mode", "new-session")],
        total_ms,
    );

    let first = sessions
        .first()
        .ok_or_else(|| ApiError::internal("No variant sessions created"))?;

    info!(
        "HLS seek for {}: start={:.1}s mode={} seek_source={} db={:.0}ms seek={:.0}ms session={:.0}ms total={:.0}ms",
        id,
        start_secs,
        query.seek_mode.as_str(),
        seek_source,
        db_ms,
        seek_lookup_ms,
        session_ms,
        total_ms
    );

    // Return the new session info so the frontend can switch to it
    Ok(Json(serde_json::json!({
        "session_id": first.session_id,
        "start_secs": first.start_secs, "requested_start": requested_start,
        "video_copied": first.video_copied,
        "reused": false,
        "variant_count": sessions.len(),
        "master_url": build_hls_master_url(&id, Some(requested_start), &master_url_suffix),
        "seek_source": seek_source,
        "timing_ms": { "db": db_ms, "seek": seek_lookup_ms, "session": session_ms, "total": total_ms },
    })))
}

/// DELETE /api/stream/{id}/hls
/// Destroys ALL HLS sessions for a media item by media ID.
/// Preferred over the session-ID variant because the frontend always knows
/// the media ID, even before MANIFEST_PARSED has fired.
pub async fn hls_stop_media(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(query): Query<HlsQuery>,
) -> impl IntoResponse {
    let owner_key = ferrite_stream::hls::HlsSessionManager::owner_key(
        &id,
        query.playback_session_id.as_deref(),
    );
    state.hls_sessions.destroy_owner_sessions(&owner_key).await;
    StatusCode::NO_CONTENT
}

/// DELETE /api/stream/{id}/hls/session/stop
/// Explicit lifecycle stop keyed by playback_session_id.
pub async fn hls_session_stop(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(query): Query<HlsPlaybackSessionQuery>,
) -> Result<impl IntoResponse, ApiError> {
    let playback_session_id = require_playback_session_id(query.playback_session_id.as_deref())?;
    let owner_key =
        ferrite_stream::hls::HlsSessionManager::owner_key(&id, Some(playback_session_id));
    state.hls_sessions.destroy_owner_sessions(&owner_key).await;
    Ok(StatusCode::NO_CONTENT)
}

/// DELETE /api/stream/{id}/hls/{session_id}
/// Destroys an HLS session (kills FFmpeg, removes files).
pub async fn hls_stop(
    State(state): State<AppState>,
    Path((_id, session_id)): Path<(String, String)>,
) -> impl IntoResponse {
    state.hls_sessions.destroy_session(&session_id).await;
    StatusCode::NO_CONTENT
}

#[cfg(test)]
mod tests {
    use super::{
        build_hls_master_url, build_seek_master_url_suffix, can_reuse_seek_session,
        hls_segment_content_type, require_playback_session_id, resolve_hls_token,
        resolve_profile_override, SeekMode,
    };
    use axum::http::{header, HeaderMap, HeaderValue};
    use ferrite_core::config::HlsSegmentMimeMode;

    fn hls_segment_content_type_matrix() -> [(&'static str, HlsSegmentMimeMode, &'static str); 6] {
        [
            ("init.mp4", HlsSegmentMimeMode::VideoMp4, "video/mp4"),
            ("init.mp4", HlsSegmentMimeMode::VideoIsoSegment, "video/mp4"),
            ("seg_001.m4s", HlsSegmentMimeMode::VideoMp4, "video/mp4"),
            (
                "seg_001.m4s",
                HlsSegmentMimeMode::VideoIsoSegment,
                "video/iso.segment",
            ),
            (
                "segment.bin",
                HlsSegmentMimeMode::VideoMp4,
                "application/octet-stream",
            ),
            (
                "segment.bin",
                HlsSegmentMimeMode::VideoIsoSegment,
                "application/octet-stream",
            ),
        ]
    }

    #[test]
    fn profile_override_prefers_query_param_over_header() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "x-ferrite-client-profile",
            HeaderValue::from_static("android"),
        );

        let resolved = resolve_profile_override(Some("roku"), &headers);
        assert_eq!(resolved, Some("roku"));
    }

    #[test]
    fn profile_override_uses_header_when_query_missing() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "x-ferrite-client-profile",
            HeaderValue::from_static("safari-ios"),
        );

        let resolved = resolve_profile_override(None, &headers);
        assert_eq!(resolved, Some("safari-ios"));
    }

    #[test]
    fn hls_token_prefers_query_over_authorization_header() {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_static("Bearer from-header"),
        );

        let token = resolve_hls_token(Some("from-query"), &headers);
        assert_eq!(token.as_deref(), Some("from-query"));
    }

    #[test]
    fn seek_master_url_suffix_contains_encoded_contract_fields() {
        let suffix = build_seek_master_url_suffix(
            Some("a token+value"),
            Some("session/one"),
            SeekMode::Precise,
        );

        assert_eq!(
            suffix,
            "&token=a%20token%2Bvalue&playback_session_id=session%2Fone&seek_mode=precise"
        );
    }

    #[test]
    fn seek_master_url_suffix_omits_default_fast_mode() {
        let suffix = build_seek_master_url_suffix(None, Some("sid"), SeekMode::Fast);
        assert_eq!(suffix, "&playback_session_id=sid");
    }

    #[test]
    fn hls_master_url_with_start_and_suffix_contract() {
        let url = build_hls_master_url("media-1", Some(123.456), "&playback_session_id=sid");
        assert_eq!(
            url,
            "/api/stream/media-1/hls/master.m3u8?start=123.456&playback_session_id=sid"
        );
    }

    #[test]
    fn hls_master_url_without_start_uses_question_mark_contract() {
        let url = build_hls_master_url("media-1", None, "&playback_session_id=sid");
        assert_eq!(
            url,
            "/api/stream/media-1/hls/master.m3u8?playback_session_id=sid"
        );
    }

    #[test]
    fn require_playback_session_id_contract() {
        assert!(require_playback_session_id(Some("sid-123")).is_ok());
        assert!(require_playback_session_id(Some("   ")).is_err());
        assert!(require_playback_session_id(None).is_err());
    }

    #[test]
    fn seek_reuse_contract_within_buffer_and_alive() {
        assert!(can_reuse_seek_session(35.0, 10.0, 40.0, true));
    }

    #[test]
    fn seek_reuse_contract_outside_buffer_or_not_alive() {
        assert!(!can_reuse_seek_session(41.0, 10.0, 40.0, true));
        assert!(!can_reuse_seek_session(35.0, 10.0, 40.0, false));
    }

    #[test]
    fn hls_segment_content_type_for_init_mp4() {
        assert_eq!(
            hls_segment_content_type("init.mp4", HlsSegmentMimeMode::VideoMp4),
            "video/mp4"
        );
    }

    #[test]
    fn hls_segment_content_type_matrix_covers_modes() {
        for (filename, mode, expected) in hls_segment_content_type_matrix() {
            assert_eq!(hls_segment_content_type(filename, mode), expected);
        }
    }

    #[test]
    fn hls_segment_content_type_fallback() {
        assert_eq!(
            hls_segment_content_type("segment.unknown", HlsSegmentMimeMode::VideoMp4),
            "application/octet-stream"
        );
    }
}
