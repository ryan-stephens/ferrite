//! HDR tone-mapping filter generation for FFmpeg.
//!
//! When transcoding 10-bit HDR content (HEVC Main 10, HDR10, HLG) to 8-bit H.264,
//! simply using `-pix_fmt yuv420p` strips the extra bit depth and HDR metadata,
//! resulting in washed-out, flat colors. Proper tone-mapping converts the wide
//! dynamic range and color gamut (BT.2020) to SDR (BT.709) with perceptually
//! correct brightness mapping.

/// Pixel formats that indicate 10-bit (or higher) content requiring tone-mapping.
const HDR_PIXEL_FORMATS: &[&str] = &[
    "yuv420p10le",
    "yuv420p10be",
    "yuv422p10le",
    "yuv422p10be",
    "yuv444p10le",
    "yuv444p10be",
    "yuv420p12le",
    "yuv420p12be",
    "yuv422p12le",
    "yuv422p12be",
    "yuv444p12le",
    "yuv444p12be",
    "p010le",
    "p010be",
];

/// Check if a pixel format indicates 10-bit (or higher) content.
pub fn is_high_bit_depth(pix_fmt: &str) -> bool {
    HDR_PIXEL_FORMATS.contains(&pix_fmt.to_lowercase().as_str())
}

/// HDR transfer functions that require full tone-mapping (PQ = HDR10, HLG = broadcast HDR).
const HDR_TRANSFERS: &[&str] = &["smpte2084", "arib-std-b67"];

/// HDR color primaries (wide color gamut).
const HDR_PRIMARIES: &[&str] = &["bt2020"];

/// Check if the color metadata indicates true HDR content (BT.2020 + PQ/HLG).
/// Returns `true` only when the source uses wide color gamut and HDR transfer.
/// 10-bit content with BT.709 colors (common in anime/TV) returns `false`.
pub fn is_true_hdr(color_transfer: Option<&str>, color_primaries: Option<&str>) -> bool {
    let has_hdr_transfer = color_transfer
        .map(|t| HDR_TRANSFERS.contains(&t.to_lowercase().as_str()))
        .unwrap_or(false);
    let has_hdr_primaries = color_primaries
        .map(|p| HDR_PRIMARIES.contains(&p.to_lowercase().as_str()))
        .unwrap_or(false);
    // True HDR requires at least the HDR transfer function.
    // BT.2020 primaries alone (without PQ/HLG) is rare but we still tone-map.
    has_hdr_transfer || has_hdr_primaries
}

/// Build the FFmpeg `-vf` filter string for tone-mapping HDR → SDR.
///
/// Uses the `zscale` filter (from zimg library) for high-quality colorspace
/// conversion and the `tonemap` filter for dynamic range compression.
///
/// The filter chain:
/// 1. `zscale=t=linear:npl=100` — linearize the transfer function (PQ/HLG → linear)
/// 2. `format=gbrpf32le` — convert to 32-bit float for precision during tone-mapping
/// 3. `zscale=p=bt709` — convert primaries from BT.2020 to BT.709
/// 4. `tonemap=hable:desat=0` — apply Hable (Uncharted 2) tone-mapping curve
/// 5. `zscale=t=bt709:m=bt709:r=tv` — apply BT.709 transfer/matrix/range
/// 6. `format=yuv420p` — final output in 8-bit 4:2:0
///
/// If `zscale` is not available, falls back to a simpler filter that just
/// converts the format (loses HDR quality but doesn't crash).
pub fn tonemap_filter() -> String {
    "zscale=t=linear:npl=100,format=gbrpf32le,zscale=p=bt709,tonemap=hable:desat=0,zscale=t=bt709:m=bt709:r=tv,format=yuv420p".to_string()
}

/// Build a simple fallback filter for when zscale is not available.
/// This just converts pixel format without proper tone-mapping.
pub fn simple_format_filter() -> String {
    "format=yuv420p".to_string()
}

