use anyhow::Result;
use reqwest::Client;
use std::path::PathBuf;
use tracing::{debug, info};

const TMDB_IMAGE_BASE: &str = "https://image.tmdb.org/t/p/";

pub struct ImageCache {
    client: Client,
    cache_dir: PathBuf,
}

impl ImageCache {
    pub fn new(cache_dir: PathBuf) -> Self {
        Self {
            client: Client::new(),
            cache_dir,
        }
    }

    /// Download a TMDB poster image and cache it locally.
    /// `tmdb_path` is the relative path from TMDB (e.g., "/abc123.jpg")
    /// Uses "w500" size for posters.
    /// `tmdb_id` is used to create a deterministic filename.
    /// Returns the local filename (just the name, not full path).
    pub async fn ensure_poster(&self, tmdb_path: &str, tmdb_id: i64) -> Result<String> {
        let filename = format!("{}_poster.jpg", tmdb_id);
        let local_path = self.cache_dir.join(&filename);

        if local_path.exists() {
            debug!("Image already cached: {}", filename);
            return Ok(filename);
        }

        let url = format!("{}w500{}", TMDB_IMAGE_BASE, tmdb_path);
        let bytes = self.client.get(&url).send().await?.bytes().await?;
        tokio::fs::write(&local_path, &bytes).await?;
        info!("Cached poster image: {} ({} bytes)", filename, bytes.len());

        Ok(filename)
    }

    /// Download a TMDB backdrop image and cache it locally.
    /// `tmdb_path` is the relative path from TMDB (e.g., "/abc123.jpg")
    /// Uses "w1280" size for backdrops.
    /// `tmdb_id` is used to create a deterministic filename.
    /// Returns the local filename (just the name, not full path).
    pub async fn ensure_backdrop(&self, tmdb_path: &str, tmdb_id: i64) -> Result<String> {
        let filename = format!("{}_backdrop.jpg", tmdb_id);
        let local_path = self.cache_dir.join(&filename);

        if local_path.exists() {
            debug!("Backdrop already cached: {}", filename);
            return Ok(filename);
        }

        let url = format!("{}w1280{}", TMDB_IMAGE_BASE, tmdb_path);
        let bytes = self.client.get(&url).send().await?.bytes().await?;
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
        let bytes = self.client.get(&url).send().await?.bytes().await?;
        tokio::fs::write(&local_path, &bytes).await?;
        info!("Cached still image: {} ({} bytes)", filename, bytes.len());

        Ok(filename)
    }
}
