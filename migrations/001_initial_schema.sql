-- Initial Ferrite schema
-- Creates core tables: libraries, media_items, playback_progress, movies

CREATE TABLE IF NOT EXISTS libraries (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    path TEXT NOT NULL,
    library_type TEXT NOT NULL,
    scan_interval_minutes INTEGER DEFAULT 60,
    last_scanned_at TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS media_items (
    id TEXT PRIMARY KEY,
    library_id TEXT NOT NULL REFERENCES libraries(id) ON DELETE CASCADE,
    media_type TEXT NOT NULL,
    file_path TEXT NOT NULL UNIQUE,
    file_size INTEGER NOT NULL,
    file_hash TEXT,
    duration_ms INTEGER,
    container_format TEXT,
    video_codec TEXT,
    audio_codec TEXT,
    width INTEGER,
    height INTEGER,
    bitrate_kbps INTEGER,
    title TEXT,
    year INTEGER,
    added_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS playback_progress (
    id TEXT PRIMARY KEY,
    media_item_id TEXT NOT NULL REFERENCES media_items(id) ON DELETE CASCADE,
    position_ms INTEGER NOT NULL DEFAULT 0,
    completed INTEGER DEFAULT 0,
    last_played_at TEXT,
    play_count INTEGER DEFAULT 0,
    UNIQUE(media_item_id)
);

CREATE TABLE IF NOT EXISTS movies (
    media_item_id TEXT PRIMARY KEY REFERENCES media_items(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    sort_title TEXT,
    year INTEGER,
    overview TEXT,
    tagline TEXT,
    rating REAL,
    content_rating TEXT,
    tmdb_id INTEGER,
    imdb_id TEXT,
    poster_path TEXT,
    backdrop_path TEXT,
    genres TEXT,
    fetched_at TEXT
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_media_items_library ON media_items(library_id);
CREATE INDEX IF NOT EXISTS idx_media_items_type ON media_items(media_type);
CREATE INDEX IF NOT EXISTS idx_media_items_file_path ON media_items(file_path);
CREATE INDEX IF NOT EXISTS idx_media_items_title ON media_items(title);
CREATE INDEX IF NOT EXISTS idx_media_items_library_type ON media_items(library_id, media_type);
CREATE INDEX IF NOT EXISTS idx_playback_media ON playback_progress(media_item_id);
CREATE INDEX IF NOT EXISTS idx_playback_last_played ON playback_progress(last_played_at);
CREATE INDEX IF NOT EXISTS idx_movies_tmdb ON movies(tmdb_id);
CREATE INDEX IF NOT EXISTS idx_movies_fetched_at ON movies(fetched_at);
