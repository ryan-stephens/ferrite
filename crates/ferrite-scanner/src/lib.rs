pub mod extract;
pub mod filename;
pub mod probe;
pub mod progress;
pub mod subtitle;
pub mod walker;
pub mod watcher;

use anyhow::Result;
use dashmap::DashSet;
use ferrite_core::media::{LibraryType, VIDEO_EXTENSIONS, AUDIO_EXTENSIONS};
use ferrite_db::chapter_repo::ChapterInsert;
use ferrite_db::library_repo;
use ferrite_db::media_repo::{self, MediaProbeData};
use ferrite_db::movie_repo;
use ferrite_db::stream_repo::StreamInsert;
use ferrite_db::tv_repo;
use filename::{ParsedFilename, ParsedMovie, ParsedEpisode};
use futures::stream::{self, StreamExt};
use progress::{ScanState, ScanStatus};
use sqlx::SqlitePool;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tracing::{debug, info, warn};

pub use progress::{ScanProgress, ScanRegistry};

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
    scan_state.total_files.store(total, std::sync::atomic::Ordering::Relaxed);

    let media_type = match library.library_type {
        LibraryType::Movie => "movie",
        LibraryType::Tv => "episode",
        LibraryType::Music => "track",
    };

    let is_movie_library = matches!(library.library_type, LibraryType::Movie);
    let is_tv_library = matches!(library.library_type, LibraryType::Tv);

    // Delta scan: load existing (file_path -> file_size) to skip unchanged files.
    // Wrapped in Arc so it is shared across all per-item futures without cloning.
    let existing: Arc<std::collections::HashMap<String, u64>> =
        Arc::new(media_repo::get_all_file_sizes(pool, library_id).await.unwrap_or_default());

    let probe_sem = Arc::new(Semaphore::new(concurrent_probes));
    let sub_sem = Arc::new(Semaphore::new(concurrent_probes));
    // Limit concurrent DB write transactions to avoid SQLite write-lock contention.
    // ffprobe and ffmpeg work proceeds freely; only the DB commit is gated.
    let write_sem = Arc::new(Semaphore::new(2));

    // Track which TV shows have been enriched this scan to avoid duplicate TMDB calls
    let enriched_shows: Arc<DashSet<String>> = Arc::new(DashSet::new());

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
            let enriched_shows = enriched_shows.clone();
            let tmdb_provider = tmdb_provider.clone();
            let image_cache = image_cache.clone();
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

                let (probe_data, streams, chapters) = {
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
                            (Some(data), stream_inserts, chapter_inserts)
                        }
                        Err(e) => {
                            warn!("ffprobe failed for {}: {}", file.path.display(), e);
                            scan_state.inc_errors();
                            (None, Vec::new(), Vec::new())
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
                if is_movie_library {
                    if let Err(e) = movie_repo::upsert_movie_skeleton(&mut *tx, &mid, &title, year.map(|y| y as i64)).await {
                        warn!("Failed to create movie skeleton for '{}': {}", title, e);
                    }
                }

                let show_id_for_enrich: Option<String> = if is_tv_library {
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

                // Spawn TMDB enrichment as a detached task so it doesn't occupy a
                // buffer_unordered slot while waiting on HTTP responses.
                if let (Some(provider), Some(img_cache)) = (tmdb_provider.clone(), image_cache.clone()) {
                    if is_tv_library {
                        if let (Some(show_id), ParsedFilename::Episode(ParsedEpisode { show_name, .. })) = (show_id_for_enrich, &parsed) {
                            if enriched_shows.insert(show_id.clone()) {
                                let pool2 = pool.clone();
                                let show_name2 = show_name.clone();
                                let scan_state2 = scan_state.clone();
                                let write_sem2 = write_sem.clone();
                                tokio::spawn(async move {
                                    match ferrite_metadata::enrichment::enrich_single_show(&pool2, &show_id, &show_name2, provider.as_ref(), img_cache.as_ref(), &write_sem2).await {
                                        Ok(true) => { scan_state2.inc_enriched(); }
                                        Ok(false) => {}
                                        Err(e) => { warn!("TMDB enrichment failed for '{}': {}", show_name2, e); }
                                    }
                                });
                            }
                        }
                    } else if is_movie_library {
                        let pool2 = pool.clone();
                        let title2 = title.clone();
                        let mid2 = mid.clone();
                        let scan_state2 = scan_state.clone();
                        let write_sem2 = write_sem.clone();
                        tokio::spawn(async move {
                            match ferrite_metadata::enrichment::enrich_single_movie(&pool2, &mid2, &title2, year.map(|y| y as i32), provider.as_ref(), img_cache.as_ref(), &write_sem2).await {
                                Ok(true) => { scan_state2.inc_enriched(); }
                                Ok(false) => {}
                                Err(e) => { warn!("TMDB enrichment failed for '{}': {}", title2, e); }
                            }
                        });
                    }
                }

                // Collect embedded subtitle stream descriptors for Phase 2
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

    // Mark library as fully indexed — items are now visible in the UI.
    scan_state.set_status(ScanStatus::Complete).await;
    info!("Scan complete for '{}': {} new items indexed", library.name, count);

    // ── Phase 2: subtitle extraction (runs after library is fully visible) ────
    // Collect items that need subtitle work (new/changed files only).
    let subtitle_items: Vec<(String, String, String, Vec<extract::EmbeddedSubtitleStream>)> = phase1_results
        .into_iter()
        .filter_map(|r| r.ok().flatten())
        .collect();

    if !subtitle_items.is_empty() {
        info!("Starting subtitle extraction for {} item(s) in '{}'", subtitle_items.len(), library.name);
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
                    let mut all_subs = subtitle::find_external_subtitles(Path::new(&file_path_str)).await;

                    if !embedded_streams.is_empty() {
                        let _permit = sub_sem.acquire().await.expect("semaphore closed");
                        scan_state.set_current(&format!("Extracting subtitles: {}", title)).await;
                        let extracted = extract::extract_embedded_subtitles(
                            &ffmpeg,
                            Path::new(&file_path_str),
                            &embedded_streams,
                            &subtitle_cache_dir,
                            &media_item_id,
                        ).await;
                        scan_state.inc_subtitles(extracted.len() as u32);
                        all_subs.extend(extracted);
                    }

                    if !all_subs.is_empty() {
                        let _write_permit = write_sem.acquire().await.expect("semaphore closed");
                        if let Err(e) = ferrite_db::subtitle_repo::replace_subtitles(&pool, &media_item_id, &all_subs).await {
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
