# Ferrite Comprehensive Codebase Review (Temp)

_Date:_ 2026-02-20

## Scope

This review covered backend (Rust workspace), database/query layer, scanner/enrichment, streaming/transcode path, and frontend (SolidJS) with a focus on your stated goal: **be a more performant alternative to Plex** while preserving reliability and UX iteration velocity.

---

## Executive Summary

Ferrite has a strong technical foundation (Rust + SQLite WAL + FFmpeg orchestration + clear crate separation), and many good performance-oriented decisions are already in place.

The largest remaining risks are mostly **scale and UX consistency issues**, not architectural blockers:

1. **Frontend data-loading pattern will bottleneck first** on larger libraries (global load + client-side sort/filter + hard page cap).
2. **Auto-rescan currently skips metadata enrichment**, so TV/movie metadata freshness can drift over time.
3. **Scan status semantics are slightly misleading** in edge cases due detached enrichment tasks and percent calculation strategy.
4. **Metadata provider request construction is fragile** (manual URL concatenation, no encoded query params).
5. **Search/filter SQL path is not index-friendly for large libraries** (`LIKE '%...%'`), so query latency will climb as data grows.

---

## What’s Working Well (Keep)

- Strong modular architecture across crates and clear ownership boundaries (`ferrite-server`, `ferrite-api`, `ferrite-scanner`, `ferrite-stream`, etc.).
- Good SQLite tuning for WAL workloads (`synchronous=NORMAL`, `busy_timeout`, mmap, cache sizing).
- Practical HLS session lifecycle controls: idle FFmpeg kill + cleanup loop + graceful shutdown.
- Good streaming strategy split (direct/remux/audio/full) and thoughtful seek logic around keyframes.
- Useful scan progress model and scan de-duplication guard for same library.

References:
- `crates/ferrite-db/src/lib.rs`
- `crates/ferrite-stream/src/hls.rs`
- `crates/ferrite-api/src/handlers/library.rs`
- `crates/ferrite-api/src/handlers/stream.rs`

---

## Findings (Prioritized)

## High Priority

### 1) Frontend only requests first 500 media items (functional + performance issue)

- API client hardcodes `per_page=500` and does not auto-paginate (`ferrite-ui/src/api.ts`, `listMedia`).
- Multiple pages rely on the global `allMedia()` store and then perform client-side sorting/filtering.
- For large libraries, users will see incomplete catalogs and increasingly expensive client-side operations.

References:
- `ferrite-ui/src/api.ts` (listMedia default params)
- `ferrite-ui/src/stores/media.ts` (global all-media loading)
- `ferrite-ui/src/pages/HomePage.tsx` (client-side sort/filter slices)
- `ferrite-ui/src/pages/LibraryPage.tsx` (client-side sort/filter)

Recommendation:
- Move to **server-driven pagination/filter/sort** per page/view.
- Keep `allMedia` only for narrowly scoped features (e.g., lightweight search cache), not primary rendering.

---

### 2) Watcher-triggered rescans do not run metadata enrichment

- `LibraryWatcher` calls `scan_library(..., None, None)` for provider/cache.
- This means auto-rescan discovers files but does not enrich TMDB metadata for those runs.

Reference:
- `crates/ferrite-scanner/src/watcher.rs` (rescan call passing `None, None`)

Recommendation:
- Build provider/cache in watcher path (same logic used by manual scan endpoint), or queue enrichment as a separate background job.

---

### 3) Scan lifecycle/status can report “complete” while detached enrichment tasks may still be running

- Inline enrichment tasks are spawned detached during phase-1 indexing.
- Status is set to `Complete` before/around later phases and can oscillate between stages, depending on detached task timing.

References:
- `crates/ferrite-scanner/src/lib.rs` (detached `tokio::spawn` enrichment tasks)
- `crates/ferrite-scanner/src/lib.rs` (multiple `set_status(Complete)` transitions)

Recommendation:
- Track detached enrichment task handles and await/join before final terminal `Complete`, or explicitly model as separate async phase in progress schema.

---

### 4) TMDB requests are manually URL-constructed without query encoding

- Query strings include raw title interpolation (`query={title}`).
- Titles with symbols/Unicode/edge punctuation can degrade match quality or break requests.

References:
- `crates/ferrite-metadata/src/tmdb.rs` (`search_movie`, `search_tv` URL formatting)

