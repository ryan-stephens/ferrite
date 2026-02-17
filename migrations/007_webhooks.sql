-- Webhooks for external notifications
CREATE TABLE IF NOT EXISTS webhooks (
    id          TEXT PRIMARY KEY,
    user_id     TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name        TEXT NOT NULL,
    url         TEXT NOT NULL,
    secret      TEXT,
    -- Comma-separated list of event types to subscribe to, or '*' for all
    events      TEXT NOT NULL DEFAULT '*',
    enabled     INTEGER NOT NULL DEFAULT 1,
    created_at  TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at  TEXT NOT NULL DEFAULT (datetime('now')),
    -- Track delivery stats
    last_triggered_at TEXT,
    last_status_code  INTEGER,
    failure_count     INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_webhooks_user ON webhooks(user_id);
CREATE INDEX IF NOT EXISTS idx_webhooks_enabled ON webhooks(enabled);
