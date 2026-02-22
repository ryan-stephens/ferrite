use crate::metrics::PlaybackMetrics;
use crate::webhooks::WebhookDispatcher;
use ferrite_core::config::AppConfig;
use ferrite_scanner::{ScanRegistry, WatcherHandle};
use ferrite_stream::hls::HlsSessionManager;
use ferrite_transcode::hwaccel::EncoderProfile;
use governor::clock::DefaultClock;
use governor::state::{InMemoryState, NotKeyed};
use governor::{Quota, RateLimiter};
use sqlx::SqlitePool;
use std::num::NonZeroU32;
use std::sync::Arc;
use tokio::sync::Semaphore;

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
}

impl AppState {
    /// Create a login rate limiter: 5 burst, 1 per second replenish.
    pub fn new_login_limiter() -> Arc<LoginRateLimiter> {
        let quota =
            Quota::per_second(NonZeroU32::new(1).unwrap()).allow_burst(NonZeroU32::new(5).unwrap());
        Arc::new(RateLimiter::direct(quota))
    }
}
