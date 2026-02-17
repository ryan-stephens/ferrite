-- Stores external subtitle files discovered alongside media files.
-- e.g. Movie.srt, Movie.en.srt, Movie.en.forced.srt

CREATE TABLE IF NOT EXISTS external_subtitles (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    media_item_id TEXT NOT NULL REFERENCES media_items(id) ON DELETE CASCADE,
    file_path TEXT NOT NULL UNIQUE,
    format TEXT NOT NULL,                -- 'srt', 'vtt', 'ass', 'ssa', 'sub', 'idx', 'sup'
    language TEXT,                        -- ISO 639 code parsed from filename, e.g. 'en', 'ja'
    title TEXT,                           -- descriptive label, e.g. 'English', 'SDH', 'Commentary'
    is_forced INTEGER NOT NULL DEFAULT 0, -- parsed from filename (e.g. '.forced.srt')
    is_sdh INTEGER NOT NULL DEFAULT 0,    -- parsed from filename (e.g. '.sdh.srt')
    file_size INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_ext_subs_media ON external_subtitles(media_item_id);
