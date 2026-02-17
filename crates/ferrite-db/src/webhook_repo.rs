use anyhow::Result;
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
pub struct WebhookRow {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub url: String,
    /// HMAC secret for signing payloads (not serialized to API responses)
    #[serde(skip_serializing)]
    pub secret: Option<String>,
    /// Comma-separated event types or '*' for all
    pub events: String,
    pub enabled: bool,
    pub created_at: String,
    pub updated_at: String,
    pub last_triggered_at: Option<String>,
    pub last_status_code: Option<i64>,
    pub failure_count: i64,
}

/// Create a new webhook.
pub async fn create_webhook(
    pool: &SqlitePool,
    user_id: &str,
    name: &str,
    url: &str,
    secret: Option<&str>,
    events: &str,
) -> Result<WebhookRow> {
    let id = Uuid::new_v4().to_string();
    let row = sqlx::query_as::<_, WebhookRow>(
        "INSERT INTO webhooks (id, user_id, name, url, secret, events) VALUES (?, ?, ?, ?, ?, ?) RETURNING *",
    )
    .bind(&id)
    .bind(user_id)
    .bind(name)
    .bind(url)
    .bind(secret)
    .bind(events)
    .fetch_one(pool)
    .await?;
    Ok(row)
}

/// List all webhooks for a user.
pub async fn list_webhooks(pool: &SqlitePool, user_id: &str) -> Result<Vec<WebhookRow>> {
    let rows = sqlx::query_as::<_, WebhookRow>(
        "SELECT * FROM webhooks WHERE user_id = ? ORDER BY created_at DESC",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

/// Get a single webhook by ID.
pub async fn get_webhook(pool: &SqlitePool, id: &str) -> Result<Option<WebhookRow>> {
    let row = sqlx::query_as::<_, WebhookRow>("SELECT * FROM webhooks WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?;
    Ok(row)
}

/// Update a webhook's configuration.
pub async fn update_webhook(
    pool: &SqlitePool,
    id: &str,
    name: &str,
    url: &str,
    secret: Option<&str>,
    events: &str,
    enabled: bool,
) -> Result<Option<WebhookRow>> {
    let row = sqlx::query_as::<_, WebhookRow>(
        "UPDATE webhooks SET name = ?, url = ?, secret = ?, events = ?, enabled = ?, \
         updated_at = datetime('now') WHERE id = ? RETURNING *",
    )
    .bind(name)
    .bind(url)
    .bind(secret)
    .bind(events)
    .bind(enabled)
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

/// Delete a webhook.
pub async fn delete_webhook(pool: &SqlitePool, id: &str) -> Result<bool> {
    let result = sqlx::query("DELETE FROM webhooks WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}

/// Get all enabled webhooks that subscribe to a given event type.
pub async fn get_webhooks_for_event(pool: &SqlitePool, event_type: &str) -> Result<Vec<WebhookRow>> {
    // Match webhooks that subscribe to '*' (all events) or contain the specific event type
    let rows = sqlx::query_as::<_, WebhookRow>(
        "SELECT * FROM webhooks WHERE enabled = 1 AND (events = '*' OR events LIKE '%' || ? || '%')",
    )
    .bind(event_type)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

/// Record the result of a webhook delivery attempt.
pub async fn record_delivery(
    pool: &SqlitePool,
    webhook_id: &str,
    status_code: Option<i64>,
    success: bool,
) -> Result<()> {
    if success {
        sqlx::query(
            "UPDATE webhooks SET last_triggered_at = datetime('now'), last_status_code = ?, \
             failure_count = 0 WHERE id = ?",
        )
        .bind(status_code)
        .bind(webhook_id)
        .execute(pool)
        .await?;
    } else {
        sqlx::query(
            "UPDATE webhooks SET last_triggered_at = datetime('now'), last_status_code = ?, \
             failure_count = failure_count + 1 WHERE id = ?",
        )
        .bind(status_code)
        .bind(webhook_id)
        .execute(pool)
        .await?;
    }
    Ok(())
}
