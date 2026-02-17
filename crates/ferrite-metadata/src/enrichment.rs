use crate::image_cache::ImageCache;
use crate::provider::MetadataProvider;
use crate::tmdb;
use anyhow::Result;
use ferrite_db::{movie_repo, tv_repo};
use sqlx::SqlitePool;
use tracing::{debug, info, warn};

/// Strip a trailing 4-digit year from a title string.
/// Returns (cleaned_title, Some(year)) if found, or (original, None) if not.
fn strip_trailing_year(title: &str) -> (String, Option<i32>) {
    let trimmed = title.trim();
    if trimmed.len() >= 5 {
        let last4 = &trimmed[trimmed.len() - 4..];
        if let Ok(y) = last4.parse::<i32>() {
            if (1900..=2099).contains(&y) {
                let prefix = trimmed[..trimmed.len() - 4].trim_end();
                if !prefix.is_empty() {
                    return (prefix.to_string(), Some(y));
                }
            }
        }
    }
    (trimmed.to_string(), None)
}

/// Enrich all movies in a library that don't have metadata yet.
/// Searches TMDB for each, downloads poster images, saves to DB.
/// Returns the number of movies successfully enriched.
pub async fn enrich_library_movies(
    pool: &SqlitePool,
    library_id: &str,
    provider: &dyn MetadataProvider,
    image_cache: &ImageCache,
) -> Result<u32> {
    let pending = movie_repo::get_movies_needing_metadata(pool, library_id).await?;

    if pending.is_empty() {
        debug!("No movies needing metadata in library {}", library_id);
        return Ok(0);
    }

    info!(
        "Enriching metadata for {} movies in library {}",
        pending.len(),
        library_id
    );
    let mut enriched = 0u32;

    for item in &pending {
        let year = item.year.map(|y| y as i32);

        // Search TMDB
        let results = match provider.search_movie(&item.title, year).await {
            Ok(r) => r,
            Err(e) => {
                warn!("TMDB search failed for '{}': {}", item.title, e);
                continue;
            }
        };

        // Pick best match
        let best = match tmdb::pick_best_match(&results, &item.title, year) {
            Some(m) => m,
            None => {
                debug!("No TMDB match for '{}'", item.title);
                continue;
            }
        };

        // Get full details
        let details = match provider.get_movie_details(best.tmdb_id).await {
            Ok(d) => d,
            Err(e) => {
                warn!(
                    "TMDB details failed for '{}' (id={}): {}",
                    item.title, best.tmdb_id, e
                );
                continue;
            }
        };

        // Download poster
        let poster_local = if let Some(ref pp) = details.poster_path {
            match image_cache.ensure_poster(pp, details.tmdb_id).await {
                Ok(f) => Some(f),
                Err(e) => {
                    warn!("Poster download failed for '{}': {}", item.title, e);
                    None
                }
            }
        } else {
            None
        };

        // Download backdrop
        let backdrop_local = if let Some(ref bp) = details.backdrop_path {
            match image_cache.ensure_backdrop(bp, details.tmdb_id).await {
                Ok(f) => Some(f),
                Err(e) => {
                    warn!("Backdrop download failed: {}", e);
                    None
                }
            }
        } else {
            None
        };

        // Save to DB
        let genres_json = serde_json::to_string(&details.genres).unwrap_or_default();
        if let Err(e) = movie_repo::update_movie_metadata(
            pool,
            &item.media_item_id,
            Some(details.tmdb_id),
            details.imdb_id.as_deref(),
            &details.title,
            details.sort_title.as_deref(),
            details.year.map(|y| y as i64),
            details.overview.as_deref(),
            details.tagline.as_deref(),
            details.rating,
            details.content_rating.as_deref(),
            poster_local.as_deref(),
            backdrop_local.as_deref(),
            Some(genres_json.as_str()),
        )
        .await
        {
            warn!("DB update failed for '{}': {}", item.title, e);
            continue;
        }

        info!(
            "Enriched: '{}' -> TMDB {} ({})",
            item.title, details.tmdb_id, details.title
        );
        enriched += 1;
    }

    info!(
        "Metadata enrichment complete: {}/{} movies enriched",
        enriched,
        pending.len()
    );
    Ok(enriched)
}

/// Enrich all TV shows in a library that don't have metadata yet.
/// Searches TMDB for each, downloads poster/backdrop images, saves to DB.
/// Returns the number of shows successfully enriched.
pub async fn enrich_library_shows(
    pool: &SqlitePool,
    library_id: &str,
    provider: &dyn MetadataProvider,
    image_cache: &ImageCache,
) -> Result<u32> {
    let pending = tv_repo::get_shows_needing_metadata(pool, library_id).await?;

    if pending.is_empty() {
        debug!("No TV shows needing metadata in library {}", library_id);
        return Ok(0);
    }

    info!(
        "Enriching metadata for {} TV shows in library {}",
        pending.len(),
        library_id
    );
    let mut enriched = 0u32;

    for (show_id, title, year) in &pending {
        // Strip trailing year from title if present (e.g. "Star Trek Lower Decks 2020" → "Star Trek Lower Decks")
        let (search_title, parsed_year) = strip_trailing_year(title);
        let year_i32 = year.map(|y| y as i32).or(parsed_year);

        // Search TMDB
        let results = match provider.search_tv(&search_title, year_i32).await {
            Ok(r) => r,
            Err(e) => {
                warn!("TMDB TV search failed for '{}': {}", title, e);
                continue;
            }
        };

        // Pick best match
        let best = match tmdb::pick_best_tv_match(&results, &search_title, year_i32) {
            Some(m) => m,
            None => {
                debug!("No TMDB match for TV show '{}' (searched: '{}')", title, search_title);
                continue;
            }
        };

        // Get full details
        let details = match provider.get_tv_details(best.tmdb_id).await {
            Ok(d) => d,
            Err(e) => {
                warn!(
                    "TMDB TV details failed for '{}' (id={}): {}",
                    title, best.tmdb_id, e
                );
                continue;
            }
        };

        // Download poster
        let poster_local = if let Some(ref pp) = details.poster_path {
            match image_cache.ensure_poster(pp, details.tmdb_id).await {
                Ok(f) => Some(f),
                Err(e) => {
                    warn!("Poster download failed for '{}': {}", title, e);
                    None
                }
            }
        } else {
            None
        };

        // Download backdrop
        let backdrop_local = if let Some(ref bp) = details.backdrop_path {
            match image_cache.ensure_backdrop(bp, details.tmdb_id).await {
                Ok(f) => Some(f),
                Err(e) => {
                    warn!("Backdrop download failed for '{}': {}", title, e);
                    None
                }
            }
        } else {
            None
        };

        // Save to DB
        let genres_json = serde_json::to_string(&details.genres).unwrap_or_default();
        if let Err(e) = tv_repo::update_show_metadata(
            pool,
            show_id,
            Some(details.tmdb_id),
            &details.title,
            details.sort_title.as_deref(),
            details.year.map(|y| y as i64),
            details.overview.as_deref(),
            details.status.as_deref(),
            poster_local.as_deref(),
            backdrop_local.as_deref(),
            Some(genres_json.as_str()),
        )
        .await
        {
            warn!("DB update failed for TV show '{}': {}", title, e);
            continue;
        }

        info!(
            "Enriched TV: '{}' -> TMDB {} ({})",
            title, details.tmdb_id, details.title
        );
        enriched += 1;
    }

    info!(
        "TV metadata enrichment complete: {}/{} shows enriched",
        enriched,
        pending.len()
    );
    Ok(enriched)
}
