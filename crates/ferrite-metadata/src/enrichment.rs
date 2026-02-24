use crate::image_cache::ImageCache;
use crate::provider::{MetadataProvider, TvSearchResult};
use crate::tmdb;
use anyhow::Result;
use ferrite_db::{movie_repo, tv_repo};
use futures::stream::{self, StreamExt};
use sqlx::SqlitePool;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use tracing::{debug, info, warn};

const EPISODE_STILL_DOWNLOAD_CONCURRENCY: usize = 8;

/// Strip a trailing 4-digit year from a title string.
/// Handles both bare years ("Cosmos 2014") and parenthesized years ("Cosmos (2014)").
/// Returns (cleaned_title, Some(year)) if found, or (original, None) if not.
fn strip_trailing_year(title: &str) -> (String, Option<i32>) {
    let trimmed = title.trim();

    // Try parenthesized year: "Title (YYYY)"
    if trimmed.ends_with(')') {
        if let Some(open) = trimmed.rfind('(') {
            let inner = &trimmed[open + 1..trimmed.len() - 1];
            if let Ok(y) = inner.trim().parse::<i32>() {
                if (1900..=2099).contains(&y) {
                    let prefix = trimmed[..open].trim_end();
                    if !prefix.is_empty() {
                        return (prefix.to_string(), Some(y));
                    }
                }
            }
        }
    }

    // Try bare trailing year: "Title 2014"
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

/// Build candidate search queries for TV matching.
///
/// This keeps the original title first, then tries a few lightweight fallback
/// rewrites for common abbreviations and aliases seen in real-world filenames
/// (e.g. "Survivor AU" -> "Australian Survivor").
fn build_tv_search_candidates(search_title: &str) -> Vec<String> {
    let base = search_title.trim();
    if base.is_empty() {
        return Vec::new();
    }

    let mut candidates = vec![base.to_string()];
    let parts: Vec<&str> = base.split_whitespace().collect();

    // Country suffix aliases often appear in scene-style names.
    // e.g. "Survivor AU" / "Survivor Australia" -> "Australian Survivor"
    if parts.len() >= 2 {
        let suffix = parts
            .last()
            .copied()
            .unwrap_or_default()
            .to_ascii_lowercase();
        let stem = parts[..parts.len() - 1].join(" ");
        if !stem.is_empty() && matches!(suffix.as_str(), "au" | "australia") {
            candidates.push(format!("{} Australia", stem));
            candidates.push(format!("Australian {}", stem));
        }
    }

    if base.contains('&') {
        candidates.push(base.replace('&', "and"));
    }

    // Deduplicate while preserving order.
    let mut deduped = Vec::with_capacity(candidates.len());
    for candidate in candidates {
        let normalized = candidate.split_whitespace().collect::<Vec<_>>().join(" ");
        if normalized.is_empty() {
            continue;
        }
        if !deduped
            .iter()
            .any(|existing: &String| existing.eq_ignore_ascii_case(&normalized))
        {
            deduped.push(normalized);
        }
    }

    deduped
}

/// Search TMDB using one or more candidate queries and return the first
/// candidate that yields a high-confidence match.
async fn find_best_tv_match(
    provider: &dyn MetadataProvider,
    search_title: &str,
    year: Option<i32>,
) -> Option<(TvSearchResult, String, usize)> {
    for candidate in build_tv_search_candidates(search_title) {
        let results = match provider.search_tv(&candidate, year).await {
            Ok(r) => r,
            Err(e) => {
                warn!(
                    "TMDB TV search failed for '{}' (candidate='{}'): {}",
                    search_title, candidate, e
                );
                continue;
            }
        };

        if let Some(best) = tmdb::pick_best_tv_match(&results, &candidate, year) {
            return Some((best, candidate, results.len()));
        }
    }

    None
}

/// Enrich all movies in a library that don't have metadata yet.
/// Searches TMDB for each, downloads poster images, saves to DB.
/// Returns the number of movies successfully enriched.
pub async fn enrich_library_movies(
    pool: &SqlitePool,
    library_id: &str,
    provider: Arc<dyn MetadataProvider>,
    image_cache: Arc<ImageCache>,
) -> Result<u32> {
    let pending = movie_repo::get_movies_needing_metadata(pool, library_id).await?;

    if pending.is_empty() {
        debug!("No movies needing metadata in library {}", library_id);
        return Ok(0);
    }

    let pending_count = pending.len();
    info!(
        "Enriching metadata for {} movies in library {}",
        pending_count, library_id
    );
    let enriched = Arc::new(AtomicU32::new(0));

    stream::iter(pending)
        .map(|item| {
            let provider = provider.clone();
            let image_cache = image_cache.clone();
            let enriched = enriched.clone();
            let pool = pool.clone();
            async move {
                let year = item.year.map(|y| y as i32);

                // Search TMDB
                let results = match provider.search_movie(&item.title, year).await {
                    Ok(r) => r,
                    Err(e) => {
                        warn!("TMDB search failed for '{}': {}", item.title, e);
                        return;
                    }
                };

                // Pick best match
                let best = match tmdb::pick_best_match(&results, &item.title, year) {
                    Some(m) => m,
                    None => {
                        debug!("No TMDB match for '{}'", item.title);
                        return;
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
                        return;
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
                    &pool,
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
                    return;
                }

                info!(
                    "Enriched: '{}' -> TMDB {} ({})",
                    item.title, details.tmdb_id, details.title
                );
                enriched.fetch_add(1, Ordering::Relaxed);
            }
        })
        .buffer_unordered(8)
        .collect::<Vec<()>>()
        .await;

    let enriched = enriched.load(Ordering::Relaxed);
    info!(
        "Metadata enrichment complete: {}/{} movies enriched",
        enriched, pending_count
    );
    Ok(enriched)
}

/// Enrich all TV shows in a library that don't have metadata yet.
/// Searches TMDB for each, downloads poster/backdrop images, saves to DB.
/// Returns the number of shows successfully enriched.
pub async fn enrich_library_shows(
    pool: &SqlitePool,
    library_id: &str,
    provider: Arc<dyn MetadataProvider>,
    image_cache: Arc<ImageCache>,
) -> Result<u32> {
    let pending = tv_repo::get_shows_needing_metadata(pool, library_id).await?;

    if pending.is_empty() {
        debug!("No TV shows needing metadata in library {}", library_id);
    }

    let pending_count = pending.len();
    info!(
        "Enriching metadata for {} TV shows in library {}",
        pending_count, library_id
    );
    let enriched = Arc::new(AtomicU32::new(0));

    stream::iter(pending)
        .map(|(show_id, title, year)| {
            let provider = provider.clone();
            let image_cache = image_cache.clone();
            let enriched = enriched.clone();
            let pool = pool.clone();
            async move {
                // Strip trailing year from title if present (e.g. "Star Trek Lower Decks 2020" → "Star Trek Lower Decks")
                let (search_title, parsed_year) = strip_trailing_year(&title);
                let year_i32 = year.map(|y| y as i32).or(parsed_year);

                let (best, matched_query, result_count) =
                    match find_best_tv_match(provider.as_ref(), &search_title, year_i32).await {
                        Some(found) => found,
                        None => {
                            debug!(
                                "No TMDB match for TV show '{}' (searched: '{}')",
                                title, search_title
                            );
                            return;
                        }
                    };

                if !matched_query.eq_ignore_ascii_case(&search_title) {
                    debug!(
                        "TMDB TV match fallback used for '{}': '{}' ({} results)",
                        title, matched_query, result_count
                    );
                }

                // Get full details
                let details = match provider.get_tv_details(best.tmdb_id).await {
                    Ok(d) => d,
                    Err(e) => {
                        warn!(
                            "TMDB TV details failed for '{}' (id={}): {}",
                            title, best.tmdb_id, e
                        );
                        return;
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
                    &pool,
                    &show_id,
                    Some(details.tmdb_id),
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
                    return;
                }

                info!(
                    "Enriched TV: '{}' -> TMDB {} ({})",
                    title, details.tmdb_id, details.title
                );
                enriched.fetch_add(1, Ordering::Relaxed);

                // Fetch episode metadata for every season we have on disk
                let seasons = match tv_repo::get_seasons_for_show(&pool, &show_id).await {
                    Ok(s) => s,
                    Err(e) => {
                        warn!("Failed to get seasons for show '{}': {}", title, e);
                        return;
                    }
                };

                for (season_id, season_number) in &seasons {
                    let episodes = match provider
                        .get_season_episodes(details.tmdb_id, *season_number)
                        .await
                    {
                        Ok(eps) => eps,
                        Err(e) => {
                            warn!(
                                "TMDB season {} fetch failed for '{}': {}",
                                season_number, title, e
                            );
                            continue;
                        }
                    };

                    // Download episode still images concurrently, then write metadata.
                    struct EpStill {
                        episode_number: i64,
                        ep_title: Option<String>,
                        overview: Option<String>,
                        air_date: Option<String>,
                        still_local: Option<String>,
                    }
                    let ic = image_cache.clone();
                    let tmdb_id = details.tmdb_id;
                    let sn = *season_number;
                    let ep_data: Vec<EpStill> = stream::iter(episodes)
                        .map(move |ep| {
                            let ic = ic.clone();
                            async move {
                                let still_local = if let Some(ref sp) = ep.still_path {
                                    match ic.ensure_still(sp, tmdb_id, sn, ep.episode_number).await
                                    {
                                        Ok(f) => Some(f),
                                        Err(e) => {
                                            debug!(
                                                "Still image download failed for S{}E{}: {}",
                                                sn, ep.episode_number, e
                                            );
                                            None
                                        }
                                    }
                                } else {
                                    None
                                };
                                EpStill {
                                    episode_number: ep.episode_number as i64,
                                    ep_title: ep.title,
                                    overview: ep.overview,
                                    air_date: ep.air_date,
                                    still_local,
                                }
                            }
                        })
                        .buffer_unordered(EPISODE_STILL_DOWNLOAD_CONCURRENCY)
                        .collect()
                        .await;

                    for ep in &ep_data {
                        if let Err(e) = tv_repo::update_episode_metadata(
                            &pool,
                            season_id,
                            ep.episode_number,
                            ep.ep_title.as_deref(),
                            ep.overview.as_deref(),
                            ep.air_date.as_deref(),
                            ep.still_local.as_deref(),
                        )
                        .await
                        {
                            warn!(
                                "Failed to update episode S{}E{} for '{}': {}",
                                season_number, ep.episode_number, title, e
                            );
                        }
                    }

                    debug!(
                        "Updated {} episode(s) for '{}' season {}",
                        ep_data.len(),
                        title,
                        season_number
                    );
                }
            }
        })
        .buffer_unordered(4)
        .collect::<Vec<()>>()
        .await;

    let enriched = enriched.load(Ordering::Relaxed);
    info!(
        "TV metadata enrichment complete: {}/{} shows enriched",
        enriched, pending_count
    );

    // Backfill episode metadata for shows that already have show-level metadata
    // but were enriched before episode fetching was implemented.
    let backfill = tv_repo::get_shows_needing_episode_metadata(pool, library_id).await?;
    if !backfill.is_empty() {
        info!(
            "Backfilling episode metadata for {} show(s) in library {}",
            backfill.len(),
            library_id
        );
        stream::iter(backfill)
            .map(|(show_id, title, tmdb_id_opt)| {
                let provider = provider.clone();
                let image_cache = image_cache.clone();
                let pool = pool.clone();
                async move {
                    let tmdb_id = match tmdb_id_opt {
                        Some(id) => id,
                        None => return,
                    };
                    let seasons = match tv_repo::get_seasons_for_show(&pool, &show_id).await {
                        Ok(s) => s,
                        Err(e) => {
                            warn!("get_seasons_for_show failed for '{}': {}", title, e);
                            return;
                        }
                    };
                    for (season_id, season_number) in &seasons {
                        let episodes =
                            match provider.get_season_episodes(tmdb_id, *season_number).await {
                                Ok(eps) => eps,
                                Err(e) => {
                                    warn!(
                                        "TMDB season {} fetch failed for '{}': {}",
                                        season_number, title, e
                                    );
                                    continue;
                                }
                            };

                        // Download stills concurrently
                        struct BackfillEp {
                            episode_number: i64,
                            ep_title: Option<String>,
                            overview: Option<String>,
                            air_date: Option<String>,
                            still_local: Option<String>,
                        }
                        let ic = image_cache.clone();
                        let sn = *season_number;
                        let ep_data: Vec<BackfillEp> = stream::iter(episodes)
                            .map(move |ep| {
                                let ic = ic.clone();
                                async move {
                                    let still_local = if let Some(ref sp) = ep.still_path {
                                        match ic
                                            .ensure_still(sp, tmdb_id, sn, ep.episode_number)
                                            .await
                                        {
                                            Ok(f) => Some(f),
                                            Err(e) => {
                                                debug!(
                                                    "Still download failed S{}E{}: {}",
                                                    sn, ep.episode_number, e
                                                );
                                                None
                                            }
                                        }
                                    } else {
                                        None
                                    };
                                    BackfillEp {
                                        episode_number: ep.episode_number as i64,
                                        ep_title: ep.title,
                                        overview: ep.overview,
                                        air_date: ep.air_date,
                                        still_local,
                                    }
                                }
                            })
                            .buffer_unordered(EPISODE_STILL_DOWNLOAD_CONCURRENCY)
                            .collect()
                            .await;

                        for ep in &ep_data {
                            let _ = tv_repo::update_episode_metadata(
                                &pool,
                                season_id,
                                ep.episode_number,
                                ep.ep_title.as_deref(),
                                ep.overview.as_deref(),
                                ep.air_date.as_deref(),
                                ep.still_local.as_deref(),
                            )
                            .await;
                        }
                        debug!(
                            "Backfilled {} episode(s) for '{}' season {}",
                            ep_data.len(),
                            title,
                            season_number
                        );
                    }
                }
            })
            .buffer_unordered(4)
            .collect::<Vec<()>>()
            .await;
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

    let (best, matched_query, result_count) =
        match find_best_tv_match(provider, &search_title, parsed_year).await {
            Some(found) => found,
            None => {
                warn!(
                    "No TMDB match for TV show '{}' (searched: '{}')",
                    title, search_title,
                );
                return Ok(false);
            }
        };

    if !matched_query.eq_ignore_ascii_case(&search_title) {
        info!(
            "TMDB TV match fallback used for '{}': '{}' ({} results)",
            title, matched_query, result_count
        );
    }

    let details = match provider.get_tv_details(best.tmdb_id).await {
        Ok(d) => d,
        Err(e) => {
            warn!(
                "TMDB TV details failed for '{}' (id={}): {}",
                title, best.tmdb_id, e
            );
            return Ok(false);
        }
    };

    // ── Phase 1: all HTTP / image work (no DB lock held) ─────────────────────

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

    let genres_json = serde_json::to_string(&details.genres).unwrap_or_default();

    // ── Phase 1: fetch seasons + episode HTTP data (no DB write lock held) ──────

    // Snapshot seasons now for the HTTP fetch phase. We re-fetch inside the
    // transaction to catch any seasons added between now and the write lock.
    let seasons_snapshot = tv_repo::get_seasons_for_show(pool, show_id)
        .await
        .unwrap_or_default();

    struct SeasonData {
        season_number: i64,
        episodes: Vec<EpisodeData>,
    }
    struct EpisodeData {
        episode_number: i64,
        ep_title: Option<String>,
        overview: Option<String>,
        air_date: Option<String>,
        still_local: Option<String>,
    }

    let mut season_data: Vec<SeasonData> = Vec::with_capacity(seasons_snapshot.len());
    for (season_id, season_number) in &seasons_snapshot {
        let on_disk_episodes = match tv_repo::get_episode_numbers_for_season(pool, season_id).await
        {
            Ok(numbers) => numbers,
            Err(e) => {
                warn!(
                    "Failed to load on-disk episodes for '{}' season {}: {}",
                    title, season_number, e
                );
                continue;
            }
        };
        if on_disk_episodes.is_empty() {
            continue;
        }

        let episodes = match provider
            .get_season_episodes(details.tmdb_id, *season_number)
            .await
        {
            Ok(eps) => eps,
            Err(e) => {
                warn!(
                    "TMDB season {} fetch failed for '{}': {}",
                    season_number, title, e
                );
                continue;
            }
        };

        let on_disk_set: std::collections::HashSet<i64> = on_disk_episodes.into_iter().collect();
        let filtered: Vec<_> = episodes
            .into_iter()
            .filter(|ep| on_disk_set.contains(&(ep.episode_number as i64)))
            .collect();

        let tmdb_id = details.tmdb_id;
        let season = *season_number;
        let ep_data: Vec<EpisodeData> = stream::iter(filtered)
            .map(|ep| async move {
                let still_local = if let Some(sp) = ep.still_path.as_deref() {
                    match image_cache
                        .ensure_still(sp, tmdb_id, season, ep.episode_number)
                        .await
                    {
                        Ok(f) => Some(f),
                        Err(e) => {
                            debug!(
                                "Still download failed S{}E{}: {}",
                                season, ep.episode_number, e
                            );
                            None
                        }
                    }
                } else {
                    None
                };

                EpisodeData {
                    episode_number: ep.episode_number as i64,
                    ep_title: ep.title,
                    overview: ep.overview,
                    air_date: ep.air_date,
                    still_local,
                }
            })
            .buffer_unordered(EPISODE_STILL_DOWNLOAD_CONCURRENCY)
            .collect()
            .await;

        debug!(
            "Fetched {} on-disk episode(s) for '{}' season {}",
            ep_data.len(),
            title,
            season_number
        );
        season_data.push(SeasonData {
            season_number: *season_number,
            episodes: ep_data,
        });
    }

    // ── Phase 2: single write_sem acquisition, all DB writes in one transaction

    let _wp = write_sem.acquire().await.expect("semaphore closed");
    let mut tx = pool.begin().await?;

    // Update show-level metadata
    sqlx::query(
        r#"UPDATE tv_shows
           SET tmdb_id       = ?,
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
    .bind(details.year.map(|y| y as i64))
    .bind(details.overview.as_deref())
    .bind(details.status.as_deref())
    .bind(poster_local.as_deref())
    .bind(backdrop_local.as_deref())
    .bind(Some(genres_json.as_str()))
    .bind(show_id)
    .execute(&mut *tx)
    .await?;

    // Re-fetch seasons inside the transaction to catch any added since the snapshot.
    let seasons_current = sqlx::query_as::<_, (String, i64)>(
        "SELECT id, season_number FROM seasons WHERE tv_show_id = ? ORDER BY season_number ASC",
    )
    .bind(show_id)
    .fetch_all(&mut *tx)
    .await
    .unwrap_or_default();

    // Build a lookup from season_number → pre-fetched episode data
    let season_data_map: std::collections::HashMap<i64, &SeasonData> = season_data
        .iter()
        .map(|sd| (sd.season_number, sd))
        .collect();

    // Write episode metadata for all current seasons (including any added after snapshot)
    for (season_id, season_number) in &seasons_current {
        if let Some(sd) = season_data_map.get(season_number) {
            // Season was in the snapshot — use pre-fetched data
            for ep in &sd.episodes {
                let _ = tv_repo::update_episode_metadata_tx(
                    &mut tx,
                    season_id,
                    ep.episode_number,
                    ep.ep_title.as_deref(),
                    ep.overview.as_deref(),
                    ep.air_date.as_deref(),
                    ep.still_local.as_deref(),
                )
                .await;
            }
        }
        // Seasons added after the snapshot will be picked up by the next
        // backfill pass (get_shows_needing_episode_metadata).
    }

    tx.commit().await?;
    drop(_wp);

    info!(
        "Enriched TV: '{}' -> TMDB {} ({})",
        title, details.tmdb_id, details.title
    );
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
        Err(e) => {
            warn!("TMDB search failed for '{}': {}", title, e);
            return Ok(false);
        }
    };

    let best = match tmdb::pick_best_match(&results, title, year) {
        Some(m) => m,
        None => {
            warn!(
                "No TMDB match for '{}' ({} results returned)",
                title,
                results.len()
            );
            return Ok(false);
        }
    };

    let details = match provider.get_movie_details(best.tmdb_id).await {
        Ok(d) => d,
        Err(e) => {
            warn!(
                "TMDB details failed for '{}' (id={}): {}",
                title, best.tmdb_id, e
            );
            return Ok(false);
        }
    };

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

    let genres_json = serde_json::to_string(&details.genres).unwrap_or_default();
    let _wp = write_sem.acquire().await.expect("semaphore closed");
    if let Err(e) = movie_repo::update_movie_metadata(
        pool,
        media_item_id,
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
        warn!("DB update failed for '{}': {}", title, e);
        return Ok(false);
    }
    drop(_wp);

    info!(
        "Enriched: '{}' -> TMDB {} ({})",
        title, details.tmdb_id, details.title
    );
    Ok(true)
}
