pub mod extract;
pub mod filename;
pub mod probe;
pub mod subtitle;
pub mod walker;
pub mod watcher;

use anyhow::Result;
use ferrite_core::media::{LibraryType, VIDEO_EXTENSIONS, AUDIO_EXTENSIONS};
use ferrite_db::chapter_repo::ChapterInsert;
use ferrite_db::library_repo;
use ferrite_db::media_repo::{self, MediaProbeData};
use ferrite_db::movie_repo;
use ferrite_db::stream_repo::StreamInsert;
use ferrite_db::tv_repo;
use filename::{ParsedFilename, ParsedMovie, ParsedEpisode};
use futures::stream::{self, StreamExt};
use sqlx::SqlitePool;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tracing::{info, warn};

/// Parsed + probed result for a single discovered file, ready for DB insertion.
struct ScannedFile {
    file_path: String,
    file_size: u64,
    title: String,
    year: Option<i32>,
    parsed: ParsedFilename,
    probe_data: Option<MediaProbeData>,
    /// Individual streams (video, audio, subtitle) discovered by ffprobe.
    streams: Vec<StreamInsert>,
    /// Chapter markers embedded in the container.
    chapters: Vec<ChapterInsert>,
}

/// Scan a single library: walk the directory, identify media files, probe them, insert into DB.
/// For movie libraries, also creates skeleton movie records for metadata enrichment.
/// `concurrent_probes` controls how many ffprobe processes run in parallel.
pub async fn scan_library(
    pool: &SqlitePool,
    library_id: &str,
    ffprobe_path: &str,
    ffmpeg_path: &str,
    concurrent_probes: usize,
    subtitle_cache_dir: &Path,
) -> Result<u32> {
    let library = library_repo::get_library(pool, library_id).await?;
    let lib_path = Path::new(&library.path);

    if !lib_path.exists() {
        warn!("Library path does not exist: {}", library.path);
        return Ok(0);
    }

    info!("Scanning library '{}' at {}", library.name, library.path);

    let extensions: &[&str] = match library.library_type {
        LibraryType::Movie | LibraryType::Tv => VIDEO_EXTENSIONS,
        LibraryType::Music => AUDIO_EXTENSIONS,
    };

    let files = walker::walk_directory(lib_path, extensions).await?;
    info!("Found {} media files in '{}'", files.len(), library.name);

    let media_type = match library.library_type {
        LibraryType::Movie => "movie",
        LibraryType::Tv => "episode",
        LibraryType::Music => "track",
    };

    let is_movie_library = matches!(library.library_type, LibraryType::Movie);
    let is_tv_library = matches!(library.library_type, LibraryType::Tv);

    // Probe files concurrently using buffer_unordered.
    // The semaphore limits how many ffprobe processes run at once.
    let semaphore = Arc::new(Semaphore::new(concurrent_probes));

    let scanned_files: Vec<ScannedFile> = stream::iter(files)
        .map(|file| {
            let sem = semaphore.clone();
            let ffprobe = ffprobe_path.to_string();
            async move {
                let file_path_str = file.path.to_string_lossy().to_string();
                let file_stem = file
                    .path
                    .file_stem()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_default();

                // Parse the filename to extract title, year, and episode info
                let parsed = filename::parse_filename(&file_stem);
                let (title, year) = match &parsed {
                    ParsedFilename::Movie(ParsedMovie { title, year }) => {
                        (title.clone(), *year)
                    }
                    ParsedFilename::Episode(ParsedEpisode { show_name, .. }) => {
                        (show_name.clone(), None)
                    }
                    ParsedFilename::Unknown(name) => {
                        (name.clone(), None)
                    }
                };

                // Acquire semaphore permit before spawning ffprobe
                let _permit = sem.acquire().await.expect("semaphore closed unexpectedly");
                let probe_result = probe::probe_file(&ffprobe, &file.path).await;

                let (probe_data, streams, chapters) = match probe_result {
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

                        let chapter_inserts: Vec<ChapterInsert> = pr.chapters
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
                        (Some(data), stream_inserts, chapter_inserts)
                    }
                    Err(e) => {
                        warn!("ffprobe failed for {}: {}", file.path.display(), e);
                        (None, Vec::new(), Vec::new())
                    }
                };

                ScannedFile {
                    file_path: file_path_str,
                    file_size: file.size,
                    title,
                    year,
                    parsed,
                    probe_data,
                    streams,
                    chapters,
                }
            }
        })
        .buffer_unordered(concurrent_probes)
        .collect()
        .await;

    // Insert all results into the database inside a single transaction.
    // Batching all writes into one transaction is 10-50x faster than individual
    // auto-commit inserts because SQLite only syncs to disk once at commit time.
    //
    // Subtitle extraction (ffmpeg subprocesses) must happen outside the transaction
    // since it is I/O-bound and can take seconds per file and must not hold the
    // write lock. It runs in Phase 2 after the transaction commits.
    let mut count = 0u32;

    {
        let mut tx = pool.begin().await?;

        for scanned in &scanned_files {
            let media_item_id = media_repo::insert_media_item(
                &mut *tx,
                &library.id,
                media_type,
                &scanned.file_path,
                scanned.file_size,
                Some(&scanned.title),
                scanned.year,
                scanned.probe_data.as_ref(),
            )
            .await?;

            // Store individual stream tracks (video, audio, subtitle) from ffprobe
            if !scanned.streams.is_empty() {
                if let Err(e) =
                    ferrite_db::stream_repo::replace_streams(
                        &mut *tx,
                        &media_item_id,
                        &scanned.streams,
                    )
                    .await
                {
                    warn!(
                        "Failed to store streams for '{}': {}",
                        scanned.title, e
                    );
                }
            }

            // Store chapter markers
            if !scanned.chapters.is_empty() {
                if let Err(e) =
                    ferrite_db::chapter_repo::replace_chapters(
                        &mut *tx,
                        &media_item_id,
                        &scanned.chapters,
                    )
                    .await
                {
                    warn!("Failed to store chapters for '{}': {}", scanned.title, e);
                }
            }

            // For movie libraries, create skeleton movie records for later metadata enrichment
            if is_movie_library {
                if let Err(e) = movie_repo::upsert_movie_skeleton(
                    &mut *tx,
                    &media_item_id,
                    &scanned.title,
                    scanned.year.map(|y| y as i64),
                )
                .await
                {
                    warn!("Failed to create movie skeleton for '{}': {}", scanned.title, e);
                }
            }

            // For TV libraries, create show → season → episode hierarchy
            if is_tv_library {
                if let ParsedFilename::Episode(ParsedEpisode {
                    show_name,
                    season,
                    episode,
                }) = &scanned.parsed
                {
                    let lib_id_str = library.id.to_string();
                    match tv_repo::upsert_tv_show(&mut *tx, &lib_id_str, show_name).await {
                        Ok(show_id) => {
                            match tv_repo::upsert_season(&mut *tx, &show_id, *season).await {
                                Ok(season_id) => {
                                    if let Err(e) = tv_repo::upsert_episode(
                                        &mut *tx,
                                        &media_item_id,
                                        &season_id,
                                        *episode,
                                    )
                                    .await
                                    {
                                        warn!(
                                            "Failed to create episode record for '{}' S{:02}E{:02}: {}",
                                            show_name, season, episode, e
                                        );
                                    }
                                }
                                Err(e) => {
                                    warn!(
                                        "Failed to create season for '{}' S{:02}: {}",
                                        show_name, season, e
                                    );
                                }
                            }
                        }
                        Err(e) => {
                            warn!("Failed to create TV show for '{}': {}", show_name, e);
                        }
                    }
                }
            }

            count += 1;
        }

        tx.commit().await?;
    }

    // Phase 2 (outside transaction): subtitle extraction and DB writes.
    // Subtitle ffmpeg extraction is slow (seconds per file) — done after the main
    // transaction commits so we don't hold the write lock during subprocess I/O.
    for scanned in &scanned_files {
        // Re-fetch the media_item_id by file_path (it was just inserted above).
        let media_item_id = match media_repo::get_media_item_id_by_path(pool, &scanned.file_path).await {
            Ok(Some(id)) => id,
            _ => continue,
        };

        // Detect external subtitle files next to the media file
        let mut all_subs =
            subtitle::find_external_subtitles(Path::new(&scanned.file_path)).await;
        if !all_subs.is_empty() {
            info!(
                "Found {} external subtitle(s) for '{}'",
                all_subs.len(),
                scanned.title
            );
        }

        // Extract embedded text-based subtitles (SRT/ASS/SSA) from the container
        let embedded_streams: Vec<extract::EmbeddedSubtitleStream> = scanned
            .streams
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
            info!(
                "Extracting {} embedded subtitle(s) from '{}'",
                embedded_streams.len(),
                scanned.title
            );
            let extracted = extract::extract_embedded_subtitles(
                ffmpeg_path,
                Path::new(&scanned.file_path),
                &embedded_streams,
                subtitle_cache_dir,
                &media_item_id,
            )
            .await;
            if !extracted.is_empty() {
                info!(
                    "Extracted {} embedded subtitle(s) for '{}'",
                    extracted.len(),
                    scanned.title
                );
                all_subs.extend(extracted);
            }
        }

        if !all_subs.is_empty() {
            if let Err(e) =
                ferrite_db::subtitle_repo::replace_subtitles(pool, &media_item_id, &all_subs)
                    .await
            {
                warn!(
                    "Failed to store subtitles for '{}': {}",
                    scanned.title, e
                );
            }
        }
    }

    library_repo::update_last_scanned(pool, library_id).await?;

    // Clean up orphaned seasons/shows left over from title-merge deduplication
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

    info!("Scan complete for '{}': {} items indexed", library.name, count);

    Ok(count)
}