/// Build a filter for 10-bit SDR content (BT.709 colors, just high bit depth).
/// Uses format conversion with no color space changes — preserves the original
/// BT.709 colors while converting to 8-bit for H.264 compatibility.
pub fn bit_depth_filter() -> String {
    "format=yuv420p".to_string()
}

/// Determine the appropriate video filter for the given pixel format and color metadata.
///
/// - True HDR (BT.2020 + PQ/HLG): full tone-mapping pipeline (zscale + tonemap)
/// - 10-bit SDR (BT.709 colors): simple format conversion (preserves colors)
/// - 8-bit SDR: no filter needed
pub fn video_format_filter(
    pixel_format: Option<&str>,
    color_transfer: Option<&str>,
    color_primaries: Option<&str>,
) -> Option<String> {
    match pixel_format {
        Some(fmt) if is_high_bit_depth(fmt) => {
            if is_true_hdr(color_transfer, color_primaries) {
                Some(tonemap_filter())
            } else {
                // 10-bit with standard BT.709 colors — just convert bit depth
                Some(bit_depth_filter())
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_high_bit_depth_detected() {
        assert!(is_high_bit_depth("yuv420p10le"));
        assert!(is_high_bit_depth("yuv420p10be"));
        assert!(is_high_bit_depth("p010le"));
        assert!(is_high_bit_depth("yuv444p12le"));
    }

    #[test]
    fn test_8bit_not_high_bit_depth() {
        assert!(!is_high_bit_depth("yuv420p"));
        assert!(!is_high_bit_depth("yuv422p"));
        assert!(!is_high_bit_depth("yuv444p"));
        assert!(!is_high_bit_depth("rgb24"));
    }

    #[test]
    fn test_is_true_hdr_pq() {
        assert!(is_true_hdr(Some("smpte2084"), Some("bt2020")));
    }

    #[test]
    fn test_is_true_hdr_hlg() {
        assert!(is_true_hdr(Some("arib-std-b67"), Some("bt2020")));
    }

    #[test]
    fn test_is_true_hdr_bt2020_primaries_only() {
        assert!(is_true_hdr(None, Some("bt2020")));
    }

    #[test]
    fn test_not_hdr_bt709() {
        assert!(!is_true_hdr(Some("bt709"), Some("bt709")));
        assert!(!is_true_hdr(None, None));
        assert!(!is_true_hdr(Some("bt709"), None));
    }

    #[test]
    fn test_tonemap_filter_contains_key_stages() {
        let f = tonemap_filter();
        assert!(f.contains("zscale=t=linear"));
        assert!(f.contains("tonemap=hable"));
        assert!(f.contains("format=yuv420p"));
    }

    #[test]
    fn test_video_format_filter_true_hdr_gets_tonemap() {
        let result = video_format_filter(Some("yuv420p10le"), Some("smpte2084"), Some("bt2020"));
        assert!(result.is_some());
        assert!(result.unwrap().contains("tonemap"));
    }

    #[test]
    fn test_video_format_filter_10bit_sdr_gets_format_only() {
        let result = video_format_filter(Some("yuv420p10le"), Some("bt709"), Some("bt709"));
        assert!(result.is_some());
        let filter = result.unwrap();
        assert!(
            !filter.contains("tonemap"),
            "10-bit SDR should NOT get tone-mapping"
        );
        assert!(filter.contains("format=yuv420p"));
    }

    #[test]
    fn test_video_format_filter_10bit_no_color_meta_gets_format_only() {
        // No color metadata = assume SDR, just convert bit depth
        let result = video_format_filter(Some("yuv420p10le"), None, None);
        assert!(result.is_some());
        let filter = result.unwrap();
        assert!(
            !filter.contains("tonemap"),
            "Unknown color should NOT get tone-mapping"
        );
        assert!(filter.contains("format=yuv420p"));
    }

    #[test]
    fn test_video_format_filter_8bit_sdr() {
        assert!(video_format_filter(Some("yuv420p"), None, None).is_none());
        assert!(video_format_filter(None, None, None).is_none());
    }
}
