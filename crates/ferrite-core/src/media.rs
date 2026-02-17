use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Type of media library.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LibraryType {
    Movie,
    Tv,
    Music,
}

/// A configured media library (a directory the user points at).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Library {
    pub id: Uuid,
    pub name: String,
    pub path: String,
    pub library_type: LibraryType,
    pub scan_interval_minutes: u32,
    pub last_scanned_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// The kind of media item stored on disk.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MediaType {
    Movie,
    Episode,
    Track,
}

/// A single media file on disk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaItem {
    pub id: Uuid,
    pub library_id: Uuid,
    pub media_type: MediaType,
    pub file_path: String,
    pub file_size: u64,
    pub file_hash: Option<String>,
    pub duration_ms: Option<u64>,
    pub container_format: Option<String>,
    pub video_codec: Option<String>,
    pub audio_codec: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub bitrate_kbps: Option<u32>,
    pub added_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Display name and metadata for a movie.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Movie {
    pub media_item_id: Uuid,
    pub title: String,
    pub sort_title: Option<String>,
    pub year: Option<i32>,
    pub overview: Option<String>,
    pub tagline: Option<String>,
    pub rating: Option<f64>,
    pub content_rating: Option<String>,
    pub tmdb_id: Option<i64>,
    pub imdb_id: Option<String>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
}

/// A TV show (parent of seasons).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TvShow {
    pub id: Uuid,
    pub library_id: Uuid,
    pub title: String,
    pub sort_title: Option<String>,
    pub year: Option<i32>,
    pub overview: Option<String>,
    pub status: Option<String>,
    pub tmdb_id: Option<i64>,
    pub tvdb_id: Option<i64>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
}

/// A season within a TV show.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Season {
    pub id: Uuid,
    pub tv_show_id: Uuid,
    pub season_number: u32,
    pub title: Option<String>,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
}

/// An episode (links to a media item).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Episode {
    pub media_item_id: Uuid,
    pub season_id: Uuid,
    pub episode_number: u32,
    pub title: Option<String>,
    pub overview: Option<String>,
    pub air_date: Option<String>,
    pub still_path: Option<String>,
}

/// A music artist.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artist {
    pub id: Uuid,
    pub library_id: Uuid,
    pub name: String,
    pub sort_name: Option<String>,
    pub overview: Option<String>,
    pub musicbrainz_id: Option<String>,
    pub image_path: Option<String>,
}

/// A music album.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Album {
    pub id: Uuid,
    pub artist_id: Uuid,
    pub title: String,
    pub year: Option<i32>,
    pub musicbrainz_id: Option<String>,
    pub cover_path: Option<String>,
}

/// A music track (links to a media item).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub media_item_id: Uuid,
    pub album_id: Uuid,
    pub track_number: Option<u32>,
    pub disc_number: Option<u32>,
    pub title: String,
}

/// An individual stream within a media file (video, audio, or subtitle track).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaStream {
    pub id: Uuid,
    pub media_item_id: Uuid,
    pub stream_index: u32,
    pub stream_type: StreamType,
    pub codec: Option<String>,
    pub language: Option<String>,
    pub title: Option<String>,
    pub is_default: bool,
    pub channels: Option<u32>,
    pub sample_rate: Option<u32>,
    pub bit_depth: Option<u32>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub is_forced: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StreamType {
    Video,
    Audio,
    Subtitle,
}

/// Playback progress for a media item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybackProgress {
    pub id: Uuid,
    pub media_item_id: Uuid,
    pub position_ms: u64,
    pub completed: bool,
    pub last_played_at: Option<DateTime<Utc>>,
    pub play_count: u32,
}

/// Known media file extensions.
pub const VIDEO_EXTENSIONS: &[&str] = &[
    "mp4", "mkv", "avi", "mov", "wmv", "flv", "webm", "m4v", "mpg", "mpeg", "ts", "m2ts",
    "vob", "ogv", "3gp",
];

pub const AUDIO_EXTENSIONS: &[&str] = &[
    "mp3", "flac", "aac", "ogg", "opus", "wav", "wma", "m4a", "alac", "aiff", "ape", "dsf",
    "dff", "wv",
];

pub const SUBTITLE_EXTENSIONS: &[&str] = &["srt", "ass", "ssa", "vtt", "sub", "idx", "sup"];

/// Check if a file extension is a recognized media type.
pub fn classify_extension(ext: &str) -> Option<MediaType> {
    let ext = ext.to_lowercase();
    if VIDEO_EXTENSIONS.contains(&ext.as_str()) {
        // Could be Movie or Episode — caller determines from context
        Some(MediaType::Movie)
    } else if AUDIO_EXTENSIONS.contains(&ext.as_str()) {
        Some(MediaType::Track)
    } else {
        None
    }
}
