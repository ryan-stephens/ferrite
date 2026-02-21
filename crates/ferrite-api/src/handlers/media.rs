use crate::auth::AuthUser;
use crate::error::ApiError;
use crate::state::AppState;
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::{Extension, Json};
use ferrite_db::{chapter_repo, movie_repo, stream_repo};
use serde::Deserialize;

fn extract_user_id(auth_user: &Option<AuthUser>) -> Option<&str> {
    auth_user.as_ref().map(|u| u.user_id.as_str())
}

#[derive(Deserialize)]
pub struct ListMediaQuery {
    pub library_id: Option<String>,
    pub search: Option<String>,
    pub genre: Option<String>,
    pub sort: Option<String>,
    pub dir: Option<String>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

pub async fn list_media(
    State(state): State<AppState>,
    auth_user: Option<Extension<AuthUser>>,
    Query(query): Query<ListMediaQuery>,
) -> Result<impl IntoResponse, ApiError> {
    let user = auth_user.map(|e| e.0);
    let user_id = extract_user_id(&user);
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(50).clamp(1, 200);

    let mq = movie_repo::MediaQuery {
        library_id: query.library_id.as_deref(),
        search: query.search.as_deref(),
        genre: query.genre.as_deref(),
        sort_by: query.sort.as_deref(),
        sort_dir: query.dir.as_deref(),
        page: page as i64,
        per_page: per_page as i64,
    };

    let items = movie_repo::list_movies_with_media(&state.db, &mq, user_id).await?;
    let total = movie_repo::count_movies_with_media(&state.db, &mq).await?;

    Ok(Json(serde_json::json!({
        "items": items,
        "total": total,
        "page": page,
        "per_page": per_page,
    })))
}

pub async fn get_media(
    State(state): State<AppState>,
    auth_user: Option<Extension<AuthUser>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let user = auth_user.map(|e| e.0);
    let user_id = extract_user_id(&user);
    // Use enriched query with movie metadata
    let item = movie_repo::get_movie_with_media(&state.db, &id, user_id)
        .await?
        .ok_or_else(|| ApiError::not_found(format!("Media item '{id}' not found")))?;
    Ok(Json(item))
}

/// GET /api/media/{id}/streams — list all audio/video/subtitle streams for a media item
pub async fn get_media_streams(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let streams = stream_repo::get_streams(&state.db, &id).await?;
    Ok(Json(streams))
}

/// GET /api/media/{id}/chapters — list all chapter markers for a media item
pub async fn get_media_chapters(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let chapters = chapter_repo::get_chapters(&state.db, &id).await?;
    Ok(Json(chapters))
}
