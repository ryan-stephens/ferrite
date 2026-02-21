/// Client-specific playback capability profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClientProfile {
    WebChrome,
    SafariIos,
    Android,
    Tvos,
    Roku,
}

impl ClientProfile {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::WebChrome => "web-chrome",
            Self::SafariIos => "safari-ios",
            Self::Android => "android",
            Self::Tvos => "tvos",
            Self::Roku => "roku",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "web-chrome" | "chrome" | "web" | "default" => Some(Self::WebChrome),
            "safari-ios" | "ios" | "iphone" | "ipad" => Some(Self::SafariIos),
            "android" => Some(Self::Android),
            "tvos" | "apple-tv" | "appletv" => Some(Self::Tvos),
            "roku" => Some(Self::Roku),
            _ => None,
        }
    }
}

struct ClientCapabilities {
    audio: &'static [&'static str],
    video: &'static [&'static str],
    containers: &'static [&'static str],
}

const WEB_CHROME_AUDIO: &[&str] = &[
    "aac",
    "mp3",
    "opus",
    "vorbis",
    "flac",
    "pcm_s16le",
    "pcm_s24le",
    "pcm_f32le",
];
const WEB_CHROME_VIDEO: &[&str] = &["h264", "vp8", "vp9", "av1"];
const WEB_CHROME_CONTAINERS: &[&str] = &["mp4", "mov", "webm", "ogg", "flac", "wav"];

const SAFARI_IOS_AUDIO: &[&str] = &["aac", "mp3", "alac"];
const SAFARI_IOS_VIDEO: &[&str] = &["h264", "hevc"];
const SAFARI_IOS_CONTAINERS: &[&str] = &["mp4", "mov", "m4v", "m4a"];

const ANDROID_AUDIO: &[&str] = &["aac", "mp3", "opus", "vorbis", "flac"];
const ANDROID_VIDEO: &[&str] = &["h264", "vp8", "vp9", "av1", "hevc"];
const ANDROID_CONTAINERS: &[&str] = &["mp4", "mov", "webm"];

const TVOS_AUDIO: &[&str] = &["aac", "mp3", "alac", "ac3", "eac3"];
const TVOS_VIDEO: &[&str] = &["h264", "hevc"];
const TVOS_CONTAINERS: &[&str] = &["mp4", "mov", "m4v", "m4a"];

const ROKU_AUDIO: &[&str] = &["aac", "mp3", "ac3", "eac3"];
const ROKU_VIDEO: &[&str] = &["h264", "hevc"];
const ROKU_CONTAINERS: &[&str] = &["mp4", "mov", "mkv", "matroska"];

fn capabilities_for(profile: ClientProfile) -> ClientCapabilities {
    match profile {
        ClientProfile::WebChrome => ClientCapabilities {
            audio: WEB_CHROME_AUDIO,
            video: WEB_CHROME_VIDEO,
            containers: WEB_CHROME_CONTAINERS,
        },
        ClientProfile::SafariIos => ClientCapabilities {
            audio: SAFARI_IOS_AUDIO,
            video: SAFARI_IOS_VIDEO,
            containers: SAFARI_IOS_CONTAINERS,
        },
        ClientProfile::Android => ClientCapabilities {
            audio: ANDROID_AUDIO,
            video: ANDROID_VIDEO,
            containers: ANDROID_CONTAINERS,
        },
        ClientProfile::Tvos => ClientCapabilities {
            audio: TVOS_AUDIO,
            video: TVOS_VIDEO,
            containers: TVOS_CONTAINERS,
        },
        ClientProfile::Roku => ClientCapabilities {
            audio: ROKU_AUDIO,
            video: ROKU_VIDEO,
            containers: ROKU_CONTAINERS,
        },
    }
}

fn contains_ignore_ascii(values: &[&str], candidate: &str) -> bool {
    values.iter().any(|v| v.eq_ignore_ascii_case(candidate))
}

fn infer_profile_from_ua(
    user_agent: Option<&str>,
    sec_ch_ua_platform: Option<&str>,
) -> ClientProfile {
    let ua = user_agent.unwrap_or_default().to_ascii_lowercase();
    let platform = sec_ch_ua_platform.unwrap_or_default().to_ascii_lowercase();

    if ua.contains("roku") {
        return ClientProfile::Roku;
    }
    if ua.contains("appletv") || ua.contains("apple tv") || ua.contains("tvos") {
        return ClientProfile::Tvos;
    }
    if ua.contains("iphone")
        || ua.contains("ipad")
        || ua.contains("ipod")
        || platform.contains("ios")
    {
        return ClientProfile::SafariIos;
    }
    if ua.contains("android") || platform.contains("android") {
        return ClientProfile::Android;
    }
    ClientProfile::WebChrome
}

