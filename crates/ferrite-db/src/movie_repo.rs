use anyhow::Result;
use sqlx::{SqlitePool, SqliteConnection};

/// Row for the movies table joined with media_items.
#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
pub struct MovieWithMediaRow {
    // From media_items
    pub id: String,
    pub library_id: String,
    pub media_type: String,
    pub file_path: String,
    pub file_size: i64,
    pub duration_ms: Option<i64>,
    pub container_format: Option<String>,
    pub video_codec: Option<String>,
    pub audio_codec: Option<String>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub bitrate_kbps: Option<i64>,
    // From movies (all optional due to LEFT JOIN)
    pub movie_title: Option<String>,
    pub sort_title: Option<String>,
    pub movie_year: Option<i64>,
    pub overview: Option<String>,
    pub tagline: Option<String>,
    pub rating: Option<f64>,
    pub content_rating: Option<String>,
    pub tmdb_id: Option<i64>,
    pub imdb_id: Option<String>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub genres: Option<String>,
    pub fetched_at: Option<String>,
    // Keep original media_items title/year as fallbacks
    pub title: Option<String>,
    pub year: Option<i64>,
    pub added_at: String,
    pub updated_at: String,
    // From playback_progress (optional due to LEFT JOIN)
    pub position_ms: Option<i64>,
    pub completed: Option<i64>,
    pub last_played_at: Option<String>,
    // Computed: 1 if this media item is a TV episode, 0 otherwise
    pub is_episode: i64,
    // Episode fields (null for non-episodes)
    pub episode_number: Option<i64>,
    pub episode_title: Option<String>,
    pub season_number: Option<i64>,
    pub show_title: Option<String>,
    pub still_path: Option<String>,
}

/// A movie row that still needs metadata fetched from an external provider.
#[derive(Debug, sqlx::FromRow)]
pub struct MovieNeedingMetadata {
    pub media_item_id: String,
    pub title: String,
    pub year: Option<i64>,
}

/// Insert a skeleton movie row (from filename parsing).
/// Uses INSERT OR IGNORE so it will NOT overwrite existing metadata.
/// Accepts `&mut SqliteConnection` so it can run inside a transaction.
pub async fn upsert_movie_skeleton(
    executor: &mut SqliteConnection,
    media_item_id: &str,
    title: &str,
    year: Option<i64>,
) -> Result<()> {
    sqlx::query(
        r#"
        INSERT OR IGNORE INTO movies (media_item_id, title, year)
        VALUES (?, ?, ?)
        "#,
    )
    .bind(media_item_id)
    .bind(title)
    .bind(year)
    .execute(executor)
    .await?;

    Ok(())
}

