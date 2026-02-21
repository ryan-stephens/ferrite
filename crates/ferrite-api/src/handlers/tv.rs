use crate::auth::AuthUser;
use crate::error::ApiError;
use crate::state::AppState;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::{Extension, Json};
use ferrite_db::tv_repo;

fn extract_user_id(auth_user: &Option<AuthUser>) -> Option<&str> {
    auth_user.as_ref().map(|u| u.user_id.as_str())
}

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
    auth_user: Option<Extension<AuthUser>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let user = auth_user.map(|e| e.0);
    let user_id = extract_user_id(&user);
    let episodes = tv_repo::list_episodes(&state.db, &id, user_id).await?;
    Ok(Json(episodes))
}

/// GET /api/episodes/{media_item_id}/next — get the next episode after this one
pub async fn next_episode(
    State(state): State<AppState>,
    Path(media_item_id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let next = tv_repo::get_next_episode(&state.db, &media_item_id).await?;
    match next {
        Some(ep) => Ok(Json(serde_json::json!({ "next": ep }))),
        None => Ok(Json(serde_json::json!({ "next": null }))),
    }
}
