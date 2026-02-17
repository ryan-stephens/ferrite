use crate::error::ApiError;
use crate::state::AppState;
use crate::webhooks::EventType;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use ferrite_db::webhook_repo;
use serde::Deserialize;
use tracing::warn;

#[derive(Deserialize)]
pub struct CreateWebhookRequest {
    pub name: String,
    pub url: String,
    pub secret: Option<String>,
    /// Comma-separated event types or '*' for all. Defaults to '*'.
    #[serde(default = "default_events")]
    pub events: String,
}

fn default_events() -> String {
    "*".into()
}

#[derive(Deserialize)]
pub struct UpdateWebhookRequest {
    pub name: String,
    pub url: String,
    pub secret: Option<String>,
    pub events: String,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

fn default_enabled() -> bool {
    true
}

/// POST /api/webhooks — Create a new webhook.
pub async fn create_webhook(
    State(state): State<AppState>,
    Json(body): Json<CreateWebhookRequest>,
) -> Result<impl IntoResponse, ApiError> {
    if body.name.trim().is_empty() {
        return Err(ApiError::bad_request("Webhook name cannot be empty"));
    }

    if body.url.trim().is_empty() {
        return Err(ApiError::bad_request("Webhook URL cannot be empty"));
    }

    if !body.url.starts_with("http://") && !body.url.starts_with("https://") {
        return Err(ApiError::bad_request("Webhook URL must start with http:// or https://"));
    }

    let user_id = extract_user_id(&state).await;

    let webhook = webhook_repo::create_webhook(
        &state.db,
        &user_id,
        body.name.trim(),
        body.url.trim(),
        body.secret.as_deref(),
        &body.events,
    )
    .await
    .map_err(|e| ApiError::internal(format!("Failed to create webhook: {}", e)))?;

    Ok((StatusCode::CREATED, Json(webhook)))
}

/// GET /api/webhooks — List all webhooks for the current user.
pub async fn list_webhooks(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    let user_id = extract_user_id(&state).await;

    let webhooks = webhook_repo::list_webhooks(&state.db, &user_id)
        .await
        .map_err(|e| ApiError::internal(format!("Failed to list webhooks: {}", e)))?;

    Ok(Json(webhooks))
}

/// GET /api/webhooks/{id} — Get a single webhook.
pub async fn get_webhook(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let webhook = webhook_repo::get_webhook(&state.db, &id)
        .await?
        .ok_or_else(|| ApiError::not_found(format!("Webhook '{id}' not found")))?;

    Ok(Json(webhook))
}

/// PUT /api/webhooks/{id} — Update a webhook.
pub async fn update_webhook(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(body): Json<UpdateWebhookRequest>,
) -> Result<impl IntoResponse, ApiError> {
    if body.name.trim().is_empty() {
        return Err(ApiError::bad_request("Webhook name cannot be empty"));
    }

    if !body.url.starts_with("http://") && !body.url.starts_with("https://") {
        return Err(ApiError::bad_request("Webhook URL must start with http:// or https://"));
    }

    let updated = webhook_repo::update_webhook(
        &state.db,
        &id,
        body.name.trim(),
        body.url.trim(),
        body.secret.as_deref(),
        &body.events,
        body.enabled,
    )
    .await
    .map_err(|e| ApiError::internal(format!("Failed to update webhook: {}", e)))?
    .ok_or_else(|| ApiError::not_found(format!("Webhook '{id}' not found")))?;

    Ok(Json(updated))
}

/// DELETE /api/webhooks/{id} — Delete a webhook.
pub async fn delete_webhook(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let deleted = webhook_repo::delete_webhook(&state.db, &id)
        .await
        .map_err(|e| ApiError::internal(format!("Failed to delete webhook: {}", e)))?;

    if !deleted {
        return Err(ApiError::not_found(format!("Webhook '{id}' not found")));
    }

    Ok(StatusCode::NO_CONTENT)
}

/// POST /api/webhooks/{id}/test — Send a test ping to a webhook.
pub async fn test_webhook(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let status = state
        .webhook_dispatcher
        .send_test_ping(&id)
        .await
        .map_err(|e| {
            warn!("Test ping failed for webhook {}: {}", id, e);
            ApiError::internal(format!("Test ping failed: {}", e))
        })?;

    Ok(Json(serde_json::json!({
        "status_code": status,
        "success": (200..300).contains(&(status as i32)),
    })))
}

/// GET /api/webhooks/events — List all supported event types.
pub async fn list_event_types() -> impl IntoResponse {
    let events: Vec<&str> = EventType::all().iter().map(|e| e.as_str()).collect();
    Json(serde_json::json!({ "events": events }))
}

/// Extract user ID (simplified — in production, from auth middleware).
async fn extract_user_id(state: &AppState) -> String {
    let result: Option<(String,)> = sqlx::query_as("SELECT id FROM users LIMIT 1")
        .fetch_optional(&state.db)
        .await
        .ok()
        .flatten();

    result
        .map(|(id,)| id)
        .unwrap_or_else(|| "default-user".into())
}
