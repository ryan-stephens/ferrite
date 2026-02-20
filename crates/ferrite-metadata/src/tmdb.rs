use crate::provider::{EpisodeMetadata, MetadataProvider, MovieDetails, MovieSearchResult, TvSearchResult, TvShowDetails};
use async_trait::async_trait;
use anyhow::{Context, Result};
use governor::{Quota, RateLimiter, clock::DefaultClock, state::{InMemoryState, NotKeyed}};
use reqwest::Client;
use serde::Deserialize;
use std::num::NonZeroU32;
use std::sync::Arc;
use tracing::debug;

const TMDB_BASE_URL: &str = "https://api.themoviedb.org/3";

pub struct TmdbProvider {
    client: Client,
    api_key: String,
    rate_limiter: Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock>>,
}

impl TmdbProvider {
    pub fn new(api_key: String, rate_per_second: u32) -> Self {
        let quota = Quota::per_second(NonZeroU32::new(rate_per_second).unwrap());
        let rate_limiter = Arc::new(RateLimiter::direct(quota));
        Self {
            client: Client::new(),
            api_key,
            rate_limiter,
        }
    }
}

#[async_trait]
impl MetadataProvider for TmdbProvider {
    async fn search_movie(&self, title: &str, year: Option<i32>) -> Result<Vec<MovieSearchResult>> {
        self.rate_limiter.until_ready().await;

        debug!("TMDB search: {}", title);

        let mut params = vec![
            ("api_key", self.api_key.as_str()),
            ("query", title),
            ("language", "en-US"),
        ];
        let year_str;
        if let Some(y) = year {
            year_str = y.to_string();
            params.push(("year", year_str.as_str()));
        }

        let response = self
            .client
            .get(format!("{}/search/movie", TMDB_BASE_URL))
            .query(&params)
            .send()
            .await
            .context("TMDB search request failed")?;

        let search: TmdbSearchResponse = response
            .json()
            .await
            .context("Failed to parse TMDB search response")?;

        let results = search
            .results
            .into_iter()
            .map(|r| {
                let year = r.release_date.as_deref().and_then(|d| {
                    if d.len() >= 4 {
                        d[..4].parse::<i32>().ok()
                    } else {
                        None
                    }
                });

                MovieSearchResult {
                    tmdb_id: r.id,
                    title: r.title.unwrap_or_default(),
                    year,
                    overview: r.overview,
                    poster_path: r.poster_path,
                    backdrop_path: r.backdrop_path,
                    rating: r.vote_average,
                    genres: Vec::new(),
                }
            })
            .collect();

        Ok(results)
    }

    async fn get_movie_details(&self, tmdb_id: i64) -> Result<MovieDetails> {
        self.rate_limiter.until_ready().await;

        let url = format!(
            "{}/movie/{}?api_key={}&language=en-US",
            TMDB_BASE_URL, tmdb_id, self.api_key
        );

        debug!("TMDB details: id={}", tmdb_id);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("TMDB details request failed")?;

        let detail: TmdbMovieDetail = response
            .json()
            .await
            .context("Failed to parse TMDB movie detail")?;

        let title = detail.title.unwrap_or_default();
        let sort_title = generate_sort_title(&title);

        let year = detail.release_date.as_deref().and_then(|d| {
            if d.len() >= 4 {
                d[..4].parse::<i32>().ok()
            } else {
                None
            }
        });

        let genres = detail
            .genres
            .unwrap_or_default()
            .into_iter()
            .map(|g| g.name)
            .collect();

        Ok(MovieDetails {
            tmdb_id: detail.id,
            imdb_id: detail.imdb_id,
            title,
            sort_title,
            tagline: detail.tagline,
            overview: detail.overview,
            year,
            rating: detail.vote_average,
            content_rating: None,
            poster_path: detail.poster_path,
            backdrop_path: detail.backdrop_path,
            genres,
        })
    }

    async fn search_tv(&self, title: &str, year: Option<i32>) -> Result<Vec<TvSearchResult>> {
        self.rate_limiter.until_ready().await;

        debug!("TMDB TV search: {}", title);

        let mut params = vec![
            ("api_key", self.api_key.as_str()),
            ("query", title),
            ("language", "en-US"),
        ];
        let year_str;
        if let Some(y) = year {
            year_str = y.to_string();
            params.push(("first_air_date_year", year_str.as_str()));
        }

        let response = self
            .client
            .get(format!("{}/search/tv", TMDB_BASE_URL))
            .query(&params)
            .send()
            .await
            .context("TMDB TV search request failed")?;

        let search: TmdbTvSearchResponse = response
            .json()
            .await
            .context("Failed to parse TMDB TV search response")?;

        let results = search
            .results
            .into_iter()
            .map(|r| {
                let year = r.first_air_date.as_deref().and_then(|d| {
                    if d.len() >= 4 {
                        d[..4].parse::<i32>().ok()
                    } else {
                        None
                    }
                });

                TvSearchResult {
                    tmdb_id: r.id,
                    title: r.name.unwrap_or_default(),
                    year,
                    overview: r.overview,
                    poster_path: r.poster_path,
                    backdrop_path: r.backdrop_path,
                    rating: r.vote_average,
                }
            })
            .collect();

        Ok(results)
    }

    async fn get_season_episodes(&self, tmdb_id: i64, season_number: i64) -> Result<Vec<EpisodeMetadata>> {
        self.rate_limiter.until_ready().await;

        let url = format!(
            "{}/tv/{}/season/{}?api_key={}&language=en-US",
            TMDB_BASE_URL, tmdb_id, season_number, self.api_key
        );

        debug!("TMDB season episodes: show={} season={}", tmdb_id, season_number);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("TMDB season request failed")?;

        let season: TmdbSeasonDetail = response
            .json()
            .await
            .context("Failed to parse TMDB season detail")?;

        let episodes = season
            .episodes
            .unwrap_or_default()
            .into_iter()
            .map(|e| EpisodeMetadata {
                episode_number: e.episode_number,
                title: e.name,
                overview: e.overview,
                air_date: e.air_date,
                still_path: e.still_path,
            })
            .collect();

        Ok(episodes)
    }

