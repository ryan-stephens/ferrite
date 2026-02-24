use dashmap::DashMap;
use serde::Serialize;
use std::sync::{
    atomic::{AtomicU32, AtomicU64, Ordering},
    Arc,
};
use tokio::sync::RwLock;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ScanStatus {
    Scanning,
    Enriching,
    Subtitles,
    Complete,
    Failed,
}

/// Per-library scan progress, updated atomically as work proceeds.
pub struct ScanState {
    pub library_id: String,
    pub status: RwLock<ScanStatus>,
    pub total_files: AtomicU32,
    pub files_probed: AtomicU32,
    pub files_inserted: AtomicU32,
    pub subtitles_extracted: AtomicU32,
    pub items_enriched: AtomicU32,
    pub errors: AtomicU32,
    pub current_item: RwLock<String>,
    pub started_at_unix: AtomicU64,
    /// Timestamp (unix secs) when the current phase started. Reset at each phase transition.
    pub phase_started_at_unix: AtomicU64,
}

impl ScanState {
    pub fn new(library_id: String) -> Arc<Self> {
        let started = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        Arc::new(Self {
            library_id,
            status: RwLock::new(ScanStatus::Scanning),
            total_files: AtomicU32::new(0),
            files_probed: AtomicU32::new(0),
            files_inserted: AtomicU32::new(0),
            subtitles_extracted: AtomicU32::new(0),
            items_enriched: AtomicU32::new(0),
            errors: AtomicU32::new(0),
            current_item: RwLock::new(String::new()),
            started_at_unix: AtomicU64::new(started),
            phase_started_at_unix: AtomicU64::new(started),
        })
    }

    pub async fn set_status(&self, status: ScanStatus) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        self.phase_started_at_unix.store(now, Ordering::Relaxed);
        *self.status.write().await = status;
    }

    pub async fn set_current(&self, item: &str) {
        *self.current_item.write().await = item.to_string();
    }

    pub fn inc_probed(&self) {
        self.files_probed.fetch_add(1, Ordering::Relaxed);
    }

    pub fn inc_inserted(&self) {
        self.files_inserted.fetch_add(1, Ordering::Relaxed);
    }

    pub fn inc_subtitles(&self, n: u32) {
        self.subtitles_extracted.fetch_add(n, Ordering::Relaxed);
    }

    pub fn inc_enriched(&self) {
        self.items_enriched.fetch_add(1, Ordering::Relaxed);
    }

    pub fn inc_errors(&self) {
        self.errors.fetch_add(1, Ordering::Relaxed);
    }
}

#[derive(Serialize)]
pub struct ScanProgress {
    pub scanning: bool,
    pub status: ScanStatus,
    pub total_files: u32,
    pub files_probed: u32,
    pub files_inserted: u32,
    pub subtitles_extracted: u32,
    pub items_enriched: u32,
    pub errors: u32,
    pub current_item: String,
    pub elapsed_seconds: u64,
    pub phase_elapsed_seconds: u64,
    pub estimated_remaining_seconds: Option<u64>,
    pub percent: u8,
}

impl ScanState {
    pub async fn to_progress(&self) -> ScanProgress {
        let status = self.status.read().await.clone();
        let scanning = matches!(
            status,
            ScanStatus::Scanning | ScanStatus::Enriching | ScanStatus::Subtitles
        );
        let total = self.total_files.load(Ordering::Relaxed);
        let probed = self.files_probed.load(Ordering::Relaxed);
        let percent = if total > 0 {
            ((probed as f32 / total as f32) * 100.0).min(100.0) as u8
        } else {
            0
        };
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let elapsed = now.saturating_sub(self.started_at_unix.load(Ordering::Relaxed));
        let phase_elapsed = now.saturating_sub(self.phase_started_at_unix.load(Ordering::Relaxed));

        // ETA: extrapolate from Phase 1 probe rate (most predictable phase).
        // Only compute when actively scanning and we have meaningful progress.
        let estimated_remaining_seconds = if matches!(status, ScanStatus::Scanning)
            && probed > 0
            && total > probed
            && phase_elapsed > 0
        {
            let rate = probed as f64 / phase_elapsed as f64;
            let remaining_files = (total - probed) as f64;
            Some((remaining_files / rate).ceil() as u64)
        } else {
            None
        };

        ScanProgress {
            scanning,
            status,
            total_files: total,
            files_probed: probed,
            files_inserted: self.files_inserted.load(Ordering::Relaxed),
            subtitles_extracted: self.subtitles_extracted.load(Ordering::Relaxed),
            items_enriched: self.items_enriched.load(Ordering::Relaxed),
            errors: self.errors.load(Ordering::Relaxed),
            current_item: self.current_item.read().await.clone(),
            elapsed_seconds: elapsed,
            phase_elapsed_seconds: phase_elapsed,
            estimated_remaining_seconds,
            percent,
        }
    }
}

/// Global registry of active/recent scans keyed by library_id.
#[derive(Clone, Default)]
pub struct ScanRegistry(Arc<DashMap<String, Arc<ScanState>>>);

impl ScanRegistry {
    pub fn new() -> Self {
        Self(Arc::new(DashMap::new()))
    }

    /// Try to start a scan for a library. Returns None if one is already running.
    pub fn try_start(&self, library_id: String) -> Option<Arc<ScanState>> {
        use dashmap::mapref::entry::Entry;
        match self.0.entry(library_id.clone()) {
            Entry::Occupied(e) => {
                let existing = e.get().clone();
                // Allow starting a new scan if the previous one is done/failed
                let status = existing.status.try_read().ok()?;
                if matches!(
                    *status,
                    ScanStatus::Scanning | ScanStatus::Enriching | ScanStatus::Subtitles
                ) {
                    return None; // Already running
                }
                drop(status);
                drop(e);
                let state = ScanState::new(library_id.clone());
                self.0.insert(library_id, state.clone());
                Some(state)
            }
            Entry::Vacant(e) => {
                let state = ScanState::new(library_id);
                e.insert(state.clone());
                Some(state)
            }
        }
    }

    pub fn get(&self, library_id: &str) -> Option<Arc<ScanState>> {
        self.0.get(library_id).map(|r| r.clone())
    }

    /// Remove a library's scan state from the registry (e.g. on library deletion).
    pub fn remove(&self, library_id: &str) {
        self.0.remove(library_id);
    }
}
