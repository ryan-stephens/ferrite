use crate::error::ApiError;
use crate::state::AppState;
use axum::extract::{Path, State};
use axum::http::{header, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use ferrite_db::media_repo;
use ferrite_transcode::thumbnails::{self, ThumbnailConfig};
use tracing::{info, warn};

/// POST /api/media/{id}/thumbnails — Generate sprite sheet for a media item.
/// Returns immediately if sprites already exist.
pub async fn generate_thumbnails(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let item = media_repo::get_media_item(&state.db, &id)
        .await?
        .ok_or_else(|| ApiError::not_found(format!("Media item '{id}' not found")))?;

    let duration_secs = item
        .duration_ms
        .map(|ms| ms as f64 / 1000.0)
        .ok_or_else(|| ApiError::bad_request("Media item has no duration"))?;

    let thumb_dir = state.config.transcode.cache_dir.join("thumbnails");

    // Check if already generated
    if thumbnails::sprite_sheet_exists(&thumb_dir, &id) {
        return Ok(Json(serde_json::json!({
            "status": "exists",
            "media_id": id,
        })));
    }

    let config = ThumbnailConfig::default();
    let video_path = std::path::Path::new(&item.file_path);

    let result = thumbnails::generate_sprite_sheet(
        &state.config.transcode.ffmpeg_path,
        video_path,
        &thumb_dir,
        &id,
        duration_secs,
        &config,
    )
    .await
    .map_err(|e| {
        warn!("Failed to generate thumbnails for {}: {}", id, e);
        ApiError::internal(format!("Thumbnail generation failed: {}", e))
    })?;

    info!(
        "Generated sprite sheet for {}: {}x{} grid, {} thumbnails",
        id, result.columns, result.rows, result.thumb_count
    );

    Ok(Json(serde_json::json!({
        "status": "generated",
        "media_id": id,
        "thumb_count": result.thumb_count,
        "columns": result.columns,
        "rows": result.rows,
        "thumb_width": result.thumb_width,
        "thumb_height": result.thumb_height,
        "interval_secs": result.interval_secs,
    })))
}

/// GET /api/media/{id}/thumbnails/sprites.jpg — Serve the sprite sheet image.
pub async fn serve_sprite_image(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let thumb_dir = state.config.transcode.cache_dir.join("thumbnails");
    let sprite_path = thumb_dir.join(format!("{}_sprites.jpg", id));

    if !sprite_path.exists() {
        return Err(ApiError::not_found(
            "Sprite sheet not found. POST to /api/media/{id}/thumbnails to generate.",
        ));
    }

    let data = tokio::fs::read(&sprite_path)
        .await
        .map_err(|e| ApiError::internal(format!("Failed to read sprite sheet: {}", e)))?;

    Ok((
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "image/jpeg"),
            (header::CACHE_CONTROL, "public, max-age=86400"),
        ],
        data,
    ))
}

/// GET /api/media/{id}/thumbnails/sprites.vtt — Serve the WebVTT thumbnail map.
pub async fn serve_sprite_vtt(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let thumb_dir = state.config.transcode.cache_dir.join("thumbnails");
    let vtt_path = thumb_dir.join(format!("{}_sprites.vtt", id));

    if !vtt_path.exists() {
        return Err(ApiError::not_found(
            "Sprite VTT not found. POST to /api/media/{id}/thumbnails to generate.",
        ));
    }

    let data = tokio::fs::read_to_string(&vtt_path)
        .await
        .map_err(|e| ApiError::internal(format!("Failed to read sprite VTT: {}", e)))?;

    Ok((
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "text/vtt"),
            (header::CACHE_CONTROL, "public, max-age=86400"),
        ],
        data,
    ))
}
