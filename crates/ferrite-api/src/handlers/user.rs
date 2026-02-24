use crate::auth::AuthUser;
use crate::error::ApiError;
use crate::state::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use ferrite_db::{preference_repo, user_repo};
use serde::{Deserialize, Serialize};

/// POST /api/users — create a new user (admin only, or first-user setup)
pub async fn create_user(
    State(state): State<AppState>,
    auth_user: Option<Extension<AuthUser>>,
    Json(req): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let user_count = user_repo::count_users(&state.db.read).await?;

    // If users already exist, only admins can create new users
    if user_count > 0 {
        let caller = auth_user
            .as_ref()
            .ok_or_else(|| ApiError::unauthorized("Authentication required"))?;

        let caller_user = user_repo::get_user_by_id(&state.db.read, &caller.user_id)
            .await?
            .ok_or_else(|| ApiError::unauthorized("User not found"))?;

        if caller_user.is_admin == 0 {
            return Err(ApiError::forbidden("Only admins can create users"));
        }
    }

    let user = user_repo::create_user(
        &state.db.write,
        &req.username,
        req.display_name.as_deref(),
        &req.password,
        req.is_admin.unwrap_or(user_count == 0), // First user is always admin
    )
    .await?;

    // Update in-memory cache
    state.user_cache.insert(user.id.clone());

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
        let caller_user = user_repo::get_user_by_id(&state.db.read, &caller.user_id)
            .await?
            .ok_or_else(|| ApiError::unauthorized("User not found"))?;

        if caller_user.is_admin == 0 {
            return Err(ApiError::forbidden("Only admins can list users"));
        }
    }

    let users = user_repo::list_users(&state.db.read).await?;
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

    let user = user_repo::get_user_by_id(&state.db.read, &caller.user_id)
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
    let user = user_repo::get_user_by_id(&state.db.read, &caller.user_id)
        .await?
        .ok_or_else(|| ApiError::not_found("User not found"))?;

    if !user_repo::verify_password(&req.current_password, &user.password_hash) {
        return Err(ApiError::unauthorized("Current password is incorrect"));
    }

    user_repo::change_password(&state.db.write, &caller.user_id, &req.new_password).await?;

    Ok(Json(
        serde_json::json!({ "message": "Password changed successfully" }),
    ))
}

#[derive(Deserialize)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

/// DELETE /api/users/{id} — delete a user (admin only, cannot delete self)
pub async fn delete_user(
    State(state): State<AppState>,
    auth_user: Option<Extension<AuthUser>>,
    axum::extract::Path(target_id): axum::extract::Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let caller = auth_user
        .as_ref()
        .ok_or_else(|| ApiError::unauthorized("Authentication required"))?;

    let caller_user = user_repo::get_user_by_id(&state.db.read, &caller.user_id)
        .await?
        .ok_or_else(|| ApiError::unauthorized("User not found"))?;

    if caller_user.is_admin == 0 {
        return Err(ApiError::forbidden("Only admins can delete users"));
    }

    if caller.user_id == target_id {
        return Err(ApiError::bad_request("Cannot delete your own account"));
    }

    user_repo::delete_user(&state.db.write, &target_id).await?;

    // Update in-memory cache
    state.user_cache.remove(&target_id);

    Ok(axum::http::StatusCode::NO_CONTENT)
}

/// PUT /api/users/{id}/password — admin resets another user's password
pub async fn admin_reset_password(
    State(state): State<AppState>,
    auth_user: Option<Extension<AuthUser>>,
    axum::extract::Path(target_id): axum::extract::Path<String>,
    Json(req): Json<AdminResetPasswordRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let caller = auth_user
        .as_ref()
        .ok_or_else(|| ApiError::unauthorized("Authentication required"))?;

    let caller_user = user_repo::get_user_by_id(&state.db.read, &caller.user_id)
        .await?
        .ok_or_else(|| ApiError::unauthorized("User not found"))?;

    if caller_user.is_admin == 0 {
        return Err(ApiError::forbidden("Only admins can reset passwords"));
    }

    user_repo::change_password(&state.db.write, &target_id, &req.new_password).await?;
    Ok(Json(
        serde_json::json!({ "message": "Password reset successfully" }),
    ))
}

#[derive(Deserialize)]
pub struct AdminResetPasswordRequest {
    pub new_password: String,
}

/// GET /api/users/setup — check if initial setup is needed (no users exist)
pub async fn setup_status(State(state): State<AppState>) -> Result<impl IntoResponse, ApiError> {
    let count = user_repo::count_users(&state.db.read).await?;
    Ok(Json(serde_json::json!({
        "setup_required": count == 0,
        "user_count": count,
    })))
}

/// GET /api/preferences — get all preferences for the current user.
/// Returns a flat JSON object of key→value strings.
pub async fn get_preferences(
    State(state): State<AppState>,
    auth_user: Option<Extension<AuthUser>>,
) -> Result<impl IntoResponse, ApiError> {
    let user_id = match auth_user.as_ref() {
        Some(u) => u.user_id.clone(),
        None => return Ok(Json(serde_json::json!({}))),
    };
    let pairs = preference_repo::get_all_preferences(&state.db.read, &user_id).await?;
    let map: serde_json::Map<String, serde_json::Value> = pairs
        .into_iter()
        .map(|(k, v)| (k, serde_json::Value::String(v)))
        .collect();
    Ok(Json(serde_json::Value::Object(map)))
}

#[derive(Deserialize, Serialize)]
pub struct SetPreferencesRequest {
    pub preferences: std::collections::HashMap<String, String>,
}

/// PUT /api/preferences — upsert one or more preferences for the current user.
pub async fn set_preferences(
    State(state): State<AppState>,
    auth_user: Option<Extension<AuthUser>>,
    Json(req): Json<SetPreferencesRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let user_id = match auth_user.as_ref() {
        Some(u) => u.user_id.clone(),
        None => return Ok(axum::http::StatusCode::NO_CONTENT),
    };
    for (key, value) in &req.preferences {
        preference_repo::set_preference(&state.db.write, &user_id, key, value).await?;
    }
    Ok(axum::http::StatusCode::NO_CONTENT)
}
