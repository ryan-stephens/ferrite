use anyhow::Result;
use ferrite_core::media::LibraryType;
use ferrite_db::library_repo;
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use sqlx::SqlitePool;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tracing::{debug, info, warn};

const MAX_INCREMENTAL_BATCH_PATHS: usize = 256;

/// Commands sent to the running watcher task for dynamic library management.
pub enum WatcherCmd {
    /// Start watching a new library directory.
    Watch { library_id: String, path: PathBuf },
    /// Stop watching a library directory (e.g. on library deletion).
    Unwatch { library_id: String },
}

/// Handle returned by `LibraryWatcher::start()` that allows callers to
/// dynamically register or unregister library directories at runtime.
#[derive(Clone)]
pub struct WatcherHandle {
    cmd_tx: mpsc::Sender<WatcherCmd>,
}

impl WatcherHandle {
    /// Register a new library directory for filesystem watching.
    /// This is safe to call from any async context (e.g. an API handler).
    pub async fn watch_library(&self, library_id: String, path: PathBuf) {
        if let Err(e) = self
            .cmd_tx
            .send(WatcherCmd::Watch { library_id, path })
            .await
        {
            warn!("Failed to send watch command to watcher task: {}", e);
        }
    }

    /// Unregister a library directory so it is no longer watched.
    /// Also drains any pending filesystem events for this library.
    pub async fn unwatch_library(&self, library_id: String) {
        if let Err(e) = self.cmd_tx.send(WatcherCmd::Unwatch { library_id }).await {
            warn!("Failed to send unwatch command to watcher task: {}", e);
        }
    }
}

pub struct LibraryWatcher {
    pool: SqlitePool,
    ffprobe_path: String,
    ffmpeg_path: String,
    debounce_seconds: u64,
    concurrent_probes: usize,
    subtitle_cache_dir: PathBuf,
    tmdb_provider: Option<Arc<dyn ferrite_metadata::provider::MetadataProvider>>,
    image_cache: Option<Arc<ferrite_metadata::image_cache::ImageCache>>,
}

