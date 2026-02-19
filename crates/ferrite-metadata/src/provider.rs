use async_trait::async_trait;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct MovieSearchResult {
    pub tmdb_id: i64,
    pub title: String,
    pub year: Option<i32>,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub rating: Option<f64>,
    pub genres: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct MovieDetails {
    pub tmdb_id: i64,
    pub imdb_id: Option<String>,
    pub title: String,
    pub sort_title: Option<String>,
    pub tagline: Option<String>,
    pub overview: Option<String>,
    pub year: Option<i32>,
    pub rating: Option<f64>,
    pub content_rating: Option<String>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub genres: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct TvSearchResult {
    pub tmdb_id: i64,
    pub title: String,
    pub year: Option<i32>,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub rating: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct TvShowDetails {
    pub tmdb_id: i64,
    pub title: String,
    pub sort_title: Option<String>,
    pub overview: Option<String>,
    pub year: Option<i32>,
    pub status: Option<String>,
    pub rating: Option<f64>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub genres: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct EpisodeMetadata {
    pub episode_number: i32,
    pub title: Option<String>,
    pub overview: Option<String>,
    pub air_date: Option<String>,
    pub still_path: Option<String>,
}

#[async_trait]
pub trait MetadataProvider: Send + Sync {
    async fn search_movie(&self, title: &str, year: Option<i32>) -> Result<Vec<MovieSearchResult>>;
    async fn get_movie_details(&self, tmdb_id: i64) -> Result<MovieDetails>;
    async fn search_tv(&self, title: &str, year: Option<i32>) -> Result<Vec<TvSearchResult>>;
    async fn get_tv_details(&self, tmdb_id: i64) -> Result<TvShowDetails>;
    async fn get_season_episodes(&self, tmdb_id: i64, season_number: i64) -> Result<Vec<EpisodeMetadata>>;
}
