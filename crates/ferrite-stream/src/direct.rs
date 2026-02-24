use axum::http::{HeaderMap, Request, StatusCode};
use axum::response::Response;
use std::path::Path;
use tower::ServiceExt;
use tower_http::services::ServeFile;

/// Serve a media file with HTTP byte-range support using `tower-http` ServeFile for optimal
/// zero-copy / sendfile support.
pub async fn serve_file(file_path: &Path, headers: &HeaderMap) -> Result<Response, StatusCode> {
    if !file_path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    let mut req = Request::builder().method("GET").uri("/"); // The URI doesn't matter for ServeFile when initialized with a specific path

    // Forward relevant headers (especially Range)
    for (name, value) in headers.iter() {
        req = req.header(name, value);
    }

    let request = req
        .body(axum::body::Body::empty())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let service = ServeFile::new(file_path);

    match service.oneshot(request).await {
        Ok(res) => Ok(res.map(axum::body::Body::new)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
