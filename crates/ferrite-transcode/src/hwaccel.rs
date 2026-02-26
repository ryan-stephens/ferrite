use serde::{Deserialize, Serialize};
use std::process::Stdio;
use tokio::process::Command;
use tracing::{debug, info, warn};

/// Supported hardware acceleration backends.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HwAccelBackend {
    /// NVIDIA NVENC (h264_nvenc)
    Nvenc,
    /// Intel Quick Sync Video (h264_qsv)
    Qsv,
    /// Video Acceleration API — Linux only (h264_vaapi)
    Vaapi,
    /// Software encoding (libx264) — always available
    Software,
}

impl std::fmt::Display for HwAccelBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HwAccelBackend::Nvenc => write!(f, "nvenc"),
            HwAccelBackend::Qsv => write!(f, "qsv"),
            HwAccelBackend::Vaapi => write!(f, "vaapi"),
            HwAccelBackend::Software => write!(f, "software"),
        }
    }
}

/// FFmpeg encoder arguments for H.264 encoding.
/// Encapsulates the encoder name and its specific quality/compatibility args.
#[derive(Debug, Clone, Serialize)]
pub struct EncoderProfile {
    pub backend: HwAccelBackend,
    /// FFmpeg encoder name (e.g. "libx264", "h264_nvenc", "h264_qsv")
    pub encoder_name: String,
    /// Additional FFmpeg args for this encoder (preset, quality, profile, etc.)
    pub encoder_args: Vec<String>,
    /// Whether hardware decoding should be used (hwaccel input flag)
    pub hw_decode_args: Vec<String>,
}

impl EncoderProfile {
    /// Software fallback — always works.
    pub fn software() -> Self {
        Self {
            backend: HwAccelBackend::Software,
            encoder_name: "libx264".to_string(),
            encoder_args: vec![
                "-preset".into(),
                "veryfast".into(),
                "-crf".into(),
                "23".into(),
                "-profile:v".into(),
                "high".into(),
                "-level".into(),
                "4.1".into(),
                "-pix_fmt".into(),
                "yuv420p".into(),
            ],
            hw_decode_args: vec![],
        }
    }

    /// NVIDIA NVENC profile.
    fn nvenc() -> Self {
        Self {
            backend: HwAccelBackend::Nvenc,
            encoder_name: "h264_nvenc".to_string(),
            encoder_args: vec![
                "-preset".into(),
                "p4".into(), // balanced speed/quality
                "-tune".into(),
                "ll".into(), // low latency for streaming
                "-rc".into(),
                "vbr".into(), // variable bitrate
                "-cq".into(),
                "23".into(), // constant quality (like CRF)
                "-profile:v".into(),
                "high".into(),
                "-level".into(),
                "4.1".into(),
                "-pix_fmt".into(),
                "yuv420p".into(),
            ],
            hw_decode_args: vec![
                "-hwaccel".into(),
                "cuda".into(),
                "-hwaccel_output_format".into(),
                "cuda".into(),
            ],
        }
    }

    /// Intel QSV profile.
    fn qsv() -> Self {
        Self {
            backend: HwAccelBackend::Qsv,
            encoder_name: "h264_qsv".to_string(),
            encoder_args: vec![
                "-preset".into(),
                "veryfast".into(),
                "-global_quality".into(),
                "23".into(),
                "-profile:v".into(),
                "high".into(),
                "-level".into(),
                "4.1".into(),
            ],
            hw_decode_args: vec![
                "-hwaccel".into(),
                "qsv".into(),
                "-hwaccel_output_format".into(),
                "qsv".into(),
            ],
        }
    }

    /// VAAPI profile (Linux).
    fn vaapi() -> Self {
        Self {
            backend: HwAccelBackend::Vaapi,
            encoder_name: "h264_vaapi".to_string(),
            encoder_args: vec![
                "-qp".into(),
                "23".into(),
                "-profile:v".into(),
                "high".into(),
                "-level".into(),
                "4.1".into(),
            ],
            hw_decode_args: vec![
                "-hwaccel".into(),
                "vaapi".into(),
                "-hwaccel_output_format".into(),
                "vaapi".into(),
                "-vaapi_device".into(),
                "/dev/dri/renderD128".into(),
            ],
        }
    }

    /// Get the FFmpeg args to set the video encoder (placed after -map).
    /// Returns: ["-c:v", "<encoder>", ...encoder_args]
    /// Builds the vec by iterating encoder_args by reference to avoid a full clone.
    pub fn video_encode_args(&self) -> Vec<String> {
        let mut args = Vec::with_capacity(2 + self.encoder_args.len());
        args.push("-c:v".to_string());
        args.push(self.encoder_name.clone());
        args.extend(self.encoder_args.iter().cloned());
        args
    }

    /// Get the FFmpeg args to set the video encoder, but WITHOUT `-pix_fmt`.
    /// Use this when pixel format conversion is handled by a video filter chain
    /// (e.g. HDR tone-mapping already outputs `format=yuv420p`).
    /// Pre-allocates the exact capacity needed to avoid reallocation.
    pub fn video_encode_args_no_pix_fmt(&self) -> Vec<String> {
        let mut args = Vec::with_capacity(2 + self.encoder_args.len());
        args.push("-c:v".to_string());
        args.push(self.encoder_name.clone());
        let mut skip_next = false;
        for arg in &self.encoder_args {
            if skip_next {
                skip_next = false;
                continue;
            }
            if arg == "-pix_fmt" {
                skip_next = true;
                continue;
            }
            args.push(arg.clone());
        }
        args
    }

