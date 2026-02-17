-- Collections / playlists
CREATE TABLE IF NOT EXISTS collections (
    id          TEXT PRIMARY KEY,
    user_id     TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name        TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    -- 'collection' = unordered set, 'playlist' = ordered list
    kind        TEXT NOT NULL DEFAULT 'collection' CHECK(kind IN ('collection', 'playlist')),
    created_at  TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at  TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS collection_items (
    id            TEXT PRIMARY KEY,
    collection_id TEXT NOT NULL REFERENCES collections(id) ON DELETE CASCADE,
    media_id      TEXT NOT NULL REFERENCES media_items(id) ON DELETE CASCADE,
    -- Position for ordered playlists (0-based). NULL for unordered collections.
    position      INTEGER,
    added_at      TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(collection_id, media_id)
);

CREATE INDEX IF NOT EXISTS idx_collections_user ON collections(user_id);
CREATE INDEX IF NOT EXISTS idx_collection_items_collection ON collection_items(collection_id, position);
CREATE INDEX IF NOT EXISTS idx_collection_items_media ON collection_items(media_id);
