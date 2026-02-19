CREATE TABLE IF NOT EXISTS chapters (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    media_item_id TEXT NOT NULL REFERENCES media_items(id) ON DELETE CASCADE,
    chapter_index INTEGER NOT NULL,
    title       TEXT,
    start_time_ms INTEGER NOT NULL,
    end_time_ms   INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_chapters_media_item ON chapters(media_item_id);
