use anyhow::Result;
use ferrite_db::library_repo;
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use sqlx::SqlitePool;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tokio::sync::mpsc;
use tracing::{debug, info, warn};

pub struct LibraryWatcher {
    pool: SqlitePool,
    ffprobe_path: String,
    ffmpeg_path: String,
    debounce_seconds: u64,
    concurrent_probes: usize,
    subtitle_cache_dir: PathBuf,
}

impl LibraryWatcher {
    pub fn new(pool: SqlitePool, ffprobe_path: String, ffmpeg_path: String, debounce_seconds: u64, concurrent_probes: usize, subtitle_cache_dir: PathBuf) -> Self {
        Self {
            pool,
            ffprobe_path,
            ffmpeg_path,
            debounce_seconds,
            concurrent_probes,
            subtitle_cache_dir,
        }
    }

    pub async fn start(self) -> Result<tokio::task::JoinHandle<()>> {
        let libraries = library_repo::list_libraries(&self.pool).await?;

        // Bridge from notify's std::sync::mpsc to tokio's async mpsc.
        let (async_tx, mut async_rx) = mpsc::channel::<PathBuf>(256);

        let (sync_tx, sync_rx) = std::sync::mpsc::channel::<Result<Event, notify::Error>>();
        let mut watcher = RecommendedWatcher::new(sync_tx, notify::Config::default())?;

        // Build a lookup table mapping library directory paths to their IDs so we
        // can figure out which library a changed file belongs to.
        let mut lib_paths: Vec<(PathBuf, String)> = Vec::new();

        for lib in &libraries {
            let path = PathBuf::from(&lib.path);
            if path.exists() {
                if let Err(e) = watcher.watch(&path, RecursiveMode::Recursive) {
                    warn!("Failed to watch library '{}' at {}: {}", lib.name, lib.path, e);
                } else {
                    lib_paths.push((path, lib.id.to_string()));
                }
            } else {
                warn!("Library path does not exist, skipping watch: {}", lib.path);
            }
        }

        info!("Watching {} library directories for changes", lib_paths.len());

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

        let handle = tokio::spawn(async move {
            // Keep the watcher alive for the lifetime of this task.
            let _watcher = watcher;

            let mut pending_libraries: HashSet<String> = HashSet::new();

            loop {
                tokio::select! {
                    Some(changed_path) = async_rx.recv() => {
                        // Map the changed path back to the library it belongs to.
                        if let Some(lib_id) = find_library_for_path(&changed_path, &lib_paths) {
                            pending_libraries.insert(lib_id);
                        } else {
                            debug!(
                                "Changed path does not match any watched library: {:?}",
                                changed_path
                            );
                        }
                    }
                    _ = tokio::time::sleep(debounce), if !pending_libraries.is_empty() => {
                        // Debounce period elapsed with pending changes — trigger rescans.
                        let libs_to_scan: Vec<String> =
                            pending_libraries.drain().collect();

                        for lib_id in libs_to_scan {
                            info!("Re-scanning library '{}' due to filesystem changes", lib_id);
                            if let Err(e) =
                                crate::scan_library(&pool, &lib_id, &ffprobe_path, &ffmpeg_path, concurrent_probes, &subtitle_cache_dir).await
                            {
                                warn!("Failed to re-scan library '{}': {}", lib_id, e);
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

        Ok(handle)
    }
}

/// Given a file path that changed, find which library directory contains it and
/// return the corresponding library ID.
fn find_library_for_path(path: &Path, lib_paths: &[(PathBuf, String)]) -> Option<String> {
    for (lib_path, lib_id) in lib_paths {
        if path.starts_with(lib_path) {
            return Some(lib_id.clone());
        }
    }
    None
}