    async fn get_tv_details(&self, tmdb_id: i64) -> Result<TvShowDetails> {
        self.rate_limiter.until_ready().await;

        let url = format!(
            "{}/tv/{}?api_key={}&language=en-US",
            TMDB_BASE_URL, tmdb_id, self.api_key
        );

        debug!("TMDB TV details: id={}", tmdb_id);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("TMDB TV details request failed")?;

        let detail: TmdbTvDetail = response
            .json()
            .await
            .context("Failed to parse TMDB TV detail")?;

        let title = detail.name.unwrap_or_default();
        let sort_title = generate_sort_title(&title);

        let year = detail.first_air_date.as_deref().and_then(|d| {
            if d.len() >= 4 {
                d[..4].parse::<i32>().ok()
            } else {
                None
            }
        });

        let genres = detail
            .genres
            .unwrap_or_default()
            .into_iter()
            .map(|g| g.name)
            .collect();

        Ok(TvShowDetails {
            tmdb_id: detail.id,
            title,
            sort_title,
            overview: detail.overview,
            year,
            status: detail.status,
            rating: detail.vote_average,
            poster_path: detail.poster_path,
            backdrop_path: detail.backdrop_path,
            genres,
        })
    }
}

/// Generate a sort-friendly title by stripping leading articles.
fn generate_sort_title(title: &str) -> Option<String> {
    let lower = title.to_lowercase();
    for prefix in &["the ", "a ", "an "] {
        if lower.starts_with(prefix) {
            return Some(title[prefix.len()..].to_string());
        }
    }
    None
}

/// Pick the best matching result from a TMDB search using string similarity.
/// Returns `None` if no result scores above the 0.6 threshold.
pub fn pick_best_match(
    results: &[MovieSearchResult],
    query_title: &str,
    query_year: Option<i32>,
) -> Option<MovieSearchResult> {
    let query_lower = query_title.to_lowercase();
    let mut best_score = 0.0f64;
    let mut best: Option<&MovieSearchResult> = None;

    for result in results {
        let result_lower = result.title.to_lowercase();
        let mut score = strsim::jaro_winkler(&query_lower, &result_lower);

        // Boost score if year matches
        if let (Some(qy), Some(ry)) = (query_year, result.year) {
            if qy == ry {
                score += 0.1;
            }
        }

        if score > best_score {
            best_score = score;
            best = Some(result);
        }
    }

    if best_score >= 0.6 {
        best.cloned()
    } else {
        None
    }
}

/// Pick the best matching TV show result from a TMDB search using string similarity.
pub fn pick_best_tv_match(
    results: &[TvSearchResult],
    query_title: &str,
    query_year: Option<i32>,
) -> Option<TvSearchResult> {
    let query_lower = query_title.to_lowercase();
    let mut best_score = 0.0f64;
    let mut best: Option<&TvSearchResult> = None;

    for result in results {
        let result_lower = result.title.to_lowercase();
        let mut score = strsim::jaro_winkler(&query_lower, &result_lower);

        if let (Some(qy), Some(ry)) = (query_year, result.year) {
            if qy == ry {
                score += 0.1;
            }
        }

        if score > best_score {
            best_score = score;
            best = Some(result);
        }
    }

    if best_score >= 0.6 {
        best.cloned()
    } else {
        None
    }
}

#[derive(Deserialize)]
struct TmdbSearchResponse {
    results: Vec<TmdbMovieResult>,
}

#[derive(Deserialize)]
struct TmdbMovieResult {
    id: i64,
    title: Option<String>,
    release_date: Option<String>,
    overview: Option<String>,
    poster_path: Option<String>,
    backdrop_path: Option<String>,
    vote_average: Option<f64>,
}

#[derive(Deserialize)]
struct TmdbMovieDetail {
    id: i64,
    imdb_id: Option<String>,
    title: Option<String>,
    tagline: Option<String>,
    overview: Option<String>,
    release_date: Option<String>,
    vote_average: Option<f64>,
    poster_path: Option<String>,
    backdrop_path: Option<String>,
    genres: Option<Vec<TmdbGenre>>,
}

#[derive(Deserialize)]
struct TmdbTvSearchResponse {
    results: Vec<TmdbTvResult>,
}

#[derive(Deserialize)]
struct TmdbTvResult {
    id: i64,
    name: Option<String>,
    first_air_date: Option<String>,
    overview: Option<String>,
    poster_path: Option<String>,
    backdrop_path: Option<String>,
    vote_average: Option<f64>,
}

#[derive(Deserialize)]
struct TmdbTvDetail {
    id: i64,
    name: Option<String>,
    overview: Option<String>,
    first_air_date: Option<String>,
    status: Option<String>,
    vote_average: Option<f64>,
    poster_path: Option<String>,
    backdrop_path: Option<String>,
    genres: Option<Vec<TmdbGenre>>,
}

#[derive(Deserialize)]
struct TmdbSeasonDetail {
    episodes: Option<Vec<TmdbEpisodeResult>>,
}

#[derive(Deserialize)]
struct TmdbEpisodeResult {
    episode_number: i32,
    name: Option<String>,
    overview: Option<String>,
    air_date: Option<String>,
    still_path: Option<String>,
}

#[derive(Deserialize)]
struct TmdbGenre {
    #[allow(dead_code)]
    id: i64,
    name: String,
}
