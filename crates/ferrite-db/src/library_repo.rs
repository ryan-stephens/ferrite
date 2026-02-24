use anyhow::Result;
use ferrite_core::media::{Library, LibraryType};
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn create_library(
    pool: &SqlitePool,
    name: &str,
    path: &str,
    library_type: LibraryType,
) -> Result<Library> {
    // Reject duplicate paths to prevent scan conflicts.
    let existing: Option<(String,)> = sqlx::query_as("SELECT id FROM libraries WHERE path = ?")
        .bind(path)
        .fetch_optional(pool)
        .await?;
    if existing.is_some() {
        anyhow::bail!("A library with this path already exists");
    }

    let id = Uuid::new_v4().to_string();
    let lib_type = serde_json::to_value(library_type)?
        .as_str()
        .unwrap_or("movie")
        .to_string();

    sqlx::query("INSERT INTO libraries (id, name, path, library_type) VALUES (?, ?, ?, ?)")
        .bind(&id)
        .bind(name)
        .bind(path)
        .bind(&lib_type)
        .execute(pool)
        .await?;

    get_library(pool, &id).await
}

pub async fn get_library(pool: &SqlitePool, id: &str) -> Result<Library> {
    let row = sqlx::query_as::<_, LibraryRow>("SELECT * FROM libraries WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(row.into())
}

pub async fn list_libraries(pool: &SqlitePool) -> Result<Vec<Library>> {
    let rows = sqlx::query_as::<_, LibraryRow>("SELECT * FROM libraries ORDER BY name")
        .fetch_all(pool)
        .await?;
    Ok(rows.into_iter().map(Into::into).collect())
}

pub async fn delete_library(pool: &SqlitePool, id: &str) -> Result<()> {
    sqlx::query("DELETE FROM libraries WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn update_last_scanned(pool: &SqlitePool, id: &str) -> Result<()> {
    sqlx::query("UPDATE libraries SET last_scanned_at = datetime('now') WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

#[derive(sqlx::FromRow)]
struct LibraryRow {
    id: String,
    name: String,
    path: String,
    library_type: String,
    scan_interval_minutes: Option<i64>,
    last_scanned_at: Option<String>,
    created_at: String,
}

impl From<LibraryRow> for Library {
    fn from(row: LibraryRow) -> Self {
        let library_type = match row.library_type.as_str() {
            "tv" => LibraryType::Tv,
            "music" => LibraryType::Music,
            _ => LibraryType::Movie,
        };
        Library {
            id: Uuid::parse_str(&row.id).unwrap_or_else(|_| Uuid::new_v4()),
            name: row.name,
            path: row.path,
            library_type,
            scan_interval_minutes: row.scan_interval_minutes.unwrap_or(60) as u32,
            last_scanned_at: row.last_scanned_at.and_then(|s| {
                chrono::NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
                    .ok()
                    .map(|dt| dt.and_utc())
            }),
            created_at: chrono::NaiveDateTime::parse_from_str(&row.created_at, "%Y-%m-%d %H:%M:%S")
                .map(|dt| dt.and_utc())
                .unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}
