# Ferrite Codebase Review (Performance + Streaming Focus)

Date: 2026-02-20

## Scope and lens

This review focuses on what most impacts your stated goals:

1. **Fast playback start (low TTFF)**
2. **High quality playback with minimal unnecessary transcode loss**
3. **Fast buffering + fast seeking**
4. **Platform-agnostic evolution (web, iOS, Android, Roku, Apple TV, etc.)**

I reviewed backend stream/transcode/session code, frontend player behavior, DB/auth hot paths, and related config/migrations.

---

## Executive verdict

Ferrite has a strong foundation (Rust, explicit stream strategy tiers, HLS session manager, WAL tuning), but in its current shape it is **optimized for a narrower single-client flow**, not yet for **Plex-class multi-device / multi-client high-performance streaming**.

Your biggest blockers are architectural and hot-path issues, not syntax-level inefficiencies:

- session model is media-global (clients can interfere)
- ABR is effectively disabled in production path
- HLS playlist/segment strategy scales poorly over long sessions
- auth middleware performs DB reads on segment requests
- seek path repeatedly invokes ffprobe and often re-encodes

If those are fixed first, you’ll unlock disproportionate gains.

---

## Findings (ordered by severity)

## P0 (Critical)

### 1) Per-media singleton HLS sessions cause cross-client interference

**Why this hurts**
- Two clients watching the same media can affect each other’s session lifecycle and seek behavior.
- This is a hard blocker for true multi-device usage and horizontal scaling.

**Evidence**
- Session mappings are keyed by `media_id`, not playback instance: @crates/ferrite-stream/src/hls.rs#137-143
- Master endpoint reuses any existing media variant session, regardless of caller/session identity: @crates/ferrite-api/src/handlers/stream.rs#310-322
- Seek path destroys/recreates media sessions (global for that media): @crates/ferrite-api/src/handlers/stream.rs#584-605 and @crates/ferrite-stream/src/hls.rs#505-520

**Recommendation**
- Introduce a **playback_session_id** (per user/device/play action), and scope FFmpeg session ownership to it.
- Keep optional dedup/caching at segment level, not by force-sharing one active transcode process per media item.

---

### 2) ABR ladder exists, but runtime path uses single-variant only

**Why this hurts**
- No real adaptive bitrate under fluctuating network/CPU conditions.
- Prevents robust cross-device experience and increases buffering risk.

**Evidence**
- `create_variant_sessions` exists but is not called from stream handlers: @crates/ferrite-stream/src/hls.rs#372-439
- Handlers call `create_single_variant_session` for both initial load and seek: @crates/ferrite-api/src/handlers/stream.rs#334-355 and #584-605

**Recommendation**
- Enable true multi-variant ABR for normal playback.
- Option: keep fast-start single variant for first seconds, then upgrade to full ladder.

---

### 3) HLS playlist mode and flags create unbounded growth (CPU, disk, latency pressure)

**Why this hurts**
- Long sessions keep appending playlist history.
- Playlist rewrite/parsing costs grow over time.
- Segment storage growth can become large under concurrency.

**Evidence**
- FFmpeg HLS args use `append_list` + `playlist_type event`: @crates/ferrite-stream/src/hls.rs#781-785
- Variant playlist is read and rewritten every request: @crates/ferrite-stream/src/hls.rs#846-869
- Segment readiness polling repeatedly reads playlist: @crates/ferrite-stream/src/hls.rs#917-954

**Recommendation**
- For VOD streaming, use a bounded strategy (`hls_list_size`, delete policy) and avoid unbounded EVENT growth.
- Reduce repeated full playlist parsing in hot path.

---

### 4) Auth middleware does DB existence checks on streaming requests

**Why this hurts**
- HLS can generate frequent request volume (manifest + segments).
- Per-request DB lookups add latency and contention for no direct playback value.

**Evidence**
- DB user check for Bearer token path: @crates/ferrite-api/src/auth.rs#89-104
- DB user check for token query path: @crates/ferrite-api/src/auth.rs#123-136
- Streaming routes are under auth middleware layer: @crates/ferrite-api/src/router.rs#65-74 and #91-94

**Recommendation**
- Validate JWT signature + expiry in middleware without DB on every segment.
- If revocation is needed, use in-memory revocation/version cache with TTL.

---

### 5) Progressive transcode/remux pipe stderr is not drained (possible FFmpeg stall)

**Why this hurts**
- If ffmpeg writes enough stderr output and nobody consumes it, process can block.
- This manifests as random stalls/buffering freezes in long-running streams.

**Evidence**
- Child spawned with `stderr(Stdio::piped())` in remux/audio/full transcode: @crates/ferrite-stream/src/transcode.rs#210-214, #381-385, #574-578
- No stderr consumer attached in these paths; only stdout is streamed.

**Recommendation**
- Either consume stderr asynchronously, or force very quiet ffmpeg logging and redirect stderr to null.

---

### 16) Multi-user playback progress schema still enforces global-per-media uniqueness

