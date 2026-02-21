use anyhow::Result;
use ferrite_db::library_repo;
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use sqlx::SqlitePool;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::time::Duration;
use tokio::sync::mpsc;
use tracing::{debug, info, warn};

const MAX_INCREMENTAL_BATCH_PATHS: usize = 256;

pub struct LibraryWatcher {
    pool: SqlitePool,
    ffprobe_path: String,
    ffmpeg_path: String,
    debounce_seconds: u64,
    concurrent_probes: usize,
    subtitle_cache_dir: PathBuf,
}

impl LibraryWatcher {
    pub fn new(
        pool: SqlitePool,
        ffprobe_path: String,
        ffmpeg_path: String,
        debounce_seconds: u64,
        concurrent_probes: usize,
        subtitle_cache_dir: PathBuf,
    ) -> Self {
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

        let handle = tokio::spawn(async move {
            // Keep the watcher alive for the lifetime of this task.
            let _watcher = watcher;

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
                            for chunk in paths.chunks(MAX_INCREMENTAL_BATCH_PATHS) {
                                if let Err(e) = crate::scan_library_incremental(
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
                                    warn!(
                                        "Incremental scan failed for '{}': {}. Falling back to full rescan.",
                                        lib_id,
                                        e
                                    );
                                    incremental_failed = true;
                                    break;
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
                                    None,
                                    None,
                                )
                                .await
                                {
                                    warn!(
                                        "Failed to run full fallback scan for '{}': {}",
                                        lib_id,
                                        full_err
                                    );
                                }
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
            (PathBuf::from(r"C:\media"), "root".to_string()),
            (PathBuf::from(r"C:\media\tv"), "tv".to_string()),
        ];

        let matched = find_library_for_path(Path::new(r"C:\media\tv\show\ep1.mkv"), &libs);
        assert_eq!(matched.as_deref(), Some("tv"));
    }

    #[test]
    fn returns_none_when_path_is_outside_all_libraries() {
        let libs = vec![(PathBuf::from(r"C:\media"), "root".to_string())];
        let matched = find_library_for_path(Path::new(r"C:\other\movie.mkv"), &libs);
        assert!(matched.is_none());
    }
}
