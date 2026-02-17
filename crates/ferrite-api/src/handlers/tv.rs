use crate::error::ApiError;
use crate::state::AppState;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;
use ferrite_db::tv_repo;

/// GET /api/shows?library_id={id} — list all TV shows in a library
pub async fn list_shows(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<ListShowsQuery>,
) -> Result<impl IntoResponse, ApiError> {
    let library_id = params
        .library_id
        .as_deref()
        .ok_or_else(|| ApiError::bad_request("library_id query parameter is required"))?;

    let shows = tv_repo::list_shows(&state.db, library_id).await?;
    Ok(Json(shows))
}

#[derive(serde::Deserialize)]
pub struct ListShowsQuery {
    pub library_id: Option<String>,
}

/// GET /api/shows/{id} — get a single TV show with season/episode counts
pub async fn get_show(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let show = tv_repo::get_show(&state.db, &id)
        .await?
        .ok_or_else(|| ApiError::not_found(format!("TV show '{id}' not found")))?;
    Ok(Json(show))
}

/// GET /api/shows/{id}/seasons — list all seasons for a TV show
pub async fn list_seasons(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let seasons = tv_repo::list_seasons(&state.db, &id).await?;
    Ok(Json(seasons))
}

/// GET /api/seasons/{id}/episodes — list all episodes in a season
pub async fn list_episodes(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let episodes = tv_repo::list_episodes(&state.db, &id).await?;
    Ok(Json(episodes))
}