impl LibraryWatcher {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        pool: SqlitePool,
        ffprobe_path: String,
        ffmpeg_path: String,
        debounce_seconds: u64,
        concurrent_probes: usize,
        subtitle_cache_dir: PathBuf,
        tmdb_provider: Option<Arc<dyn ferrite_metadata::provider::MetadataProvider>>,
        image_cache: Option<Arc<ferrite_metadata::image_cache::ImageCache>>,
    ) -> Self {
        Self {
            pool,
            ffprobe_path,
            ffmpeg_path,
            debounce_seconds,
            concurrent_probes,
            subtitle_cache_dir,
            tmdb_provider,
            image_cache,
        }
    }

    pub async fn start(self) -> Result<WatcherHandle> {
        let libraries = library_repo::list_libraries(&self.pool).await?;

        // Bridge from notify's std::sync::mpsc to tokio's async mpsc.
        let (async_tx, mut async_rx) = mpsc::channel::<PathBuf>(256);

        // Command channel for dynamic library registration/unregistration at runtime.
        let (cmd_tx, mut cmd_rx) = mpsc::channel::<WatcherCmd>(16);

        let (sync_tx, sync_rx) = std::sync::mpsc::channel::<Result<Event, notify::Error>>();
        let mut watcher = RecommendedWatcher::new(sync_tx, notify::Config::default())?;

        // Build a lookup table mapping library directory paths to their IDs so we
        // can figure out which library a changed file belongs to.
        let mut lib_paths: Vec<(PathBuf, String)> = Vec::new();

        for lib in &libraries {
            let path = PathBuf::from(&lib.path);
            if path.exists() {
                if let Err(e) = watcher.watch(&path, RecursiveMode::Recursive) {
                    warn!(
                        "Failed to watch library '{}' at {}: {}",
                        lib.name, lib.path, e
                    );
                } else {
                    lib_paths.push((path, lib.id.to_string()));
                }
            } else {
                warn!("Library path does not exist, skipping watch: {}", lib.path);
            }
        }

        info!(
            "Watching {} library directories for changes",
            lib_paths.len()
        );

        // Spawn a blocking task to read from the synchronous notify channel and
        // forward relevant paths into the async channel.
        let bridge_tx = async_tx.clone();
        std::thread::spawn(move || {
            for result in sync_rx {
                match result {
                    Ok(event) => {
                        let dominated = matches!(
                            event.kind,
                            EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_)
                        );
                        if dominated {
                            for path in event.paths {
                                debug!("Filesystem event: {:?}", path);
                                if bridge_tx.blocking_send(path).is_err() {
                                    // The async receiver has been dropped; shut down.
                                    return;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        warn!("Filesystem watcher error: {}", e);
                    }
                }
            }
        });

        let pool = self.pool;
        let ffprobe_path = self.ffprobe_path;
        let ffmpeg_path = self.ffmpeg_path;
        let debounce = Duration::from_secs(self.debounce_seconds);
        let concurrent_probes = self.concurrent_probes;
        let subtitle_cache_dir = self.subtitle_cache_dir;
        let tmdb_provider = self.tmdb_provider;
        let image_cache = self.image_cache;

        tokio::spawn(async move {
            // Keep the watcher alive for the lifetime of this task.
            // Moved into the closure so we can mutably borrow it for
            // dynamic watch registration via commands.
            let mut watcher = watcher;

            let mut pending_libraries: HashMap<String, HashSet<PathBuf>> = HashMap::new();

            loop {
                tokio::select! {
                    Some(changed_path) = async_rx.recv() => {
                        // Map the changed path back to the library it belongs to.
                        if let Some(lib_id) = find_library_for_path(&changed_path, &lib_paths) {
                            pending_libraries
                                .entry(lib_id)
                                .or_default()
                                .insert(changed_path);
                        } else {
                            debug!(
                                "Changed path does not match any watched library: {:?}",
                                changed_path
                            );
                        }
                    }
                    Some(cmd) = cmd_rx.recv() => {
                        match cmd {
                            WatcherCmd::Watch { library_id, path } => {
                                if path.exists() {
                                    match watcher.watch(&path, RecursiveMode::Recursive) {
                                        Ok(()) => {
                                            info!("Now watching new library '{}' at {:?}", library_id, path);
                                            lib_paths.push((path, library_id));
                                        }
                                        Err(e) => {
                                            warn!("Failed to watch new library '{}' at {:?}: {}", library_id, path, e);
                                        }
                                    }
                                } else {
                                    warn!("New library path does not exist, skipping watch: {:?}", path);
                                }
                            }
                            WatcherCmd::Unwatch { library_id } => {
                                // Remove from lib_paths and unwatch the directory.
                                if let Some(pos) = lib_paths.iter().position(|(_, id)| id == &library_id) {
                                    let (path, _) = lib_paths.remove(pos);
                                    if let Err(e) = watcher.unwatch(&path) {
                                        warn!("Failed to unwatch library '{}' at {:?}: {}", library_id, path, e);
                                    } else {
                                        info!("Stopped watching deleted library '{}' at {:?}", library_id, path);
                                    }
                                }
                                // Drain any pending events for this library.
                                pending_libraries.remove(&library_id);
                            }
                        }
                    }
                    _ = tokio::time::sleep(debounce), if !pending_libraries.is_empty() => {
                        // Debounce period elapsed with pending changes — process batched path sets.
                        let batches: Vec<(String, Vec<PathBuf>)> = pending_libraries
                            .drain()
                            .map(|(lib_id, paths)| (lib_id, paths.into_iter().collect()))
                            .collect();

                        for (lib_id, paths) in batches {
                            info!(
                                "Incremental scan for library '{}' due to {} changed path(s)",
                                lib_id,
                                paths.len()
                            );
                            let mut incremental_failed = false;
                            let mut indexed_total = 0u32;
                            for chunk in paths.chunks(MAX_INCREMENTAL_BATCH_PATHS) {
                                match crate::scan_library_incremental(
                                    &pool,
                                    &lib_id,
                                    &ffprobe_path,
                                    &ffmpeg_path,
                                    concurrent_probes,
                                    &subtitle_cache_dir,
                                    chunk,
                                )
                                .await
                                {
                                    Ok(n) => indexed_total += n,
                                    Err(e) => {
                                        warn!(
                                            "Incremental scan failed for '{}': {}. Falling back to full rescan.",
                                            lib_id,
                                            e
                                        );
                                        incremental_failed = true;
                                        break;
                                    }
                                }
                            }

                            if incremental_failed {
                                let scan_state = crate::progress::ScanState::new(lib_id.clone());
                                if let Err(full_err) = crate::scan_library(
                                    &pool,
                                    &lib_id,
                                    &ffprobe_path,
                                    &ffmpeg_path,
                                    concurrent_probes,
                                    &subtitle_cache_dir,
                                    scan_state,
                                    tmdb_provider.clone(),
                                    image_cache.clone(),
                                )
                                .await
                                {
                                    warn!(
                                        "Failed to run full fallback scan for '{}': {}",
                                        lib_id,
                                        full_err
                                    );
                                }
                            } else if indexed_total > 0 {
                                // Enrich newly added items after incremental scan
                                enrich_library_after_scan(
                                    &pool,
                                    &lib_id,
                                    tmdb_provider.as_ref(),
                                    image_cache.as_ref(),
                                )
                                .await;
                            }
                        }
                    }
                    else => {
                        // Both the async channel is closed and there are no pending
                        // libraries, so we can exit.
                        info!("Library watcher shutting down");
                        break;
                    }
                }
            }
        });

        Ok(WatcherHandle { cmd_tx })
    }
}

/// Run metadata enrichment for a library after an incremental scan indexed new items.
/// Determines the library type and calls the appropriate enrichment function.
async fn enrich_library_after_scan(
    pool: &SqlitePool,
    library_id: &str,
    tmdb_provider: Option<&Arc<dyn ferrite_metadata::provider::MetadataProvider>>,
    image_cache: Option<&Arc<ferrite_metadata::image_cache::ImageCache>>,
) {
    let (provider, cache) = match (tmdb_provider, image_cache) {
        (Some(p), Some(c)) => (p, c),
        _ => return, // No metadata provider configured
    };

    let library = match library_repo::get_library(pool, library_id).await {
        Ok(lib) => lib,
        Err(e) => {
            warn!(
                "Failed to load library '{}' for enrichment: {}",
                library_id, e
            );
            return;
        }
    };

    match library.library_type {
        LibraryType::Tv => {
            info!(
                "Enriching TV metadata after incremental scan for '{}'",
                library.name
            );
            match ferrite_metadata::enrichment::enrich_library_shows(
                pool,
                library_id,
                provider.as_ref(),
                cache.as_ref(),
            )
            .await
            {
                Ok(n) if n > 0 => info!("Enriched {} TV show(s) in '{}'", n, library.name),
                Ok(_) => {}
                Err(e) => warn!("TV enrichment failed for '{}': {}", library.name, e),
            }
        }
        LibraryType::Movie => {
            info!(
                "Enriching movie metadata after incremental scan for '{}'",
                library.name
            );
            match ferrite_metadata::enrichment::enrich_library_movies(
                pool,
                library_id,
                provider.as_ref(),
                cache.as_ref(),
            )
            .await
            {
                Ok(n) if n > 0 => info!("Enriched {} movie(s) in '{}'", n, library.name),
                Ok(_) => {}
                Err(e) => warn!("Movie enrichment failed for '{}': {}", library.name, e),
            }
        }
        LibraryType::Music => {} // No metadata enrichment for music libraries yet
    }
}

/// Given a file path that changed, find which library directory contains it and
/// return the corresponding library ID.
fn find_library_for_path(path: &Path, lib_paths: &[(PathBuf, String)]) -> Option<String> {
    lib_paths
        .iter()
        .filter(|(lib_path, _)| path.starts_with(lib_path))
        .max_by_key(|(lib_path, _)| lib_path.components().count())
        .map(|(_, lib_id)| lib_id.clone())
}

#[cfg(test)]
mod tests {
    use super::find_library_for_path;
    use std::path::{Path, PathBuf};

    #[test]
    fn picks_most_specific_library_path_for_nested_roots() {
        let libs = vec![
            (PathBuf::from("/media"), "root".to_string()),
            (PathBuf::from("/media/tv"), "tv".to_string()),
        ];

        let matched = find_library_for_path(Path::new("/media/tv/show/ep1.mkv"), &libs);
        assert_eq!(matched.as_deref(), Some("tv"));
    }

    #[test]
    fn returns_none_when_path_is_outside_all_libraries() {
        let libs = vec![(PathBuf::from("/media"), "root".to_string())];
        let matched = find_library_for_path(Path::new("/other/movie.mkv"), &libs);
        assert!(matched.is_none());
    }
}
