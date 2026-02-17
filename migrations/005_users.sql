-- Multi-user support.
-- Adds a users table and makes playback_progress per-user.

CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    display_name TEXT,
    password_hash TEXT NOT NULL,
    is_admin INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    last_login_at TEXT
);

-- Add user_id column to playback_progress (nullable for backward compat with existing rows).
ALTER TABLE playback_progress ADD COLUMN user_id TEXT REFERENCES users(id) ON DELETE CASCADE;

-- Drop old unique constraint and add new one that includes user_id.
-- SQLite doesn't support DROP CONSTRAINT, so we create a new unique index instead.
-- The old UNIQUE(media_item_id) from the CREATE TABLE is kept but the new composite
-- index will be used for per-user lookups.
CREATE UNIQUE INDEX IF NOT EXISTS idx_progress_user_media
    ON playback_progress(user_id, media_item_id);

CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);
