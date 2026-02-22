# Ferrite Implementation Plan (Address Every Suggestion from 2026-02-20 Review)

Source review: @docs/temp-codebase-review-2026-02-20.md#1-348
Date: 2026-02-20

---

## 1) Program goal

Ship Ferrite as a **platform-agnostic, high-performance streaming backend** with:

- fast start (low TTFF)
- fast seek/buffering
- high quality under varying network/device constraints
- correct multi-user behavior
- scalable behavior under concurrency

---

## 2) Success metrics (release gates)

### Playback SLOs (must-have)

1. TTFF p95:
   - direct/remux: <= 1.5s (LAN), <= 2.5s (WAN)
   - transcode/HLS: <= 3.0s
2. Seek completion p95:
   - buffered seek: <= 400ms
   - unbuffered seek: <= 1.5s
3. Rebuffer ratio p95 per session: <= 1.0%
4. Startup failure rate: < 0.5%
5. Segment request auth overhead p95: <= 1ms server-side CPU
6. No cross-client session interference during concurrent playback of same media

### Correctness gates

1. Per-user playback progress is isolated and never overwritten by another user.
2. Session stop/seek on one client cannot break another client playing same media.
3. ABR actually switches levels under induced bandwidth changes.

---

## 3) Traceability matrix (every review suggestion)

| Review ID | Suggestion | Planned workstream | Milestone |
|---|---|---|---|
| R1 | Per-media singleton HLS sessions | WS2 Session Model Redesign | M2 |
| R2 | ABR exists but runtime uses single variant | WS3 ABR + Ladder Runtime | M3 |
| R3 | Unbounded HLS playlist growth | WS1 HLS Scalability | M1 |
| R4 | Auth middleware DB checks on stream hot path | WS1 Auth Hot Path | M1 |
| R5 | Progressive transcode stderr not drained | WS1 Transcode Process Reliability | M1 |
| R6 | Seek path expensive (ffprobe per seek, re-encode heavy) | WS4 Seek Optimization | M4 |
| R7 | Native HLS lifecycle/start gaps | WS2 Session + Native HLS Consistency | M2 |
| R8 | Segment wait uses polling + full playlist reads | WS1 Segment Readiness Refactor | M1-M2 |
| R9 | Multi-user playback schema/query inconsistency | WS5 Multi-user Data Correctness | M2 |
| R10 | Static global compatibility model (not client-aware) | WS3 Client Capability Profiles | M3 |
| R11 | HLS master playlist lacks CODECS metadata | WS3 Manifest Correctness | M3 |
| R12 | Segment duration default too high for low-latency feel | WS1 Latency tuning | M1 |
| R13 | m4s content type/client compatibility validation | WS3 Cross-client Compatibility | M3 |
| R14 | FTS not integrated into query path | WS6 Search Scalability | M4 |
| R15 | Missing playback-critical integration tests | WS7 Test Harness + Regression | M1-M4 |
| R16 | playback_progress still globally unique per media | WS5 Multi-user Data Correctness | M2 |
| R17 | Transcode admission fail-fast (try_acquire) | WS1 Queueing + Admission Control | M1 |
| R18 | Watcher rescans full library on any file change | WS6 Incremental Scan Pipeline | M4 |
| A5 | Add hard playback SLOs + telemetry | WS0 Observability Foundation | M0 |

Note: R9 and R16 overlap; both are explicitly covered.

---

## 4) Workstreams and concrete implementation tasks

## WS0 — Observability foundation (A5)

### Objective
Provide metrics and benchmarking before changing architecture.

### Tasks
1. Add playback/session metrics collection in API + player.
   - Backend touchpoints: @crates/ferrite-api/src/handlers/stream.rs#33-655, @crates/ferrite-stream/src/hls.rs#137-1218
   - Frontend touchpoints: @ferrite-ui/src/components/Player.tsx#247-1199, @ferrite-ui/src/lib/perf.ts#35-158
