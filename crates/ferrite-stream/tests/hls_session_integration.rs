use ferrite_stream::hls::HlsSessionManager;
use ferrite_transcode::hwaccel::EncoderProfile;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

struct TestEnv {
    root: PathBuf,
    ffmpeg_path: String,
}

#[derive(Debug)]
struct MasterVariant {
    bandwidth_bps: u64,
    uri: String,
}

fn parse_master_variants(master: &str) -> Vec<MasterVariant> {
    let mut variants = Vec::new();
    let mut lines = master
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .peekable();

    while let Some(line) = lines.next() {
        if !line.starts_with("#EXT-X-STREAM-INF:") {
            continue;
        }

        let attrs = line.trim_start_matches("#EXT-X-STREAM-INF:");
        let bandwidth_bps = attrs
            .split(',')
            .find_map(|attr| attr.strip_prefix("BANDWIDTH="))
            .and_then(|v| v.parse::<u64>().ok());

        let Some(uri_line) = lines.next() else {
            continue;
        };
        if uri_line.starts_with('#') {
            continue;
        }

        if let Some(bandwidth_bps) = bandwidth_bps {
            variants.push(MasterVariant {
                bandwidth_bps,
                uri: uri_line.to_string(),
            });
        }
    }

    variants
}

fn pick_variant_for_bandwidth(
    variants: &[MasterVariant],
    throughput_bps: u64,
) -> Option<&MasterVariant> {
    variants
        .iter()
        .filter(|v| v.bandwidth_bps <= throughput_bps)
        .max_by_key(|v| v.bandwidth_bps)
        .or_else(|| variants.iter().min_by_key(|v| v.bandwidth_bps))
}

fn session_id_from_variant_uri(uri: &str) -> Option<String> {
    uri.split("/hls/")
        .nth(1)
        .and_then(|rest| rest.split('/').next())
        .map(str::to_string)
}

fn first_media_line(playlist: &str) -> Option<&str> {
    playlist
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty() && !line.starts_with('#'))
}

impl TestEnv {
    fn new(name: &str) -> Self {
        let root = std::env::temp_dir().join(format!(
            "ferrite-hls-integration-{}-{}",
            name,
            uuid::Uuid::new_v4()
        ));
        std::fs::create_dir_all(&root).expect("create temp root");

        let ffmpeg_path = create_fake_ffmpeg(&root);

        Self { root, ffmpeg_path }
    }

    fn manager(&self) -> Arc<HlsSessionManager> {
        let cache_dir = self.root.join("cache");
        std::fs::create_dir_all(&cache_dir).expect("create cache dir");
        Arc::new(HlsSessionManager::new(
            cache_dir,
            self.ffmpeg_path.clone(),
            2,
            30,
            30,
            30,
            EncoderProfile::software(),
        ))
    }

    fn media_file(&self, name: &str) -> PathBuf {
        let file = self.root.join(name);
        std::fs::write(&file, b"dummy-media").expect("create media file");
        file
    }
}

impl Drop for TestEnv {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

fn create_fake_ffmpeg(root: &Path) -> String {
    #[cfg(windows)]
    {
        let script = root.join("fake-ffmpeg.cmd");
        let content = "@echo off\r\n\
setlocal\r\n\
echo #EXTM3U>playlist.m3u8\r\n\
echo #EXT-X-VERSION:7>>playlist.m3u8\r\n\
echo #EXT-X-TARGETDURATION:2>>playlist.m3u8\r\n\
echo #EXT-X-MAP:URI=\"init.mp4\">>playlist.m3u8\r\n\
echo #EXTINF:2.000,>>playlist.m3u8\r\n\
echo seg_000.m4s>>playlist.m3u8\r\n\
type nul > init.mp4\r\n\
type nul > seg_000.m4s\r\n\
exit /b 0\r\n";
        std::fs::write(&script, content).expect("write fake ffmpeg cmd");
        script.to_string_lossy().to_string()
    }

    #[cfg(not(windows))]
    {
        use std::os::unix::fs::PermissionsExt;

        let script = root.join("fake-ffmpeg.sh");
        let content = "#!/bin/sh\n\
set -e\n\
cat > playlist.m3u8 <<'EOF'\n\
#EXTM3U\n\
#EXT-X-VERSION:7\n\
#EXT-X-TARGETDURATION:2\n\
#EXT-X-MAP:URI=\"init.mp4\"\n\
#EXTINF:2.000,\n\
seg_000.m4s\n\
EOF\n\
: > init.mp4\n\
: > seg_000.m4s\n\
exit 0\n";
        std::fs::write(&script, content).expect("write fake ffmpeg sh");

        let mut perms = std::fs::metadata(&script)
            .expect("metadata fake ffmpeg")
            .permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&script, perms).expect("chmod fake ffmpeg");

        script.to_string_lossy().to_string()
    }
}

