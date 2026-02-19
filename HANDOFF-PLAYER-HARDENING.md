# Ferrite Player Hardening — Agent Handoff Document

> **Date:** Feb 18, 2026
> **Goal:** Ensure an extremely performant video/audio playback and UX — all edge cases thought through, bulletproof player experience.
> **Project:** `C:\Users\ryans\source\repos\ferrite\`
> **Key files:** `ferrite-ui/src/components/Player.tsx` (~1100 lines), `ferrite-ui/src/api.ts`, `crates/ferrite-stream/src/hls.rs`, `crates/ferrite-transcode/src/`

---

## Current State Summary

Phase 2A of the roadmap is **fully complete**. All major player features are implemented and working:

- HLS adaptive streaming with HLS.js 1.5
- Four-tier playback: DirectPlay → Remux → AudioTranscode → FullTranscode
- Quality selector (Auto ABR + manual level pinning)
- Subtitle track picker (custom VTT renderer, no native `<track>`)
- Audio track switching (triggers HLS session recreation)
- Chapter markers on timeline with hover tooltips
- Up Next auto-play with 15s countdown overlay
- Playback preference persistence (sessionStorage per-library + server-side language defaults)
- Seek generation counter for stale callback detection
- Session expired recovery (404 detection → auto-restart at current position)
- Performance monitoring instrumentation (span-based timing, PerfOverlay)
- Keyboard shortcuts (play/pause, seek ±10s, volume, speed, fullscreen, PiP)
- Progress reporting every 10s with `keepalive: true`

---

## Architecture Quick Reference

### Streaming Flow
```
Browser → GET /api/stream/{id}/hls/master.m3u8 → Backend spawns FFmpeg
       → HLS.js fetches variant playlist + fMP4 segments
       → Seek: POST /api/stream/{id}/hls/seek → Backend kills old FFmpeg, spawns new
       → Frontend destroys old HLS.js, resets video element, creates new instance
