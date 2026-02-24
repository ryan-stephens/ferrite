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
                // Hand off SIGKILL escalation to a background task so the
                // caller is not blocked waiting for the old process to die.
                tokio::spawn(async move {
                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                    let _ = child.kill().await;
                });
                return;
            }
        }
        // Windows: no SIGTERM, kill immediately.
        let _ = child.kill().await;
        debug!("Killed FFmpeg (immediate SIGKILL)");
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
- The log line inside `kill_ffmpeg` ("Killed FFmpeg for session … SIGTERM+SIGKILL") will need
  to move into the spawned task or be split into two log lines (SIGTERM sent / SIGKILL sent).

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

`segment_count` is **only updated when a segment is served to the client** (inside
`wait_for_segment` → `refresh_segment_count`). FFmpeg typically encodes 2–5 segments ahead of
what the player has fetched. So `buffered_secs()` consistently underestimates the true encoded
window by 4–10 seconds.

Example: FFmpeg has encoded 0–60s, player has fetched 0–20s. User seeks to 28s.
- `buffered_end` = 20s (cached count = 10 segments × 2s)
- Reuse check fails → old FFmpeg killed → new FFmpeg spawned from 28s
- Old FFmpeg already had 28s encoded and ready to serve

### Fix

Before evaluating reuse, read the playlist from disk to get the true encoded window. This
replaces what would be a 2s SIGTERM wait with a single `read_to_string` call (~1ms).

```rust
// crates/ferrite-api/src/handlers/stream.rs  —  hls_seek(), before the reuse check

if let Some(existing) = state.hls_sessions.get_session_for_owner(&owner_key) {
    // Read the playlist from disk to get the true encoded boundary, not just
    // the number of segments already served to this client.
    let true_buffered_end = {
        let playlist_path = existing.output_dir.join("playlist.m3u8");
        match tokio::fs::read_to_string(&playlist_path).await {
            Ok(content) => {
                let segment_count = content.lines()
                    .filter(|l| l.starts_with("#EXTINF:"))
                    .count() as f64;
                existing.start_secs + segment_count * existing.segment_duration as f64
            }
            Err(_) => existing.start_secs + existing.buffered_secs(), // fallback to cached
        }
    };

    if can_reuse_seek_session(
        requested_start,
        existing.start_secs,
        true_buffered_end,
        existing.is_ffmpeg_alive().await,
    ) {
        // ... existing reuse path ...
    }
}
```

Alternatively, expose a `true_buffered_secs()` method on `HlsSession` that reads from the
playlist directly, keeping the business logic in `hls.rs` rather than the handler.

### Files to change

- `crates/ferrite-stream/src/hls.rs` — add `true_buffered_secs()` method on `HlsSession`
- `crates/ferrite-api/src/handlers/stream.rs` — `hls_seek()`, use `true_buffered_secs()` in
  reuse check (~line 882)

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
immediately. The ABR promotion logic in `hls_master_playlist` (`should_promote_ladder` at
stream.rs:543) handles upgrading a single-variant session to the full ladder on the **second**
master playlist request.

### Fix

On the first `/master.m3u8` request, use `create_single_variant_session_owned` (same as seek).
Promotion to the full ABR ladder happens on the next master playlist poll (the player typically
re-fetches the master playlist every few seconds via HLS.js).

```rust
// crates/ferrite-api/src/handlers/stream.rs  —  hls_master_playlist()

// Replace the initial creation branch:
} else {
    let _permit = acquire_transcode_permit(&state, &id, "hls-master").await?;

    // Start with a single variant (fastest TTFF). The player will re-request
    // the master playlist after it starts buffering; the should_promote_ladder
    // logic will then spawn the full ABR ladder on that second request.
    let create_result = state
        .hls_sessions
        .create_single_variant_session_owned(   // ← was create_variant_sessions_owned
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
        )
        .await;

    create_result.map_err(|e| { ... })?
};
```

The `should_promote_ladder` check at stream.rs:543-545 already handles promoting a
single-variant session to the full ABR ladder on the next master playlist request:

```rust
let should_promote_ladder = reused && existing_variants.len() == 1 && !seek_created_single;
```

The `seek_created_single` flag prevents promotion immediately after a seek — that guard also
covers the initial single-variant session created by this change.

### Additional consideration: promotion timing

Currently the promote path is triggered when `existing_variants.len() == 1 && !seek_created_single`.
After this change, an initial play will also be `len() == 1`. The `seek_created_single` check
(stream.rs:543-544) uses `(existing_variants[0].start_secs - requested_start).abs() < 1.0` to
distinguish a seek from a re-poll. A re-poll of the master playlist during initial playback will
have `requested_start == 0` and `session.start_secs == 0`, so `seek_created_single` will be
`true` and promotion will be blocked.

The simplest fix: track whether a session was created as an "initial play" vs a "seek" (a single
boolean field on `HlsSession`, or a separate `DashMap` key in `HlsSessionManager`), and use that
to gate promotion instead of the position heuristic. This makes the intent explicit rather than
relying on position coincidence.

### Files to change

- `crates/ferrite-api/src/handlers/stream.rs` — `hls_master_playlist()`, initial creation branch
  (~line 584)
- `crates/ferrite-stream/src/hls.rs` — optionally add `is_seek_session: bool` to `HlsSession`
  and update `should_promote_ladder` logic
- `crates/ferrite-api/src/handlers/stream.rs` — update `seek_created_single` check to use the
  new flag (~line 543)

---

## Implementation Order

Do these in order — each builds on the previous and can be shipped independently:

| Step | Change | Seek impact | TTFF impact | Risk |
|------|--------|-------------|-------------|------|
| 1 | SIGTERM fire-and-forget | −2s per cold seek | low | Low |
| 2 | Adaptive segment polling | −0–250ms per segment | −0–250ms | Low |
| 3 | Accurate buffer check | fewer FFmpeg restarts | none | Low |
| 4 | Single-variant initial play | none | −N×encode overhead | Medium |

Steps 1–3 are localized changes with no behavior change under normal conditions — they only
remove artificial wait time. Step 4 changes the ABR ladder startup flow and requires verifying
that the promotion heuristic works correctly after the change.

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
