use crate::error::ApiError;
use crate::state::AppState;
use axum::extract::{Path, State};
use axum::http::header;
use axum::response::IntoResponse;
use axum::Json;
use ferrite_db::subtitle_repo;
use tokio::fs;

/// GET /api/media/{id}/subtitles — list all external subtitles for a media item
pub async fn list_subtitles(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let subs = subtitle_repo::get_subtitles(&state.db.read, &id).await?;
    Ok(Json(subs))
}

/// GET /api/subtitles/{id}/serve — serve a subtitle file, converting to VTT if needed.
/// Browsers only support WebVTT natively for <track> elements, so SRT/ASS/SSA
/// files are converted on-the-fly to VTT.
pub async fn serve_subtitle(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, ApiError> {
    let sub = subtitle_repo::get_subtitle_by_id(&state.db.read, id)
        .await?
        .ok_or_else(|| ApiError::not_found(format!("Subtitle {id} not found")))?;

    let content = fs::read_to_string(&sub.file_path).await.map_err(|e| {
        tracing::error!("Failed to read subtitle file {}: {}", sub.file_path, e);
        ApiError::internal("Failed to read subtitle file")
    })?;

    let vtt = match sub.format.as_str() {
        "vtt" => content,
        "srt" => srt_to_vtt(&content),
        "ass" | "ssa" => ass_to_vtt(&content),
        _ => {
            return Err(ApiError::bad_request(format!(
                "Unsupported subtitle format '{}' for browser playback",
                sub.format
            )));
        }
    };

    Ok(([(header::CONTENT_TYPE, "text/vtt; charset=utf-8")], vtt))
}

/// Convert SRT subtitle text to WebVTT format.
/// SRT uses commas in timestamps (00:01:23,456) while VTT uses dots (00:01:23.456).
/// VTT also requires a "WEBVTT" header line.
fn srt_to_vtt(srt: &str) -> String {
    let mut vtt = String::with_capacity(srt.len() + 16);
    vtt.push_str("WEBVTT\n\n");

    for line in srt.lines() {
        // Replace SRT timestamp commas with VTT dots
        // Pattern: 00:01:23,456 --> 00:01:24,789
        if line.contains(" --> ") && line.contains(',') {
            vtt.push_str(&line.replace(',', "."));
        } else {
            vtt.push_str(line);
        }
        vtt.push('\n');
    }

    vtt
}

/// Convert ASS/SSA subtitle text to WebVTT format.
/// Extracts dialogue lines and converts timestamps.
/// ASS timestamps: h:mm:ss.cc (centiseconds)
/// VTT timestamps: hh:mm:ss.mmm (milliseconds)
fn ass_to_vtt(ass: &str) -> String {
    let mut vtt = String::with_capacity(ass.len());
    vtt.push_str("WEBVTT\n\n");

    let mut cue_index = 1u32;

    for line in ass.lines() {
        // Dialogue lines: Dialogue: Layer,Start,End,Style,Name,MarginL,MarginR,MarginV,Effect,Text
        if let Some(rest) = line.strip_prefix("Dialogue:") {
            let fields: Vec<&str> = rest.splitn(10, ',').collect();
            if fields.len() >= 10 {
                let start = fields[1].trim();
                let end = fields[2].trim();
                let text = fields[9].trim();

                if let (Some(vtt_start), Some(vtt_end)) =
                    (ass_time_to_vtt(start), ass_time_to_vtt(end))
                {
                    // Strip ASS override tags like {\b1}, {\i1}, {\an8}, etc.
                    let clean_text = strip_ass_tags(text);
                    // Convert \N to newlines
                    let clean_text = clean_text.replace("\\N", "\n").replace("\\n", "\n");

                    if !clean_text.trim().is_empty() {
                        vtt.push_str(&format!("{cue_index}\n"));
                        vtt.push_str(&format!("{vtt_start} --> {vtt_end}\n"));
                        vtt.push_str(&clean_text);
                        vtt.push_str("\n\n");
                        cue_index += 1;
                    }
                }
            }
        }
    }

    vtt
}

/// Convert ASS timestamp (h:mm:ss.cc) to VTT timestamp (hh:mm:ss.mmm).
fn ass_time_to_vtt(ass_time: &str) -> Option<String> {
    let parts: Vec<&str> = ass_time.split(':').collect();
    if parts.len() != 3 {
        return None;
    }

    let hours: u32 = parts[0].parse().ok()?;
    let minutes: u32 = parts[1].parse().ok()?;

    // Seconds part: "ss.cc" (centiseconds)
    let sec_parts: Vec<&str> = parts[2].split('.').collect();
    if sec_parts.len() != 2 {
        return None;
    }

    let seconds: u32 = sec_parts[0].parse().ok()?;
    let centiseconds: u32 = sec_parts[1].parse().ok()?;
    let milliseconds = centiseconds * 10;

    Some(format!(
        "{:02}:{:02}:{:02}.{:03}",
        hours, minutes, seconds, milliseconds
    ))
}

/// Strip ASS override tags like {\b1}, {\i0}, {\an8}, {\pos(x,y)}, etc.
fn strip_ass_tags(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut in_tag = false;

    for ch in text.chars() {
        if ch == '{' {
            in_tag = true;
        } else if ch == '}' {
            in_tag = false;
        } else if !in_tag {
            result.push(ch);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_srt_to_vtt_basic() {
        let srt = "1\n00:00:01,000 --> 00:00:04,000\nHello world\n\n2\n00:00:05,500 --> 00:00:08,200\nSecond line\n";
        let vtt = srt_to_vtt(srt);
        assert!(vtt.starts_with("WEBVTT\n\n"));
        assert!(vtt.contains("00:00:01.000 --> 00:00:04.000"));
        assert!(vtt.contains("00:00:05.500 --> 00:00:08.200"));
        assert!(vtt.contains("Hello world"));
    }

    #[test]
    fn test_ass_time_to_vtt() {
        assert_eq!(
            ass_time_to_vtt("0:00:01.00"),
            Some("00:00:01.000".to_string())
        );
        assert_eq!(
            ass_time_to_vtt("1:23:45.67"),
            Some("01:23:45.670".to_string())
        );
        assert_eq!(
            ass_time_to_vtt("0:02:30.50"),
            Some("00:02:30.500".to_string())
        );
    }

    #[test]
    fn test_strip_ass_tags() {
        assert_eq!(strip_ass_tags(r"{\b1}Bold text{\b0}"), "Bold text");
        assert_eq!(strip_ass_tags(r"{\an8}Top text"), "Top text");
        assert_eq!(strip_ass_tags("No tags here"), "No tags here");
        assert_eq!(strip_ass_tags(r"{\pos(320,50)}Positioned"), "Positioned");
    }

    #[test]
    fn test_ass_to_vtt_dialogue() {
        let ass = "[Events]\nDialogue: 0,0:00:01.00,0:00:04.00,Default,,0,0,0,,Hello world\nDialogue: 0,0:00:05.50,0:00:08.20,Default,,0,0,0,,{\\b1}Bold{\\b0}\n";
        let vtt = ass_to_vtt(ass);
        assert!(vtt.starts_with("WEBVTT\n\n"));
        assert!(vtt.contains("00:00:01.000 --> 00:00:04.000"));
        assert!(vtt.contains("Hello world"));
        assert!(vtt.contains("Bold"));
        assert!(!vtt.contains("{\\b1}"));
    }
}