**Why this hurts**
- The schema still enforces one `playback_progress` row per media item globally, which conflicts with per-user progress expectations.
- Under multiple users/devices, updates can overwrite/fail and downstream list queries can become semantically wrong.

**Evidence**
- Original unique constraint remains: `UNIQUE(media_item_id)`: @migrations/001_initial_schema.sql#34-42
- Later migration adds composite uniqueness, but does not remove original constraint: @migrations/005_users.sql#17-22
- Media/episode queries join playback progress only by `media_item_id` (no user filter): @crates/ferrite-db/src/movie_repo.rs#173-174 and @crates/ferrite-db/src/tv_repo.rs#287-290

**Recommendation**
- Migrate `playback_progress` to strict `(user_id, media_item_id)` uniqueness only.
- Ensure all API-facing media list/detail queries filter/join progress by the authenticated user.

---

## P1 (High)

### 6) Seek path is expensive (ffprobe per seek + frequent re-encode path)

**Why this hurts**
- Seek responsiveness degrades under rapid scrubbing and concurrent users.
- CPU spikes from repeated keyframe probing and re-encoding workflows.

**Evidence**
- `find_keyframe_before()` executes ffprobe process per call: @crates/ferrite-stream/src/transcode.rs#61-101
- Called in HLS master and HLS seek handlers: @crates/ferrite-api/src/handlers/stream.rs#290-297 and #553-560
- Video copy in HLS only allowed for near-zero start (`start_secs < 0.5`): @crates/ferrite-stream/src/hls.rs#643-647

**Recommendation**
- Precompute/store keyframe index during scan.
- Offer two seek modes: **fast keyframe seek** (copy-friendly) vs **precise seek** (re-encode).

---

### 7) Native HLS (Safari/iOS path) has start/session lifecycle gaps

**Why this hurts**
- Resume/seek behavior can be slower and less deterministic on native HLS clients.
- Session cleanup can lag/leak until timeout.

**Evidence**
- Native HLS source does not pass `start` in URL: @ferrite-ui/src/components/Player.tsx#454-463
- Session stop depends on `hlsSessionId`, which native branch does not set from headers/events: @ferrite-ui/src/components/Player.tsx#212-217

**Recommendation**
- Unify startup flow so native HLS also obtains explicit session/start metadata and session id.
- Ensure explicit stop endpoint call on close for native HLS sessions.

---

### 8) HLS segment wait path uses polling + full playlist reads

**Why this hurts**
- Adds avoidable latency and CPU overhead under load.

**Evidence**
- Poll loop every 500ms + playlist read/parsing in wait path: @crates/ferrite-stream/src/hls.rs#917-954

**Recommendation**
- Switch to more event-driven segment readiness strategy (or lower-overhead state tracking).

---

### 9) Multi-user playback schema/query model is inconsistent and can leak/wrongly merge progress

**Why this hurts**
- Incorrect resume states and duplicate joins in user scenarios.
- Performance degradation due over-join cardinality.

**Evidence**
- Old unique constraint remains on `playback_progress(media_item_id)`: @migrations/001_initial_schema.sql#34-42
- New composite unique index added later: @migrations/005_users.sql#21-22
- Media queries join playback_progress by media_item_id only (no user filter): @crates/ferrite-db/src/movie_repo.rs#173-174 and #237-238
- TV episodes query also joins on media_item_id only: @crates/ferrite-db/src/tv_repo.rs#287-290

**Recommendation**
- Migrate `playback_progress` uniqueness to `(user_id, media_item_id)` only.
- Always join/filter by current user_id in API-facing media queries.

---

### 10) Static codec/container compatibility model is not client-aware

**Why this hurts**
- You’ll over-transcode for capable devices and under-serve incompatible ones.
- This undermines both quality and performance at platform scale.

**Evidence**
- Backend compatibility lists are fixed/global: @crates/ferrite-stream/src/compat.rs#1-46
- Frontend duplicates separate compatibility logic: @ferrite-ui/src/utils.ts#39-57

**Recommendation**
- Introduce **client capability profiles** (web-chrome, safari-ios, tvOS, Android, Roku, etc.)
- Make strategy decisions per request/profile, not globally.

---

### 11) HLS master playlist lacks richer signaling needed for broad client compatibility

**Why this hurts**
- Some clients and heuristics perform better with explicit CODECS/AUDIO metadata.

**Evidence**
- Stream-INF generation includes BANDWIDTH/NAME/RESOLUTION, but not CODECS: @crates/ferrite-stream/src/hls.rs#837-840

**Recommendation**
- Emit CODECS and related attributes in master playlists.

---

### 12) 6-second segment default trades startup/seek speed for throughput stability

**Why this hurts**
- Longer segments increase startup and seek granularity latency.
- Contradicts “fast as possible seek/buffer” target.

**Evidence**
- Default segment duration is 6 seconds: @crates/ferrite-core/src/config.rs#78-80

**Recommendation**
- For VOD low-latency feel, target 2s segments (and tune startup buffer policy).

