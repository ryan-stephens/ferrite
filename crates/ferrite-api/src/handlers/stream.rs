use crate::error::ApiError;
use crate::state::AppState;
use axum::body::Body;
use axum::extract::{Path, Query, State};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use ferrite_db::{media_repo, stream_repo, subtitle_repo};
use ferrite_stream::compat::{self, StreamStrategy};
use ferrite_stream::{direct, transcode};
use ferrite_stream::transcode::find_keyframe_before;
use serde::Deserialize;
use sqlx::SqlitePool;
use std::time::Instant;
use tokio_util::io::ReaderStream;
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

    let requested_start = query.start.unwrap_or(0.0);

    // Snap to the nearest keyframe for fast demuxer-level seeking (-ss before -i).
    // FFmpeg will also get a precise -ss after -i with the delta so the output
    // starts at the exact requested_start, not the keyframe.
    let start_secs = if requested_start > 0.5 {
        let ffprobe_path = &state.config.transcode.ffprobe_path;
        find_keyframe_before(ffprobe_path, file_path, requested_start)
            .await
            .unwrap_or(requested_start)
    } else {
        requested_start
    };

    let sub_path = resolve_subtitle_path(&state.db, query.subtitle_id).await;

    // Fetch all video stream metadata in a single DB round-trip
    let video_meta = stream_repo::get_video_meta(&state.db, &id)
        .await
        .unwrap_or(None);
    let pixel_format = video_meta.as_ref().and_then(|m| m.pixel_format.clone());
    let frame_rate = video_meta.as_ref().and_then(|m| m.frame_rate.clone());
    let color_transfer = video_meta.as_ref().and_then(|m| m.color_transfer.clone());
    let color_primaries = video_meta.as_ref().and_then(|m| m.color_primaries.clone());

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

    // Build response with custom headers so the frontend knows the actual
    // media time and session IDs immediately (before MANIFEST_PARSED fires).
    let session_ids: Vec<String> = sessions.iter().map(|s| s.session_id.clone()).collect();
    let mut resp_headers = HeaderMap::new();
    resp_headers.insert(header::CONTENT_TYPE, "application/vnd.apple.mpegurl".parse().unwrap());
    resp_headers.insert(header::CACHE_CONTROL, "no-store".parse().unwrap());
    let video_copied = sessions.first().map(|s| s.video_copied).unwrap_or(false);
    resp_headers.insert("x-hls-start-secs", format!("{:.3}", start).parse().unwrap());
    resp_headers.insert("x-hls-video-copied", if video_copied { "1" } else { "0" }.parse().unwrap());
    resp_headers.insert("x-hls-session-ids", session_ids.join(",").parse().unwrap());
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

    let mut resp_headers = HeaderMap::new();
    resp_headers.insert(header::CONTENT_TYPE, "application/vnd.apple.mpegurl".parse().unwrap());
    resp_headers.insert(header::CACHE_CONTROL, "no-store".parse().unwrap());

    Ok((StatusCode::OK, resp_headers, playlist))
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

    let session = state.hls_sessions.get_session(&session_id)
        .ok_or_else(|| ApiError::not_found(format!("HLS session '{session_id}' not found")))?;

    // Wait for the segment to be ready (polls playlist until FFmpeg finalizes it)
    let path = state.hls_sessions.wait_for_segment(&session, &filename)
        .await
        .map_err(|e| {
            warn!("Failed to wait for HLS segment {} for session {}: {}", filename, session_id, e);
            ApiError::internal(e.to_string())
        })?
        .ok_or_else(|| ApiError::not_found(format!("HLS segment '{filename}' not found")))?;

    let total_ms = t0.elapsed().as_secs_f64() * 1000.0;

    // Open the file and stream it directly to the response — no full-file buffering.
    let file = tokio::fs::File::open(&path).await.map_err(|e| {
        warn!("Failed to open HLS segment file {:?}: {}", path, e);
        ApiError::internal(e.to_string())
    })?;
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

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

    let token = query.token.clone().or_else(|| {
        headers
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "))
            .map(|t| t.to_string())
    });

    let token_suffix = token
        .as_deref()
        .map(|t| {
            format!(
                "?token={}",
                percent_encoding::utf8_percent_encode(t, percent_encoding::NON_ALPHANUMERIC)
            )
        })
        .unwrap_or_default();

    // Fast path: if the current session already covers the requested time, reuse it.
    // This avoids destroying and recreating an FFmpeg process for seeks that land
    // within the already-transcoded buffer (e.g. sequential +10s presses).
    if let Some(existing) = state.hls_sessions.get_session_for_media(&id) {
        let buffered_end = existing.start_secs + existing.buffered_secs();
        // Reuse if the target is within the buffered window and FFmpeg is still alive.
        if requested_start >= existing.start_secs
            && requested_start < buffered_end
            && existing.is_ffmpeg_alive().await
        {
            existing.touch().await;
            let total_ms = t0.elapsed().as_secs_f64() * 1000.0;
            info!(
                "HLS seek for {} reused session {} (target={:.1}s buffered=[{:.1}s,{:.1}s)) total={:.0}ms",
                id, existing.session_id, requested_start, existing.start_secs, buffered_end, total_ms
            );
            return Ok(Json(serde_json::json!({
                "session_id": existing.session_id,
                "start_secs": existing.start_secs,
                "video_copied": existing.video_copied,
                "reused": true,
                "variant_count": 1,
                "master_url": format!("/api/stream/{}/hls/master.m3u8?start={:.3}{}", id, requested_start,
                    if token_suffix.is_empty() { String::new() } else { format!("&{}", &token_suffix[1..]) }),
                "timing_ms": { "db": 0, "session": 0, "total": total_ms },
            })));
        }
    }

    let item = media_repo::get_media_item(&state.db, &id)
        .await?
        .ok_or_else(|| ApiError::not_found(format!("Media item '{id}' not found")))?;
    let db_ms = t0.elapsed().as_secs_f64() * 1000.0;

    let file_path = std::path::Path::new(&item.file_path);
    let duration_secs = item.duration_ms.map(|ms| ms as f64 / 1000.0);

    // Snap to the nearest keyframe for fast demuxer-level seeking (-ss before -i).
    // FFmpeg will also get a precise -ss after -i with the delta so the output
    // starts at the exact requested_start, not the keyframe.
    let start_secs = if requested_start > 0.5 {
        let ffprobe_path = &state.config.transcode.ffprobe_path;
        find_keyframe_before(ffprobe_path, file_path, requested_start)
            .await
            .unwrap_or(requested_start)
    } else {
        requested_start
    };

    let sub_path = resolve_subtitle_path(&state.db, query.subtitle_id).await;

    // Fetch all video stream metadata in a single DB round-trip
    let video_meta = stream_repo::get_video_meta(&state.db, &id)
        .await
        .unwrap_or(None);
    let pixel_format = video_meta.as_ref().and_then(|m| m.pixel_format.clone());
    let frame_rate = video_meta.as_ref().and_then(|m| m.frame_rate.clone());
    let color_transfer = video_meta.as_ref().and_then(|m| m.color_transfer.clone());
    let color_primaries = video_meta.as_ref().and_then(|m| m.color_primaries.clone());

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
        .await
        .map_err(|e| {
            warn!("Failed to create HLS seek session for {}: {}", id, e);
            ApiError::internal(e.to_string())
        })?;
    let session_ms = t1.elapsed().as_secs_f64() * 1000.0;
    let total_ms = t0.elapsed().as_secs_f64() * 1000.0;

    let first = sessions.first().ok_or_else(|| ApiError::internal("No variant sessions created"))?;

    info!(
        "HLS seek for {}: start={:.1}s db={:.0}ms session={:.0}ms total={:.0}ms",
        id, start_secs, db_ms, session_ms, total_ms
    );

    // Return the new session info so the frontend can switch to it
    Ok(Json(serde_json::json!({
        "session_id": first.session_id,
        "start_secs": first.start_secs,
        "video_copied": first.video_copied,
        "reused": false,
        "variant_count": sessions.len(),
        "master_url": format!("/api/stream/{}/hls/master.m3u8?start={:.3}{}", id, requested_start,
            if token_suffix.is_empty() { String::new() } else { format!("&{}", &token_suffix[1..]) }),
        "timing_ms": { "db": db_ms, "session": session_ms, "total": total_ms },
    })))
}

/// DELETE /api/stream/{id}/hls
/// Destroys ALL HLS sessions for a media item by media ID.
/// Preferred over the session-ID variant because the frontend always knows
/// the media ID, even before MANIFEST_PARSED has fired.
pub async fn hls_stop_media(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    state.hls_sessions.destroy_media_sessions(&id).await;
    StatusCode::NO_CONTENT
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
