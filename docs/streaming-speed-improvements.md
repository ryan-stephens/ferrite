# Streaming Speed Improvements

**Goal:** Reduce time-to-first-frame (TTFF) on initial playback and reduce seek latency to be
as close to instantaneous as possible.

This document covers four concrete improvements ranked by impact, with exact file locations and
implementation notes for each.

---

## 1. SIGTERM Fire-and-Forget on Session Destroy

**Impact: HIGH — removes a mandatory 2-second stall from every cold seek**

### Problem

`HlsSession::kill_ffmpeg()` in `crates/ferrite-stream/src/hls.rs:160-180` sends SIGTERM then
**synchronously awaits a 2-second sleep** before sending SIGKILL:

```rust
unsafe { libc::kill(pid as libc::pid_t, libc::SIGTERM) };
tokio::time::sleep(std::time::Duration::from_secs(2)).await;  // ← blocks the caller
let _ = child.kill().await;
```

`kill_ffmpeg` is called inside `destroy_session` → `destroy_owner_sessions`, which is called at
the top of `create_single_variant_session_owned` before spawning the replacement FFmpeg process.
The full seek critical path therefore looks like:

```
hls_seek (stream.rs:858)
  └── create_single_variant_session_owned (hls.rs:859)
        ├── destroy_owner_sessions (hls.rs:892)   ← waits 2s here
        │     └── kill_ffmpeg
        │           ├── SIGTERM
        │           ├── sleep(2s)  ← mandatory stall
        │           └── SIGKILL
        └── spawn new FFmpeg + wait_for_first_segment  ← can't start until sleep finishes
```

Every seek that leaves the already-buffered window adds at least **2 seconds** before the new
FFmpeg process can even start.

### Fix

Send SIGTERM, then immediately move the `Child` into a detached background task that handles
the 2-second escalation. The caller returns instantly.

```rust
// crates/ferrite-stream/src/hls.rs  —  kill_ffmpeg()
pub async fn kill_ffmpeg(&self) {
    if let Some(mut child) = self.ffmpeg_handle.lock().await.take() {
        #[cfg(unix)]
        {
            if let Some(pid) = child.id() {
                // SAFETY: pid is a valid process ID from a child we own.
                unsafe { libc::kill(pid as libc::pid_t, libc::SIGTERM) };
                debug!("Sent SIGTERM to FFmpeg for session {}", self.session_id);
                // Hand off SIGKILL escalation to a background task so the
                // caller is not blocked waiting for the old process to die.
                let session_id = self.session_id.clone();
                tokio::spawn(async move {
                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                    let _ = child.kill().await;
                    debug!("Sent SIGKILL to FFmpeg for session {}", session_id);
                });
                return;
            }
        }
        // Windows / no pid: kill immediately.
        let _ = child.kill().await;
        debug!("Killed FFmpeg for session {} (immediate)", self.session_id);
    }
}
```

The SIGTERM is still sent — FFmpeg still gets its 2-second window to flush write buffers and
close the playlist cleanly. The only change is that **Ferrite no longer waits for that window**
before starting the next encode.

### Files to change

- `crates/ferrite-stream/src/hls.rs` — `kill_ffmpeg()` (~line 160)

### Caveats

- The spawned task holds a `tokio::process::Child`, which is `Send`. No reference counting
  needed — ownership moves into the task.
- If the server shuts down during the 2-second window, the orphaned child task will be
  cancelled by the Tokio runtime. The OS will reap the FFmpeg process anyway.

---

## 2. Adaptive Segment Polling Intervals

**Impact: MEDIUM — cuts per-segment wait from up to 250ms to up to 50ms for fast segments**

### Problem

`wait_for_segment()` in `hls.rs:1369-1415` polls `playlist.m3u8` at a fixed **250ms interval**
for up to 30 seconds. A segment that FFmpeg finishes writing 5ms after the request arrives still
makes the client wait up to 245ms before it is served.

With 2-second segments, the browser typically requests the next segment about 1 second before
the current one finishes playing. FFmpeg, encoding in real-time, usually has the next segment
ready within 100-300ms of that request. The 250ms poll interval adds between 0 and 250ms of
random extra latency on top of that — a significant fraction of the inter-segment timing budget.

`wait_for_first_segment()` at `hls.rs:502-532` already uses an adaptive schedule
(50ms × 20 → 100ms × 40 → 250ms × 40). `wait_for_segment` should adopt the same approach.

### Fix

Replace the fixed 250ms loop with an adaptive schedule that front-loads tight polling:

```rust
// crates/ferrite-stream/src/hls.rs  —  wait_for_segment(), replace the polling loop

// Adaptive polling: fast at first (most segments arrive within 500ms),
// fall back to slower polling for segments that take longer (e.g. HDR transcode).
let poll_schedule: &[(u64, u32)] = &[
    (50,  10),   //  0 –  500ms: poll every 50ms
    (100, 10),   //  0.5 – 1.5s: poll every 100ms
    (250, 116),  //  1.5 – 30s:  poll every 250ms
];

'outer: for &(interval_ms, count) in poll_schedule {
    for _ in 0..count {
        // ... existing mtime-check + playlist-read logic ...
        tokio::time::sleep(std::time::Duration::from_millis(interval_ms)).await;
    }
}
```

The total timeout remains 30 seconds. The fast early phase (10 × 50ms = 500ms) adds negligible
CPU overhead — `metadata()` + conditional `read_to_string()` is a few microseconds per call.

### Files to change

- `crates/ferrite-stream/src/hls.rs` — `wait_for_segment()` polling loop (~line 1369)

---

## 3. Accurate Buffer Check Before Seek Reuse

**Impact: MEDIUM — prevents unnecessary FFmpeg restarts for seeks just beyond the served window**

### Problem

`hls_seek` at `stream.rs:881-910` checks `can_reuse_seek_session` to decide whether to reuse
the existing session or destroy it and spawn a new FFmpeg process. The reuse condition is:

```rust
fn can_reuse_seek_session(requested_start, session_start, buffered_end, ffmpeg_alive) -> bool {
    ffmpeg_alive && requested_start >= session_start && requested_start < buffered_end
}
```

`buffered_end` is computed as `session.start_secs + session.buffered_secs()`, where
`buffered_secs()` at `hls.rs:134-139` reads from the cached `segment_count` atomic:

```rust
pub fn buffered_secs(&self) -> f64 {
    let count = self.segment_count.load(Ordering::Acquire);
    count as f64 * self.segment_duration as f64
}
```

This has **two bugs**:

**Bug A: Underestimated upper bound.** `segment_count` is only updated when a segment is
**served to the client** (inside `wait_for_segment` → `refresh_segment_count`). FFmpeg typically
encodes 2–5 segments ahead of what the player has fetched. So `buffered_secs()` consistently
underestimates the true encoded window by 4–10 seconds.

Example: FFmpeg has encoded 0–60s, player has fetched 0–20s. User seeks to 28s.
- `buffered_end` = 20s (cached count = 10 segments × 2s)
- Reuse check fails → old FFmpeg killed → new FFmpeg spawned from 28s
- Old FFmpeg already had 28s encoded and ready to serve

**Bug B: Missing lower bound.** The FFmpeg args include `delete_segments` with
`hls_list_size=30` (`hls.rs:1222-1231`). This creates a **sliding window** — after FFmpeg has
written more than 30 segments, older segment files are deleted from disk and removed from the
playlist. The current reuse check uses `session_start` as the lower bound, but after 60+
seconds of encoding those early segments no longer exist.

Example: Session started at t=100s. FFmpeg has encoded t=100s to t=220s (60 segments).
Playlist shows segments 31–60 (t=160s–220s). Segments 1–30 deleted from disk.
- Current check: `requested >= 100s` → true for a seek to 120s
- But seg_010 (t=120s) was deleted — player gets a 404

### Fix

Add a `playlist_segment_range()` method on `HlsSession` that parses the playlist from disk to
determine the **actual available window** (both lower and upper bounds). This replaces what
would be a 2s SIGTERM wait + FFmpeg restart with a single `read_to_string` call (~1ms).

#### New method on `HlsSession` in `hls.rs`:

```rust
/// Parse the playlist from disk to determine the actual available segment range.
/// Returns `(available_start_secs, available_end_secs)` relative to the media timeline.
/// Uses segment filenames (seg_NNN.m4s) rather than EXTINF count because the playlist
/// is a sliding window (hls_list_size + delete_segments) — the count is capped but
/// segment numbers keep incrementing.
pub async fn playlist_available_range(&self) -> Option<(f64, f64)> {
    let playlist_path = self.output_dir.join("playlist.m3u8");
    let content = tokio::fs::read_to_string(&playlist_path).await.ok()?;

    let mut min_seg: Option<u64> = None;
    let mut max_seg: Option<u64> = None;

    for line in content.lines() {
        let line = line.trim();
        if let Some(num_str) = line.strip_prefix("seg_").and_then(|s| s.strip_suffix(".m4s")) {
            if let Ok(n) = num_str.parse::<u64>() {
                min_seg = Some(min_seg.map_or(n, |m: u64| m.min(n)));
                max_seg = Some(max_seg.map_or(n, |m: u64| m.max(n)));
            }
        }
    }

    let seg_dur = self.segment_duration as f64;
    Some((
        self.start_secs + min_seg? as f64 * seg_dur,
        self.start_secs + (max_seg? + 1) as f64 * seg_dur,
    ))
}
```

