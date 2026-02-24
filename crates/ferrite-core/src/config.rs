use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub scanner: ScannerConfig,
    pub transcode: TranscodeConfig,
    #[serde(default)]
    pub metadata: MetadataConfig,
    /// Authentication config. If absent, server runs without auth.
    pub auth: Option<AuthConfig>,
    /// DLNA/UPnP server config
    #[serde(default)]
    pub dlna: DlnaConfig,
    /// Self-update config. If absent from ferrite.toml, defaults are used.
    #[serde(default)]
    pub update: UpdateConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    /// Allowed CORS origins for remote browser access.
    /// If empty or absent, all origins are allowed (recommended for seedbox deployment).
    /// Example: `["https://my.domain.com", "http://192.168.1.100:8080"]`
    #[serde(default)]
    pub cors_origins: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub path: PathBuf,
    /// Maximum number of connections in the SQLite pool.
    /// Default is 16. Higher values reduce contention under concurrent load.
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
}

fn default_max_connections() -> u32 {
    16
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScannerConfig {
    pub concurrent_probes: usize,
    pub watch_debounce_seconds: u64,
    /// Directory where extracted embedded subtitles are stored.
    /// Defaults to `cache/subtitles` under the data directory.
    #[serde(default = "default_subtitle_cache_dir")]
    pub subtitle_cache_dir: PathBuf,
}

fn default_subtitle_cache_dir() -> PathBuf {
    PathBuf::from("cache/subtitles")
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HlsSegmentMimeMode {
    /// Serve `.m4s` with `video/mp4` (recommended cross-client default).
    #[default]
    VideoMp4,
    /// Serve `.m4s` with `video/iso.segment` for strict ISO segment typing.
    VideoIsoSegment,
}

impl HlsSegmentMimeMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::VideoMp4 => "video-mp4",
            Self::VideoIsoSegment => "video-iso-segment",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscodeConfig {
    pub ffmpeg_path: String,
    pub ffprobe_path: String,
    pub cache_dir: PathBuf,
    pub max_concurrent_transcodes: usize,
    /// Maximum seconds to wait in the transcode queue before returning 503.
    #[serde(default = "default_transcode_queue_timeout_secs")]
    pub transcode_queue_timeout_secs: u64,
    /// HLS segment duration in seconds
    #[serde(default = "default_hls_segment_duration")]
    pub hls_segment_duration: u64,
    /// Number of segments retained in each live HLS playlist window.
    /// Older segments are deleted to cap disk usage.
    #[serde(default = "default_hls_playlist_window_segments")]
    pub hls_playlist_window_segments: u32,
    /// HLS session timeout — sessions idle for this many seconds are cleaned up
    #[serde(default = "default_hls_session_timeout")]
    pub hls_session_timeout_secs: u64,
    /// MIME mode for HLS fMP4 media segments (`.m4s`).
    ///
    /// - `video-mp4` (default): best interoperability across web, tvOS, and Roku-like clients.
    /// - `video-iso-segment`: explicit ISO BMFF segment MIME type.
    #[serde(default)]
    pub hls_segment_mime_mode: HlsSegmentMimeMode,
    /// Seconds of no segment requests before FFmpeg is killed (client paused).
    /// Increase for slow/satellite connections where segment downloads take longer.
    #[serde(default = "default_hls_ffmpeg_idle_secs")]
    pub hls_ffmpeg_idle_secs: u64,
    /// Hardware acceleration preference: "nvenc", "qsv", "vaapi", "software", or null for auto-detect.
    #[serde(default)]
    pub hw_accel: Option<String>,
}

fn default_transcode_queue_timeout_secs() -> u64 {
    15
}

fn default_hls_segment_duration() -> u64 {
    2
}

fn default_hls_playlist_window_segments() -> u32 {
    30
}

fn default_hls_session_timeout() -> u64 {
    30
}

fn default_hls_ffmpeg_idle_secs() -> u64 {
    30
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataConfig {
    pub tmdb_api_key: Option<String>,
    pub image_cache_dir: PathBuf,
    pub rate_limit_per_second: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    /// Secret used to sign JWT tokens — use a long random string
    pub jwt_secret: String,
    /// Token expiration in days
    #[serde(default = "default_token_expiry_days")]
    pub token_expiry_days: u64,
    /// If true, skip per-request DB user existence checks on /api/stream hot paths.
    /// JWT signature + expiry are still validated.
    #[serde(default = "default_auth_hotpath_no_db")]
    pub auth_hotpath_no_db: bool,
    /// Optional API keys for programmatic access
    #[serde(default)]
    pub api_keys: Vec<String>,
    // Legacy fields — ignored but accepted for backward compatibility with old configs
    #[serde(default, skip_serializing)]
    pub username: Option<String>,
    #[serde(default, skip_serializing)]
    pub password_hash: Option<String>,
}

fn default_token_expiry_days() -> u64 {
    30
}

fn default_auth_hotpath_no_db() -> bool {
    false
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DlnaConfig {
    /// Enable DLNA/UPnP server for network media device discovery
    #[serde(default = "default_dlna_enabled")]
    pub enabled: bool,
    /// Friendly name shown to DLNA clients (e.g. smart TVs)
    #[serde(default = "default_dlna_friendly_name")]
    pub friendly_name: String,
}

impl Default for DlnaConfig {
    fn default() -> Self {
        Self {
            enabled: default_dlna_enabled(),
            friendly_name: default_dlna_friendly_name(),
        }
    }
}

fn default_dlna_enabled() -> bool {
    true
}

fn default_dlna_friendly_name() -> String {
    "Ferrite Media Server".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConfig {
    /// Disable self-update entirely (e.g. if managed by a package manager)
    #[serde(default)]
    pub disabled: bool,
    /// GitHub repo to check for releases (default: "ryan-stephens/ferrite")
    #[serde(default = "default_update_repo")]
    pub repo: String,
    /// Optional GitHub token for higher API rate limits (60/hr unauthenticated → 5000/hr).
    /// Can also be set via `GITHUB_TOKEN` env var.
    #[serde(default)]
    pub github_token: Option<String>,
}

impl Default for UpdateConfig {
    fn default() -> Self {
        Self {
            disabled: false,
            repo: default_update_repo(),
            github_token: None,
        }
    }
}

fn default_update_repo() -> String {
    "ryan-stephens/ferrite".to_string()
}

impl Default for MetadataConfig {
    fn default() -> Self {
        Self {
            tmdb_api_key: None,
            image_cache_dir: PathBuf::from("cache/images"),
            rate_limit_per_second: 4,
        }
    }
}

impl AppConfig {
    /// Resolve all relative paths in the config against a base data directory.
    ///
    /// Resolution order for data_dir:
    /// 1. `$FERRITE_DATA_DIR` env var
    /// 2. `<exe_dir>/data/` (seedbox: ~/ferrite/data/)
    /// 3. CWD (development fallback)
    ///
    /// Paths that are already absolute are left unchanged.
    pub fn resolve_paths(&mut self) {
        let data_dir = Self::resolve_data_dir();
        tracing::info!("Data directory: {}", data_dir.display());

        // Ensure data dir exists
        if let Err(e) = std::fs::create_dir_all(&data_dir) {
            tracing::warn!(
                "Failed to create data directory {}: {}",
                data_dir.display(),
                e
            );
        }

        // Resolve database path
        if self.database.path.is_relative() {
            self.database.path = data_dir.join(&self.database.path);
        }

        // Resolve transcode cache dir
        if self.transcode.cache_dir.is_relative() {
            self.transcode.cache_dir = data_dir.join(&self.transcode.cache_dir);
        }

        // Resolve image cache dir
        if self.metadata.image_cache_dir.is_relative() {
            self.metadata.image_cache_dir = data_dir.join(&self.metadata.image_cache_dir);
        }

        // Resolve subtitle cache dir
        if self.scanner.subtitle_cache_dir.is_relative() {
            self.scanner.subtitle_cache_dir = data_dir.join(&self.scanner.subtitle_cache_dir);
        }

        // Ensure cache directories exist
        for dir in [
            &self.transcode.cache_dir,
            &self.metadata.image_cache_dir,
            &self.scanner.subtitle_cache_dir,
        ] {
            if let Err(e) = std::fs::create_dir_all(dir) {
                tracing::warn!("Failed to create cache directory {}: {}", dir.display(), e);
            }
        }
    }

    /// Determine the base data directory.
    fn resolve_data_dir() -> PathBuf {
        // 1. Explicit env var
        if let Ok(dir) = std::env::var("FERRITE_DATA_DIR") {
            return PathBuf::from(dir);
        }

        // 2. Exe-relative data/ (seedbox: ~/ferrite/data/)
        if let Ok(exe) = std::env::current_exe() {
            if let Some(exe_dir) = exe.parent() {
                let data = exe_dir.join("data");
                // Use exe-relative if the exe is NOT in a cargo target/ directory (i.e. deployed)
                if !exe_dir.to_string_lossy().contains("target") {
                    return data;
                }
            }
        }

        // 3. CWD fallback (development)
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 8080,
                cors_origins: Vec::new(),
            },
            database: DatabaseConfig {
                path: PathBuf::from("ferrite.db"),
                max_connections: default_max_connections(),
            },
            scanner: ScannerConfig {
                concurrent_probes: 4,
                watch_debounce_seconds: 2,
                subtitle_cache_dir: default_subtitle_cache_dir(),
            },
            transcode: TranscodeConfig {
                ffmpeg_path: "ffmpeg".to_string(),
                ffprobe_path: "ffprobe".to_string(),
                cache_dir: PathBuf::from("cache/transcode"),
                max_concurrent_transcodes: 2,
                transcode_queue_timeout_secs: default_transcode_queue_timeout_secs(),
                hls_segment_duration: default_hls_segment_duration(),
                hls_playlist_window_segments: default_hls_playlist_window_segments(),
                hls_session_timeout_secs: default_hls_session_timeout(),
                hls_segment_mime_mode: HlsSegmentMimeMode::default(),
                hls_ffmpeg_idle_secs: default_hls_ffmpeg_idle_secs(),
                hw_accel: None,
            },
            metadata: MetadataConfig::default(),
            auth: None,
            dlna: DlnaConfig::default(),
            update: UpdateConfig::default(),
        }
    }
}