```

### Key Backend Components
| File | Purpose |
|------|---------|
| `crates/ferrite-stream/src/hls.rs` | HLS session manager (DashMap, FFmpeg lifecycle, segment serving) |
| `crates/ferrite-stream/src/compat.rs` | Codec compatibility → streaming strategy selection |
| `crates/ferrite-stream/src/direct.rs` | Byte-range serving for DirectPlay |
| `crates/ferrite-stream/src/transcode.rs` | Progressive transcode pipes (remux, audio, full) |
| `crates/ferrite-transcode/src/hwaccel.rs` | NVENC/QSV/VAAPI auto-detection + profiles |
| `crates/ferrite-transcode/src/variants.rs` | Quality variant ladder (2160p→360p) |
| `crates/ferrite-transcode/src/tonemap.rs` | HDR→SDR tone-mapping (BT.2020/PQ/HLG detection) |
| `crates/ferrite-transcode/src/audio.rs` | Audio passthrough/transcode rules |

### Key Frontend Components
| File | Purpose |
|------|---------|
| `ferrite-ui/src/components/Player.tsx` | Main player (~1100 lines, 30+ signals) |
| `ferrite-ui/src/pages/PlayerPage.tsx` | Route wrapper, episode navigation |
| `ferrite-ui/src/pages/ShowDetailPage.tsx` | TV show → season → episode hierarchy |
| `ferrite-ui/src/api.ts` | Typed API client with all endpoints |
| `ferrite-ui/src/lib/perf.ts` | Performance span tracker |

---

## What's Working Well

1. **Seek generation counter** — Prevents all stale callback issues from rapid/overlapping seeks
2. **Video element reset** — `detachMedia()` → `destroy()` → `removeAttribute('src')` → `load()` prevents black screen on seek
3. **Session expired recovery** — Detects 404 on segment load → auto-restarts at current position
4. **Custom subtitle renderer** — No browser `<track>` cue accumulation bugs; cues driven by `actualTime()` in `onTimeUpdate()`
5. **Progress reporting** — `keepalive: true` on `apiQuiet()` ensures progress saved even on unmount
6. **Cleanup discipline** — All timers, event listeners, HLS instances properly cleaned in `onCleanup()`
7. **Quality persistence across seeks** — Re-applies user's quality preference after each seek MANIFEST_PARSED

---

## Known Edge Cases & Potential Issues to Investigate

### HIGH PRIORITY — Playback Reliability

#### 1. Concurrent Session Creation Race (Backend)
**File:** `crates/ferrite-stream/src/hls.rs` — `get_or_create_session()`
**Issue:** Two simultaneous master playlist requests for the same media can each spawn separate FFmpeg processes. The second insert into `media_sessions` DashMap overwrites the first, orphaning one FFmpeg process (CPU leak).
**Impact:** Low probability for single-user, but increases with multiple browser tabs or auto-reload.
**Fix:** Use `DashMap::entry().or_insert_with()` pattern or add a mutex around session creation per media_id.

#### 2. Segment Timeout on FFmpeg Silent Failure (Backend)
**File:** `crates/ferrite-stream/src/hls.rs` — `get_segment()` polling loop
**Issue:** If FFmpeg hangs (corrupt file, permissions, disk full), `get_segment()` polls for 30 seconds before returning 404. Client experiences a long unexplained stall.
**Impact:** Rare but frustrating when it happens.
**Improvement:** Monitor FFmpeg stderr for error patterns; short-circuit the wait if stderr indicates failure.

#### 3. Pause Duration vs Session Timeout Mismatch
**Backend:** FFmpeg killed after 30s with no segment requests. Session destroyed after `session_timeout_secs` (default much longer).
**Frontend:** Session expired recovery fires on 404, but only for `FRAG_LOAD_ERROR | LEVEL_LOAD_ERROR | MANIFEST_LOAD_ERROR`.
**Edge case:** If user pauses for 31s, FFmpeg is killed. HLS.js buffer runs out → requests next segment → 404 → recovery fires → new session.
**Potential issue:** The `sessionExpiredRecovering` flag is never reset back to `false`. If the recovery seek itself fails (e.g., backend is temporarily down), the player is stuck — no further recovery attempts.
**Fix:** Reset `sessionExpiredRecovering = false` after successful recovery, or use a retry counter.

#### 4. `keepalive: true` Payload Size Limit
**File:** `ferrite-ui/src/api.ts` — `apiQuiet()`
**Issue:** `keepalive: true` has a browser-enforced 64KB payload limit. Our progress payloads are tiny (~50 bytes), so this is safe today. But if other fire-and-forget calls grow (e.g., batch operations), they could silently fail.
**Impact:** None currently. Just a design constraint to document.

#### 5. Audio Track Switch on Non-HLS Streams
**File:** `Player.tsx` — `changeAudioTrack()`
**Issue:** For non-HLS transcoded streams, the `audio_stream` param isn't applied. Audio button shows, user selects new track, nothing happens.
**Impact:** Low — most transcoded content uses HLS. But confusing when it occurs.
**Fix:** Either hide audio selector for non-HLS transcoded streams, or implement audio track selection for progressive transcode (add `audio_stream` query param to `/api/stream/{id}`).

### MEDIUM PRIORITY — UX Polish

#### 6. Subtitle Fetch Not Cancellable on Rapid Seeks
**File:** `Player.tsx` — `applySubtitle()`
**Issue:** Each seek calls `applySubtitle()` which fetches VTT. If user seeks 5 times rapidly, 5 fetches are in flight. The last one to resolve "wins" (overwrites `subtitleCues`), but there's no AbortController cancellation, so earlier responses could resolve after the latest one, causing brief subtitle flicker.
**Impact:** Very low — subtitle fetches are small text files (~50ms). But architecturally impure.
**Fix:** Use an AbortController pattern: create controller on each `applySubtitle()` call, abort previous.

#### 7. Up Next Countdown Continues When Seeking Backward
**File:** `Player.tsx` — `startUpNextCountdown()`
**Issue:** If user seeks backward out of the final 30 seconds after Up Next has started, `upNextStarted = true` is set, but the counter keeps running. The `cancelUpNext()` function exists but isn't called on seek.
**Fix:** In `hlsSeekTo()` or `seekToTime()`, check if target is outside the final 30s window and call `cancelUpNext()`.

#### 8. Double-Click Fullscreen Conflicts with Play/Pause Click
**File:** `Player.tsx` — video element `onClick` + `onDblClick`
**Issue:** Both `onClick` and `onDblClick` fire on double-click. The first click triggers play/pause, then the double-click triggers fullscreen/seek. This causes a brief play→pause→play cycle.
**Impact:** Minor visual flicker on double-click.
**Fix:** Debounce single-click with ~250ms delay; cancel if double-click detected.

#### 9. Volume Slider Not Accessible via Keyboard
**Issue:** Volume slider is an `<input type="range">` nested inside a hover-reveal container. Keyboard-only users can't easily adjust volume (must use ↑/↓ keys, which is undiscoverable).
**Impact:** Accessibility concern.
**Fix:** Add ARIA labels, ensure tab focus reveals the slider.

#### 10. Controls Auto-Hide While Menu Is Open
**File:** `Player.tsx` — `showControls()` / `hideControlsDelayed()`
**Issue:** The `showQualityMenu` and `showSubtitleMenu` are checked in the hide delay guard, but if the user opens a menu, moves mouse out of the player area (e.g., to a second monitor), and doesn't touch anything for 3s, controls won't hide because the menu check prevents it. However, the menu itself has no timeout.
**Impact:** Minor — controls staying visible while a menu is open is arguably correct behavior.
**Note:** This is working as designed; just documenting the interaction.

### LOW PRIORITY — Performance Optimization

#### 11. HLS.js Buffer Tuning for Seedbox Bandwidth
**File:** `Player.tsx` — `HLS_CONFIG`
**Current:** `maxBufferLength: 30`, `maxMaxBufferLength: 60`, `abrEwmaDefaultEstimate: 10Mbps`
**Consideration:** On a seedbox with high bandwidth (100Mbps+), we could afford larger buffers and more aggressive prefetch. On mobile/limited bandwidth, smaller buffers reduce memory usage.
**Improvement:** Make HLS config adaptive based on detected bandwidth or configurable via user preferences.

#### 12. Subtitle Cue Search is O(n) Linear Scan
**File:** `Player.tsx` — `onTimeUpdate()` subtitle cue search
**Issue:** `subtitleCues.find(c => t >= c.start && t < c.end)` scans the entire array every ~16ms. For a 2-hour movie with 1500+ cues, this is trivial (~0.01ms), but could be optimized with binary search.
**Impact:** Negligible. Only worth doing if profiling shows it matters.

#### 13. Master Playlist Header Fetch is Redundant
**File:** `Player.tsx` — initial playback `fetch(authUrl(masterUrl))` before HLS.js load
**Issue:** A separate `fetch()` call reads `x-hls-session-ids` from response headers. Then HLS.js also fetches the same URL. This means the master playlist is fetched twice on initial load.
**Impact:** ~1 extra HTTP request (~5ms on localhost). The session ID extraction is necessary for cleanup, but could be done via a custom HLS.js loader or `Hls.Events.MANIFEST_LOADED` data.

---

## Backend Hardening Opportunities

### 14. FFmpeg Process Orphan Prevention
**File:** `crates/ferrite-stream/src/hls.rs`
**Current:** FFmpeg stderr is captured in a spawned task, but the output is just logged at WARN level.
**Improvement:** Parse stderr for known error patterns (`No such file`, `Permission denied`, `Disk quota exceeded`) and proactively fail the session instead of waiting for the 30s segment timeout.

### 15. Segment Serving — Cache-Control Headers
**Current:** Segments served with `Cache-Control: max-age=3600`.
**Issue:** If a reverse proxy (nginx, Cloudflare) caches segments, and the user seeks (destroying the session), cached segment URLs now point to deleted files. Subsequent requests from cache serve stale 200s to HLS.js, which may cause decoder errors.
**Fix:** Use `Cache-Control: no-store` for playlist files (they change), `max-age=3600` for segments (immutable once written). Or scope to `private` to prevent CDN caching.

### 16. Graceful FFmpeg Shutdown
**Current:** `child.start_kill()` sends SIGKILL (immediate).
**Improvement:** Send SIGTERM first, wait 2s, then SIGKILL. This lets FFmpeg flush its write buffers and close the playlist cleanly, preventing truncated final segments.

### 17. HLS Session Timeout Configuration
**Current:** Cleanup runs every 15s; FFmpeg killed after 30s idle; session destroyed after configurable timeout.
**Edge case:** The 30s FFmpeg kill is hardcoded. For users on very slow connections (satellite, rural), segments take longer to download — a 30s idle window may be too aggressive.
**Fix:** Make the FFmpeg idle timeout configurable (e.g., `hls_ffmpeg_idle_secs = 30`).

---

## Player.tsx State Machine Reference

```
INIT → onMount()
  ├─ Fetch audio tracks, subtitles, chapters, next episode (parallel)
  ├─ Load server preferences (language defaults)
  ├─ Restore session preferences (audio, subtitle, quality)
  ├─ Determine stream type → HLS.js / Native HLS / Progressive / Direct
  ├─ Create HLS instance → loadSource → attachMedia
  └─ MANIFEST_PARSED → populate quality → restore quality pref → play()

