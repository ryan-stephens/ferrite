use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

/// API-layer error type that implements `IntoResponse` for Axum handlers.
/// Handlers can return `Result<impl IntoResponse, ApiError>` and use `?` freely.
#[derive(Debug)]
pub struct ApiError {
    status: StatusCode,
    message: String,
}

impl ApiError {
    pub fn not_found(msg: impl Into<String>) -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            message: msg.into(),
        }
    }

    pub fn unauthorized(msg: impl Into<String>) -> Self {
        Self {
            status: StatusCode::UNAUTHORIZED,
            message: msg.into(),
        }
    }

    pub fn forbidden(msg: impl Into<String>) -> Self {
        Self {
            status: StatusCode::FORBIDDEN,
            message: msg.into(),
        }
    }

    pub fn bad_request(msg: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            message: msg.into(),
        }
    }

    pub fn internal(msg: impl Into<String>) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: msg.into(),
        }
    }

    pub fn service_unavailable(msg: impl Into<String>) -> Self {
        Self {
            status: StatusCode::SERVICE_UNAVAILABLE,
            message: msg.into(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = serde_json::json!({ "error": self.message });
        (self.status, Json(body)).into_response()
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.status, self.message)
    }
}

// ---------------------------------------------------------------------------
// From impls — allow `?` in handlers for common error types
// ---------------------------------------------------------------------------

impl From<sqlx::Error> for ApiError {
    fn from(e: sqlx::Error) -> Self {
        tracing::error!("Database error: {e}");
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Internal database error".to_string(),
        }
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(e: anyhow::Error) -> Self {
        tracing::error!("Internal error: {e}");
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Internal server error".to_string(),
        }
    }
}

impl From<std::io::Error> for ApiError {
    fn from(e: std::io::Error) -> Self {
        tracing::error!("IO error: {e}");
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Internal IO error".to_string(),
        }
    }
}

impl From<ferrite_core::error::FerriteError> for ApiError {
    fn from(e: ferrite_core::error::FerriteError) -> Self {
        match &e {
            ferrite_core::error::FerriteError::NotFound(msg) => Self::not_found(msg.clone()),
            ferrite_core::error::FerriteError::Config(msg) => Self::bad_request(msg.clone()),
            other => {
                tracing::error!("Application error: {other}");
                Self::internal("Internal server error")
            }
        }
    }
}
