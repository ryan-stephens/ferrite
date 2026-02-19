use ferrite_db::subtitle_repo::SubtitleInsert;
use std::path::Path;
use tokio::process::Command;
use tracing::{debug, info, warn};

/// Text-based subtitle codecs that can be extracted to external files.
/// PGS (hdmv_pgs_subtitle) and DVB are bitmap-based and cannot be extracted as text.
const EXTRACTABLE_CODECS: &[&str] = &[
    "subrip",      // SRT
    "ass",         // ASS/SSA
    "ssa",         // SSA
    "webvtt",      // WebVTT
    "mov_text",    // MP4 text subtitles
    "srt",         // Alias for subrip
];

/// Output format mapping: codec_name → file extension
fn codec_to_extension(codec: &str) -> &'static str {
    match codec {
        "subrip" | "srt" => "srt",
        "ass" | "ssa" => "ass",
        "webvtt" => "vtt",
        "mov_text" => "srt",
        _ => "srt",
    }
}

/// Information about an embedded subtitle stream discovered by ffprobe.
pub struct EmbeddedSubtitleStream {
    pub stream_index: u32,
    pub codec_name: String,
    pub language: Option<String>,
    pub title: Option<String>,
    pub is_default: bool,
    pub is_forced: bool,
}

/// Check if a codec is a text-based subtitle that can be extracted.
pub fn is_extractable_subtitle(codec_name: &str) -> bool {
    EXTRACTABLE_CODECS.contains(&codec_name.to_lowercase().as_str())
}

/// Extract embedded text-based subtitles from a media file using FFmpeg.
///
/// Files are written to `{subtitle_cache_dir}/{media_item_id}/` so they never
/// appear inside the user's library directory.
///
/// Returns a list of `SubtitleInsert` entries for the extracted files.
pub async fn extract_embedded_subtitles(
    ffmpeg_path: &str,
    media_file: &Path,
    streams: &[EmbeddedSubtitleStream],
    subtitle_cache_dir: &Path,
    media_item_id: &str,
) -> Vec<SubtitleInsert> {
    let media_stem = match media_file.file_stem().and_then(|s| s.to_str()) {
        Some(s) => s,
        None => return Vec::new(),
    };

    // Each media item gets its own subdirectory so files are easy to clean up
    let output_dir = subtitle_cache_dir.join(media_item_id);
    if let Err(e) = tokio::fs::create_dir_all(&output_dir).await {
        warn!("Failed to create subtitle cache dir {}: {}", output_dir.display(), e);
        return Vec::new();
    }

    let mut extracted = Vec::new();

    for stream in streams {
        let ext = codec_to_extension(&stream.codec_name);
        let lang_part = stream.language.as_deref().unwrap_or("und");
        let forced_part = if stream.is_forced { ".forced" } else { "" };

        // Output path: {cache}/{media_item_id}/Movie.embedded.0.eng.srt
        let output_name = format!(
            "{}.embedded.{}.{}{}{}",
            media_stem, stream.stream_index, lang_part, forced_part,
            if ext.is_empty() { String::new() } else { format!(".{}", ext) }
        );
        let output_path = output_dir.join(&output_name);

        // Skip if already extracted (idempotent)
        if output_path.exists() {
            debug!(
                "Embedded subtitle already extracted: {}",
                output_path.display()
            );
            // Still add to the result so it gets registered in the DB
            if let Ok(meta) = tokio::fs::metadata(&output_path).await {
                extracted.push(SubtitleInsert {
                    file_path: output_path.to_string_lossy().to_string(),
                    format: ext.to_string(),
                    language: stream.language.clone(),
                    title: stream.title.clone().or_else(|| {
                        Some(format!("Embedded #{}", stream.stream_index))
                    }),
                    is_forced: stream.is_forced,
                    is_sdh: false,
                    file_size: meta.len(),
                });
            }
            continue;
        }

        // Extract using FFmpeg: ffmpeg -i input -map 0:s:{relative_index} -c:s copy output
        // We use the absolute stream index with -map 0:{index}
        let result = Command::new(ffmpeg_path)
            .args([
                "-hide_banner",
                "-nostdin",
                "-y",
                "-i",
                &media_file.to_string_lossy(),
                "-map",
                &format!("0:{}", stream.stream_index),
                "-c:s",
                // For SRT output from subrip/mov_text, use srt codec
                // For ASS/SSA, use copy to preserve formatting
                match stream.codec_name.as_str() {
                    "ass" | "ssa" => "copy",
                    _ => "srt",
                },
            ])
            .arg(&output_path)
            .output()
            .await;

        match result {
            Ok(output) if output.status.success() => {
                let file_size = tokio::fs::metadata(&output_path)
                    .await
                    .map(|m| m.len())
                    .unwrap_or(0);

                if file_size == 0 {
                    // Empty extraction — remove the file
                    let _ = tokio::fs::remove_file(&output_path).await;
                    debug!(
                        "Extracted empty subtitle (stream {}), removed",
                        stream.stream_index
                    );
                    continue;
                }

                info!(
                    "Extracted embedded subtitle: stream {} ({}) → {} ({} bytes)",
                    stream.stream_index,
                    stream.codec_name,
                    output_path.display(),
                    file_size,
                );

                extracted.push(SubtitleInsert {
                    file_path: output_path.to_string_lossy().to_string(),
                    format: ext.to_string(),
                    language: stream.language.clone(),
                    title: stream.title.clone().or_else(|| {
                        Some(format!("Embedded #{}", stream.stream_index))
                    }),
                    is_forced: stream.is_forced,
                    is_sdh: false,
                    file_size,
                });
            }
            Ok(output) => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                warn!(
                    "FFmpeg failed to extract subtitle stream {} from {}: {}",
                    stream.stream_index,
                    media_file.display(),
                    stderr.lines().last().unwrap_or("unknown error"),
                );
            }
            Err(e) => {
                warn!(
                    "Failed to run FFmpeg for subtitle extraction (stream {}): {}",
                    stream.stream_index, e
                );
            }
        }
    }

    extracted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_extractable_subtitle() {
        assert!(is_extractable_subtitle("subrip"));
        assert!(is_extractable_subtitle("ass"));
        assert!(is_extractable_subtitle("ssa"));
        assert!(is_extractable_subtitle("webvtt"));
        assert!(is_extractable_subtitle("mov_text"));
        assert!(is_extractable_subtitle("srt"));
        // Bitmap-based subtitles should NOT be extractable
        assert!(!is_extractable_subtitle("hdmv_pgs_subtitle"));
        assert!(!is_extractable_subtitle("dvd_subtitle"));
        assert!(!is_extractable_subtitle("dvb_subtitle"));
    }

    #[test]
    fn test_codec_to_extension() {
        assert_eq!(codec_to_extension("subrip"), "srt");
        assert_eq!(codec_to_extension("srt"), "srt");
        assert_eq!(codec_to_extension("ass"), "ass");
        assert_eq!(codec_to_extension("ssa"), "ass");
        assert_eq!(codec_to_extension("webvtt"), "vtt");
        assert_eq!(codec_to_extension("mov_text"), "srt");
    }
}
