use crate::error::ApiError;
use crate::state::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use ferrite_db::{library_repo, media_repo};
use serde::Deserialize;

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
        "tv" => ferrite_core::media::LibraryType::Tv,
        "music" => ferrite_core::media::LibraryType::Music,
        _ => ferrite_core::media::LibraryType::Movie,
    };

    let lib = library_repo::create_library(&state.db, &req.name, &req.path, lib_type).await?;
    Ok((StatusCode::CREATED, Json(lib)))
}

pub async fn delete_library(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    // Delete media items first, then the library
    media_repo::delete_media_items_for_library(&state.db, &id).await?;
    library_repo::delete_library(&state.db, &id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn scan_library(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    // Verify library exists before spawning background task
    let _library = ferrite_db::library_repo::get_library(&state.db, &id).await?;

    // Spawn the entire scan + enrichment as a background task so the HTTP
    // response returns immediately (avoids reverse-proxy 504 timeouts on
    // large libraries).
    let db = state.db.clone();
    let config = state.config.clone();
    let lib_id = id.clone();
    tokio::spawn(async move {
        let ffprobe_path = &config.transcode.ffprobe_path;
        let ffmpeg_path = &config.transcode.ffmpeg_path;
        let concurrent_probes = config.scanner.concurrent_probes;
        let subtitle_cache_dir = &config.scanner.subtitle_cache_dir;

        match ferrite_scanner::scan_library(&db, &lib_id, ffprobe_path, ffmpeg_path, concurrent_probes, subtitle_cache_dir).await {
            Ok(count) => {
                tracing::info!("Background scan complete for library {}: {} items", lib_id, count);
            }
            Err(e) => {
                tracing::error!("Background scan failed for library {}: {}", lib_id, e);
                return;
            }
        }

        // Metadata enrichment (TMDb)
        if config.metadata.tmdb_api_key.is_some() {
            let api_key = config.metadata.tmdb_api_key.as_ref().unwrap();
            let provider = ferrite_metadata::tmdb::TmdbProvider::new(
                api_key.clone(),
                config.metadata.rate_limit_per_second,
            );
            let image_cache = ferrite_metadata::image_cache::ImageCache::new(
                config.metadata.image_cache_dir.clone(),
            );
            if let Err(e) = ferrite_metadata::enrichment::enrich_library_movies(
                &db,
                &lib_id,
                &provider,
                &image_cache,
            )
            .await
            {
                tracing::warn!("Movie metadata enrichment failed for library {}: {}", lib_id, e);
            }
            if let Err(e) = ferrite_metadata::enrichment::enrich_library_shows(
                &db,
                &lib_id,
                &provider,
                &image_cache,
            )
            .await
            {
                tracing::warn!("TV metadata enrichment failed for library {}: {}", lib_id, e);
            }
        }
    });

    Ok((StatusCode::ACCEPTED, Json(serde_json::json!({ "status": "scanning" }))))
}
