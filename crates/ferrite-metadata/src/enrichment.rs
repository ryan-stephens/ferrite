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
                warn!("No TMDB match for movie '{}' ({} results returned)", item.title, results.len());
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
        let db_result: Result<(), anyhow::Error> = async {
            movie_repo::update_movie_metadata(
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
            .await?;
            let mut conn = pool.acquire().await?;
            movie_repo::upsert_fts_for_media(
                &mut conn,
                &item.media_item_id,
                &details.title,
                details.overview.as_deref().unwrap_or(""),
                &genres_json,
            )
            .await?;
            Ok(())
        }
        .await;
        if let Err(e) = db_result {
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
                warn!("No TMDB match for TV show '{}' (searched: '{}', {} results returned)", title, search_title, results.len());
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

        // Fetch episode metadata for every season we have on disk
        let seasons = match tv_repo::get_seasons_for_show(pool, show_id).await {
            Ok(s) => s,
            Err(e) => {
                warn!("Failed to get seasons for show '{}': {}", title, e);
                continue;
            }
        };

        for (season_id, season_number) in &seasons {
            let episodes = match provider.get_season_episodes(details.tmdb_id, *season_number).await {
                Ok(eps) => eps,
                Err(e) => {
                    warn!(
                        "TMDB season {} fetch failed for '{}': {}",
                        season_number, title, e
                    );
                    continue;
                }
            };

            for ep in &episodes {
                // Cache still image if present
                let still_local = if let Some(ref sp) = ep.still_path {
                    match image_cache.ensure_still(sp, details.tmdb_id, *season_number, ep.episode_number).await {
                        Ok(f) => Some(f),
                        Err(e) => {
                            debug!("Still image download failed for S{}E{}: {}", season_number, ep.episode_number, e);
                            None
                        }
                    }
                } else {
                    None
                };

                if let Err(e) = tv_repo::update_episode_metadata(
                    pool,
                    season_id,
                    ep.episode_number as i64,
                    ep.title.as_deref(),
                    ep.overview.as_deref(),
                    ep.air_date.as_deref(),
                    still_local.as_deref(),
                )
                .await
                {
                    warn!(
                        "Failed to update episode S{}E{} for '{}': {}",
                        season_number, ep.episode_number, title, e
                    );
                    continue;
                }

                // Update FTS index for this episode's media_item
                if let Ok(media_item_id) = tv_repo::get_episode_media_item_id(pool, season_id, ep.episode_number as i64).await {
                    if let Some(mid) = media_item_id {
                        let fts_title = format!("{} {}", details.title, ep.title.as_deref().unwrap_or(""));
                        let fts_overview = ep.overview.as_deref().unwrap_or("");
                        if let Ok(mut conn) = pool.acquire().await {
                            let _ = movie_repo::upsert_fts_for_media(&mut conn, &mid, &fts_title, fts_overview, &genres_json).await;
                        }
                    }
                }
            }

            debug!(
                "Updated {} episode(s) for '{}' season {}",
                episodes.len(), title, season_number
            );
        }
    }

    info!(
        "TV metadata enrichment complete: {}/{} shows enriched",
        enriched,
        pending.len()
    );

    // Backfill episode metadata for shows that already have show-level metadata
    // but were enriched before episode fetching was implemented.
    let backfill = tv_repo::get_shows_needing_episode_metadata(pool, library_id).await?;
    if !backfill.is_empty() {
        info!(
            "Backfilling episode metadata for {} show(s) in library {}",
            backfill.len(), library_id
        );
        for (show_id, title, tmdb_id_opt) in &backfill {
            let tmdb_id = match tmdb_id_opt {
                Some(id) => *id,
                None => continue,
            };
            let seasons = match tv_repo::get_seasons_for_show(pool, show_id).await {
                Ok(s) => s,
                Err(e) => { warn!("get_seasons_for_show failed for '{}': {}", title, e); continue; }
            };
            for (season_id, season_number) in &seasons {
                let episodes = match provider.get_season_episodes(tmdb_id, *season_number).await {
                    Ok(eps) => eps,
                    Err(e) => { warn!("TMDB season {} fetch failed for '{}': {}", season_number, title, e); continue; }
                };
                for ep in &episodes {
                    let still_local = if let Some(ref sp) = ep.still_path {
                        match image_cache.ensure_still(sp, tmdb_id, *season_number, ep.episode_number).await {
                            Ok(f) => Some(f),
                            Err(e) => { debug!("Still download failed S{}E{}: {}", season_number, ep.episode_number, e); None }
                        }
                    } else { None };
                    let _ = tv_repo::update_episode_metadata(
                        pool, season_id, ep.episode_number as i64,
                        ep.title.as_deref(), ep.overview.as_deref(),
                        ep.air_date.as_deref(), still_local.as_deref(),
                    ).await;
                }
                debug!("Backfilled {} episode(s) for '{}' season {}", episodes.len(), title, season_number);
            }
        }
    }

    Ok(enriched)
}

