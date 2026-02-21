# Playback Dashboard / Query Template

Use these snippets to build weekly trend dashboards from benchmark snapshots under `docs/benchmarks/`.

## 1) Select newest playback benchmark snapshot

```powershell
$latest = Get-ChildItem docs/benchmarks/playback-baseline-*.json |
  Where-Object { $_.Name -notmatch 'template' } |
  Sort-Object LastWriteTime |
  Select-Object -Last 1
$latest.FullName
```

## 2) Scenario latency rows (for weekly table)

```powershell
Get-Content $latest.FullName | node -e "const fs=require('fs');const d=JSON.parse(fs.readFileSync(0,'utf8'));for(const [name,row] of Object.entries(d.scenarios||{})){console.log(`${name},p50=${row.p50_ms},p95=${row.p95_ms},max=${row.max_ms},count=${row.count}`)}"
```

## 3) Backend timing metrics by name

```powershell
Get-Content $latest.FullName | node -e "const fs=require('fs');const d=JSON.parse(fs.readFileSync(0,'utf8'));for(const row of d.backend_metrics?.timings||[]){console.log(`${row.name},avg=${row.avg_ms},p95=${row.p95_ms},count=${row.count}`)}"
```

## 4) Backend counter metrics by name

```powershell
Get-Content $latest.FullName | node -e "const fs=require('fs');const d=JSON.parse(fs.readFileSync(0,'utf8'));for(const row of d.backend_metrics?.counters||[]){console.log(`${row.name},value=${row.value}`)}"
```

## 5) Suggested dashboard sections

1. TTFF: `direct`, `remux`, `audio_transcode`, `full_transcode`, `hls_startup`
2. Seek: `hls_seek` latency + `seek_latency_ms` backend series
3. ABR: `hls_abr_switch` decision latency + switch rate
4. Queue pressure: `transcode_queue_wait_ms`
5. Stability: `rebuffer_count`, `rebuffer_ms`, `hls_segment_wait_ms`, `auth_hotpath_ms`
