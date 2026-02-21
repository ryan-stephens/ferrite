use crate::auth::AuthUser;
use crate::error::ApiError;
use crate::state::AppState;
use axum::extract::State;
use axum::response::{Html, IntoResponse, Json};
use axum::Extension;
use ferrite_db::user_repo;
use serde::Deserialize;
use serde_json::json;

pub async fn health() -> impl IntoResponse {
    Json(json!({ "status": "ok" }))
}

pub async fn info() -> impl IntoResponse {
    Json(json!({
        "name": "Ferrite",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

/// GET /api/system/encoder — returns the active video encoder profile and backend.
pub async fn encoder_info(State(state): State<AppState>) -> impl IntoResponse {
    let profile = &state.encoder_profile;
    Json(json!({
        "backend": format!("{}", profile.backend),
        "encoder_name": profile.encoder_name,
        "is_hardware": profile.is_hardware(),
    }))
}

/// GET /api/admin/streams — list all active HLS transcode sessions (admin only).
pub async fn list_active_streams(
    State(state): State<AppState>,
    auth_user: Option<Extension<AuthUser>>,
) -> Result<impl IntoResponse, ApiError> {
    ensure_admin_if_present(&state, auth_user.as_ref()).await?;

    let sessions = state.hls_sessions.list_active_sessions().await;
    let count = sessions.len();
    let items: Vec<serde_json::Value> = sessions
        .into_iter()
        .map(|s| {
            json!({
                "session_id": s.session_id,
                "media_id": s.media_id,
                "variant_label": s.variant_label,
                "start_secs": s.start_secs,
                "width": s.width,
                "height": s.height,
                "bitrate_kbps": s.bitrate_kbps,
                "idle_secs": s.idle_secs,
                "age_secs": s.age_secs,
            })
        })
        .collect();
    Ok(Json(json!({ "sessions": items, "count": count })))
}

#[derive(Deserialize)]
pub struct TrackPlaybackMetricRequest {
    pub metric: String,
    pub value_ms: Option<f64>,
    pub increment: Option<u64>,
    #[serde(default)]
    pub labels: std::collections::HashMap<String, String>,
}

/// GET /api/system/metrics — snapshot in-memory playback metrics (admin only).
pub async fn playback_metrics(
    State(state): State<AppState>,
    auth_user: Option<Extension<AuthUser>>,
) -> Result<impl IntoResponse, ApiError> {
    ensure_admin_if_present(&state, auth_user.as_ref()).await?;
    Ok(Json(state.playback_metrics.snapshot()))
}

/// DELETE /api/system/metrics — reset in-memory playback metrics (admin only).
pub async fn reset_playback_metrics(
    State(state): State<AppState>,
    auth_user: Option<Extension<AuthUser>>,
) -> Result<impl IntoResponse, ApiError> {
    ensure_admin_if_present(&state, auth_user.as_ref()).await?;
    state.playback_metrics.reset();
    Ok(Json(json!({ "status": "ok" })))
}

/// POST /api/system/metrics/track — ingest client-side playback metrics.
pub async fn track_playback_metric(
    State(state): State<AppState>,
    Json(req): Json<TrackPlaybackMetricRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let mut labels: Vec<(String, String)> = req
        .labels
        .into_iter()
        .filter(|(k, _)| matches!(k.as_str(), "stream" | "path" | "mode" | "operation"))
        .collect();
    labels.sort_by(|a, b| a.0.cmp(&b.0));
    let label_refs: Vec<(&str, &str)> = labels
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();

    match req.metric.as_str() {
        "playback_ttff_ms" | "seek_latency_ms" | "rebuffer_ms" => {
            let value = req
                .value_ms
                .ok_or_else(|| ApiError::bad_request("value_ms is required for timing metrics"))?;
            state
                .playback_metrics
                .record_timing(&req.metric, &label_refs, value);
        }
        "rebuffer_count" => {
            state.playback_metrics.increment_counter(
                &req.metric,
                &label_refs,
                req.increment.unwrap_or(1),
            );
        }
        _ => {
            return Err(ApiError::bad_request("Unsupported metric name"));
        }
    }

    Ok(Json(json!({ "status": "ok" })))
}

async fn ensure_admin_if_present(
    state: &AppState,
    auth_user: Option<&Extension<AuthUser>>,
) -> Result<(), ApiError> {
    if let Some(Extension(user)) = auth_user {
        let caller = user_repo::get_user_by_id(&state.db, &user.user_id)
            .await?
            .ok_or_else(|| ApiError::unauthorized("User not found"))?;
        if caller.is_admin == 0 {
            return Err(ApiError::forbidden("Admin access required"));
        }
    }
    Ok(())
}

/// Serve the embedded web UI. For M1 this is a simple inline HTML page.
/// Later this will serve the SolidJS build via rust-embed.
pub async fn serve_frontend() -> impl IntoResponse {
    Html(FRONTEND_HTML)
}

const FRONTEND_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Ferrite</title>
    <script src="https://cdn.jsdelivr.net/npm/hls.js@1/dist/hls.min.js"></script>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: #0a0a0f;
            color: #e0e0e0;
            min-height: 100vh;
        }
        header {
            background: #14141f;
            border-bottom: 1px solid #2a2a3a;
            padding: 1rem 2rem;
            display: flex;
            align-items: center;
            justify-content: space-between;
        }
        header h1 { color: #ff6b35; font-size: 1.5rem; }
        .container { max-width: 1400px; margin: 0 auto; padding: 2rem; padding-bottom: 4rem; }
        .toolbar {
            display: flex;
            gap: 1rem;
            margin-bottom: 2rem;
            align-items: center;
            flex-wrap: wrap;
        }
        .toolbar .search-box {
            flex: 1;
            min-width: 200px;
            max-width: 400px;
            position: relative;
        }
        .toolbar .search-box input {
            width: 100%;
            padding-left: 2.2rem;
        }
        .toolbar .search-box::before {
            content: '\1F50D';
            position: absolute;
            left: 0.7rem;
            top: 50%;
            transform: translateY(-50%);
            font-size: 0.85rem;
            opacity: 0.5;
            pointer-events: none;
        }
        button {
            background: #ff6b35;
            color: white;
            border: none;
            padding: 0.6rem 1.2rem;
            border-radius: 6px;
            cursor: pointer;
            font-size: 0.9rem;
            font-weight: 500;
        }
        button:hover { background: #e55a25; }
        button.secondary { background: #2a2a3a; }
        button.secondary:hover { background: #3a3a4a; }
        input, select {
            background: #1a1a2a;
            color: #e0e0e0;
            border: 1px solid #2a2a3a;
            padding: 0.6rem 1rem;
            border-radius: 6px;
            font-size: 0.9rem;
        }
        /* Continue Watching row */
        .continue-section { margin-bottom: 2rem; }
        .continue-section h3 { font-size: 1rem; color: #aaa; margin-bottom: 0.8rem; }
        .continue-row {
            display: flex;
            gap: 1rem;
            overflow-x: auto;
            padding-bottom: 0.5rem;
            scrollbar-width: thin;
            scrollbar-color: #2a2a3a transparent;
        }
        .continue-row::-webkit-scrollbar { height: 6px; }
        .continue-row::-webkit-scrollbar-track { background: transparent; }
        .continue-row::-webkit-scrollbar-thumb { background: #2a2a3a; border-radius: 3px; }
        .continue-card {
            flex-shrink: 0;
            width: 140px;
            background: #14141f;
            border-radius: 8px;
            overflow: hidden;
            cursor: pointer;
            border: 1px solid #2a2a3a;
            transition: transform 0.2s;
        }
        .continue-card:hover { transform: translateY(-2px); }
        .continue-card .card-thumb {
            width: 100%;
            aspect-ratio: 2/3;
            background: #1a1a2a;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 2rem;
            color: #3a3a4a;
            position: relative;
        }
        .continue-card .card-thumb img { width: 100%; height: 100%; object-fit: cover; }
        .continue-card .card-info { padding: 0.5rem; }
        .continue-card .card-title {
            font-weight: 600;
            font-size: 0.75rem;
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
        }
        .continue-card .card-meta { font-size: 0.65rem; color: #888; margin-top: 0.15rem; }
        .grid {
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
            gap: 1.5rem;
        }
        .card {
            background: #14141f;
            border-radius: 10px;
            overflow: hidden;
            cursor: pointer;
            transition: transform 0.2s, box-shadow 0.2s;
            border: 1px solid #2a2a3a;
        }
        .card:hover { transform: translateY(-4px); box-shadow: 0 8px 24px rgba(0,0,0,0.4); }
        .card-thumb {
            width: 100%;
            aspect-ratio: 2/3;
            background: #1a1a2a;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 3rem;
            color: #3a3a4a;
            position: relative;
        }
        .card-thumb img { width: 100%; height: 100%; object-fit: cover; }
        .card-progress {
            position: absolute;
            bottom: 0;
            left: 0;
            right: 0;
            height: 3px;
            background: rgba(0,0,0,0.5);
        }
        .card-progress-fill {
            height: 100%;
            background: #ff6b35;
            border-radius: 0 1px 0 0;
        }
        .card-completed {
            position: absolute;
            top: 6px;
            right: 6px;
            background: rgba(0,0,0,0.7);
            color: #4ade80;
            width: 22px;
            height: 22px;
            border-radius: 50%;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 0.7rem;
        }
        .card-duration-hover {
            position: absolute;
            bottom: 8px;
            right: 6px;
            background: rgba(0,0,0,0.8);
            color: #fff;
            padding: 0.15rem 0.4rem;
            border-radius: 3px;
            font-size: 0.65rem;
            opacity: 0;
            transition: opacity 0.2s;
            pointer-events: none;
        }
        .card:hover .card-duration-hover { opacity: 1; }
        .card-info { padding: 0.8rem; }
        .card-title {
            font-weight: 600;
            font-size: 0.9rem;
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
        }
        .card-meta { font-size: 0.75rem; color: #888; margin-top: 0.25rem; }
        .card-badges { display: flex; gap: 0.3rem; margin-top: 0.4rem; flex-wrap: wrap; }
        .badge {
            display: inline-block;
            padding: 0.15rem 0.4rem;
            border-radius: 4px;
            font-size: 0.65rem;
            font-weight: 600;
            text-transform: uppercase;
        }
        .badge-direct { background: #1a3a1a; color: #4ade80; }
        .badge-transcode { background: #3a2a1a; color: #fbbf24; }
        .badge-codec { background: #1a1a2a; color: #93c5fd; }
        .badge-res { background: #2a1a2a; color: #c4b5fd; }
        .player-overlay {
            display: none;
            position: fixed;
            top: 0; left: 0; right: 0; bottom: 0;
            background: rgba(0,0,0,0.95);
            z-index: 100;
            flex-direction: column;
            align-items: center;
            justify-content: center;
        }
        .player-overlay.active { display: flex; }
        .player-overlay video { max-width: 90vw; max-height: 75vh; border-radius: 8px 8px 0 0; }
        .player-overlay .close-btn {
            position: absolute;
            top: 1rem;
            right: 1.5rem;
            font-size: 2rem;
            cursor: pointer;
            color: white;
            background: none;
            border: none;
            z-index: 110;
        }
        .player-title {
            position: absolute;
            top: 1rem;
            left: 1.5rem;
            font-size: 1rem;
            font-weight: 600;
            color: white;
            z-index: 110;
            text-shadow: 0 1px 4px rgba(0,0,0,0.8);
        }
        .custom-controls {
            width: 90vw;
            max-width: 90vw;
            background: #14141f;
            padding: 0.6rem 1rem;
            border-radius: 0 0 8px 8px;
            display: flex;
            align-items: center;
            gap: 0.8rem;
        }
        .custom-controls .play-btn {
            background: none;
            border: none;
            color: white;
            font-size: 1.2rem;
            cursor: pointer;
            padding: 0.2rem;
            width: 2rem;
        }
        .fullscreen-btn {
            background: none;
            border: none;
            color: #aaa;
            font-size: 1rem;
            cursor: pointer;
            padding: 0.2rem;
        }
        .fullscreen-btn:hover { color: white; }
        .timeline-container {
            flex: 1;
            height: 6px;
            background: #2a2a3a;
            border-radius: 3px;
            cursor: pointer;
            position: relative;
        }
        .timeline-container:hover { height: 10px; margin: -2px 0; }
        .timeline-progress {
            height: 100%;
            background: #ff6b35;
            border-radius: 3px;
            pointer-events: none;
            transition: width 0.15s linear;
        }
        .timeline-buffered {
            position: absolute;
            top: 0;
            left: 0;
            height: 100%;
            background: rgba(255,255,255,0.15);
            border-radius: 3px;
            pointer-events: none;
        }
        .time-display {
            font-size: 0.8rem;
            color: #aaa;
            white-space: nowrap;
            min-width: 5rem;
            text-align: center;
        }
        .volume-control {
            display: flex;
            align-items: center;
            gap: 0.3rem;
        }
        .volume-control input[type=range] {
            -webkit-appearance: none;
            appearance: none;
            width: 80px;
            height: 4px;
            background: #2a2a3a;
            border-radius: 2px;
            outline: none;
            cursor: pointer;
        }
        .volume-control input[type=range]::-webkit-slider-thumb {
            -webkit-appearance: none;
            appearance: none;
            width: 14px;
            height: 14px;
            background: #ff6b35;
            border-radius: 50%;
            cursor: pointer;
        }
        .volume-control input[type=range]::-moz-range-thumb {
            width: 14px;
            height: 14px;
            background: #ff6b35;
            border-radius: 50%;
            border: none;
            cursor: pointer;
        }
        .volume-control input[type=range]::-moz-range-track {
            background: #2a2a3a;
            height: 4px;
            border-radius: 2px;
        }
        .detail-overlay {
            display: none;
            position: fixed;
            top: 0; left: 0; right: 0; bottom: 0;
            background: rgba(0,0,0,0.95);
            z-index: 100;
            overflow-y: auto;
        }
        .detail-overlay.active { display: block; }
        .detail-content {
            max-width: 900px;
            margin: 3rem auto;
            display: flex;
            gap: 2rem;
            padding: 2rem;
        }
        .detail-poster { flex-shrink: 0; }
        .detail-poster img {
            width: 300px;
            border-radius: 10px;
            box-shadow: 0 8px 32px rgba(0,0,0,0.5);
        }
        .detail-poster .no-poster {
            width: 300px;
            height: 450px;
            background: #1a1a2a;
            border-radius: 10px;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 4rem;
            color: #3a3a4a;
        }
        .detail-info { flex: 1; }
        .detail-info h2 { font-size: 1.8rem; margin-bottom: 0.3rem; }
        .detail-meta { color: #888; margin-bottom: 0.5rem; font-size: 0.9rem; }
        .detail-rating { color: #fbbf24; font-size: 1rem; margin-bottom: 0.8rem; }
        .detail-genres { display: flex; gap: 0.4rem; flex-wrap: wrap; margin-bottom: 1rem; }
        .detail-genres span {
            background: #2a2a3a;
            padding: 0.2rem 0.6rem;
            border-radius: 12px;
            font-size: 0.75rem;
            color: #aaa;
        }
        .detail-overview {
            color: #bbb;
            line-height: 1.7;
            margin-bottom: 1.5rem;
            font-size: 0.95rem;
        }
        .detail-badges { margin-bottom: 1.5rem; }
        .detail-actions { display: flex; gap: 0.8rem; align-items: center; flex-wrap: wrap; }
        .detail-actions .resume-info { font-size: 0.85rem; color: #aaa; }
        .empty {
            text-align: center;
            padding: 4rem;
            color: #666;
        }
        .empty h2 { margin-bottom: 1rem; }
        .loading-spinner {
            text-align: center;
            padding: 3rem;
            color: #666;
        }
        .loading-spinner::after {
            content: '';
            display: inline-block;
            width: 24px;
            height: 24px;
            border: 3px solid #2a2a3a;
            border-top-color: #ff6b35;
            border-radius: 50%;
            animation: spin 0.8s linear infinite;
        }
        @keyframes spin { to { transform: rotate(360deg); } }
        .libraries-list {
            margin-bottom: 2rem;
            display: flex;
            gap: 0.5rem;
            flex-wrap: wrap;
        }
        .lib-chip {
            background: #2a2a3a;
            padding: 0.4rem 0.8rem;
            border-radius: 20px;
            font-size: 0.8rem;
            display: flex;
            align-items: center;
            gap: 0.5rem;
            cursor: pointer;
        }
        .lib-chip.active { background: #ff6b35; }
        .lib-chip .delete-lib {
            cursor: pointer;
            opacity: 0.6;
            font-size: 0.9rem;
        }
        .lib-chip .delete-lib:hover { opacity: 1; }
        .dialog-overlay {
            display: none;
            position: fixed;
            top: 0; left: 0; right: 0; bottom: 0;
            background: rgba(0,0,0,0.7);
            z-index: 200;
            align-items: center;
            justify-content: center;
        }
        .dialog-overlay.active { display: flex; }
        .dialog {
            background: #14141f;
            border: 1px solid #2a2a3a;
            border-radius: 12px;
            padding: 2rem;
            min-width: 400px;
        }
        .dialog h3 { margin-bottom: 1rem; }
        .dialog label { display: block; margin-bottom: 0.5rem; font-size: 0.85rem; color: #aaa; }
        .dialog input, .dialog select { width: 100%; margin-bottom: 1rem; }
        .dialog-actions { display: flex; gap: 0.5rem; justify-content: flex-end; }
        .status-bar {
            position: fixed;
            bottom: 0;
            left: 0; right: 0;
            background: #14141f;
            border-top: 1px solid #2a2a3a;
            padding: 0.5rem 2rem;
            font-size: 0.8rem;
            color: #888;
        }
        .status-bar.scanning { color: #ff6b35; }
        @keyframes pulse { 0%,100%{opacity:1} 50%{opacity:0.5} }
        .login-overlay {
            display: none;
            position: fixed;
            top: 0; left: 0; right: 0; bottom: 0;
            background: #0a0a0f;
            z-index: 300;
            align-items: center;
            justify-content: center;
        }
        .login-overlay.active { display: flex; }
        .login-card {
            background: #14141f;
            border: 1px solid #2a2a3a;
            border-radius: 12px;
            padding: 2.5rem;
            width: 360px;
            text-align: center;
        }
        .login-card label { display: block; text-align: left; margin-bottom: 0.3rem; font-size: 0.85rem; color: #aaa; }
        .login-card input { width: 100%; margin-bottom: 1rem; }
        .status-bar.scanning::before {
            content: '';
            display: inline-block;
            width: 8px;
            height: 8px;
            background: #ff6b35;
            border-radius: 50%;
            margin-right: 0.5rem;
            animation: pulse 1.2s ease-in-out infinite;
        }
    </style>
</head>
<body>
    <header>
        <h1>Ferrite</h1>
        <div style="display:flex;align-items:center;gap:1rem;">
            <span id="item-count" style="color: #888; font-size: 0.85rem;"></span>
            <button id="logout-btn" class="secondary" onclick="logout()" style="display:none;font-size:0.8rem;padding:0.4rem 0.8rem;">Logout</button>
        </div>
    </header>

    <div class="container">
        <div class="toolbar">
            <button onclick="showAddLibrary()">+ Add Library</button>
            <button class="secondary" onclick="refreshAll()">Refresh</button>
            <div class="search-box">
                <input type="text" id="search-input" placeholder="Search... (/ to focus)" oninput="filterAndRender()">
            </div>
            <select id="sort-select" onchange="filterAndRender()" style="min-width:140px;">
                <option value="title-asc">Title A-Z</option>
                <option value="title-desc">Title Z-A</option>
                <option value="year-desc">Year (Newest)</option>
                <option value="year-asc">Year (Oldest)</option>
                <option value="rating-desc">Rating (Best)</option>
                <option value="added-desc">Recently Added</option>
                <option value="played-desc">Recently Played</option>
            </select>
        </div>

        <div class="libraries-list" id="libraries"></div>
        <div id="continue-watching"></div>
        <div class="grid" id="media-grid"></div>
        <div class="loading-spinner" id="loading" style="display:none;"></div>
        <div class="empty" id="empty-state" style="display:none;">
            <h2 id="empty-title">No media yet</h2>
            <p id="empty-message">Add a library to get started. Point it at a folder containing your media files.</p>
        </div>
    </div>

    <div class="player-overlay" id="player">
        <button class="close-btn" onclick="closePlayer()">&times;</button>
        <div class="player-title" id="player-title"></div>
        <video id="video" autoplay></video>
        <div class="custom-controls" id="custom-controls">
            <button class="play-btn" id="play-btn" onclick="togglePlay()">&#9654;</button>
            <span class="time-display" id="time-current">0:00</span>
            <div class="timeline-container" id="timeline" onclick="seekTimeline(event)">
                <div class="timeline-buffered" id="timeline-buffered"></div>
                <div class="timeline-progress" id="timeline-progress"></div>
            </div>
            <span class="time-display" id="time-total">0:00</span>
            <div class="volume-control">
                <span id="volume-icon" style="color:#aaa;font-size:0.9rem;cursor:pointer;" onclick="toggleMute()">&#128266;</span>
                <input type="range" id="volume" min="0" max="100" step="1" value="100" oninput="setVolume(this.value)">
            </div>
            <button class="fullscreen-btn" onclick="toggleFullscreen()" title="Fullscreen (F)">&#x26F6;</button>
        </div>
    </div>

    <div class="detail-overlay" id="detail-view">
        <button class="close-btn" onclick="closeDetail()" style="position:absolute;top:1rem;right:1.5rem;font-size:2rem;cursor:pointer;color:white;background:none;border:none;z-index:110;">&times;</button>
        <div class="detail-content">
            <div class="detail-poster" id="detail-poster"></div>
            <div class="detail-info">
                <h2 id="detail-title"></h2>
                <div class="detail-meta" id="detail-meta"></div>
                <div class="detail-rating" id="detail-rating"></div>
                <div class="detail-genres" id="detail-genres"></div>
                <p class="detail-overview" id="detail-overview"></p>
                <div class="detail-badges" id="detail-badges"></div>
                <div class="detail-actions" id="detail-actions">
                    <button onclick="playFromDetail()">&#9654; Play</button>
                </div>
            </div>
        </div>
    </div>

    <div class="dialog-overlay" id="add-dialog">
        <div class="dialog">
            <h3>Add Library</h3>
            <label>Name</label>
            <input type="text" id="lib-name" placeholder="Movies">
            <label>Path</label>
            <input type="text" id="lib-path" placeholder="/media/movies">
            <label>Type</label>
            <select id="lib-type">
                <option value="movie">Movies</option>
                <option value="tv">TV Shows</option>
                <option value="music">Music</option>
            </select>
            <div class="dialog-actions">
                <button class="secondary" onclick="hideAddLibrary()">Cancel</button>
                <button onclick="addLibrary()">Add & Scan</button>
            </div>
        </div>
    </div>

    <div class="login-overlay" id="login-page">
        <div class="login-card">
            <h1 style="color: #ff6b35; margin-bottom: 0.5rem;">Ferrite</h1>
            <p style="color: #888; margin-bottom: 1.5rem;">Sign in to continue</p>
            <div id="login-error" style="color:#ef4444;font-size:0.85rem;margin-bottom:1rem;display:none;"></div>
            <label>Username</label>
            <input type="text" id="login-username" placeholder="Username"
                   onkeydown="if(event.key==='Enter')document.getElementById('login-password').focus()">
            <label>Password</label>
            <input type="password" id="login-password" placeholder="Password"
                   onkeydown="if(event.key==='Enter')doLogin()">
            <button onclick="doLogin()" style="width:100%;margin-top:0.5rem;">Sign In</button>
        </div>
    </div>

    <div class="status-bar" id="status">Ready</div>

    <script>
        let currentLibrary = null;
        let allMediaItems = [];  // Full items from API (for client-side filter/sort)

        // Restore persisted UI state
        const savedSort = localStorage.getItem('ferrite-sort');
        if (savedSort) document.getElementById('sort-select').value = savedSort;
        const savedSearch = localStorage.getItem('ferrite-search');
        if (savedSearch) document.getElementById('search-input').value = savedSearch;

        function getToken() { return localStorage.getItem('ferrite-token'); }
        function setToken(t) { localStorage.setItem('ferrite-token', t); }
        function clearToken() { localStorage.removeItem('ferrite-token'); }

        function authHeaders() {
            const h = { 'Content-Type': 'application/json' };
            const t = getToken();
            if (t) h['Authorization'] = 'Bearer ' + t;
            return h;
        }

        // Append token as query param for URLs used in src attributes (video, img)
        function authUrl(url) {
            const t = getToken();
            if (!t) return url;
            const sep = url.includes('?') ? '&' : '?';
            return url + sep + 'token=' + encodeURIComponent(t);
        }

        async function api(method, path, body) {
            const opts = { method, headers: authHeaders() };
            if (body) opts.body = JSON.stringify(body);
            const res = await fetch(path, opts);
            if (res.status === 401) { clearToken(); showLoginPage(); throw new Error('Unauthorized'); }
            if (!res.ok) throw new Error(`${res.status} ${res.statusText}`);
            return res.json();
        }

        // Fire-and-forget API call (for progress reporting — no await needed)
        function apiQuiet(method, path, body) {
            const opts = { method, headers: authHeaders() };
            if (body) opts.body = JSON.stringify(body);
            fetch(path, opts).catch(() => {});
        }

        function setStatus(msg, scanning) {
            const el = document.getElementById('status');
            el.textContent = msg;
            el.classList.toggle('scanning', !!scanning);
        }

        function formatSize(bytes) {
            if (bytes < 1024) return bytes + ' B';
            if (bytes < 1048576) return (bytes / 1024).toFixed(1) + ' KB';
            if (bytes < 1073741824) return (bytes / 1048576).toFixed(1) + ' MB';
            return (bytes / 1073741824).toFixed(2) + ' GB';
        }

        function formatDuration(ms) {
            if (!ms) return '';
            const s = Math.floor(ms / 1000);
            const h = Math.floor(s / 3600);
            const m = Math.floor((s % 3600) / 60);
            if (h > 0) return `${h}h ${m}m`;
            return `${m}m`;
        }

        const COMPAT_AUDIO = ['aac','mp3','opus','vorbis','flac'];
        const COMPAT_VIDEO = ['h264','vp8','vp9','av1'];
        const COMPAT_CONTAINER = ['mp4','mov','webm','ogg'];

        function getStreamBadge(item) {
            const cOk = item.container_format && COMPAT_CONTAINER.includes(item.container_format.toLowerCase());
            const vOk = !item.video_codec || COMPAT_VIDEO.includes(item.video_codec.toLowerCase());
            const aOk = !item.audio_codec || COMPAT_AUDIO.includes(item.audio_codec.toLowerCase());
            if (cOk && vOk && aOk) return '<span class="badge badge-direct">Direct Play</span>';
            if (vOk) return '<span class="badge badge-transcode">Audio Transcode</span>';
            return '<span class="badge badge-transcode">Full Transcode</span>';
        }

        function getResLabel(w, h) {
            if (!w || !h) return '';
            if (h >= 2160) return '4K';
            if (h >= 1080) return '1080p';
            if (h >= 720) return '720p';
            if (h >= 480) return '480p';
            return `${h}p`;
        }

        function getDisplayTitle(item) {
            return item.movie_title || item.title || item.file_path.split(/[/\\\\]/).pop();
        }

        function getDisplayYear(item) {
            return item.movie_year || item.year || null;
        }

        async function loadLibraries() {
            const libs = await api('GET', '/api/libraries');
            const el = document.getElementById('libraries');
            el.innerHTML = libs.map(lib => `
                <div class="lib-chip ${currentLibrary === lib.id ? 'active' : ''}"
                     onclick="selectLibrary('${lib.id}')">
                    ${lib.name}
                    <span class="delete-lib" onclick="event.stopPropagation(); deleteLibrary('${lib.id}')">&times;</span>
                </div>
            `).join('');
        }

        async function loadMedia() {
            document.getElementById('loading').style.display = 'block';
            document.getElementById('media-grid').innerHTML = '';
            document.getElementById('empty-state').style.display = 'none';

            const params = new URLSearchParams({ page: '1', per_page: '500' });
            if (currentLibrary) params.set('library_id', currentLibrary);
            const data = await api('GET', `/api/media?${params}`);

            document.getElementById('loading').style.display = 'none';
            allMediaItems = data.items;
            document.getElementById('item-count').textContent = data.total ? `${data.total} items` : '';

            renderContinueWatching();
            filterAndRender();
        }

        function renderContinueWatching() {
            const el = document.getElementById('continue-watching');
            const inProgress = allMediaItems.filter(item =>
                item.position_ms && item.position_ms > 0 && !item.completed && item.duration_ms
            ).sort((a, b) => {
                const aDate = a.last_played_at || '';
                const bDate = b.last_played_at || '';
                return bDate.localeCompare(aDate);
            }).slice(0, 10);

            if (inProgress.length === 0) {
                el.innerHTML = '';
                return;
            }

            el.innerHTML = `
                <div class="continue-section">
                    <h3>Continue Watching</h3>
                    <div class="continue-row">${inProgress.map(item => {
                        const title = getDisplayTitle(item);
                        const pct = Math.min(100, (item.position_ms / item.duration_ms) * 100);
                        const remaining = formatDuration(item.duration_ms - item.position_ms);
                        const thumb = item.poster_path
                            ? `<img src="${authUrl('/api/images/' + item.poster_path)}" loading="lazy">`
                            : '&#9654;';
                        return `
                        <div class="continue-card" onclick="showDetail('${item.id}')">
                            <div class="card-thumb">
                                ${thumb}
                                <div class="card-progress"><div class="card-progress-fill" style="width:${pct}%"></div></div>
                            </div>
                            <div class="card-info">
                                <div class="card-title">${title}</div>
                                <div class="card-meta">${remaining} left</div>
                            </div>
                        </div>`;
                    }).join('')}</div>
                </div>`;
        }

        function filterAndRender() {
            const searchText = document.getElementById('search-input').value.toLowerCase().trim();
            const sortValue = document.getElementById('sort-select').value;

            // Persist to localStorage
            localStorage.setItem('ferrite-search', searchText);
            localStorage.setItem('ferrite-sort', sortValue);

            // Filter
            let items = allMediaItems;
            if (searchText) {
                items = items.filter(item => {
                    const title = getDisplayTitle(item).toLowerCase();
                    const overview = (item.overview || '').toLowerCase();
                    return title.includes(searchText) || overview.includes(searchText);
                });
            }

            // Sort
            items = [...items].sort((a, b) => {
                switch (sortValue) {
                    case 'title-asc': return getDisplayTitle(a).localeCompare(getDisplayTitle(b));
                    case 'title-desc': return getDisplayTitle(b).localeCompare(getDisplayTitle(a));
                    case 'year-desc': return (getDisplayYear(b) || 0) - (getDisplayYear(a) || 0);
                    case 'year-asc': return (getDisplayYear(a) || 0) - (getDisplayYear(b) || 0);
                    case 'rating-desc': return (b.rating || 0) - (a.rating || 0);
                    case 'added-desc': return (b.added_at || '').localeCompare(a.added_at || '');
                    case 'played-desc': return (b.last_played_at || '').localeCompare(a.last_played_at || '');
                    default: return 0;
                }
            });

            renderGrid(items, searchText);
        }

        function renderGrid(items, searchText) {
            const grid = document.getElementById('media-grid');
            const empty = document.getElementById('empty-state');

            if (items.length === 0) {
                grid.innerHTML = '';
                empty.style.display = 'block';
                if (searchText) {
                    document.getElementById('empty-title').textContent = 'No results';
                    document.getElementById('empty-message').textContent = `No items match "${searchText}". Try a different search.`;
                } else if (allMediaItems.length === 0) {
                    document.getElementById('empty-title').textContent = 'No media yet';
                    document.getElementById('empty-message').textContent = 'Add a library to get started. Point it at a folder containing your media files.';
                } else {
                    document.getElementById('empty-title').textContent = 'No items in this library';
                    document.getElementById('empty-message').textContent = 'Try scanning the library or adding media files to its folder.';
                }
            } else {
                empty.style.display = 'none';
                grid.innerHTML = items.map(item => {
                    const dur = formatDuration(item.duration_ms);
                    const res = getResLabel(item.width, item.height);
                    const streamBadge = getStreamBadge(item);
                    const codecBadge = item.video_codec ? `<span class="badge badge-codec">${item.video_codec}</span>` : '';
                    const audioBadge = item.audio_codec ? `<span class="badge badge-codec">${item.audio_codec}</span>` : '';
                    const resBadge = res ? `<span class="badge badge-res">${res}</span>` : '';
                    const displayTitle = getDisplayTitle(item);
                    const displayYear = getDisplayYear(item);
                    const thumb = item.poster_path
                        ? `<img src="${authUrl('/api/images/' + item.poster_path)}" loading="lazy">`
                        : '&#9654;';

                    // Progress indicators
                    let progressHtml = '';
                    if (item.completed) {
                        progressHtml = '<div class="card-completed">&#10003;</div>';
                    } else if (item.position_ms && item.position_ms > 0 && item.duration_ms) {
                        const pct = Math.min(100, (item.position_ms / item.duration_ms) * 100);
                        progressHtml = `<div class="card-progress"><div class="card-progress-fill" style="width:${pct}%"></div></div>`;
                    }

                    // Duration hover overlay
                    const durationHover = dur ? `<div class="card-duration-hover">${dur}</div>` : '';

                    const metaParts = [formatSize(item.file_size)];
                    if (dur) metaParts.push(dur);
                    if (displayYear) metaParts.push(displayYear);
                    if (item.rating) metaParts.push('&#9733; ' + item.rating);
                    return `
                    <div class="card" onclick="showDetail('${item.id}')">
                        <div class="card-thumb">${thumb}${progressHtml}${durationHover}</div>
                        <div class="card-info">
                            <div class="card-title">${displayTitle}</div>
                            <div class="card-meta">${metaParts.join(' &middot; ')}</div>
                            <div class="card-badges">${streamBadge}${resBadge}${codecBadge}${audioBadge}</div>
                        </div>
                    </div>`;
                }).join('');
            }
        }

        function selectLibrary(id) {
            currentLibrary = currentLibrary === id ? null : id;
            loadLibraries();
            loadMedia();
        }

        // ---- Player state ----
        let currentItem = null;
        let isTranscoded = false;
        let isHls = false;          // true when playing via hls.js
        let seekOffset = 0;         // only used for non-HLS transcoded streams
        let hlsStartOffset = 0;    // media time offset where HLS stream begins (for seeking/resume)
        let knownDuration = 0;
        let isSeeking = false;
        let lastProgressReport = 0;
        let currentHls = null;      // hls.js instance
        let currentHlsSessionId = null;
        let currentPlaybackSessionId = null;

        function createPlaybackSessionId() {
            if (window.crypto && typeof window.crypto.randomUUID === 'function') {
                return window.crypto.randomUUID();
            }
            return `sys-${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 10)}`;
        }

        function isDirectPlay(item) {
            const cOk = item.container_format && COMPAT_CONTAINER.includes(item.container_format.toLowerCase());
            const vOk = !item.video_codec || COMPAT_VIDEO.includes(item.video_codec.toLowerCase());
            const aOk = !item.audio_codec || COMPAT_AUDIO.includes(item.audio_codec.toLowerCase());
            return cOk && vOk && aOk;
        }

        function isFullTranscode(item) {
            const vOk = !item.video_codec || COMPAT_VIDEO.includes(item.video_codec.toLowerCase());
            return !vOk;
        }

        async function playMedia(id, resumePosition) {
            currentItem = await api('GET', `/api/media/${id}`);
            isTranscoded = !isDirectPlay(currentItem);
            isHls = false;
            seekOffset = 0;
            hlsStartOffset = 0;
            knownDuration = currentItem.duration_ms ? currentItem.duration_ms / 1000 : 0;
            isSeeking = false;
            lastProgressReport = 0;
            currentHlsSessionId = null;
            currentPlaybackSessionId = createPlaybackSessionId();

            // Clean up any previous hls.js instance
            if (currentHls) {
                currentHls.destroy();
                currentHls = null;
            }

            const video = document.getElementById('video');
            const displayTitle = getDisplayTitle(currentItem);
            document.getElementById('player-title').textContent = displayTitle;

            // Always use custom controls for consistent UX
            video.controls = false;
            document.getElementById('custom-controls').style.display = 'flex';

            // Determine resume position.
            // resumePosition is explicitly set by the caller:
            //   - undefined/null  = no preference (auto-resume from DB if available)
            //   - 0               = play from start (do NOT auto-resume)
            //   - >0              = resume from this position
            let startAt;
            if (resumePosition != null) {
                // Caller explicitly chose a position (including 0 for "Play from Start")
                startAt = resumePosition;
            } else if (currentItem.position_ms && currentItem.position_ms > 0 && !currentItem.completed) {
                // Auto-resume from saved position
                startAt = currentItem.position_ms / 1000;
            } else {
                startAt = 0;
            }

            // Determine playback path:
            // 1. DirectPlay — serve file directly
            // 2. FullTranscode + hls.js — HLS adaptive streaming
            // 3. AudioTranscode or fallback — fMP4 pipe (existing path)

            const useHls = isFullTranscode(currentItem) && typeof Hls !== 'undefined' && Hls.isSupported();
            const useNativeHls = isFullTranscode(currentItem) && !useHls
                && video.canPlayType('application/vnd.apple.mpegurl');

            if (useHls) {
                // ---- HLS via hls.js ----
                // Pass start time so FFmpeg begins transcoding from the resume point
                isHls = true;
                hlsStartOffset = startAt;
                const startParam = startAt > 0.5 ? `&start=${startAt.toFixed(3)}` : '';
                const playbackParam = `&playback_session_id=${encodeURIComponent(currentPlaybackSessionId)}`;
                const masterUrl = `/api/stream/${id}/hls/master.m3u8?_=1${startParam}${playbackParam}`;

                const hls = new Hls({
                    xhrSetup: function(xhr, url) {
                        const t = getToken();
                        if (t) xhr.setRequestHeader('Authorization', 'Bearer ' + t);
                    }
                });

                currentHls = hls;

                hls.on(Hls.Events.MANIFEST_PARSED, function() {
                    // Extract session ID from the variant URL for cleanup later
                    if (hls.levels && hls.levels.length > 0) {
                        const lvlUrl = hls.levels[0].url;
                        const match = lvlUrl.match(/\/hls\/([^/]+)\/playlist\.m3u8/);
                        if (match) currentHlsSessionId = match[1];
                    }
                    // Stream starts at time 0 in the HLS timeline (FFmpeg used -ss)
                    // so we don't need to seek — just play from the beginning of the stream
                    video.play();
                });

                hls.on(Hls.Events.ERROR, function(event, data) {
                    if (data.fatal) {
                        console.error('HLS fatal error:', data.type, data.details);
                        // Fall back to fMP4 pipe
                        hls.destroy();
                        currentHls = null;
                        isHls = false;
                        console.log('Falling back to fMP4 pipe stream');
                        seekOffset = startAt > 0.5 ? startAt : 0;
                        if (startAt > 0.5) {
                            video.src = authUrl(`/api/stream/${id}?start=${startAt.toFixed(3)}`);
                        } else {
                            video.src = authUrl(`/api/stream/${id}`);
                        }
                        video.play();
                    }
                });

                hls.loadSource(authUrl(masterUrl));
                hls.attachMedia(video);
            } else if (useNativeHls) {
                // ---- Safari native HLS ----
                isHls = true;
                const masterUrl = authUrl(
                    `/api/stream/${id}/hls/master.m3u8?playback_session_id=${encodeURIComponent(currentPlaybackSessionId)}`,
                );
                video.src = masterUrl;
                video.addEventListener('loadedmetadata', function onMeta() {
                    if (startAt > 1) video.currentTime = startAt;
                    video.removeEventListener('loadedmetadata', onMeta);
                });
                video.play();
            } else if (isTranscoded && startAt > 1) {
                // ---- fMP4 pipe fallback (AudioTranscode or no hls.js) ----
                seekOffset = startAt;
                video.src = authUrl(`/api/stream/${id}?start=${startAt.toFixed(3)}`);
                video.play();
            } else {
                // ---- DirectPlay or fMP4 from beginning ----
                video.src = authUrl(`/api/stream/${id}`);
                video.play();
            }

            document.getElementById('player').classList.add('active');

            // For direct play resume, seek after metadata loads
            if (!isTranscoded && startAt > 1) {
                video.addEventListener('loadedmetadata', function onMeta() {
                    video.currentTime = startAt;
                    video.removeEventListener('loadedmetadata', onMeta);
                });
            }

            document.getElementById('time-total').textContent = fmtTime(knownDuration);
            document.getElementById('time-current').textContent = fmtTime(startAt);
            if (knownDuration > 0) {
                document.getElementById('timeline-progress').style.width = ((startAt / knownDuration) * 100) + '%';
            } else {
                document.getElementById('timeline-progress').style.width = '0%';
            }
        }

        function closePlayer() {
            // Report final progress before closing
            if (currentItem) {
                const video = document.getElementById('video');
                const posMs = isHls
                    ? Math.floor((hlsStartOffset + video.currentTime) * 1000)
                    : Math.floor((seekOffset + video.currentTime) * 1000);
                if (posMs > 0) {
                    apiQuiet('PUT', `/api/progress/${currentItem.id}`, { position_ms: posMs });
                }
            }

            // Destroy hls.js instance
            if (currentHls) {
                currentHls.destroy();
                currentHls = null;
            }

            // Clean up HLS session on the server.
            // Native HLS may not expose session IDs to JS, so fallback to owner-key cleanup.
            if (currentItem && isHls) {
                if (currentHlsSessionId) {
                    apiQuiet('DELETE', `/api/stream/${currentItem.id}/hls/${currentHlsSessionId}`);
                    currentHlsSessionId = null;
                } else if (currentPlaybackSessionId) {
                    apiQuiet(
                        'DELETE',
                        `/api/stream/${currentItem.id}/hls?playback_session_id=${encodeURIComponent(currentPlaybackSessionId)}`,
                    );
                }
            }
            currentPlaybackSessionId = null;

            const video = document.getElementById('video');
            video.pause();
            video.removeAttribute('src');
            video.load();
            document.getElementById('player').classList.remove('active');
            isHls = false;
            hlsStartOffset = 0;
            currentItem = null;

            // Refresh the grid to show updated progress
            loadMedia();
        }

        function togglePlay() {
            const video = document.getElementById('video');
            if (video.paused) {
                video.play();
            } else {
                video.pause();
            }
        }

        function toggleFullscreen() {
            const player = document.getElementById('player');
            if (document.fullscreenElement) {
                document.exitFullscreen();
            } else {
                player.requestFullscreen().catch(() => {});
            }
        }

        let lastVolume = 100;
        function setVolume(val) {
            const v = parseInt(val);
            document.getElementById('video').volume = v / 100;
            lastVolume = v > 0 ? v : lastVolume;
            const icon = document.getElementById('volume-icon');
            if (v === 0) icon.innerHTML = '&#128263;';
            else if (v < 50) icon.innerHTML = '&#128265;';
            else icon.innerHTML = '&#128266;';
        }
        function toggleMute() {
            const video = document.getElementById('video');
            const slider = document.getElementById('volume');
            if (video.volume > 0) {
                lastVolume = parseInt(slider.value);
                slider.value = 0;
                setVolume(0);
            } else {
                slider.value = lastVolume || 50;
                setVolume(slider.value);
            }
        }

        async function hlsSeekTo(targetTime) {
            if (!currentItem) return;
            const video = document.getElementById('video');
            const id = currentItem.id;

            isSeeking = true;
            video.pause();

            // Update UI immediately
            document.getElementById('time-current').textContent = fmtTime(targetTime);
            if (knownDuration > 0) {
                document.getElementById('timeline-progress').style.width =
                    ((targetTime / knownDuration) * 100) + '%';
            }

            try {
                // Call the seek endpoint to create a new HLS session at targetTime
                const playbackId = currentPlaybackSessionId || createPlaybackSessionId();
                currentPlaybackSessionId = playbackId;
                const seekRes = await api(
                    'POST',
                    `/api/stream/${id}/hls/seek?start=${targetTime.toFixed(3)}&playback_session_id=${encodeURIComponent(playbackId)}`,
                );

                // Destroy old hls.js instance
                if (currentHls) {
                    currentHls.destroy();
                    currentHls = null;
                }

                // Update offset and session ID
                hlsStartOffset = seekRes.start_secs || targetTime;
                currentHlsSessionId = seekRes.session_id;

                // Create new hls.js instance pointing to the new session's playlist
                const hls = new Hls({
                    xhrSetup: function(xhr, url) {
                        const t = getToken();
                        if (t) xhr.setRequestHeader('Authorization', 'Bearer ' + t);
                    }
                });
                currentHls = hls;

                hls.on(Hls.Events.MANIFEST_PARSED, function() {
                    video.play();
                    isSeeking = false;
                });

                hls.on(Hls.Events.ERROR, function(event, data) {
                    if (data.fatal) {
                        console.error('HLS seek error:', data.type, data.details);
                        isSeeking = false;
                    }
                });

                const sourceUrl = seekRes.master_url || seekRes.playlist_url;
                if (!sourceUrl) {
                    throw new Error('HLS seek response missing master_url');
                }
                hls.loadSource(sourceUrl);
                hls.attachMedia(video);
            } catch (e) {
                console.error('HLS seek failed:', e);
                isSeeking = false;
            }
        }

        async function seekTimeline(event) {
            if (!currentItem || !knownDuration) return;
            const rect = event.currentTarget.getBoundingClientRect();
            const fraction = Math.max(0, Math.min(1, (event.clientX - rect.left) / rect.width));
            const targetTime = fraction * knownDuration;
            const video = document.getElementById('video');

            if (isHls) {
                // HLS: create a new session starting at the target time
                await hlsSeekTo(targetTime);
            } else if (isTranscoded) {
                // fMP4 pipe: need to re-spawn FFmpeg from keyframe
                isSeeking = true;
                video.pause();

                document.getElementById('time-current').textContent = fmtTime(targetTime);
                document.getElementById('timeline-progress').style.width = (fraction * 100) + '%';

                let actualStart = targetTime;
                try {
                    const kfRes = await fetch(authUrl(`/api/stream/${currentItem.id}/keyframe?time=${targetTime.toFixed(3)}`));
                    if (kfRes.ok) {
                        const kfData = await kfRes.json();
                        actualStart = kfData.keyframe;
                    }
                } catch (e) {}

                seekOffset = actualStart;
                video.src = authUrl(`/api/stream/${currentItem.id}?start=${actualStart.toFixed(3)}`);
                video.play();

                document.getElementById('time-current').textContent = fmtTime(actualStart);
                const actualFraction = actualStart / knownDuration;
                document.getElementById('timeline-progress').style.width = (actualFraction * 100) + '%';
                setTimeout(() => { isSeeking = false; }, 500);
            } else {
                // DirectPlay: native seeking
                video.currentTime = targetTime;
            }
        }

        // Seek forward/backward by delta seconds (for keyboard shortcuts)
        async function seekRelative(deltaSec) {
            if (!currentItem || !knownDuration) return;
            const video = document.getElementById('video');

            if (isHls) {
                // HLS: calculate actual media time and seek
                const currentSec = hlsStartOffset + video.currentTime;
                const targetTime = Math.max(0, Math.min(knownDuration, currentSec + deltaSec));
                await hlsSeekTo(targetTime);
            } else if (isTranscoded) {
                // fMP4 pipe: need to re-spawn FFmpeg from keyframe
                const currentSec = seekOffset + video.currentTime;
                const targetTime = Math.max(0, Math.min(knownDuration, currentSec + deltaSec));

                isSeeking = true;
                video.pause();

                let actualStart = targetTime;
                try {
                    const kfRes = await fetch(authUrl(`/api/stream/${currentItem.id}/keyframe?time=${targetTime.toFixed(3)}`));
                    if (kfRes.ok) {
                        const kfData = await kfRes.json();
                        actualStart = kfData.keyframe;
                    }
                } catch (e) {}

                seekOffset = actualStart;
                video.src = authUrl(`/api/stream/${currentItem.id}?start=${actualStart.toFixed(3)}`);
                video.play();

                document.getElementById('time-current').textContent = fmtTime(actualStart);
                const pct = (actualStart / knownDuration) * 100;
                document.getElementById('timeline-progress').style.width = pct + '%';
                setTimeout(() => { isSeeking = false; }, 500);
            } else {
                // DirectPlay: native seeking
                const targetTime = Math.max(0, Math.min(knownDuration, video.currentTime + deltaSec));
                video.currentTime = targetTime;
            }
        }

        function fmtTime(totalSec) {
            if (!totalSec || !isFinite(totalSec)) return '0:00';
            const h = Math.floor(totalSec / 3600);
            const m = Math.floor((totalSec % 3600) / 60);
            const s = Math.floor(totalSec % 60);
            if (h > 0) return `${h}:${m.toString().padStart(2,'0')}:${s.toString().padStart(2,'0')}`;
            return `${m}:${s.toString().padStart(2,'0')}`;
        }

        // Update custom controls + report progress
        (function() {
            const video = document.getElementById('video');
            video.addEventListener('timeupdate', () => {
                if (!currentItem || isSeeking) return;
                const playBtn = document.getElementById('play-btn');
                playBtn.innerHTML = video.paused ? '&#9654;' : '&#10074;&#10074;';

                // For HLS, add hlsStartOffset (FFmpeg started transcoding from that point)
                // For fMP4 pipe, add seekOffset
                const currentSec = isHls
                    ? (hlsStartOffset + video.currentTime)
                    : (seekOffset + video.currentTime);
                document.getElementById('time-current').textContent = fmtTime(currentSec);

                if (knownDuration > 0) {
                    const pct = Math.min(100, (currentSec / knownDuration) * 100);
                    document.getElementById('timeline-progress').style.width = pct + '%';
                }

                // Report progress every 10 seconds
                const now = Date.now();
                if (now - lastProgressReport > 10000) {
                    lastProgressReport = now;
                    const posMs = Math.floor(currentSec * 1000);
                    if (posMs > 0) {
                        apiQuiet('PUT', `/api/progress/${currentItem.id}`, { position_ms: posMs });
                    }
                }
            });
            video.addEventListener('play', () => {
                document.getElementById('play-btn').innerHTML = '&#10074;&#10074;';
            });
            video.addEventListener('pause', () => {
                document.getElementById('play-btn').innerHTML = '&#9654;';
            });
            // Mark as completed when video ends
            video.addEventListener('ended', () => {
                if (currentItem) {
                    apiQuiet('POST', `/api/progress/${currentItem.id}/complete`);
                }
            });
            // Update buffered indicator
            video.addEventListener('progress', () => {
                if (!knownDuration || !video.buffered.length) return;
                const bufferedEnd = video.buffered.end(video.buffered.length - 1);
                const bufferedTotal = isHls ? (hlsStartOffset + bufferedEnd) : (seekOffset + bufferedEnd);
                const pct = Math.min(100, (bufferedTotal / knownDuration) * 100);
                document.getElementById('timeline-buffered').style.width = pct + '%';
            });
        })();

        let currentDetailItem = null;

        async function showDetail(id) {
            currentDetailItem = await api('GET', `/api/media/${id}`);
            const item = currentDetailItem;
            const displayTitle = getDisplayTitle(item);
            const displayYear = getDisplayYear(item);

            document.getElementById('detail-title').textContent = displayTitle;

            const metaParts = [];
            if (displayYear) metaParts.push(displayYear);
            if (item.content_rating) metaParts.push(item.content_rating);
            if (item.duration_ms) metaParts.push(formatDuration(item.duration_ms));
            document.getElementById('detail-meta').textContent = metaParts.join(' \u2022 ');

            document.getElementById('detail-rating').innerHTML = item.rating
                ? '&#9733; ' + item.rating + ' / 10'
                : '';

            let genresHtml = '';
            if (item.genres) {
                try {
                    const genres = JSON.parse(item.genres);
                    genresHtml = genres.map(g => `<span>${g}</span>`).join('');
                } catch(e) {}
            }
            document.getElementById('detail-genres').innerHTML = genresHtml;
            document.getElementById('detail-overview').textContent = item.overview || '';

            const posterEl = document.getElementById('detail-poster');
            if (item.poster_path) {
                posterEl.innerHTML = `<img src="${authUrl('/api/images/' + item.poster_path)}">`;
            } else {
                posterEl.innerHTML = '<div class="no-poster">&#127910;</div>';
            }

            const res = getResLabel(item.width, item.height);
            const streamBadge = getStreamBadge(item);
            const codecBadge = item.video_codec ? `<span class="badge badge-codec">${item.video_codec}</span>` : '';
            const audioBadge = item.audio_codec ? `<span class="badge badge-codec">${item.audio_codec}</span>` : '';
            const resBadge = res ? `<span class="badge badge-res">${res}</span>` : '';
            document.getElementById('detail-badges').innerHTML = `${streamBadge}${resBadge}${codecBadge}${audioBadge}`;

            // Play/Resume buttons
            const actionsEl = document.getElementById('detail-actions');
            if (item.position_ms && item.position_ms > 0 && !item.completed && item.duration_ms) {
                const resumeTime = fmtTime(item.position_ms / 1000);
                actionsEl.innerHTML = `
                    <button onclick="playFromDetail(true)">&#9654; Resume from ${resumeTime}</button>
                    <button class="secondary" onclick="playFromDetail(false)">Play from Start</button>
                `;
            } else {
                actionsEl.innerHTML = `<button onclick="playFromDetail(false)">&#9654; Play</button>`;
            }

            document.getElementById('detail-view').classList.add('active');
        }

        function closeDetail() {
            document.getElementById('detail-view').classList.remove('active');
            currentDetailItem = null;
        }

        function playFromDetail(resume) {
            if (!currentDetailItem) return;
            const id = currentDetailItem.id;
            if (resume) {
                const resumePos = currentDetailItem.position_ms / 1000;
                closeDetail();
                playMedia(id, resumePos);
            } else {
                // "Play from Start" — reset progress in DB so the detail view
                // won't show stale resume position next time
                apiQuiet('PUT', `/api/progress/${id}`, { position_ms: 0 });
                closeDetail();
                playMedia(id, 0);
            }
        }

        function showAddLibrary() {
            document.getElementById('add-dialog').classList.add('active');
        }

        function hideAddLibrary() {
            document.getElementById('add-dialog').classList.remove('active');
        }

        async function addLibrary() {
            const name = document.getElementById('lib-name').value.trim();
            const path = document.getElementById('lib-path').value.trim();
            const type_ = document.getElementById('lib-type').value;
            if (!name || !path) return alert('Name and path are required');

            setStatus('Creating library...', true);
            const lib = await api('POST', '/api/libraries', { name, path, library_type: type_ });
            hideAddLibrary();

            setStatus('Scanning library...', true);
            await api('POST', `/api/libraries/${lib.id}/scan`);
            setStatus('Scan complete');

            await loadLibraries();
            await loadMedia();
        }

        async function deleteLibrary(id) {
            if (!confirm('Delete this library? Media files on disk will not be affected.')) return;
            await api('DELETE', `/api/libraries/${id}`);
            if (currentLibrary === id) currentLibrary = null;
            await loadLibraries();
            await loadMedia();
        }

        async function refreshAll() {
            setStatus('Refreshing...', true);
            const libs = await api('GET', '/api/libraries');
            for (const lib of libs) {
                setStatus(`Scanning ${lib.name}...`, true);
                await api('POST', `/api/libraries/${lib.id}/scan`);
            }
            setStatus('Refresh complete');
            await loadMedia();
        }

        // ---- Keyboard shortcuts ----
        document.addEventListener('keydown', (e) => {
            const playerActive = document.getElementById('player').classList.contains('active');
            const detailActive = document.getElementById('detail-view').classList.contains('active');
            const dialogActive = document.getElementById('add-dialog').classList.contains('active');
            const isInputFocused = ['INPUT','SELECT','TEXTAREA'].includes(document.activeElement.tagName);

            if (e.key === 'Escape') {
                if (playerActive) {
                    closePlayer();
                } else if (detailActive) {
                    closeDetail();
                } else if (dialogActive) {
                    hideAddLibrary();
                } else if (isInputFocused) {
                    document.activeElement.blur();
                    document.getElementById('search-input').value = '';
                    localStorage.removeItem('ferrite-search');
                    filterAndRender();
                }
                return;
            }

            // Player keyboard shortcuts
            if (playerActive && !isInputFocused) {
                if (e.key === ' ' || e.code === 'Space') {
                    e.preventDefault();
                    togglePlay();
                } else if (e.key === 'ArrowLeft') {
                    e.preventDefault();
                    seekRelative(-10);
                } else if (e.key === 'ArrowRight') {
                    e.preventDefault();
                    seekRelative(10);
                } else if (e.key === 'm' || e.key === 'M') {
                    toggleMute();
                } else if (e.key === 'f' || e.key === 'F') {
                    toggleFullscreen();
                }
                return;
            }

            // Browse mode shortcuts
            if (!playerActive && !detailActive && !dialogActive && !isInputFocused) {
                if (e.key === '/' || (e.ctrlKey && e.key === 'k')) {
                    e.preventDefault();
                    document.getElementById('search-input').focus();
                }
            }
        });

        // ---- Auth functions ----
        function showLoginPage() {
            document.getElementById('login-page').classList.add('active');
            document.getElementById('login-error').style.display = 'none';
            document.getElementById('login-username').value = '';
            document.getElementById('login-password').value = '';
            setTimeout(() => document.getElementById('login-username').focus(), 100);
        }
        function hideLoginPage() {
            document.getElementById('login-page').classList.remove('active');
        }
        async function doLogin() {
            const username = document.getElementById('login-username').value.trim();
            const password = document.getElementById('login-password').value;
            if (!username || !password) return;
            try {
                const res = await fetch('/api/auth/login', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ username, password }),
                });
                if (!res.ok) {
                    const err = await res.json();
                    const errEl = document.getElementById('login-error');
                    errEl.textContent = err.error || 'Invalid credentials';
                    errEl.style.display = 'block';
                    return;
                }
                const data = await res.json();
                setToken(data.token);
                hideLoginPage();
                await loadLibraries();
                await loadMedia();
            } catch (e) {
                const errEl = document.getElementById('login-error');
                errEl.textContent = 'Connection error';
                errEl.style.display = 'block';
            }
        }
        function logout() {
            clearToken();
            showLoginPage();
        }

        // Initial load — check auth status first
        (async () => {
            try {
                const status = await fetch('/api/auth/status').then(r => r.json());
                if (status.auth_required) {
                    document.getElementById('logout-btn').style.display = 'inline-block';
                    if (!getToken()) { showLoginPage(); return; }
                    // Validate token with a test call
                    try {
                        await api('GET', '/api/system/info');
                    } catch (e) { return; /* showLoginPage already called by api() on 401 */ }
                }
            } catch (e) {
                // auth/status unreachable — proceed without auth
            }
            await loadLibraries();
            await loadMedia();
        })();
    </script>
</body>
</html>"#;