/// Enrich a single TV show by show_id. Used for inline enrichment during scanning.
/// Returns Ok(true) if enriched, Ok(false) if already enriched or no match.
///
/// All HTTP and image downloads are done first (no DB lock held), then a single
/// write_sem acquisition covers all DB writes in one transaction.
pub async fn enrich_single_show(
    pool: &SqlitePool,
    show_id: &str,
    title: &str,
    provider: &dyn MetadataProvider,
    image_cache: &ImageCache,
    write_sem: &tokio::sync::Semaphore,
) -> Result<bool> {
    let (search_title, parsed_year) = strip_trailing_year(title);

    let results = match provider.search_tv(&search_title, parsed_year).await {
        Ok(r) => r,
        Err(e) => { warn!("TMDB TV search failed for '{}': {}", title, e); return Ok(false); }
    };

    let best = match tmdb::pick_best_tv_match(&results, &search_title, parsed_year) {
        Some(m) => m,
        None => { warn!("No TMDB match for TV show '{}' (searched: '{}', {} results returned)", title, search_title, results.len()); return Ok(false); }
    };

    let details = match provider.get_tv_details(best.tmdb_id).await {
        Ok(d) => d,
        Err(e) => { warn!("TMDB TV details failed for '{}' (id={}): {}", title, best.tmdb_id, e); return Ok(false); }
    };

    // ── Phase 1: all HTTP / image work (no DB lock held) ─────────────────────

    let poster_local = if let Some(ref pp) = details.poster_path {
        match image_cache.ensure_poster(pp, details.tmdb_id).await {
            Ok(f) => Some(f),
            Err(e) => { warn!("Poster download failed for '{}': {}", title, e); None }
        }
    } else { None };

    let backdrop_local = if let Some(ref bp) = details.backdrop_path {
        match image_cache.ensure_backdrop(bp, details.tmdb_id).await {
            Ok(f) => Some(f),
            Err(e) => { warn!("Backdrop download failed for '{}': {}", title, e); None }
        }
    } else { None };

    let genres_json = serde_json::to_string(&details.genres).unwrap_or_default();

    // Fetch seasons from DB (read-only, no write lock needed)
    let seasons = tv_repo::get_seasons_for_show(pool, show_id).await.unwrap_or_default();

    // For each season, fetch all episode metadata + still images via HTTP
    struct SeasonData {
        season_id: String,
        episodes: Vec<EpisodeData>,
    }
    struct EpisodeData {
        episode_number: i64,
        ep_title: Option<String>,
        overview: Option<String>,
        air_date: Option<String>,
        still_local: Option<String>,
    }

    let mut season_data: Vec<SeasonData> = Vec::with_capacity(seasons.len());
    for (season_id, season_number) in &seasons {
        let episodes = match provider.get_season_episodes(details.tmdb_id, *season_number).await {
            Ok(eps) => eps,
            Err(e) => { warn!("TMDB season {} fetch failed for '{}': {}", season_number, title, e); continue; }
        };
        let mut ep_data: Vec<EpisodeData> = Vec::with_capacity(episodes.len());
        for ep in &episodes {
            let still_local = if let Some(ref sp) = ep.still_path {
                match image_cache.ensure_still(sp, details.tmdb_id, *season_number, ep.episode_number).await {
                    Ok(f) => Some(f),
                    Err(e) => { debug!("Still download failed S{}E{}: {}", season_number, ep.episode_number, e); None }
                }
            } else { None };
            ep_data.push(EpisodeData {
                episode_number: ep.episode_number as i64,
                ep_title: ep.title.clone(),
                overview: ep.overview.clone(),
                air_date: ep.air_date.clone(),
                still_local,
            });
        }
        debug!("Fetched {} episode(s) for '{}' season {}", ep_data.len(), title, season_number);
        season_data.push(SeasonData { season_id: season_id.clone(), episodes: ep_data });
    }

    // ── Phase 2: single write_sem acquisition, all DB writes in one transaction

    let _wp = write_sem.acquire().await.expect("semaphore closed");
    let mut tx = pool.begin().await?;

    // Update show-level metadata
    sqlx::query(
        r#"UPDATE tv_shows
           SET tmdb_id       = ?,
               sort_title    = ?,
               year          = ?,
               overview      = ?,
               status        = ?,
               poster_path   = ?,
               backdrop_path = ?,
               genres        = ?,
               fetched_at    = datetime('now')
           WHERE id = ?"#,
    )
    .bind(Some(details.tmdb_id))
    .bind(details.sort_title.as_deref())
    .bind(details.year.map(|y| y as i64))
    .bind(details.overview.as_deref())
    .bind(details.status.as_deref())
    .bind(poster_local.as_deref())
    .bind(backdrop_local.as_deref())
    .bind(Some(genres_json.as_str()))
    .bind(show_id)
    .execute(&mut *tx)
    .await?;

    // Update all episode metadata
    for sd in &season_data {
        for ep in &sd.episodes {
            let _ = tv_repo::update_episode_metadata_tx(
                &mut *tx,
                &sd.season_id,
                ep.episode_number,
                ep.ep_title.as_deref(),
                ep.overview.as_deref(),
                ep.air_date.as_deref(),
                ep.still_local.as_deref(),
            ).await;
        }
    }

    tx.commit().await?;
    drop(_wp);

    info!("Enriched TV: '{}' -> TMDB {} ({})", title, details.tmdb_id, details.title);
    Ok(true)
}

