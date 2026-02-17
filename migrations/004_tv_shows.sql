-- TV shows, seasons, and episodes.
-- A tv_show groups episodes by show name within a library.
-- Seasons group episodes within a show.
-- Episodes link to media_items (the actual files on disk).

CREATE TABLE IF NOT EXISTS tv_shows (
    id TEXT PRIMARY KEY,
    library_id TEXT NOT NULL REFERENCES libraries(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    sort_title TEXT,
    year INTEGER,
    overview TEXT,
    status TEXT,
    tmdb_id INTEGER,
    tvdb_id INTEGER,
    poster_path TEXT,
    backdrop_path TEXT,
    genres TEXT,
    fetched_at TEXT,
    UNIQUE(library_id, title)
);

CREATE TABLE IF NOT EXISTS seasons (
    id TEXT PRIMARY KEY,
    tv_show_id TEXT NOT NULL REFERENCES tv_shows(id) ON DELETE CASCADE,
    season_number INTEGER NOT NULL,
    title TEXT,
    overview TEXT,
    poster_path TEXT,
    UNIQUE(tv_show_id, season_number)
);

CREATE TABLE IF NOT EXISTS episodes (
    media_item_id TEXT PRIMARY KEY REFERENCES media_items(id) ON DELETE CASCADE,
    season_id TEXT NOT NULL REFERENCES seasons(id) ON DELETE CASCADE,
    episode_number INTEGER NOT NULL,
    title TEXT,
    overview TEXT,
    air_date TEXT,
    still_path TEXT,
    UNIQUE(season_id, episode_number)
);

CREATE INDEX IF NOT EXISTS idx_tv_shows_library ON tv_shows(library_id);
CREATE INDEX IF NOT EXISTS idx_seasons_show ON seasons(tv_show_id);
CREATE INDEX IF NOT EXISTS idx_episodes_season ON episodes(season_id);
