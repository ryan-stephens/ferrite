use anyhow::Result;
use sqlx::SqlitePool;

/// Get a single preference value for a user.
pub async fn get_preference(
    pool: &SqlitePool,
    user_id: &str,
    key: &str,
) -> Result<Option<String>> {
    let row = sqlx::query_scalar::<_, String>(
        "SELECT value FROM user_preferences WHERE user_id = ? AND key = ?",
    )
    .bind(user_id)
    .bind(key)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

/// Set (upsert) a preference value for a user.
pub async fn set_preference(
    pool: &SqlitePool,
    user_id: &str,
    key: &str,
    value: &str,
) -> Result<()> {
    sqlx::query(
        "INSERT INTO user_preferences (user_id, key, value) VALUES (?, ?, ?)
         ON CONFLICT(user_id, key) DO UPDATE SET value = excluded.value",
    )
    .bind(user_id)
    .bind(key)
    .bind(value)
    .execute(pool)
    .await?;
    Ok(())
}

/// Delete a preference for a user.
pub async fn delete_preference(pool: &SqlitePool, user_id: &str, key: &str) -> Result<()> {
    sqlx::query("DELETE FROM user_preferences WHERE user_id = ? AND key = ?")
        .bind(user_id)
        .bind(key)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get all preferences for a user as key-value pairs.
pub async fn get_all_preferences(
    pool: &SqlitePool,
    user_id: &str,
) -> Result<Vec<(String, String)>> {
    let rows = sqlx::query_as::<_, (String, String)>(
        "SELECT key, value FROM user_preferences WHERE user_id = ? ORDER BY key",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}