PLAYING → onTimeUpdate() fires ~60fps
  ├─ Update currentTime signal (capped at knownDuration)
  ├─ Scan subtitle cues for active cue
  ├─ Report progress every 10s
  └─ Check if within final 30s → show Up Next overlay

SEEKING → hlsSeekTo() / seekToTime()
  ├─ Bump seekGeneration (stale callback detection)
  ├─ Set isSeeking=true, buffering=true
  ├─ Destroy old HLS instance + reset video element
  ├─ Call backend seek API
  ├─ Create new HLS instance → loadSource → attachMedia
  ├─ MANIFEST_PARSED → re-apply subtitle + quality + play()
  └─ 200ms delay → isSeeking=false

SESSION_EXPIRED → 404 on segment/playlist load
  ├─ Detect via HLS.Events.ERROR (404 + FRAG/LEVEL/MANIFEST load error)
  ├─ Set sessionExpiredRecovering=true
  └─ hlsSeekTo(currentTime()) → new session at current position

EPISODE_CHANGE → Up Next fires / user navigates
  ├─ markCompleted(current episode)
  ├─ props.onNextEpisode(nextMediaId) → route navigation
  ├─ Old Player unmounts → onCleanup → destroyHls
  └─ New Player mounts fresh → full INIT cycle

CLOSE → handleClose()
  ├─ Report progress (lastConfirmedTime if mid-seek)
  ├─ destroyHls() → kill FFmpeg session
  ├─ Reset video element
  └─ props.onClose() → unmount
