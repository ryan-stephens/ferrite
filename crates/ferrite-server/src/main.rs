use anyhow::Result;
use clap::Parser;
use ferrite_api::router::build_router;
use ferrite_api::state::AppState;
use ferrite_core::config::AppConfig;
use std::future::Future;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(name = "ferrite", about = "A high-performance media server")]
struct Cli {
    /// Path to configuration file
    #[arg(short, long, default_value = "config/ferrite.toml")]
    config: PathBuf,

    /// Port to listen on (overrides config)
    #[arg(short, long)]
    port: Option<u16>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Generate a bcrypt password hash for use in ferrite.toml [auth] section
    HashPassword {
        /// The password to hash
        password: String,
    },
    /// Initialize a new Ferrite configuration file with sensible defaults
    Init {
        /// Port to listen on
        #[arg(long, default_value = "8080")]
        port: u16,
        /// Output directory for the config file
        #[arg(long, default_value = "config")]
        output_dir: PathBuf,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let cli = Cli::parse();

    // Handle subcommands that don't need the full server
    match &cli.command {
        Some(Commands::HashPassword { password }) => {
            match bcrypt::hash(password, bcrypt::DEFAULT_COST) {
                Ok(hash) => {
                    println!("{hash}");
                    return Ok(());
                }
                Err(e) => {
                    eprintln!("Error hashing password: {e}");
                    std::process::exit(1);
                }
            }
        }
        Some(Commands::Init { port, output_dir }) => {
            return run_init(*port, output_dir).await;
        }
        None => {}
    }

    // Determine config file path: FERRITE_CONFIG env var > CLI arg > default
    let config_path = std::env::var("FERRITE_CONFIG")
        .map(PathBuf::from)
        .unwrap_or(cli.config);

    // Load config or use defaults
    let mut config = if config_path.exists() {
        let content = tokio::fs::read_to_string(&config_path).await?;
        info!("Loaded config from {}", config_path.display());
        toml::from_str::<AppConfig>(&content)?
    } else {
        info!(
            "Config file not found at {}, using defaults",
            config_path.display()
        );
        AppConfig::default()
    };

    // CLI overrides
    if let Some(port) = cli.port {
        config.server.port = port;
    }

    // Environment variable overrides (highest priority after CLI).
    // These are critical for seedbox/container deployment where config files
    // may not be practical.
    if let Ok(port) = std::env::var("FERRITE_PORT") {
        if let Ok(p) = port.parse::<u16>() {
            config.server.port = p;
        }
    }
    if let Ok(host) = std::env::var("FERRITE_HOST") {
        config.server.host = host;
    }
    if let Ok(path) = std::env::var("FERRITE_DB_PATH") {
        config.database.path = PathBuf::from(path);
    }
    if let Ok(path) = std::env::var("FERRITE_FFMPEG_PATH") {
        config.transcode.ffmpeg_path = path;
    }
    if let Ok(path) = std::env::var("FERRITE_FFPROBE_PATH") {
        config.transcode.ffprobe_path = path;
    }
    if let Ok(secret) = std::env::var("FERRITE_JWT_SECRET") {
        // Create or update auth config with the env var secret
        if let Some(ref mut auth) = config.auth {
            auth.jwt_secret = secret;
        } else {
            config.auth = Some(ferrite_core::config::AuthConfig {
                jwt_secret: secret,
                token_expiry_days: 30,
                auth_hotpath_no_db: false,
                api_keys: Vec::new(),
                username: None,
                password_hash: None,
            });
        }
    }

    // Resolve all relative paths against the data directory.
    // This ensures paths work correctly whether running from the repo root (dev)
    // or from ~/ferrite/ (seedbox deployment).
    // Note: FERRITE_DATA_DIR is checked inside resolve_paths().
    config.resolve_paths();

    // Detect hardware-accelerated encoders
    let hw_pref = config.transcode.hw_accel.as_deref().and_then(|s| match s {
        "nvenc" => Some(ferrite_transcode::hwaccel::HwAccelBackend::Nvenc),
        "qsv" => Some(ferrite_transcode::hwaccel::HwAccelBackend::Qsv),
        "vaapi" => Some(ferrite_transcode::hwaccel::HwAccelBackend::Vaapi),
        "software" => Some(ferrite_transcode::hwaccel::HwAccelBackend::Software),
        _ => {
            tracing::warn!("Unknown hw_accel value '{}', using auto-detect", s);
            None
        }
    });
    let hw_caps =
        ferrite_transcode::hwaccel::detect_and_select(&config.transcode.ffmpeg_path, hw_pref).await;
    info!(
        "Video encoder: {} (backend={})",
        hw_caps.selected_profile.encoder_name, hw_caps.selected_profile.backend,
    );
    let encoder_profile = Arc::new(hw_caps.selected_profile.clone());

    // Initialize database (before background tasks that may need it)
    let pool =
        ferrite_db::create_pool(&config.database.path, config.database.max_connections).await?;

    // Initialize HLS session manager
    let hls_cache_dir = config.transcode.cache_dir.join("hls");
    tokio::fs::create_dir_all(&hls_cache_dir).await?;

    let hls_manager = Arc::new(ferrite_stream::hls::HlsSessionManager::new(
        hls_cache_dir,
        config.transcode.ffmpeg_path.clone(),
        config.transcode.hls_segment_duration,
        config.transcode.hls_playlist_window_segments,
        config.transcode.hls_session_timeout_secs,
        config.transcode.hls_ffmpeg_idle_secs,
        hw_caps.selected_profile,
    ));

    // Spawn HLS cleanup background task (supervised — logs panics)
    let cleanup_manager = hls_manager.clone();
    tokio::spawn(supervised_task("HLS cleanup", async move {
        cleanup_manager.cleanup_loop().await;
    }));

    // Build app state and router
    let transcode_semaphore = Arc::new(tokio::sync::Semaphore::new(
        config.transcode.max_concurrent_transcodes,
    ));

    // Initialize webhook dispatcher
    let webhook_dispatcher = Arc::new(ferrite_api::webhooks::WebhookDispatcher::new(pool.clone()));

    // Keep a reference for graceful shutdown cleanup (before state is moved into the router)
    let shutdown_hls_manager = hls_manager.clone();

    // Build TMDB provider + image cache for the filesystem watcher so that
    // incrementally discovered media gets metadata without a manual rescan.
    let (watcher_tmdb, watcher_img_cache) = if let Some(ref api_key) = config.metadata.tmdb_api_key
    {
        let provider: std::sync::Arc<dyn ferrite_metadata::provider::MetadataProvider> =
            std::sync::Arc::new(ferrite_metadata::tmdb::TmdbProvider::new(
                api_key.clone(),
                config.metadata.rate_limit_per_second,
            ));
        let cache = std::sync::Arc::new(ferrite_metadata::image_cache::ImageCache::new(
            config.metadata.image_cache_dir.clone(),
        ));
        (Some(provider), Some(cache))
    } else {
        (None, None)
    };

    // Start filesystem watcher for auto-rescan (before AppState so the handle
    // can be stored for dynamic library registration from API handlers).
    let watcher = ferrite_scanner::watcher::LibraryWatcher::new(
        pool.clone(),
        config.transcode.ffprobe_path.clone(),
        config.transcode.ffmpeg_path.clone(),
        config.scanner.watch_debounce_seconds,
        config.scanner.concurrent_probes,
        config.scanner.subtitle_cache_dir.clone(),
        watcher_tmdb,
        watcher_img_cache,
    );
    let watcher_handle = match watcher.start().await {
        Ok(handle) => {
            info!("Filesystem watcher started");
            Some(handle)
        }
        Err(e) => {
            tracing::warn!("Filesystem watcher failed to start: {}", e);
            None
        }
    };

    let state = AppState {
        db: pool.clone(),
        config: Arc::new(config.clone()),
        hls_sessions: hls_manager,
        transcode_semaphore,
        login_limiter: AppState::new_login_limiter(),
        encoder_profile,
        webhook_dispatcher,
        scan_registry: ferrite_scanner::ScanRegistry::new(),
        watcher_handle,
        playback_metrics: Arc::new(ferrite_api::metrics::PlaybackMetrics::default()),
        update_state: Arc::new(ferrite_api::state::UpdateState::new()),
    };

    // Spawn background update check (every 6 hours, log-only, never auto-applies)
    if !config.update.disabled {
        let bg_update_config = config.update.clone();
        let bg_update_state = state.update_state.clone();
        tokio::spawn(supervised_task("update check", async move {
            // Initial delay: wait 60s after startup before first check
            tokio::time::sleep(std::time::Duration::from_secs(60)).await;
            loop {
                match ferrite_api::handlers::system::fetch_latest_release(&bg_update_config).await {
                    Ok(result) => {
                        // Cache the result
                        {
                            let mut cache = bg_update_state.cached.lock().await;
                            *cache = Some((std::time::Instant::now(), result.clone()));
                        }
                        if result.update_available {
                            info!(
                                "Update available: v{} → v{} ({})",
                                result.current_version, result.latest_version, result.release_url
                            );
                        } else {
                            tracing::debug!(
                                "Update check: running latest (v{})",
                                result.current_version
                            );
                        }
                    }
                    Err(e) => {
                        tracing::debug!("Background update check failed: {e}");
                    }
                }
                // Sleep 6 hours before next check
                tokio::time::sleep(std::time::Duration::from_secs(6 * 60 * 60)).await;
            }
        }));
    }

    let mut router = build_router(state);
    let addr = SocketAddr::new(config.server.host.parse()?, config.server.port);

    // DLNA/UPnP server
    if config.dlna.enabled {
        let server_uuid = uuid::Uuid::new_v4().to_string();
        let http_base_url = format!("http://{}:{}", config.server.host, config.server.port);

        // Merge DLNA HTTP routes into the main router
        let dlna_state = ferrite_dlna::routes::DlnaState {
            db: pool.clone(),
            server_uuid: server_uuid.clone(),
            friendly_name: config.dlna.friendly_name.clone(),
            http_base_url: http_base_url.clone(),
        };
        router = router.merge(ferrite_dlna::routes::build_dlna_router(dlna_state));

        // Start SSDP discovery in background.
        // Binding to port 1900 requires elevated privileges on Linux; failure is non-fatal.
        let ssdp = std::sync::Arc::new(ferrite_dlna::ssdp::SsdpServer::new(
            server_uuid,
            http_base_url,
        ));
        tokio::spawn(async move {
            if let Err(e) = ssdp.run().await {
                tracing::warn!(
                    "SSDP server stopped (DLNA discovery unavailable): {}. \
                     On Linux, binding port 1900 requires CAP_NET_BIND_SERVICE or running as root.",
                    e
                );
            }
        });

        info!("DLNA server enabled ({})", config.dlna.friendly_name);
    }

    info!("Ferrite starting on http://{}", addr);
    info!(
        "Open http://localhost:{} in your browser",
        config.server.port
    );

    let listener = tokio::net::TcpListener::bind(addr).await?;

    // Graceful shutdown: wait for Ctrl+C, then clean up FFmpeg processes
    axum::serve(listener, router)
        .with_graceful_shutdown(async move {
            let _ = tokio::signal::ctrl_c().await;
            info!("Shutdown signal received — cleaning up...");
            shutdown_hls_manager.destroy_all_sessions().await;
            info!("Graceful shutdown complete");
        })
        .await?;

    Ok(())
}

/// Wrap a background task future with logging so unexpected exits or panics
/// are visible in logs instead of silently disappearing.
async fn supervised_task(name: &'static str, fut: impl Future<Output = ()>) {
    fut.await;
    // If we get here, the task exited (cleanup loop is infinite, so this
    // indicates something went wrong).
    error!("Background task '{}' exited unexpectedly", name);
}

/// Generate a default configuration file with a random JWT secret.
async fn run_init(port: u16, output_dir: &PathBuf) -> Result<()> {
    let config_path = output_dir.join("ferrite.toml");

    if config_path.exists() {
        eprintln!("Config file already exists at {}", config_path.display());
        eprintln!("Remove it first if you want to regenerate.");
        std::process::exit(1);
    }

    // Generate a random JWT secret using two UUID v4s for 244 bits of entropy
    let jwt_secret = format!(
        "{}{}",
        uuid::Uuid::new_v4().simple(),
        uuid::Uuid::new_v4().simple()
    );

    let config_content = format!(
        r#"# Ferrite Media Server Configuration
# Generated by `ferrite init`

[server]
host = "0.0.0.0"
port = {port}
# Allowed CORS origins. Empty = allow all (recommended for seedbox).
# Example: cors_origins = ["https://my.domain.com"]
cors_origins = []

[database]
path = "ferrite.db"
max_connections = 16

[scanner]
concurrent_probes = 4
watch_debounce_seconds = 2

[transcode]
ffmpeg_path = "ffmpeg"
ffprobe_path = "ffprobe"
cache_dir = "cache/transcode"
max_concurrent_transcodes = 2
# queue wait timeout (seconds) before returning 503 when transcode slots are saturated
transcode_queue_timeout_secs = 15
# low-latency segment size
hls_segment_duration = 2
# max segments retained in live playlist window
hls_playlist_window_segments = 30
hls_session_timeout_secs = 30
# fMP4 media segment MIME mode: "video-mp4" (default) or "video-iso-segment"
hls_segment_mime_mode = "video-mp4"
# Hardware acceleration: "nvenc", "qsv", "vaapi", "software", or omit for auto-detect
# hw_accel = "software"

[metadata]
image_cache_dir = "cache/images"
rate_limit_per_second = 4
# tmdb_api_key = "your-tmdb-api-key"

[auth]
jwt_secret = "{jwt_secret}"
token_expiry_days = 30
# skip per-request DB user checks on /api/stream hot path (JWT validation still applies)
auth_hotpath_no_db = false

[dlna]
enabled = true
friendly_name = "Ferrite Media Server"
"#
    );

    // Create output directory
    tokio::fs::create_dir_all(output_dir).await?;
    tokio::fs::write(&config_path, &config_content).await?;

    println!("✓ Config written to {}", config_path.display());
    println!();
    println!("Next steps:");
    println!("  1. Start the server:  ./ferrite");
    println!("  2. Open http://localhost:{port} in your browser");
    println!("  3. Create your admin account on first visit");
    println!();
    println!("Environment variable overrides:");
    println!("  FERRITE_PORT          Listen port");
    println!("  FERRITE_HOST          Bind address");
    println!("  FERRITE_DATA_DIR      Base data directory");
    println!("  FERRITE_DB_PATH       Database file path");
    println!("  FERRITE_FFMPEG_PATH   FFmpeg binary path");
    println!("  FERRITE_FFPROBE_PATH  FFprobe binary path");
    println!("  FERRITE_JWT_SECRET    Auth secret (overrides config)");
    println!("  FERRITE_CONFIG        Config file path");
    println!("  FERRITE_STATIC_DIR    SPA static files directory");

    Ok(())
}
