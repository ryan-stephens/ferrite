use crate::error::ApiError;
use crate::state::AppState;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use ferrite_db::collection_repo;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateCollectionRequest {
    pub name: String,
    #[serde(default)]
    pub description: String,
    /// "collection" (default) or "playlist"
    #[serde(default = "default_kind")]
    pub kind: String,
}

fn default_kind() -> String {
    "collection".into()
}

#[derive(Deserialize, Default)]
pub struct ListCollectionsQuery {
    /// Filter by kind: "collection" or "playlist"
    pub kind: Option<String>,
}

#[derive(Deserialize)]
pub struct AddItemRequest {
    pub media_id: String,
}

#[derive(Deserialize)]
pub struct ReorderRequest {
    pub media_id: String,
    pub position: i64,
}

#[derive(Deserialize)]
pub struct UpdateCollectionRequest {
    pub name: String,
    #[serde(default)]
    pub description: String,
}

/// POST /api/collections — Create a new collection or playlist.
pub async fn create_collection(
    State(state): State<AppState>,
    Json(body): Json<CreateCollectionRequest>,
) -> Result<impl IntoResponse, ApiError> {
    if body.name.trim().is_empty() {
        return Err(ApiError::bad_request("Collection name cannot be empty"));
    }

    if body.kind != "collection" && body.kind != "playlist" {
        return Err(ApiError::bad_request(
            "Kind must be 'collection' or 'playlist'",
        ));
    }

    // Use a default user ID for now (multi-user support uses auth middleware to extract user)
    let user_id = extract_user_id(&state).await;

    let collection = collection_repo::create_collection(
        &state.db.write,
        &user_id,
        body.name.trim(),
        body.description.trim(),
        &body.kind,
    )
    .await
    .map_err(|e| ApiError::internal(format!("Failed to create collection: {}", e)))?;

    Ok((StatusCode::CREATED, Json(collection)))
}

/// GET /api/collections — List all collections for the current user.
pub async fn list_collections(
    State(state): State<AppState>,
    Query(query): Query<ListCollectionsQuery>,
) -> Result<impl IntoResponse, ApiError> {
    let user_id = extract_user_id(&state).await;

    let collections =
        collection_repo::list_collections(&state.db.read, &user_id, query.kind.as_deref())
            .await
            .map_err(|e| ApiError::internal(format!("Failed to list collections: {}", e)))?;

    // Enrich with item counts
    let mut result = Vec::with_capacity(collections.len());
    for c in collections {
        let count = collection_repo::count_items(&state.db.read, &c.id)
            .await
            .unwrap_or(0);
        result.push(serde_json::json!({
            "id": c.id,
            "user_id": c.user_id,
            "name": c.name,
            "description": c.description,
            "kind": c.kind,
            "item_count": count,
            "created_at": c.created_at,
            "updated_at": c.updated_at,
        }));
    }

    Ok(Json(result))
}

/// GET /api/collections/{id} — Get a collection with its items.
pub async fn get_collection(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let collection = collection_repo::get_collection(&state.db.read, &id)
        .await?
        .ok_or_else(|| ApiError::not_found(format!("Collection '{id}' not found")))?;

    let items = collection_repo::list_items(&state.db.read, &id)
        .await
        .map_err(|e| ApiError::internal(format!("Failed to list items: {}", e)))?;

    Ok(Json(serde_json::json!({
        "id": collection.id,
        "user_id": collection.user_id,
        "name": collection.name,
        "description": collection.description,
        "kind": collection.kind,
        "item_count": items.len(),
        "items": items,
        "created_at": collection.created_at,
        "updated_at": collection.updated_at,
    })))
}

/// PUT /api/collections/{id} — Update a collection's name and description.
pub async fn update_collection(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(body): Json<UpdateCollectionRequest>,
) -> Result<impl IntoResponse, ApiError> {
    if body.name.trim().is_empty() {
        return Err(ApiError::bad_request("Collection name cannot be empty"));
    }

    let updated = collection_repo::update_collection(
        &state.db.write,
        &id,
        body.name.trim(),
        body.description.trim(),
    )
    .await
    .map_err(|e| ApiError::internal(format!("Failed to update collection: {}", e)))?
    .ok_or_else(|| ApiError::not_found(format!("Collection '{id}' not found")))?;

    Ok(Json(updated))
}

/// DELETE /api/collections/{id} — Delete a collection.
pub async fn delete_collection(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let deleted = collection_repo::delete_collection(&state.db.write, &id)
        .await
        .map_err(|e| ApiError::internal(format!("Failed to delete collection: {}", e)))?;

    if !deleted {
        return Err(ApiError::not_found(format!("Collection '{id}' not found")));
    }

    Ok(StatusCode::NO_CONTENT)
}

/// POST /api/collections/{id}/items — Add a media item to a collection.
pub async fn add_item(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(body): Json<AddItemRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // Verify collection exists
    collection_repo::get_collection(&state.db.read, &id)
        .await?
        .ok_or_else(|| ApiError::not_found(format!("Collection '{id}' not found")))?;

    let item = collection_repo::add_item(&state.db.write, &id, &body.media_id)
        .await
        .map_err(|e| {
            if e.to_string().contains("UNIQUE constraint") {
                ApiError::bad_request("Item already in collection")
            } else {
                ApiError::internal(format!("Failed to add item: {}", e))
            }
        })?;

    Ok((StatusCode::CREATED, Json(item)))
}

/// DELETE /api/collections/{collection_id}/items/{media_id} — Remove an item.
pub async fn remove_item(
    State(state): State<AppState>,
    Path((collection_id, media_id)): Path<(String, String)>,
) -> Result<impl IntoResponse, ApiError> {
    let removed = collection_repo::remove_item(&state.db.write, &collection_id, &media_id)
        .await
        .map_err(|e| ApiError::internal(format!("Failed to remove item: {}", e)))?;

    if !removed {
        return Err(ApiError::not_found("Item not found in collection"));
    }

    Ok(StatusCode::NO_CONTENT)
}

/// PUT /api/collections/{id}/reorder — Reorder an item in a playlist.
pub async fn reorder_item(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(body): Json<ReorderRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let collection = collection_repo::get_collection(&state.db.read, &id)
        .await?
        .ok_or_else(|| ApiError::not_found(format!("Collection '{id}' not found")))?;

    if collection.kind != "playlist" {
        return Err(ApiError::bad_request(
            "Reordering is only supported for playlists",
        ));
    }

    let reordered =
        collection_repo::reorder_item(&state.db.write, &id, &body.media_id, body.position)
            .await
            .map_err(|e| ApiError::internal(format!("Failed to reorder item: {}", e)))?;

    if !reordered {
        return Err(ApiError::not_found("Item not found in playlist"));
    }

    // Return updated item list
    let items = collection_repo::list_items(&state.db.read, &id)
        .await
        .map_err(|e| ApiError::internal(format!("Failed to list items: {}", e)))?;

    Ok(Json(items))
}

/// Extract user ID from the first user in the database (simplified).
/// In a full implementation, this would come from the auth middleware.
async fn extract_user_id(state: &AppState) -> String {
    // Try to get the first user; fall back to a default ID
    let result: Option<(String,)> = sqlx::query_as("SELECT id FROM users LIMIT 1")
        .fetch_optional(&state.db.read)
        .await
        .ok()
        .flatten();

    result
        .map(|(id,)| id)
        .unwrap_or_else(|| "default-user".into())
}
