pub mod extract;
pub mod filename;
pub mod probe;
pub mod progress;
pub mod subtitle;
pub mod walker;
pub mod watcher;

use anyhow::Result;
use ferrite_core::media::{LibraryType, AUDIO_EXTENSIONS, VIDEO_EXTENSIONS};
use ferrite_db::chapter_repo::ChapterInsert;
use ferrite_db::library_repo;
use ferrite_db::media_repo::{self, MediaProbeData};
use ferrite_db::movie_repo;
use ferrite_db::stream_repo::StreamInsert;
use ferrite_db::tv_repo;
use filename::{ParsedEpisode, ParsedFilename, ParsedMovie};
use futures::stream::{self, StreamExt};
use progress::{ScanState, ScanStatus};
use sqlx::SqlitePool;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Semaphore;
use tracing::{debug, info, warn};

pub use progress::{ScanProgress, ScanRegistry};
pub use watcher::WatcherHandle;

/// Scan a single library using a per-item concurrent pipeline.
///
/// Each file is probed, inserted into the DB, and has subtitles extracted
/// concurrently. Items become visible in the UI as they are inserted rather
/// than waiting for the entire scan to complete.
///
/// `scan_state` tracks live progress for the status API endpoint.
/// `tmdb_provider` and `image_cache` are optional — if provided, metadata
/// enrichment runs inline as each new show/movie is first encountered.
pub async fn scan_library(
    pool: &SqlitePool,
    library_id: &str,
    ffprobe_path: &str,
    ffmpeg_path: &str,
    concurrent_probes: usize,
    subtitle_cache_dir: &Path,
    scan_state: Arc<ScanState>,
    tmdb_provider: Option<Arc<dyn ferrite_metadata::provider::MetadataProvider>>,
    image_cache: Option<Arc<ferrite_metadata::image_cache::ImageCache>>,
) -> Result<u32> {
    let library = library_repo::get_library(pool, library_id).await?;
    let lib_path = Path::new(&library.path);

    if !lib_path.exists() {
        warn!("Library path does not exist: {}", library.path);
        scan_state.set_status(ScanStatus::Failed).await;
        return Ok(0);
    }

    info!("Scanning library '{}' at {}", library.name, library.path);

    let extensions: &[&str] = match library.library_type {
        LibraryType::Movie | LibraryType::Tv => VIDEO_EXTENSIONS,
        LibraryType::Music => AUDIO_EXTENSIONS,
    };

    let files = walker::walk_directory(lib_path, extensions).await?;
    let total = files.len() as u32;
    info!("Found {} media files in '{}'", total, library.name);
    scan_state
        .total_files
        .store(total, std::sync::atomic::Ordering::Relaxed);

    let media_type = match library.library_type {
        LibraryType::Movie => "movie",
        LibraryType::Tv => "episode",
        LibraryType::Music => "track",
    };

    let is_movie_library = matches!(library.library_type, LibraryType::Movie);
    let is_tv_library = matches!(library.library_type, LibraryType::Tv);

    // Delta scan: load existing (file_path -> file_size) to skip unchanged files.
    // Wrapped in Arc so it is shared across all per-item futures without cloning.
    let existing: Arc<std::collections::HashMap<String, u64>> = Arc::new(
        media_repo::get_all_file_sizes(pool, library_id)
            .await
            .unwrap_or_default(),
    );

    let probe_sem = Arc::new(Semaphore::new(concurrent_probes));
    let sub_sem = Arc::new(Semaphore::new(concurrent_probes));
    // SQLite effectively allows one writer at a time; serializing write
    // transactions avoids lock-wait thrash during full-library scans.
    // ffprobe and ffmpeg work still proceed concurrently.
    let write_sem = Arc::new(Semaphore::new(1));

    let mut count = 0u32;

    let phase1_results = stream::iter(files)
        .map(|file| {
            let pool = pool.clone();
            let probe_sem = probe_sem.clone();
            let write_sem = write_sem.clone();
            let ffprobe = ffprobe_path.to_string();
            let library_id = library_id.to_string();
            let library_uuid = library.id;
            let media_type = media_type.to_string();
            let scan_state = scan_state.clone();
            let existing = existing.clone(); // cheap Arc clone

            async move {
                let file_path_str = file.path.to_string_lossy().to_string();
                let file_stem = file
                    .path
                    .file_stem()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_default();

                let parsed = filename::parse_filename(&file_stem);
                let (title, year) = match &parsed {
                    ParsedFilename::Movie(ParsedMovie { title, year }) => (title.clone(), *year),
                    ParsedFilename::Episode(ParsedEpisode { show_name, .. }) => (show_name.clone(), None),
                    ParsedFilename::Unknown(name) => (name.clone(), None),
                };

                // Delta scan: skip unchanged files
                let already_indexed = existing.get(&file_path_str)
                    .map(|&sz| sz == file.size)
                    .unwrap_or(false);

                if already_indexed {
                    debug!("Skipping unchanged file: {}", file_path_str);
                    scan_state.inc_probed();
                    return Ok(None);
                }

                scan_state.set_current(&format!("Probing: {}", title)).await;

                let (probe_data, streams, chapters, keyframe_index_ms) = {
                    let _permit = probe_sem.acquire().await.expect("semaphore closed");
                    match probe::probe_file(&ffprobe, &file.path).await {
                        Ok(pr) => {
                            let stream_inserts = pr.streams.iter().map(|s| StreamInsert {
                                stream_index: s.index,
                                stream_type: s.stream_type.clone(),
                                codec_name: s.codec_name.clone(),
                                codec_long_name: s.codec_long_name.clone(),
                                profile: s.profile.clone(),
                                language: s.language.clone(),
                                title: s.title.clone(),
                                is_default: s.is_default,
                                is_forced: s.is_forced,
                                width: s.width,
                                height: s.height,
                                frame_rate: s.frame_rate.clone(),
                                pixel_format: s.pixel_format.clone(),
                                bit_depth: s.bit_depth,
                                color_space: s.color_space.clone(),
                                color_transfer: s.color_transfer.clone(),
                                color_primaries: s.color_primaries.clone(),
                                channels: s.channels,
                                channel_layout: s.channel_layout.clone(),
                                sample_rate: s.sample_rate,
                                bitrate_bps: s.bitrate_bps,
                            }).collect();
                            let chapter_inserts = pr.chapters.iter().map(|c| ChapterInsert {
                                chapter_index: c.chapter_index,
                                title: c.title.clone(),
                                start_time_ms: c.start_time_ms,
                                end_time_ms: c.end_time_ms,
                            }).collect();
                            let data = MediaProbeData {
                                container_format: pr.container_format,
                                video_codec: pr.video_codec,
                                audio_codec: pr.audio_codec,
                                width: pr.width,
                                height: pr.height,
                                duration_ms: pr.duration_ms,
                                bitrate_kbps: pr.bitrate_kbps,
                            };
                            (
                                Some(data),
                                stream_inserts,
                                chapter_inserts,
                                pr.keyframe_index_ms,
                            )
                        }
                        Err(e) => {
                            warn!("ffprobe failed for {}: {}", file.path.display(), e);
                            scan_state.inc_errors();
                            (None, Vec::new(), Vec::new(), None)
                        }
                    }
                };

                scan_state.inc_probed();
                scan_state.set_current(&format!("Indexing: {}", title)).await;

                // ── Phase 1: insert media item, streams, TV hierarchy ─────────
                // Gated behind write_sem to limit concurrent SQLite write transactions.
                let _write_permit = write_sem.acquire().await.expect("semaphore closed");
                let mut tx = pool.begin().await?;

                let mid = media_repo::insert_media_item(
                    &mut *tx,
                    &library_uuid,
                    &media_type,
                    &file_path_str,
                    file.size,
                    Some(&title),
                    year,
                    probe_data.as_ref(),
                ).await?;

                if !streams.is_empty() {
                    if let Err(e) = ferrite_db::stream_repo::replace_streams(&mut *tx, &mid, &streams).await {
                        warn!("Failed to store streams for '{}': {}", title, e);
                    }
                }
                if !chapters.is_empty() {
                    if let Err(e) = ferrite_db::chapter_repo::replace_chapters(&mut *tx, &mid, &chapters).await {
                        warn!("Failed to store chapters for '{}': {}", title, e);
                    }
                }
                if let Some(keyframes_ms) = keyframe_index_ms.as_ref() {
                    if let Err(e) = ferrite_db::keyframe_repo::replace_keyframes(&mut *tx, &mid, keyframes_ms).await {
                        warn!("Failed to store keyframe index for '{}': {}", title, e);
                    }
                }
                if is_movie_library {
                    if let Err(e) = movie_repo::upsert_movie_skeleton(&mut *tx, &mid, &title, year.map(|y| y as i64)).await {
                        warn!("Failed to create movie skeleton for '{}': {}", title, e);
                    }
                }

                let _show_id: Option<String> = if is_tv_library {
                    if let ParsedFilename::Episode(ParsedEpisode { show_name, season, episode }) = &parsed {
                        match tv_repo::upsert_tv_show(&mut *tx, &library_id, show_name).await {
                            Ok(show_id) => {
                                match tv_repo::upsert_season(&mut *tx, &show_id, *season).await {
                                    Ok(season_id) => {
                                        if let Err(e) = tv_repo::upsert_episode(&mut *tx, &mid, &season_id, *episode).await {
                                            warn!("Failed to create episode for '{}' S{:02}E{:02}: {}", show_name, season, episode, e);
                                        }
                                    }
                                    Err(e) => warn!("Failed to create season for '{}' S{:02}: {}", show_name, season, e),
                                }
                                Some(show_id)
                            }
                            Err(e) => { warn!("Failed to create TV show '{}': {}", show_name, e); None }
                        }
                    } else { None }
                } else { None };

                tx.commit().await?;
                drop(_write_permit);
                scan_state.inc_inserted();

                // Collect embedded subtitle stream descriptors for subtitle phase
                let embedded_streams: Vec<extract::EmbeddedSubtitleStream> = streams
                    .iter()
                    .filter(|s| s.stream_type == "subtitle")
                    .filter(|s| s.codec_name.as_deref().map(extract::is_extractable_subtitle).unwrap_or(false))
                    .map(|s| extract::EmbeddedSubtitleStream {
                        stream_index: s.stream_index,
                        codec_name: s.codec_name.clone().unwrap_or_default(),
                        language: s.language.clone(),
                        title: s.title.clone(),
                        is_default: s.is_default,
                        is_forced: s.is_forced,
                    })
                    .collect();

                // Return item info needed for Phase 2 subtitle work
                Ok(Some((mid, file_path_str, title, embedded_streams)))
            }
        })
        .buffer_unordered(concurrent_probes * 2)
        .collect::<Vec<Result<Option<(String, String, String, Vec<extract::EmbeddedSubtitleStream>)>>>>()
        .await;

    for r in &phase1_results {
        match r {
            Ok(Some(_)) => count += 1,
            Ok(None) => {}
            Err(e) => {
                warn!("Item processing error: {}", e);
                scan_state.inc_errors();
            }
        }
    }

    library_repo::update_last_scanned(pool, library_id).await?;

    if is_tv_library {
        let empty_seasons = tv_repo::delete_empty_seasons(pool).await.unwrap_or(0);
        let empty_shows = tv_repo::delete_empty_shows(pool).await.unwrap_or(0);
        if empty_seasons > 0 || empty_shows > 0 {
            info!(
                "Cleaned up {} orphaned season(s) and {} orphaned show(s) after scan of '{}'",
                empty_seasons, empty_shows, library.name
            );
        }
    }

    info!(
        "Phase 1 complete for '{}': {} new items indexed",
        library.name, count
    );

    // ── Phase 2: metadata enrichment (runs AFTER all files are committed) ─────
    // This is critical: enrichment must happen after all episodes are in the DB
    // so that enrich_library_shows sees every season and episode for each show.
    // Previously, enrichment was fire-and-forget during Phase 1, causing a race
    // where shows were enriched before all their episodes were committed.
    if let (Some(provider), Some(img_cache)) = (tmdb_provider.as_ref(), image_cache.as_ref()) {
        scan_state.set_status(ScanStatus::Enriching).await;

        if is_tv_library {
            scan_state.set_current("Enriching TV show metadata...").await;
            match ferrite_metadata::enrichment::enrich_library_shows(
                pool,
                library_id,
                provider.as_ref(),
                img_cache.as_ref(),
            )
            .await
            {
                Ok(n) => {
                    if n > 0 {
                        info!("Enriched {} TV show(s) in '{}'", n, library.name);
                    }
                }
                Err(e) => warn!("TV enrichment failed for '{}': {}", library.name, e),
            }
        } else if is_movie_library {
            scan_state.set_current("Enriching movie metadata...").await;
            match ferrite_metadata::enrichment::enrich_library_movies(
                pool,
                library_id,
                provider.as_ref(),
                img_cache.as_ref(),
            )
            .await
            {
                Ok(n) => {
                    if n > 0 {
                        info!("Enriched {} movie(s) in '{}'", n, library.name);
                    }
                }
                Err(e) => warn!("Movie enrichment failed for '{}': {}", library.name, e),
            }
        }
    }

    // Mark library as fully indexed — items are now visible in the UI.
    scan_state.set_status(ScanStatus::Complete).await;
    info!(
        "Scan complete for '{}': {} new items indexed",
        library.name, count
    );

    // ── Phase 3: subtitle extraction (runs after library is fully visible) ────
    // Collect items that need subtitle work (new/changed files only).
    let subtitle_items: Vec<(String, String, String, Vec<extract::EmbeddedSubtitleStream>)> =
        phase1_results
            .into_iter()
            .filter_map(|r| r.ok().flatten())
            .collect();

    if !subtitle_items.is_empty() {
        info!(
            "Starting subtitle extraction for {} item(s) in '{}'",
            subtitle_items.len(),
            library.name
        );
        scan_state.set_status(ScanStatus::Subtitles).await;

        let subtitle_results: Vec<()> = stream::iter(subtitle_items)
            .map(|item| {
                let (media_item_id, file_path_str, title, embedded_streams) = item;
                let pool = pool.clone();
                let sub_sem = sub_sem.clone();
                let write_sem = write_sem.clone();
                let ffmpeg = ffmpeg_path.to_string();
                let subtitle_cache_dir = subtitle_cache_dir.to_path_buf();
                let scan_state = scan_state.clone();

                async move {
                    let mut all_subs =
                        subtitle::find_external_subtitles(Path::new(&file_path_str)).await;

                    if !embedded_streams.is_empty() {
                        let _permit = sub_sem.acquire().await.expect("semaphore closed");
                        scan_state
                            .set_current(&format!("Extracting subtitles: {}", title))
                            .await;
                        let extracted = extract::extract_embedded_subtitles(
                            &ffmpeg,
                            Path::new(&file_path_str),
                            &embedded_streams,
                            &subtitle_cache_dir,
                            &media_item_id,
                        )
                        .await;
                        scan_state.inc_subtitles(extracted.len() as u32);
                        all_subs.extend(extracted);
                    }

                    if !all_subs.is_empty() {
                        let _write_permit = write_sem.acquire().await.expect("semaphore closed");
                        if let Err(e) = ferrite_db::subtitle_repo::replace_subtitles(
                            &pool,
                            &media_item_id,
                            &all_subs,
                        )
                        .await
                        {
                            warn!("Failed to store subtitles for '{}': {}", title, e);
                        }
                    }
                }
            })
            .buffer_unordered(concurrent_probes * 2)
            .collect()
            .await;

        let _ = subtitle_results;
        info!("Subtitle extraction complete for '{}'", library.name);
        scan_state.set_status(ScanStatus::Complete).await;
    }

    Ok(count)
}

