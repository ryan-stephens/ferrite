use crate::auth;
use crate::handlers::{collection, image, library, media, progress, stream, subtitle, system, thumbnail, tv, user, webhook};
use crate::state::AppState;
use axum::http::{header, Method, Request};
use axum::middleware;
use axum::routing::{delete, get, post, put};
use axum::Router;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;

/// Build the full Axum router with all API routes and middleware.
pub fn build_router(state: AppState) -> Router {
    // Public routes — no auth required
    let public_routes = Router::new()
        .route("/api/health", get(system::health))
        .route("/api/auth/login", post(auth::login))
        .route("/api/auth/status", get(auth::auth_status))
        .route("/api/users/setup", get(user::setup_status))
        .route("/api/users", post(user::create_user));

    // Protected routes — auth middleware applied
    let protected_routes = Router::new()
        .route("/api/system/info", get(system::info))
        .route("/api/system/encoder", get(system::encoder_info))
        // Libraries
        .route("/api/libraries", get(library::list_libraries))
        .route("/api/libraries", post(library::create_library))
        .route("/api/libraries/{id}", delete(library::delete_library))
        .route("/api/libraries/{id}/scan", post(library::scan_library))
        // Media
        .route("/api/media", get(media::list_media))
        .route("/api/media/{id}", get(media::get_media))
        .route("/api/media/{id}/streams", get(media::get_media_streams))
        // Subtitles
        .route("/api/media/{id}/subtitles", get(subtitle::list_subtitles))
        .route("/api/subtitles/{id}/serve", get(subtitle::serve_subtitle))
        // TV Shows
        .route("/api/shows", get(tv::list_shows))
        .route("/api/shows/{id}", get(tv::get_show))
        .route("/api/shows/{id}/seasons", get(tv::list_seasons))
        .route("/api/seasons/{id}/episodes", get(tv::list_episodes))
        // Images
        .route("/api/images/{filename}", get(image::serve_image))
        // Collections & Playlists
        .route("/api/collections", get(collection::list_collections).post(collection::create_collection))
        .route("/api/collections/{id}", get(collection::get_collection).put(collection::update_collection).delete(collection::delete_collection))
        .route("/api/collections/{id}/items", post(collection::add_item))
        .route("/api/collections/{id}/items/{media_id}", delete(collection::remove_item))
        .route("/api/collections/{id}/reorder", put(collection::reorder_item))
        // Webhooks
        .route("/api/webhooks", get(webhook::list_webhooks).post(webhook::create_webhook))
        .route("/api/webhooks/events", get(webhook::list_event_types))
        .route("/api/webhooks/{id}", get(webhook::get_webhook).put(webhook::update_webhook).delete(webhook::delete_webhook))
        .route("/api/webhooks/{id}/test", post(webhook::test_webhook))
        // Thumbnails
        .route("/api/media/{id}/thumbnails", post(thumbnail::generate_thumbnails))
        .route("/api/media/{id}/thumbnails/sprites.jpg", get(thumbnail::serve_sprite_image))
        .route("/api/media/{id}/thumbnails/sprites.vtt", get(thumbnail::serve_sprite_vtt))
        // Streaming
        .route("/api/stream/{id}", get(stream::stream_media))
        .route("/api/stream/{id}/keyframe", get(stream::find_keyframe))
        // HLS Streaming
        .route("/api/stream/{id}/hls/master.m3u8", get(stream::hls_master_playlist))
        .route("/api/stream/{id}/hls/seek", post(stream::hls_seek))
        .route("/api/stream/{id}/hls/{session_id}/playlist.m3u8", get(stream::hls_variant_playlist))
        .route("/api/stream/{id}/hls/{session_id}/{filename}", get(stream::hls_segment))
        .route("/api/stream/{id}/hls/{session_id}", delete(stream::hls_stop))
        // Users
        .route("/api/users", get(user::list_users))
        .route("/api/users/me", get(user::get_current_user))
        .route("/api/users/me/password", put(user::change_password))
        // Playback Progress
        .route(
            "/api/progress/{media_id}",
            get(progress::get_progress).put(progress::update_progress),
        )
        .route(
            "/api/progress/{media_id}/complete",
            post(progress::mark_completed),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth::require_auth,
        ));

    // Resolve the SPA directory by checking multiple locations in priority order.
    // This ensures the binary finds the UI whether running from the repo root (dev),
    // from ~/ferrite/ (seedbox), or from an installed location.
    let spa_dir = resolve_spa_dir();
    let app = Router::new()
        .merge(public_routes)
        .merge(protected_routes);

    let app = if let Some(dir) = spa_dir {
        tracing::info!("Serving SPA from: {}", dir.display());
        let index = dir.join("index.html");
        let serve = ServeDir::new(&dir)
            .not_found_service(ServeFile::new(&index));
        app.fallback_service(serve)
    } else {
        tracing::info!("No SPA directory found, using embedded HTML fallback");
        app.fallback(system::serve_frontend)
    };

    // Build CORS policy based on config.
    // Empty cors_origins = allow all origins (recommended for seedbox/remote access).
    // Populated cors_origins = restrict to listed origins only.
    let cors_origins = &state.config.server.cors_origins;
    let cors = if cors_origins.is_empty() {
        CorsLayer::new()
            .allow_origin(AllowOrigin::any())
            .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
            .allow_headers([
                header::AUTHORIZATION,
                header::CONTENT_TYPE,
                header::ACCEPT,
                "X-API-Key".parse().unwrap(),
            ])
            .expose_headers([
                "X-Seek-Actual".parse().unwrap(),
                "X-Content-Duration".parse().unwrap(),
                "X-Total-Duration".parse().unwrap(),
                "x-hls-start-secs".parse().unwrap(),
                "Server-Timing".parse().unwrap(),
            ])
            // Note: allow_credentials(true) is incompatible with allow_origin(any()),
            // but credentials are sent via Authorization header which works fine without it.
    } else {
        let allowed: Vec<header::HeaderValue> = cors_origins
            .iter()
            .filter_map(|o| o.parse().ok())
            .collect();
        CorsLayer::new()
            .allow_origin(AllowOrigin::list(allowed))
            .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
            .allow_headers([
                header::AUTHORIZATION,
                header::CONTENT_TYPE,
                header::ACCEPT,
                "X-API-Key".parse().unwrap(),
            ])
            .expose_headers([
                "X-Seek-Actual".parse().unwrap(),
                "X-Content-Duration".parse().unwrap(),
                "X-Total-Duration".parse().unwrap(),
                "x-hls-start-secs".parse().unwrap(),
                "Server-Timing".parse().unwrap(),
            ])
            .allow_credentials(true)
    };

    app
        .layer(cors)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    // Redact auth tokens from logged URLs to prevent leaking
                    // JWTs into log files (HLS playlists pass ?token=...).
                    let path = request.uri().path();
                    let safe_query = request
                        .uri()
                        .query()
                        .map(|q| redact_token_param(q))
                        .unwrap_or_default();
                    let uri = if safe_query.is_empty() {
                        path.to_string()
                    } else {
                        format!("{}?{}", path, safe_query)
                    };
                    tracing::info_span!(
                        "http_request",
                        method = %request.method(),
                        uri = %uri,
                    )
                }),
        )
        .with_state(state)
}