    /// Get the FFmpeg args for hardware-accelerated decoding (placed before -i).
    /// If `has_software_filters` is true, we strip `-hwaccel_output_format` so that
    /// the hardware decoder automatically downloads frames to CPU memory for filtering.
    pub fn hw_input_args(&self, has_software_filters: bool) -> Vec<String> {
        if has_software_filters {
            let mut args = Vec::new();
            let mut skip_next = false;
            for arg in &self.hw_decode_args {
                if skip_next {
                    skip_next = false;
                    continue;
                }
                if arg == "-hwaccel_output_format" {
                    skip_next = true;
                    continue;
                }
                args.push(arg.clone());
            }
            args
        } else {
            self.hw_decode_args.clone()
        }
    }

    /// Whether this profile uses hardware acceleration.
    pub fn is_hardware(&self) -> bool {
        self.backend != HwAccelBackend::Software
    }
}

/// Detected hardware capabilities from probing FFmpeg.
#[derive(Debug, Clone, Serialize)]
pub struct HwCapabilities {
    pub nvenc_available: bool,
    pub qsv_available: bool,
    pub vaapi_available: bool,
    pub selected_profile: EncoderProfile,
}

/// Probe FFmpeg for available hardware encoders and select the best one.
/// Priority: NVENC > QSV > VAAPI > Software
///
/// `preferred` allows the user to force a specific backend via config.
/// If the preferred backend is not available, falls back to auto-detection.
pub async fn detect_and_select(
    ffmpeg_path: &str,
    preferred: Option<HwAccelBackend>,
) -> HwCapabilities {
    let available = probe_encoders(ffmpeg_path).await;

    info!(
        "HW encoder detection: nvenc={}, qsv={}, vaapi={}",
        available.0, available.1, available.2
    );

    let profile = match preferred {
        Some(HwAccelBackend::Nvenc) if available.0 => {
            info!("Using preferred HW encoder: NVENC");
            EncoderProfile::nvenc()
        }
        Some(HwAccelBackend::Qsv) if available.1 => {
            info!("Using preferred HW encoder: QSV");
            EncoderProfile::qsv()
        }
        Some(HwAccelBackend::Vaapi) if available.2 => {
            info!("Using preferred HW encoder: VAAPI");
            EncoderProfile::vaapi()
        }
        Some(HwAccelBackend::Software) => {
            info!("Using software encoder (forced by config)");
            EncoderProfile::software()
        }
        Some(pref) => {
            warn!(
                "Preferred HW encoder {:?} not available, falling back to auto-detect",
                pref
            );
            auto_select(available)
        }
        None => auto_select(available),
    };

    HwCapabilities {
        nvenc_available: available.0,
        qsv_available: available.1,
        vaapi_available: available.2,
        selected_profile: profile,
    }
}

/// Auto-select the best available encoder. Priority: NVENC > QSV > VAAPI > Software.
fn auto_select(available: (bool, bool, bool)) -> EncoderProfile {
    if available.0 {
        info!("Auto-selected HW encoder: NVENC");
        EncoderProfile::nvenc()
    } else if available.1 {
        info!("Auto-selected HW encoder: QSV");
        EncoderProfile::qsv()
    } else if available.2 {
        info!("Auto-selected HW encoder: VAAPI");
        EncoderProfile::vaapi()
    } else {
        info!("No HW encoders available, using software (libx264)");
        EncoderProfile::software()
    }
}

/// Probe FFmpeg for available H.264 hardware encoders.
/// Returns (nvenc, qsv, vaapi) availability.
async fn probe_encoders(ffmpeg_path: &str) -> (bool, bool, bool) {
    let output = match Command::new(ffmpeg_path)
        .args(["-hide_banner", "-encoders"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .await
    {
        Ok(o) => o,
        Err(e) => {
            warn!("Failed to probe FFmpeg encoders: {}", e);
            return (false, false, false);
        }
    };

    let text = String::from_utf8_lossy(&output.stdout);

    let nvenc = text.contains("h264_nvenc");
    let qsv = text.contains("h264_qsv");
    let vaapi = text.contains("h264_vaapi");

    debug!(
        "FFmpeg encoder probe: h264_nvenc={}, h264_qsv={}, h264_vaapi={}",
        nvenc, qsv, vaapi
    );

    (nvenc, qsv, vaapi)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_software_profile_args() {
        let profile = EncoderProfile::software();
        let args = profile.video_encode_args();
        assert_eq!(args[0], "-c:v");
        assert_eq!(args[1], "libx264");
        assert!(args.contains(&"-preset".to_string()));
        assert!(args.contains(&"veryfast".to_string()));
        assert!(!profile.is_hardware());
    }

    #[test]
    fn test_nvenc_profile_args() {
        let profile = EncoderProfile::nvenc();
        let args = profile.video_encode_args();
        assert_eq!(args[0], "-c:v");
        assert_eq!(args[1], "h264_nvenc");
        assert!(profile.is_hardware());
        assert!(!profile.hw_input_args(false).is_empty());
    }

    #[test]
    fn test_auto_select_nvenc_priority() {
        let profile = auto_select((true, true, true));
        assert_eq!(profile.backend, HwAccelBackend::Nvenc);
    }

    #[test]
    fn test_auto_select_qsv_fallback() {
        let profile = auto_select((false, true, true));
        assert_eq!(profile.backend, HwAccelBackend::Qsv);
    }

    #[test]
    fn test_auto_select_vaapi_fallback() {
        let profile = auto_select((false, false, true));
        assert_eq!(profile.backend, HwAccelBackend::Vaapi);
    }

    #[test]
    fn test_auto_select_software_fallback() {
        let profile = auto_select((false, false, false));
        assert_eq!(profile.backend, HwAccelBackend::Software);
    }
}