2. Define metric names and labels:
   - `playback_ttff_ms`, `seek_latency_ms`, `rebuffer_count`, `rebuffer_ms`, `auth_hotpath_ms`, `transcode_queue_wait_ms`, `hls_segment_wait_ms`
3. Add synthetic benchmark script (single-user and N-user scenarios) for regression runs.
4. Add dashboard/query templates and weekly perf trend snapshots.

### Acceptance
- Baseline metrics available for direct/remux/transcode/HLS.
- Repeatable benchmark output checked into docs.

---

## WS1 — Stream hot-path reliability + scalability (R3, R4, R5, R8, R12, R17)

### Objective
Remove immediate bottlenecks and reduce failures under load.

### Tasks

#### R5: Drain FFmpeg stderr in progressive transcode paths
1. In remux/audio/full transcode paths, consume stderr asynchronously or disable pipe buffering safely.
   - @crates/ferrite-stream/src/transcode.rs#210-214
   - @crates/ferrite-stream/src/transcode.rs#381-385
   - @crates/ferrite-stream/src/transcode.rs#574-578
2. Add long-run test to ensure no process stall under sustained playback.

#### R4: Remove DB roundtrip from auth hot path for segments
1. Split auth policy:
   - signature+expiry validation only on high-volume segment routes
   - optional revocation/version cache with TTL
2. Avoid per-request `get_user_by_id` in stream request hot path.
   - @crates/ferrite-api/src/auth.rs#89-104 and #123-136
3. Add metric for auth decision latency p95/p99.

#### R3 + R8: Bound playlist/segment growth and reduce polling
1. Replace VOD `event+append_list` defaults with bounded VOD strategy.
   - @crates/ferrite-stream/src/hls.rs#781-785
2. Add list-size + segment deletion policy.
3. Replace repeated full playlist polling in `wait_for_segment()` with lower-overhead readiness tracking.
   - @crates/ferrite-stream/src/hls.rs#917-954
4. Keep fallback polling with reduced cost/timeouts for safety.

#### R12: Lower segment duration for latency profile
1. Introduce config profile (latency-first vs throughput-first).
2. Set latency profile default `hls_segment_duration` to 2s.
   - @crates/ferrite-core/src/config.rs#78-80
3. Tune GOP/keyint and startup buffer accordingly.

#### R17: Replace fail-fast transcode admission with queued admission
1. Replace `try_acquire()` with queue+timeout admission policy.
   - @crates/ferrite-api/src/handlers/stream.rs#76-81, #105-110, #133-138, #324-330, #574-580
2. Add queue wait telemetry and fairness policy (optional priority classes).
3. Return structured overload response only on timeout, not instant spike.

### Acceptance
- No stderr-induced stalls in soak test.
- Segment/auth CPU overhead reduced measurably.
- Fewer 503s during burst traffic.
- Startup/seek p95 improves against baseline.

---

## WS2 — Session model redesign + native HLS consistency (R1, R7)

### Objective
Eliminate cross-client interference and normalize session lifecycle across HLS.js and native HLS.

### Tasks

#### R1: Per-playback session ownership
1. Introduce `playback_session_id` (server-generated UUID) and model session maps by playback session, not just media id.
   - @crates/ferrite-stream/src/hls.rs#137-143
2. Update handlers to require/propagate playback session context.
   - @crates/ferrite-api/src/handlers/stream.rs#258-655
3. Keep optional media-level dedupe cache separate from ownership semantics.
4. Add explicit lifecycle API for start/heartbeat/stop.

#### R7: Native HLS parity
1. Ensure native HLS startup receives actual start/session metadata (same behavior as HLS.js flow).
   - @ferrite-ui/src/components/Player.tsx#454-463
2. Ensure stop path always cleans up session even when `hlsSessionId` was not initialized by HLS.js events.
   - @ferrite-ui/src/components/Player.tsx#212-217