/// Enrich a single movie by media_item_id. Used for inline enrichment during scanning.
/// Returns Ok(true) if enriched, Ok(false) if no match found.
pub async fn enrich_single_movie(
    pool: &SqlitePool,
    media_item_id: &str,
    title: &str,
    year: Option<i32>,
    provider: &dyn MetadataProvider,
    image_cache: &ImageCache,
    write_sem: &tokio::sync::Semaphore,
) -> Result<bool> {
    let results = match provider.search_movie(title, year).await {
        Ok(r) => r,
        Err(e) => { warn!("TMDB search failed for '{}': {}", title, e); return Ok(false); }
    };

    let best = match tmdb::pick_best_match(&results, title, year) {
        Some(m) => m,
        None => { debug!("No TMDB match for '{}'", title); return Ok(false); }
    };

    let details = match provider.get_movie_details(best.tmdb_id).await {
        Ok(d) => d,
        Err(e) => { warn!("TMDB details failed for '{}' (id={}): {}", title, best.tmdb_id, e); return Ok(false); }
    };

    let poster_local = if let Some(ref pp) = details.poster_path {
        match image_cache.ensure_poster(pp, details.tmdb_id).await {
            Ok(f) => Some(f),
            Err(e) => { warn!("Poster download failed for '{}': {}", title, e); None }
        }
    } else { None };

    let backdrop_local = if let Some(ref bp) = details.backdrop_path {
        match image_cache.ensure_backdrop(bp, details.tmdb_id).await {
            Ok(f) => Some(f),
            Err(e) => { warn!("Backdrop download failed: {}", e); None }
        }
    } else { None };

    let genres_json = serde_json::to_string(&details.genres).unwrap_or_default();
    let _wp = write_sem.acquire().await.expect("semaphore closed");
    if let Err(e) = movie_repo::update_movie_metadata(
        pool, media_item_id, Some(details.tmdb_id), details.imdb_id.as_deref(),
        &details.title, details.sort_title.as_deref(), details.year.map(|y| y as i64),
        details.overview.as_deref(), details.tagline.as_deref(), details.rating,
        details.content_rating.as_deref(), poster_local.as_deref(),
        backdrop_local.as_deref(), Some(genres_json.as_str()),
    ).await {
        warn!("DB update failed for '{}': {}", title, e);
        return Ok(false);
    }
    drop(_wp);

    info!("Enriched: '{}' -> TMDB {} ({})", title, details.tmdb_id, details.title);
    Ok(true)
}
