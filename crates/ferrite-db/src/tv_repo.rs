use anyhow::Result;
use sqlx::{SqliteConnection, SqlitePool};
use uuid::Uuid;

// ── Upsert helpers (called during scanning) ──────────────────────────────────

/// Normalize a show title for fuzzy matching:
/// lowercase, strip all non-alphanumeric characters, collapse spaces.
/// e.g. "Star Trek: Lower Decks" → "star trek lower decks"
///      "Star Trek Lower Decks 2020" → "star trek lower decks 2020"
fn normalize_title(title: &str) -> String {
    title
        .chars()
        .filter_map(|c| {
            if c.is_alphanumeric() {
                Some(c.to_ascii_lowercase())
            } else if c.is_whitespace() {
                Some(' ')
            } else {
                None // strip punctuation (colons, dashes, apostrophes, etc.)
            }
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// Strip a trailing 4-digit year (19xx or 20xx) from a normalized title.
/// e.g. "star trek lower decks 2020" → "star trek lower decks"
fn strip_year_suffix(s: &str) -> &str {
    if s.len() >= 5 {
        let (prefix, suffix) = s.split_at(s.len() - 4);
        if (suffix.starts_with("19") || suffix.starts_with("20"))
            && suffix.chars().all(|c| c.is_ascii_digit())
            && prefix.ends_with(' ')
        {
            return prefix.trim_end();
        }
    }
    s
}

/// Ensure a tv_show row exists for the given library + title.
/// Uses normalized fuzzy matching so title variants like
/// "Star Trek: Lower Decks" and "Star Trek Lower Decks 2020"
/// resolve to the same show. Returns the show's ID.
/// Accepts `&mut SqliteConnection` so it can run inside a transaction.
pub async fn upsert_tv_show(
    executor: &mut SqliteConnection,
    library_id: &str,
    title: &str,
) -> Result<String> {
    // 1. Exact match first (fast path — covered by primary key / unique index)
    let row: Option<(String,)> =
        sqlx::query_as("SELECT id FROM tv_shows WHERE library_id = ? AND title = ?")
            .bind(library_id)
            .bind(title)
            .fetch_optional(&mut *executor)
            .await?;

    if let Some((id,)) = row {
        return Ok(id);
    }

    // 2. Fuzzy match via the normalized_title index (O(1) instead of O(N) full scan).
    //    The normalized_title column is maintained by the DB and indexed, so this is
    //    a single index seek rather than loading all shows into Rust memory.
    let norm_incoming = normalize_title(title);
    let norm_base = strip_year_suffix(&norm_incoming).to_string();

    let fuzzy_row: Option<(String,)> = sqlx::query_as(
        "SELECT id FROM tv_shows WHERE library_id = ? AND normalized_title = ? LIMIT 1",
    )
    .bind(library_id)
    .bind(&norm_base)
    .fetch_optional(&mut *executor)
    .await?;

    if let Some((id,)) = fuzzy_row {
        return Ok(id);
    }

    // 3. No match — insert new show with its normalized title pre-computed
    let id = Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO tv_shows (id, library_id, title, normalized_title) VALUES (?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(library_id)
    .bind(title)
    .bind(&norm_base)
    .execute(&mut *executor)
    .await?;

    Ok(id)
}

/// Ensure a season row exists for the given show + season number.
/// Returns the season's ID (existing or newly created).
/// Accepts `&mut SqliteConnection` so it can run inside a transaction.
pub async fn upsert_season(
    executor: &mut SqliteConnection,
    tv_show_id: &str,
    season_number: u32,
) -> Result<String> {
    let row: Option<(String,)> =
        sqlx::query_as("SELECT id FROM seasons WHERE tv_show_id = ? AND season_number = ?")
            .bind(tv_show_id)
            .bind(season_number as i64)
            .fetch_optional(&mut *executor)
            .await?;

    if let Some((id,)) = row {
        return Ok(id);
    }

    let id = Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO seasons (id, tv_show_id, season_number) VALUES (?, ?, ?)")
        .bind(&id)
        .bind(tv_show_id)
        .bind(season_number as i64)
        .execute(&mut *executor)
        .await?;

    Ok(id)
}

/// Insert or update an episode row linking a media_item to a season.
/// Accepts `&mut SqliteConnection` so it can run inside a transaction.
pub async fn upsert_episode(
    executor: &mut SqliteConnection,
    media_item_id: &str,
    season_id: &str,
    episode_number: u32,
) -> Result<()> {
    sqlx::query(
        r#"INSERT OR REPLACE INTO episodes (media_item_id, season_id, episode_number)
           VALUES (?, ?, ?)"#,
    )
    .bind(media_item_id)
    .bind(season_id)
    .bind(episode_number as i64)
    .execute(executor)
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
pub async fn list_shows(pool: &SqlitePool, library_id: &str) -> Result<Vec<TvShowRow>> {
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
pub async fn get_show(pool: &SqlitePool, show_id: &str) -> Result<Option<TvShowRow>> {
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
pub async fn list_seasons(pool: &SqlitePool, show_id: &str) -> Result<Vec<SeasonRow>> {
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
    user_id: Option<&str>,
) -> Result<Vec<EpisodeRow>> {
    let rows = sqlx::query_as::<_, EpisodeRow>(
        r#"SELECT e.media_item_id, e.season_id, e.episode_number,
                  e.title AS episode_title, e.overview, e.air_date, e.still_path,
                  mi.file_path, mi.file_size, mi.duration_ms,
                  mi.video_codec, mi.audio_codec, mi.width, mi.height,
                  pp.position_ms, pp.completed, pp.last_played_at
           FROM episodes e
           JOIN media_items mi ON mi.id = e.media_item_id
           LEFT JOIN playback_progress pp ON pp.media_item_id = e.media_item_id AND pp.user_id IS ?
           WHERE e.season_id = ?
           ORDER BY e.episode_number ASC"#,
    )
    .bind(user_id)
    .bind(season_id)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

/// Response type for the next-episode endpoint.
#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
pub struct NextEpisodeRow {
    pub media_item_id: String,
    pub season_id: String,
    pub episode_number: i64,
    pub season_number: i64,
    pub episode_title: Option<String>,
    pub overview: Option<String>,
    pub still_path: Option<String>,
    pub duration_ms: Option<i64>,
    pub show_title: String,
    pub show_poster_path: Option<String>,
}

/// Find the next episode after the given media_item_id within the same show.
/// Looks first in the same season (next episode_number), then the first episode
/// of the next season. Returns None if the given item is the last episode.
pub async fn get_next_episode(
    pool: &SqlitePool,
    media_item_id: &str,
) -> Result<Option<NextEpisodeRow>> {
    let row = sqlx::query_as::<_, NextEpisodeRow>(
        r#"
        WITH current AS (
            SELECT e.episode_number, e.season_id, s.season_number, s.tv_show_id
            FROM episodes e
            JOIN seasons s ON s.id = e.season_id
            WHERE e.media_item_id = ?
        ),
        -- Next episode in the same season (lowest episode_number above current)
        same_season_next AS (
            SELECT
                e.media_item_id,
                e.season_id,
                e.episode_number,
                s.season_number,
                e.title       AS episode_title,
                e.overview,
                e.still_path,
                mi.duration_ms,
                ts.title      AS show_title,
                ts.poster_path AS show_poster_path
            FROM episodes e
            JOIN seasons s ON s.id = e.season_id
            JOIN media_items mi ON mi.id = e.media_item_id
            JOIN tv_shows ts ON ts.id = s.tv_show_id
            JOIN current c ON e.season_id = c.season_id
            WHERE e.episode_number > c.episode_number
            ORDER BY e.episode_number ASC
            LIMIT 1
        ),
        -- First episode of the next season (lowest season_number above current, then lowest episode_number)
        next_season_first AS (
            SELECT
                e.media_item_id,
                e.season_id,
                e.episode_number,
                s.season_number,
                e.title       AS episode_title,
                e.overview,
                e.still_path,
                mi.duration_ms,
                ts.title      AS show_title,
                ts.poster_path AS show_poster_path
            FROM episodes e
            JOIN seasons s ON s.id = e.season_id
            JOIN media_items mi ON mi.id = e.media_item_id
            JOIN tv_shows ts ON ts.id = s.tv_show_id
            JOIN current c ON s.tv_show_id = c.tv_show_id
            WHERE s.season_number > c.season_number
            ORDER BY s.season_number ASC, e.episode_number ASC
            LIMIT 1
        )
        -- Prefer same-season next; fall back to next season's first episode
        SELECT * FROM same_season_next
        UNION ALL
        SELECT * FROM next_season_first
        LIMIT 1
        "#,
    )
    .bind(media_item_id)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

/// Remove seasons that have no episodes (orphaned after a rescan/merge).
pub async fn delete_empty_seasons(pool: &SqlitePool) -> Result<u64> {
    let result = sqlx::query(
        "DELETE FROM seasons WHERE id NOT IN (SELECT DISTINCT season_id FROM episodes)",
    )
    .execute(pool)
    .await?;
    Ok(result.rows_affected())
}

/// Remove tv_shows that have no seasons (orphaned after a rescan/merge).
pub async fn delete_empty_shows(pool: &SqlitePool) -> Result<u64> {
    let result = sqlx::query(
        "DELETE FROM tv_shows WHERE id NOT IN (SELECT DISTINCT tv_show_id FROM seasons)",
    )
    .execute(pool)
    .await?;
    Ok(result.rows_affected())
}

/// Update episode metadata fields (title, overview, air_date, still_path) by
/// matching on season_id + episode_number. Only updates rows that exist in the
/// episodes table (i.e. files we actually have on disk).
/// still_path is only overwritten when the new value is non-NULL, preserving
/// any previously cached image path if TMDB returns no still for this episode.
pub async fn update_episode_metadata(
    pool: &SqlitePool,
    season_id: &str,
    episode_number: i64,
    title: Option<&str>,
    overview: Option<&str>,
    air_date: Option<&str>,
    still_path: Option<&str>,
) -> Result<()> {
    sqlx::query(
        r#"UPDATE episodes
           SET title      = COALESCE(?, title),
               overview   = COALESCE(?, overview),
               air_date   = COALESCE(?, air_date),
               still_path = COALESCE(?, still_path)
           WHERE season_id = ? AND episode_number = ?"#,
    )
    .bind(title)
    .bind(overview)
    .bind(air_date)
    .bind(still_path)
    .bind(season_id)
    .bind(episode_number)
    .execute(pool)
    .await?;

    Ok(())
}

/// Update episode metadata within an existing transaction (no pool, no semaphore).
/// still_path is only overwritten when the new value is non-NULL, preserving
/// any previously cached image path if TMDB returns no still for this episode.
pub async fn update_episode_metadata_tx(
    conn: &mut SqliteConnection,
    season_id: &str,
    episode_number: i64,
    title: Option<&str>,
    overview: Option<&str>,
    air_date: Option<&str>,
    still_path: Option<&str>,
) -> Result<()> {
    sqlx::query(
        r#"UPDATE episodes
           SET title      = COALESCE(?, title),
               overview   = COALESCE(?, overview),
               air_date   = COALESCE(?, air_date),
               still_path = COALESCE(?, still_path)
           WHERE season_id = ? AND episode_number = ?"#,
    )
    .bind(title)
    .bind(overview)
    .bind(air_date)
    .bind(still_path)
    .bind(season_id)
    .bind(episode_number)
    .execute(&mut *conn)
    .await?;

    Ok(())
}

/// Return (season_id, season_number) pairs for all seasons of a show.
pub async fn get_seasons_for_show(pool: &SqlitePool, show_id: &str) -> Result<Vec<(String, i64)>> {
    let rows: Vec<(String, i64)> = sqlx::query_as(
        "SELECT id, season_number FROM seasons WHERE tv_show_id = ? ORDER BY season_number ASC",
    )
    .bind(show_id)
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

/// Get TV shows that have show-level metadata (tmdb_id set) but still have
/// episodes missing title, still_path, or overview — used to backfill episode
/// metadata for shows enriched before episode fetching was implemented, and
/// for shows that gained new seasons after initial enrichment.
pub async fn get_shows_needing_episode_metadata(
    pool: &SqlitePool,
    library_id: &str,
) -> Result<Vec<(String, String, Option<i64>)>> {
    let rows: Vec<(String, String, Option<i64>)> = sqlx::query_as(
        r#"SELECT DISTINCT ts.id, ts.title, ts.tmdb_id
           FROM tv_shows ts
           JOIN seasons s ON s.tv_show_id = ts.id
           JOIN episodes e ON e.season_id = s.id
           WHERE ts.library_id = ?
             AND ts.tmdb_id IS NOT NULL
             AND (e.title IS NULL OR e.still_path IS NULL OR e.overview IS NULL)"#,
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
