use anyhow::{anyhow, Result};
use reqwest::Client;
use std::path::PathBuf;
use std::time::Duration;
use tracing::{debug, info, warn};

const TMDB_IMAGE_BASE: &str = "https://image.tmdb.org/t/p/";
const REQUEST_TIMEOUT_SECS: u64 = 10;
const MAX_RETRIES: u32 = 3;

pub struct ImageCache {
    client: Client,
    cache_dir: PathBuf,
}

impl ImageCache {
    pub fn new(cache_dir: PathBuf) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
            .build()
            .unwrap_or_default();
        Self { client, cache_dir }
    }

    /// Download a URL with retry + exponential backoff. Returns raw bytes.
    async fn fetch_with_retry(&self, url: &str) -> Result<Vec<u8>> {
        let mut last_err = anyhow!("no attempts made");
        for attempt in 0..MAX_RETRIES {
            if attempt > 0 {
                let delay = Duration::from_secs(1u64 << (attempt - 1)); // 1s, 2s
                warn!("Image download retry {}/{} for {} (waiting {}s)", attempt, MAX_RETRIES - 1, url, delay.as_secs());
                tokio::time::sleep(delay).await;
            }
            match self.client.get(url).send().await {
                Ok(resp) if resp.status().is_success() => {
                    match resp.bytes().await {
                        Ok(b) => return Ok(b.to_vec()),
                        Err(e) => last_err = e.into(),
                    }
                }
                Ok(resp) => {
                    last_err = anyhow!("HTTP {}", resp.status());
                }
                Err(e) => {
                    last_err = e.into();
                }
            }
        }
        Err(last_err)
    }

    /// Download a TMDB poster image and cache it locally.
    /// Uses "w500" size for posters.
    /// Returns the local filename (just the name, not full path).
    pub async fn ensure_poster(&self, tmdb_path: &str, tmdb_id: i64) -> Result<String> {
        let filename = format!("{}_poster.jpg", tmdb_id);
        let local_path = self.cache_dir.join(&filename);

        if local_path.exists() {
            debug!("Image already cached: {}", filename);
            return Ok(filename);
        }

        let url = format!("{}w500{}", TMDB_IMAGE_BASE, tmdb_path);
        let bytes = self.fetch_with_retry(&url).await?;
        tokio::fs::write(&local_path, &bytes).await?;
        info!("Cached poster image: {} ({} bytes)", filename, bytes.len());

        Ok(filename)
    }

    /// Download a TMDB backdrop image and cache it locally.
    /// Uses "w1280" size for backdrops.
    /// Returns the local filename (just the name, not full path).
    pub async fn ensure_backdrop(&self, tmdb_path: &str, tmdb_id: i64) -> Result<String> {
        let filename = format!("{}_backdrop.jpg", tmdb_id);
        let local_path = self.cache_dir.join(&filename);

        if local_path.exists() {
            debug!("Backdrop already cached: {}", filename);
            return Ok(filename);
        }

        let url = format!("{}w1280{}", TMDB_IMAGE_BASE, tmdb_path);
        let bytes = self.fetch_with_retry(&url).await?;
        tokio::fs::write(&local_path, &bytes).await?;
        info!("Cached backdrop image: {} ({} bytes)", filename, bytes.len());

        Ok(filename)
    }

    /// Download a TMDB episode still image and cache it locally.
    /// Uses "w300" size (suitable for episode thumbnails).
    /// Returns the local filename.
    pub async fn ensure_still(&self, tmdb_path: &str, tmdb_id: i64, season_number: i64, episode_number: i32) -> Result<String> {
        let filename = format!("{}_s{}_e{}_still.jpg", tmdb_id, season_number, episode_number);
        let local_path = self.cache_dir.join(&filename);

        if local_path.exists() {
            debug!("Still already cached: {}", filename);
            return Ok(filename);
        }

        let url = format!("{}w300{}", TMDB_IMAGE_BASE, tmdb_path);
        let bytes = self.fetch_with_retry(&url).await?;
        tokio::fs::write(&local_path, &bytes).await?;
        info!("Cached still image: {} ({} bytes)", filename, bytes.len());

        Ok(filename)
    }
}
