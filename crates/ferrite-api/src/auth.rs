use axum::extract::{Request, State};
use axum::http::{header, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::Json;
use chrono::{Duration, Utc};
use ferrite_db::user_repo;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use subtle::ConstantTimeEq;

use crate::state::AppState;

/// Authenticated user info extracted by the auth middleware.
/// Downstream handlers can access this via `Extension<AuthUser>`.
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: String,
    pub username: String,
}

// ---------------------------------------------------------------------------
// JWT Claims
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// User ID (UUID string)
    pub sub: String,
    /// Username (for display / convenience)
    pub username: String,
    pub exp: usize,
    pub iat: usize,
}

fn create_token(
    user_id: &str,
    username: &str,
    jwt_secret: &str,
    expiry_days: u64,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let claims = Claims {
        sub: user_id.to_string(),
        username: username.to_string(),
        exp: (now + Duration::days(expiry_days as i64)).timestamp() as usize,
        iat: now.timestamp() as usize,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
}

fn validate_token(
    token: &str,
    jwt_secret: &str,
) -> Result<Claims, jsonwebtoken::errors::Error> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(data.claims)
}

// ---------------------------------------------------------------------------
// Auth Middleware
// ---------------------------------------------------------------------------

pub async fn require_auth(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Response {
    let auth_config = match &state.config.auth {
        Some(c) => c,
        None => return next.run(request).await,
    };

    // Try to extract AuthUser from various auth methods.
    // If successful, insert into request extensions so handlers can access it.

    // 1. Bearer token
    if let Some(val) = request.headers().get(header::AUTHORIZATION) {
        if let Ok(s) = val.to_str() {
            if let Some(token) = s.strip_prefix("Bearer ") {
                if let Ok(claims) = validate_token(token, &auth_config.jwt_secret) {
                    // Verify the user still exists in the DB (guards against stale
                    // JWTs after a DB reset where the user_id is gone).
                    if user_repo::get_user_by_id(&state.db, &claims.sub)
                        .await
                        .ok()
                        .flatten()
                        .is_some()
                    {
                        request.extensions_mut().insert(AuthUser {
                            user_id: claims.sub,
                            username: claims.username,
                        });
                        return next.run(request).await;
                    }
                }
            }
        }
    }

    // 2. X-API-Key header (constant-time comparison to prevent timing attacks)
    if let Some(val) = request.headers().get("X-API-Key") {
        if let Ok(key) = val.to_str() {
            if api_key_matches(key, &auth_config.api_keys) {
                return next.run(request).await;
            }
        }
    }

    // 3. ?token= query parameter (for <video>/<img> src that can't set headers)
    if let Some(query) = request.uri().query() {
        for pair in query.split('&') {
            if let Some(token) = pair.strip_prefix("token=") {
                let decoded = percent_decode(token);
                if let Ok(claims) = validate_token(&decoded, &auth_config.jwt_secret) {
                    if user_repo::get_user_by_id(&state.db, &claims.sub)
                        .await
                        .ok()
                        .flatten()
                        .is_some()
                    {
                        request.extensions_mut().insert(AuthUser {
                            user_id: claims.sub,
                            username: claims.username,
                        });
                        return next.run(request).await;
                    }
                }
            }
        }
    }

    // 4. ?api_key= query parameter (constant-time comparison)
    if let Some(query) = request.uri().query() {
        for pair in query.split('&') {
            if let Some(key) = pair.strip_prefix("api_key=") {
                let decoded = percent_decode(key);
                if api_key_matches(&decoded, &auth_config.api_keys) {
                    return next.run(request).await;
                }
            }
        }
    }

    (
        StatusCode::UNAUTHORIZED,
        Json(serde_json::json!({ "error": "Unauthorized" })),
    )
        .into_response()
}

/// Minimal percent-decoding for API key query values.
fn percent_decode(input: &str) -> String {
    percent_encoding::percent_decode_str(input)
        .decode_utf8_lossy()
        .into_owned()
}

/// Constant-time comparison of an API key against a list of valid keys.
/// Prevents timing attacks by always comparing against all keys in constant time.
fn api_key_matches(candidate: &str, valid_keys: &[String]) -> bool {
    let candidate_bytes = candidate.as_bytes();
    let mut found = false;
    for key in valid_keys {
        let key_bytes = key.as_bytes();
        // Only compare if lengths match (length itself leaks, but that's acceptable
        // since API keys should all be the same length in practice)
        if candidate_bytes.len() == key_bytes.len()
            && candidate_bytes.ct_eq(key_bytes).into()
        {
            found = true;
        }
    }
    found
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub expires_in_days: u64,
}

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> impl IntoResponse {
    // Rate limit login attempts to prevent brute-force attacks
    if state.login_limiter.check().is_err() {
        tracing::warn!("Login rate limit exceeded");
        return (
            StatusCode::TOO_MANY_REQUESTS,
            Json(serde_json::json!({ "error": "Too many login attempts, please try again later" })),
        )
            .into_response();
    }

    let auth_config = match &state.config.auth {
        Some(c) => c,
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({ "error": "Authentication is not configured" })),
            )
                .into_response();
        }
    };

    // Look up user in the database
    let user = match user_repo::get_user_by_username(&state.db, &req.username).await {
        Ok(Some(u)) => u,
        Ok(None) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({ "error": "Invalid credentials" })),
            )
                .into_response();
        }
        Err(e) => {
            tracing::error!("Database error during login: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Internal server error" })),
            )
                .into_response();
        }
    };

    // Verify password against stored bcrypt hash
    if !user_repo::verify_password(&req.password, &user.password_hash) {
        return (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({ "error": "Invalid credentials" })),
        )
            .into_response();
    }

    // Update last login timestamp (fire-and-forget)
    let _ = user_repo::update_last_login(&state.db, &user.id).await;

    match create_token(
        &user.id,
        &user.username,
        &auth_config.jwt_secret,
        auth_config.token_expiry_days,
    ) {
        Ok(token) => Json(LoginResponse {
            token,
            expires_in_days: auth_config.token_expiry_days,
        })
        .into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": "Failed to create token" })),
        )
            .into_response(),
    }
}

pub async fn auth_status(State(state): State<AppState>) -> impl IntoResponse {
    let user_count = ferrite_db::user_repo::count_users(&state.db).await.unwrap_or(0);
    Json(serde_json::json!({
        "auth_required": state.config.auth.is_some(),
        "has_users": user_count > 0,
    }))
}