3. Validate resume/seek parity across Safari/iOS vs Chromium/HLS.js.

### Acceptance
- Two clients on same media no longer affect each other.
- Native HLS and HLS.js show equivalent session cleanup and seek semantics.

---

## WS3 — ABR runtime, client capability profiles, and manifest correctness (R2, R10, R11, R13)

### Objective
Enable real adaptive playback quality across heterogeneous clients.

### Tasks

#### R2: Enable true multi-variant ABR in runtime
1. Use multi-variant session creation for normal playback path.
   - Existing implementation exists: @crates/ferrite-stream/src/hls.rs#372-439
   - Current runtime uses single-variant: @crates/ferrite-api/src/handlers/stream.rs#334-355
2. Optional fast-start strategy:
   - Start with one variant for first-frame speed
   - Promote to full ladder after startup window

#### R10: Client capability profiles
1. Add capability resolver (request headers + UA + explicit client profile override).
2. Strategy selection becomes per-client, not global static mapping.
   - Current static logic: @crates/ferrite-stream/src/compat.rs#1-72 and @ferrite-ui/src/utils.ts#39-57
3. Define initial profiles: web-chrome, safari-ios, android, tvos, roku.

#### R11: Manifest metadata completeness
1. Add `CODECS` (and related useful tags) to master playlist `EXT-X-STREAM-INF` generation.
   - @crates/ferrite-stream/src/hls.rs#807-840
2. Verify parsers/players on all target platforms.

#### R13: m4s content type compatibility validation
1. Build compatibility test matrix for segment MIME behavior across clients.
   - current mapping: @crates/ferrite-api/src/handlers/stream.rs#464-470
2. Introduce configurable compatibility mode if needed.

### Acceptance
- ABR switches levels under network shaping tests.
- No avoidable transcode due to incorrect static compatibility assumptions.
- Manifest passes validation and client playback matrix.

---

## WS4 — Seek optimization and keyframe indexing (R6)

### Objective
Make seek fast and predictable without repeated ffprobe process overhead.

### Tasks
1. Extend scanner to persist keyframe index metadata (or coarse seek map) during probe pass.
   - probe entry: @crates/ferrite-scanner/src/probe.rs#61-210
2. Add DB table + repository for keyframe lookup by media id.
3. Replace per-seek ffprobe process calls where possible.
   - Current repeated calls: @crates/ferrite-stream/src/transcode.rs#61-101 and @crates/ferrite-api/src/handlers/stream.rs#290-297 and #553-560
4. Implement dual seek modes:
   - fast keyframe seek (default for responsiveness)
   - precise seek (opt-in)

### Acceptance
- Seek p95 significantly improved under rapid scrub tests.
- ffprobe process count during seek-heavy runs reduced by >90%.

---

## WS5 — Multi-user progress correctness and query isolation (R9, R16)

### Objective
Guarantee per-user progress correctness and query consistency.

### Tasks
1. Add migration to rebuild `playback_progress` constraints:
   - remove implicit global uniqueness by media
   - enforce only `(user_id, media_item_id)` uniqueness
   - preserve legacy rows safely
2. Update repository upsert/query behavior accordingly.
   - @crates/ferrite-db/src/progress_repo.rs#20-70 and #156-183
3. Update media/tv list/detail queries to join playback progress by user_id context.
   - @crates/ferrite-db/src/movie_repo.rs#173-174 and #237-238
   - @crates/ferrite-db/src/tv_repo.rs#287-290
4. Add migration validation and backfill script checks.

### Acceptance
- Multiple users can keep distinct progress for same media without conflict.
- API list/detail returns correct user-specific progress fields.

---

## WS6 — Scanner and search scalability (R14, R18)

### Objective
Reduce unnecessary scan pressure and improve large-library search performance.

### Tasks

