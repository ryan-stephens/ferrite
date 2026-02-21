# Weekly Playback Performance Snapshot

_Date:_ YYYY-MM-DD  
_Environment:_ (host class, CPU/GPU profile, dataset)

## Benchmark run inputs

- iterations: N
- concurrency: N
- seek target: N seconds
- media IDs:
  - direct:
  - remux:
  - audio_transcode:
  - full_transcode:
  - hls:

## Client-observed latency summary (from benchmark JSON)

| Scenario | P50 (ms) | P95 (ms) | Max (ms) | Notes |
|---|---:|---:|---:|---|
| direct |  |  |  |  |
| remux |  |  |  |  |
| audio_transcode |  |  |  |  |
| full_transcode |  |  |  |  |
| hls_startup |  |  |  |  |
| hls_seek |  |  |  |  |

## Backend metric snapshot highlights

| Metric | Key label set | Avg (ms) / Count | Last | Comment |
|---|---|---|---|---|
| playback_ttff_ms | path=stream,stream=direct |  |  |  |
| playback_ttff_ms | path=stream,stream=remux |  |  |  |
| playback_ttff_ms | path=stream,stream=audio-transcode |  |  |  |
| playback_ttff_ms | path=stream,stream=full-transcode |  |  |  |
| playback_ttff_ms | path=hls_master,stream=hls |  |  |  |
| seek_latency_ms | path=hls_seek,mode=reused |  |  |  |
| seek_latency_ms | path=hls_seek,mode=new-session |  |  |  |
| auth_hotpath_ms | path=stream,method=bearer,outcome=ok |  |  |  |
| transcode_queue_wait_ms | operation=hls-master |  |  |  |
| hls_segment_wait_ms | path=hls_segment |  |  |  |
| rebuffer_count | path=player,stream=hls |  |  |  |
| rebuffer_ms | path=player,stream=hls |  |  |  |

## Week-over-week trend

- TTFF trend:
- Seek trend:
- Rebuffer trend:
- Queue wait trend:

## Regressions / action items

1.
2.
3.
