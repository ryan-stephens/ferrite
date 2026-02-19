/// Audio codecs that browsers can play natively in HTML5 <video>.
const BROWSER_COMPATIBLE_AUDIO: &[&str] = &[
    "aac", "mp3", "opus", "vorbis", "flac", "pcm_s16le", "pcm_s24le", "pcm_f32le",
];

/// Video codecs that browsers can play natively.
const BROWSER_COMPATIBLE_VIDEO: &[&str] = &["h264", "vp8", "vp9", "av1"];

/// Container formats that browsers can play natively.
const BROWSER_COMPATIBLE_CONTAINERS: &[&str] = &["mp4", "mov", "webm", "ogg", "flac", "wav"];

/// Check if the audio codec is browser-compatible.
pub fn is_audio_compatible(codec: &str) -> bool {
    BROWSER_COMPATIBLE_AUDIO.contains(&codec.to_lowercase().as_str())
}

/// Check if the video codec is browser-compatible.
pub fn is_video_compatible(codec: &str) -> bool {
    BROWSER_COMPATIBLE_VIDEO.contains(&codec.to_lowercase().as_str())
}

/// Check if a container format is browser-compatible.
pub fn is_container_compatible(format: &str) -> bool {
    BROWSER_COMPATIBLE_CONTAINERS.contains(&format.to_lowercase().as_str())
}

/// Determine what streaming strategy to use for a given file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StreamStrategy {
    /// File can be served directly — all codecs and container are browser-compatible.
    DirectPlay,
    /// Video and audio are compatible but container isn't (e.g. MKV with H.264+AAC).
    /// Remux to MP4 with `-c:v copy -c:a copy` — zero CPU cost.
    Remux,
    /// Video is compatible but audio needs transcoding. Remux with audio transcode to AAC.
    AudioTranscode,
    /// Video needs transcoding (HEVC/AV1/etc → H.264). Full re-encode.
    FullTranscode,
}

/// Analyze a media item's codecs and determine the best streaming strategy.
pub fn determine_strategy(
    container_format: Option<&str>,
    video_codec: Option<&str>,
    audio_codec: Option<&str>,
) -> StreamStrategy {
    let container_ok = container_format
        .map(is_container_compatible)
        .unwrap_or(false);
    let video_ok = video_codec
        .map(is_video_compatible)
        .unwrap_or(true); // No video = audio-only, that's fine
    let audio_ok = audio_codec
        .map(is_audio_compatible)
        .unwrap_or(true); // No audio = silent video, serve directly

    if container_ok && video_ok && audio_ok {
        StreamStrategy::DirectPlay
    } else if video_ok && audio_ok && !container_ok {
        // Both codecs are browser-compatible, just the container is wrong
        // (e.g. MKV with H.264 + AAC). Zero-cost remux to MP4.
        StreamStrategy::Remux
    } else if video_ok && !audio_ok {
        // Video is fine, just need to transcode audio (cheap operation)
        StreamStrategy::AudioTranscode
    } else {
        // Video needs transcoding (HEVC, AV1, MPEG2, etc.)
        StreamStrategy::FullTranscode
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mp4_h264_aac_is_direct() {
        assert_eq!(
            determine_strategy(Some("mp4"), Some("h264"), Some("aac")),
            StreamStrategy::DirectPlay,
        );
    }

    #[test]
    fn test_mkv_h264_ac3_needs_audio_transcode() {
        assert_eq!(
            determine_strategy(Some("matroska"), Some("h264"), Some("ac3")),
            StreamStrategy::AudioTranscode,
        );
    }

    #[test]
    fn test_mkv_h264_dts_needs_audio_transcode() {
        assert_eq!(
            determine_strategy(Some("matroska"), Some("h264"), Some("dts")),
            StreamStrategy::AudioTranscode,
        );
    }

    #[test]
    fn test_mkv_h264_eac3_needs_audio_transcode() {
        assert_eq!(
            determine_strategy(Some("matroska"), Some("h264"), Some("eac3")),
            StreamStrategy::AudioTranscode,
        );
    }

    #[test]
    fn test_mp4_h264_opus_is_direct() {
        assert_eq!(
            determine_strategy(Some("mp4"), Some("h264"), Some("opus")),
            StreamStrategy::DirectPlay,
        );
    }

    #[test]
    fn test_hevc_needs_full_transcode() {
        assert_eq!(
            determine_strategy(Some("matroska"), Some("hevc"), Some("ac3")),
            StreamStrategy::FullTranscode,
        );
    }

    #[test]
    fn test_webm_vp9_opus_is_direct() {
        assert_eq!(
            determine_strategy(Some("webm"), Some("vp9"), Some("opus")),
            StreamStrategy::DirectPlay,
        );
    }

    // ---- Remux tests ----

    #[test]
    fn test_mkv_h264_aac_is_remux() {
        assert_eq!(
            determine_strategy(Some("matroska"), Some("h264"), Some("aac")),
            StreamStrategy::Remux,
        );
    }

    #[test]
    fn test_mkv_h264_opus_is_remux() {
        assert_eq!(
            determine_strategy(Some("matroska"), Some("h264"), Some("opus")),
            StreamStrategy::Remux,
        );
    }

    #[test]
    fn test_mkv_h264_flac_is_remux() {
        assert_eq!(
            determine_strategy(Some("matroska"), Some("h264"), Some("flac")),
            StreamStrategy::Remux,
        );
    }

    #[test]
    fn test_mkv_vp9_opus_is_remux() {
        assert_eq!(
            determine_strategy(Some("matroska"), Some("vp9"), Some("opus")),
            StreamStrategy::Remux,
        );
    }

    #[test]
    fn test_avi_h264_aac_is_remux() {
        assert_eq!(
            determine_strategy(Some("avi"), Some("h264"), Some("aac")),
            StreamStrategy::Remux,
        );
    }

    #[test]
    fn test_flac_container_direct_play() {
        assert_eq!(
            determine_strategy(Some("flac"), None, Some("flac")),
            StreamStrategy::DirectPlay,
        );
    }

    #[test]
    fn test_wav_container_direct_play() {
        assert_eq!(
            determine_strategy(Some("wav"), None, Some("pcm_s16le")),
            StreamStrategy::DirectPlay,
        );
    }
}