#### R18: Incremental watcher scans
1. Replace whole-library rescan trigger on file changes with path-scoped incremental pipeline.
   - Current behavior: @crates/ferrite-scanner/src/watcher.rs#103-124
2. Keep explicit full scan endpoint unchanged for manual operations.
   - @crates/ferrite-api/src/handlers/library.rs#48-105
3. Add debounce + batch + dedupe for high event burst periods.

#### R14: Integrate FTS search path
1. Route title/overview search in media listing to FTS when query present.
   - FTS migration exists: @migrations/013_fts5_search.sql#4-35
   - Current LIKE filtering: @crates/ferrite-db/src/movie_repo.rs#241-244
2. Add fallback path if FTS unavailable.
3. Add search relevance and latency tests.

### Acceptance
- Watcher change storm does not trigger expensive full scans.
- Search latency and CPU improve on large data sets.

---

## WS7 — Test harness and regression safety net (R15)

### Objective
Prevent regressions as architecture changes land.

### Tasks
1. Populate `tests/integration/` with playback-critical scenarios:
   - concurrent same-media playback isolation
   - ABR switch under throttling
   - seek within-buffer reuse + outside-buffer recreate
   - auth hot path load test
   - progress isolation by user
2. Add API contract tests for new session/capability behavior.
3. Add perf regression CI job for benchmark script output thresholds.

### Acceptance
- Integration tests cover all high-risk playback flows.
- CI blocks regressions on latency and error-rate thresholds.

---

## 5) Milestone schedule (pragmatic order)

## M0 (Days 1-2) — Instrumentation + baseline
- WS0 complete baseline metrics and benchmark harness.

## M1 (Week 1) — Hot-path stabilization
- WS1: R5, R4, R3, R12, R17 initial delivery.
- WS7: initial integration scaffolding.

## M2 (Weeks 2-3) — Correctness architecture
- WS2: R1 + R7
- WS5: R9 + R16 schema/query fixes
- WS1: R8 completion

## M3 (Weeks 4-6) — Quality/platform maturity
- WS3: R2 + R10 + R11 + R13

## M4 (Weeks 7-8) — Scale and polish
- WS4: R6 seek indexing
- WS6: R18 + R14
- WS7: full regression + perf gates

---

## 6) Rollout strategy

1. Add feature flags for high-risk behavior:
   - `PLAYBACK_SESSION_V2`
   - `ABR_RUNTIME_V2`
   - `AUTH_HOTPATH_NO_DB`
   - `HLS_BOUNDED_PLAYLIST`
   - `SEEK_INDEX_V1`
2. Canary rollout:
   - 5% internal users -> 25% -> 100%
3. Automatic rollback triggers:
   - TTFF regression > 15%
   - rebuffer ratio > 2x baseline
   - seek failure > 1%

---

## 7) Platform validation matrix (required before full release)

- Web (Chromium + Firefox)
- Safari macOS
- Safari iOS
- Android (Chrome + WebView if app shell planned)
- tvOS/WebKit player path
- Roku playback target profile

For each platform validate:
- startup TTFF
- seek behavior (rapid scrub + long jump)
- ABR switching
- subtitle/audio track switching
- session cleanup on close/pause

---

## 8) Immediate execution backlog (next 10 business days)

1. Implement metrics + benchmark harness (WS0).
2. Patch stderr draining in transcode paths (R5).
3. Remove DB lookup from segment auth path behind feature flag (R4).
4. Replace try_acquire fail-fast with queued admission timeout (R17).
5. Switch HLS bounded playlist policy + reduce segment duration profile (R3, R12).
6. Add first integration tests: concurrent playback isolation + seek regressions (R15).

---

## 9) Definition of done for this plan

This plan is complete when:

1. Every review suggestion R1-R18 has shipped and is verified against an acceptance check.
2. Architecture goals A1-A5 are visible in production metrics.
3. Playback SLOs in Section 2 are met for the supported client matrix.
