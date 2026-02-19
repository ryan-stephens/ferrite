use regex::Regex;
use std::sync::LazyLock;

#[derive(Debug, Clone)]
pub struct ParsedMovie {
    pub title: String,
    pub year: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct ParsedEpisode {
    pub show_name: String,
    pub season: u32,
    pub episode: u32,
}

#[derive(Debug, Clone)]
pub enum ParsedFilename {
    Movie(ParsedMovie),
    Episode(ParsedEpisode),
    Unknown(String),
}

// -- TV episode patterns (checked first) --

/// Matches `Show Name S01E05` or `show.name.s01e05` (case-insensitive).
static RE_EPISODE_SXXEXX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)^(.+?)[.\s_-]+s(\d{1,2})e(\d{1,2})").unwrap()
});

/// Matches `Show Name 1x05`.
static RE_EPISODE_NX_NN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)^(.+?)[.\s_-]+(\d{1,2})x(\d{2,3})").unwrap()
});

// -- Movie patterns --

/// Matches `The Matrix (1999)`.
static RE_MOVIE_PAREN_YEAR: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(.+?)\s*\((\d{4})\)").unwrap()
});

/// Matches `Movie Title [2020]`.
static RE_MOVIE_BRACKET_YEAR: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(.+?)\s*\[(\d{4})\]").unwrap()
});

/// Matches `The.Matrix.1999.BluRay` — dot/underscore/space separated with a 4-digit year
/// followed by end-of-string or another separator token.
static RE_MOVIE_DOT_YEAR: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(.+?)[.\s_-]+((?:19|20)\d{2})(?:[.\s_-]|$)").unwrap()
});

/// Replace dots and underscores with spaces, collapse runs of whitespace, and trim.
pub fn clean_title(raw: &str) -> String {
    let replaced = raw.replace(['.', '_'], " ");
    let collapsed: String = replaced
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    collapsed.trim().to_string()
}

/// Strip a trailing 4-digit year (19xx or 20xx) from a show name.
/// e.g. "Star Trek Lower Decks 2020" → "Star Trek Lower Decks"
/// Leaves the name unchanged if no trailing year is found.
pub fn strip_trailing_year(name: &str) -> &str {
    // Match a trailing year separated by a space
    if let Some(rest) = name.strip_suffix(|c: char| c.is_ascii_digit()) {
        // Check if the last token is a 4-digit year (19xx or 20xx)
        let trimmed = name.trim_end();
        if trimmed.len() >= 5 {
            let (prefix, suffix) = trimmed.split_at(trimmed.len() - 4);
            if (suffix.starts_with("19") || suffix.starts_with("20"))
                && suffix.chars().all(|c| c.is_ascii_digit())
                && prefix.ends_with(' ')
            {
                return prefix.trim_end();
            }
        }
        let _ = rest;
    }
    name
}

