PS C:\Users\ryans\source\repos\ferrite> cargo run
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.43s
Running `target\debug\ferrite.exe`
2026-02-22T04:45:50.253806Z  INFO ferrite: Loaded config from config/ferrite.toml
2026-02-22T04:45:50.254458Z  INFO ferrite_core::config: Data directory: C:\Users\ryans\source\repos\ferrite
2026-02-22T04:45:50.303959Z  INFO ferrite_transcode::hwaccel: HW encoder detection: nvenc=true, qsv=true, vaapi=true
2026-02-22T04:45:50.304100Z  INFO ferrite_transcode::hwaccel: Auto-selected HW encoder: NVENC
2026-02-22T04:45:50.304345Z  INFO ferrite: Video encoder: h264_nvenc (backend=nvenc)
2026-02-22T04:45:50.312360Z  INFO ferrite_db: Database connected at C:\Users\ryans\source\repos\ferrite\ferrite.db
2026-02-22T04:45:50.312909Z  INFO ferrite_db: Database migrations applied
2026-02-22T04:45:50.515362Z  INFO ferrite_api::router: Serving SPA from: ferrite-ui/dist
2026-02-22T04:45:50.516608Z  INFO ferrite: DLNA server enabled (Ferrite Media Server)
2026-02-22T04:45:50.518380Z  INFO ferrite_scanner::watcher: Watching 1 library directories for changes
2026-02-22T04:45:50.518659Z  INFO ferrite: Filesystem watcher started
2026-02-22T04:45:50.518741Z  INFO ferrite: Ferrite starting on http://0.0.0.0:8080
2026-02-22T04:45:50.518823Z  INFO ferrite: Open http://localhost:8080 in your browser
2026-02-22T04:45:50.519168Z  INFO ferrite_dlna::ssdp: SSDP server listening on 239.255.255.250:1900
2026-02-22T04:46:05.180031Z  INFO http_request{method=GET uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/master.m3u8?playback_session_id=e7437d42%2Db1c9%2D4ae1%2Db3f2%2D27134056d86a}: ferrite_api::handlers::stream: Transcode permit acquired: op=hls-master media_id=5dbce626-0946-435a-aa93-f7837292a783 wait_ms=0.0
2026-02-22T04:46:05.180286Z  INFO http_request{method=GET uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/master.m3u8?playback_session_id=e7437d42%2Db1c9%2D4ae1%2Db3f2%2D27134056d86a}: ferrite_stream::hls: Creating 4 ABR variant sessions for media 5dbce626-0946-435a-aa93-f7837292a783 (source=1920x1080)
2026-02-22T04:46:05.181305Z  INFO http_request{method=GET uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/master.m3u8?playback_session_id=e7437d42%2Db1c9%2D4ae1%2Db3f2%2D27134056d86a}: ferrite_stream::hls: Creating HLS session 1fb90c41-4ce6-4981-9e4c-af27429fbd15 for media 5dbce626-0946-435a-aa93-f7837292a783 at 0.0s variant=1080p (C:\Users\ryans\Movies\Star.Trek.Lower.Decks.2020.S05E05.1080p.HEVC.x265-MeGusta.mkv)
2026-02-22T04:46:05.181536Z  INFO http_request{method=GET uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/master.m3u8?playback_session_id=e7437d42%2Db1c9%2D4ae1%2Db3f2%2D27134056d86a}: ferrite_stream::hls: 10-bit SDR detected (pix=yuv420p10le, transfer=Some("bt709")), applying bit-depth conversion only
2026-02-22T04:46:05.181902Z  INFO http_request{method=GET uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/master.m3u8?playback_session_id=e7437d42%2Db1c9%2D4ae1%2Db3f2%2D27134056d86a}: ferrite_stream::hls: HLS ffmpeg args: ["-hide_banner", "-nostdin", "-i", "C:\\Users\\ryans\\Movies\\Star.Trek.Lower.Decks.2020.S05E05.1080p.HEVC.x265-MeGusta.mkv", "-map", "0:v:0", "-map", "0:a:0", "-vf", "format=yuv420p", "-c:v", "h264_nvenc", "-preset", "p4", "-tune", "ll", "-rc", "vbr", "-cq", "23", "-profile:v", "high", "-level", "4.1", "-g", "48", "-keyint_min", "48", "-c:a", "aac", "-b:a", "192k", "-ac", "2", "-f", "hls", "-hls_time", "2", "-hls_list_size", "30", "-hls_segment_type", "fmp4", "-hls_fmp4_init_filename", "init.mp4", "-hls_segment_filename", "seg_%03d.m4s", "-hls_flags", "independent_segments+delete_segments+temp_file", "playlist.m3u8"]
2026-02-22T04:46:05.200168Z  INFO http_request{method=GET uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/master.m3u8?playback_session_id=e7437d42%2Db1c9%2D4ae1%2Db3f2%2D27134056d86a}: ferrite_stream::hls: Creating HLS session b47461b6-4c7d-47d3-90c1-c0118124723c for media 5dbce626-0946-435a-aa93-f7837292a783 at 0.0s variant=720p (C:\Users\ryans\Movies\Star.Trek.Lower.Decks.2020.S05E05.1080p.HEVC.x265-MeGusta.mkv)
2026-02-22T04:46:05.200668Z  INFO http_request{method=GET uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/master.m3u8?playback_session_id=e7437d42%2Db1c9%2D4ae1%2Db3f2%2D27134056d86a}: ferrite_stream::hls: HLS variant scaling active — falling back to software encoder
2026-02-22T04:46:05.200843Z  INFO http_request{method=GET uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/master.m3u8?playback_session_id=e7437d42%2Db1c9%2D4ae1%2Db3f2%2D27134056d86a}: ferrite_stream::hls: 10-bit SDR detected (pix=yuv420p10le, transfer=Some("bt709")), applying bit-depth conversion only
2026-02-22T04:46:05.200993Z  INFO http_request{method=GET uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/master.m3u8?playback_session_id=e7437d42%2Db1c9%2D4ae1%2Db3f2%2D27134056d86a}: ferrite_stream::hls: HLS ffmpeg args: ["-hide_banner", "-nostdin", "-i", "C:\\Users\\ryans\\Movies\\Star.Trek.Lower.Decks.2020.S05E05.1080p.HEVC.x265-MeGusta.mkv", "-map", "0:v:0", "-map", "0:a:0", "-vf", "format=yuv420p,scale=-2:720", "-c:v", "libx264", "-preset", "veryfast", "-crf", "23", "-profile:v", "high", "-level", "4.1", "-b:v", "2800k", "-maxrate", "4200k", "-bufsize", "5600k", "-g", "48", "-keyint_min", "48", "-c:a", "aac", "-b:a", "128k", "-ac", "2", "-f", "hls", "-hls_time", "2", "-hls_list_size", "30", "-hls_segment_type", "fmp4", "-hls_fmp4_init_filename", "init.mp4", "-hls_segment_filename", "seg_%03d.m4s", "-hls_flags", "independent_segments+delete_segments+temp_file", "playlist.m3u8"]
2026-02-22T04:46:05.219561Z  INFO http_request{method=GET uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/master.m3u8?playback_session_id=e7437d42%2Db1c9%2D4ae1%2Db3f2%2D27134056d86a}: ferrite_stream::hls: Creating HLS session 036082ae-984f-4ac5-8eb6-0fd21378633b for media 5dbce626-0946-435a-aa93-f7837292a783 at 0.0s variant=480p (C:\Users\ryans\Movies\Star.Trek.Lower.Decks.2020.S05E05.1080p.HEVC.x265-MeGusta.mkv)
2026-02-22T04:46:05.220145Z  INFO http_request{method=GET uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/master.m3u8?playback_session_id=e7437d42%2Db1c9%2D4ae1%2Db3f2%2D27134056d86a}: ferrite_stream::hls: HLS variant scaling active — falling back to software encoder
2026-02-22T04:46:05.220792Z  INFO http_request{method=GET uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/master.m3u8?playback_session_id=e7437d42%2Db1c9%2D4ae1%2Db3f2%2D27134056d86a}: ferrite_stream::hls: 10-bit SDR detected (pix=yuv420p10le, transfer=Some("bt709")), applying bit-depth conversion only
2026-02-22T04:46:05.221398Z  INFO http_request{method=GET uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/master.m3u8?playback_session_id=e7437d42%2Db1c9%2D4ae1%2Db3f2%2D27134056d86a}: ferrite_stream::hls: HLS ffmpeg args: ["-hide_banner", "-nostdin", "-i", "C:\\Users\\ryans\\Movies\\Star.Trek.Lower.Decks.2020.S05E05.1080p.HEVC.x265-MeGusta.mkv", "-map", "0:v:0", "-map", "0:a:0", "-vf", "format=yuv420p,scale=-2:480", "-c:v", "libx264", "-preset", "veryfast", "-crf", "23", "-profile:v", "high", "-level", "4.1", "-b:v", "1400k", "-maxrate", "2100k", "-bufsize", "2800k", "-g", "48", "-keyint_min", "48", "-c:a", "aac", "-b:a", "128k", "-ac", "2", "-f", "hls", "-hls_time", "2", "-hls_list_size", "30", "-hls_segment_type", "fmp4", "-hls_fmp4_init_filename", "init.mp4", "-hls_segment_filename", "seg_%03d.m4s", "-hls_flags", "independent_segments+delete_segments+temp_file", "playlist.m3u8"]
2026-02-22T04:46:05.232406Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]: Input #0, matroska,webm, from 'C:\Users\ryans\Movies\Star.Trek.Lower.Decks.2020.S05E05.1080p.HEVC.x265-MeGusta.mkv':
2026-02-22T04:46:05.233107Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:   Metadata:
2026-02-22T04:46:05.233395Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:     ENCODER         : Lavf61.5.101
2026-02-22T04:46:05.233588Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:   Duration: 00:24:29.21, start: 0.000000, bitrate: 1428 kb/s
2026-02-22T04:46:05.233742Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:   Stream #0:0: Video: hevc (Main 10), yuv420p10le(tv, bt709, progressive), 1920x1080 [SAR 1:1 DAR 16:9], 23.98 fps, 23.98 tbr, 1k tbn (default)
2026-02-22T04:46:05.233927Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:     Metadata:
2026-02-22T04:46:05.234094Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:       BPS             : 9130526
2026-02-22T04:46:05.234237Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:       NUMBER_OF_FRAMES: 35225
2026-02-22T04:46:05.234377Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:       NUMBER_OF_BYTES : 1676793787
2026-02-22T04:46:05.234514Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:05.234659Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:05.234798Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:       ENCODER         : Lavc61.11.100 libx265
2026-02-22T04:46:05.234970Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:       DURATION        : 00:24:29.176000000
2026-02-22T04:46:05.235110Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:   Stream #0:1(eng): Audio: eac3, 48000 Hz, 5.1(side), fltp, 640 kb/s, start 0.024000 (default)
2026-02-22T04:46:05.235279Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:     Metadata:
2026-02-22T04:46:05.235405Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:       BPS             : 640000
2026-02-22T04:46:05.235542Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:       NUMBER_OF_FRAMES: 45912
2026-02-22T04:46:05.235676Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:       NUMBER_OF_BYTES : 117534720
2026-02-22T04:46:05.235803Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:05.235926Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:05.236049Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:       DURATION        : 00:24:29.208000000
2026-02-22T04:46:05.236167Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:   Stream #0:2(eng): Subtitle: ass (ssa)
2026-02-22T04:46:05.236299Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:     Metadata:
2026-02-22T04:46:05.236426Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:       title           : English [SDH]
2026-02-22T04:46:05.236559Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:       BPS             : 106
2026-02-22T04:46:05.236681Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:       NUMBER_OF_FRAMES: 588
2026-02-22T04:46:05.236794Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:       NUMBER_OF_BYTES : 19142
2026-02-22T04:46:05.236917Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:05.237038Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:05.237167Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:       ENCODER         : Lavc61.11.100 ssa
2026-02-22T04:46:05.237289Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:       DURATION        : 00:24:21.372000000
2026-02-22T04:46:05.237498Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]: Stream mapping:
2026-02-22T04:46:05.237620Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:   Stream #0:0 -> #0:0 (hevc (native) -> h264 (h264_nvenc))
2026-02-22T04:46:05.237733Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:   Stream #0:1 -> #0:1 (eac3 (native) -> aac (native))
2026-02-22T04:46:05.241982Z  INFO http_request{method=GET uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/master.m3u8?playback_session_id=e7437d42%2Db1c9%2D4ae1%2Db3f2%2D27134056d86a}: ferrite_stream::hls: Creating HLS session 03e343a2-c1a0-450b-9e63-5f658180a051 for media 5dbce626-0946-435a-aa93-f7837292a783 at 0.0s variant=360p (C:\Users\ryans\Movies\Star.Trek.Lower.Decks.2020.S05E05.1080p.HEVC.x265-MeGusta.mkv)
2026-02-22T04:46:05.242379Z  INFO http_request{method=GET uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/master.m3u8?playback_session_id=e7437d42%2Db1c9%2D4ae1%2Db3f2%2D27134056d86a}: ferrite_stream::hls: HLS variant scaling active — falling back to software encoder
2026-02-22T04:46:05.242626Z  INFO http_request{method=GET uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/master.m3u8?playback_session_id=e7437d42%2Db1c9%2D4ae1%2Db3f2%2D27134056d86a}: ferrite_stream::hls: 10-bit SDR detected (pix=yuv420p10le, transfer=Some("bt709")), applying bit-depth conversion only
2026-02-22T04:46:05.242928Z  INFO http_request{method=GET uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/master.m3u8?playback_session_id=e7437d42%2Db1c9%2D4ae1%2Db3f2%2D27134056d86a}: ferrite_stream::hls: HLS ffmpeg args: ["-hide_banner", "-nostdin", "-i", "C:\\Users\\ryans\\Movies\\Star.Trek.Lower.Decks.2020.S05E05.1080p.HEVC.x265-MeGusta.mkv", "-map", "0:v:0", "-map", "0:a:0", "-vf", "format=yuv420p,scale=-2:360", "-c:v", "libx264", "-preset", "veryfast", "-crf", "23", "-profile:v", "high", "-level", "4.1", "-b:v", "800k", "-maxrate", "1200k", "-bufsize", "1600k", "-g", "48", "-keyint_min", "48", "-c:a", "aac", "-b:a", "96k", "-ac", "2", "-f", "hls", "-hls_time", "2", "-hls_list_size", "30", "-hls_segment_type", "fmp4", "-hls_fmp4_init_filename", "init.mp4", "-hls_segment_filename", "seg_%03d.m4s", "-hls_flags", "independent_segments+delete_segments+temp_file", "playlist.m3u8"]
2026-02-22T04:46:05.252833Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]: Input #0, matroska,webm, from 'C:\Users\ryans\Movies\Star.Trek.Lower.Decks.2020.S05E05.1080p.HEVC.x265-MeGusta.mkv':
2026-02-22T04:46:05.253833Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:   Metadata:
2026-02-22T04:46:05.254824Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:     ENCODER         : Lavf61.5.101
2026-02-22T04:46:05.255001Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:   Duration: 00:24:29.21, start: 0.000000, bitrate: 1428 kb/s
2026-02-22T04:46:05.255274Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:   Stream #0:0: Video: hevc (Main 10), yuv420p10le(tv, bt709, progressive), 1920x1080 [SAR 1:1 DAR 16:9], 23.98 fps, 23.98 tbr, 1k tbn (default)
2026-02-22T04:46:05.255420Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:     Metadata:
2026-02-22T04:46:05.255573Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:       BPS             : 9130526
2026-02-22T04:46:05.255902Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:       NUMBER_OF_FRAMES: 35225
2026-02-22T04:46:05.255993Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:       NUMBER_OF_BYTES : 1676793787
2026-02-22T04:46:05.256148Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:05.256291Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:05.256657Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:       ENCODER         : Lavc61.11.100 libx265
2026-02-22T04:46:05.256778Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:       DURATION        : 00:24:29.176000000
2026-02-22T04:46:05.256878Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:   Stream #0:1(eng): Audio: eac3, 48000 Hz, 5.1(side), fltp, 640 kb/s, start 0.024000 (default)
2026-02-22T04:46:05.257007Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:     Metadata:
2026-02-22T04:46:05.257186Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:       BPS             : 640000
2026-02-22T04:46:05.257288Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:       NUMBER_OF_FRAMES: 45912
2026-02-22T04:46:05.257404Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:       NUMBER_OF_BYTES : 117534720
2026-02-22T04:46:05.257495Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:05.257849Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:05.259319Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:       DURATION        : 00:24:29.208000000
2026-02-22T04:46:05.259694Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:   Stream #0:2(eng): Subtitle: ass (ssa)
2026-02-22T04:46:05.259895Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:     Metadata:
2026-02-22T04:46:05.260233Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:       title           : English [SDH]
2026-02-22T04:46:05.260490Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:       BPS             : 106
2026-02-22T04:46:05.260895Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:       NUMBER_OF_FRAMES: 588
2026-02-22T04:46:05.261134Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:       NUMBER_OF_BYTES : 19142
2026-02-22T04:46:05.261275Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:05.261476Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:05.261897Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:       ENCODER         : Lavc61.11.100 ssa
2026-02-22T04:46:05.262221Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:       DURATION        : 00:24:21.372000000
2026-02-22T04:46:05.262524Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]: Stream mapping:
2026-02-22T04:46:05.263514Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:   Stream #0:0 -> #0:0 (hevc (native) -> h264 (libx264))
2026-02-22T04:46:05.263720Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:   Stream #0:1 -> #0:1 (eac3 (native) -> aac (native))
2026-02-22T04:46:05.292948Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]: Input #0, matroska,webm, from 'C:\Users\ryans\Movies\Star.Trek.Lower.Decks.2020.S05E05.1080p.HEVC.x265-MeGusta.mkv':
2026-02-22T04:46:05.293423Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:   Metadata:
2026-02-22T04:46:05.293631Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:     ENCODER         : Lavf61.5.101
2026-02-22T04:46:05.293951Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:   Duration: 00:24:29.21, start: 0.000000, bitrate: 1428 kb/s
2026-02-22T04:46:05.294072Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:   Stream #0:0: Video: hevc (Main 10), yuv420p10le(tv, bt709, progressive), 1920x1080 [SAR 1:1 DAR 16:9], 23.98 fps, 23.98 tbr, 1k tbn (default)
2026-02-22T04:46:05.294237Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:     Metadata:
2026-02-22T04:46:05.294663Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:       BPS             : 9130526
2026-02-22T04:46:05.294894Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:       NUMBER_OF_FRAMES: 35225
2026-02-22T04:46:05.295032Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:       NUMBER_OF_BYTES : 1676793787
2026-02-22T04:46:05.295344Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:05.295527Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:05.296332Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:       ENCODER         : Lavc61.11.100 libx265
2026-02-22T04:46:05.296522Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:       DURATION        : 00:24:29.176000000
2026-02-22T04:46:05.296670Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:   Stream #0:1(eng): Audio: eac3, 48000 Hz, 5.1(side), fltp, 640 kb/s, start 0.024000 (default)
2026-02-22T04:46:05.296866Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:     Metadata:
2026-02-22T04:46:05.297101Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:       BPS             : 640000
2026-02-22T04:46:05.297268Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:       NUMBER_OF_FRAMES: 45912
2026-02-22T04:46:05.297589Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:       NUMBER_OF_BYTES : 117534720
2026-02-22T04:46:05.297705Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:05.297823Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:05.298312Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:       DURATION        : 00:24:29.208000000
2026-02-22T04:46:05.298805Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:   Stream #0:2(eng): Subtitle: ass (ssa)
2026-02-22T04:46:05.299199Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:     Metadata:
2026-02-22T04:46:05.299543Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:       title           : English [SDH]
2026-02-22T04:46:05.299673Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:       BPS             : 106
2026-02-22T04:46:05.299854Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:       NUMBER_OF_FRAMES: 588
2026-02-22T04:46:05.300354Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:       NUMBER_OF_BYTES : 19142
2026-02-22T04:46:05.300520Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:05.300913Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:05.301435Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:       ENCODER         : Lavc61.11.100 ssa
2026-02-22T04:46:05.301865Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:       DURATION        : 00:24:21.372000000
2026-02-22T04:46:05.302565Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]: Stream mapping:
2026-02-22T04:46:05.303632Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:   Stream #0:0 -> #0:0 (hevc (native) -> h264 (libx264))
2026-02-22T04:46:05.304348Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:   Stream #0:1 -> #0:1 (eac3 (native) -> aac (native))
2026-02-22T04:46:05.343256Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]: [libx264 @ 000002220cecb640] using SAR=1/1
2026-02-22T04:46:05.344065Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]: [libx264 @ 000002220cecb640] using cpu capabilities: MMX2 SSE2Fast SSSE3 SSE4.2 AVX FMA3 BMI2 AVX2
2026-02-22T04:46:05.346080Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]: [libx264 @ 000002220cecb640] profile High, level 4.1, 4:2:0, 8-bit
2026-02-22T04:46:05.346726Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]: [libx264 @ 000002220cecb640] 264 - core 165 r3223 0480cb0 - H.264/MPEG-4 AVC codec - Copyleft 2003-2025 - http://www.videolan.org/x264.html - options: cabac=1 ref=1 deblock=1:0:0 analyse=0x3:0x113 me=hex subme=2 psy=1 psy_rd=1.00:0.00 mixed_ref=0 me_range=16 chroma_me=1 trellis=0 8x8dct=1 cqm=0 deadzone=21,11 fast_pskip=1 chroma_qp_offset=0 threads=22 lookahead_threads=5 sliced_threads=0 nr=0 decimate=1 interlaced=0 bluray_compat=0 constrained_intra=0 bframes=3 b_pyramid=2 b_adapt=1 b_bias=0 direct=1 weightb=1 open_gop=0 weightp=1 keyint=48 keyint_min=25 scenecut=40 intra_refresh=0 rc_lookahead=10 rc=crf mbtree=1 crf=23.0 qcomp=0.60 qpmin=0 qpmax=69 qpstep=4 vbv_maxrate=4200 vbv_bufsize=5600 crf_max=0.0 nal_hrd=none filler=0 ip_ratio=1.40 aq=1:1.00
2026-02-22T04:46:05.347107Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]: [hls @ 000002220cf33a40] Opening 'init.mp4' for writing
2026-02-22T04:46:05.347505Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]: Output #0, hls, to 'playlist.m3u8':
2026-02-22T04:46:05.347906Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:   Metadata:
2026-02-22T04:46:05.348270Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]: Input #0, matroska,webm, from 'C:\Users\ryans\Movies\Star.Trek.Lower.Decks.2020.S05E05.1080p.HEVC.x265-MeGusta.mkv':
2026-02-22T04:46:05.348366Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:     encoder         : Lavf62.3.100
2026-02-22T04:46:05.348500Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:   Metadata:
2026-02-22T04:46:05.349120Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:     ENCODER         : Lavf61.5.101
2026-02-22T04:46:05.349309Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:   Duration: 00:24:29.21, start: 0.000000, bitrate: 1428 kb/s
2026-02-22T04:46:05.348752Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:   Stream #0:0: Video: h264, yuv420p(tv, bt709, progressive), 1280x720 [SAR 1:1 DAR 16:9], q=2-31, 2800 kb/s, 23.98 fps, 24k tbn (default)
2026-02-22T04:46:05.349769Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:   Stream #0:0: Video: hevc (Main 10), yuv420p10le(tv, bt709, progressive), 1920x1080 [SAR 1:1 DAR 16:9], 23.98 fps, 23.98 tbr, 1k tbn (default)
2026-02-22T04:46:05.350224Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:     Metadata:
2026-02-22T04:46:05.350621Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:     Metadata:
2026-02-22T04:46:05.350938Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:       encoder         : Lavc62.11.100 libx264
2026-02-22T04:46:05.351087Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:       BPS             : 9130526
2026-02-22T04:46:05.351794Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:       BPS             : 9130526
2026-02-22T04:46:05.352189Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:       NUMBER_OF_FRAMES: 35225
2026-02-22T04:46:05.352260Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:       NUMBER_OF_FRAMES: 35225
2026-02-22T04:46:05.353115Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:       NUMBER_OF_BYTES : 1676793787
2026-02-22T04:46:05.352659Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:       NUMBER_OF_BYTES : 1676793787
2026-02-22T04:46:05.353269Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:05.353620Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:05.354432Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:05.354640Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:05.354830Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:       ENCODER         : Lavc61.11.100 libx265
2026-02-22T04:46:05.355844Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:       DURATION        : 00:24:29.176000000
2026-02-22T04:46:05.355070Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:       DURATION        : 00:24:29.176000000
2026-02-22T04:46:05.356022Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:   Stream #0:1(eng): Audio: eac3, 48000 Hz, 5.1(side), fltp, 640 kb/s, start 0.024000 (default)
2026-02-22T04:46:05.356173Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:     Side data:
2026-02-22T04:46:05.356958Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:       cpb: bitrate max/min/avg: 4200000/0/2800000 buffer size: 5600000 vbv_delay: N/A
2026-02-22T04:46:05.356617Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:     Metadata:
2026-02-22T04:46:05.357091Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:   Stream #0:1(eng): Audio: aac (LC), 48000 Hz, stereo, fltp, 128 kb/s (default)
2026-02-22T04:46:05.357603Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:       BPS             : 640000
2026-02-22T04:46:05.358221Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:     Metadata:
2026-02-22T04:46:05.358350Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:       NUMBER_OF_FRAMES: 45912
2026-02-22T04:46:05.358512Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:       encoder         : Lavc62.11.100 aac
2026-02-22T04:46:05.358774Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:       BPS             : 640000
2026-02-22T04:46:05.358646Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:       NUMBER_OF_BYTES : 117534720
2026-02-22T04:46:05.358938Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:       NUMBER_OF_FRAMES: 45912
2026-02-22T04:46:05.359602Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:       NUMBER_OF_BYTES : 117534720
2026-02-22T04:46:05.359471Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:05.360016Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:05.359752Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:05.360273Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:       DURATION        : 00:24:29.208000000
2026-02-22T04:46:05.361101Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:   Stream #0:2(eng): Subtitle: ass (ssa)
2026-02-22T04:46:05.360999Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:05.361429Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:     Metadata:
2026-02-22T04:46:05.361877Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:       title           : English [SDH]
2026-02-22T04:46:05.361564Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]:       DURATION        : 00:24:29.208000000
2026-02-22T04:46:05.362665Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:       BPS             : 106
2026-02-22T04:46:05.363779Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:       NUMBER_OF_FRAMES: 588
2026-02-22T04:46:05.364007Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:       NUMBER_OF_BYTES : 19142
2026-02-22T04:46:05.364714Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:05.365607Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:05.366331Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:       ENCODER         : Lavc61.11.100 ssa
2026-02-22T04:46:05.366581Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:       DURATION        : 00:24:21.372000000
2026-02-22T04:46:05.366850Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]: Stream mapping:
2026-02-22T04:46:05.367390Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:   Stream #0:0 -> #0:0 (hevc (native) -> h264 (libx264))
2026-02-22T04:46:05.367648Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:   Stream #0:1 -> #0:1 (eac3 (native) -> aac (native))
2026-02-22T04:46:05.382740Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]: [libx264 @ 000001b0b927b640] using SAR=1280/1281
2026-02-22T04:46:05.384972Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]: [libx264 @ 000001b0b927b640] using cpu capabilities: MMX2 SSE2Fast SSSE3 SSE4.2 AVX FMA3 BMI2 AVX2
2026-02-22T04:46:05.387292Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]: [libx264 @ 000001b0b927b640] profile High, level 4.1, 4:2:0, 8-bit
2026-02-22T04:46:05.387740Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]: [libx264 @ 000001b0b927b640] 264 - core 165 r3223 0480cb0 - H.264/MPEG-4 AVC codec - Copyleft 2003-2025 - http://www.videolan.org/x264.html - options: cabac=1 ref=1 deblock=1:0:0 analyse=0x3:0x113 me=hex subme=2 psy=1 psy_rd=1.00:0.00 mixed_ref=0 me_range=16 chroma_me=1 trellis=0 8x8dct=1 cqm=0 deadzone=21,11 fast_pskip=1 chroma_qp_offset=0 threads=15 lookahead_threads=3 sliced_threads=0 nr=0 decimate=1 interlaced=0 bluray_compat=0 constrained_intra=0 bframes=3 b_pyramid=2 b_adapt=1 b_bias=0 direct=1 weightb=1 open_gop=0 weightp=1 keyint=48 keyint_min=25 scenecut=40 intra_refresh=0 rc_lookahead=10 rc=crf mbtree=1 crf=23.0 qcomp=0.60 qpmin=0 qpmax=69 qpstep=4 vbv_maxrate=2100 vbv_bufsize=2800 crf_max=0.0 nal_hrd=none filler=0 ip_ratio=1.40 aq=1:1.00
2026-02-22T04:46:05.388246Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]: [hls @ 000001b0b92e3a40] Opening 'init.mp4' for writing
2026-02-22T04:46:05.388493Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]: Output #0, hls, to 'playlist.m3u8':
2026-02-22T04:46:05.388698Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:   Metadata:
2026-02-22T04:46:05.388875Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:     encoder         : Lavf62.3.100
2026-02-22T04:46:05.389297Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:   Stream #0:0: Video: h264, yuv420p(tv, bt709, progressive), 854x480 [SAR 1280:1281 DAR 16:9], q=2-31, 1400 kb/s, 23.98 fps, 24k tbn (default)
2026-02-22T04:46:05.389901Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:     Metadata:
2026-02-22T04:46:05.390818Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:       encoder         : Lavc62.11.100 libx264
2026-02-22T04:46:05.391310Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:       BPS             : 9130526
2026-02-22T04:46:05.391460Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:       NUMBER_OF_FRAMES: 35225
2026-02-22T04:46:05.393243Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:       NUMBER_OF_BYTES : 1676793787
2026-02-22T04:46:05.394078Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:05.394861Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:05.395774Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:       DURATION        : 00:24:29.176000000
2026-02-22T04:46:05.396611Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:     Side data:
2026-02-22T04:46:05.396817Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:       cpb: bitrate max/min/avg: 2100000/0/1400000 buffer size: 2800000 vbv_delay: N/A
2026-02-22T04:46:05.397108Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:   Stream #0:1(eng): Audio: aac (LC), 48000 Hz, stereo, fltp, 128 kb/s (default)
2026-02-22T04:46:05.397351Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:     Metadata:
2026-02-22T04:46:05.397466Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:       encoder         : Lavc62.11.100 aac
2026-02-22T04:46:05.399203Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:       BPS             : 640000
2026-02-22T04:46:05.400677Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:       NUMBER_OF_FRAMES: 45912
2026-02-22T04:46:05.400978Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:       NUMBER_OF_BYTES : 117534720
2026-02-22T04:46:05.401185Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:05.401779Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:05.403147Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]:       DURATION        : 00:24:29.208000000
2026-02-22T04:46:05.461870Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]: [libx264 @ 000001f1eecba180] using SAR=1/1
2026-02-22T04:46:05.462252Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]: [libx264 @ 000001f1eecba180] using cpu capabilities: MMX2 SSE2Fast SSSE3 SSE4.2 AVX FMA3 BMI2 AVX2
2026-02-22T04:46:05.466215Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]: [libx264 @ 000001f1eecba180] profile High, level 4.1, 4:2:0, 8-bit
2026-02-22T04:46:05.466488Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]: [libx264 @ 000001f1eecba180] 264 - core 165 r3223 0480cb0 - H.264/MPEG-4 AVC codec - Copyleft 2003-2025 - http://www.videolan.org/x264.html - options: cabac=1 ref=1 deblock=1:0:0 analyse=0x3:0x113 me=hex subme=2 psy=1 psy_rd=1.00:0.00 mixed_ref=0 me_range=16 chroma_me=1 trellis=0 8x8dct=1 cqm=0 deadzone=21,11 fast_pskip=1 chroma_qp_offset=0 threads=11 lookahead_threads=2 sliced_threads=0 nr=0 decimate=1 interlaced=0 bluray_compat=0 constrained_intra=0 bframes=3 b_pyramid=2 b_adapt=1 b_bias=0 direct=1 weightb=1 open_gop=0 weightp=1 keyint=48 keyint_min=25 scenecut=40 intra_refresh=0 rc_lookahead=10 rc=crf mbtree=1 crf=23.0 qcomp=0.60 qpmin=0 qpmax=69 qpstep=4 vbv_maxrate=1200 vbv_bufsize=1600 crf_max=0.0 nal_hrd=none filler=0 ip_ratio=1.40 aq=1:1.00
2026-02-22T04:46:05.466805Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]: [hls @ 000001f1eed24ac0] Opening 'init.mp4' for writing
2026-02-22T04:46:05.467164Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]: Output #0, hls, to 'playlist.m3u8':
2026-02-22T04:46:05.467367Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:   Metadata:
2026-02-22T04:46:05.467663Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:     encoder         : Lavf62.3.100
2026-02-22T04:46:05.467881Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:   Stream #0:0: Video: h264, yuv420p(tv, bt709, progressive), 640x360 [SAR 1:1 DAR 16:9], q=2-31, 800 kb/s, 23.98 fps, 24k tbn (default)
2026-02-22T04:46:05.468087Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:     Metadata:
2026-02-22T04:46:05.468471Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:       encoder         : Lavc62.11.100 libx264
2026-02-22T04:46:05.469090Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:       BPS             : 9130526
2026-02-22T04:46:05.469645Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:       NUMBER_OF_FRAMES: 35225
2026-02-22T04:46:05.470792Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:       NUMBER_OF_BYTES : 1676793787
2026-02-22T04:46:05.471035Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:05.471166Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:05.471392Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:       DURATION        : 00:24:29.176000000
2026-02-22T04:46:05.471580Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:     Side data:
2026-02-22T04:46:05.472099Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:       cpb: bitrate max/min/avg: 1200000/0/800000 buffer size: 1600000 vbv_delay: N/A
2026-02-22T04:46:05.474005Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:   Stream #0:1(eng): Audio: aac (LC), 48000 Hz, stereo, fltp, 96 kb/s (default)
2026-02-22T04:46:05.474288Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:     Metadata:
2026-02-22T04:46:05.474488Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:       encoder         : Lavc62.11.100 aac
2026-02-22T04:46:05.474664Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:       BPS             : 640000
2026-02-22T04:46:05.474871Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:       NUMBER_OF_FRAMES: 45912
2026-02-22T04:46:05.475062Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:       NUMBER_OF_BYTES : 117534720
2026-02-22T04:46:05.475232Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:05.475404Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:05.475526Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]:       DURATION        : 00:24:29.208000000
2026-02-22T04:46:05.522731Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]: [hls @ 000001c76b6c3d40] Opening 'init.mp4' for writing
2026-02-22T04:46:05.523240Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]: Output #0, hls, to 'playlist.m3u8':
2026-02-22T04:46:05.524537Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:   Metadata:
2026-02-22T04:46:05.525163Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:     encoder         : Lavf62.3.100
2026-02-22T04:46:05.525412Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:   Stream #0:0: Video: h264 (High), yuv420p(tv, bt709, progressive), 1920x1080 [SAR 1:1 DAR 16:9], q=2-31, 23.98 fps, 24k tbn (default)
2026-02-22T04:46:05.528684Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:     Metadata:
2026-02-22T04:46:05.531855Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:       encoder         : Lavc62.11.100 h264_nvenc
2026-02-22T04:46:05.532270Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:       BPS             : 9130526
2026-02-22T04:46:05.533048Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:       NUMBER_OF_FRAMES: 35225
2026-02-22T04:46:05.533297Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:       NUMBER_OF_BYTES : 1676793787
2026-02-22T04:46:05.533550Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:05.533693Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:05.533993Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:       DURATION        : 00:24:29.176000000
2026-02-22T04:46:05.534337Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:     Side data:
2026-02-22T04:46:05.534693Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:       cpb: bitrate max/min/avg: 0/0/0 buffer size: 0 vbv_delay: N/A
2026-02-22T04:46:05.534837Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:   Stream #0:1(eng): Audio: aac (LC), 48000 Hz, stereo, fltp, 192 kb/s (default)
2026-02-22T04:46:05.534979Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:     Metadata:
2026-02-22T04:46:05.535109Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:       encoder         : Lavc62.11.100 aac
2026-02-22T04:46:05.536071Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:       BPS             : 640000
2026-02-22T04:46:05.536604Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:       NUMBER_OF_FRAMES: 45912
2026-02-22T04:46:05.536751Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:       NUMBER_OF_BYTES : 117534720
2026-02-22T04:46:05.541524Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:05.541998Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:05.542170Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]:       DURATION        : 00:24:29.208000000
2026-02-22T04:46:05.748822Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]: [hls @ 000001c76b6c3d40] Opening 'seg_000.m4s.tmp' for writing
2026-02-22T04:46:05.767713Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]: frame=   49 fp[hls @ 000001c76b6c3d40] Opening 'playlist.m3u8.tmp' for writinglapsed=0:00:00.51
2026-02-22T04:46:05.836915Z  INFO http_request{method=GET uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/master.m3u8?playback_session_id=e7437d42%2Db1c9%2D4ae1%2Db3f2%2D27134056d86a}: ferrite_stream::hls: HLS session 1fb90c41-4ce6-4981-9e4c-af27429fbd15 ready (first segment generated in 0.6s)
2026-02-22T04:46:05.881436Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]: frame=   57 fp[hls @ 000002220cf33a40] Opening 'seg_000.m4s.tmp' for writing elapsed=0:00:00.50
2026-02-22T04:46:05.907425Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]: [hls @ 000002220cf33a40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:05.980637Z  INFO http_request{method=GET uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/master.m3u8?playback_session_id=e7437d42%2Db1c9%2D4ae1%2Db3f2%2D27134056d86a}: ferrite_stream::hls: HLS session b47461b6-4c7d-47d3-90c1-c0118124723c ready (first segment generated in 0.8s)
2026-02-22T04:46:05.986451Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]: [hls @ 000001c76b6c3d40] Opening 'seg_001.m4s.tmp' for writing
2026-02-22T04:46:06.013514Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]: [hls @ 000001c76b6c3d40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:06.020705Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]: frame=   55 fp[hls @ 000001b0b92e3a40] Opening 'seg_000.m4s.tmp' for writing elapsed=0:00:00.51
2026-02-22T04:46:06.042898Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]: [hls @ 000001b0b92e3a40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:06.099341Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]: [hls @ 000002220cf33a40] Opening 'seg_001.m4s.tmp' for writing
2026-02-22T04:46:06.110005Z  INFO http_request{method=GET uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/master.m3u8?playback_session_id=e7437d42%2Db1c9%2D4ae1%2Db3f2%2D27134056d86a}: ferrite_stream::hls: HLS session 036082ae-984f-4ac5-8eb6-0fd21378633b ready (first segment generated in 0.9s)
2026-02-22T04:46:06.117816Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]: frame=   49 fp[hls @ 000001f1eed24ac0] Opening 'seg_000.m4s.tmp' for writing elapsed=0:00:00.51
2026-02-22T04:46:06.119291Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]: [hls @ 000002220cf33a40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:06.133374Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]: [hls @ 000001f1eed24ac0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:06.180979Z  INFO http_request{method=GET uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/master.m3u8?playback_session_id=e7437d42%2Db1c9%2D4ae1%2Db3f2%2D27134056d86a}: ferrite_stream::hls: HLS session 03e343a2-c1a0-450b-9e63-5f658180a051 ready (first segment generated in 0.9s)
2026-02-22T04:46:06.182027Z  INFO http_request{method=GET uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/master.m3u8?playback_session_id=e7437d42%2Db1c9%2D4ae1%2Db3f2%2D27134056d86a}: ferrite_api::handlers::stream: HLS master playlist for 5dbce626-0946-435a-aa93-f7837292a783: variants=4 reused=false mode=fast seek_source=none db=1ms seek=0ms session=1001ms total=1003ms
2026-02-22T04:46:06.225351Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]: [hls @ 000001b0b92e3a40] Opening 'seg_001.m4s.tmp' for writing
2026-02-22T04:46:06.242294Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]: [hls @ 000001b0b92e3a40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:06.244117Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]: [hls @ 000001c76b6c3d40] Opening 'seg_002.m4s.tmp' for writing
2026-02-22T04:46:06.261823Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]: [hls @ 000001c76b6c3d40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:06.300462Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]: frame=  165 fp[hls @ 000002220cf33a40] Opening 'seg_002.m4s.tmp' for writing elapsed=0:00:01.02
2026-02-22T04:46:06.325192Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]: [hls @ 000002220cf33a40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:06.337914Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]: [hls @ 000001f1eed24ac0] Opening 'seg_001.m4s.tmp' for writing
2026-02-22T04:46:06.358680Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]: [hls @ 000001f1eed24ac0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:06.461569Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]: frame=  160 fp[hls @ 000001b0b92e3a40] Opening 'seg_002.m4s.tmp' for writing elapsed=0:00:01.02
2026-02-22T04:46:06.489204Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]: [hls @ 000001b0b92e3a40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:06.506148Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]: [hls @ 000002220cf33a40] Opening 'seg_003.m4s.tmp' for writing
2026-02-22T04:46:06.534958Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]: [hls @ 000002220cf33a40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:06.600812Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]: frame=  142 fp[hls @ 000001f1eed24ac0] Opening 'seg_002.m4s.tmp' for writing elapsed=0:00:01.03
2026-02-22T04:46:06.625100Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]: [hls @ 000001f1eed24ac0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:06.631091Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]: frame=  147 fp[hls @ 000001c76b6c3d40] Opening 'seg_003.m4s.tmp' for writing elapsed=0:00:01.02
2026-02-22T04:46:06.651143Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]: [hls @ 000001c76b6c3d40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:06.696092Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]: [hls @ 000001b0b92e3a40] Opening 'seg_003.m4s.tmp' for writing
2026-02-22T04:46:06.717895Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]: [hls @ 000001b0b92e3a40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:06.752609Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]: [hls @ 000002220cf33a40] Opening 'seg_004.m4s.tmp' for writing
2026-02-22T04:46:06.775507Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]: [hls @ 000002220cf33a40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:06.844855Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]: [hls @ 000001f1eed24ac0] Opening 'seg_003.m4s.tmp' for writing
2026-02-22T04:46:06.864445Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]: [hls @ 000001f1eed24ac0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:06.891256Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]: frame=  216 fp[hls @ 000001c76b6c3d40] Opening 'seg_004.m4s.tmp' for writing elapsed=0:00:01.53
2026-02-22T04:46:06.917755Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]: [hls @ 000001c76b6c3d40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:06.928284Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]: frame=  263 fp[hls @ 000001b0b92e3a40] Opening 'seg_004.m4s.tmp' for writing elapsed=0:00:01.54
2026-02-22T04:46:06.950392Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]: [hls @ 000001b0b92e3a40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:06.973553Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]: frame=  271 fp[hls @ 000002220cf33a40] Opening 'seg_005.m4s.tmp' for writing elapsed=0:00:01.53
2026-02-22T04:46:06.993723Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]: [hls @ 000002220cf33a40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:07.110522Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]: frame=  245 fp[hls @ 000001f1eed24ac0] Opening 'seg_004.m4s.tmp' for writing elapsed=0:00:01.54
2026-02-22T04:46:07.138259Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]: [hls @ 000001f1eed24ac0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:07.147068Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]: [hls @ 000001c76b6c3d40] Opening 'seg_005.m4s.tmp' for writing
2026-02-22T04:46:07.188646Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]: [hls @ 000001c76b6c3d40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:07.207447Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]: [hls @ 000001b0b92e3a40] Opening 'seg_005.m4s.tmp' for writing
2026-02-22T04:46:07.210764Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]: [hls @ 000002220cf33a40] Opening 'seg_006.m4s.tmp' for writing
2026-02-22T04:46:07.227426Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]: [hls @ 000001b0b92e3a40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:07.227730Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]: [hls @ 000002220cf33a40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:07.376928Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]: [hls @ 000001f1eed24ac0] Opening 'seg_005.m4s.tmp' for writing
2026-02-22T04:46:07.379570Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]: frame=  367 fp[hls @ 000001b0b92e3a40] Opening 'seg_006.m4s.tmp' for writing elapsed=0:00:02.06
2026-02-22T04:46:07.400037Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]: [hls @ 000001f1eed24ac0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:07.400302Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]: [hls @ 000001b0b92e3a40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:07.438922Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]: frame=  385 fp[hls @ 000002220cf33a40] Opening 'seg_007.m4s.tmp' for writing elapsed=0:00:02.05
2026-02-22T04:46:07.487106Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]: [hls @ 000002220cf33a40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:07.541782Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]: frame=  343 fp[hls @ 000001f1eed24ac0] Opening 'seg_006.m4s.tmp' for writing elapsed=0:00:02.07
2026-02-22T04:46:07.567249Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]: [hls @ 000001b0b92e3a40] Opening 'seg_007.m4s.tmp' for writing
2026-02-22T04:46:07.570072Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]: [hls @ 000001f1eed24ac0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:07.584071Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]: [hls @ 000001b0b92e3a40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:07.599137Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]: frame=  303 fp[hls @ 000001c76b6c3d40] Opening 'seg_006.m4s.tmp' for writing elapsed=0:00:02.06
2026-02-22T04:46:07.644411Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]: [hls @ 000001c76b6c3d40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:07.710790Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]: [hls @ 000002220cf33a40] Opening 'seg_008.m4s.tmp' for writing
2026-02-22T04:46:07.743620Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]: [hls @ 000002220cf33a40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:07.766930Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]: [hls @ 000001b0b92e3a40] Opening 'seg_008.m4s.tmp' for writing
2026-02-22T04:46:07.770813Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]: [hls @ 000001f1eed24ac0] Opening 'seg_007.m4s.tmp' for writing
2026-02-22T04:46:07.788484Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]: [hls @ 000001b0b92e3a40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:07.805400Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]: [hls @ 000001f1eed24ac0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:07.912798Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]: frame=  365 fp[hls @ 000001c76b6c3d40] Opening 'seg_007.m4s.tmp' for writing elapsed=0:00:02.57
2026-02-22T04:46:07.928177Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]: [hls @ 000001c76b6c3d40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:07.936748Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]: frame=  478 fp[hls @ 000002220cf33a40] Opening 'seg_009.m4s.tmp' for writing elapsed=0:00:02.57
2026-02-22T04:46:07.953667Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]: [hls @ 000002220cf33a40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:07.973098Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]: frame=  456 fp[hls @ 000001f1eed24ac0] Opening 'seg_008.m4s.tmp' for writing elapsed=0:00:02.58
2026-02-22T04:46:08.020035Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]: [hls @ 000001f1eed24ac0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:08.028783Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]: frame=  477 fp[hls @ 000001b0b92e3a40] Opening 'seg_009.m4s.tmp' for writing elapsed=0:00:02.58
2026-02-22T04:46:08.045166Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]: [hls @ 000001b0b92e3a40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:08.124896Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]: [hls @ 000001c76b6c3d40] Opening 'seg_008.m4s.tmp' for writing
2026-02-22T04:46:08.152098Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]: [hls @ 000001c76b6c3d40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:08.193582Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]: [hls @ 000001f1eed24ac0] Opening 'seg_009.m4s.tmp' for writing
2026-02-22T04:46:08.217652Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]: [hls @ 000001f1eed24ac0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:08.218984Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]: [hls @ 000002220cf33a40] Opening 'seg_010.m4s.tmp' for writing
2026-02-22T04:46:08.233356Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]: [hls @ 000002220cf33a40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:08.284478Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]: [hls @ 000001b0b92e3a40] Opening 'seg_010.m4s.tmp' for writing
2026-02-22T04:46:08.420912Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]: [hls @ 000001b0b92e3a40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:08.479025Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]: frame=  475 fp[hls @ 000001c76b6c3d40] Opening 'seg_009.m4s.tmp' for writing elapsed=0:00:03.11
2026-02-22T04:46:08.482425Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]: frame=  546 fp[hls @ 000001f1eed24ac0] Opening 'seg_010.m4s.tmp' for writing elapsed=0:00:03.10
2026-02-22T04:46:08.527711Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]: [hls @ 000001f1eed24ac0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:08.535894Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]: [hls @ 000001c76b6c3d40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:08.769577Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]: frame=  556 fp[hls @ 000002220cf33a40] Opening 'seg_011.m4s.tmp' for writing elapsed=0:00:03.10
2026-02-22T04:46:08.816142Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]: [hls @ 000002220cf33a40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:08.820859Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]: frame=  555 fp[hls @ 000001b0b92e3a40] Opening 'seg_011.m4s.tmp' for writing elapsed=0:00:03.10
2026-02-22T04:46:08.870832Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]: frame=  526 fp[hls @ 000001c76b6c3d40] Opening 'seg_010.m4s.tmp' for writing elapsed=0:00:03.62
2026-02-22T04:46:08.874414Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]: [hls @ 000001b0b92e3a40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:08.917836Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]: [hls @ 000001c76b6c3d40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:08.929328Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]: [hls @ 000001f1eed24ac0] Opening 'seg_011.m4s.tmp' for writing
2026-02-22T04:46:08.968565Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]: [hls @ 000001f1eed24ac0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:09.278041Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]: [hls @ 000001c76b6c3d40] Opening 'seg_011.m4s.tmp' for writing
2026-02-22T04:46:09.300968Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]: frame=  605 fp[hls @ 000002220cf33a40] Opening 'seg_012.m4s.tmp' for writing elapsed=0:00:03.62
2026-02-22T04:46:09.331665Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]: [hls @ 000001c76b6c3d40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:09.337555Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]: [hls @ 000002220cf33a40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:09.406062Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]: frame=  596 fp[hls @ 000001f1eed24ac0] Opening 'seg_012.m4s.tmp' for writing elapsed=0:00:03.62
2026-02-22T04:46:09.444223Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]: [hls @ 000001f1eed24ac0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:09.462577Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]: frame=  596 fp[hls @ 000001b0b92e3a40] Opening 'seg_012.m4s.tmp' for writing/A speed=6.43x elapsed=0:00:04.13
2026-02-22T04:46:09.499087Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]: [hls @ 000001b0b92e3a40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:09.815868Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]: frame=  655 fp[hls @ 000001f1eed24ac0] Opening 'seg_013.m4s.tmp' for writing elapsed=0:00:04.14
2026-02-22T04:46:09.841410Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]: frame=  582 fp[hls @ 000001c76b6c3d40] Opening 'seg_012.m4s.tmp' for writing elapsed=0:00:04.13
2026-02-22T04:46:09.861475Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]: frame=  652 fp[hls @ 000002220cf33a40] Opening 'seg_013.m4s.tmp' for writing elapsed=0:00:04.13
2026-02-22T04:46:09.877490Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]: [hls @ 000001f1eed24ac0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:09.887547Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]: [hls @ 000001c76b6c3d40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:09.895584Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]: [hls @ 000002220cf33a40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:10.035056Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]: frame=  684 fp[hls @ 000001b0b92e3a40] Opening 'seg_013.m4s.tmp' for writing elapsed=0:00:04.65
2026-02-22T04:46:10.074712Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]: [hls @ 000001b0b92e3a40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:10.254335Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]: frame=  625 fp[hls @ 000001c76b6c3d40] Opening 'seg_013.m4s.tmp' for writing elapsed=0:00:04.66
2026-02-22T04:46:10.313242Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]: [hls @ 000001c76b6c3d40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:10.358122Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]: frame=  710 fp[hls @ 000001f1eed24ac0] Opening 'seg_014.m4s.tmp' for writing elapsed=0:00:04.65
2026-02-22T04:46:10.393970Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]: [hls @ 000001f1eed24ac0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:10.405063Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]: frame=  692 fp[hls @ 000002220cf33a40] Opening 'seg_014.m4s.tmp' for writing elapsed=0:00:04.65
2026-02-22T04:46:10.449271Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]: frame=  740 fp[hls @ 000002220cf33a40] Opening 'playlist.m3u8.tmp' for writinglapsed=0:00:05.17
2026-02-22T04:46:10.604775Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]: frame=  728 fp[hls @ 000001b0b92e3a40] Opening 'seg_014.m4s.tmp' for writing elapsed=0:00:05.16
2026-02-22T04:46:10.675080Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]: [hls @ 000001b0b92e3a40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:10.836158Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]: frame=  684 fp[hls @ 000001c76b6c3d40] Opening 'seg_014.m4s.tmp' for writing elapsed=0:00:05.17
2026-02-22T04:46:10.883190Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]: frame=  758 fp[hls @ 000001f1eed24ac0] Opening 'seg_015.m4s.tmp' for writing elapsed=0:00:05.18
2026-02-22T04:46:10.889607Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]: [hls @ 000001c76b6c3d40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:10.989791Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]: [hls @ 000001f1eed24ac0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:11.014069Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]: frame=  782 fp[hls @ 000002220cf33a40] Opening 'seg_015.m4s.tmp' for writing elapsed=0:00:05.68
2026-02-22T04:46:11.055971Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]: [hls @ 000002220cf33a40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:11.277075Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]: frame=  769 fp[hls @ 000001b0b92e3a40] Opening 'seg_015.m4s.tmp' for writing elapsed=0:00:05.69
2026-02-22T04:46:11.327583Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]: [hls @ 000001b0b92e3a40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:11.336698Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]: frame=  721 fp[hls @ 000001c76b6c3d40] Opening 'seg_015.m4s.tmp' for writing elapsed=0:00:05.69
2026-02-22T04:46:11.428094Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]: [hls @ 000001c76b6c3d40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:11.536113Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]: frame=  800 fp[hls @ 000001f1eed24ac0] Opening 'seg_016.m4s.tmp' for writing elapsed=0:00:05.71
2026-02-22T04:46:11.622445Z  WARN ferrite_stream::hls: ffmpeg HLS [03e343a2-c1a0-450b-9e63-5f658180a051]: frame=  836 fp[hls @ 000001f1eed24ac0] Opening 'playlist.m3u8.tmp' for writinglapsed=0:00:06.22
2026-02-22T04:46:11.623781Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]: frame=  823 fp[hls @ 000002220cf33a40] Opening 'seg_016.m4s.tmp' for writing elapsed=0:00:06.19
2026-02-22T04:46:11.624656Z  INFO http_request{method=POST uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/seek?start=329.152&playback_session_id=e7437d42-b1c9-4ae1-b3f2-27134056d86a}: ferrite_api::handlers::stream: Transcode permit acquired: op=hls-seek media_id=5dbce626-0946-435a-aa93-f7837292a783 wait_ms=0.0
2026-02-22T04:46:11.657808Z  WARN ferrite_stream::hls: ffmpeg HLS [b47461b6-4c7d-47d3-90c1-c0118124723c]: [hls @ 000002220cf33a40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:11.685892Z  WARN ferrite_stream::hls: ffmpeg HLS [036082ae-984f-4ac5-8eb6-0fd21378633b]: frame=  806 fps=130 q=28.0 size=N/A time=00:00:33.53 bitrate=N/A speed= 5.4x elapsed=0:00:06.20
2026-02-22T04:46:11.710751Z  INFO http_request{method=POST uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/seek?start=329.152&playback_session_id=e7437d42-b1c9-4ae1-b3f2-27134056d86a}: ferrite_stream::hls: Destroyed HLS session 03e343a2-c1a0-450b-9e63-5f658180a051
2026-02-22T04:46:11.717959Z  INFO http_request{method=POST uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/seek?start=329.152&playback_session_id=e7437d42-b1c9-4ae1-b3f2-27134056d86a}: ferrite_stream::hls: Destroyed HLS session 036082ae-984f-4ac5-8eb6-0fd21378633b
2026-02-22T04:46:11.723025Z  INFO http_request{method=POST uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/seek?start=329.152&playback_session_id=e7437d42-b1c9-4ae1-b3f2-27134056d86a}: ferrite_stream::hls: Destroyed HLS session b47461b6-4c7d-47d3-90c1-c0118124723c
2026-02-22T04:46:11.746834Z  WARN ferrite_stream::hls: ffmpeg HLS [1fb90c41-4ce6-4981-9e4c-af27429fbd15]: frame=  769 fps=124 q=16.0 size=N/A time=00:00:32.15 bitrate=N/A speed=5.17x elapsed=0:00:06.21
2026-02-22T04:46:11.803902Z  INFO http_request{method=POST uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/seek?start=329.152&playback_session_id=e7437d42-b1c9-4ae1-b3f2-27134056d86a}: ferrite_stream::hls: Destroyed HLS session 1fb90c41-4ce6-4981-9e4c-af27429fbd15
2026-02-22T04:46:11.804278Z  INFO http_request{method=POST uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/seek?start=329.152&playback_session_id=e7437d42-b1c9-4ae1-b3f2-27134056d86a}: ferrite_stream::hls: Creating single HLS session for seek: media 5dbce626-0946-435a-aa93-f7837292a783 at 329.2s variant=1080p
2026-02-22T04:46:11.805842Z  INFO http_request{method=POST uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/seek?start=329.152&playback_session_id=e7437d42-b1c9-4ae1-b3f2-27134056d86a}: ferrite_stream::hls: Creating HLS session b39f9019-134e-4e25-b794-4c0bd7bbb351 for media 5dbce626-0946-435a-aa93-f7837292a783 at 329.2s variant=1080p (C:\Users\ryans\Movies\Star.Trek.Lower.Decks.2020.S05E05.1080p.HEVC.x265-MeGusta.mkv)
2026-02-22T04:46:11.806336Z  INFO http_request{method=POST uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/seek?start=329.152&playback_session_id=e7437d42-b1c9-4ae1-b3f2-27134056d86a}: ferrite_stream::hls: 10-bit SDR detected (pix=yuv420p10le, transfer=Some("bt709")), applying bit-depth conversion only
2026-02-22T04:46:11.806692Z  INFO http_request{method=POST uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/seek?start=329.152&playback_session_id=e7437d42-b1c9-4ae1-b3f2-27134056d86a}: ferrite_stream::hls: HLS ffmpeg args: ["-hide_banner", "-nostdin", "-ss", "329.152", "-i", "C:\\Users\\ryans\\Movies\\Star.Trek.Lower.Decks.2020.S05E05.1080p.HEVC.x265-MeGusta.mkv", "-ss", "0.000", "-map", "0:v:0", "-map", "0:a:0", "-vf", "format=yuv420p", "-c:v", "h264_nvenc", "-preset", "p4", "-tune", "ll", "-rc", "vbr", "-cq", "23", "-profile:v", "high", "-level", "4.1", "-g", "48", "-keyint_min", "48", "-c:a", "aac", "-b:a", "192k", "-ac", "2", "-f", "hls", "-hls_time", "2", "-hls_list_size", "30", "-hls_segment_type", "fmp4", "-hls_fmp4_init_filename", "init.mp4", "-hls_segment_filename", "seg_%03d.m4s", "-hls_flags", "independent_segments+delete_segments+temp_file", "playlist.m3u8"]
2026-02-22T04:46:11.878115Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]: Input #0, matroska,webm, from 'C:\Users\ryans\Movies\Star.Trek.Lower.Decks.2020.S05E05.1080p.HEVC.x265-MeGusta.mkv':
2026-02-22T04:46:11.878539Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:   Metadata:
2026-02-22T04:46:11.878722Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:     ENCODER         : Lavf61.5.101
2026-02-22T04:46:11.879297Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:   Duration: 00:24:29.21, start: 0.000000, bitrate: 1428 kb/s
2026-02-22T04:46:11.879637Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:   Stream #0:0: Video: hevc (Main 10), yuv420p10le(tv, bt709, progressive), 1920x1080 [SAR 1:1 DAR 16:9], 23.98 fps, 23.98 tbr, 1k tbn (default)
2026-02-22T04:46:11.880092Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:     Metadata:
2026-02-22T04:46:11.880435Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:       BPS             : 9130526
2026-02-22T04:46:11.880793Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:       NUMBER_OF_FRAMES: 35225
2026-02-22T04:46:11.881178Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:       NUMBER_OF_BYTES : 1676793787
2026-02-22T04:46:11.881505Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:11.881979Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:11.882387Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:       ENCODER         : Lavc61.11.100 libx265
2026-02-22T04:46:11.882850Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:       DURATION        : 00:24:29.176000000
2026-02-22T04:46:11.883287Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:   Stream #0:1(eng): Audio: eac3, 48000 Hz, 5.1(side), fltp, 640 kb/s, start 0.024000 (default)
2026-02-22T04:46:11.883592Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:     Metadata:
2026-02-22T04:46:11.883870Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:       BPS             : 640000
2026-02-22T04:46:11.884213Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:       NUMBER_OF_FRAMES: 45912
2026-02-22T04:46:11.884472Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:       NUMBER_OF_BYTES : 117534720
2026-02-22T04:46:11.884781Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:11.885079Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:11.885374Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:       DURATION        : 00:24:29.208000000
2026-02-22T04:46:11.885713Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:   Stream #0:2(eng): Subtitle: ass (ssa)
2026-02-22T04:46:11.886071Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:     Metadata:
2026-02-22T04:46:11.886408Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:       title           : English [SDH]
2026-02-22T04:46:11.886719Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:       BPS             : 106
2026-02-22T04:46:11.886986Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:       NUMBER_OF_FRAMES: 588
2026-02-22T04:46:11.887390Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:       NUMBER_OF_BYTES : 19142
2026-02-22T04:46:11.887536Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:11.887676Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:11.887891Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:       ENCODER         : Lavc61.11.100 ssa
2026-02-22T04:46:11.888606Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:       DURATION        : 00:24:21.372000000
2026-02-22T04:46:11.888909Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]: Stream mapping:
2026-02-22T04:46:11.889354Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:   Stream #0:0 -> #0:0 (hevc (native) -> h264 (h264_nvenc))
2026-02-22T04:46:11.889720Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:   Stream #0:1 -> #0:1 (eac3 (native) -> aac (native))
2026-02-22T04:46:12.228293Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]: [hls @ 0000027c8d8f3b80] Opening 'init.mp4' for writing
2026-02-22T04:46:12.228894Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]: Output #0, hls, to 'playlist.m3u8':
2026-02-22T04:46:12.229691Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:   Metadata:
2026-02-22T04:46:12.230374Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:     encoder         : Lavf62.3.100
2026-02-22T04:46:12.231221Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:   Stream #0:0: Video: h264 (High), yuv420p(tv, bt709, progressive), 1920x1080 [SAR 1:1 DAR 16:9], q=2-31, 23.98 fps, 24k tbn (default)
2026-02-22T04:46:12.233114Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:     Metadata:
2026-02-22T04:46:12.233830Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:       encoder         : Lavc62.11.100 h264_nvenc
2026-02-22T04:46:12.234405Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:       BPS             : 9130526
2026-02-22T04:46:12.235967Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:       NUMBER_OF_FRAMES: 35225
2026-02-22T04:46:12.236139Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:       NUMBER_OF_BYTES : 1676793787
2026-02-22T04:46:12.236510Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:12.237467Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:12.238013Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:       DURATION        : 00:24:29.176000000
2026-02-22T04:46:12.238336Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:     Side data:
2026-02-22T04:46:12.238669Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:       cpb: bitrate max/min/avg: 0/0/0 buffer size: 0 vbv_delay: N/A
2026-02-22T04:46:12.239219Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:   Stream #0:1(eng): Audio: aac (LC), 48000 Hz, stereo, fltp, 192 kb/s (default)
2026-02-22T04:46:12.239474Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:     Metadata:
2026-02-22T04:46:12.239676Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:       encoder         : Lavc62.11.100 aac
2026-02-22T04:46:12.239910Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:       BPS             : 640000
2026-02-22T04:46:12.240142Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:       NUMBER_OF_FRAMES: 45912
2026-02-22T04:46:12.240374Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:       NUMBER_OF_BYTES : 117534720
2026-02-22T04:46:12.240573Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:12.240824Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:12.241102Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]:       DURATION        : 00:24:29.208000000
2026-02-22T04:46:12.468767Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]: frame=   49 fp[hls @ 0000027c8d8f3b80] Opening 'seg_000.m4s.tmp' for writing elapsed=0:00:00.51
2026-02-22T04:46:12.476575Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]: [hls @ 0000027c8d8f3b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:12.700598Z  INFO http_request{method=POST uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/seek?start=329.152&playback_session_id=e7437d42-b1c9-4ae1-b3f2-27134056d86a}: ferrite_stream::hls: HLS session b39f9019-134e-4e25-b794-4c0bd7bbb351 ready (first segment generated in 0.9s)
2026-02-22T04:46:12.701929Z  INFO http_request{method=POST uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/seek?start=329.152&playback_session_id=e7437d42-b1c9-4ae1-b3f2-27134056d86a}: ferrite_api::handlers::stream: HLS seek for 5dbce626-0946-435a-aa93-f7837292a783: start=329.2s mode=fast seek_source=requested db=7ms seek=1ms session=1032ms total=1085ms
2026-02-22T04:46:12.756046Z  INFO http_request{method=GET uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/master.m3u8?start=329.152&playback_session_id=e7437d42%2Db1c9%2D4ae1%2Db3f2%2D27134056d86a}: ferrite_api::handlers::stream: HLS master playlist for 5dbce626-0946-435a-aa93-f7837292a783: variants=1 reused=true mode=fast seek_source=requested db=3ms seek=1ms session=0ms total=4ms
2026-02-22T04:46:13.195239Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]: [hls @ 0000027c8d8f3b80] Opening 'seg_001.m4s.tmp' for writing
2026-02-22T04:46:13.197511Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]: [hls @ 0000027c8d8f3b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:13.408596Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]: frame=   97 fp[hls @ 0000027c8d8f3b80] Opening 'seg_002.m4s.tmp' for writing elapsed=0:00:01.04
2026-02-22T04:46:13.416019Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]: [hls @ 0000027c8d8f3b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:13.548646Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]: frame=  156 fp[hls @ 0000027c8d8f3b80] Opening 'seg_003.m4s.tmp' for writing elapsed=0:00:01.55
2026-02-22T04:46:13.561890Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]: [hls @ 0000027c8d8f3b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:13.707186Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]: [hls @ 0000027c8d8f3b80] Opening 'seg_004.m4s.tmp' for writing
2026-02-22T04:46:14.024434Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]: frame=  241 fp[hls @ 0000027c8d8f3b80] Opening 'playlist.m3u8.tmp' for writinglapsed=0:00:02.07
2026-02-22T04:46:14.132347Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]: [hls @ 0000027c8d8f3b80] Opening 'seg_005.m4s.tmp' for writing
2026-02-22T04:46:14.140424Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]: [hls @ 0000027c8d8f3b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:14.241602Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]: [hls @ 0000027c8d8f3b80] Opening 'seg_006.m4s.tmp' for writing
2026-02-22T04:46:14.250534Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]: [hls @ 0000027c8d8f3b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:14.345561Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]: [hls @ 0000027c8d8f3b80] Opening 'seg_007.m4s.tmp' for writing
2026-02-22T04:46:14.355198Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]: [hls @ 0000027c8d8f3b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:14.574319Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]: [hls @ 0000027c8d8f3b80] Opening 'seg_008.m4s.tmp' for writing
2026-02-22T04:46:14.589213Z  WARN ferrite_stream::hls: ffmpeg HLS [b39f9019-134e-4e25-b794-4c0bd7bbb351]: frame=  433 fp[hls @ 0000027c8d8f3b80] Opening 'playlist.m3u8.tmp' for writinglapsed=0:00:02.58
2026-02-22T04:46:14.744224Z  INFO http_request{method=POST uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/seek?start=736.832&playback_session_id=e7437d42-b1c9-4ae1-b3f2-27134056d86a}: ferrite_api::handlers::stream: Transcode permit acquired: op=hls-seek media_id=5dbce626-0946-435a-aa93-f7837292a783 wait_ms=0.0
2026-02-22T04:46:14.853951Z  INFO http_request{method=POST uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/seek?start=736.832&playback_session_id=e7437d42-b1c9-4ae1-b3f2-27134056d86a}: ferrite_stream::hls: Destroyed HLS session b39f9019-134e-4e25-b794-4c0bd7bbb351
2026-02-22T04:46:14.854386Z  INFO http_request{method=POST uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/seek?start=736.832&playback_session_id=e7437d42-b1c9-4ae1-b3f2-27134056d86a}: ferrite_stream::hls: Creating single HLS session for seek: media 5dbce626-0946-435a-aa93-f7837292a783 at 736.8s variant=1080p
2026-02-22T04:46:14.855669Z  INFO http_request{method=POST uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/seek?start=736.832&playback_session_id=e7437d42-b1c9-4ae1-b3f2-27134056d86a}: ferrite_stream::hls: Creating HLS session 095aaf19-6e9a-4dfc-af5e-d28afb90e2a8 for media 5dbce626-0946-435a-aa93-f7837292a783 at 736.8s variant=1080p (C:\Users\ryans\Movies\Star.Trek.Lower.Decks.2020.S05E05.1080p.HEVC.x265-MeGusta.mkv)
2026-02-22T04:46:14.855944Z  INFO http_request{method=POST uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/seek?start=736.832&playback_session_id=e7437d42-b1c9-4ae1-b3f2-27134056d86a}: ferrite_stream::hls: 10-bit SDR detected (pix=yuv420p10le, transfer=Some("bt709")), applying bit-depth conversion only
2026-02-22T04:46:14.856230Z  INFO http_request{method=POST uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/seek?start=736.832&playback_session_id=e7437d42-b1c9-4ae1-b3f2-27134056d86a}: ferrite_stream::hls: HLS ffmpeg args: ["-hide_banner", "-nostdin", "-ss", "736.832", "-i", "C:\\Users\\ryans\\Movies\\Star.Trek.Lower.Decks.2020.S05E05.1080p.HEVC.x265-MeGusta.mkv", "-ss", "0.000", "-map", "0:v:0", "-map", "0:a:0", "-vf", "format=yuv420p", "-c:v", "h264_nvenc", "-preset", "p4", "-tune", "ll", "-rc", "vbr", "-cq", "23", "-profile:v", "high", "-level", "4.1", "-g", "48", "-keyint_min", "48", "-c:a", "aac", "-b:a", "192k", "-ac", "2", "-f", "hls", "-hls_time", "2", "-hls_list_size", "30", "-hls_segment_type", "fmp4", "-hls_fmp4_init_filename", "init.mp4", "-hls_segment_filename", "seg_%03d.m4s", "-hls_flags", "independent_segments+delete_segments+temp_file", "playlist.m3u8"]
2026-02-22T04:46:14.937093Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]: Input #0, matroska,webm, from 'C:\Users\ryans\Movies\Star.Trek.Lower.Decks.2020.S05E05.1080p.HEVC.x265-MeGusta.mkv':
2026-02-22T04:46:14.937637Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:   Metadata:
2026-02-22T04:46:14.938023Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:     ENCODER         : Lavf61.5.101
2026-02-22T04:46:14.938209Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:   Duration: 00:24:29.21, start: 0.000000, bitrate: 1428 kb/s
2026-02-22T04:46:14.938601Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:   Stream #0:0: Video: hevc (Main 10), yuv420p10le(tv, bt709, progressive), 1920x1080 [SAR 1:1 DAR 16:9], 23.98 fps, 23.98 tbr, 1k tbn (default)
2026-02-22T04:46:14.938875Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:     Metadata:
2026-02-22T04:46:14.939161Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:       BPS             : 9130526
2026-02-22T04:46:14.939341Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:       NUMBER_OF_FRAMES: 35225
2026-02-22T04:46:14.939509Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:       NUMBER_OF_BYTES : 1676793787
2026-02-22T04:46:14.939746Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:14.939997Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:14.940341Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:       ENCODER         : Lavc61.11.100 libx265
2026-02-22T04:46:14.940749Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:       DURATION        : 00:24:29.176000000
2026-02-22T04:46:14.940956Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:   Stream #0:1(eng): Audio: eac3, 48000 Hz, 5.1(side), fltp, 640 kb/s, start 0.024000 (default)
2026-02-22T04:46:14.941295Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:     Metadata:
2026-02-22T04:46:14.941696Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:       BPS             : 640000
2026-02-22T04:46:14.942010Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:       NUMBER_OF_FRAMES: 45912
2026-02-22T04:46:14.942353Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:       NUMBER_OF_BYTES : 117534720
2026-02-22T04:46:14.942681Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:14.942951Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:14.943323Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:       DURATION        : 00:24:29.208000000
2026-02-22T04:46:14.943668Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:   Stream #0:2(eng): Subtitle: ass (ssa)
2026-02-22T04:46:14.944202Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:     Metadata:
2026-02-22T04:46:14.944530Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:       title           : English [SDH]
2026-02-22T04:46:14.944718Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:       BPS             : 106
2026-02-22T04:46:14.945254Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:       NUMBER_OF_FRAMES: 588
2026-02-22T04:46:14.945757Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:       NUMBER_OF_BYTES : 19142
2026-02-22T04:46:14.946381Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:14.947208Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:14.947728Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:       ENCODER         : Lavc61.11.100 ssa
2026-02-22T04:46:14.948238Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:       DURATION        : 00:24:21.372000000
2026-02-22T04:46:14.948529Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]: Stream mapping:
2026-02-22T04:46:14.948710Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:   Stream #0:0 -> #0:0 (hevc (native) -> h264 (h264_nvenc))
2026-02-22T04:46:14.948888Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:   Stream #0:1 -> #0:1 (eac3 (native) -> aac (native))
2026-02-22T04:46:15.275327Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]: [hls @ 000002640c1e3b80] Opening 'init.mp4' for writing
2026-02-22T04:46:15.276007Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]: Output #0, hls, to 'playlist.m3u8':
2026-02-22T04:46:15.276293Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:   Metadata:
2026-02-22T04:46:15.276624Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:     encoder         : Lavf62.3.100
2026-02-22T04:46:15.276879Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:   Stream #0:0: Video: h264 (High), yuv420p(tv, bt709, progressive), 1920x1080 [SAR 1:1 DAR 16:9], q=2-31, 23.98 fps, 24k tbn (default)
2026-02-22T04:46:15.277117Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:     Metadata:
2026-02-22T04:46:15.277338Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:       encoder         : Lavc62.11.100 h264_nvenc
2026-02-22T04:46:15.277651Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:       BPS             : 9130526
2026-02-22T04:46:15.277878Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:       NUMBER_OF_FRAMES: 35225
2026-02-22T04:46:15.278166Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:       NUMBER_OF_BYTES : 1676793787
2026-02-22T04:46:15.278369Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:15.278601Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:15.278831Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:       DURATION        : 00:24:29.176000000
2026-02-22T04:46:15.279064Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:     Side data:
2026-02-22T04:46:15.279298Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:       cpb: bitrate max/min/avg: 0/0/0 buffer size: 0 vbv_delay: N/A
2026-02-22T04:46:15.279683Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:   Stream #0:1(eng): Audio: aac (LC), 48000 Hz, stereo, fltp, 192 kb/s (default)
2026-02-22T04:46:15.279904Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:     Metadata:
2026-02-22T04:46:15.280128Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:       encoder         : Lavc62.11.100 aac
2026-02-22T04:46:15.280394Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:       BPS             : 640000
2026-02-22T04:46:15.280915Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:       NUMBER_OF_FRAMES: 45912
2026-02-22T04:46:15.281291Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:       NUMBER_OF_BYTES : 117534720
2026-02-22T04:46:15.281682Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:15.282102Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:15.283314Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]:       DURATION        : 00:24:29.208000000
2026-02-22T04:46:15.382324Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]: [hls @ 000002640c1e3b80] Opening 'seg_000.m4s.tmp' for writing
2026-02-22T04:46:15.391875Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]: [hls @ 000002640c1e3b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:15.441804Z  INFO http_request{method=POST uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/seek?start=736.832&playback_session_id=e7437d42-b1c9-4ae1-b3f2-27134056d86a}: ferrite_stream::hls: HLS session 095aaf19-6e9a-4dfc-af5e-d28afb90e2a8 ready (first segment generated in 0.6s)
2026-02-22T04:46:15.443284Z  INFO http_request{method=POST uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/seek?start=736.832&playback_session_id=e7437d42-b1c9-4ae1-b3f2-27134056d86a}: ferrite_api::handlers::stream: HLS seek for 5dbce626-0946-435a-aa93-f7837292a783: start=736.8s mode=fast seek_source=requested db=125ms seek=2ms session=663ms total=980ms
2026-02-22T04:46:15.466372Z  INFO http_request{method=GET uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/master.m3u8?start=736.832&playback_session_id=e7437d42%2Db1c9%2D4ae1%2Db3f2%2D27134056d86a}: ferrite_api::handlers::stream: HLS master playlist for 5dbce626-0946-435a-aa93-f7837292a783: variants=1 reused=true mode=fast seek_source=requested db=1ms seek=1ms session=0ms total=3ms
2026-02-22T04:46:15.486704Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]: frame=   81 fp[hls @ 000002640c1e3b80] Opening 'seg_001.m4s.tmp' for writing0 speed= 6.6x elapsed=0:00:00.51
2026-02-22T04:46:15.498244Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]: [hls @ 000002640c1e3b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:15.592218Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]: [hls @ 000002640c1e3b80] Opening 'seg_002.m4s.tmp' for writing
2026-02-22T04:46:15.604128Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]: [hls @ 000002640c1e3b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:15.704525Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]: [hls @ 000002640c1e3b80] Opening 'seg_003.m4s.tmp' for writing
2026-02-22T04:46:15.715168Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]: [hls @ 000002640c1e3b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:15.836444Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]: [hls @ 000002640c1e3b80] Opening 'seg_004.m4s.tmp' for writing
2026-02-22T04:46:15.846861Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]: [hls @ 000002640c1e3b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:15.944264Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]: [hls @ 000002640c1e3b80] Opening 'seg_005.m4s.tmp' for writing
2026-02-22T04:46:15.954321Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]: [hls @ 000002640c1e3b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:16.050454Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]: frame=  296 fp[hls @ 000002640c1e3b80] Opening 'seg_006.m4s.tmp' for writing0 speed=  12x elapsed=0:00:01.02
2026-02-22T04:46:16.059774Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]: [hls @ 000002640c1e3b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:16.154549Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]: [hls @ 000002640c1e3b80] Opening 'seg_007.m4s.tmp' for writing
2026-02-22T04:46:16.165704Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]: [hls @ 000002640c1e3b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:16.260727Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]: [hls @ 000002640c1e3b80] Opening 'seg_008.m4s.tmp' for writing
2026-02-22T04:46:16.274819Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]: [hls @ 000002640c1e3b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:16.370123Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]: [hls @ 000002640c1e3b80] Opening 'seg_009.m4s.tmp' for writing
2026-02-22T04:46:16.381498Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]: [hls @ 000002640c1e3b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:16.473152Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]: [hls @ 000002640c1e3b80] Opening 'seg_010.m4s.tmp' for writing
2026-02-22T04:46:16.482206Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]: [hls @ 000002640c1e3b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:16.573981Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]: frame=  529 fp[hls @ 000002640c1e3b80] Opening 'seg_011.m4s.tmp' for writing0 speed=14.4x elapsed=0:00:01.54
2026-02-22T04:46:16.582978Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]: [hls @ 000002640c1e3b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:16.669147Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]: [hls @ 000002640c1e3b80] Opening 'seg_012.m4s.tmp' for writing
2026-02-22T04:46:16.678548Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]: [hls @ 000002640c1e3b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:16.769135Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]: [hls @ 000002640c1e3b80] Opening 'seg_013.m4s.tmp' for writing
2026-02-22T04:46:16.777947Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]: [hls @ 000002640c1e3b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:16.868194Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]: [hls @ 000002640c1e3b80] Opening 'seg_014.m4s.tmp' for writing
2026-02-22T04:46:16.879451Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]: [hls @ 000002640c1e3b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:16.969695Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]: [hls @ 000002640c1e3b80] Opening 'seg_015.m4s.tmp' for writing
2026-02-22T04:46:16.978146Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]: [hls @ 000002640c1e3b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:17.069058Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]: frame=  785 fp[hls @ 000002640c1e3b80] Opening 'seg_016.m4s.tmp' for writing0 speed=15.9x elapsed=0:00:02.06
2026-02-22T04:46:17.078305Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]: [hls @ 000002640c1e3b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:17.166902Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]: [hls @ 000002640c1e3b80] Opening 'seg_017.m4s.tmp' for writing
2026-02-22T04:46:17.175136Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]: [hls @ 000002640c1e3b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:17.266486Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]: [hls @ 000002640c1e3b80] Opening 'seg_018.m4s.tmp' for writing
2026-02-22T04:46:17.275453Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]: [hls @ 000002640c1e3b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:17.363482Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]: [hls @ 000002640c1e3b80] Opening 'seg_019.m4s.tmp' for writing
2026-02-22T04:46:17.371060Z  WARN ferrite_stream::hls: ffmpeg HLS [095aaf19-6e9a-4dfc-af5e-d28afb90e2a8]: [hls @ 000002640c1e3b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:17.384088Z  INFO http_request{method=POST uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/seek?start=190.474&playback_session_id=e7437d42-b1c9-4ae1-b3f2-27134056d86a}: ferrite_api::handlers::stream: Transcode permit acquired: op=hls-seek media_id=5dbce626-0946-435a-aa93-f7837292a783 wait_ms=0.0
2026-02-22T04:46:17.465466Z  INFO http_request{method=POST uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/seek?start=190.474&playback_session_id=e7437d42-b1c9-4ae1-b3f2-27134056d86a}: ferrite_stream::hls: Destroyed HLS session 095aaf19-6e9a-4dfc-af5e-d28afb90e2a8
2026-02-22T04:46:17.465817Z  INFO http_request{method=POST uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/seek?start=190.474&playback_session_id=e7437d42-b1c9-4ae1-b3f2-27134056d86a}: ferrite_stream::hls: Creating single HLS session for seek: media 5dbce626-0946-435a-aa93-f7837292a783 at 190.5s variant=1080p
2026-02-22T04:46:17.466986Z  INFO http_request{method=POST uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/seek?start=190.474&playback_session_id=e7437d42-b1c9-4ae1-b3f2-27134056d86a}: ferrite_stream::hls: Creating HLS session 1f2b4e31-b738-4890-9ca7-bc0bf10eb892 for media 5dbce626-0946-435a-aa93-f7837292a783 at 190.5s variant=1080p (C:\Users\ryans\Movies\Star.Trek.Lower.Decks.2020.S05E05.1080p.HEVC.x265-MeGusta.mkv)
2026-02-22T04:46:17.467270Z  INFO http_request{method=POST uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/seek?start=190.474&playback_session_id=e7437d42-b1c9-4ae1-b3f2-27134056d86a}: ferrite_stream::hls: 10-bit SDR detected (pix=yuv420p10le, transfer=Some("bt709")), applying bit-depth conversion only
2026-02-22T04:46:17.467731Z  INFO http_request{method=POST uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/seek?start=190.474&playback_session_id=e7437d42-b1c9-4ae1-b3f2-27134056d86a}: ferrite_stream::hls: HLS ffmpeg args: ["-hide_banner", "-nostdin", "-ss", "190.474", "-i", "C:\\Users\\ryans\\Movies\\Star.Trek.Lower.Decks.2020.S05E05.1080p.HEVC.x265-MeGusta.mkv", "-ss", "0.000", "-map", "0:v:0", "-map", "0:a:0", "-vf", "format=yuv420p", "-c:v", "h264_nvenc", "-preset", "p4", "-tune", "ll", "-rc", "vbr", "-cq", "23", "-profile:v", "high", "-level", "4.1", "-g", "48", "-keyint_min", "48", "-c:a", "aac", "-b:a", "192k", "-ac", "2", "-f", "hls", "-hls_time", "2", "-hls_list_size", "30", "-hls_segment_type", "fmp4", "-hls_fmp4_init_filename", "init.mp4", "-hls_segment_filename", "seg_%03d.m4s", "-hls_flags", "independent_segments+delete_segments+temp_file", "playlist.m3u8"]
2026-02-22T04:46:17.542312Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]: Input #0, matroska,webm, from 'C:\Users\ryans\Movies\Star.Trek.Lower.Decks.2020.S05E05.1080p.HEVC.x265-MeGusta.mkv':
2026-02-22T04:46:17.542646Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:   Metadata:
2026-02-22T04:46:17.542823Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:     ENCODER         : Lavf61.5.101
2026-02-22T04:46:17.542970Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:   Duration: 00:24:29.21, start: 0.000000, bitrate: 1428 kb/s
2026-02-22T04:46:17.543343Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:   Stream #0:0: Video: hevc (Main 10), yuv420p10le(tv, bt709, progressive), 1920x1080 [SAR 1:1 DAR 16:9], 23.98 fps, 23.98 tbr, 1k tbn (default)
2026-02-22T04:46:17.543546Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:     Metadata:
2026-02-22T04:46:17.543742Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:       BPS             : 9130526
2026-02-22T04:46:17.544912Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:       NUMBER_OF_FRAMES: 35225
2026-02-22T04:46:17.545447Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:       NUMBER_OF_BYTES : 1676793787
2026-02-22T04:46:17.545896Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:17.546223Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:17.546401Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:       ENCODER         : Lavc61.11.100 libx265
2026-02-22T04:46:17.546770Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:       DURATION        : 00:24:29.176000000
2026-02-22T04:46:17.546973Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:   Stream #0:1(eng): Audio: eac3, 48000 Hz, 5.1(side), fltp, 640 kb/s, start 0.024000 (default)
2026-02-22T04:46:17.547159Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:     Metadata:
2026-02-22T04:46:17.547660Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:       BPS             : 640000
2026-02-22T04:46:17.548017Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:       NUMBER_OF_FRAMES: 45912
2026-02-22T04:46:17.548312Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:       NUMBER_OF_BYTES : 117534720
2026-02-22T04:46:17.549340Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:17.549700Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:17.550014Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:       DURATION        : 00:24:29.208000000
2026-02-22T04:46:17.550180Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:   Stream #0:2(eng): Subtitle: ass (ssa)
2026-02-22T04:46:17.550370Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:     Metadata:
2026-02-22T04:46:17.550517Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:       title           : English [SDH]
2026-02-22T04:46:17.550628Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:       BPS             : 106
2026-02-22T04:46:17.550737Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:       NUMBER_OF_FRAMES: 588
2026-02-22T04:46:17.550848Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:       NUMBER_OF_BYTES : 19142
2026-02-22T04:46:17.550948Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:17.551061Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:17.551173Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:       ENCODER         : Lavc61.11.100 ssa
2026-02-22T04:46:17.551444Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:       DURATION        : 00:24:21.372000000
2026-02-22T04:46:17.551636Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]: Stream mapping:
2026-02-22T04:46:17.551813Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:   Stream #0:0 -> #0:0 (hevc (native) -> h264 (h264_nvenc))
2026-02-22T04:46:17.551957Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:   Stream #0:1 -> #0:1 (eac3 (native) -> aac (native))
2026-02-22T04:46:17.790550Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]: [hls @ 00000277c0a83b80] Opening 'init.mp4' for writing
2026-02-22T04:46:17.791136Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]: Output #0, hls, to 'playlist.m3u8':
2026-02-22T04:46:17.791390Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:   Metadata:
2026-02-22T04:46:17.791760Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:     encoder         : Lavf62.3.100
2026-02-22T04:46:17.792020Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:   Stream #0:0: Video: h264 (High), yuv420p(tv, bt709, progressive), 1920x1080 [SAR 1:1 DAR 16:9], q=2-31, 23.98 fps, 24k tbn (default)
2026-02-22T04:46:17.792245Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:     Metadata:
2026-02-22T04:46:17.792539Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:       encoder         : Lavc62.11.100 h264_nvenc
2026-02-22T04:46:17.792905Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:       BPS             : 9130526
2026-02-22T04:46:17.793371Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:       NUMBER_OF_FRAMES: 35225
2026-02-22T04:46:17.793718Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:       NUMBER_OF_BYTES : 1676793787
2026-02-22T04:46:17.793975Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:17.794256Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:17.794899Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:       DURATION        : 00:24:29.176000000
2026-02-22T04:46:17.795352Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:     Side data:
2026-02-22T04:46:17.795537Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:       cpb: bitrate max/min/avg: 0/0/0 buffer size: 0 vbv_delay: N/A
2026-02-22T04:46:17.796304Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:   Stream #0:1(eng): Audio: aac (LC), 48000 Hz, stereo, fltp, 192 kb/s (default)
2026-02-22T04:46:17.796747Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:     Metadata:
2026-02-22T04:46:17.797054Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:       encoder         : Lavc62.11.100 aac
2026-02-22T04:46:17.797484Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:       BPS             : 640000
2026-02-22T04:46:17.797867Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:       NUMBER_OF_FRAMES: 45912
2026-02-22T04:46:17.798072Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:       NUMBER_OF_BYTES : 117534720
2026-02-22T04:46:17.799144Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:17.799869Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:17.800597Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]:       DURATION        : 00:24:29.208000000
2026-02-22T04:46:17.914083Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]: [hls @ 00000277c0a83b80] Opening 'seg_000.m4s.tmp' for writing
2026-02-22T04:46:17.922591Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]: [hls @ 00000277c0a83b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:17.977079Z  INFO http_request{method=POST uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/seek?start=190.474&playback_session_id=e7437d42-b1c9-4ae1-b3f2-27134056d86a}: ferrite_stream::hls: HLS session 1f2b4e31-b738-4890-9ca7-bc0bf10eb892 ready (first segment generated in 0.5s)
2026-02-22T04:46:17.977778Z  INFO http_request{method=POST uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/seek?start=190.474&playback_session_id=e7437d42-b1c9-4ae1-b3f2-27134056d86a}: ferrite_api::handlers::stream: HLS seek for 5dbce626-0946-435a-aa93-f7837292a783: start=190.5s mode=fast seek_source=requested db=1ms seek=1ms session=593ms total=596ms
2026-02-22T04:46:17.991538Z  INFO http_request{method=GET uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/master.m3u8?start=190.474&playback_session_id=e7437d42%2Db1c9%2D4ae1%2Db3f2%2D27134056d86a}: ferrite_api::handlers::stream: HLS master playlist for 5dbce626-0946-435a-aa93-f7837292a783: variants=1 reused=true mode=fast seek_source=requested db=1ms seek=0ms session=0ms total=2ms
2026-02-22T04:46:18.069323Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]: frame=   97 fp[hls @ 00000277c0a83b80] Opening 'seg_001.m4s.tmp' for writing elapsed=0:00:00.52
2026-02-22T04:46:18.090248Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]: [hls @ 00000277c0a83b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:18.215678Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]: [hls @ 00000277c0a83b80] Opening 'seg_002.m4s.tmp' for writing
2026-02-22T04:46:18.225735Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]: [hls @ 00000277c0a83b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:18.349231Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]: [hls @ 00000277c0a83b80] Opening 'seg_003.m4s.tmp' for writing
2026-02-22T04:46:18.358466Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]: [hls @ 00000277c0a83b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:18.469733Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]: [hls @ 00000277c0a83b80] Opening 'seg_004.m4s.tmp' for writing
2026-02-22T04:46:18.478915Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]: [hls @ 00000277c0a83b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:18.585301Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]: frame=  287 fp[hls @ 00000277c0a83b80] Opening 'seg_005.m4s.tmp' for writing elapsed=0:00:01.02
2026-02-22T04:46:18.595305Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]: [hls @ 00000277c0a83b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:18.687176Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]: [hls @ 00000277c0a83b80] Opening 'seg_006.m4s.tmp' for writing
2026-02-22T04:46:18.696102Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]: [hls @ 00000277c0a83b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:18.798163Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]: [hls @ 00000277c0a83b80] Opening 'seg_007.m4s.tmp' for writing
2026-02-22T04:46:18.807547Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]: [hls @ 00000277c0a83b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:18.900625Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]: [hls @ 00000277c0a83b80] Opening 'seg_008.m4s.tmp' for writing
2026-02-22T04:46:18.909774Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]: [hls @ 00000277c0a83b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:19.001788Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]: [hls @ 00000277c0a83b80] Opening 'seg_009.m4s.tmp' for writing
2026-02-22T04:46:19.010464Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]: [hls @ 00000277c0a83b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:19.101293Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]: frame=  527 fp[hls @ 00000277c0a83b80] Opening 'seg_010.m4s.tmp' for writing elapsed=0:00:01.55
2026-02-22T04:46:19.112670Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]: [hls @ 00000277c0a83b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:19.202669Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]: [hls @ 00000277c0a83b80] Opening 'seg_011.m4s.tmp' for writing
2026-02-22T04:46:19.214145Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]: [hls @ 00000277c0a83b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:19.305239Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]: [hls @ 00000277c0a83b80] Opening 'seg_012.m4s.tmp' for writing
2026-02-22T04:46:19.315859Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]: [hls @ 00000277c0a83b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:19.407134Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]: [hls @ 00000277c0a83b80] Opening 'seg_013.m4s.tmp' for writing
2026-02-22T04:46:19.416448Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]: [hls @ 00000277c0a83b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:19.506048Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]: [hls @ 00000277c0a83b80] Opening 'seg_014.m4s.tmp' for writing
2026-02-22T04:46:19.511294Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]: [hls @ 00000277c0a83b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:19.601023Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]: [hls @ 00000277c0a83b80] Opening 'seg_015.m4s.tmp' for writing
2026-02-22T04:46:19.605772Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]: [hls @ 00000277c0a83b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:19.693025Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]: frame=  769 fp[hls @ 00000277c0a83b80] Opening 'seg_016.m4s.tmp' for writing elapsed=0:00:02.06
2026-02-22T04:46:19.697661Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]: [hls @ 00000277c0a83b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:19.785682Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]: [hls @ 00000277c0a83b80] Opening 'seg_017.m4s.tmp' for writing
2026-02-22T04:46:19.791460Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]: [hls @ 00000277c0a83b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:19.879769Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]: [hls @ 00000277c0a83b80] Opening 'seg_018.m4s.tmp' for writing
2026-02-22T04:46:19.883582Z  WARN ferrite_stream::hls: ffmpeg HLS [1f2b4e31-b738-4890-9ca7-bc0bf10eb892]: [hls @ 00000277c0a83b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:19.989902Z  INFO http_request{method=DELETE uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/session/stop?playback_session_id=e7437d42-b1c9-4ae1-b3f2-27134056d86a}: ferrite_stream::hls: Destroyed HLS session 1f2b4e31-b738-4890-9ca7-bc0bf10eb892
2026-02-22T04:46:22.331980Z  INFO http_request{method=GET uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/master.m3u8?start=192.158&playback_session_id=81e2ed1e%2D740d%2D4dbc%2Da35d%2D8b3ab6ee7b3f}: ferrite_api::handlers::stream: Transcode permit acquired: op=hls-master media_id=5dbce626-0946-435a-aa93-f7837292a783 wait_ms=0.0
2026-02-22T04:46:22.332196Z  INFO http_request{method=GET uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/master.m3u8?start=192.158&playback_session_id=81e2ed1e%2D740d%2D4dbc%2Da35d%2D8b3ab6ee7b3f}: ferrite_stream::hls: Creating 4 ABR variant sessions for media 5dbce626-0946-435a-aa93-f7837292a783 (source=1920x1080)
2026-02-22T04:46:22.333492Z  INFO http_request{method=GET uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/master.m3u8?start=192.158&playback_session_id=81e2ed1e%2D740d%2D4dbc%2Da35d%2D8b3ab6ee7b3f}: ferrite_stream::hls: Creating HLS session 51941939-3e4d-4cad-aaa4-10ae36a6781b for media 5dbce626-0946-435a-aa93-f7837292a783 at 192.2s variant=1080p (C:\Users\ryans\Movies\Star.Trek.Lower.Decks.2020.S05E05.1080p.HEVC.x265-MeGusta.mkv)
2026-02-22T04:46:22.333799Z  INFO http_request{method=GET uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/master.m3u8?start=192.158&playback_session_id=81e2ed1e%2D740d%2D4dbc%2Da35d%2D8b3ab6ee7b3f}: ferrite_stream::hls: 10-bit SDR detected (pix=yuv420p10le, transfer=Some("bt709")), applying bit-depth conversion only
2026-02-22T04:46:22.334067Z  INFO http_request{method=GET uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/master.m3u8?start=192.158&playback_session_id=81e2ed1e%2D740d%2D4dbc%2Da35d%2D8b3ab6ee7b3f}: ferrite_stream::hls: HLS ffmpeg args: ["-hide_banner", "-nostdin", "-ss", "192.158", "-i", "C:\\Users\\ryans\\Movies\\Star.Trek.Lower.Decks.2020.S05E05.1080p.HEVC.x265-MeGusta.mkv", "-ss", "0.000", "-map", "0:v:0", "-map", "0:a:0", "-vf", "format=yuv420p", "-c:v", "h264_nvenc", "-preset", "p4", "-tune", "ll", "-rc", "vbr", "-cq", "23", "-profile:v", "high", "-level", "4.1", "-g", "48", "-keyint_min", "48", "-c:a", "aac", "-b:a", "192k", "-ac", "2", "-f", "hls", "-hls_time", "2", "-hls_list_size", "30", "-hls_segment_type", "fmp4", "-hls_fmp4_init_filename", "init.mp4", "-hls_segment_filename", "seg_%03d.m4s", "-hls_flags", "independent_segments+delete_segments+temp_file", "playlist.m3u8"]
2026-02-22T04:46:22.356267Z  INFO http_request{method=GET uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/master.m3u8?start=192.158&playback_session_id=81e2ed1e%2D740d%2D4dbc%2Da35d%2D8b3ab6ee7b3f}: ferrite_stream::hls: Creating HLS session 0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a for media 5dbce626-0946-435a-aa93-f7837292a783 at 192.2s variant=720p (C:\Users\ryans\Movies\Star.Trek.Lower.Decks.2020.S05E05.1080p.HEVC.x265-MeGusta.mkv)
2026-02-22T04:46:22.356609Z  INFO http_request{method=GET uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/master.m3u8?start=192.158&playback_session_id=81e2ed1e%2D740d%2D4dbc%2Da35d%2D8b3ab6ee7b3f}: ferrite_stream::hls: HLS variant scaling active — falling back to software encoder
2026-02-22T04:46:22.357141Z  INFO http_request{method=GET uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/master.m3u8?start=192.158&playback_session_id=81e2ed1e%2D740d%2D4dbc%2Da35d%2D8b3ab6ee7b3f}: ferrite_stream::hls: 10-bit SDR detected (pix=yuv420p10le, transfer=Some("bt709")), applying bit-depth conversion only
2026-02-22T04:46:22.358195Z  INFO http_request{method=GET uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/master.m3u8?start=192.158&playback_session_id=81e2ed1e%2D740d%2D4dbc%2Da35d%2D8b3ab6ee7b3f}: ferrite_stream::hls: HLS ffmpeg args: ["-hide_banner", "-nostdin", "-ss", "192.158", "-i", "C:\\Users\\ryans\\Movies\\Star.Trek.Lower.Decks.2020.S05E05.1080p.HEVC.x265-MeGusta.mkv", "-ss", "0.000", "-map", "0:v:0", "-map", "0:a:0", "-vf", "format=yuv420p,scale=-2:720", "-c:v", "libx264", "-preset", "veryfast", "-crf", "23", "-profile:v", "high", "-level", "4.1", "-b:v", "2800k", "-maxrate", "4200k", "-bufsize", "5600k", "-g", "48", "-keyint_min", "48", "-c:a", "aac", "-b:a", "128k", "-ac", "2", "-f", "hls", "-hls_time", "2", "-hls_list_size", "30", "-hls_segment_type", "fmp4", "-hls_fmp4_init_filename", "init.mp4", "-hls_segment_filename", "seg_%03d.m4s", "-hls_flags", "independent_segments+delete_segments+temp_file", "playlist.m3u8"]
2026-02-22T04:46:22.384622Z  INFO http_request{method=GET uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/master.m3u8?start=192.158&playback_session_id=81e2ed1e%2D740d%2D4dbc%2Da35d%2D8b3ab6ee7b3f}: ferrite_stream::hls: Creating HLS session 5e396190-60e0-425c-87d3-afff2131b752 for media 5dbce626-0946-435a-aa93-f7837292a783 at 192.2s variant=480p (C:\Users\ryans\Movies\Star.Trek.Lower.Decks.2020.S05E05.1080p.HEVC.x265-MeGusta.mkv)
2026-02-22T04:46:22.385461Z  INFO http_request{method=GET uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/master.m3u8?start=192.158&playback_session_id=81e2ed1e%2D740d%2D4dbc%2Da35d%2D8b3ab6ee7b3f}: ferrite_stream::hls: HLS variant scaling active — falling back to software encoder
2026-02-22T04:46:22.385930Z  INFO http_request{method=GET uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/master.m3u8?start=192.158&playback_session_id=81e2ed1e%2D740d%2D4dbc%2Da35d%2D8b3ab6ee7b3f}: ferrite_stream::hls: 10-bit SDR detected (pix=yuv420p10le, transfer=Some("bt709")), applying bit-depth conversion only
2026-02-22T04:46:22.386501Z  INFO http_request{method=GET uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/master.m3u8?start=192.158&playback_session_id=81e2ed1e%2D740d%2D4dbc%2Da35d%2D8b3ab6ee7b3f}: ferrite_stream::hls: HLS ffmpeg args: ["-hide_banner", "-nostdin", "-ss", "192.158", "-i", "C:\\Users\\ryans\\Movies\\Star.Trek.Lower.Decks.2020.S05E05.1080p.HEVC.x265-MeGusta.mkv", "-ss", "0.000", "-map", "0:v:0", "-map", "0:a:0", "-vf", "format=yuv420p,scale=-2:480", "-c:v", "libx264", "-preset", "veryfast", "-crf", "23", "-profile:v", "high", "-level", "4.1", "-b:v", "1400k", "-maxrate", "2100k", "-bufsize", "2800k", "-g", "48", "-keyint_min", "48", "-c:a", "aac", "-b:a", "128k", "-ac", "2", "-f", "hls", "-hls_time", "2", "-hls_list_size", "30", "-hls_segment_type", "fmp4", "-hls_fmp4_init_filename", "init.mp4", "-hls_segment_filename", "seg_%03d.m4s", "-hls_flags", "independent_segments+delete_segments+temp_file", "playlist.m3u8"]
2026-02-22T04:46:22.395185Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]: Input #0, matroska,webm, from 'C:\Users\ryans\Movies\Star.Trek.Lower.Decks.2020.S05E05.1080p.HEVC.x265-MeGusta.mkv':
2026-02-22T04:46:22.395726Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:   Metadata:
2026-02-22T04:46:22.396336Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:     ENCODER         : Lavf61.5.101
2026-02-22T04:46:22.396700Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:   Duration: 00:24:29.21, start: 0.000000, bitrate: 1428 kb/s
2026-02-22T04:46:22.396942Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:   Stream #0:0: Video: hevc (Main 10), yuv420p10le(tv, bt709, progressive), 1920x1080 [SAR 1:1 DAR 16:9], 23.98 fps, 23.98 tbr, 1k tbn (default)
2026-02-22T04:46:22.397411Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:     Metadata:
2026-02-22T04:46:22.397577Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:       BPS             : 9130526
2026-02-22T04:46:22.398130Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:       NUMBER_OF_FRAMES: 35225
2026-02-22T04:46:22.398266Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:       NUMBER_OF_BYTES : 1676793787
2026-02-22T04:46:22.398385Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:22.398721Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:22.399311Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:       ENCODER         : Lavc61.11.100 libx265
2026-02-22T04:46:22.399459Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:       DURATION        : 00:24:29.176000000
2026-02-22T04:46:22.399594Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:   Stream #0:1(eng): Audio: eac3, 48000 Hz, 5.1(side), fltp, 640 kb/s, start 0.024000 (default)
2026-02-22T04:46:22.399722Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:     Metadata:
2026-02-22T04:46:22.400075Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:       BPS             : 640000
2026-02-22T04:46:22.400449Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:       NUMBER_OF_FRAMES: 45912
2026-02-22T04:46:22.400802Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:       NUMBER_OF_BYTES : 117534720
2026-02-22T04:46:22.401216Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:22.401689Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:22.401990Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:       DURATION        : 00:24:29.208000000
2026-02-22T04:46:22.402155Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:   Stream #0:2(eng): Subtitle: ass (ssa)
2026-02-22T04:46:22.402452Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:     Metadata:
2026-02-22T04:46:22.403369Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:       title           : English [SDH]
2026-02-22T04:46:22.403611Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:       BPS             : 106
2026-02-22T04:46:22.403775Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:       NUMBER_OF_FRAMES: 588
2026-02-22T04:46:22.403929Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:       NUMBER_OF_BYTES : 19142
2026-02-22T04:46:22.404074Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:22.404220Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:22.404570Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:       ENCODER         : Lavc61.11.100 ssa
2026-02-22T04:46:22.405278Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:       DURATION        : 00:24:21.372000000
2026-02-22T04:46:22.406739Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]: Stream mapping:
2026-02-22T04:46:22.406865Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:   Stream #0:0 -> #0:0 (hevc (native) -> h264 (h264_nvenc))
2026-02-22T04:46:22.407084Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:   Stream #0:1 -> #0:1 (eac3 (native) -> aac (native))
2026-02-22T04:46:22.410627Z  INFO http_request{method=GET uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/master.m3u8?start=192.158&playback_session_id=81e2ed1e%2D740d%2D4dbc%2Da35d%2D8b3ab6ee7b3f}: ferrite_stream::hls: Creating HLS session cee7f91c-0d8f-4af5-810a-eb7926783150 for media 5dbce626-0946-435a-aa93-f7837292a783 at 192.2s variant=360p (C:\Users\ryans\Movies\Star.Trek.Lower.Decks.2020.S05E05.1080p.HEVC.x265-MeGusta.mkv)
2026-02-22T04:46:22.410842Z  INFO http_request{method=GET uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/master.m3u8?start=192.158&playback_session_id=81e2ed1e%2D740d%2D4dbc%2Da35d%2D8b3ab6ee7b3f}: ferrite_stream::hls: HLS variant scaling active — falling back to software encoder
2026-02-22T04:46:22.411058Z  INFO http_request{method=GET uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/master.m3u8?start=192.158&playback_session_id=81e2ed1e%2D740d%2D4dbc%2Da35d%2D8b3ab6ee7b3f}: ferrite_stream::hls: 10-bit SDR detected (pix=yuv420p10le, transfer=Some("bt709")), applying bit-depth conversion only
2026-02-22T04:46:22.411351Z  INFO http_request{method=GET uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/master.m3u8?start=192.158&playback_session_id=81e2ed1e%2D740d%2D4dbc%2Da35d%2D8b3ab6ee7b3f}: ferrite_stream::hls: HLS ffmpeg args: ["-hide_banner", "-nostdin", "-ss", "192.158", "-i", "C:\\Users\\ryans\\Movies\\Star.Trek.Lower.Decks.2020.S05E05.1080p.HEVC.x265-MeGusta.mkv", "-ss", "0.000", "-map", "0:v:0", "-map", "0:a:0", "-vf", "format=yuv420p,scale=-2:360", "-c:v", "libx264", "-preset", "veryfast", "-crf", "23", "-profile:v", "high", "-level", "4.1", "-b:v", "800k", "-maxrate", "1200k", "-bufsize", "1600k", "-g", "48", "-keyint_min", "48", "-c:a", "aac", "-b:a", "96k", "-ac", "2", "-f", "hls", "-hls_time", "2", "-hls_list_size", "30", "-hls_segment_type", "fmp4", "-hls_fmp4_init_filename", "init.mp4", "-hls_segment_filename", "seg_%03d.m4s", "-hls_flags", "independent_segments+delete_segments+temp_file", "playlist.m3u8"]
2026-02-22T04:46:22.427337Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]: Input #0, matroska,webm, from 'C:\Users\ryans\Movies\Star.Trek.Lower.Decks.2020.S05E05.1080p.HEVC.x265-MeGusta.mkv':
2026-02-22T04:46:22.427783Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:   Metadata:
2026-02-22T04:46:22.428081Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:     ENCODER         : Lavf61.5.101
2026-02-22T04:46:22.428586Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:   Duration: 00:24:29.21, start: 0.000000, bitrate: 1428 kb/s
2026-02-22T04:46:22.428752Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:   Stream #0:0: Video: hevc (Main 10), yuv420p10le(tv, bt709, progressive), 1920x1080 [SAR 1:1 DAR 16:9], 23.98 fps, 23.98 tbr, 1k tbn (default)
2026-02-22T04:46:22.429108Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:     Metadata:
2026-02-22T04:46:22.429331Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:       BPS             : 9130526
2026-02-22T04:46:22.429704Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:       NUMBER_OF_FRAMES: 35225
2026-02-22T04:46:22.429851Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:       NUMBER_OF_BYTES : 1676793787
2026-02-22T04:46:22.430153Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:22.430536Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:22.430730Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:       ENCODER         : Lavc61.11.100 libx265
2026-02-22T04:46:22.430962Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:       DURATION        : 00:24:29.176000000
2026-02-22T04:46:22.431566Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:   Stream #0:1(eng): Audio: eac3, 48000 Hz, 5.1(side), fltp, 640 kb/s, start 0.024000 (default)
2026-02-22T04:46:22.432018Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:     Metadata:
2026-02-22T04:46:22.432252Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:       BPS             : 640000
2026-02-22T04:46:22.432623Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:       NUMBER_OF_FRAMES: 45912
2026-02-22T04:46:22.432981Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:       NUMBER_OF_BYTES : 117534720
2026-02-22T04:46:22.433106Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:22.433222Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:22.433618Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:       DURATION        : 00:24:29.208000000
2026-02-22T04:46:22.433926Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:   Stream #0:2(eng): Subtitle: ass (ssa)
2026-02-22T04:46:22.434248Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:     Metadata:
2026-02-22T04:46:22.434651Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:       title           : English [SDH]
2026-02-22T04:46:22.434940Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:       BPS             : 106
2026-02-22T04:46:22.435177Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:       NUMBER_OF_FRAMES: 588
2026-02-22T04:46:22.436332Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:       NUMBER_OF_BYTES : 19142
2026-02-22T04:46:22.436523Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:22.436734Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:22.436966Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:       ENCODER         : Lavc61.11.100 ssa
2026-02-22T04:46:22.437407Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:       DURATION        : 00:24:21.372000000
2026-02-22T04:46:22.437584Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]: Stream mapping:
2026-02-22T04:46:22.437767Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:   Stream #0:0 -> #0:0 (hevc (native) -> h264 (libx264))
2026-02-22T04:46:22.437931Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:   Stream #0:1 -> #0:1 (eac3 (native) -> aac (native))
2026-02-22T04:46:22.467802Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]: Input #0, matroska,webm, from 'C:\Users\ryans\Movies\Star.Trek.Lower.Decks.2020.S05E05.1080p.HEVC.x265-MeGusta.mkv':
2026-02-22T04:46:22.468794Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:   Metadata:
2026-02-22T04:46:22.469685Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:     ENCODER         : Lavf61.5.101
2026-02-22T04:46:22.470734Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:   Duration: 00:24:29.21, start: 0.000000, bitrate: 1428 kb/s
2026-02-22T04:46:22.471840Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:   Stream #0:0: Video: hevc (Main 10), yuv420p10le(tv, bt709, progressive), 1920x1080 [SAR 1:1 DAR 16:9], 23.98 fps, 23.98 tbr, 1k tbn (default)
2026-02-22T04:46:22.472431Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:     Metadata:
2026-02-22T04:46:22.473754Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:       BPS             : 9130526
2026-02-22T04:46:22.474009Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:       NUMBER_OF_FRAMES: 35225
2026-02-22T04:46:22.474245Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:       NUMBER_OF_BYTES : 1676793787
2026-02-22T04:46:22.474784Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:22.475389Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:22.476286Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:       ENCODER         : Lavc61.11.100 libx265
2026-02-22T04:46:22.476873Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:       DURATION        : 00:24:29.176000000
2026-02-22T04:46:22.477683Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:   Stream #0:1(eng): Audio: eac3, 48000 Hz, 5.1(side), fltp, 640 kb/s, start 0.024000 (default)
2026-02-22T04:46:22.478736Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:     Metadata:
2026-02-22T04:46:22.479009Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:       BPS             : 640000
2026-02-22T04:46:22.479511Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:       NUMBER_OF_FRAMES: 45912
2026-02-22T04:46:22.480123Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:       NUMBER_OF_BYTES : 117534720
2026-02-22T04:46:22.480908Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:22.482933Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:22.483935Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:       DURATION        : 00:24:29.208000000
2026-02-22T04:46:22.484317Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:   Stream #0:2(eng): Subtitle: ass (ssa)
2026-02-22T04:46:22.484594Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:     Metadata:
2026-02-22T04:46:22.484920Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:       title           : English [SDH]
2026-02-22T04:46:22.485197Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:       BPS             : 106
2026-02-22T04:46:22.485871Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:       NUMBER_OF_FRAMES: 588
2026-02-22T04:46:22.486165Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:       NUMBER_OF_BYTES : 19142
2026-02-22T04:46:22.487131Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:22.487469Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:22.487683Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:       ENCODER         : Lavc61.11.100 ssa
2026-02-22T04:46:22.488459Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:       DURATION        : 00:24:21.372000000
2026-02-22T04:46:22.489408Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]: Stream mapping:
2026-02-22T04:46:22.489831Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:   Stream #0:0 -> #0:0 (hevc (native) -> h264 (libx264))
2026-02-22T04:46:22.490069Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:   Stream #0:1 -> #0:1 (eac3 (native) -> aac (native))
2026-02-22T04:46:22.662765Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]: Input #0, matroska,webm, from 'C:\Users\ryans\Movies\Star.Trek.Lower.Decks.2020.S05E05.1080p.HEVC.x265-MeGusta.mkv':
2026-02-22T04:46:22.663476Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:   Metadata:
2026-02-22T04:46:22.663904Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:     ENCODER         : Lavf61.5.101
2026-02-22T04:46:22.664925Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:   Duration: 00:24:29.21, start: 0.000000, bitrate: 1428 kb/s
2026-02-22T04:46:22.666911Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:   Stream #0:0: Video: hevc (Main 10), yuv420p10le(tv, bt709, progressive), 1920x1080 [SAR 1:1 DAR 16:9], 23.98 fps, 23.98 tbr, 1k tbn (default)
2026-02-22T04:46:22.676940Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:     Metadata:
2026-02-22T04:46:22.678155Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:       BPS             : 9130526
2026-02-22T04:46:22.679979Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:       NUMBER_OF_FRAMES: 35225
2026-02-22T04:46:22.680424Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:       NUMBER_OF_BYTES : 1676793787
2026-02-22T04:46:22.680846Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:22.681425Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:22.682389Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:       ENCODER         : Lavc61.11.100 libx265
2026-02-22T04:46:22.682789Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:       DURATION        : 00:24:29.176000000
2026-02-22T04:46:22.683097Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:   Stream #0:1(eng): Audio: eac3, 48000 Hz, 5.1(side), fltp, 640 kb/s, start 0.024000 (default)
2026-02-22T04:46:22.683904Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:     Metadata:
2026-02-22T04:46:22.684421Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:       BPS             : 640000
2026-02-22T04:46:22.684829Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:       NUMBER_OF_FRAMES: 45912
2026-02-22T04:46:22.685176Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:       NUMBER_OF_BYTES : 117534720
2026-02-22T04:46:22.685600Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:22.685937Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:22.686454Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:       DURATION        : 00:24:29.208000000
2026-02-22T04:46:22.686728Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:   Stream #0:2(eng): Subtitle: ass (ssa)
2026-02-22T04:46:22.687218Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:     Metadata:
2026-02-22T04:46:22.687854Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:       title           : English [SDH]
2026-02-22T04:46:22.688316Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:       BPS             : 106
2026-02-22T04:46:22.689238Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:       NUMBER_OF_FRAMES: 588
2026-02-22T04:46:22.690151Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:       NUMBER_OF_BYTES : 19142
2026-02-22T04:46:22.690844Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:22.691240Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:22.691554Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:       ENCODER         : Lavc61.11.100 ssa
2026-02-22T04:46:22.692556Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:       DURATION        : 00:24:21.372000000
2026-02-22T04:46:22.693530Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]: Stream mapping:
2026-02-22T04:46:22.694778Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:   Stream #0:0 -> #0:0 (hevc (native) -> h264 (libx264))
2026-02-22T04:46:22.695109Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:   Stream #0:1 -> #0:1 (eac3 (native) -> aac (native))
2026-02-22T04:46:22.706845Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]: [libx264 @ 0000020cd8a9fac0] using SAR=1/1
2026-02-22T04:46:22.708074Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]: [libx264 @ 0000020cd8a9fac0] using cpu capabilities: MMX2 SSE2Fast SSSE3 SSE4.2 AVX FMA3 BMI2 AVX2
2026-02-22T04:46:22.715707Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]: [libx264 @ 0000020cd8a9fac0] profile High, level 4.1, 4:2:0, 8-bit
2026-02-22T04:46:22.716777Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]: [libx264 @ 0000020cd8a9fac0] 264 - core 165 r3223 0480cb0 - H.264/MPEG-4 AVC codec - Copyleft 2003-2025 - http://www.videolan.org/x264.html - options: cabac=1 ref=1 deblock=1:0:0 analyse=0x3:0x113 me=hex subme=2 psy=1 psy_rd=1.00:0.00 mixed_ref=0 me_range=16 chroma_me=1 trellis=0 8x8dct=1 cqm=0 deadzone=21,11 fast_pskip=1 chroma_qp_offset=0 threads=22 lookahead_threads=5 sliced_threads=0 nr=0 decimate=1 interlaced=0 bluray_compat=0 constrained_intra=0 bframes=3 b_pyramid=2 b_adapt=1 b_bias=0 direct=1 weightb=1 open_gop=0 weightp=1 keyint=48 keyint_min=25 scenecut=40 intra_refresh=0 rc_lookahead=10 rc=crf mbtree=1 crf=23.0 qcomp=0.60 qpmin=0 qpmax=69 qpstep=4 vbv_maxrate=4200 vbv_bufsize=5600 crf_max=0.0 nal_hrd=none filler=0 ip_ratio=1.40 aq=1:1.00
2026-02-22T04:46:22.717871Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]: [hls @ 0000020cd8a94f40] Opening 'init.mp4' for writing
2026-02-22T04:46:22.718240Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]: Output #0, hls, to 'playlist.m3u8':
2026-02-22T04:46:22.718972Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:   Metadata:
2026-02-22T04:46:22.719352Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:     encoder         : Lavf62.3.100
2026-02-22T04:46:22.719679Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:   Stream #0:0: Video: h264, yuv420p(tv, bt709, progressive), 1280x720 [SAR 1:1 DAR 16:9], q=2-31, 2800 kb/s, 23.98 fps, 24k tbn (default)
2026-02-22T04:46:22.720302Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:     Metadata:
2026-02-22T04:46:22.720558Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:       encoder         : Lavc62.11.100 libx264
2026-02-22T04:46:22.721393Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:       BPS             : 9130526
2026-02-22T04:46:22.722139Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:       NUMBER_OF_FRAMES: 35225
2026-02-22T04:46:22.722488Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:       NUMBER_OF_BYTES : 1676793787
2026-02-22T04:46:22.723261Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:22.723711Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:22.725881Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:       DURATION        : 00:24:29.176000000
2026-02-22T04:46:22.728911Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:     Side data:
2026-02-22T04:46:22.730533Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:       cpb: bitrate max/min/avg: 4200000/0/2800000 buffer size: 5600000 vbv_delay: N/A
2026-02-22T04:46:22.732956Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:   Stream #0:1(eng): Audio: aac (LC), 48000 Hz, stereo, fltp, 128 kb/s (default)
2026-02-22T04:46:22.733386Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:     Metadata:
2026-02-22T04:46:22.735399Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:       encoder         : Lavc62.11.100 aac
2026-02-22T04:46:22.736211Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:       BPS             : 640000
2026-02-22T04:46:22.736706Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:       NUMBER_OF_FRAMES: 45912
2026-02-22T04:46:22.737553Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:       NUMBER_OF_BYTES : 117534720
2026-02-22T04:46:22.738168Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:22.739006Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:22.740365Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]:       DURATION        : 00:24:29.208000000
2026-02-22T04:46:22.829230Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]: [libx264 @ 000002b50527fac0] using SAR=1280/1281
2026-02-22T04:46:22.830023Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]: [libx264 @ 000002b50527fac0] using cpu capabilities: MMX2 SSE2Fast SSSE3 SSE4.2 AVX FMA3 BMI2 AVX2
2026-02-22T04:46:22.836686Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]: [libx264 @ 000002b50527fac0] profile High, level 4.1, 4:2:0, 8-bit
2026-02-22T04:46:22.837581Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]: [libx264 @ 000002b50527fac0] 264 - core 165 r3223 0480cb0 - H.264/MPEG-4 AVC codec - Copyleft 2003-2025 - http://www.videolan.org/x264.html - options: cabac=1 ref=1 deblock=1:0:0 analyse=0x3:0x113 me=hex subme=2 psy=1 psy_rd=1.00:0.00 mixed_ref=0 me_range=16 chroma_me=1 trellis=0 8x8dct=1 cqm=0 deadzone=21,11 fast_pskip=1 chroma_qp_offset=0 threads=15 lookahead_threads=3 sliced_threads=0 nr=0 decimate=1 interlaced=0 bluray_compat=0 constrained_intra=0 bframes=3 b_pyramid=2 b_adapt=1 b_bias=0 direct=1 weightb=1 open_gop=0 weightp=1 keyint=48 keyint_min=25 scenecut=40 intra_refresh=0 rc_lookahead=10 rc=crf mbtree=1 crf=23.0 qcomp=0.60 qpmin=0 qpmax=69 qpstep=4 vbv_maxrate=2100 vbv_bufsize=2800 crf_max=0.0 nal_hrd=none filler=0 ip_ratio=1.40 aq=1:1.00
2026-02-22T04:46:22.838279Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]: [hls @ 000002b505274f40] Opening 'init.mp4' for writing
2026-02-22T04:46:22.838718Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]: Output #0, hls, to 'playlist.m3u8':
2026-02-22T04:46:22.839795Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:   Metadata:
2026-02-22T04:46:22.842534Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:     encoder         : Lavf62.3.100
2026-02-22T04:46:22.847613Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:   Stream #0:0: Video: h264, yuv420p(tv, bt709, progressive), 854x480 [SAR 1280:1281 DAR 16:9], q=2-31, 1400 kb/s, 23.98 fps, 24k tbn (default)
2026-02-22T04:46:22.851779Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:     Metadata:
2026-02-22T04:46:22.855088Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:       encoder         : Lavc62.11.100 libx264
2026-02-22T04:46:22.863273Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:       BPS             : 9130526
2026-02-22T04:46:22.867841Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:       NUMBER_OF_FRAMES: 35225
2026-02-22T04:46:22.868642Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:       NUMBER_OF_BYTES : 1676793787
2026-02-22T04:46:22.869474Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:22.869857Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:22.870233Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:       DURATION        : 00:24:29.176000000
2026-02-22T04:46:22.871119Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:     Side data:
2026-02-22T04:46:22.871469Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:       cpb: bitrate max/min/avg: 2100000/0/1400000 buffer size: 2800000 vbv_delay: N/A
2026-02-22T04:46:22.873524Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:   Stream #0:1(eng): Audio: aac (LC), 48000 Hz, stereo, fltp, 128 kb/s (default)
2026-02-22T04:46:22.875001Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:     Metadata:
2026-02-22T04:46:22.875276Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:       encoder         : Lavc62.11.100 aac
2026-02-22T04:46:22.875487Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:       BPS             : 640000
2026-02-22T04:46:22.875903Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:       NUMBER_OF_FRAMES: 45912
2026-02-22T04:46:22.877116Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:       NUMBER_OF_BYTES : 117534720
2026-02-22T04:46:22.877443Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:22.879011Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:22.879855Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]:       DURATION        : 00:24:29.208000000
2026-02-22T04:46:22.925693Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]: [hls @ 00000271a2e33b80] Opening 'init.mp4' for writing
2026-02-22T04:46:22.927678Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]: Output #0, hls, to 'playlist.m3u8':
2026-02-22T04:46:22.928377Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:   Metadata:
2026-02-22T04:46:22.928886Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:     encoder         : Lavf62.3.100
2026-02-22T04:46:22.929560Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:   Stream #0:0: Video: h264 (High), yuv420p(tv, bt709, progressive), 1920x1080 [SAR 1:1 DAR 16:9], q=2-31, 23.98 fps, 24k tbn (default)
2026-02-22T04:46:22.930004Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:     Metadata:
2026-02-22T04:46:22.933866Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:       encoder         : Lavc62.11.100 h264_nvenc
2026-02-22T04:46:22.934141Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:       BPS             : 9130526
2026-02-22T04:46:22.934349Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:       NUMBER_OF_FRAMES: 35225
2026-02-22T04:46:22.934926Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:       NUMBER_OF_BYTES : 1676793787
2026-02-22T04:46:22.936648Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:22.937254Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:22.937564Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:       DURATION        : 00:24:29.176000000
2026-02-22T04:46:22.937896Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:     Side data:
2026-02-22T04:46:22.938167Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:       cpb: bitrate max/min/avg: 0/0/0 buffer size: 0 vbv_delay: N/A
2026-02-22T04:46:22.938581Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:   Stream #0:1(eng): Audio: aac (LC), 48000 Hz, stereo, fltp, 192 kb/s (default)
2026-02-22T04:46:22.938806Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:     Metadata:
2026-02-22T04:46:22.942238Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:       encoder         : Lavc62.11.100 aac
2026-02-22T04:46:22.942654Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:       BPS             : 640000
2026-02-22T04:46:22.943162Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:       NUMBER_OF_FRAMES: 45912
2026-02-22T04:46:22.943545Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:       NUMBER_OF_BYTES : 117534720
2026-02-22T04:46:22.944337Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:22.955948Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:22.962759Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]:       DURATION        : 00:24:29.208000000
2026-02-22T04:46:23.098216Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]: [libx264 @ 00000295454ffac0] using SAR=1/1
2026-02-22T04:46:23.099063Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]: [libx264 @ 00000295454ffac0] using cpu capabilities: MMX2 SSE2Fast SSSE3 SSE4.2 AVX FMA3 BMI2 AVX2
2026-02-22T04:46:23.103247Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]: [libx264 @ 00000295454ffac0] profile High, level 4.1, 4:2:0, 8-bit
2026-02-22T04:46:23.104660Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]: [libx264 @ 00000295454ffac0] 264 - core 165 r3223 0480cb0 - H.264/MPEG-4 AVC codec - Copyleft 2003-2025 - http://www.videolan.org/x264.html - options: cabac=1 ref=1 deblock=1:0:0 analyse=0x3:0x113 me=hex subme=2 psy=1 psy_rd=1.00:0.00 mixed_ref=0 me_range=16 chroma_me=1 trellis=0 8x8dct=1 cqm=0 deadzone=21,11 fast_pskip=1 chroma_qp_offset=0 threads=11 lookahead_threads=2 sliced_threads=0 nr=0 decimate=1 interlaced=0 bluray_compat=0 constrained_intra=0 bframes=3 b_pyramid=2 b_adapt=1 b_bias=0 direct=1 weightb=1 open_gop=0 weightp=1 keyint=48 keyint_min=25 scenecut=40 intra_refresh=0 rc_lookahead=10 rc=crf mbtree=1 crf=23.0 qcomp=0.60 qpmin=0 qpmax=69 qpstep=4 vbv_maxrate=1200 vbv_bufsize=1600 crf_max=0.0 nal_hrd=none filler=0 ip_ratio=1.40 aq=1:1.00
2026-02-22T04:46:23.105559Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]: [hls @ 00000295454f4f40] Opening 'init.mp4' for writing
2026-02-22T04:46:23.105850Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]: Output #0, hls, to 'playlist.m3u8':
2026-02-22T04:46:23.106697Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:   Metadata:
2026-02-22T04:46:23.107561Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:     encoder         : Lavf62.3.100
2026-02-22T04:46:23.109082Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:   Stream #0:0: Video: h264, yuv420p(tv, bt709, progressive), 640x360 [SAR 1:1 DAR 16:9], q=2-31, 800 kb/s, 23.98 fps, 24k tbn (default)
2026-02-22T04:46:23.109449Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:     Metadata:
2026-02-22T04:46:23.109750Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:       encoder         : Lavc62.11.100 libx264
2026-02-22T04:46:23.110002Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:       BPS             : 9130526
2026-02-22T04:46:23.110226Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:       NUMBER_OF_FRAMES: 35225
2026-02-22T04:46:23.111046Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:       NUMBER_OF_BYTES : 1676793787
2026-02-22T04:46:23.112467Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:23.114234Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:23.115667Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:       DURATION        : 00:24:29.176000000
2026-02-22T04:46:23.116308Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:     Side data:
2026-02-22T04:46:23.116614Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:       cpb: bitrate max/min/avg: 1200000/0/800000 buffer size: 1600000 vbv_delay: N/A
2026-02-22T04:46:23.117130Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:   Stream #0:1(eng): Audio: aac (LC), 48000 Hz, stereo, fltp, 96 kb/s (default)
2026-02-22T04:46:23.119647Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:     Metadata:
2026-02-22T04:46:23.120072Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:       encoder         : Lavc62.11.100 aac
2026-02-22T04:46:23.125518Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:       BPS             : 640000
2026-02-22T04:46:23.129574Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:       NUMBER_OF_FRAMES: 45912
2026-02-22T04:46:23.129984Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:       NUMBER_OF_BYTES : 117534720
2026-02-22T04:46:23.130459Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:       _STATISTICS_WRITING_APP: mkvmerge v84.0 ('Sleeper') 64-bit
2026-02-22T04:46:23.130874Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:       _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-22T04:46:23.133873Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]:       DURATION        : 00:24:29.208000000
2026-02-22T04:46:23.285099Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]: [hls @ 00000271a2e33b80] Opening 'seg_000.m4s.tmp' for writing
2026-02-22T04:46:23.324181Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]: [hls @ 00000271a2e33b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:23.335316Z  INFO http_request{method=GET uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/master.m3u8?start=192.158&playback_session_id=81e2ed1e%2D740d%2D4dbc%2Da35d%2D8b3ab6ee7b3f}: ferrite_stream::hls: HLS session 51941939-3e4d-4cad-aaa4-10ae36a6781b ready (first segment generated in 1.0s)
2026-02-22T04:46:23.472279Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]: frame=    0 fp[hls @ 0000020cd8a94f40] Opening 'seg_000.m4s.tmp' for writing/A elapsed=0:00:00.51
2026-02-22T04:46:23.472297Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]: frame=    0 fp[hls @ 000002b505274f40] Opening 'seg_000.m4s.tmp' for writing/A elapsed=0:00:00.50
2026-02-22T04:46:23.494788Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]: frame=   49 fp[hls @ 0000020cd8a94f40] Opening 'playlist.m3u8.tmp' for writingspeed=1.93x elapsed=0:00:01.03
2026-02-22T04:46:23.495967Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]: [hls @ 000002b505274f40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:23.522029Z  INFO http_request{method=GET uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/master.m3u8?start=192.158&playback_session_id=81e2ed1e%2D740d%2D4dbc%2Da35d%2D8b3ab6ee7b3f}: ferrite_stream::hls: HLS session 0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a ready (first segment generated in 1.1s)
2026-02-22T04:46:23.526102Z  INFO http_request{method=GET uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/master.m3u8?start=192.158&playback_session_id=81e2ed1e%2D740d%2D4dbc%2Da35d%2D8b3ab6ee7b3f}: ferrite_stream::hls: HLS session 5e396190-60e0-425c-87d3-afff2131b752 ready (first segment generated in 1.1s)
2026-02-22T04:46:23.595006Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]: frame=    0 fp[hls @ 00000295454f4f40] Opening 'seg_000.m4s.tmp' for writing/A elapsed=0:00:00.51
2026-02-22T04:46:23.616707Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]: [hls @ 00000295454f4f40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:23.629178Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]: frame=   71 fp[hls @ 00000271a2e33b80] Opening 'seg_001.m4s.tmp' for writing0 speed=2.84x elapsed=0:00:01.04
2026-02-22T04:46:23.643979Z  INFO http_request{method=GET uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/master.m3u8?start=192.158&playback_session_id=81e2ed1e%2D740d%2D4dbc%2Da35d%2D8b3ab6ee7b3f}: ferrite_stream::hls: HLS session cee7f91c-0d8f-4af5-810a-eb7926783150 ready (first segment generated in 1.2s)
2026-02-22T04:46:23.645131Z  INFO http_request{method=GET uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/master.m3u8?start=192.158&playback_session_id=81e2ed1e%2D740d%2D4dbc%2Da35d%2D8b3ab6ee7b3f}: ferrite_api::handlers::stream: HLS master playlist for 5dbce626-0946-435a-aa93-f7837292a783: variants=4 reused=false mode=fast seek_source=requested db=1ms seek=0ms session=1313ms total=1315ms
2026-02-22T04:46:23.656411Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]: [hls @ 00000271a2e33b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:24.022204Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]: frame=   62 fp[hls @ 00000295454f4f40] Opening 'seg_001.m4s.tmp' for writing0 speed=2.43x elapsed=0:00:01.02
2026-02-22T04:46:24.023395Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]: frame=  141 fp[hls @ 00000271a2e33b80] Opening 'seg_002.m4s.tmp' for writing0 speed=3.69x elapsed=0:00:01.55
2026-02-22T04:46:24.052842Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]: [hls @ 00000295454f4f40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:24.055018Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]: [hls @ 00000271a2e33b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:24.251528Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]: frame=   53 fp[hls @ 000002b505274f40] Opening 'seg_001.m4s.tmp' for writing/A dup=1 drop=0 speed=3.05x elapsed=0:00:01.54
2026-02-22T04:46:24.252145Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]: frame=  113 fp[hls @ 0000020cd8a94f40] Opening 'seg_001.m4s.tmp' for writing0 speed=2.98x elapsed=0:00:01.55
2026-02-22T04:46:24.289043Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]: [hls @ 000002b505274f40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:24.291269Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]: [hls @ 0000020cd8a94f40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:24.485947Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]: frame=  191 fp[hls @ 00000271a2e33b80] Opening 'seg_003.m4s.tmp' for writing0 speed=3.85x elapsed=0:00:02.06
2026-02-22T04:46:24.522234Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]: [hls @ 00000271a2e33b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:24.681724Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]: frame=  169 fp[hls @ 0000020cd8a94f40] Opening 'seg_002.m4s.tmp' for writing0 speed=3.38x elapsed=0:00:02.06
2026-02-22T04:46:24.712513Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]: [hls @ 0000020cd8a94f40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:24.718843Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]: frame=  171 fp[hls @ 000002b505274f40] Opening 'seg_002.m4s.tmp' for writing0 speed=3.43x elapsed=0:00:02.05
2026-02-22T04:46:24.737350Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]: [hls @ 000002b505274f40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:24.761400Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]: frame=  120 fp[hls @ 00000295454f4f40] Opening 'seg_002.m4s.tmp' for writing/A dup=1 drop=0 speed=3.74x elapsed=0:00:02.05
2026-02-22T04:46:24.802035Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]: [hls @ 00000295454f4f40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:24.973646Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]: [hls @ 00000271a2e33b80] Opening 'seg_004.m4s.tmp' for writing
2026-02-22T04:46:25.016206Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]: frame=  241 fp[hls @ 00000271a2e33b80] Opening 'playlist.m3u8.tmp' for writingspeed=3.93x elapsed=0:00:02.57
2026-02-22T04:46:25.063732Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]: frame=  231 fp[hls @ 0000020cd8a94f40] Opening 'seg_003.m4s.tmp' for writing0 speed=3.69x elapsed=0:00:02.59
2026-02-22T04:46:25.105688Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]: frame=  228 fp[hls @ 000002b505274f40] Opening 'seg_003.m4s.tmp' for writing0 speed=3.67x elapsed=0:00:02.56
2026-02-22T04:46:25.105982Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]: [hls @ 0000020cd8a94f40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:25.134255Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]: [hls @ 000002b505274f40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:25.216327Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]: [hls @ 00000295454f4f40] Opening 'seg_003.m4s.tmp' for writing
2026-02-22T04:46:25.239493Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]: [hls @ 00000295454f4f40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:25.431578Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]: [hls @ 00000271a2e33b80] Opening 'seg_005.m4s.tmp' for writing
2026-02-22T04:46:25.459545Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]: [hls @ 000002b505274f40] Opening 'seg_004.m4s.tmp' for writing
2026-02-22T04:46:25.504864Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]: frame=  237 fp[hls @ 00000295454f4f40] Opening 'seg_004.m4s.tmp' for writing0 speed=3.84x elapsed=0:00:02.57
2026-02-22T04:46:25.563989Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]: frame=  289 fp[hls @ 00000271a2e33b80] Opening 'playlist.m3u8.tmp' for writingspeed= 3.9x elapsed=0:00:03.11
2026-02-22T04:46:25.572032Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]: [hls @ 00000295454f4f40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:25.578253Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]: [hls @ 000002b505274f40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:25.591542Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]: frame=  284 fp[hls @ 0000020cd8a94f40] Opening 'seg_004.m4s.tmp' for writing0 speed=3.78x elapsed=0:00:03.11
2026-02-22T04:46:25.660680Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]: [hls @ 0000020cd8a94f40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:25.937346Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]: [hls @ 00000271a2e33b80] Opening 'seg_006.m4s.tmp' for writing
2026-02-22T04:46:25.981434Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]: [hls @ 00000271a2e33b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:26.144742Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]: frame=  301 fp[hls @ 00000295454f4f40] Opening 'seg_005.m4s.tmp' for writing0 speed=4.04x elapsed=0:00:03.08
2026-02-22T04:46:26.244627Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]: [hls @ 00000295454f4f40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:26.244613Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]: frame=  285 fp[hls @ 000002b505274f40] Opening 'seg_005.m4s.tmp' for writing/A dup=1 drop=0 speed= 3.7x elapsed=0:00:03.61
2026-02-22T04:46:26.299831Z  WARN ferrite_stream::hls: ffmpeg HLS [5e396190-60e0-425c-87d3-afff2131b752]: [hls @ 000002b505274f40] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:26.342477Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]: frame=  348 fp[hls @ 00000271a2e33b80] Opening 'seg_007.m4s.tmp' for writing0 speed=3.99x elapsed=0:00:03.63
2026-02-22T04:46:26.391556Z  WARN ferrite_stream::hls: ffmpeg HLS [0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a]: frame=  307 fp[hls @ 0000020cd8a94f40] Opening 'seg_005.m4s.tmp' for writing0 speed=3.51x elapsed=0:00:03.62
2026-02-22T04:46:26.402843Z  WARN ferrite_stream::hls: ffmpeg HLS [51941939-3e4d-4cad-aaa4-10ae36a6781b]: [hls @ 00000271a2e33b80] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T04:46:26.425580Z  WARN ferrite_stream::hls: ffmpeg HLS [cee7f91c-0d8f-4af5-810a-eb7926783150]: frame=  335 fps= 93 q=28.0 size=N/A time=00:00:14.05 bitrate=N/A dup=1 drop=0 speed=3.88x elapsed=0:00:03.62
2026-02-22T04:46:26.457602Z  INFO http_request{method=DELETE uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/session/stop?playback_session_id=81e2ed1e-740d-4dbc-a35d-8b3ab6ee7b3f}: ferrite_stream::hls: Destroyed HLS session cee7f91c-0d8f-4af5-810a-eb7926783150
2026-02-22T04:46:26.461555Z  INFO http_request{method=DELETE uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/session/stop?playback_session_id=81e2ed1e-740d-4dbc-a35d-8b3ab6ee7b3f}: ferrite_stream::hls: Destroyed HLS session 5e396190-60e0-425c-87d3-afff2131b752
2026-02-22T04:46:26.472588Z  INFO http_request{method=DELETE uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/session/stop?playback_session_id=81e2ed1e-740d-4dbc-a35d-8b3ab6ee7b3f}: ferrite_stream::hls: Destroyed HLS session 0fc36760-0f3c-4cdc-b7d9-c9390da8ae4a
2026-02-22T04:46:26.538337Z  INFO http_request{method=DELETE uri=/api/stream/5dbce626-0946-435a-aa93-f7837292a783/hls/session/stop?playback_session_id=81e2ed1e-740d-4dbc-a35d-8b3ab6ee7b3f}: ferrite_stream::hls: Destroyed HLS session 51941939-3e4d-4cad-aaa4-10ae36a6781b