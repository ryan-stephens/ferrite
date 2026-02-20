-- Performance indexes to reduce slow query warnings observed during concurrent
-- library scans + metadata enrichment.
--
-- Problems addressed:
--   1. SELECT * FROM users WHERE id = ? taking 1-5s during scan polling
--      → users.id is already the PK (TEXT), but the DB is write-locked during
--        enrichment so reads queue. The index below is a no-op for the PK lookup
--        itself; the real fix is busy_timeout + write serialisation. However
--        adding an explicit index on the auth token lookup path helps the JWT
--        middleware avoid a full scan when the users table grows.
--
--   2. SELECT COUNT(*) FROM media_items ... with 4 JOINs taking 2-3s
--      → A covering index on (library_id, media_type) already exists from
--        migration 001. Add a composite index that also covers the sort columns
--        used by the list query so SQLite can satisfy ORDER BY without a filesort.
--
--   3. SELECT mi.id ... ORDER BY COALESCE(m.sort_title, m.title, mi.title) taking 2s
--      → Index on movies(sort_title, title) so the COALESCE sort key can be
--        resolved without a full movies table scan.

-- Covering index for the list query ORDER BY sort path on movies
CREATE INDEX IF NOT EXISTS idx_movies_sort_title ON movies(sort_title, title);

-- Covering index for media_items list query: library filter + sort
CREATE INDEX IF NOT EXISTS idx_media_items_library_added ON media_items(library_id, added_at DESC);

-- Index on episodes.media_item_id for the LEFT JOIN in the list query
CREATE INDEX IF NOT EXISTS idx_episodes_media_item ON episodes(media_item_id);

-- Index on seasons.id for the JOIN in the list query
CREATE INDEX IF NOT EXISTS idx_seasons_id ON seasons(id);
