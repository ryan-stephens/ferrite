-- Coarse keyframe index persisted during scan to avoid per-seek ffprobe calls.
-- Stores monotonically increasing keyframe timestamps in milliseconds.
CREATE TABLE IF NOT EXISTS media_keyframes (
    media_item_id TEXT NOT NULL REFERENCES media_items(id) ON DELETE CASCADE,
    pts_ms INTEGER NOT NULL,
    PRIMARY KEY (media_item_id, pts_ms)
);

-- Supports nearest-keyframe lookup:
--   WHERE media_item_id = ? AND pts_ms <= ? ORDER BY pts_ms DESC LIMIT 1
CREATE INDEX IF NOT EXISTS idx_media_keyframes_lookup
    ON media_keyframes(media_item_id, pts_ms);