/// Incremental, path-scoped scan for watcher change bursts.
///
/// Unlike `scan_library`, this routine avoids walking the whole library tree and
/// only processes media file paths reported by the watcher.
#[allow(clippy::too_many_arguments)]
pub async fn scan_library_incremental(
    pool: &SqlitePool,
    library_id: &str,
    ffprobe_path: &str,
    ffmpeg_path: &str,
    _concurrent_probes: usize,
    subtitle_cache_dir: &Path,
    changed_paths: &[PathBuf],
) -> Result<u32> {
    let library = library_repo::get_library(pool, library_id).await?;
    let lib_path = Path::new(&library.path);

    if !lib_path.exists() {
        warn!("Library path does not exist: {}", library.path);
        return Ok(0);
    }

    let extensions: &[&str] = match library.library_type {
        LibraryType::Movie | LibraryType::Tv => VIDEO_EXTENSIONS,
        LibraryType::Music => AUDIO_EXTENSIONS,
    };

    let media_type = match library.library_type {
        LibraryType::Movie => "movie",
        LibraryType::Tv => "episode",
        LibraryType::Music => "track",
    };

    let existing: HashMap<String, u64> = media_repo::get_all_file_sizes(pool, library_id)
        .await
        .unwrap_or_default();

    let is_movie_library = matches!(library.library_type, LibraryType::Movie);
    let is_tv_library = matches!(library.library_type, LibraryType::Tv);

    let unique_paths: HashSet<PathBuf> = changed_paths
        .iter()
        .filter(|p| p.starts_with(lib_path))
        .cloned()
        .collect();

    let mut pending_paths: Vec<PathBuf> = unique_paths.into_iter().collect();
    let mut visited_paths: HashSet<PathBuf> = HashSet::new();

    let mut indexed_count = 0u32;
    let mut removed_count = 0u32;

    while let Some(path) = pending_paths.pop() {
        if !visited_paths.insert(path.clone()) {
            continue;
        }

        let file_path_str = path.to_string_lossy().to_string();

        if !path.exists() {
            let ext = path
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| e.to_ascii_lowercase());
            let removed_rows = if ext.as_deref().is_some_and(|e| extensions.contains(&e)) {
                media_repo::delete_media_item_by_path(pool, &file_path_str).await
            } else {
                media_repo::delete_media_items_by_path_prefix(pool, &file_path_str).await
            };

            match removed_rows {
                Ok(rows) if rows > 0 => {
                    removed_count = removed_count.saturating_add(rows as u32);
                    info!(
                        "Incremental scan removed {} row(s) for missing path: {}",
                        rows, file_path_str
                    );
                }
                Ok(_) => {}
                Err(e) => warn!(
                    "Failed to remove media for missing path '{}': {}",
                    file_path_str, e
                ),
            }
            continue;
        }

        let metadata = match tokio::fs::metadata(&path).await {
            Ok(m) if m.is_file() => m,
            Ok(m) if m.is_dir() => {
                if path.as_path() == lib_path {
                    debug!(
                        "Skipping library-root directory change during incremental scan: {}",
                        path.display()
                    );
                    continue;
                }

                debug!(
                    "Expanding changed directory during incremental scan: {}",
                    path.display()
                );

                match walker::walk_directory(&path, extensions).await {
                    Ok(files) => {
                        pending_paths.extend(files.into_iter().map(|f| f.path));
                    }
                    Err(e) => {
                        warn!("Failed to walk changed directory {}: {}", path.display(), e);
                    }
                }
                continue;
            }
            Ok(_) => continue,
            Err(e) => {
                warn!(
                    "Failed to read metadata for changed path {}: {}",
                    file_path_str, e
                );
                continue;
            }
        };

        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_ascii_lowercase());
        let is_media_path = ext.as_deref().is_some_and(|e| extensions.contains(&e));
        if !is_media_path {
            continue;
        }

        let already_indexed = existing
            .get(&file_path_str)
            .map(|&sz| sz == metadata.len())
            .unwrap_or(false);

        if already_indexed {
            debug!(
                "Skipping unchanged media path in incremental scan: {}",
                file_path_str
            );
            continue;
        }

        let file_stem = path
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();
        let parsed = filename::parse_filename(&file_stem);
        let (title, year) = match &parsed {
            ParsedFilename::Movie(ParsedMovie { title, year }) => (title.clone(), *year),
            ParsedFilename::Episode(ParsedEpisode { show_name, .. }) => (show_name.clone(), None),
            ParsedFilename::Unknown(name) => (name.clone(), None),
        };

        let (probe_data, streams, chapters, keyframe_index_ms) =
            match probe::probe_file(ffprobe_path, &path).await {
                Ok(pr) => {
                    let stream_inserts: Vec<StreamInsert> = pr
                        .streams
                        .iter()
                        .map(|s| StreamInsert {
                            stream_index: s.index,
                            stream_type: s.stream_type.clone(),
                            codec_name: s.codec_name.clone(),
                            codec_long_name: s.codec_long_name.clone(),
                            profile: s.profile.clone(),
                            language: s.language.clone(),
                            title: s.title.clone(),
                            is_default: s.is_default,
                            is_forced: s.is_forced,
                            width: s.width,
                            height: s.height,
                            frame_rate: s.frame_rate.clone(),
                            pixel_format: s.pixel_format.clone(),
                            bit_depth: s.bit_depth,
                            color_space: s.color_space.clone(),
                            color_transfer: s.color_transfer.clone(),
                            color_primaries: s.color_primaries.clone(),
                            channels: s.channels,
                            channel_layout: s.channel_layout.clone(),
                            sample_rate: s.sample_rate,
                            bitrate_bps: s.bitrate_bps,
                        })
                        .collect();
                    let chapter_inserts: Vec<ChapterInsert> = pr
                        .chapters
                        .iter()
                        .map(|c| ChapterInsert {
                            chapter_index: c.chapter_index,
                            title: c.title.clone(),
                            start_time_ms: c.start_time_ms,
                            end_time_ms: c.end_time_ms,
                        })
                        .collect();
                    let data = MediaProbeData {
                        container_format: pr.container_format,
                        video_codec: pr.video_codec,
                        audio_codec: pr.audio_codec,
                        width: pr.width,
                        height: pr.height,
                        duration_ms: pr.duration_ms,
                        bitrate_kbps: pr.bitrate_kbps,
                    };

                    (
                        Some(data),
                        stream_inserts,
                        chapter_inserts,
                        pr.keyframe_index_ms,
                    )
                }
                Err(e) => {
                    warn!("ffprobe failed for changed path {}: {}", path.display(), e);
                    (None, Vec::new(), Vec::new(), None)
                }
            };

        let mut tx = pool.begin().await?;

        let mid = media_repo::insert_media_item(
            &mut *tx,
            &library.id,
            media_type,
            &file_path_str,
            metadata.len(),
            Some(&title),
            year,
            probe_data.as_ref(),
        )
        .await?;

        if !streams.is_empty() {
            if let Err(e) = ferrite_db::stream_repo::replace_streams(&mut *tx, &mid, &streams).await
            {
                warn!("Failed to store streams for '{}': {}", title, e);
            }
        }
        if !chapters.is_empty() {
            if let Err(e) =
                ferrite_db::chapter_repo::replace_chapters(&mut *tx, &mid, &chapters).await
            {
                warn!("Failed to store chapters for '{}': {}", title, e);
            }
        }
        if let Some(keyframes_ms) = keyframe_index_ms.as_ref() {
            if let Err(e) =
                ferrite_db::keyframe_repo::replace_keyframes(&mut *tx, &mid, keyframes_ms).await
            {
                warn!("Failed to store keyframe index for '{}': {}", title, e);
            }
        }

        if is_movie_library {
            if let Err(e) =
                movie_repo::upsert_movie_skeleton(&mut *tx, &mid, &title, year.map(|y| y as i64))
                    .await
            {
                warn!("Failed to create movie skeleton for '{}': {}", title, e);
            }
        }

        if is_tv_library {
            if let ParsedFilename::Episode(ParsedEpisode {
                show_name,
                season,
                episode,
            }) = &parsed
            {
                match tv_repo::upsert_tv_show(&mut *tx, library_id, show_name).await {
                    Ok(show_id) => {
                        match tv_repo::upsert_season(&mut *tx, &show_id, *season).await {
                            Ok(season_id) => {
                                if let Err(e) =
                                    tv_repo::upsert_episode(&mut *tx, &mid, &season_id, *episode)
                                        .await
                                {
                                    warn!(
                                        "Failed to create episode for '{}' S{:02}E{:02}: {}",
                                        show_name, season, episode, e
                                    );
                                }
                            }
                            Err(e) => warn!(
                                "Failed to create season for '{}' S{:02}: {}",
                                show_name, season, e
                            ),
                        }
                    }
                    Err(e) => warn!("Failed to create TV show '{}': {}", show_name, e),
                }
            }
        }

        tx.commit().await?;
        indexed_count = indexed_count.saturating_add(1);

        let mut all_subtitles = subtitle::find_external_subtitles(&path).await;
        let embedded_streams: Vec<extract::EmbeddedSubtitleStream> = streams
            .iter()
            .filter(|s| s.stream_type == "subtitle")
            .filter(|s| {
                s.codec_name
                    .as_deref()
                    .map(extract::is_extractable_subtitle)
                    .unwrap_or(false)
            })
            .map(|s| extract::EmbeddedSubtitleStream {
                stream_index: s.stream_index,
                codec_name: s.codec_name.clone().unwrap_or_default(),
                language: s.language.clone(),
                title: s.title.clone(),
                is_default: s.is_default,
                is_forced: s.is_forced,
            })
            .collect();

        if !embedded_streams.is_empty() {
            let extracted = extract::extract_embedded_subtitles(
                ffmpeg_path,
                &path,
                &embedded_streams,
                subtitle_cache_dir,
                &mid,
            )
            .await;
            all_subtitles.extend(extracted);
        }

        if !all_subtitles.is_empty() {
            if let Err(e) =
                ferrite_db::subtitle_repo::replace_subtitles(pool, &mid, &all_subtitles).await
            {
                warn!("Failed to store subtitles for '{}': {}", title, e);
            }
        }
    }

    if is_tv_library {
        let empty_seasons = tv_repo::delete_empty_seasons(pool).await.unwrap_or(0);
        let empty_shows = tv_repo::delete_empty_shows(pool).await.unwrap_or(0);
        if empty_seasons > 0 || empty_shows > 0 {
            info!(
                "Incremental cleanup removed {} orphaned season(s) and {} orphaned show(s)",
                empty_seasons, empty_shows
            );
        }
    }

    if indexed_count > 0 || removed_count > 0 {
        library_repo::update_last_scanned(pool, library_id).await?;
    }

    info!(
        "Incremental scan complete for '{}': indexed={} removed={}",
        library.name, indexed_count, removed_count
    );

    Ok(indexed_count.saturating_add(removed_count))
}
