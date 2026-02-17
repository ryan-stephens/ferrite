use anyhow::Result;
use sqlx::SqlitePool;
use uuid::Uuid;

// ── Upsert helpers (called during scanning) ──────────────────────────────────

/// Ensure a tv_show row exists for the given library + title.
/// Returns the show's ID (existing or newly created).
pub async fn upsert_tv_show(
    pool: &SqlitePool,
    library_id: &str,
    title: &str,
) -> Result<String> {
    let row: Option<(String,)> = sqlx::query_as(
        "SELECT id FROM tv_shows WHERE library_id = ? AND title = ?",
    )
    .bind(library_id)
    .bind(title)
    .fetch_optional(pool)
    .await?;

    if let Some((id,)) = row {
        return Ok(id);
    }

    let id = Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO tv_shows (id, library_id, title) VALUES (?, ?, ?)",
    )
    .bind(&id)
    .bind(library_id)
    .bind(title)
    .execute(pool)
    .await?;

    Ok(id)
}

/// Ensure a season row exists for the given show + season number.
/// Returns the season's ID (existing or newly created).
pub async fn upsert_season(
    pool: &SqlitePool,
    tv_show_id: &str,
    season_number: u32,
) -> Result<String> {
    let row: Option<(String,)> = sqlx::query_as(
        "SELECT id FROM seasons WHERE tv_show_id = ? AND season_number = ?",
    )
    .bind(tv_show_id)
    .bind(season_number as i64)
    .fetch_optional(pool)
    .await?;

    if let Some((id,)) = row {
        return Ok(id);
    }

    let id = Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO seasons (id, tv_show_id, season_number) VALUES (?, ?, ?)",
    )
    .bind(&id)
    .bind(tv_show_id)
    .bind(season_number as i64)
    .execute(pool)
    .await?;

    Ok(id)
}

/// Insert or update an episode row linking a media_item to a season.
pub async fn upsert_episode(
    pool: &SqlitePool,
    media_item_id: &str,
    season_id: &str,
    episode_number: u32,
) -> Result<()> {
    sqlx::query(
        r#"INSERT INTO episodes (media_item_id, season_id, episode_number)
           VALUES (?, ?, ?)
           ON CONFLICT(media_item_id) DO UPDATE SET
               season_id = excluded.season_id,
               episode_number = excluded.episode_number"#,
    )
    .bind(media_item_id)
    .bind(season_id)
    .bind(episode_number as i64)
    .execute(pool)
    .await?;

    Ok(())
}

// ── Query types ──────────────────────────────────────────────────────────────

/// A TV show row for API responses.
#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
pub struct TvShowRow {
    pub id: String,
    pub library_id: String,
    pub title: String,
    pub sort_title: Option<String>,
    pub year: Option<i64>,
    pub overview: Option<String>,
    pub status: Option<String>,
    pub tmdb_id: Option<i64>,
    pub tvdb_id: Option<i64>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub genres: Option<String>,
    pub fetched_at: Option<String>,
    pub season_count: i64,
    pub episode_count: i64,
}

/// A season row for API responses.
#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
pub struct SeasonRow {
    pub id: String,
    pub tv_show_id: String,
    pub season_number: i64,
    pub title: Option<String>,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub episode_count: i64,
}

/// An episode row joined with media_item data for API responses.
#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
pub struct EpisodeRow {
    pub media_item_id: String,
    pub season_id: String,
    pub episode_number: i64,
    pub episode_title: Option<String>,
    pub overview: Option<String>,
    pub air_date: Option<String>,
    pub still_path: Option<String>,
    // From media_items
    pub file_path: String,
    pub file_size: i64,
    pub duration_ms: Option<i64>,
    pub video_codec: Option<String>,
    pub audio_codec: Option<String>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    // From playback_progress
    pub position_ms: Option<i64>,
    pub completed: Option<i64>,
    pub last_played_at: Option<String>,
}

// ── Query functions ──────────────────────────────────────────────────────────

