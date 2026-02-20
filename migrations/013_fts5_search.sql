-- FTS5 virtual table for full-text search across media titles, overviews, and genres.
-- Populated by triggers on movies, tv_shows, and episodes tables.

CREATE VIRTUAL TABLE IF NOT EXISTS media_fts USING fts5(
    media_item_id UNINDEXED,
    title,
    overview,
    genres,
    content='',
    tokenize='unicode61 remove_diacritics 1'
);

-- Populate from existing movies
INSERT OR IGNORE INTO media_fts (media_item_id, title, overview, genres)
SELECT
    mi.id,
    COALESCE(m.title, mi.title, ''),
    COALESCE(m.overview, ''),
    COALESCE(m.genres, '')
FROM media_items mi
LEFT JOIN movies m ON m.media_item_id = mi.id
WHERE mi.media_type = 'movie';

-- Populate from existing TV episodes (use show title + episode title)
INSERT OR IGNORE INTO media_fts (media_item_id, title, overview, genres)
SELECT
    mi.id,
    COALESCE(ts.title, mi.title, '') || ' ' || COALESCE(ep.title, ''),
    COALESCE(ep.overview, ts.overview, ''),
    COALESCE(ts.genres, '')
FROM media_items mi
JOIN episodes ep ON ep.media_item_id = mi.id
JOIN seasons s ON s.id = ep.season_id
JOIN tv_shows ts ON ts.id = s.tv_show_id;
