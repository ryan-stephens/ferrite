-- Rebuild playback_progress to remove legacy global uniqueness by media item.
-- Enforce per-user uniqueness only: (user_id, media_item_id).
-- Existing rows are preserved as-is (including legacy NULL user_id rows).

CREATE TABLE IF NOT EXISTS playback_progress_v2 (
    id TEXT PRIMARY KEY,
    media_item_id TEXT NOT NULL REFERENCES media_items(id) ON DELETE CASCADE,
    user_id TEXT REFERENCES users(id) ON DELETE CASCADE,
    position_ms INTEGER NOT NULL DEFAULT 0,
    completed INTEGER DEFAULT 0,
    last_played_at TEXT,
    play_count INTEGER DEFAULT 0,
    UNIQUE(user_id, media_item_id)
);

INSERT INTO playback_progress_v2 (
    id,
    media_item_id,
    user_id,
    position_ms,
    completed,
    last_played_at,
    play_count
)
SELECT
    id,
    media_item_id,
    user_id,
    position_ms,
    completed,
    last_played_at,
    play_count
FROM playback_progress;

DROP TABLE playback_progress;
ALTER TABLE playback_progress_v2 RENAME TO playback_progress;

CREATE INDEX IF NOT EXISTS idx_playback_media ON playback_progress(media_item_id);
CREATE INDEX IF NOT EXISTS idx_playback_last_played ON playback_progress(last_played_at);
CREATE UNIQUE INDEX IF NOT EXISTS idx_progress_user_media
    ON playback_progress(user_id, media_item_id);
CREATE INDEX IF NOT EXISTS idx_playback_progress_user_last_played
    ON playback_progress(user_id, last_played_at DESC);
