use axum::body::Body;
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::Response;
use std::path::Path;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt};
use tokio_util::io::ReaderStream;
use tracing::debug;

/// Serve a media file with HTTP byte-range support.
pub async fn serve_file(file_path: &Path, headers: &HeaderMap) -> Result<Response, StatusCode> {
    let metadata = tokio::fs::metadata(file_path)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let file_size = metadata.len();
    let content_type = mime_guess::from_path(file_path)
        .first_or_octet_stream()
        .to_string();

    // Parse Range header
    let range = headers
        .get(header::RANGE)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| parse_range(s, file_size));

    match range {
        Some((start, end)) => {
            let length = end - start + 1;
            debug!(
                "Serving range {}-{}/{} for {}",
                start,
                end,
                file_size,
                file_path.display()
            );

            let mut file = File::open(file_path)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            file.seek(std::io::SeekFrom::Start(start))
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            let limited = file.take(length);
            let stream = ReaderStream::new(limited);
            let body = Body::from_stream(stream);

            Ok(Response::builder()
                .status(StatusCode::PARTIAL_CONTENT)
                .header(header::CONTENT_TYPE, content_type)
                .header(header::ACCEPT_RANGES, "bytes")
                .header(
                    header::CONTENT_RANGE,
                    format!("bytes {}-{}/{}", start, end, file_size),
                )
                .header(header::CONTENT_LENGTH, length.to_string())
                .body(body)
                .unwrap())
        }
        None => {
            debug!(
                "Serving full file {}/{} for {}",
                file_size,
                file_size,
                file_path.display()
            );

            let file = File::open(file_path)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            let stream = ReaderStream::new(file);
            let body = Body::from_stream(stream);

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, content_type)
                .header(header::ACCEPT_RANGES, "bytes")
                .header(header::CONTENT_LENGTH, file_size.to_string())
                .body(body)
                .unwrap())
        }
    }
}

/// Parse an HTTP Range header value like "bytes=0-1023" or "bytes=1024-".
fn parse_range(range_header: &str, file_size: u64) -> Option<(u64, u64)> {
    let range_str = range_header.strip_prefix("bytes=")?;
    let parts: Vec<&str> = range_str.splitn(2, '-').collect();

    if parts.len() != 2 {
        return None;
    }

    if parts[0].is_empty() {
        // Suffix range: "-500" means last 500 bytes
        let suffix: u64 = parts[1].parse().ok()?;
        let start = file_size.saturating_sub(suffix);
        return Some((start, file_size - 1));
    }

    let start: u64 = parts[0].parse().ok()?;

    let end: u64 = if parts[1].is_empty() {
        file_size - 1
    } else {
        parts[1].parse().ok()?
    };

    if start > end || start >= file_size {
        return None;
    }

    let end = end.min(file_size - 1);
    Some((start, end))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_range_full() {
        assert_eq!(parse_range("bytes=0-999", 1000), Some((0, 999)));
    }

    #[test]
    fn test_parse_range_open_end() {
        assert_eq!(parse_range("bytes=500-", 1000), Some((500, 999)));
    }

    #[test]
    fn test_parse_range_suffix() {
        assert_eq!(parse_range("bytes=-200", 1000), Some((800, 999)));
    }

    #[test]
    fn test_parse_range_invalid() {
        assert_eq!(parse_range("bytes=1000-999", 1000), None);
        assert_eq!(parse_range("bytes=1001-", 1000), None);
        assert_eq!(parse_range("invalid", 1000), None);
    }

    #[test]
    fn test_parse_range_clamp() {
        assert_eq!(parse_range("bytes=0-5000", 1000), Some((0, 999)));
    }
}
