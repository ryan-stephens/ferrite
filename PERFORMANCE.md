# Ferrite Performance Review & Optimization Tracker

> **Last updated:** 2026-02-19
> **Goal:** Achieve a highly performant application, exceeding Plex's performance across all core features.

---

## Plex Comparison Summary

| Feature | Plex | Ferrite | Winner |
|---|---|---|---|
| Direct Play latency | ~200ms | ~100ms (no transcoder overhead) | **Ferrite** |
| HLS session startup | 3-5s (all variants) | <1s (single variant) | **Ferrite** |
| Seek within buffer | Always restarts FFmpeg | Reuses session (0-1ms) | **Ferrite** |
| Seek to new position | 2-5s | 1-3s (frame-accurate) | **Ferrite** |
| HW accel support | NVENC, QSV, VAAPI, VideoToolbox | NVENC, QSV, VAAPI | Plex |
| Audio passthrough | Yes (surround) | Yes (surround) | Tie |
| HDR tone-mapping | zscale + tonemap | zscale + tonemap (Hable) | Tie |
| Concurrent transcodes | Semaphore + queue | Semaphore + reject (503) | Plex |
| DB performance | PostgreSQL/SQLite | SQLite (well-tuned) | Plex |
| Library scan speed | Batched inserts | Sequential inserts | **Plex** |
| Memory efficiency | Streaming segments | Full segment buffering | **Plex** |

---

## Existing Strengths (No Action Needed)

These are areas where Ferrite is already at or above Plex's level:

