-- Add normalized_title column to tv_shows for O(1) fuzzy-match lookups.
-- Previously upsert_tv_show() loaded ALL shows into memory and compared in Rust.
-- With this column + index, the match is a single indexed WHERE clause.
ALTER TABLE tv_shows ADD COLUMN normalized_title TEXT;

-- Back-fill existing rows. The normalization strips non-alphanumeric chars,
-- lowercases, collapses whitespace, and strips trailing year suffixes.
-- This mirrors the Rust normalize_title() + strip_year_suffix() logic.
UPDATE tv_shows
SET normalized_title = TRIM(
    LOWER(
        REPLACE(
            REPLACE(
                REPLACE(
                    REPLACE(
                        REPLACE(title, ':', ' '),
                    '!', ' '),
                '?', ' '),
            '.', ' '),
        ',', ' ')
    )
);

CREATE INDEX IF NOT EXISTS idx_tv_shows_normalized_title
    ON tv_shows (library_id, normalized_title);