```

---

## Testing Checklist for Bulletproof Playback

### Functional Tests
- [ ] Direct play (MP4 + H.264 + AAC) — play, seek, resume
- [ ] Remux (MKV + H.264 + AAC) — play, seek, close
- [ ] Audio transcode (MKV + H.264 + DTS) — play, seek, audio track switch
- [ ] Full transcode (MKV + HEVC + DTS) — play, seek, quality switch
- [ ] HDR content (4K HEVC + BT.2020 + PQ) — tone-mapping, no washed-out colors
- [ ] 10-bit SDR (anime with 10-bit H.264) — no unnecessary tone-mapping

### Seek Tests
- [ ] Single seek forward/backward
- [ ] Rapid seeking (mash → → → rapidly)
- [ ] Seek during active seek (interrupt in-flight seek)
- [ ] Seek to very start (0s) and very end (duration - 1s)
- [ ] Timeline drag-scrub across full duration
- [ ] Double-click seek (left/right third of player)

### Session Recovery Tests
- [ ] Pause for 35s → resume → should auto-recover (new session)
- [ ] Pause for 35s → seek → should work cleanly
- [ ] Close player while paused → reopen → should resume from last position
- [ ] Close player during active seek → reopen → should resume from lastConfirmedTime

### Audio/Subtitle/Quality Tests
- [ ] Switch audio track mid-playback → verify audio changes
- [ ] Switch subtitle track → verify cues display correctly
- [ ] Switch quality level → verify resolution changes
- [ ] Switch quality during seek → verify preference persists
- [ ] Episode transition → verify audio/subtitle/quality preferences carry over

### Up Next Tests
- [ ] Let episode reach final 30s → Up Next overlay appears
- [ ] Cancel Up Next → overlay dismissed, playback continues
- [ ] Let countdown reach 0 → auto-navigates to next episode
- [ ] Seek backward out of final 30s → verify behavior (currently: overlay stays)
- [ ] No next episode → verify no overlay appears

### Edge Cases
- [ ] Open player → close immediately (before MANIFEST_PARSED fires)
- [ ] Two browser tabs playing same media simultaneously
- [ ] Network interruption mid-playback → verify recovery
- [ ] Extremely long media (4+ hours) — timeline precision, progress reporting
- [ ] Missing duration metadata — player doesn't crash
- [ ] PiP mode → controls, playback continues

---

## Files Modified in Phase 2A (for reference)

| File | Changes |
|------|---------|
| `ferrite-ui/src/components/Player.tsx` | Quality selector, subtitle picker, audio persistence, chapter markers, Up Next, session recovery, preference persistence, seek generation counter |
| `ferrite-ui/src/components/MediaCard.tsx` | Unwatched dot indicator |
| `ferrite-ui/src/api.ts` | New types (ExternalSubtitle, Chapter, NextEpisode, TvShow, Season, Episode, User, UserPreferences, ActiveStream), new API functions (listSubtitles, listChapters, nextEpisode, listUsers, etc.), `keepalive: true` on apiQuiet |
| `ferrite-ui/src/pages/ShowsPage.tsx` | New — TV show grid |
| `ferrite-ui/src/pages/ShowDetailPage.tsx` | New — Show → Season → Episode hierarchy |
| `ferrite-ui/src/pages/AdminPage.tsx` | New — Live stream activity dashboard |
| `ferrite-ui/src/pages/SettingsPage.tsx` | User management section (admin) |
| `ferrite-ui/src/pages/PlayerPage.tsx` | Episode navigation, onNextEpisode callback |
| `crates/ferrite-db/src/tv_repo.rs` | Next episode query (two-CTE approach) |
| `crates/ferrite-db/src/chapter_repo.rs` | New — chapter extraction |
| `crates/ferrite-db/src/preference_repo.rs` | New — user preferences |
| `crates/ferrite-api/src/handlers/tv.rs` | Next episode endpoint |
| `crates/ferrite-api/src/handlers/chapter.rs` | Chapter listing endpoint |
| `crates/ferrite-api/src/handlers/preference.rs` | Preference get/set endpoints |
| `crates/ferrite-api/src/handlers/admin.rs` | Active streams endpoint |
| `migrations/009_chapters.sql` | Chapters table |
| `migrations/010_user_preferences.sql` | User preferences table |
| `ROADMAP.md` | Phase 2A marked complete, Phase 2B partial |

---

## User Preferences & Constraints

- **Don't run `npm run dev`** — user runs dev servers manually
- **Must stop `ferrite.exe` before `cargo build`** on Windows — use `cargo check` for quick verification
- **Uses JetBrains Rider** (Rust) and **WebStorm** (frontend)
- **Pre-existing TS error**: `PlayerPage.tsx(15,30)` has a `string | string[]` type error — not related to player, acceptable
- **113 tests must pass**: `cargo test --workspace`
- **Zero clippy warnings**: `cargo clippy --workspace -- -D warnings`
