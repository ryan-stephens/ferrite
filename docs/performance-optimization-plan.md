# Ferrite Performance Optimization & Architecture Plan

## 1. Overview
The goal of Ferrite is to be a highly performant, self-hosted alternative to Plex. The core architecture uses a Rust backend (tokio, axum, sqlx) with a SQLite database, and a React frontend.

While SQLite is the correct choice for a self-hosted media server (as it requires zero external dependencies and is used by Plex, Jellyfin, and Emby), it introduces specific concurrency challenges. SQLite supports concurrent readers via Write-Ahead Logging (WAL), but only allows **a single concurrent writer**. 

Current benchmarks and logs (notably `014_performance_indexes.sql`) reveal that during heavy library scans or metadata enrichment, write locks are held for extended periods. This causes read queries to queue up and block, resulting in latency spikes (1-5 seconds) on critical paths like authentication and media list fetching.

This document outlines the identified bottlenecks and the implementation plan to resolve them, ensuring Ferrite can scale to handle high-concurrency streaming and background processing simultaneously.

---

## 2. Identified Bottlenecks

### A. The Authentication Hot-Path (Highest Priority)
**The Problem:**
Every API request, including streaming media segments (e.g., HLS `ts` files), requires authentication. Currently, the JWT/token validation middleware performs a database lookup (`SELECT * FROM users WHERE id = ?`). When a library scan is active and holding a write lock, this simple lookup takes 1-5 seconds. Under high concurrent playback, this will collapse the streaming pipeline.

**The Solution:**
The auth hot-path must be zero-I/O.
- Implement an in-memory cache (e.g., using `DashMap`) in `ferrite-api` for user sessions, permissions, and token revocation lists.
- Load the cache on startup and update it via events when a user is modified in the database.
- Alternatively, transition to fully stateless JWTs where the signature and expiration are sufficient for validation, falling back to the DB only for critical mutations.

### B. SQLite Concurrency & Connection Management
**The Problem:**
Using a single connection pool for both reads and writes leads to lock contention. If multiple threads attempt to write simultaneously, SQLite returns `database is locked` or blocks until the `busy_timeout` is reached.

**The Solution:**
- **Split Connection Pools:** Create two distinct `sqlx` connection pools:
  1. A **Reader Pool**: Configured with multiple connections (e.g., 10-20) for handling `SELECT` queries concurrently.
  2. A **Writer Pool**: Configured with exactly 1 connection. This serializes all writes at the application level, preventing SQLx threads from fighting over the SQLite lock and eliminating `database is locked` errors.
- **Aggressive Pragmas:** Ensure the following are set on initialization:
  - `PRAGMA journal_mode = WAL;` (Enables concurrent readers while writing)
  - `PRAGMA synchronous = NORMAL;` (Safe with WAL, significant speedup)
  - `PRAGMA busy_timeout = 5000;`
  - `PRAGMA temp_store = MEMORY;`
  - `PRAGMA mmap_size = 30000000000;`

### C. Library Scanning & Write Transactions
**The Problem:**
The `ferrite-scanner` crate processes thousands of files. If it inserts media items row-by-row, the overhead of individual transactions is massive. If it opens one massive transaction for the entire scan, it blocks all other writes and can starve readers.

**The Solution:**
- **Write Batching:** Accumulate metadata and media items in memory and insert them in batches (e.g., 500 records per transaction).
- **Yielding:** After committing a batch, the scanner thread must explicitly yield (`tokio::task::yield_now()`) or sleep briefly. This allows the single writer connection to be acquired by other tasks (e.g., user preferences updates, playback progress tracking) and lets queued readers slip through.
- **Fast-Path Probing:** Avoid running `ffprobe` or hashing files if the `mtime` and file size haven't changed since the last scan.

### D. Subtitle Extraction I/O Thrashing
**The Problem:**
In `ferrite-scanner`, "Phase 3" eagerly extracts embedded subtitles from every new media file using FFmpeg. The concurrency for this is currently set to `concurrent_probes * 2` (e.g., if you have 4 probe threads, it runs 8 concurrent FFmpeg extractions). 
Running 8 concurrent FFmpeg processes to demux subtitles from massive MKV files on a mechanical hard drive (or even a standard SSD) causes severe **I/O thrashing**. The disk head has to jump between 8 different massive files simultaneously, destroying sequential read speeds and making the extraction phase agonizingly slow.

**The Solution:**
- **Dedicated I/O Concurrency Limit:** Subtitle extraction must be bottlenecked by a separate, much stricter semaphore (e.g., 1 or 2 concurrent extractions max) to maintain sequential disk read performance.
- **Lazy / On-Demand Extraction (Ideal):** Instead of eagerly extracting all subtitles for the entire library during the scan, defer extraction until a user actually selects that subtitle track during playback. The DB can store the *metadata* that the stream exists (from the initial fast probe), but the actual `.srt`/`.vtt` file shouldn't be extracted via FFmpeg until requested.

### E. Playback & Streaming I/O
**The Problem:**
Serving large media files or transcoding streams requires efficient memory and socket handling. Inefficient buffering can lead to high memory usage and latency, especially during seeking or ABR (Adaptive Bitrate) switches.

**The Solution:**
- **Zero-Copy Direct Play:** Ensure `ferrite-stream` utilizes zero-copy techniques (like `sendfile`) for Direct Play. `tower-http`'s `ServeFile` or optimized `tokio::fs::File` streams should be used to offload data transfer to the OS kernel.
- **Transcoder Buffer Management:** When acting as a middleman for transcoded output (e.g., FFmpeg stdout), ensure buffers are efficiently sized and reused to minimize allocations.

---

## 3. Implementation Plan

### Phase 1: Zero-I/O Authentication
1. Refactor `ferrite-api/src/auth.rs` to introduce an in-memory `DashMap` for valid users and API keys.
2. Initialize this cache on application startup by querying the `ferrite-db`.
3. Update the auth middleware to check the cache instead of the database.
4. Add mechanisms to invalidate or update the cache when a user is created, updated, or deleted.
5. Run the `auth-hotpath-load.mjs` benchmark to verify the latency drop.

### Phase 2: SQLite Connection Splitting & Tuning
1. Modify `ferrite-db/src/lib.rs` (or where the pool is initialized) to return two pools: `ReaderPool` and `WriterPool`.
2. Apply the performance-oriented Pragmas explicitly when opening connections.
3. Update repository methods (`movie_repo.rs`, `media_repo.rs`, etc.) to accept the appropriate pool depending on whether they are reading or mutating data.

### Phase 3: Scanner Batching & Yielding
1. Review `ferrite-scanner/src/lib.rs` and the insertion logic.
2. Implement a batching mechanism for discovered media items.
3. Wrap batch inserts in transactions and implement yielding to release the writer lock.

### Phase 4: Subtitle Extraction Optimization
1. Review `ferrite-scanner/src/lib.rs` and the subtitle extraction loop.
2. Reduce the semaphore concurrency limit from `concurrent_probes * 2` to a strict `1` or `2` to prevent disk thrashing.
3. Investigate transitioning extraction to a fully lazy architecture (where subtitles are only demuxed on-demand when requested by a streaming client).

### Phase 5: Streaming I/O Verification
1. Review `ferrite-stream` for efficient file serving.
2. Ensure integration with `tower-http` is optimal for Direct Play scenarios.
3. Run the playback benchmarks (`playback-baseline.mjs`) to ensure latency thresholds are met under concurrency.