---

### 17) Transcode admission is fail-fast (`try_acquire`) instead of queued

**Why this hurts**
- During spikes, users get immediate `503` instead of bounded queueing/backpressure.
- This harms perceived reliability and causes avoidable playback retries.

**Evidence**
- Remux/audio/full/HLS session creation all use `try_acquire()` and reject when saturated: @crates/ferrite-api/src/handlers/stream.rs#76-81, #105-110, #133-138, #324-330, #574-580

**Recommendation**
- Introduce queued admission with timeout (or class-based scheduling) so short bursts do not hard-fail playback.

---

### 18) File watcher rescans entire library on any filesystem change

**Why this hurts**
- A single file event can trigger full-library scan work.
- On busy libraries this can create sustained DB/transcode contention that degrades playback responsiveness.

**Evidence**
- Changed path is mapped to library id, then full `scan_library()` is invoked for that library: @crates/ferrite-scanner/src/watcher.rs#103-124

**Recommendation**
- Move to incremental path-scoped rescans (changed-file pipeline) instead of whole-library rescans.
- Keep full scans for explicit/manual operations only.

---

## P2 (Medium)

### 13) HLS segment content type for `.m4s` may be suboptimal for some clients

**Evidence**
- `video/iso.segment` used for `.m4s`: @crates/ferrite-api/src/handlers/stream.rs#464-470

**Recommendation**
- Validate against target clients (Roku/tvOS/etc.) and consider standardized serving semantics.

---

### 14) Search scalability work is partially landed but not fully integrated

**Evidence**
- FTS table exists in migration: @migrations/013_fts5_search.sql#4-35
- Current listing path still uses LIKE filtering: @crates/ferrite-db/src/movie_repo.rs#241-244 and #271-273

**Recommendation**
- Route high-cardinality title search to FTS for large libraries.

---

### 15) Automated test coverage for playback-critical integration is minimal

**Evidence**
- `tests/integration/` is empty.

**Recommendation**
- Add regression tests for seek/reuse/session teardown/auth hot path behavior.

---

## What Ferrite is already doing well

- Good direct/remux/audio/full strategy model baseline: @crates/ferrite-stream/src/compat.rs#27-70
- WAL-mode and practical SQLite pragmas: @crates/ferrite-db/src/lib.rs#25-43
- HLS process lifecycle cleanup loop and idle-kill controls: @crates/ferrite-stream/src/hls.rs#1040-1075
- Thoughtful tone-mapping path for HDR/10-bit sources: @crates/ferrite-transcode/src/tonemap.rs#53-106

---

## Recommended architecture direction for platform-agnostic high performance

### 1) Move to explicit playback sessions (not media-global sessions)

- Session key should include playback identity (user/device/session UUID).
- Maintain independent seek/timeline per client.
- Optional segment cache can still deduplicate compute.

### 2) Introduce client capability profiles

Per client family, negotiate:
- max resolution / bitrate
- codec/container support (H264/H265/AV1/VP9)
- audio support (AAC/AC3/EAC3/Opus/etc.)
- HDR support

This unlocks both **quality** and **CPU efficiency**.

### 3) Implement true ABR ladder and startup strategy

- Re-enable multi-variant sessions for normal playback.
- Keep quick start path if desired, but switch to full ladder quickly.

### 4) Fix hot path auth and playlist scaling

- Eliminate DB checks per segment request.
- Bound playlists/segment retention and reduce polling overhead.

### 5) Add hard playback SLOs + telemetry

Track per playback session:
- TTFF (time to first frame)
- startup failure rate
- rebuffer ratio + count
- median/95p seek completion time
- segment generation latency
- transcode queue wait time

Without this, optimization is guesswork.

---

## Suggested execution order (implementation roadmap)

### Phase 0 (1-2 days): instrumentation + guardrails
- Add metrics around startup, rebuffer, seek, transcode queue, auth latency.
- Add synthetic playback benchmark script.

### Phase 1 (1 week): remove major bottlenecks
- Fix stderr drain issue in progressive transcode paths.
- Remove per-segment DB auth checks.
- Bound HLS playlist/segment growth.
- Reduce segment duration for latency-oriented profile.

### Phase 2 (1-2 weeks): session architecture correctness
- Convert to per-playback session ownership.
- Ensure seek/stop cannot disrupt other clients.
- Normalize native HLS lifecycle behavior.

### Phase 3 (2-4 weeks): quality + cross-platform maturity
- Turn on true ABR ladder.
- Add client profile negotiation and codec-aware decisions.
- Add CODECS-rich manifests and platform validation matrix (web/iOS/Android/Roku/tvOS).

---

## Bottom line

If your target is truly “extremely high performing Plex alternative,” your current biggest gains will come from:

1. **session model redesign**
2. **real ABR activation**
3. **hot-path auth + HLS scaling fixes**
4. **client capability-driven strategy decisions**

Those four areas are the difference between “works on my browser” and “fast, reliable, multi-device streaming platform.”
