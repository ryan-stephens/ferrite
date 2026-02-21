/// Audio codecs that browsers can play natively in HLS/fMP4 containers.
/// When the source uses one of these, we can copy the audio stream instead of
/// re-encoding to stereo AAC, preserving surround sound (5.1/7.1).
const PASSTHROUGH_CODECS: &[&str] = &[
    "aac",  // AAC (LC, HE-AAC) — universal browser support
    "mp3",  // MP3 — widely supported in fMP4
    "opus", // Opus — supported in modern browsers (Chrome, Firefox, Edge)
    "flac", // FLAC — supported in fMP4 by Chrome/Edge
    "alac", // Apple Lossless — Safari
];

/// Check if the source audio codec can be passed through to the browser
/// without re-encoding. Returns `true` if the codec is browser-compatible.
pub fn can_passthrough(audio_codec: &str) -> bool {
    PASSTHROUGH_CODECS.contains(&audio_codec.to_lowercase().as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aac_passthrough() {
        assert!(can_passthrough("aac"));
        assert!(can_passthrough("AAC"));
    }

    #[test]
    fn opus_passthrough() {
        assert!(can_passthrough("opus"));
    }

    #[test]
    fn mp3_passthrough() {
        assert!(can_passthrough("mp3"));
    }

    #[test]
    fn flac_passthrough() {
        assert!(can_passthrough("flac"));
    }

    #[test]
    fn dts_no_passthrough() {
        assert!(!can_passthrough("dts"));
    }

    #[test]
    fn ac3_no_passthrough() {
        assert!(!can_passthrough("ac3"));
    }

    #[test]
    fn eac3_no_passthrough() {
        assert!(!can_passthrough("eac3"));
    }

    #[test]
    fn truehd_no_passthrough() {
        assert!(!can_passthrough("truehd"));
    }
}
