use crate::auth::AuthUser;
use crate::error::ApiError;
use crate::state::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use ferrite_db::user_repo;
use serde::Deserialize;

/// POST /api/users — create a new user (admin only, or first-user setup)
pub async fn create_user(
    State(state): State<AppState>,
    auth_user: Option<Extension<AuthUser>>,
    Json(req): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let user_count = user_repo::count_users(&state.db).await?;

    // If users already exist, only admins can create new users
    if user_count > 0 {
        let caller = auth_user
            .as_ref()
            .ok_or_else(|| ApiError::unauthorized("Authentication required"))?;

        let caller_user = user_repo::get_user_by_id(&state.db, &caller.user_id)
            .await?
            .ok_or_else(|| ApiError::unauthorized("User not found"))?;

        if caller_user.is_admin == 0 {
            return Err(ApiError::forbidden("Only admins can create users"));
        }
    }

    let user = user_repo::create_user(
        &state.db,
        &req.username,
        req.display_name.as_deref(),
        &req.password,
        req.is_admin.unwrap_or(user_count == 0), // First user is always admin
    )
    .await?;

    Ok(Json(user))
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
    pub display_name: Option<String>,
    pub is_admin: Option<bool>,
}

/// GET /api/users — list all users (admin only)
pub async fn list_users(
    State(state): State<AppState>,
    auth_user: Option<Extension<AuthUser>>,
) -> Result<impl IntoResponse, ApiError> {
    // Require admin if auth is configured
    if let Some(caller) = auth_user.as_ref() {
        let caller_user = user_repo::get_user_by_id(&state.db, &caller.user_id)
            .await?
            .ok_or_else(|| ApiError::unauthorized("User not found"))?;

        if caller_user.is_admin == 0 {
            return Err(ApiError::forbidden("Only admins can list users"));
        }
    }

    let users = user_repo::list_users(&state.db).await?;
    Ok(Json(users))
}

/// GET /api/users/me — get the current authenticated user
pub async fn get_current_user(
    State(state): State<AppState>,
    auth_user: Option<Extension<AuthUser>>,
) -> Result<impl IntoResponse, ApiError> {
    let caller = auth_user
        .as_ref()
        .ok_or_else(|| ApiError::unauthorized("Authentication required"))?;

    let user = user_repo::get_user_by_id(&state.db, &caller.user_id)
        .await?
        .ok_or_else(|| ApiError::not_found("User not found"))?;

    Ok(Json(user))
}

/// PUT /api/users/me/password — change the current user's password
pub async fn change_password(
    State(state): State<AppState>,
    auth_user: Option<Extension<AuthUser>>,
    Json(req): Json<ChangePasswordRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let caller = auth_user
        .as_ref()
        .ok_or_else(|| ApiError::unauthorized("Authentication required"))?;

    // Verify current password
    let user = user_repo::get_user_by_id(&state.db, &caller.user_id)
        .await?
        .ok_or_else(|| ApiError::not_found("User not found"))?;

    if !user_repo::verify_password(&req.current_password, &user.password_hash) {
        return Err(ApiError::unauthorized("Current password is incorrect"));
    }

    user_repo::change_password(&state.db, &caller.user_id, &req.new_password).await?;

    Ok(Json(serde_json::json!({ "message": "Password changed successfully" })))
}

#[derive(Deserialize)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

/// GET /api/users/setup — check if initial setup is needed (no users exist)
pub async fn setup_status(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    let count = user_repo::count_users(&state.db).await?;
    Ok(Json(serde_json::json!({
        "setup_required": count == 0,
        "user_count": count,
    })))
}
