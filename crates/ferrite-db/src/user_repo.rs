use anyhow::Result;
use sqlx::SqlitePool;
use uuid::Uuid;

/// A row from the users table.
#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
pub struct UserRow {
    pub id: String,
    pub username: String,
    pub display_name: Option<String>,
    /// Excluded from default serialization — only used internally for password verification.
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub is_admin: i64,
    pub created_at: String,
    pub last_login_at: Option<String>,
}

/// Create a new user with a bcrypt-hashed password. Returns the new user row.
pub async fn create_user(
    pool: &SqlitePool,
    username: &str,
    display_name: Option<&str>,
    password: &str,
    is_admin: bool,
) -> Result<UserRow> {
    let id = Uuid::new_v4().to_string();
    let hash = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;

    sqlx::query(
        r#"INSERT INTO users (id, username, display_name, password_hash, is_admin)
           VALUES (?, ?, ?, ?, ?)"#,
    )
    .bind(&id)
    .bind(username)
    .bind(display_name)
    .bind(&hash)
    .bind(is_admin as i32)
    .execute(pool)
    .await?;

    get_user_by_id(pool, &id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Failed to retrieve newly created user"))
}

/// Get a user by ID.
pub async fn get_user_by_id(pool: &SqlitePool, id: &str) -> Result<Option<UserRow>> {
    let row = sqlx::query_as::<_, UserRow>("SELECT * FROM users WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?;
    Ok(row)
}

/// Get a user by username (for login).
pub async fn get_user_by_username(pool: &SqlitePool, username: &str) -> Result<Option<UserRow>> {
    let row = sqlx::query_as::<_, UserRow>("SELECT * FROM users WHERE username = ?")
        .bind(username)
        .fetch_optional(pool)
        .await?;
    Ok(row)
}

/// List all users (admin view).
pub async fn list_users(pool: &SqlitePool) -> Result<Vec<UserRow>> {
    let rows = sqlx::query_as::<_, UserRow>(
        "SELECT * FROM users ORDER BY created_at ASC",
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

/// Verify a plaintext password against a user's stored bcrypt hash.
pub fn verify_password(password: &str, hash: &str) -> bool {
    bcrypt::verify(password, hash).unwrap_or(false)
}

/// Update last_login_at timestamp for a user.
pub async fn update_last_login(pool: &SqlitePool, user_id: &str) -> Result<()> {
    sqlx::query("UPDATE users SET last_login_at = datetime('now') WHERE id = ?")
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Count total users (used to check if setup is needed).
pub async fn count_users(pool: &SqlitePool) -> Result<i64> {
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await?;
    Ok(row.0)
}

/// Delete a user by ID.
pub async fn delete_user(pool: &SqlitePool, user_id: &str) -> Result<()> {
    sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Change a user's password.
pub async fn change_password(pool: &SqlitePool, user_id: &str, new_password: &str) -> Result<()> {
    let hash = bcrypt::hash(new_password, bcrypt::DEFAULT_COST)?;
    sqlx::query("UPDATE users SET password_hash = ? WHERE id = ?")
        .bind(&hash)
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}
