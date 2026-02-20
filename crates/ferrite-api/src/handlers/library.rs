use crate::error::ApiError;
use crate::state::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use ferrite_db::{library_repo, media_repo};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct CreateLibraryRequest {
    pub name: String,
    pub path: String,
    pub library_type: String,
}

pub async fn list_libraries(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    let libs = library_repo::list_libraries(&state.db).await?;
    Ok(Json(libs))
}

pub async fn create_library(
    State(state): State<AppState>,
    Json(req): Json<CreateLibraryRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let lib_type = match req.library_type.as_str() {
        "movies" | "movie" => ferrite_core::media::LibraryType::Movie,
        "tv" => ferrite_core::media::LibraryType::Tv,
        "music" => ferrite_core::media::LibraryType::Music,
        other => return Err(ApiError::bad_request(&format!("Unknown library type: '{}'", other))),
    };

    let lib = library_repo::create_library(&state.db, &req.name, &req.path, lib_type).await?;
    Ok((StatusCode::CREATED, Json(lib)))
}

pub async fn delete_library(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    media_repo::delete_media_items_for_library(&state.db, &id).await?;
    library_repo::delete_library(&state.db, &id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn scan_library(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let _library = ferrite_db::library_repo::get_library(&state.db, &id).await?;

    // Prevent duplicate concurrent scans for the same library
    let scan_state = state.scan_registry.try_start(id.clone())
        .ok_or_else(|| ApiError::bad_request("Scan already in progress for this library"))?;

    let db = state.db.clone();
    let config = state.config.clone();
    let lib_id = id.clone();

    tokio::spawn(async move {
        let ffprobe_path = config.transcode.ffprobe_path.clone();
        let ffmpeg_path = config.transcode.ffmpeg_path.clone();
        let concurrent_probes = config.scanner.concurrent_probes;
        let subtitle_cache_dir = config.scanner.subtitle_cache_dir.clone();

        // Build optional TMDB provider for inline enrichment
        let (tmdb_provider, image_cache) = if let Some(ref api_key) = config.metadata.tmdb_api_key {
            let provider: Arc<dyn ferrite_metadata::provider::MetadataProvider> = Arc::new(
                ferrite_metadata::tmdb::TmdbProvider::new(
                    api_key.clone(),
                    config.metadata.rate_limit_per_second,
                )
            );
            let cache = Arc::new(ferrite_metadata::image_cache::ImageCache::new(
                config.metadata.image_cache_dir.clone(),
            ));
            (Some(provider), Some(cache))
        } else {
            (None, None)
        };

        match ferrite_scanner::scan_library(
            &db,
            &lib_id,
            &ffprobe_path,
            &ffmpeg_path,
            concurrent_probes,
            &subtitle_cache_dir,
            scan_state,
            tmdb_provider,
            image_cache,
        ).await {
            Ok(count) => {
                tracing::info!("Scan complete for library {}: {} new items", lib_id, count);
            }
            Err(e) => {
                tracing::error!("Scan failed for library {}: {}", lib_id, e);
            }
        }
    });

    Ok((StatusCode::ACCEPTED, Json(serde_json::json!({ "status": "scanning" }))))
}

pub async fn scan_status(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    match state.scan_registry.get(&id) {
        Some(scan_state) => Ok(Json(scan_state.to_progress().await)),
        None => Ok(Json(ferrite_scanner::ScanProgress {
            scanning: false,
            status: ferrite_scanner::progress::ScanStatus::Complete,
            total_files: 0,
            files_probed: 0,
            files_inserted: 0,
            subtitles_extracted: 0,
            items_enriched: 0,
            errors: 0,
            current_item: String::new(),
            elapsed_seconds: 0,
            percent: 100,
        })),
    }
}
