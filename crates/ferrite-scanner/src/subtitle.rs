use ferrite_core::media::SUBTITLE_EXTENSIONS;
use ferrite_db::subtitle_repo::SubtitleInsert;
use std::path::Path;
use tokio::fs;
use tracing::debug;

/// Well-known ISO 639-1 and 639-2 language codes for subtitle filename parsing.
const KNOWN_LANGS: &[&str] = &[
    "en", "eng", "es", "spa", "fr", "fre", "fra", "de", "ger", "deu", "it", "ita", "pt", "por",
    "ru", "rus", "ja", "jpn", "ko", "kor", "zh", "zho", "chi", "ar", "ara", "hi", "hin", "nl",
    "dut", "nld", "sv", "swe", "no", "nor", "da", "dan", "fi", "fin", "pl", "pol", "cs", "cze",
    "ces", "hu", "hun", "ro", "ron", "rum", "tr", "tur", "th", "tha", "vi", "vie", "uk", "ukr",
    "el", "gre", "ell", "he", "heb", "id", "ind", "ms", "msa", "may", "bg", "bul", "hr", "hrv",
    "sk", "slk", "slo", "sl", "slv", "sr", "srp", "lt", "lit", "lv", "lav", "et", "est",
];

/// Scan the directory containing a media file for external subtitle files.
/// Matches files that share the same stem as the media file.
///
/// Naming conventions supported:
/// - `Movie.srt` — no language tag
/// - `Movie.en.srt` — language code
/// - `Movie.en.forced.srt` — language + forced flag
/// - `Movie.en.sdh.srt` — language + SDH flag
/// - `Movie.English.srt` — full language name (stored as title)
pub async fn find_external_subtitles(media_file: &Path) -> Vec<SubtitleInsert> {
    let parent = match media_file.parent() {
        Some(p) => p,
        None => return Vec::new(),
    };

    let media_stem = match media_file.file_stem().and_then(|s| s.to_str()) {
        Some(s) => s.to_lowercase(),
        None => return Vec::new(),
    };

    let mut subtitles = Vec::new();

    let mut entries = match fs::read_dir(parent).await {
        Ok(e) => e,
        Err(_) => return Vec::new(),
    };

    while let Ok(Some(entry)) = entries.next_entry().await {
        let path = entry.path();

        // Must be a file with a subtitle extension
        let ext = match path.extension().and_then(|e| e.to_str()) {
            Some(e) => e.to_lowercase(),
            None => continue,
        };
        if !SUBTITLE_EXTENSIONS.contains(&ext.as_str()) {
            continue;
        }

        // Must share the same stem prefix as the media file
        let sub_stem = match path.file_stem().and_then(|s| s.to_str()) {
            Some(s) => s,
            None => continue,
        };
        let sub_stem_lower = sub_stem.to_lowercase();

        // The subtitle stem must start with the media stem
        // e.g. "movie" matches "movie", "movie.en", "movie.en.forced"
        if !sub_stem_lower.starts_with(&media_stem) {
            continue;
        }

        // Parse the suffix after the media stem (e.g. ".en.forced")
        let suffix = &sub_stem[media_stem.len()..];
        let (language, title, is_forced, is_sdh) = parse_subtitle_suffix(suffix);

        let file_size = entry.metadata().await.map(|m| m.len()).unwrap_or(0);

        debug!(
            "Found external subtitle: {} (lang={:?}, forced={}, sdh={})",
            path.display(),
            language,
            is_forced,
            is_sdh,
        );

        subtitles.push(SubtitleInsert {
            file_path: path.to_string_lossy().to_string(),
            format: ext,
            language,
            title,
            is_forced,
            is_sdh,
            file_size,
        });
    }

    subtitles.sort_by(|a, b| a.file_path.cmp(&b.file_path));
    subtitles
}

/// Parse the suffix after the media stem to extract language, flags, and title.
/// Examples:
/// - "" → (None, None, false, false)
/// - ".en" → (Some("en"), None, false, false)
/// - ".en.forced" → (Some("en"), None, true, false)
/// - ".en.sdh" → (Some("en"), None, false, true)
/// - ".English" → (None, Some("English"), false, false)
fn parse_subtitle_suffix(suffix: &str) -> (Option<String>, Option<String>, bool, bool) {
    if suffix.is_empty() {
        return (None, None, false, false);
    }

    // Remove leading dot and split by dots
    let parts: Vec<&str> = suffix
        .trim_start_matches('.')
        .split('.')
        .filter(|s| !s.is_empty())
        .collect();

    if parts.is_empty() {
        return (None, None, false, false);
    }

    let mut language: Option<String> = None;
    let mut title: Option<String> = None;
    let mut is_forced = false;
    let mut is_sdh = false;

    for part in &parts {
        let lower = part.to_lowercase();
        if lower == "forced" {
            is_forced = true;
        } else if lower == "sdh" || lower == "cc" {
            is_sdh = true;
        } else if language.is_none() && KNOWN_LANGS.contains(&lower.as_str()) {
            language = Some(lower);
        } else if title.is_none() {
            // Anything else becomes the title (e.g. "English", "Commentary", "Signs")
            title = Some(part.to_string());
        }
    }

    (language, title, is_forced, is_sdh)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_suffix() {
        let (lang, title, forced, sdh) = parse_subtitle_suffix("");
        assert_eq!(lang, None);
        assert_eq!(title, None);
        assert!(!forced);
        assert!(!sdh);
    }

    #[test]
    fn language_only() {
        let (lang, title, forced, sdh) = parse_subtitle_suffix(".en");
        assert_eq!(lang, Some("en".to_string()));
        assert_eq!(title, None);
        assert!(!forced);
        assert!(!sdh);
    }

    #[test]
    fn language_forced() {
        let (lang, title, forced, sdh) = parse_subtitle_suffix(".en.forced");
        assert_eq!(lang, Some("en".to_string()));
        assert_eq!(title, None);
        assert!(forced);
        assert!(!sdh);
    }

    #[test]
    fn language_sdh() {
        let (lang, title, forced, sdh) = parse_subtitle_suffix(".en.sdh");
        assert_eq!(lang, Some("en".to_string()));
        assert_eq!(title, None);
        assert!(!forced);
        assert!(sdh);
    }

    #[test]
    fn full_language_name_as_title() {
        let (lang, title, forced, sdh) = parse_subtitle_suffix(".English");
        assert_eq!(lang, None);
        assert_eq!(title, Some("English".to_string()));
        assert!(!forced);
        assert!(!sdh);
    }

    #[test]
    fn three_letter_code() {
        let (lang, title, forced, sdh) = parse_subtitle_suffix(".jpn");
        assert_eq!(lang, Some("jpn".to_string()));
        assert_eq!(title, None);
        assert!(!forced);
        assert!(!sdh);
    }

    #[test]
    fn cc_flag() {
        let (lang, title, forced, sdh) = parse_subtitle_suffix(".en.cc");
        assert_eq!(lang, Some("en".to_string()));
        assert_eq!(title, None);
        assert!(!forced);
        assert!(sdh);
    }
}