/// Resolve a client profile using explicit override first, then UA/platform heuristics.
pub fn resolve_client_profile(
    explicit_override: Option<&str>,
    user_agent: Option<&str>,
    sec_ch_ua_platform: Option<&str>,
) -> ClientProfile {
    if let Some(profile) = explicit_override.and_then(ClientProfile::parse) {
        return profile;
    }
    infer_profile_from_ua(user_agent, sec_ch_ua_platform)
}

/// Check if the audio codec is compatible for a specific client profile.
pub fn is_audio_compatible(profile: ClientProfile, codec: &str) -> bool {
    contains_ignore_ascii(capabilities_for(profile).audio, codec)
}

/// Check if the video codec is compatible for a specific client profile.
pub fn is_video_compatible(profile: ClientProfile, codec: &str) -> bool {
    contains_ignore_ascii(capabilities_for(profile).video, codec)
}

/// Check if a container format is compatible for a specific client profile.
pub fn is_container_compatible(profile: ClientProfile, format: &str) -> bool {
    contains_ignore_ascii(capabilities_for(profile).containers, format)
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
pub fn determine_strategy_for_profile(
    profile: ClientProfile,
    container_format: Option<&str>,
    video_codec: Option<&str>,
    audio_codec: Option<&str>,
) -> StreamStrategy {
    let container_ok = container_format
        .map(|format| is_container_compatible(profile, format))
        .unwrap_or(false);
    let video_ok = video_codec
        .map(|codec| is_video_compatible(profile, codec))
        .unwrap_or(true); // No video = audio-only, that's fine
    let audio_ok = audio_codec
        .map(|codec| is_audio_compatible(profile, codec))
        .unwrap_or(true); // No audio = silent video, serve directly

    if container_ok && video_ok && audio_ok {
        StreamStrategy::DirectPlay
    } else if video_ok && audio_ok && !container_ok {
        // Both codecs are client-compatible, just the container is wrong.
        StreamStrategy::Remux
    } else if video_ok && !audio_ok {
        // Video is fine, just need to transcode audio (cheap operation).
        StreamStrategy::AudioTranscode
    } else {
        // Video needs transcoding (HEVC/AV1/etc. for that profile).
        StreamStrategy::FullTranscode
    }
}

/// Analyze a media item's codecs and determine the best streaming strategy.
/// Defaults to the baseline web profile for compatibility with existing callsites.
pub fn determine_strategy(
    container_format: Option<&str>,
    video_codec: Option<&str>,
    audio_codec: Option<&str>,
) -> StreamStrategy {
    determine_strategy_for_profile(
        ClientProfile::WebChrome,
        container_format,
        video_codec,
        audio_codec,
    )
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

    #[test]
    fn test_profile_override_wins_over_user_agent() {
        let profile = resolve_client_profile(
            Some("roku"),
            Some("Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X)"),
            None,
        );
        assert_eq!(profile, ClientProfile::Roku);
    }

    #[test]
    fn test_profile_inferred_from_ios_user_agent() {
        let profile = resolve_client_profile(
            None,
            Some("Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X)"),
            None,
        );
        assert_eq!(profile, ClientProfile::SafariIos);
    }

    #[test]
    fn test_profile_inferred_from_sec_ch_platform() {
        let profile = resolve_client_profile(None, None, Some("Android"));
        assert_eq!(profile, ClientProfile::Android);
    }

    #[test]
    fn test_safari_ios_hevc_mp4_is_direct() {
        assert_eq!(
            determine_strategy_for_profile(
                ClientProfile::SafariIos,
                Some("mp4"),
                Some("hevc"),
                Some("aac"),
            ),
            StreamStrategy::DirectPlay,
        );
    }

    #[test]
    fn test_web_chrome_hevc_mp4_is_full_transcode() {
        assert_eq!(
            determine_strategy_for_profile(
                ClientProfile::WebChrome,
                Some("mp4"),
                Some("hevc"),
                Some("aac"),
            ),
            StreamStrategy::FullTranscode,
        );
    }
}