/// List all TV shows in a library, with season and episode counts.
pub async fn list_shows(
    pool: &SqlitePool,
    library_id: &str,
) -> Result<Vec<TvShowRow>> {
    let rows = sqlx::query_as::<_, TvShowRow>(
        r#"SELECT ts.*,
                  (SELECT COUNT(*) FROM seasons s WHERE s.tv_show_id = ts.id) AS season_count,
                  (SELECT COUNT(*) FROM episodes e
                   JOIN seasons s ON s.id = e.season_id
                   WHERE s.tv_show_id = ts.id) AS episode_count
           FROM tv_shows ts
           WHERE ts.library_id = ?
           ORDER BY COALESCE(ts.sort_title, ts.title) ASC"#,
    )
    .bind(library_id)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

/// Get a single TV show by ID with counts.
pub async fn get_show(
    pool: &SqlitePool,
    show_id: &str,
) -> Result<Option<TvShowRow>> {
    let row = sqlx::query_as::<_, TvShowRow>(
        r#"SELECT ts.*,
                  (SELECT COUNT(*) FROM seasons s WHERE s.tv_show_id = ts.id) AS season_count,
                  (SELECT COUNT(*) FROM episodes e
                   JOIN seasons s ON s.id = e.season_id
                   WHERE s.tv_show_id = ts.id) AS episode_count
           FROM tv_shows ts
           WHERE ts.id = ?"#,
    )
    .bind(show_id)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

/// List all seasons for a TV show, with episode counts, ordered by season number.
pub async fn list_seasons(
    pool: &SqlitePool,
    show_id: &str,
) -> Result<Vec<SeasonRow>> {
    let rows = sqlx::query_as::<_, SeasonRow>(
        r#"SELECT s.*,
                  (SELECT COUNT(*) FROM episodes e WHERE e.season_id = s.id) AS episode_count
           FROM seasons s
           WHERE s.tv_show_id = ?
           ORDER BY s.season_number ASC"#,
    )
    .bind(show_id)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

/// List all episodes in a season, joined with media_item and playback_progress data.
pub async fn list_episodes(
    pool: &SqlitePool,
    season_id: &str,
) -> Result<Vec<EpisodeRow>> {
    let rows = sqlx::query_as::<_, EpisodeRow>(
        r#"SELECT e.media_item_id, e.season_id, e.episode_number,
                  e.title AS episode_title, e.overview, e.air_date, e.still_path,
                  mi.file_path, mi.file_size, mi.duration_ms,
                  mi.video_codec, mi.audio_codec, mi.width, mi.height,
                  pp.position_ms, pp.completed, pp.last_played_at
           FROM episodes e
           JOIN media_items mi ON mi.id = e.media_item_id
           LEFT JOIN playback_progress pp ON pp.media_item_id = e.media_item_id
           WHERE e.season_id = ?
           ORDER BY e.episode_number ASC"#,
    )
    .bind(season_id)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

/// Get TV shows that have no metadata yet (fetched_at IS NULL) for a given library.
pub async fn get_shows_needing_metadata(
    pool: &SqlitePool,
    library_id: &str,
) -> Result<Vec<(String, String, Option<i64>)>> {
    let rows: Vec<(String, String, Option<i64>)> = sqlx::query_as(
        "SELECT id, title, year FROM tv_shows WHERE library_id = ? AND fetched_at IS NULL",
    )
    .bind(library_id)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

/// Update all metadata fields for an existing tv_show row.
/// Sets fetched_at to the current time.
#[allow(clippy::too_many_arguments)]
pub async fn update_show_metadata(
    pool: &SqlitePool,
    show_id: &str,
    tmdb_id: Option<i64>,
    title: &str,
    sort_title: Option<&str>,
    year: Option<i64>,
    overview: Option<&str>,
    status: Option<&str>,
    poster_path: Option<&str>,
    backdrop_path: Option<&str>,
    genres_json: Option<&str>,
) -> Result<()> {
    sqlx::query(
        r#"
        UPDATE tv_shows
        SET tmdb_id       = ?,
            title         = ?,
            sort_title    = ?,
            year          = ?,
            overview      = ?,
            status        = ?,
            poster_path   = ?,
            backdrop_path = ?,
            genres        = ?,
            fetched_at    = datetime('now')
        WHERE id = ?
        "#,
    )
    .bind(tmdb_id)
    .bind(title)
    .bind(sort_title)
    .bind(year)
    .bind(overview)
    .bind(status)
    .bind(poster_path)
    .bind(backdrop_path)
    .bind(genres_json)
    .bind(show_id)
    .execute(pool)
    .await?;

    Ok(())
}