- [x] **Streaming strategy selection** — `compat::determine_strategy()` correctly prioritizes DirectPlay → Remux → AudioTranscode → FullTranscode
- [x] **Frame-accurate HLS seeking** — Dual `-ss` approach (fast demuxer seek before `-i`, precise trim after `-i`)
- [x] **HLS session reuse** — `buffered_secs()` check returns `reused: true` for seeks within already-transcoded content (0-1ms vs Plex's 2-5s)
- [x] **Single-variant fast startup** — `create_single_variant_session()` spawns 1 FFmpeg process instead of N for ABR
- [x] **SQLite configuration** — WAL mode, `synchronous=NORMAL`, 20MB page cache, `temp_store=MEMORY`, 5s busy timeout
- [x] **Hardware acceleration** — Auto-detection with priority (NVENC > QSV > VAAPI > Software), NVENC `p4` preset with low-latency tuning
- [x] **Audio passthrough in HLS** — AAC/MP3/Opus/FLAC/ALAC correctly identified as browser-compatible
- [x] **Seek debouncing** — 400ms debounce in `seekRelative()` prevents rapid +10s presses from spawning multiple FFmpeg processes
- [x] **Transcode semaphore** — `try_acquire()` (non-blocking) rejects excess requests with 503 instead of queuing

---

## Optimization Items

### 🔴 Critical

#### 1. Batch scanner DB inserts in a single transaction
- **Status:** `[ ]` Not started
- **Location:** `crates/ferrite-scanner/src/lib.rs` — `scan_library()`, lines 186-347
- **Problem:** After concurrent ffprobe completes, the scanner inserts results one file at a time in a sequential loop. Each insert is a separate SQLite transaction (auto-commit). For a 1000-file library this means thousands of individual transactions.
- **Fix:** Wrap the entire insert loop in a single transaction:
  ```rust
  let mut tx = pool.begin().await?;
  for scanned in &scanned_files {
      // all inserts use &mut tx instead of pool
  }
  tx.commit().await?;
  ```
- **Impact:** 10-50x faster library scanning for large libraries. A 5000-file library that takes 30s to insert would take 1-3s.

#### 2. Combine 4 sequential video metadata queries into 1
- **Status:** `[ ]` Not started
- **Location:** `crates/ferrite-api/src/handlers/stream.rs` — `hls_master_playlist()` and `hls_seek()`
- **Problem:** Both handlers make 4 sequential DB queries before spawning FFmpeg:
  1. `get_media_item()` — media metadata
  2. `get_video_pixel_format()` — pixel format
  3. `get_video_frame_rate()` — frame rate
  4. `get_video_color_metadata()` — color space/transfer/primaries

  All hit the same `media_streams` table for the same `media_item_id`. Each query adds ~0.5-1ms latency.
- **Fix:** Create a single `get_video_metadata()` function in `stream_repo.rs`:
  ```sql
  SELECT pixel_format, frame_rate, color_space, color_transfer, color_primaries
  FROM media_streams
  WHERE media_item_id = ? AND stream_type = 'video'
  ORDER BY stream_index LIMIT 1
  ```
- **Impact:** ~3ms saved per HLS session creation/seek. Compounds with concurrent users.

---

### 🟡 High

#### 3. Add `mmap_size` pragma for SQLite
- **Status:** `[ ]` Not started
- **Location:** `crates/ferrite-db/src/lib.rs` — `create_pool()`
- **Problem:** Missing `mmap_size` pragma which enables memory-mapped I/O for reads. This is one of the biggest SQLite performance wins for read-heavy workloads.
- **Fix:** Add to connection options:
  ```rust
  .pragma("mmap_size", "268435456")  // 256MB memory-mapped I/O
  ```
- **Impact:** 10-30% faster reads for large databases by avoiding `read()` syscalls.

#### 4. Stream HLS segments instead of buffering in memory
- **Status:** `[ ]` Not started
- **Location:** `crates/ferrite-stream/src/hls.rs` — `get_segment()`
- **Problem:** HLS segments (2-6MB each) are read entirely into a `Vec<u8>` before sending to the client. This means full memory allocation per segment and no streaming — client waits for full read before receiving first byte.
- **Fix:** Use `tokio::fs::File` + `ReaderStream` to stream segments directly from disk to the HTTP response without buffering the entire file in memory.
- **Impact:** Lower memory usage (especially with concurrent viewers), faster time-to-first-byte for segments.

#### 5. Add missing `playback_progress` index
- **Status:** `[ ]` Not started
- **Location:** New migration file
- **Problem:** `get_recently_played()` filters by `user_id` and orders by `last_played_at DESC`, but there's no composite index for this query pattern.
- **Fix:** Add migration:
  ```sql
  CREATE INDEX IF NOT EXISTS idx_progress_user_last_played
      ON playback_progress(user_id, last_played_at DESC);
  ```
- **Impact:** Faster "Continue Watching" queries, especially as progress table grows.

#### 6. Optimize `upsert_tv_show()` with normalized_title column
- **Status:** `[ ]` Not started
- **Location:** `crates/ferrite-db/src/tv_repo.rs` — `upsert_tv_show()`, lines 48-99
- **Problem:** When no exact title match is found, it loads **all shows in the library** into memory and does string normalization in Rust. For a library with 500 TV shows, this is 500 rows loaded + 500 string normalizations per new file.
- **Fix:** Add a `normalized_title` column to `tv_shows` and index it. Compute the normalized form once during insert, then query directly:
  ```sql
  SELECT id FROM tv_shows WHERE library_id = ? AND normalized_title = ?
  ```
- **Impact:** O(1) lookup instead of O(N). Matters for large TV libraries during scanning.

#### 7. Increase HLS.js buffer sizes and enable Web Worker
- **Status:** `[ ]` Not started
- **Location:** `ferrite-ui/src/components/Player.tsx` — `HLS_CONFIG`, lines 50-59
- **Problem:** Current buffer config is conservative for a local/LAN server.
- **Fix:**
  ```typescript
  const HLS_CONFIG = {
    maxBufferLength: 60,        // Buffer 60s ahead (vs 30s)
    maxMaxBufferLength: 120,    // Allow up to 120s
    backBufferLength: 30,       // Keep 30s of back-buffer for instant rewind
    maxBufferSize: 60 * 1000 * 1000,
    maxBufferHole: 0.5,
    lowLatencyMode: false,
    startFragPrefetch: true,
    testBandwidth: false,
    abrEwmaDefaultEstimate: 10_000_000,
    enableWorker: true,         // Offload demuxing to Web Worker
  };
  ```
- **Impact:** Fewer rebuffering events, instant backward seeks within back-buffer.

---

### 🟡 Medium

#### 8. Increase progress report interval to 30s
- **Status:** `[ ]` Not started
- **Location:** `ferrite-ui/src/components/Player.tsx` — `onTimeUpdate()`, line 539
- **Problem:** Progress is reported every 10s. Each report is an HTTP POST + DB write. For 10 concurrent viewers, that's 60 writes/minute. Plex uses 30s.
- **Fix:** Change `10000` to `30000`:
  ```typescript
  if (now - lastProgressReport > 30000) {
  ```
- **Impact:** 3x fewer DB writes for progress tracking. `handleClose()` already reports the final position, so no data is lost.

#### 9. Fix dynamic SQL in `list_movies_with_media()` for prepared statement caching
- **Status:** `[ ]` Not started
- **Location:** `crates/ferrite-db/src/movie_repo.rs` — `list_movies_with_media()`, lines 199-243
- **Problem:** The query is built dynamically with `format!()`, which means sqlx can't cache the prepared statement. Each call re-prepares the SQL.
- **Fix:** Use a fixed set of query variants (with/without library_id, with/without search, etc.) as static strings, or use sqlx's `QueryBuilder`.
- **Impact:** Minor per-query (~0.1ms), but this is the main listing endpoint called on every page load.

---

### 🟢 Low

#### 10. Avoid vector cloning in `video_encode_args()`
- **Status:** `[ ]` Not started
- **Location:** `crates/ferrite-transcode/src/hwaccel.rs` — `video_encode_args()`, lines 120-124
- **Problem:** Allocates and clones vectors on every call. Encoder profiles are immutable after creation.
- **Fix:** Pre-compute the args once and return `&[String]` slices, or use `Cow<[String]>`.
- **Impact:** Negligible — called once per FFmpeg spawn. Code quality improvement.

#### 11. Deduplicate WHERE binding logic in `count_movies_with_media()`
- **Status:** `[ ]` Not started
- **Location:** `crates/ferrite-db/src/movie_repo.rs` — `count_movies_with_media()`, lines 247-273
- **Problem:** The count query manually re-binds parameters instead of reusing `bind_where_params()`. Maintenance risk.
- **Fix:** Refactor to share binding logic with `list_movies_with_media()`.
- **Impact:** Code quality only, no runtime performance change.

---

## Future Considerations

These are not bugs or issues, but potential enhancements for even higher performance:

- **VideoToolbox support (macOS)** — Plex supports this; would help Mac-based servers
- **Transcode queuing** — Currently rejects with 503 when semaphore is full; could queue requests with a timeout instead
- **Response caching** — Cache media item metadata responses (ETag/Last-Modified) to avoid DB hits for repeated requests
- **Thumbnail generation pipeline** — Pre-generate seek thumbnails (BIF/WebVTT) for instant visual seeking
- **Connection pooling tuning** — Current default is 16 connections; profile under load to find optimal value
- **SQLite read replicas** — For very high read concurrency, consider opening separate read-only connections that bypass the WAL writer lock
