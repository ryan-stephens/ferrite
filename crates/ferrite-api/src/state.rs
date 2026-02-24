use crate::metrics::PlaybackMetrics;
use crate::webhooks::WebhookDispatcher;
use ferrite_core::config::AppConfig;
use ferrite_scanner::{ScanRegistry, WatcherHandle};
use ferrite_stream::hls::HlsSessionManager;
use ferrite_transcode::hwaccel::EncoderProfile;
use governor::clock::DefaultClock;
use governor::state::{InMemoryState, NotKeyed};
use governor::{Quota, RateLimiter};
use serde::Serialize;
use sqlx::SqlitePool;
use std::num::NonZeroU32;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::{Mutex, Semaphore};

/// Global (not per-IP) login rate limiter type.
pub type LoginRateLimiter = RateLimiter<NotKeyed, InMemoryState, DefaultClock>;

/// Shared application state injected into all Axum handlers.
#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub config: Arc<AppConfig>,
    pub hls_sessions: Arc<HlsSessionManager>,
    /// Limits the number of concurrent FFmpeg transcode processes.
    pub transcode_semaphore: Arc<Semaphore>,
    /// Rate limiter for login attempts (brute-force protection).
    /// Allows a burst of 5 attempts, replenishing 1 per second.
    pub login_limiter: Arc<LoginRateLimiter>,
    /// Detected video encoder profile (HW-accelerated or software).
    pub encoder_profile: Arc<EncoderProfile>,
    /// Webhook event dispatcher for external notifications.
    pub webhook_dispatcher: Arc<WebhookDispatcher>,
    /// Registry of active library scan progress states.
    pub scan_registry: ScanRegistry,
    /// Handle to the filesystem watcher for dynamic library registration.
    /// `None` if the watcher failed to start (non-fatal).
    pub watcher_handle: Option<WatcherHandle>,
    /// In-memory playback and hot-path metrics (WS0 observability).
    pub playback_metrics: Arc<PlaybackMetrics>,
    /// Cached state for the self-update version check.
    pub update_state: Arc<UpdateState>,
}

/// Cached result of a GitHub release version check.
#[derive(Clone, Serialize)]
pub struct UpdateCheckResult {
    pub current_version: String,
    pub latest_version: String,
    pub update_available: bool,
    pub release_url: String,
    pub release_notes: String,
    pub published_at: String,
    pub download_url: Option<String>,
    pub download_size_bytes: Option<u64>,
}

/// Current phase of an in-progress update.
#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePhase {
    Idle,
    Downloading,
    Verifying,
    Extracting,
    Swapping,
    Restarting,
    Failed,
}

/// Snapshot of update progress, returned by GET /api/system/update/status.
#[derive(Clone, Serialize)]
pub struct UpdateProgress {
    pub state: UpdatePhase,
    pub progress_pct: u8,
    pub downloaded_bytes: u64,
    pub total_bytes: u64,
    pub error: Option<String>,
}

impl Default for UpdateProgress {
    fn default() -> Self {
        Self {
            state: UpdatePhase::Idle,
            progress_pct: 0,
            downloaded_bytes: 0,
            total_bytes: 0,
            error: None,
        }
    }
}

/// In-memory state for the self-update system.
pub struct UpdateState {
    /// Cached version check result (15-minute TTL).
    pub cached: Mutex<Option<(Instant, UpdateCheckResult)>>,
    /// Guard to prevent concurrent update operations.
    pub in_progress: AtomicBool,
    /// Current progress of an in-flight update.
    pub progress: Mutex<UpdateProgress>,
}

impl Default for UpdateState {
    fn default() -> Self {
        Self::new()
    }
}

impl UpdateState {
    pub fn new() -> Self {
        Self {
            cached: Mutex::new(None),
            in_progress: AtomicBool::new(false),
            progress: Mutex::new(UpdateProgress::default()),
        }
    }

    /// Try to acquire the update lock. Returns false if an update is already running.
    pub fn try_start(&self) -> bool {
        self.in_progress
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
    }

    /// Release the update lock.
    pub fn finish(&self) {
        self.in_progress.store(false, Ordering::SeqCst);
    }
}

impl AppState {
    /// Create a login rate limiter: 5 burst, 1 per second replenish.
    pub fn new_login_limiter() -> Arc<LoginRateLimiter> {
        let quota =
            Quota::per_second(NonZeroU32::new(1).unwrap()).allow_burst(NonZeroU32::new(5).unwrap());
        Arc::new(RateLimiter::direct(quota))
    }
}
