use anyhow::Result;
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
pub struct CollectionRow {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub description: String,
    pub kind: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
pub struct CollectionItemRow {
    pub id: String,
    pub collection_id: String,
    pub media_id: String,
    pub position: Option<i64>,
    pub added_at: String,
}

/// Create a new collection or playlist.
pub async fn create_collection(
    pool: &SqlitePool,
    user_id: &str,
    name: &str,
    description: &str,
    kind: &str,
) -> Result<CollectionRow> {
    let id = Uuid::new_v4().to_string();
    let row = sqlx::query_as::<_, CollectionRow>(
        "INSERT INTO collections (id, user_id, name, description, kind) VALUES (?, ?, ?, ?, ?) RETURNING *",
    )
    .bind(&id)
    .bind(user_id)
    .bind(name)
    .bind(description)
    .bind(kind)
    .fetch_one(pool)
    .await?;
    Ok(row)
}

/// List all collections for a user.
pub async fn list_collections(
    pool: &SqlitePool,
    user_id: &str,
    kind: Option<&str>,
) -> Result<Vec<CollectionRow>> {
    let rows = if let Some(k) = kind {
        sqlx::query_as::<_, CollectionRow>(
            "SELECT * FROM collections WHERE user_id = ? AND kind = ? ORDER BY updated_at DESC",
        )
        .bind(user_id)
        .bind(k)
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query_as::<_, CollectionRow>(
            "SELECT * FROM collections WHERE user_id = ? ORDER BY updated_at DESC",
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?
    };
    Ok(rows)
}

/// Get a single collection by ID.
pub async fn get_collection(pool: &SqlitePool, id: &str) -> Result<Option<CollectionRow>> {
    let row = sqlx::query_as::<_, CollectionRow>("SELECT * FROM collections WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?;
    Ok(row)
}

/// Update a collection's name and/or description.
pub async fn update_collection(
    pool: &SqlitePool,
    id: &str,
    name: &str,
    description: &str,
) -> Result<Option<CollectionRow>> {
    let row = sqlx::query_as::<_, CollectionRow>(
        "UPDATE collections SET name = ?, description = ?, updated_at = datetime('now') WHERE id = ? RETURNING *",
    )
    .bind(name)
    .bind(description)
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

/// Delete a collection and all its items (CASCADE).
pub async fn delete_collection(pool: &SqlitePool, id: &str) -> Result<bool> {
    let result = sqlx::query("DELETE FROM collections WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}

/// Add a media item to a collection. For playlists, appends at the end.
pub async fn add_item(
    pool: &SqlitePool,
    collection_id: &str,
    media_id: &str,
) -> Result<CollectionItemRow> {
    let id = Uuid::new_v4().to_string();

    // For playlists, compute the next position
    let next_pos: (i64,) = sqlx::query_as(
        "SELECT COALESCE(MAX(position), -1) + 1 FROM collection_items WHERE collection_id = ?",
    )
    .bind(collection_id)
    .fetch_one(pool)
    .await?;

    let row = sqlx::query_as::<_, CollectionItemRow>(
        "INSERT INTO collection_items (id, collection_id, media_id, position) VALUES (?, ?, ?, ?) RETURNING *",
    )
    .bind(&id)
    .bind(collection_id)
    .bind(media_id)
    .bind(next_pos.0)
    .fetch_one(pool)
    .await?;

    // Touch the collection's updated_at
    sqlx::query("UPDATE collections SET updated_at = datetime('now') WHERE id = ?")
        .bind(collection_id)
        .execute(pool)
        .await?;

    Ok(row)
}

/// Remove a media item from a collection.
pub async fn remove_item(pool: &SqlitePool, collection_id: &str, media_id: &str) -> Result<bool> {
    let result =
        sqlx::query("DELETE FROM collection_items WHERE collection_id = ? AND media_id = ?")
            .bind(collection_id)
            .bind(media_id)
            .execute(pool)
            .await?;

    if result.rows_affected() > 0 {
        sqlx::query("UPDATE collections SET updated_at = datetime('now') WHERE id = ?")
            .bind(collection_id)
            .execute(pool)
            .await?;
    }

    Ok(result.rows_affected() > 0)
}

/// List items in a collection, ordered by position for playlists.
pub async fn list_items(pool: &SqlitePool, collection_id: &str) -> Result<Vec<CollectionItemRow>> {
    let rows = sqlx::query_as::<_, CollectionItemRow>(
        "SELECT * FROM collection_items WHERE collection_id = ? ORDER BY position ASC, added_at ASC",
    )
    .bind(collection_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

/// Reorder an item within a playlist to a new position.
/// Shifts other items to make room.
pub async fn reorder_item(
    pool: &SqlitePool,
    collection_id: &str,
    media_id: &str,
    new_position: i64,
) -> Result<bool> {
    // Get current position
    let current: Option<(i64,)> = sqlx::query_as(
        "SELECT position FROM collection_items WHERE collection_id = ? AND media_id = ?",
    )
    .bind(collection_id)
    .bind(media_id)
    .fetch_optional(pool)
    .await?;

    let current_pos = match current {
        Some((pos,)) => pos,
        None => return Ok(false),
    };

    if current_pos == new_position {
        return Ok(true);
    }

    if new_position < current_pos {
        // Moving up: shift items in [new_position, current_pos) down by 1
        sqlx::query(
            "UPDATE collection_items SET position = position + 1 \
             WHERE collection_id = ? AND position >= ? AND position < ?",
        )
        .bind(collection_id)
        .bind(new_position)
        .bind(current_pos)
        .execute(pool)
        .await?;
    } else {
        // Moving down: shift items in (current_pos, new_position] up by 1
        sqlx::query(
            "UPDATE collection_items SET position = position - 1 \
             WHERE collection_id = ? AND position > ? AND position <= ?",
        )
        .bind(collection_id)
        .bind(current_pos)
        .bind(new_position)
        .execute(pool)
        .await?;
    }

    // Set the item to its new position
    sqlx::query(
        "UPDATE collection_items SET position = ? WHERE collection_id = ? AND media_id = ?",
    )
    .bind(new_position)
    .bind(collection_id)
    .bind(media_id)
    .execute(pool)
    .await?;

    sqlx::query("UPDATE collections SET updated_at = datetime('now') WHERE id = ?")
        .bind(collection_id)
        .execute(pool)
        .await?;

    Ok(true)
}

/// Count items in a collection.
pub async fn count_items(pool: &SqlitePool, collection_id: &str) -> Result<i64> {
    let count: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM collection_items WHERE collection_id = ?")
            .bind(collection_id)
            .fetch_one(pool)
            .await?;
    Ok(count.0)
}