#[tokio::test]
async fn concurrent_media_sessions_are_isolated() {
    let env = TestEnv::new("isolation");
    let manager = env.manager();

    let media_a = env.media_file("a.mkv");
    let media_b = env.media_file("b.mkv");

    manager
        .create_single_variant_session(
            "media-a",
            &media_a,
            Some(1200.0),
            Some(1920),
            Some(1080),
            Some(5000),
            0.0,
            0.0,
            None,
            None,
            None,
            None,
            Some("aac"),
            Some("h264"),
            None,
            None,
            false,
        )
        .await
        .expect("create session A");

    manager
        .create_single_variant_session(
            "media-b",
            &media_b,
            Some(800.0),
            Some(1920),
            Some(1080),
            Some(4500),
            0.0,
            0.0,
            None,
            None,
            None,
            None,
            Some("aac"),
            Some("h264"),
            None,
            None,
            false,
        )
        .await
        .expect("create session B");

    assert!(manager.get_session_for_media("media-a").is_some());
    assert!(manager.get_session_for_media("media-b").is_some());

    manager.destroy_media_sessions("media-a").await;

    assert!(manager.get_session_for_media("media-a").is_none());
    assert!(manager.get_session_for_media("media-b").is_some());

    manager.destroy_all_sessions().await;
}

#[tokio::test]
async fn same_media_different_owner_keys_do_not_interfere() {
    let env = TestEnv::new("same-media-owners");
    let manager = env.manager();

    let media = env.media_file("shared.mkv");
    let owner_a = HlsSessionManager::owner_key("media-shared", Some("playback-a"));
    let owner_b = HlsSessionManager::owner_key("media-shared", Some("playback-b"));

    let session_a = manager
        .create_single_variant_session_owned(
            &owner_a,
            "media-shared",
            &media,
            Some(2400.0),
            Some(1920),
            Some(1080),
            Some(5000),
            0.0,
            0.0,
            None,
            None,
            None,
            None,
            Some("aac"),
            Some("h264"),
            None,
            None,
            false,
        )
        .await
        .expect("create owner A session");

    let session_b = manager
        .create_single_variant_session_owned(
            &owner_b,
            "media-shared",
            &media,
            Some(2400.0),
            Some(1920),
            Some(1080),
            Some(5000),
            120.0,
            120.0,
            None,
            None,
            None,
            None,
            Some("aac"),
            Some("h264"),
            None,
            None,
            false,
        )
        .await
        .expect("create owner B session");

    let sid_a = session_a[0].session_id.clone();
    let sid_b = session_b[0].session_id.clone();
    assert_ne!(sid_a, sid_b);

    let selected_a = manager
        .get_session_for_owner(&owner_a)
        .expect("owner A session exists");
    let selected_b = manager
        .get_session_for_owner(&owner_b)
        .expect("owner B session exists");
    assert_eq!(selected_a.session_id, sid_a);
    assert_eq!(selected_b.session_id, sid_b);

    manager.destroy_owner_sessions(&owner_a).await;

    assert!(manager.get_session(&sid_a).is_none());
    assert!(manager.get_session(&sid_b).is_some());

    manager.destroy_all_sessions().await;
}

#[tokio::test]
async fn startup_owner_key_creates_multiple_abr_variants() {
    let env = TestEnv::new("abr-owner-key");
    let manager = env.manager();

    let media = env.media_file("abr.mkv");
    let owner = HlsSessionManager::owner_key("media-abr", Some("playback-abr"));

    let sessions = manager
        .create_variant_sessions_owned(
            &owner,
            "media-abr",
            &media,
            Some(2400.0),
            Some(1920),
            Some(1080),
            Some(5000),
            0.0,
            0.0,
            None,
            None,
            None,
            None,
            Some("aac"),
            Some("h264"),
            None,
            None,
        )
        .await
        .expect("create owner-keyed ABR sessions");

    assert!(
        sessions.len() > 1,
        "expected ABR startup to produce multiple quality variants"
    );

    let mapped = manager.get_variant_sessions_owned(&owner);
    assert_eq!(mapped.len(), sessions.len());

    manager.destroy_owner_sessions(&owner).await;

    for s in sessions {
        assert!(manager.get_session(&s.session_id).is_none());
    }
}

