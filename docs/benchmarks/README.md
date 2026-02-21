# Playback Benchmark Harness (WS0)

This directory contains the baseline playback benchmark workflow introduced for WS0.

## 1) Prerequisites

- Ferrite server running and reachable.
- Admin JWT token (needed for `/api/system/metrics` reset/read endpoints).
- Media IDs representing each playback mode:
  - direct play source (`--direct-id`)
  - remux source (`--remux-id`)
  - audio-transcode source (`--audio-id`)
  - full-transcode source (`--full-id`)
  - HLS source (`--hls-id`)

## 2) Run benchmark

From repo root:

```powershell
node scripts/playback-benchmark.mjs `
  --base-url http://127.0.0.1:8080 `
  --token <JWT_TOKEN> `
  --direct-id <MEDIA_ID_DIRECT> `
  --remux-id <MEDIA_ID_REMUX> `
  --audio-id <MEDIA_ID_AUDIO> `
  --full-id <MEDIA_ID_FULL> `
  --hls-id <MEDIA_ID_HLS> `
  --iterations 3 `
  --concurrency 1 `
  --abr-throttle-kbps 300 `
  --seek-secs 600 `
  --out docs/benchmarks/playback-baseline-<date>.json
```

For an N-user stress scenario, increase `--concurrency` (e.g. `--concurrency 8`).

## 3) Output

The script writes a JSON file containing:

- client-observed startup/seek/ABR-switch latency summaries per scenario
- backend in-memory metric snapshot (`/api/system/metrics`) after the run

`hls_abr_switch` simulates constrained bandwidth and verifies selected HLS
variant bandwidth drops under throttling pressure.

See `playback-baseline-template.json` for output schema.

## 4) Suggested baseline cadence

1. Run single-user baseline (`concurrency=1`) weekly.
2. Run burst baseline (`concurrency=4` or `8`) weekly.
3. Commit output JSON snapshots under `docs/benchmarks/`.
4. Copy key numbers into `weekly-snapshot-template.md` for trend tracking.

## 5) Dashboard / query templates (WS0)

Use `dashboard-query-template.md` for copy-paste query snippets when building
weekly playback dashboards from benchmark snapshots.

## 6) Perf regression threshold gate (WS7)

Use `docs/benchmarks/perf-thresholds.json` as the shared threshold contract and
validate a benchmark snapshot with:

```powershell
node scripts/check-playback-thresholds.mjs `
  --benchmark docs/benchmarks/playback-baseline-<date>.json `
  --thresholds docs/benchmarks/perf-thresholds.json
```

The check fails when any configured scenario/metric breaches its constraints.
This gate is also wired into CI via `.github/workflows/perf-gate.yml`.

## 7) Auth hot-path load harness (WS7)

Use this to stress `/api/stream/...` auth paths and verify `auth_hotpath_ms` behavior:

```powershell
node scripts/auth-hotpath-load.mjs `
  --base-url http://127.0.0.1:8080 `
  --token <ADMIN_JWT> `
  --media-id <MEDIA_ID> `
  --auth-mode bearer `
  --iterations 20 `
  --concurrency 16 `
  --out docs/benchmarks/auth-hotpath-bearer-<date>.json
```

Supported `--auth-mode` values:

- `bearer`
- `token-query`
- `api-key-header` (requires `--api-key`)
- `api-key-query` (requires `--api-key`)

Command matrix for initial baseline snapshots:

```powershell
# bearer
node scripts/auth-hotpath-load.mjs `
  --base-url http://127.0.0.1:8080 `
  --token <ADMIN_JWT> `
  --media-id <MEDIA_ID> `
  --auth-mode bearer `
  --iterations 20 `
  --concurrency 16 `
  --out docs/benchmarks/auth-hotpath-baseline-bearer-<date>.json

# token query
node scripts/auth-hotpath-load.mjs `
  --base-url http://127.0.0.1:8080 `
  --token <ADMIN_JWT> `
  --media-id <MEDIA_ID> `
  --auth-mode token-query `
  --iterations 20 `
  --concurrency 16 `
  --out docs/benchmarks/auth-hotpath-baseline-token-query-<date>.json

# api key header
node scripts/auth-hotpath-load.mjs `
  --base-url http://127.0.0.1:8080 `
  --token <ADMIN_JWT> `
  --media-id <MEDIA_ID> `
  --auth-mode api-key-header `
  --api-key <API_KEY> `
  --iterations 20 `
  --concurrency 16 `
  --out docs/benchmarks/auth-hotpath-baseline-api-key-header-<date>.json

# api key query
node scripts/auth-hotpath-load.mjs `
  --base-url http://127.0.0.1:8080 `
  --token <ADMIN_JWT> `
  --media-id <MEDIA_ID> `
  --auth-mode api-key-query `
  --api-key <API_KEY> `
  --iterations 20 `
  --concurrency 16 `
  --out docs/benchmarks/auth-hotpath-baseline-api-key-query-<date>.json
```

Or run all canonical WS7 integration scenarios plus the auth matrix with:

```powershell
node tests/integration/run-playback-critical.mjs `
  --include-live-auth true `
  --base-url http://127.0.0.1:8080 `
  --token <ADMIN_JWT> `
  --media-id <MEDIA_ID> `
  --api-key <API_KEY>
```

The output includes a client-side latency summary and the backend
`auth_hotpath_ms` timing series from `/api/system/metrics`.

## 8) Auth hot-path threshold gate (WS7)

Use `docs/benchmarks/auth-hotpath-thresholds.json` to validate auth hot-path
benchmark output:

```powershell
node scripts/check-auth-hotpath-thresholds.mjs `
  --benchmark docs/benchmarks/auth-hotpath-baseline-<date>.json `
  --thresholds docs/benchmarks/auth-hotpath-thresholds.json
```

The auth gate validates:

- client latency summary fields under `auth_hotpath`
- required backend `auth_hotpath_ms` timing rows under `backend_metrics`

See `auth-hotpath-baseline-template.json` for the expected output schema.
CI wiring lives in `.github/workflows/auth-hotpath-gate.yml`.