#### Updated reuse check in `stream.rs` (`hls_seek`):

```rust
if let Some(existing) = state.hls_sessions.get_session_for_owner(&owner_key) {
    let ffmpeg_alive = existing.is_ffmpeg_alive().await;
    // Read the playlist to get the true available window on disk, accounting
    // for both the encode-ahead (upper bound) and segment deletion (lower bound).
    let reuse = if let Some((available_start, available_end)) =
        existing.playlist_available_range().await
    {
        can_reuse_seek_session(requested_start, available_start, available_end, ffmpeg_alive)
    } else {
        // Playlist unreadable — fall back to cached estimate
        let buffered_end = existing.start_secs + existing.buffered_secs();
        can_reuse_seek_session(requested_start, existing.start_secs, buffered_end, ffmpeg_alive)
    };

    if reuse {
        existing.touch();
        // ... existing reuse response ...
    }
}
```

#### Updated `can_reuse_seek_session`:

No signature change needed — the first positional argument (`session_start`) now receives
`available_start` (accounting for deleted segments) instead of the session's original
`start_secs`.

### Files to change

- `crates/ferrite-stream/src/hls.rs` — add `playlist_available_range()` method on `HlsSession`
- `crates/ferrite-api/src/handlers/stream.rs` — `hls_seek()`, use `playlist_available_range()`
  in reuse check (~line 882)

---

## 4. Single-Variant First on Initial Playback

**Impact: MEDIUM — reduces TTFF for transcode paths by eliminating N-1 unnecessary encodes**

### Problem

`hls_master_playlist` at `stream.rs:584-613` always calls `create_variant_sessions_owned()`,
which spawns **one FFmpeg process per quality tier** (up to 5) before returning the playlist.
Phase 2 of `create_variant_sessions_owned` (hls.rs:799-803) waits for all first segments in
parallel:

```rust
let wait_futures: Vec<_> = sessions
    .iter()
    .map(|s| Self::wait_for_first_segment(s))
    .collect();
futures::future::join_all(wait_futures).await;
```

The response is gated on `max(all variant first-segment times)`. For HEVC → H.264 software
transcode at 5 different resolutions, this can mean waiting for 5 simultaneous encodes to all
produce their first segment before the player sees anything.

The `hls_seek` endpoint already does this correctly — it calls
`create_single_variant_session_owned` (1 FFmpeg process at the highest quality) and returns
immediately.

### Why the existing promotion heuristic breaks

The ABR promotion logic in `hls_master_playlist` at `stream.rs:543-545`:

```rust
let seek_created_single = existing_variants.len() == 1
    && (existing_variants[0].start_secs - requested_start).abs() < 1.0;
let should_promote_ladder = reused && existing_variants.len() == 1 && !seek_created_single;
```

This uses a **position heuristic** — it assumes that if the existing session's start position
matches the requested start, the session was created by a seek (and should NOT be promoted).
After this change, initial play also creates a single variant. On the next master playlist poll:
- `existing_variants.len() == 1` → true
- `existing_variants[0].start_secs ≈ 0`, `requested_start = 0` → diff < 1.0
- `seek_created_single = true` → `should_promote_ladder = false`

**Promotion never happens.** The player is stuck on a single quality forever.

### Fix

Replace the position heuristic with an explicit flag. Add `awaiting_promotion: AtomicBool` to
`HlsSession` — set to `true` when created by the initial play path, `false` when created by
the seek path. The promotion check reads this flag directly.

#### Step A: Add field to `HlsSession` in `hls.rs`:

```rust
// crates/ferrite-stream/src/hls.rs  —  HlsSession struct
pub struct HlsSession {
    // ... existing fields ...
    /// True when this session was created as a single-variant for fast initial TTFF
    /// and should be promoted to the full ABR ladder on the next master playlist poll.
    /// Set to false for seek-created sessions (which should stay single-variant).
    pub awaiting_promotion: std::sync::atomic::AtomicBool,
}
```

Initialize to `false` in both `create_session` and `create_session_no_wait`.

#### Step B: Add parameter to `create_single_variant_session_owned` in `hls.rs`:

Add an `awaiting_promotion: bool` parameter (or create a separate
`create_initial_play_session_owned` wrapper that sets it to `true`). When constructing the
`HlsSession`, pass this value:

```rust
awaiting_promotion: std::sync::atomic::AtomicBool::new(awaiting_promotion),
```

The seek path (`hls_seek` in `stream.rs`) calls with `awaiting_promotion: false`.
The initial play path (`hls_master_playlist` in `stream.rs`) calls with `awaiting_promotion: true`.

#### Step C: Replace the promotion check in `stream.rs`:

