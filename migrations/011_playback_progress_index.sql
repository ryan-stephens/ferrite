-- Add index on (user_id, last_played_at) for efficient "recently played" queries.
-- The existing unique index on (user_id, media_item_id) covers point lookups,
-- but ORDER BY last_played_at DESC queries do a full scan without this index.
CREATE INDEX IF NOT EXISTS idx_playback_progress_user_last_played
    ON playback_progress (user_id, last_played_at DESC);
