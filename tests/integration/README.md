# Playback-Critical Integration Suite (WS7)

This directory holds canonical playback regression scenarios from the WS7 implementation plan.

## Scenarios

The suite covers these high-risk flows:

1. `same-media-isolation`
   - Concurrent same-media playback isolation.
   - Backed by `concurrent_media_sessions_are_isolated` in `crates/ferrite-stream/tests/hls_session_integration.rs`.
2. `abr-switch-under-throttle`
   - ABR variant downgrade selection under constrained throughput.
   - Backed by `abr_master_playlist_supports_variant_downgrade_under_throttle`.
3. `seek-reuse-recreate`
   - Seek within-buffer session reuse and far-seek session recreation.
   - Backed by `get_or_create_session_reuses_nearby_start_and_recreates_far_start`.
4. `progress-isolation-by-user`
   - Per-user playback progress isolation.
   - Backed by `crates/ferrite-db/tests/playback_progress_user_isolation.rs`.

## Run static suite

From repo root:

```powershell
node tests/integration/run-playback-critical.mjs
```

Run a subset:

```powershell
node tests/integration/run-playback-critical.mjs `
  --cases same-media-isolation,seek-reuse-recreate
```

## Run live auth hot-path matrix

This suite can optionally drive live auth hot-path benchmark runs and threshold checks.

```powershell
node tests/integration/run-playback-critical.mjs `
  --include-live-auth true `
  --base-url http://127.0.0.1:8080 `
  --token <ADMIN_JWT> `
  --media-id <MEDIA_ID> `
  --api-key <API_KEY> `
  --iterations 20 `
  --concurrency 16
```

With `--api-key` provided, it runs all four auth modes:

- `bearer`
- `token-query`
- `api-key-header`
- `api-key-query`

Without `--api-key`, only bearer and token-query modes are executed.