/// Parse a media file stem (filename without extension) into a structured result.
///
/// Detection order:
/// 1. TV episode patterns (`S01E05`, `1x05`)
/// 2. Movie patterns (parenthesised year, bracketed year, dot-separated year)
/// 3. Fallback to `Unknown` with a cleaned-up title
pub fn parse_filename(file_stem: &str) -> ParsedFilename {
    // --- TV episodes (checked first) ---

    if let Some(caps) = RE_EPISODE_SXXEXX.captures(file_stem) {
        let raw = clean_title(&caps[1]);
        let show_name = strip_trailing_year(&raw).to_string();
        let season: u32 = caps[2].parse().unwrap_or(0);
        let episode: u32 = caps[3].parse().unwrap_or(0);
        return ParsedFilename::Episode(ParsedEpisode {
            show_name,
            season,
            episode,
        });
    }

    if let Some(caps) = RE_EPISODE_NX_NN.captures(file_stem) {
        let raw = clean_title(&caps[1]);
        let show_name = strip_trailing_year(&raw).to_string();
        let season: u32 = caps[2].parse().unwrap_or(0);
        let episode: u32 = caps[3].parse().unwrap_or(0);
        return ParsedFilename::Episode(ParsedEpisode {
            show_name,
            season,
            episode,
        });
    }

    // --- Movies ---

    if let Some(caps) = RE_MOVIE_PAREN_YEAR.captures(file_stem) {
        let title = clean_title(&caps[1]);
        let year: Option<i32> = caps[2].parse().ok();
        return ParsedFilename::Movie(ParsedMovie { title, year });
    }

    if let Some(caps) = RE_MOVIE_BRACKET_YEAR.captures(file_stem) {
        let title = clean_title(&caps[1]);
        let year: Option<i32> = caps[2].parse().ok();
        return ParsedFilename::Movie(ParsedMovie { title, year });
    }

    if let Some(caps) = RE_MOVIE_DOT_YEAR.captures(file_stem) {
        let title = clean_title(&caps[1]);
        let year: Option<i32> = caps[2].parse().ok();
        return ParsedFilename::Movie(ParsedMovie { title, year });
    }

    // --- Fallback ---

    ParsedFilename::Unknown(clean_title(file_stem))
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- Movie tests ----

    #[test]
    fn movie_paren_year() {
        let result = parse_filename("The Matrix (1999)");
        match result {
            ParsedFilename::Movie(m) => {
                assert_eq!(m.title, "The Matrix");
                assert_eq!(m.year, Some(1999));
            }
            other => panic!("Expected Movie, got {:?}", other),
        }
    }

    #[test]
    fn movie_dot_separated_with_tags() {
        let result = parse_filename("The.Matrix.1999.BluRay.1080p");
        match result {
            ParsedFilename::Movie(m) => {
                assert_eq!(m.title, "The Matrix");
                assert_eq!(m.year, Some(1999));
            }
            other => panic!("Expected Movie, got {:?}", other),
        }
    }

    #[test]
    fn movie_bracket_year() {
        let result = parse_filename("Movie Title [2020]");
        match result {
            ParsedFilename::Movie(m) => {
                assert_eq!(m.title, "Movie Title");
                assert_eq!(m.year, Some(2020));
            }
            other => panic!("Expected Movie, got {:?}", other),
        }
    }

    // ---- Episode tests ----

    #[test]
    fn episode_sxxexx_uppercase() {
        let result = parse_filename("Breaking Bad S03E05");
        match result {
            ParsedFilename::Episode(e) => {
                assert_eq!(e.show_name, "Breaking Bad");
                assert_eq!(e.season, 3);
                assert_eq!(e.episode, 5);
            }
            other => panic!("Expected Episode, got {:?}", other),
        }
    }

    #[test]
    fn episode_sxxexx_dot_separated_lowercase() {
        let result = parse_filename("breaking.bad.s03e05.720p");
        match result {
            ParsedFilename::Episode(e) => {
                assert_eq!(e.show_name, "breaking bad");
                assert_eq!(e.season, 3);
                assert_eq!(e.episode, 5);
            }
            other => panic!("Expected Episode, got {:?}", other),
        }
    }

    #[test]
    fn episode_with_trailing_year() {
        let result = parse_filename("Star.Trek.Lower.Decks.2020.S01E01.Strange.Energies");
        match result {
            ParsedFilename::Episode(e) => {
                assert_eq!(e.show_name, "Star Trek Lower Decks");
                assert_eq!(e.season, 1);
                assert_eq!(e.episode, 1);
            }
            other => panic!("Expected Episode, got {:?}", other),
        }
    }

    #[test]
    fn episode_year_not_stripped_when_part_of_name() {
        // Year in the middle should not be stripped
        let result = parse_filename("Show.2020.Name.S01E01");
        match result {
            ParsedFilename::Episode(e) => {
                // The year is not at the end, so name stays as-is after clean_title
                assert_eq!(e.show_name, "Show 2020 Name");
            }
            other => panic!("Expected Episode, got {:?}", other),
        }
    }

    #[test]
    fn episode_nxnn_format() {
        let result = parse_filename("Show Name 2x10");
        match result {
            ParsedFilename::Episode(e) => {
                assert_eq!(e.show_name, "Show Name");
                assert_eq!(e.season, 2);
                assert_eq!(e.episode, 10);
            }
            other => panic!("Expected Episode, got {:?}", other),
        }
    }

    // ---- Unknown / fallback tests ----

    #[test]
    fn unknown_plain_title() {
        let result = parse_filename("Some Random File");
        match result {
            ParsedFilename::Unknown(s) => assert_eq!(s, "Some Random File"),
            other => panic!("Expected Unknown, got {:?}", other),
        }
    }

    #[test]
    fn unknown_dot_separated() {
        let result = parse_filename("some.random.file");
        match result {
            ParsedFilename::Unknown(s) => assert_eq!(s, "some random file"),
            other => panic!("Expected Unknown, got {:?}", other),
        }
    }

    // ---- clean_title tests ----

    #[test]
    fn clean_title_replaces_dots_and_underscores() {
        assert_eq!(clean_title("hello.world_test"), "hello world test");
    }

    #[test]
    fn clean_title_collapses_multiple_spaces() {
        assert_eq!(clean_title("hello...world___test"), "hello world test");
    }

    #[test]
    fn clean_title_trims_whitespace() {
        assert_eq!(clean_title("  hello  "), "hello");
    }
}