#[tokio::test]
async fn abr_master_playlist_supports_variant_downgrade_under_throttle() {
    let env = TestEnv::new("abr-throttle");
    let manager = env.manager();

    let media = env.media_file("abr-throttle.mkv");
    let owner = HlsSessionManager::owner_key("media-abr-throttle", Some("playback-abr-throttle"));

    let sessions = manager
        .create_variant_sessions_owned(
            &owner,
            "media-abr-throttle",
            &media,
            Some(3600.0),
            Some(1920),
            Some(1080),
            Some(8000),
            0.0,
            0.0,
            None,
            None,
            None,
            None,
            Some("aac"),
            Some("h264"),
            None,
            None,
        )
        .await
        .expect("create ABR sessions");

    assert!(sessions.len() > 1, "expected multiple ABR variants");

    let master = manager.generate_master_playlist(&sessions, "media-abr-throttle", None);
    let variants = parse_master_variants(&master);
    assert_eq!(variants.len(), sessions.len());

    let max_bw = variants
        .iter()
        .map(|v| v.bandwidth_bps)
        .max()
        .expect("no variant bandwidth");
    let min_bw = variants
        .iter()
        .map(|v| v.bandwidth_bps)
        .min()
        .expect("no variant bandwidth");

    let high_pick = pick_variant_for_bandwidth(&variants, max_bw).expect("high bandwidth pick");
    let low_pick = pick_variant_for_bandwidth(&variants, min_bw).expect("low bandwidth pick");

    assert!(
        low_pick.bandwidth_bps <= high_pick.bandwidth_bps,
        "low-throughput selection should not exceed high-throughput selection"
    );
    if max_bw > min_bw {
        assert_ne!(
            high_pick.uri, low_pick.uri,
            "expected bandwidth drop to select a different variant"
        );
    }

    let by_session_id: HashMap<String, _> = sessions
        .iter()
        .map(|s| (s.session_id.clone(), Arc::clone(s)))
        .collect();

    for picked in [high_pick, low_pick] {
        let sid =
            session_id_from_variant_uri(&picked.uri).expect("extract session id from variant uri");
        let session = by_session_id
            .get(&sid)
            .expect("picked variant session should exist");
        let playlist = manager
            .get_variant_playlist(session, "media-abr-throttle", None)
            .await
            .expect("read picked variant playlist");
        assert!(
            first_media_line(&playlist).is_some(),
            "variant playlist should contain at least one media line"
        );
    }

    manager.destroy_owner_sessions(&owner).await;
}

#[tokio::test]
async fn get_or_create_session_reuses_nearby_start_and_recreates_far_start() {
    let env = TestEnv::new("seek-reuse-recreate");
    let manager = env.manager();

    let media = env.media_file("seek-reuse-recreate.mkv");

    let first = manager
        .get_or_create_session(
            "media-seek-reuse",
            &media,
            Some(3600.0),
            Some(1920),
            Some(1080),
            Some(6000),
            0.0,
            0.0,
            None,
            None,
            None,
            None,
            Some("aac"),
            Some("h264"),
            None,
            None,
        )
        .await
        .expect("create initial session");

    let reused = manager
        .get_or_create_session(
            "media-seek-reuse",
            &media,
            Some(3600.0),
            Some(1920),
            Some(1080),
            Some(6000),
            1.0,
            1.0,
            None,
            None,
            None,
            None,
            Some("aac"),
            Some("h264"),
            None,
            None,
        )
        .await
        .expect("reuse nearby session");

    assert_eq!(
        first.session_id, reused.session_id,
        "start offsets within one segment should reuse the same session"
    );

    let recreated = manager
        .get_or_create_session(
            "media-seek-reuse",
            &media,
            Some(3600.0),
            Some(1920),
            Some(1080),
            Some(6000),
            9.0,
            9.0,
            None,
            None,
            None,
            None,
            Some("aac"),
            Some("h264"),
            None,
            None,
        )
        .await
        .expect("recreate far session");

    assert_ne!(
        recreated.session_id, reused.session_id,
        "distant start should force new session creation"
    );
    assert!(
        manager.get_session(&reused.session_id).is_none(),
        "replaced session should be destroyed"
    );

    let mapped = manager
        .get_session_for_media("media-seek-reuse")
        .expect("active media mapping should point at recreated session");
    assert_eq!(mapped.session_id, recreated.session_id);

    manager.destroy_all_sessions().await;
}

#[tokio::test]
async fn seek_creates_new_session_and_replaces_old_mapping() {
    let env = TestEnv::new("seek-regression");
    let manager = env.manager();

    let media = env.media_file("seek.mkv");

    let first = manager
        .create_single_variant_session(
            "media-seek",
            &media,
            Some(3600.0),
            Some(1920),
            Some(1080),
            Some(6000),
            0.0,
            0.0,
            None,
            None,
            None,
            None,
            Some("aac"),
            Some("h264"),
            None,
            None,
            false,
        )
        .await
        .expect("create initial session");

    let old_session_id = first[0].session_id.clone();

    let second = manager
        .create_single_variant_session(
            "media-seek",
            &media,
            Some(3600.0),
            Some(1920),
            Some(1080),
            Some(6000),
            900.0,
            900.0,
            None,
            None,
            None,
            None,
            Some("aac"),
            Some("h264"),
            None,
            None,
            false,
        )
        .await
        .expect("create seek session");

    let new_session = &second[0];
    assert_ne!(old_session_id, new_session.session_id);
    assert!(manager.get_session(&old_session_id).is_none());

    let mapped = manager
        .get_session_for_media("media-seek")
        .expect("active session expected");
    assert_eq!(mapped.session_id, new_session.session_id);

    manager.destroy_all_sessions().await;
}