/// Resolve the SPA directory by checking multiple locations in priority order.
/// Returns the first directory that contains an `index.html`, or `None`.
///
/// Search order:
/// 1. `$FERRITE_STATIC_DIR` env var (explicit override)
/// 2. `<exe_dir>/static/` (deployed alongside binary, e.g. ~/ferrite/static/)
/// 3. `ferrite-ui/dist/` relative to CWD (development mode, running from repo root)
fn resolve_spa_dir() -> Option<std::path::PathBuf> {
    let candidates: Vec<std::path::PathBuf> = vec![
        // 1. Explicit env var override
        std::env::var("FERRITE_STATIC_DIR").ok().map(std::path::PathBuf::from),
        // 2. Exe-relative static/ (seedbox: ~/ferrite/static/)
        std::env::current_exe().ok().and_then(|exe| {
            exe.parent().map(|dir| dir.join("static"))
        }),
        // 3. CWD-relative ferrite-ui/dist/ (dev mode)
        Some(std::path::PathBuf::from("ferrite-ui/dist")),
    ]
    .into_iter()
    .flatten()
    .collect();

    for dir in candidates {
        if dir.join("index.html").exists() {
            return Some(dir);
        }
    }
    None
}

/// Redact the `token` query parameter from a URL query string so JWT values
/// don't leak into log files. Other parameters are preserved as-is.
/// Example: `"token=abc123&start=5.0"` → `"token=[REDACTED]&start=5.0"`
fn redact_token_param(query: &str) -> String {
    query
        .split('&')
        .map(|pair| {
            if let Some(key) = pair.split('=').next() {
                if key == "token" {
                    return "token=[REDACTED]".to_string();
                }
            }
            pair.to_string()
        })
        .collect::<Vec<_>>()
        .join("&")
}
