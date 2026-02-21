-- Keep media_fts synchronized for newly scanned/updated media rows.
-- Also rebuild existing entries to repair any stale/missing index state.

DROP TABLE IF EXISTS media_fts;

CREATE VIRTUAL TABLE media_fts USING fts5(
    media_item_id UNINDEXED,
    title,
    overview,
    genres,
    tokenize='unicode61 remove_diacritics 1'
);

-- Rebuild from movies
INSERT INTO media_fts (media_item_id, title, overview, genres)
SELECT
    mi.id,
    COALESCE(m.title, mi.title, ''),
    COALESCE(m.overview, ''),
    COALESCE(m.genres, '')
FROM media_items mi
LEFT JOIN movies m ON m.media_item_id = mi.id
WHERE mi.media_type = 'movie';

-- Rebuild from TV episodes
INSERT INTO media_fts (media_item_id, title, overview, genres)
SELECT
    mi.id,
    COALESCE(ts.title, mi.title, '') || ' ' || COALESCE(ep.title, ''),
    COALESCE(ep.overview, ts.overview, ''),
    COALESCE(ts.genres, '')
FROM media_items mi
JOIN episodes ep ON ep.media_item_id = mi.id
JOIN seasons s ON s.id = ep.season_id
JOIN tv_shows ts ON ts.id = s.tv_show_id;

DROP TRIGGER IF EXISTS trg_movies_fts_ai;
DROP TRIGGER IF EXISTS trg_movies_fts_au;
DROP TRIGGER IF EXISTS trg_movies_fts_ad;
DROP TRIGGER IF EXISTS trg_episodes_fts_ai;
DROP TRIGGER IF EXISTS trg_episodes_fts_au;
DROP TRIGGER IF EXISTS trg_episodes_fts_ad;
DROP TRIGGER IF EXISTS trg_tv_shows_fts_au;

CREATE TRIGGER trg_movies_fts_ai
AFTER INSERT ON movies
BEGIN
    DELETE FROM media_fts WHERE media_item_id = NEW.media_item_id;
    INSERT INTO media_fts (media_item_id, title, overview, genres)
    SELECT
        mi.id,
        COALESCE(NEW.title, mi.title, ''),
        COALESCE(NEW.overview, ''),
        COALESCE(NEW.genres, '')
    FROM media_items mi
    WHERE mi.id = NEW.media_item_id;
END;

CREATE TRIGGER trg_movies_fts_au
AFTER UPDATE ON movies
BEGIN
    DELETE FROM media_fts WHERE media_item_id = NEW.media_item_id;
    INSERT INTO media_fts (media_item_id, title, overview, genres)
    SELECT
        mi.id,
        COALESCE(NEW.title, mi.title, ''),
        COALESCE(NEW.overview, ''),
        COALESCE(NEW.genres, '')
    FROM media_items mi
    WHERE mi.id = NEW.media_item_id;
END;

CREATE TRIGGER trg_movies_fts_ad
AFTER DELETE ON movies
BEGIN
    DELETE FROM media_fts WHERE media_item_id = OLD.media_item_id;
END;

CREATE TRIGGER trg_episodes_fts_ai
AFTER INSERT ON episodes
BEGIN
    DELETE FROM media_fts WHERE media_item_id = NEW.media_item_id;
    INSERT INTO media_fts (media_item_id, title, overview, genres)
    SELECT
        NEW.media_item_id,
        COALESCE(ts.title, mi.title, '') || ' ' || COALESCE(NEW.title, ''),
        COALESCE(NEW.overview, ts.overview, ''),
        COALESCE(ts.genres, '')
    FROM media_items mi
    JOIN seasons s ON s.id = NEW.season_id
    JOIN tv_shows ts ON ts.id = s.tv_show_id
    WHERE mi.id = NEW.media_item_id;
END;

CREATE TRIGGER trg_episodes_fts_au
AFTER UPDATE ON episodes
BEGIN
    DELETE FROM media_fts WHERE media_item_id = NEW.media_item_id;
    INSERT INTO media_fts (media_item_id, title, overview, genres)
    SELECT
        NEW.media_item_id,
        COALESCE(ts.title, mi.title, '') || ' ' || COALESCE(NEW.title, ''),
        COALESCE(NEW.overview, ts.overview, ''),
        COALESCE(ts.genres, '')
    FROM media_items mi
    JOIN seasons s ON s.id = NEW.season_id
    JOIN tv_shows ts ON ts.id = s.tv_show_id
    WHERE mi.id = NEW.media_item_id;
END;

CREATE TRIGGER trg_episodes_fts_ad
AFTER DELETE ON episodes
BEGIN
    DELETE FROM media_fts WHERE media_item_id = OLD.media_item_id;
END;

CREATE TRIGGER trg_tv_shows_fts_au
AFTER UPDATE OF title, overview, genres ON tv_shows
BEGIN
    DELETE FROM media_fts
    WHERE media_item_id IN (
        SELECT e.media_item_id
        FROM episodes e
        JOIN seasons s ON s.id = e.season_id
        WHERE s.tv_show_id = NEW.id
    );

    INSERT INTO media_fts (media_item_id, title, overview, genres)
    SELECT
        e.media_item_id,
        COALESCE(NEW.title, mi.title, '') || ' ' || COALESCE(e.title, ''),
        COALESCE(e.overview, NEW.overview, ''),
        COALESCE(NEW.genres, '')
    FROM episodes e
    JOIN seasons s ON s.id = e.season_id
    JOIN media_items mi ON mi.id = e.media_item_id
    WHERE s.tv_show_id = NEW.id;
END;
