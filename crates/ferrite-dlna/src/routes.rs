use crate::content_directory;
use crate::description;
use axum::body::Body;
use axum::extract::State;
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::Router;
use sqlx::SqlitePool;
use std::sync::Arc;
use tracing::{debug, warn};

/// Shared state for DLNA HTTP routes.
#[derive(Clone)]
pub struct DlnaState {
    pub db: SqlitePool,
    pub server_uuid: String,
    pub friendly_name: String,
    pub http_base_url: String,
}

/// Build the Axum router for DLNA HTTP endpoints.
/// These routes should be nested under the main server router.
pub fn build_dlna_router(state: DlnaState) -> Router {
    Router::new()
        // Device description
        .route("/dlna/device.xml", get(device_description))
        // SCPD documents
        .route(
            "/dlna/scpd/content-directory.xml",
            get(content_directory_scpd),
        )
        .route(
            "/dlna/scpd/connection-manager.xml",
            get(connection_manager_scpd),
        )
        // SOAP control endpoints
        .route(
            "/dlna/control/content-directory",
            post(content_directory_control),
        )
        .route(
            "/dlna/control/connection-manager",
            post(connection_manager_control),
        )
        // Event subscription endpoints (minimal — just accept and return OK)
        .route(
            "/dlna/event/content-directory",
            get(event_stub).post(event_stub),
        )
        .route(
            "/dlna/event/connection-manager",
            get(event_stub).post(event_stub),
        )
        .with_state(Arc::new(state))
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

async fn device_description(State(state): State<Arc<DlnaState>>) -> impl IntoResponse {
    let xml = description::device_description_xml(
        &state.server_uuid,
        &state.friendly_name,
        &state.http_base_url,
    );
    xml_response(xml)
}

async fn content_directory_scpd() -> impl IntoResponse {
    xml_response(description::content_directory_scpd().to_string())
}

async fn connection_manager_scpd() -> impl IntoResponse {
    xml_response(description::connection_manager_scpd().to_string())
}

async fn content_directory_control(
    State(state): State<Arc<DlnaState>>,
    body: String,
) -> impl IntoResponse {
    debug!("DLNA ContentDirectory SOAP request");

    // Determine which action is being called from the SOAP body
    if body.contains("Browse") {
        if let Some(req) = content_directory::parse_browse_request(&body) {
            match content_directory::handle_browse(&state.db, &req, &state.http_base_url).await {
                Ok(response) => return soap_response(response),
                Err(e) => {
                    warn!("DLNA Browse error: {}", e);
                    return soap_fault("Browse failed");
                }
            }
        }
        return soap_fault("Invalid Browse request");
    }

    if body.contains("GetSystemUpdateID") {
        return soap_response(content_directory::handle_get_system_update_id());
    }

    if body.contains("GetSearchCapabilities") {
        return soap_response(content_directory::handle_get_search_capabilities());
    }

    if body.contains("GetSortCapabilities") {
        return soap_response(content_directory::handle_get_sort_capabilities());
    }

    soap_fault("Unknown action")
}

async fn connection_manager_control(body: String) -> impl IntoResponse {
    debug!("DLNA ConnectionManager SOAP request");

    if body.contains("GetProtocolInfo") {
        return soap_response(content_directory::handle_get_protocol_info());
    }

    if body.contains("GetCurrentConnectionIDs") {
        return soap_response(content_directory::handle_get_current_connection_ids());
    }

    if body.contains("GetCurrentConnectionInfo") {
        return soap_response(content_directory::handle_get_current_connection_info());
    }

    soap_fault("Unknown action")
}

async fn event_stub() -> impl IntoResponse {
    StatusCode::OK
}

// ---------------------------------------------------------------------------
// Response helpers
// ---------------------------------------------------------------------------

fn xml_response(body: String) -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/xml; charset=utf-8")
        .body(Body::from(body))
        .unwrap()
}

fn soap_response(body: String) -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/xml; charset=utf-8")
        .header("EXT", "")
        .body(Body::from(body))
        .unwrap()
}

fn soap_fault(message: &str) -> Response {
    let body = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<s:Envelope xmlns:s="http://schemas.xmlsoap.org/soap/envelope/" s:encodingStyle="http://schemas.xmlsoap.org/soap/encoding/">
<s:Body>
<s:Fault>
<faultcode>s:Client</faultcode>
<faultstring>{}</faultstring>
</s:Fault>
</s:Body>
</s:Envelope>"#,
        quick_xml::escape::escape(message)
    );

    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .header(header::CONTENT_TYPE, "text/xml; charset=utf-8")
        .body(Body::from(body))
        .unwrap()
}