```rust
// crates/ferrite-api/src/handlers/stream.rs  —  hls_master_playlist()

// OLD:
let seek_created_single = existing_variants.len() == 1
    && (existing_variants[0].start_secs - requested_start).abs() < 1.0;
let should_promote_ladder = reused && existing_variants.len() == 1 && !seek_created_single;

// NEW:
let should_promote_ladder = reused
    && existing_variants.len() == 1
    && existing_variants[0]
        .awaiting_promotion
        .load(std::sync::atomic::Ordering::Acquire);
```

When promotion fires (the session gets replaced by full ABR variants), the old session is
destroyed. The new sessions have `awaiting_promotion: false`, so promotion only fires once.

#### Step D: Change the initial creation branch in `hls_master_playlist`:

```rust
// crates/ferrite-api/src/handlers/stream.rs  —  hls_master_playlist(), else branch (~line 584)

} else {
    let _permit = acquire_transcode_permit(&state, &id, "hls-master").await?;

    // Start with a single variant at the highest quality for fastest TTFF.
    // The player re-polls master.m3u8 within a few seconds; the
    // should_promote_ladder check will then spawn the full ABR ladder.
    let create_result = state
        .hls_sessions
        .create_single_variant_session_owned(
            &owner_key,
            &id,
            file_path,
            duration_secs,
            item.width.map(|w| w as u32),
            item.height.map(|h| h as u32),
            item.bitrate_kbps.map(|b| b as u32),
            start_secs,
            requested_start,
            sub_path.as_deref(),
            pixel_format.as_deref(),
            query.audio_stream,
            frame_rate.as_deref(),
            item.audio_codec.as_deref(),
            item.video_codec.as_deref(),
            color_transfer.as_deref(),
            color_primaries.as_deref(),
            true,  // awaiting_promotion = true for initial play
        )
        .await;

    create_result.map_err(|e| {
        warn!("Failed to create HLS session for {}: {}", id, e);
        ApiError::internal(e.to_string())
    })?
};
```

### Files to change

- `crates/ferrite-stream/src/hls.rs`:
  - Add `awaiting_promotion: AtomicBool` field to `HlsSession` (~line 16)
  - Initialize to `false` in `create_session` and `create_session_no_wait`
  - Add `awaiting_promotion: bool` parameter to `create_single_variant_session_owned` (and
    its `_owned` variant), pass through to session construction
  - Update `make_test_session` in tests to include the new field
- `crates/ferrite-api/src/handlers/stream.rs`:
  - Initial creation branch: call `create_single_variant_session_owned` with
    `awaiting_promotion: true` (~line 584)
  - `hls_seek`: call with `awaiting_promotion: false` (~line 939)
  - Replace `seek_created_single` heuristic with `awaiting_promotion.load()` check (~line 543)

---

## Implementation Order

Do these in order — each builds on the previous and can be shipped independently:

| Step | Change | Seek impact | TTFF impact | Risk |
|------|--------|-------------|-------------|------|
| 1 | SIGTERM fire-and-forget | −2s per cold seek | none | Low |
| 2 | Adaptive segment polling | −0–250ms per segment | −0–250ms | Low |
| 3 | Accurate buffer check | fewer FFmpeg restarts | none | Low–Medium |
| 4 | Single-variant initial play | none | −N×encode overhead | Medium |

Steps 1–2 are localized changes with no behavior change under normal conditions — they only
remove artificial wait time. Step 3 adds a playlist read on the seek hot path (trivial I/O cost)
and must correctly handle the sliding-window semantics of `delete_segments`. Step 4 changes the
ABR ladder startup flow and adds a new field to `HlsSession`; verify that promotion fires
exactly once on the second master playlist poll and never fires after seeks.

---

## Not Included / Out of Scope

- **inotify for segment detection**: Would replace filesystem polling with kernel event
  notification, giving sub-millisecond segment latency. Linux-only (requires a conditional
  compilation path for macOS/Windows), adds a new dependency (`inotify` crate), and the
  adaptive polling in improvement #2 closes most of the gap at much lower complexity cost.

- **Remux/AudioTranscode seek path**: The `stream_media` handler at `stream.rs:232` uses
  direct `transcode::serve_remux` / `serve_audio_transcode` with a pipe-based response. These
  paths don't use the HLS session manager and seek by re-spawning FFmpeg with `-ss`. No
  significant latency improvements are available there without architectural changes.

- **HW decoder for hardware encoder paths**: `spawn_ffmpeg` adds HW input args only when
  `needs_scaling && !needs_software` (hls.rs:1113-1115). HW decoding could also be enabled
  for same-resolution HW encodes, potentially reducing decode overhead. This is a minor
  encoder-specific tuning and is separate from user-facing latency.
