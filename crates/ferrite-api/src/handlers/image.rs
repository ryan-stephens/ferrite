use crate::state::AppState;
use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use tokio::fs::File;
use tokio_util::io::ReaderStream;

/// Serve a cached image file (posters, backdrops) from the image cache directory.
/// Uses streaming instead of reading the entire file into memory, and supports
/// ETag/If-None-Match for cache validation (images are immutable once cached).
pub async fn serve_image(
    State(state): State<AppState>,
    Path(filename): Path<String>,
    headers: HeaderMap,
) -> impl IntoResponse {
    // Sanitize: prevent directory traversal
    if filename.contains("..") || filename.contains('/') || filename.contains('\\') {
        return StatusCode::BAD_REQUEST.into_response();
    }

    let image_path = state.config.metadata.image_cache_dir.join(&filename);

    let metadata = match tokio::fs::metadata(&image_path).await {
        Ok(m) => m,
        Err(_) => return StatusCode::NOT_FOUND.into_response(),
    };

    // Generate ETag from filename + file size (images are immutable once cached)
    let etag = format!("\"{}_{}_{}\"", filename, metadata.len(), "v1");

    // Check If-None-Match — return 304 if the client already has this version
    if let Some(if_none_match) = headers.get(header::IF_NONE_MATCH) {
        if let Ok(client_etag) = if_none_match.to_str() {
            if client_etag == etag {
                return Response::builder()
                    .status(StatusCode::NOT_MODIFIED)
                    .header(header::ETAG, &etag)
                    .header(header::CACHE_CONTROL, "public, max-age=86400, immutable")
                    .body(Body::empty())
                    .unwrap()
                    .into_response();
            }
        }
    }

    let content_type = if filename.ends_with(".png") {
        "image/png"
    } else if filename.ends_with(".webp") {
        "image/webp"
    } else {
        "image/jpeg"
    };

    // Stream the file instead of reading it all into memory
    let file = match File::open(&image_path).await {
        Ok(f) => f,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, content_type)
        .header(header::CONTENT_LENGTH, metadata.len().to_string())
        .header(header::CACHE_CONTROL, "public, max-age=86400, immutable")
        .header(header::ETAG, &etag)
        .body(body)
        .unwrap()
        .into_response()
}
