use serde::Serialize;

/// A quality variant for adaptive bitrate HLS streaming.
/// Each variant defines a target resolution and bitrate that FFmpeg will encode to.
#[derive(Debug, Clone, Serialize)]
pub struct QualityVariant {
    /// Human-readable label (e.g. "1080p", "720p")
    pub label: String,
    /// Target video height in pixels
    pub height: u32,
    /// Target video width in pixels (0 = auto-scale to maintain aspect ratio)
    pub width: u32,
    /// Target video bitrate in kbps
    pub video_bitrate_kbps: u32,
    /// Target audio bitrate in kbps
    pub audio_bitrate_kbps: u32,
    /// Total bandwidth in bits/sec for HLS master playlist BANDWIDTH tag
    pub bandwidth_bps: u64,
}

/// Standard quality ladder for adaptive bitrate streaming.
/// Returns variants sorted from highest to lowest quality.
pub fn standard_variants() -> Vec<QualityVariant> {
    vec![
        QualityVariant {
            label: "2160p".into(),
            height: 2160,
            width: 3840,
            video_bitrate_kbps: 14000,
            audio_bitrate_kbps: 192,
            bandwidth_bps: 15_000_000,
        },
        QualityVariant {
            label: "1080p".into(),
            height: 1080,
            width: 1920,
            video_bitrate_kbps: 5000,
            audio_bitrate_kbps: 192,
            bandwidth_bps: 5_500_000,
        },
        QualityVariant {
            label: "720p".into(),
            height: 720,
            width: 1280,
            video_bitrate_kbps: 2800,
            audio_bitrate_kbps: 128,
            bandwidth_bps: 3_100_000,
        },
        QualityVariant {
            label: "480p".into(),
            height: 480,
            width: 854,
            video_bitrate_kbps: 1400,
            audio_bitrate_kbps: 128,
            bandwidth_bps: 1_600_000,
        },
        QualityVariant {
            label: "360p".into(),
            height: 360,
            width: 640,
            video_bitrate_kbps: 800,
            audio_bitrate_kbps: 96,
            bandwidth_bps: 950_000,
        },
    ]
}

/// Select which quality variants to offer based on the source media dimensions.
/// Rules:
/// - Never upscale: only include variants whose height ≤ source height.
/// - Always include at least one variant (the closest to source resolution).
/// - The highest variant uses the source's native resolution (no scaling).
pub fn select_variants(
    source_width: Option<u32>,
    source_height: Option<u32>,
) -> Vec<QualityVariant> {
    let src_h = source_height.unwrap_or(1080);
    let src_w = source_width.unwrap_or(1920);

    let all = standard_variants();

    // Filter to variants that don't upscale
    let mut selected: Vec<QualityVariant> = all.into_iter().filter(|v| v.height <= src_h).collect();

    // If source is smaller than all standard variants, use the smallest one
    // but adjust its resolution to match the source
    if selected.is_empty() {
        selected.push(QualityVariant {
            label: format!("{}p", src_h),
            height: src_h,
            width: src_w,
            video_bitrate_kbps: 800,
            audio_bitrate_kbps: 96,
            bandwidth_bps: 950_000,
        });
        return selected;
    }

    // Ensure the top variant matches the source's native resolution so the
    // highest quality path has no scaling — this enables -c:v copy for H.264.
    if let Some(top) = selected.first() {
        if top.height == src_h {
            // Already matches a standard tier, just fix the width for accuracy
            let top_mut = &mut selected[0];
            top_mut.width = src_w;
        } else {
            // Source height (e.g. 960p) doesn't match any standard tier.
            // Insert a native-resolution variant at the top so we avoid scaling.
            let native = QualityVariant {
                label: format!("{}p", src_h),
                height: src_h,
                width: src_w,
                video_bitrate_kbps: top.video_bitrate_kbps,
                audio_bitrate_kbps: top.audio_bitrate_kbps,
                bandwidth_bps: top.bandwidth_bps,
            };
            selected.insert(0, native);
        }
    }

    selected
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2160p_source_gets_all_variants() {
        let variants = select_variants(Some(3840), Some(2160));
        assert_eq!(variants.len(), 5);
        assert_eq!(variants[0].label, "2160p");
        assert_eq!(variants[0].width, 3840);
        assert_eq!(variants[4].label, "360p");
    }

    #[test]
    fn test_960p_source_gets_native_variant_at_top() {
        // 1920x960 (2:1 aspect) doesn't match any standard tier.
        // The top variant should be a native 960p to avoid scaling.
        let variants = select_variants(Some(1920), Some(960));
        assert_eq!(variants[0].label, "960p");
        assert_eq!(variants[0].height, 960);
        assert_eq!(variants[0].width, 1920);
        // 720p and below should follow
        assert_eq!(variants[1].label, "720p");
    }

    #[test]
    fn test_1080p_source_skips_2160p() {
        let variants = select_variants(Some(1920), Some(1080));
        assert_eq!(variants.len(), 4);
        assert_eq!(variants[0].label, "1080p");
        assert_eq!(variants[3].label, "360p");
    }

    #[test]
    fn test_720p_source_skips_1080p_and_2160p() {
        let variants = select_variants(Some(1280), Some(720));
        assert_eq!(variants.len(), 3);
        assert_eq!(variants[0].label, "720p");
        assert_eq!(variants[0].width, 1280);
    }

    #[test]
    fn test_480p_source_gets_two_variants() {
        let variants = select_variants(Some(854), Some(480));
        assert_eq!(variants.len(), 2);
        assert_eq!(variants[0].label, "480p");
        assert_eq!(variants[1].label, "360p");
    }

    #[test]
    fn test_small_source_gets_one_variant() {
        let variants = select_variants(Some(320), Some(240));
        assert_eq!(variants.len(), 1);
        assert_eq!(variants[0].label, "240p");
        assert_eq!(variants[0].height, 240);
    }

    #[test]
    fn test_none_dimensions_defaults_to_1080p() {
        let variants = select_variants(None, None);
        assert_eq!(variants.len(), 4);
        assert_eq!(variants[0].label, "1080p");
    }
}