/// Update all metadata fields for an existing movie row.
/// Sets fetched_at to the current time.
#[allow(clippy::too_many_arguments)]
pub async fn update_movie_metadata(
    pool: &SqlitePool,
    media_item_id: &str,
    tmdb_id: Option<i64>,
    imdb_id: Option<&str>,
    title: &str,
    sort_title: Option<&str>,
    year: Option<i64>,
    overview: Option<&str>,
    tagline: Option<&str>,
    rating: Option<f64>,
    content_rating: Option<&str>,
    poster_path: Option<&str>,
    backdrop_path: Option<&str>,
    genres_json: Option<&str>,
) -> Result<()> {
    sqlx::query(
        r#"
        UPDATE movies
        SET tmdb_id        = ?,
            imdb_id        = ?,
            title          = ?,
            sort_title     = ?,
            year           = ?,
            overview       = ?,
            tagline        = ?,
            rating         = ?,
            content_rating = ?,
            poster_path    = ?,
            backdrop_path  = ?,
            genres         = ?,
            fetched_at     = datetime('now')
        WHERE media_item_id = ?
        "#,
    )
    .bind(tmdb_id)
    .bind(imdb_id)
    .bind(title)
    .bind(sort_title)
    .bind(year)
    .bind(overview)
    .bind(tagline)
    .bind(rating)
    .bind(content_rating)
    .bind(poster_path)
    .bind(backdrop_path)
    .bind(genres_json)
    .bind(media_item_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch a single movie joined with its media_item row.
/// Returns `None` if the media_item_id does not exist.
pub async fn get_movie_with_media(
    pool: &SqlitePool,
    media_item_id: &str,
) -> Result<Option<MovieWithMediaRow>> {
    let row = sqlx::query_as::<_, MovieWithMediaRow>(
        r#"
        SELECT mi.id, mi.library_id, mi.media_type, mi.file_path, mi.file_size, mi.duration_ms,
               mi.container_format, mi.video_codec, mi.audio_codec, mi.width, mi.height, mi.bitrate_kbps,
               COALESCE(m.title, mi.title) AS movie_title,
               m.sort_title,
               COALESCE(m.year, mi.year) AS movie_year,
               m.overview,
               m.tagline, m.rating, m.content_rating,
               m.tmdb_id,
               m.imdb_id,
               COALESCE(ep.still_path, m.poster_path, ts.poster_path) AS poster_path,
               m.backdrop_path,
               COALESCE(m.genres, ts.genres) AS genres,
               m.fetched_at,
               mi.title, mi.year, mi.added_at, mi.updated_at,
               pp.position_ms, pp.completed, pp.last_played_at,
               CASE WHEN ep.media_item_id IS NOT NULL THEN 1 ELSE 0 END AS is_episode,
               ep.episode_number,
               ep.title AS episode_title,
               s.season_number,
               ts.title AS show_title,
               ep.still_path
        FROM media_items mi
        LEFT JOIN movies m ON m.media_item_id = mi.id
        LEFT JOIN playback_progress pp ON pp.media_item_id = mi.id
        LEFT JOIN episodes ep ON ep.media_item_id = mi.id
        LEFT JOIN seasons s ON s.id = ep.season_id
        LEFT JOIN tv_shows ts ON ts.id = s.tv_show_id
        WHERE mi.id = ?
        "#,
    )
    .bind(media_item_id)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

/// Query parameters for filtering, sorting, and paginating media listings.
#[derive(Debug, Default)]
pub struct MediaQuery<'a> {
    pub library_id: Option<&'a str>,
    pub search: Option<&'a str>,
    pub genre: Option<&'a str>,
    pub sort_by: Option<&'a str>,
    pub sort_dir: Option<&'a str>,
    pub page: i64,
    pub per_page: i64,
}

/// List movies joined with media_items, with search, filter, sort, and pagination.
///
/// Uses a fixed SQL template with nullable parameter checks (`? IS NULL OR ...`) so
/// SQLite can cache the prepared statement regardless of which filters are active.
/// Previously the query was built dynamically (different SQL per call), which
/// prevented prepared statement caching and caused repeated parse overhead.
pub async fn list_movies_with_media(
    pool: &SqlitePool,
    query: &MediaQuery<'_>,
) -> Result<Vec<MovieWithMediaRow>> {
    let offset = (query.page - 1) * query.per_page;
    let order_clause = build_order_clause(query);

    let sql = format!(
        r#"
        SELECT mi.id, mi.library_id, mi.media_type, mi.file_path, mi.file_size, mi.duration_ms,
               mi.container_format, mi.video_codec, mi.audio_codec, mi.width, mi.height, mi.bitrate_kbps,
               COALESCE(m.title, mi.title) AS movie_title,
               m.sort_title,
               COALESCE(m.year, mi.year) AS movie_year,
               m.overview,
               m.tagline, m.rating, m.content_rating,
               m.tmdb_id,
               m.imdb_id,
               COALESCE(ep.still_path, m.poster_path, ts.poster_path) AS poster_path,
               m.backdrop_path,
               COALESCE(m.genres, ts.genres) AS genres,
               m.fetched_at,
               mi.title, mi.year, mi.added_at, mi.updated_at,
               pp.position_ms, pp.completed, pp.last_played_at,
               CASE WHEN ep.media_item_id IS NOT NULL THEN 1 ELSE 0 END AS is_episode,
               ep.episode_number,
               ep.title AS episode_title,
               s.season_number,
               ts.title AS show_title,
               ep.still_path
        FROM media_items mi
        LEFT JOIN movies m ON m.media_item_id = mi.id
        LEFT JOIN playback_progress pp ON pp.media_item_id = mi.id
        LEFT JOIN episodes ep ON ep.media_item_id = mi.id
        LEFT JOIN seasons s ON s.id = ep.season_id
        LEFT JOIN tv_shows ts ON ts.id = s.tv_show_id
        WHERE (? IS NULL OR mi.library_id = ?)
          AND (? IS NULL OR COALESCE(m.title, mi.title) LIKE '%' || ? || '%')
          AND (? IS NULL OR m.genres LIKE '%' || ? || '%')
        {order_clause}
        LIMIT ? OFFSET ?
        "#
    );

    let rows = sqlx::query_as::<_, MovieWithMediaRow>(&sql)
        .bind(query.library_id).bind(query.library_id)
        .bind(query.search).bind(query.search)
        .bind(query.genre).bind(query.genre)
        .bind(query.per_page)
        .bind(offset)
        .fetch_all(pool)
        .await?;
    Ok(rows)
}

/// Count movies (media_items) matching the same filters as list_movies_with_media.
/// Uses the same fixed-SQL pattern so the prepared statement is shared/cached.
pub async fn count_movies_with_media(
    pool: &SqlitePool,
    query: &MediaQuery<'_>,
) -> Result<i64> {
    let row: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*)
        FROM media_items mi
        LEFT JOIN movies m ON m.media_item_id = mi.id
        WHERE (? IS NULL OR mi.library_id = ?)
          AND (? IS NULL OR COALESCE(m.title, mi.title) LIKE '%' || ? || '%')
          AND (? IS NULL OR m.genres LIKE '%' || ? || '%')
        "#,
    )
    .bind(query.library_id).bind(query.library_id)
    .bind(query.search).bind(query.search)
    .bind(query.genre).bind(query.genre)
    .fetch_one(pool)
    .await?;
    Ok(row.0)
}

/// Build ORDER BY clause from query parameters.
/// Only the sort expression varies; the WHERE clause is now fixed SQL.
fn build_order_clause(query: &MediaQuery<'_>) -> String {
    let order_expr = match query.sort_by.unwrap_or("title") {
        "year" => "COALESCE(m.year, mi.year)",
        "rating" => "m.rating",
        "added" | "added_at" => "mi.added_at",
        "duration" => "mi.duration_ms",
        "size" | "file_size" => "mi.file_size",
        _ => "COALESCE(m.sort_title, m.title, mi.title)",
    };

    let dir = match query.sort_dir.unwrap_or("asc") {
        "desc" => "DESC",
        _ => "ASC",
    };

    // NULLS LAST for nullable sort columns
    format!("ORDER BY {order_expr} IS NULL, {order_expr} {dir}")
}

/// Get movies that have a skeleton row but no metadata yet (fetched_at IS NULL),
/// filtered to a specific library.
pub async fn get_movies_needing_metadata(
    pool: &SqlitePool,
    library_id: &str,
) -> Result<Vec<MovieNeedingMetadata>> {
    let rows = sqlx::query_as::<_, MovieNeedingMetadata>(
        r#"
        SELECT media_item_id, title, year
        FROM movies
        WHERE fetched_at IS NULL
          AND media_item_id IN (SELECT id FROM media_items WHERE library_id = ?)
        "#,
    )
    .bind(library_id)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}
