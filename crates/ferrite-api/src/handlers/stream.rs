use crate::error::ApiError;
use crate::state::AppState;
use axum::extract::{Path, Query, State};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use ferrite_db::{media_repo, stream_repo, subtitle_repo};
use ferrite_stream::compat::{self, StreamStrategy};
use ferrite_stream::{direct, transcode};
use serde::Deserialize;
use sqlx::SqlitePool;
use std::time::Instant;
use tracing::{info, warn};

#[derive(Deserialize, Default)]
pub struct StreamQuery {
    /// Seek position in seconds (for transcoded streams)
    pub start: Option<f64>,
    /// External subtitle ID to burn into the video stream
    pub subtitle_id: Option<i64>,
    /// Audio stream index to select (0-based within audio streams). Defaults to first audio stream.
    pub audio_stream: Option<u32>,
}

#[derive(Deserialize)]
pub struct KeyframeQuery {
    pub time: f64,
}

pub async fn stream_media(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(query): Query<StreamQuery>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let item = match media_repo::get_media_item(&state.db, &id).await {
        Ok(Some(item)) => item,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let file_path = std::path::Path::new(&item.file_path);

    // Determine streaming strategy based on codec compatibility
    let strategy = compat::determine_strategy(
        item.container_format.as_deref(),
        item.video_codec.as_deref(),
        item.audio_codec.as_deref(),
    );

    info!(
        "Stream {}: strategy={:?} (container={:?}, video={:?}, audio={:?})",
        item.title.as_deref().unwrap_or("unknown"),
        strategy,
        item.container_format,
        item.video_codec,
        item.audio_codec,
    );

    let duration_secs = item.duration_ms.map(|ms| ms as f64 / 1000.0);

    match strategy {
        StreamStrategy::DirectPlay => {
            match direct::serve_file(file_path, &headers).await {
                Ok(response) => response.into_response(),
                Err(status) => status.into_response(),
            }
        }
        StreamStrategy::Remux => {
            // Zero-cost remux: both codecs are compatible, just the container is wrong.
            // No transcode permit needed since this is essentially just a copy operation,
            // but we still acquire one to limit concurrent FFmpeg processes.
            let _permit = match state.transcode_semaphore.try_acquire() {
                Ok(permit) => permit,
                Err(_) => {
                    warn!("Transcode limit reached, rejecting remux request for {}", id);
                    return (StatusCode::SERVICE_UNAVAILABLE, "Transcode limit reached, try again later").into_response();
                }
            };
            let ffmpeg_path = &state.config.transcode.ffmpeg_path;
            let ffprobe_path = &state.config.transcode.ffprobe_path;
            let sub_path = resolve_subtitle_path(&state.db, query.subtitle_id).await;
            match transcode::serve_remux(
                ffmpeg_path,
                ffprobe_path,
                file_path,
                duration_secs,
                query.start,
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
            // Acquire transcode permit to enforce max_concurrent_transcodes
            let _permit = match state.transcode_semaphore.try_acquire() {
                Ok(permit) => permit,
                Err(_) => {
                    warn!("Transcode limit reached, rejecting audio transcode request for {}", id);
                    return (StatusCode::SERVICE_UNAVAILABLE, "Transcode limit reached, try again later").into_response();
                }
            };
            let ffmpeg_path = &state.config.transcode.ffmpeg_path;
            let ffprobe_path = &state.config.transcode.ffprobe_path;
            let sub_path = resolve_subtitle_path(&state.db, query.subtitle_id).await;
            match transcode::serve_audio_transcode(
                ffmpeg_path,
                ffprobe_path,
                file_path,
                duration_secs,
                query.start,
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
            // Acquire transcode permit to enforce max_concurrent_transcodes
            let _permit = match state.transcode_semaphore.try_acquire() {
                Ok(permit) => permit,
                Err(_) => {
                    warn!("Transcode limit reached, rejecting full transcode request for {}", id);
                    return (StatusCode::SERVICE_UNAVAILABLE, "Transcode limit reached, try again later").into_response();
                }
            };
            let ffmpeg_path = &state.config.transcode.ffmpeg_path;
            let ffprobe_path = &state.config.transcode.ffprobe_path;
            let sub_path = resolve_subtitle_path(&state.db, query.subtitle_id).await;
            let pixel_format = stream_repo::get_video_pixel_format(&state.db, &id)
                .await
                .unwrap_or(None);
            let color_meta = stream_repo::get_video_color_metadata(&state.db, &id)
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
    }
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

    let item = media_repo::get_media_item(&state.db, &id)
        .await?
        .ok_or_else(|| ApiError::not_found(format!("Media item '{id}' not found")))?;
    let db_ms = t0.elapsed().as_secs_f64() * 1000.0;

    let file_path = std::path::Path::new(&item.file_path);
    let ffprobe_path = &state.config.transcode.ffprobe_path;

    let t1 = Instant::now();
    let keyframe = transcode::find_keyframe_before(ffprobe_path, file_path, query.time)
        .await
        .unwrap_or(query.time);
    let ffprobe_ms = t1.elapsed().as_secs_f64() * 1000.0;
    let total_ms = t0.elapsed().as_secs_f64() * 1000.0;

    info!(
        "Keyframe lookup for {}: requested={:.1}s keyframe={:.3}s db={:.0}ms ffprobe={:.0}ms total={:.0}ms",
        id, query.time, keyframe, db_ms, ffprobe_ms, total_ms
    );

    let timing = format!(
        "db;dur={:.1}, ffprobe;dur={:.1}, total;dur={:.1}",
        db_ms, ffprobe_ms, total_ms
    );

    let mut headers = HeaderMap::new();
    headers.insert("Server-Timing", timing.parse().unwrap());

    Ok((headers, Json(serde_json::json!({
        "requested": query.time,
        "keyframe": keyframe,
        "timing_ms": { "db": db_ms, "ffprobe": ffprobe_ms, "total": total_ms },
    }))))
}

// ---------------------------------------------------------------------------
// HLS Streaming Endpoints
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
pub struct HlsQuery {
    pub token: Option<String>,
    /// Start time in seconds for seeking. FFmpeg will begin transcoding from this point.
    pub start: Option<f64>,
    /// External subtitle ID to burn into the video stream
    pub subtitle_id: Option<i64>,
    /// Audio stream index to select (0-based within audio streams). Defaults to first audio stream.
    pub audio_stream: Option<u32>,
}

/// Resolve a subtitle_id to a file path on disk.
async fn resolve_subtitle_path(pool: &SqlitePool, subtitle_id: Option<i64>) -> Option<std::path::PathBuf> {
    let id = subtitle_id?;
    match subtitle_repo::get_subtitle_by_id(pool, id).await {
        Ok(Some(sub)) => {
            let path = std::path::PathBuf::from(&sub.file_path);
            if path.exists() {
                info!("Subtitle burn-in requested: id={}, path={}", id, sub.file_path);
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

    let item = media_repo::get_media_item(&state.db, &id)
        .await?
        .ok_or_else(|| ApiError::not_found(format!("Media item '{id}' not found")))?;
    let db_ms = t0.elapsed().as_secs_f64() * 1000.0;

    let file_path = std::path::Path::new(&item.file_path);
    let duration_secs = item.duration_ms.map(|ms| ms as f64 / 1000.0);

    // Extract auth token from query or Authorization header for playlist URL rewriting
    let token = query.token.clone().or_else(|| {
        headers
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "))
            .map(|t| t.to_string())
    });

    let start_secs = query.start.unwrap_or(0.0);

    let sub_path = resolve_subtitle_path(&state.db, query.subtitle_id).await;

    // Fetch video pixel format for HDR tone-mapping detection
    let pixel_format = stream_repo::get_video_pixel_format(&state.db, &id)
        .await
        .unwrap_or(None);

    // Fetch video frame rate for accurate GOP calculation
    let frame_rate = stream_repo::get_video_frame_rate(&state.db, &id)
        .await
        .unwrap_or(None);

    // Fetch color metadata for HDR vs 10-bit SDR distinction
    let color_meta = stream_repo::get_video_color_metadata(&state.db, &id)
        .await
        .unwrap_or(None);
    let color_transfer = color_meta.as_ref().and_then(|m| m.color_transfer.clone());
    let color_primaries = color_meta.as_ref().and_then(|m| m.color_primaries.clone());

    // Check if we already have variant sessions for this media.
    // After a seek, there will be a single-variant session — reuse it instead
    // of spawning 4 new ABR variants (which would add ~4.5s of latency).
    let t1 = Instant::now();
    let existing_variants = state.hls_sessions.get_variant_sessions(&id);
    let reused = !existing_variants.is_empty();
    let sessions = if reused {
        // Reuse existing variant sessions (touch them to keep alive)
        for s in &existing_variants {
            s.touch().await;
        }
        existing_variants
    } else {
        // Acquire transcode permit before spawning FFmpeg (enforces max_concurrent_transcodes).
        let _permit = match state.transcode_semaphore.try_acquire() {
            Ok(permit) => permit,
            Err(_) => {
                warn!("Transcode limit reached, rejecting HLS session for {}", id);
                return Err(ApiError::service_unavailable(
                    "Transcode limit reached, try again later",
                ));
            }
        };

        // Both paths use single-variant for fast startup
        state
            .hls_sessions
            .create_single_variant_session(
                &id,
                file_path,
                duration_secs,
                item.width.map(|w| w as u32),
                item.height.map(|h| h as u32),
                item.bitrate_kbps.map(|b| b as u32),
                start_secs,
                sub_path.as_deref(),
                pixel_format.as_deref(),
                query.audio_stream,
                frame_rate.as_deref(),
                item.audio_codec.as_deref(),
                item.video_codec.as_deref(),
                color_transfer.as_deref(),
                color_primaries.as_deref(),
            )
            .await
            .map_err(|e| {
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

    info!(
        "HLS master playlist for {}: variants={} reused={} db={:.0}ms session={:.0}ms total={:.0}ms",
        id, sessions.len(), reused, db_ms, session_ms, total_ms
    );

    let timing = format!(
        "db;dur={:.1}, session;dur={:.1};desc=\"{}\", total;dur={:.1}",
        db_ms, session_ms, if reused { "reused" } else { "created" }, total_ms
    );

    // Build response with custom header for start offset so the frontend
    // knows the actual media time this HLS stream starts at.
    let mut resp_headers = HeaderMap::new();
    resp_headers.insert(header::CONTENT_TYPE, "application/vnd.apple.mpegurl".parse().unwrap());
    resp_headers.insert("x-hls-start-secs", format!("{:.3}", start).parse().unwrap());
    resp_headers.insert("Server-Timing", timing.parse().unwrap());

    Ok((StatusCode::OK, resp_headers, playlist))
}

/// GET /api/stream/{id}/hls/{session_id}/playlist.m3u8
/// Returns the variant playlist with URLs rewritten to absolute API paths.
pub async fn hls_variant_playlist(
    State(state): State<AppState>,
    Path((id, session_id)): Path<(String, String)>,
    Query(query): Query<HlsQuery>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, ApiError> {
    let session = state.hls_sessions.get_session(&session_id)
        .ok_or_else(|| ApiError::not_found(format!("HLS session '{session_id}' not found")))?;

    let token = query.token.clone().or_else(|| {
        headers
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "))
            .map(|t| t.to_string())
    });

    let playlist = state
        .hls_sessions
        .get_variant_playlist(&session, &id, token.as_deref())
        .await
        .map_err(|e| {
            warn!("Failed to read HLS playlist for session {}: {}", session_id, e);
            ApiError::internal(e.to_string())
        })?;

    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/vnd.apple.mpegurl")],
        playlist,
    ))
}

/// GET /api/stream/{id}/hls/{session_id}/{filename}
/// Serves an HLS segment (init.mp4 or seg_NNN.m4s) from disk.
pub async fn hls_segment(
    State(state): State<AppState>,
    Path((_id, session_id, filename)): Path<(String, String, String)>,
) -> Result<impl IntoResponse, ApiError> {
    let t0 = Instant::now();

    let session = state.hls_sessions.get_session(&session_id)
        .ok_or_else(|| ApiError::not_found(format!("HLS session '{session_id}' not found")))?;

    let data = state.hls_sessions.get_segment(&session, &filename)
        .await
        .map_err(|e| {
            warn!("Failed to read HLS segment {} for session {}: {}", filename, session_id, e);
            ApiError::internal(e.to_string())
        })?
        .ok_or_else(|| ApiError::not_found(format!("HLS segment '{filename}' not found")))?;

    let total_ms = t0.elapsed().as_secs_f64() * 1000.0;

    // Determine content type from extension
    let content_type = if filename.ends_with(".mp4") {
        "video/mp4"
    } else if filename.ends_with(".m4s") {
        "video/iso.segment"
    } else {
        "application/octet-stream"
    };

    let timing = format!("total;dur={:.1}", total_ms);
    let mut resp_headers = HeaderMap::new();
    resp_headers.insert(header::CONTENT_TYPE, content_type.parse().unwrap());
    resp_headers.insert(header::CACHE_CONTROL, "max-age=3600".parse().unwrap());
    resp_headers.insert("Server-Timing", timing.parse().unwrap());

    Ok((StatusCode::OK, resp_headers, data))
}

/// POST /api/stream/{id}/hls/seek?start=123.456
/// Destroys any existing HLS session for this media and creates a new one
/// starting from the specified time. Returns the new session info as JSON.
pub async fn hls_seek(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(query): Query<HlsQuery>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, ApiError> {
    let t0 = Instant::now();
    let start_secs = query.start.unwrap_or(0.0);

    let item = media_repo::get_media_item(&state.db, &id)
        .await?
        .ok_or_else(|| ApiError::not_found(format!("Media item '{id}' not found")))?;
    let db_ms = t0.elapsed().as_secs_f64() * 1000.0;

    let file_path = std::path::Path::new(&item.file_path);
    let duration_secs = item.duration_ms.map(|ms| ms as f64 / 1000.0);

    let token = query.token.clone().or_else(|| {
        headers
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "))
            .map(|t| t.to_string())
    });

    let sub_path = resolve_subtitle_path(&state.db, query.subtitle_id).await;

    // Fetch video pixel format for HDR tone-mapping detection
    let pixel_format = stream_repo::get_video_pixel_format(&state.db, &id)
        .await
        .unwrap_or(None);

    // Fetch video frame rate for accurate GOP calculation
    let frame_rate = stream_repo::get_video_frame_rate(&state.db, &id)
        .await
        .unwrap_or(None);

    // Fetch color metadata for HDR vs 10-bit SDR distinction
    let color_meta = stream_repo::get_video_color_metadata(&state.db, &id)
        .await
        .unwrap_or(None);
    let color_transfer = color_meta.as_ref().and_then(|m| m.color_transfer.clone());
    let color_primaries = color_meta.as_ref().and_then(|m| m.color_primaries.clone());

    // Acquire transcode permit before spawning FFmpeg (enforces max_concurrent_transcodes).
    let _permit = match state.transcode_semaphore.try_acquire() {
        Ok(permit) => permit,
        Err(_) => {
            warn!("Transcode limit reached, rejecting HLS seek for {}", id);
            return Err(ApiError::service_unavailable(
                "Transcode limit reached, try again later",
            ));
        }
    };

    // Create a single variant session for fast seeking (1 FFmpeg process instead of N).
    let t1 = Instant::now();
    let sessions = state
        .hls_sessions
        .create_single_variant_session(
            &id,
            file_path,
            duration_secs,
            item.width.map(|w| w as u32),
            item.height.map(|h| h as u32),
            item.bitrate_kbps.map(|b| b as u32),
            start_secs,
            sub_path.as_deref(),
            pixel_format.as_deref(),
            query.audio_stream,
            frame_rate.as_deref(),
            item.audio_codec.as_deref(),
            item.video_codec.as_deref(),
            color_transfer.as_deref(),
            color_primaries.as_deref(),
        )
        .await
        .map_err(|e| {
            warn!("Failed to create HLS seek session for {}: {}", id, e);
            ApiError::internal(e.to_string())
        })?;
    let session_ms = t1.elapsed().as_secs_f64() * 1000.0;
    let total_ms = t0.elapsed().as_secs_f64() * 1000.0;

    let token_suffix = token
        .as_deref()
        .map(|t| {
            format!(
                "?token={}",
                percent_encoding::utf8_percent_encode(t, percent_encoding::NON_ALPHANUMERIC)
            )
        })
        .unwrap_or_default();

    let first = sessions.first().ok_or_else(|| ApiError::internal("No variant sessions created"))?;

    info!(
        "HLS seek for {}: start={:.1}s db={:.0}ms session={:.0}ms total={:.0}ms",
        id, start_secs, db_ms, session_ms, total_ms
    );

    // Return the new session info so the frontend can switch to it
    Ok(Json(serde_json::json!({
        "session_id": first.session_id,
        "start_secs": first.start_secs,
        "variant_count": sessions.len(),
        "master_url": format!("/api/stream/{}/hls/master.m3u8?start={:.3}{}", id, start_secs,
            if token_suffix.is_empty() { String::new() } else { format!("&{}", &token_suffix[1..]) }),
        "timing_ms": { "db": db_ms, "session": session_ms, "total": total_ms },
    })))
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
