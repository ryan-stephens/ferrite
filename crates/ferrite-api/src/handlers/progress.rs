use crate::auth::AuthUser;
use crate::error::ApiError;
use crate::state::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use ferrite_db::progress_repo;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateProgressRequest {
    pub position_ms: i64,
}

/// Extract the user_id from the auth extension (if present).
fn extract_user_id(auth_user: &Option<AuthUser>) -> Option<&str> {
    auth_user.as_ref().map(|u| u.user_id.as_str())
}

/// PUT /api/progress/{media_id} — update playback position
pub async fn update_progress(
    State(state): State<AppState>,
    auth_user: Option<Extension<AuthUser>>,
    Path(media_id): Path<String>,
    Json(req): Json<UpdateProgressRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let user = auth_user.map(|e| e.0);
    let user_id = extract_user_id(&user);
    progress_repo::upsert_progress(&state.db.write, &media_id, user_id, req.position_ms).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// GET /api/progress/{media_id} — get playback progress for a media item
pub async fn get_progress(
    State(state): State<AppState>,
    auth_user: Option<Extension<AuthUser>>,
    Path(media_id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let user = auth_user.map(|e| e.0);
    let user_id = extract_user_id(&user);
    let progress = progress_repo::get_progress(&state.db.read, &media_id, user_id).await?;
    match progress {
        Some(row) => Ok(Json(serde_json::json!(row)).into_response()),
        None => Ok(Json(serde_json::json!({
            "media_item_id": media_id,
            "position_ms": 0,
            "completed": 0,
        }))
        .into_response()),
    }
}

/// DELETE /api/progress/{media_id} — reset progress (mark as unwatched)
pub async fn reset_progress(
    State(state): State<AppState>,
    auth_user: Option<Extension<AuthUser>>,
    Path(media_id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let user = auth_user.map(|e| e.0);
    let user_id = extract_user_id(&user);
    progress_repo::reset_progress(&state.db.write, &media_id, user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// POST /api/progress/{media_id}/complete — mark as completed
pub async fn mark_completed(
    State(state): State<AppState>,
    auth_user: Option<Extension<AuthUser>>,
    Path(media_id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let user = auth_user.map(|e| e.0);
    let user_id = extract_user_id(&user);
    progress_repo::mark_completed(&state.db.write, &media_id, user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