Recommendation:
- Use `reqwest` URL/query builder (`.query(&[("query", title), ...])`) instead of string concatenation.

---

## Medium Priority

### 5) “Refresh All” UX state is timer-based, not actual scan-state-based

- Global scanning indicator resets after 3 seconds regardless of real scan progress.
- Settings page has real polling via `scanStatus`, but global store uses optimistic timeout.

References:
- `ferrite-ui/src/stores/media.ts` (`refreshAll`)
- `ferrite-ui/src/pages/SettingsPage.tsx` (actual scan polling implementation)

Recommendation:
- Consolidate on one truth source: use scan-status polling and derive status globally.

---

### 6) Scan percent is based only on `files_probed`, not full lifecycle

- During `enriching` and `subtitles`, percent can stay at 100 while still actively processing.

Reference:
- `crates/ferrite-scanner/src/progress.rs` (`percent` calculation)

Recommendation:
- Either:
  - add phase-specific progress fields, or
  - present phase-aware percent (`probe`, `enrich`, `subtitle`) instead of a single scalar.

---

### 7) Query strategy for title/genre search will slow with growth

- `LIKE '%...%'` on joined fields is not index-friendly.
- Acceptable now, but this path will become a noticeable bottleneck on larger datasets.

Reference:
- `crates/ferrite-db/src/movie_repo.rs` (`list_movies_with_media`, `count_movies_with_media` WHERE filters)

Recommendation:
- Add FTS5 virtual table for title/overview/genres search (or tokenized search table + triggers).

---

### 8) “Other” library type silently maps to Movie backend type

- UI offers `other`, backend default branch maps unknown types to `Movie`.
- This can create hidden misconfiguration and confusing behavior.

References:
- `ferrite-ui/src/pages/SettingsPage.tsx` (library type options)
- `crates/ferrite-api/src/handlers/library.rs` (library_type mapping)

Recommendation:
- Remove `other` from UI or add explicit backend support/validation error.

---

## Lower Priority / Hardening

### 9) Metadata image cache lacks resilience controls

- No explicit timeout/retry/backoff/circuit behavior on image downloads.
- Transient CDN/API errors can reduce metadata completeness over time.

Reference:
- `crates/ferrite-metadata/src/image_cache.rs`

Recommendation:
- Add short timeout, bounded retries with jitter, and lightweight error counters/metrics.

---

### 10) Some expensive list/count patterns in TV queries may need optimization later

- Correlated count subqueries are fine now, but could be replaced with pre-aggregated counts or indexed/materialized strategy when library scale grows.

Reference:
- `crates/ferrite-db/src/tv_repo.rs` (`list_shows`, `get_show`)

---

## Performance Strategy Recommendations (Roadmap)

## Phase 1 (Immediate, highest ROI)

1. Server-side pagination/filter/sort end-to-end for all list pages.
2. Unify scan status model in frontend and remove timer-based faux completion.
3. Fix watcher rescans to include enrichment path.
4. Move TMDB URL generation to encoded query builder.

## Phase 2 (Scale preparedness)

1. Introduce FTS5-backed search.
2. Add lightweight telemetry around:
   - scan duration per phase,
   - enrichment success/failure rates,
   - stream startup latency (TTFF),
   - HLS session churn.
3. Add API-level caching hints for frequently requested list endpoints.

## Phase 3 (Advanced)

1. Background job queue abstraction for enrichment/image fetch/retry.
2. Optional Redis/memory cache layer for hot metadata and search responses.
3. Benchmark suite for large synthetic libraries (10k, 50k, 100k items).

---

## Suggested Test Coverage Additions

1. **Watcher integration test**: file add/change triggers scan + enrichment.
2. **Pagination contract tests**: frontend retrieves full result set across pages.
3. **Scan status correctness tests**: transitions for scanning/enriching/subtitles/complete.
4. **TMDB query encoding tests**: punctuation/unicode-heavy titles.
5. **Large dataset query perf smoke test**: regression guard for list/search latency.

---

## Closing Notes

You are already far ahead of most self-hosted media projects in architecture quality and runtime fundamentals. The biggest win now is reducing **data-plane friction at scale** (querying, pagination, enrichment consistency, and state accuracy), which directly supports your “faster-than-Plex” goal in real-world libraries.
