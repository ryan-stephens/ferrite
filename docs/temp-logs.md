
badchiefy@charon ~/ferrite $ ./ferrite 2>&1 | tee -a ferrite.log
2026-02-22T05:05:29.805694Z  INFO ferrite: Loaded config from config/ferrite.toml
2026-02-22T05:05:29.805949Z  INFO ferrite_core::config: Data directory: /mnt/mpathj/badchiefy/ferrite/data
2026-02-22T05:05:29.816525Z  INFO ferrite_transcode::hwaccel: HW encoder detection: nvenc=false, qsv=false, vaapi=false
2026-02-22T05:05:29.816544Z  INFO ferrite_transcode::hwaccel: No HW encoders available, using software (libx264)
2026-02-22T05:05:29.816548Z  INFO ferrite: Video encoder: libx264 (backend=software)
2026-02-22T05:05:29.819150Z  INFO ferrite_db: Database connected at /mnt/mpathj/badchiefy/ferrite/data/ferrite.db
2026-02-22T05:05:29.819512Z  INFO ferrite_db: Database migrations applied
2026-02-22T05:05:29.821861Z  INFO ferrite_api::router: Serving SPA from: /mnt/mpathj/badchiefy/ferrite/static
2026-02-22T05:05:29.823211Z  INFO ferrite: DLNA server enabled (Ferrite Media Server)
2026-02-22T05:05:29.823255Z  WARN ferrite: SSDP server stopped (DLNA discovery unavailable): Operation not permitted (os error 1). On Linux, binding port 1900 requires CAP_NET_BIND_SERVICE or running as root.
2026-02-22T05:05:29.844620Z  INFO ferrite_scanner::watcher: Watching 1 library directories for changes
2026-02-22T05:05:29.844745Z  INFO ferrite: Filesystem watcher started
2026-02-22T05:05:29.844753Z  INFO ferrite: Ferrite starting on http://0.0.0.0:12345
2026-02-22T05:05:29.844760Z  INFO ferrite: Open http://localhost:12345 in your browser
2026-02-22T05:05:44.707336Z  INFO http_request{method=GET uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/master.m3u8?token=[REDACTED]&playback_session_id=4a6b450a%2D1400%2D47b9%2Db0c0%2D71a1ced8fcd4}: ferrite_api::handlers::stream: Transcode permit acquired: op=hls-master media_id=ca663e67-38d2-4f47-9008-ed2de1f68a50 wait_ms=0.0
2026-02-22T05:05:44.707380Z  INFO http_request{method=GET uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/master.m3u8?token=[REDACTED]&playback_session_id=4a6b450a%2D1400%2D47b9%2Db0c0%2D71a1ced8fcd4}: ferrite_stream::hls: Creating 4 ABR variant sessions for media ca663e67-38d2-4f47-9008-ed2de1f68a50 (source=1920x1080)
2026-02-22T05:05:44.707773Z  INFO http_request{method=GET uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/master.m3u8?token=[REDACTED]&playback_session_id=4a6b450a%2D1400%2D47b9%2Db0c0%2D71a1ced8fcd4}: ferrite_stream::hls: Creating HLS session 18766485-3d70-4989-94a0-0cdfd785aacc for media ca663e67-38d2-4f47-9008-ed2de1f68a50 at 0.0s variant=1080p (/home/badchiefy/files/tv/Asia (2024)/Season 1/Asia.S01E01.Beneath.the.Waves.1080p.WEBRip.10bit.EAC3.2.0.x265-iVy.mkv)
2026-02-22T05:05:44.707812Z  INFO http_request{method=GET uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/master.m3u8?token=[REDACTED]&playback_session_id=4a6b450a%2D1400%2D47b9%2Db0c0%2D71a1ced8fcd4}: ferrite_stream::hls: 10-bit SDR detected (pix=yuv420p10le, transfer=Some("bt709")), applying bit-depth conversion only
2026-02-22T05:05:44.707839Z  INFO http_request{method=GET uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/master.m3u8?token=[REDACTED]&playback_session_id=4a6b450a%2D1400%2D47b9%2Db0c0%2D71a1ced8fcd4}: ferrite_stream::hls: HLS ffmpeg args: ["-hide_banner", "-nostdin", "-i", "/home/badchiefy/files/tv/Asia (2024)/Season 1/Asia.S01E01.Beneath.the.Waves.1080p.WEBRip.10bit.EAC3.2.0.x265-iVy.mkv", "-map", "0:v:0", "-map", "0:a:0", "-vf", "format=yuv420p", "-c:v", "libx264", "-preset", "veryfast", "-crf", "23", "-profile:v", "high", "-level", "4.1", "-g", "180", "-keyint_min", "180", "-c:a", "aac", "-b:a", "192k", "-ac", "2", "-f", "hls", "-hls_time", "6", "-hls_list_size", "30", "-hls_segment_type", "fmp4", "-hls_fmp4_init_filename", "init.mp4", "-hls_segment_filename", "seg_%03d.m4s", "-hls_flags", "independent_segments+delete_segments+temp_file", "playlist.m3u8"]
2026-02-22T05:05:44.708523Z  INFO http_request{method=GET uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/master.m3u8?token=[REDACTED]&playback_session_id=4a6b450a%2D1400%2D47b9%2Db0c0%2D71a1ced8fcd4}: ferrite_stream::hls: Creating HLS session 4ad4c5ee-cc0e-4d4d-b078-3158e049d70f for media ca663e67-38d2-4f47-9008-ed2de1f68a50 at 0.0s variant=720p (/home/badchiefy/files/tv/Asia (2024)/Season 1/Asia.S01E01.Beneath.the.Waves.1080p.WEBRip.10bit.EAC3.2.0.x265-iVy.mkv)
2026-02-22T05:05:44.708680Z  INFO http_request{method=GET uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/master.m3u8?token=[REDACTED]&playback_session_id=4a6b450a%2D1400%2D47b9%2Db0c0%2D71a1ced8fcd4}: ferrite_stream::hls: 10-bit SDR detected (pix=yuv420p10le, transfer=Some("bt709")), applying bit-depth conversion only
2026-02-22T05:05:44.708812Z  INFO http_request{method=GET uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/master.m3u8?token=[REDACTED]&playback_session_id=4a6b450a%2D1400%2D47b9%2Db0c0%2D71a1ced8fcd4}: ferrite_stream::hls: HLS ffmpeg args: ["-hide_banner", "-nostdin", "-i", "/home/badchiefy/files/tv/Asia (2024)/Season 1/Asia.S01E01.Beneath.the.Waves.1080p.WEBRip.10bit.EAC3.2.0.x265-iVy.mkv", "-map", "0:v:0", "-map", "0:a:0", "-vf", "format=yuv420p,scale=-2:720", "-c:v", "libx264", "-preset", "veryfast", "-crf", "23", "-profile:v", "high", "-level", "4.1", "-b:v", "2800k", "-maxrate", "4200k", "-bufsize", "5600k", "-g", "180", "-keyint_min", "180", "-c:a", "aac", "-b:a", "128k", "-ac", "2", "-f", "hls", "-hls_time", "6", "-hls_list_size", "30", "-hls_segment_type", "fmp4", "-hls_fmp4_init_filename", "init.mp4", "-hls_segment_filename", "seg_%03d.m4s", "-hls_flags", "independent_segments+delete_segments+temp_file", "playlist.m3u8"]
2026-02-22T05:05:44.715662Z  INFO http_request{method=GET uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/master.m3u8?token=[REDACTED]&playback_session_id=4a6b450a%2D1400%2D47b9%2Db0c0%2D71a1ced8fcd4}: ferrite_stream::hls: Creating HLS session 87dd079e-9778-479b-a821-43cc5589274c for media ca663e67-38d2-4f47-9008-ed2de1f68a50 at 0.0s variant=480p (/home/badchiefy/files/tv/Asia (2024)/Season 1/Asia.S01E01.Beneath.the.Waves.1080p.WEBRip.10bit.EAC3.2.0.x265-iVy.mkv)
2026-02-22T05:05:44.715686Z  INFO http_request{method=GET uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/master.m3u8?token=[REDACTED]&playback_session_id=4a6b450a%2D1400%2D47b9%2Db0c0%2D71a1ced8fcd4}: ferrite_stream::hls: 10-bit SDR detected (pix=yuv420p10le, transfer=Some("bt709")), applying bit-depth conversion only
2026-02-22T05:05:44.715711Z  INFO http_request{method=GET uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/master.m3u8?token=[REDACTED]&playback_session_id=4a6b450a%2D1400%2D47b9%2Db0c0%2D71a1ced8fcd4}: ferrite_stream::hls: HLS ffmpeg args: ["-hide_banner", "-nostdin", "-i", "/home/badchiefy/files/tv/Asia (2024)/Season 1/Asia.S01E01.Beneath.the.Waves.1080p.WEBRip.10bit.EAC3.2.0.x265-iVy.mkv", "-map", "0:v:0", "-map", "0:a:0", "-vf", "format=yuv420p,scale=-2:480", "-c:v", "libx264", "-preset", "veryfast", "-crf", "23", "-profile:v", "high", "-level", "4.1", "-b:v", "1400k", "-maxrate", "2100k", "-bufsize", "2800k", "-g", "180", "-keyint_min", "180", "-c:a", "aac", "-b:a", "128k", "-ac", "2", "-f", "hls", "-hls_time", "6", "-hls_list_size", "30", "-hls_segment_type", "fmp4", "-hls_fmp4_init_filename", "init.mp4", "-hls_segment_filename", "seg_%03d.m4s", "-hls_flags", "independent_segments+delete_segments+temp_file", "playlist.m3u8"]
2026-02-22T05:05:44.720862Z  INFO http_request{method=GET uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/master.m3u8?token=[REDACTED]&playback_session_id=4a6b450a%2D1400%2D47b9%2Db0c0%2D71a1ced8fcd4}: ferrite_stream::hls: Creating HLS session 7890035a-d09f-48e6-b100-3ef031e401fe for media ca663e67-38d2-4f47-9008-ed2de1f68a50 at 0.0s variant=360p (/home/badchiefy/files/tv/Asia (2024)/Season 1/Asia.S01E01.Beneath.the.Waves.1080p.WEBRip.10bit.EAC3.2.0.x265-iVy.mkv)
2026-02-22T05:05:44.720949Z  INFO http_request{method=GET uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/master.m3u8?token=[REDACTED]&playback_session_id=4a6b450a%2D1400%2D47b9%2Db0c0%2D71a1ced8fcd4}: ferrite_stream::hls: 10-bit SDR detected (pix=yuv420p10le, transfer=Some("bt709")), applying bit-depth conversion only
2026-02-22T05:05:44.721054Z  INFO http_request{method=GET uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/master.m3u8?token=[REDACTED]&playback_session_id=4a6b450a%2D1400%2D47b9%2Db0c0%2D71a1ced8fcd4}: ferrite_stream::hls: HLS ffmpeg args: ["-hide_banner", "-nostdin", "-i", "/home/badchiefy/files/tv/Asia (2024)/Season 1/Asia.S01E01.Beneath.the.Waves.1080p.WEBRip.10bit.EAC3.2.0.x265-iVy.mkv", "-map", "0:v:0", "-map", "0:a:0", "-vf", "format=yuv420p,scale=-2:360", "-c:v", "libx264", "-preset", "veryfast", "-crf", "23", "-profile:v", "high", "-level", "4.1", "-b:v", "800k", "-maxrate", "1200k", "-bufsize", "1600k", "-g", "180", "-keyint_min", "180", "-c:a", "aac", "-b:a", "96k", "-ac", "2", "-f", "hls", "-hls_time", "6", "-hls_list_size", "30", "-hls_segment_type", "fmp4", "-hls_fmp4_init_filename", "init.mp4", "-hls_segment_filename", "seg_%03d.m4s", "-hls_flags", "independent_segments+delete_segments+temp_file", "playlist.m3u8"]
2026-02-22T05:05:44.722388Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]: Input #0, matroska,webm, from '/home/badchiefy/files/tv/Asia (2024)/Season 1/Asia.S01E01.Beneath.the.Waves.1080p.WEBRip.10bit.EAC3.2.0.x265-iVy.mkv':
2026-02-22T05:05:44.722413Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]:   Metadata:
2026-02-22T05:05:44.722426Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]:     creation_time   : 2025-09-05T17:37:40.000000Z
2026-02-22T05:05:44.722441Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]:     ENCODER         : Lavf61.7.100
2026-02-22T05:05:44.722462Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]:   Duration: 00:58:00.05, start: -0.005000, bitrate: 4284 kb/s
2026-02-22T05:05:44.722479Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]:   Stream #0:0: Video: hevc (Main 10), yuv420p10le(tv, bt709), 1920x1080 [SAR 1:1 DAR 16:9], 30 fps, 30 tbr, 1k tbn (default)
2026-02-22T05:05:44.722496Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]:       Metadata:
2026-02-22T05:05:44.722510Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]:         DURATION        : 00:58:00.053000000
2026-02-22T05:05:44.722524Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]:   Stream #0:1(eng): Audio: eac3, 48000 Hz, stereo, fltp, 640 kb/s (default)
2026-02-22T05:05:44.722540Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]:       Metadata:
2026-02-22T05:05:44.722553Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]:         DURATION        : 00:58:00.000000000
2026-02-22T05:05:44.722567Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]:   Stream #0:2(eng): Subtitle: ass (ssa)
2026-02-22T05:05:44.722582Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]:       Metadata:
2026-02-22T05:05:44.722596Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]:         DURATION        : 00:57:24.800000000
2026-02-22T05:05:44.722611Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]:   Stream #0:3(eng): Subtitle: ass (ssa)
2026-02-22T05:05:44.722632Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]:       Metadata:
2026-02-22T05:05:44.722646Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]:         title           : SDH
2026-02-22T05:05:44.722660Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]:         DURATION        : 00:57:24.800000000
2026-02-22T05:05:44.726568Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]: Stream mapping:
2026-02-22T05:05:44.726633Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]:   Stream #0:0 -> #0:0 (hevc (native) -> h264 (libx264))
2026-02-22T05:05:44.726667Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]:   Stream #0:1 -> #0:1 (eac3 (native) -> aac (native))
2026-02-22T05:05:44.730032Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]: Input #0, matroska,webm, from '/home/badchiefy/files/tv/Asia (2024)/Season 1/Asia.S01E01.Beneath.the.Waves.1080p.WEBRip.10bit.EAC3.2.0.x265-iVy.mkv':
2026-02-22T05:05:44.730046Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]:   Metadata:
2026-02-22T05:05:44.730052Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]:     creation_time   : 2025-09-05T17:37:40.000000Z
2026-02-22T05:05:44.730062Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]:     ENCODER         : Lavf61.7.100
2026-02-22T05:05:44.730080Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]:   Duration: 00:58:00.05, start: -0.005000, bitrate: 4284 kb/s
2026-02-22T05:05:44.730091Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]:   Stream #0:0: Video: hevc (Main 10), yuv420p10le(tv, bt709), 1920x1080 [SAR 1:1 DAR 16:9], 30 fps, 30 tbr, 1k tbn (default)
2026-02-22T05:05:44.730099Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]:       Metadata:
2026-02-22T05:05:44.730106Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]:         DURATION        : 00:58:00.053000000
2026-02-22T05:05:44.730114Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]:   Stream #0:1(eng): Audio: eac3, 48000 Hz, stereo, fltp, 640 kb/s (default)
2026-02-22T05:05:44.730124Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]:       Metadata:
2026-02-22T05:05:44.730139Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]:         DURATION        : 00:58:00.000000000
2026-02-22T05:05:44.730149Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]:   Stream #0:2(eng): Subtitle: ass (ssa)
2026-02-22T05:05:44.730155Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]:       Metadata:
2026-02-22T05:05:44.730160Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]:         DURATION        : 00:57:24.800000000
2026-02-22T05:05:44.730167Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]:   Stream #0:3(eng): Subtitle: ass (ssa)
2026-02-22T05:05:44.730176Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]:       Metadata:
2026-02-22T05:05:44.730183Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]:         title           : SDH
2026-02-22T05:05:44.730189Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]:         DURATION        : 00:57:24.800000000
2026-02-22T05:05:44.732048Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]: Input #0, matroska,webm, from '/home/badchiefy/files/tv/Asia (2024)/Season 1/Asia.S01E01.Beneath.the.Waves.1080p.WEBRip.10bit.EAC3.2.0.x265-iVy.mkv':
2026-02-22T05:05:44.732060Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]:   Metadata:
2026-02-22T05:05:44.732068Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]:     creation_time   : 2025-09-05T17:37:40.000000Z
2026-02-22T05:05:44.732077Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]:     ENCODER         : Lavf61.7.100
2026-02-22T05:05:44.732094Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]:   Duration: 00:58:00.05, start: -0.005000, bitrate: 4284 kb/s
2026-02-22T05:05:44.732103Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]:   Stream #0:0: Video: hevc (Main 10), yuv420p10le(tv, bt709), 1920x1080 [SAR 1:1 DAR 16:9], 30 fps, 30 tbr, 1k tbn (default)
2026-02-22T05:05:44.732111Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]:       Metadata:
2026-02-22T05:05:44.732114Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]:         DURATION        : 00:58:00.053000000
2026-02-22T05:05:44.732117Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]:   Stream #0:1(eng): Audio: eac3, 48000 Hz, stereo, fltp, 640 kb/s (default)
2026-02-22T05:05:44.732121Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]:       Metadata:
2026-02-22T05:05:44.732129Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]:         DURATION        : 00:58:00.000000000
2026-02-22T05:05:44.732134Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]:   Stream #0:2(eng): Subtitle: ass (ssa)
2026-02-22T05:05:44.732137Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]:       Metadata:
2026-02-22T05:05:44.732141Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]:         DURATION        : 00:57:24.800000000
2026-02-22T05:05:44.732146Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]:   Stream #0:3(eng): Subtitle: ass (ssa)
2026-02-22T05:05:44.732150Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]:       Metadata:
2026-02-22T05:05:44.732154Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]:         title           : SDH
2026-02-22T05:05:44.732156Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]:         DURATION        : 00:57:24.800000000
2026-02-22T05:05:44.733530Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]: Input #0, matroska,webm, from '/home/badchiefy/files/tv/Asia (2024)/Season 1/Asia.S01E01.Beneath.the.Waves.1080p.WEBRip.10bit.EAC3.2.0.x265-iVy.mkv':
2026-02-22T05:05:44.733536Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]:   Metadata:
2026-02-22T05:05:44.733539Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]:     creation_time   : 2025-09-05T17:37:40.000000Z
2026-02-22T05:05:44.733542Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]:     ENCODER         : Lavf61.7.100
2026-02-22T05:05:44.733545Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]:   Duration: 00:58:00.05, start: -0.005000, bitrate: 4284 kb/s
2026-02-22T05:05:44.733548Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]:   Stream #0:0: Video: hevc (Main 10), yuv420p10le(tv, bt709), 1920x1080 [SAR 1:1 DAR 16:9], 30 fps, 30 tbr, 1k tbn (default)
2026-02-22T05:05:44.733552Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]:       Metadata:
2026-02-22T05:05:44.733555Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]:         DURATION        : 00:58:00.053000000
2026-02-22T05:05:44.733557Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]:   Stream #0:1(eng): Audio: eac3, 48000 Hz, stereo, fltp, 640 kb/s (default)
2026-02-22T05:05:44.733561Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]:       Metadata:
2026-02-22T05:05:44.733563Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]:         DURATION        : 00:58:00.000000000
2026-02-22T05:05:44.733566Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]:   Stream #0:2(eng): Subtitle: ass (ssa)
2026-02-22T05:05:44.733569Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]:       Metadata:
2026-02-22T05:05:44.733571Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]:         DURATION        : 00:57:24.800000000
2026-02-22T05:05:44.733574Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]:   Stream #0:3(eng): Subtitle: ass (ssa)
2026-02-22T05:05:44.733577Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]:       Metadata:
2026-02-22T05:05:44.733580Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]:         title           : SDH
2026-02-22T05:05:44.733583Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]:         DURATION        : 00:57:24.800000000
2026-02-22T05:05:44.734178Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]: Stream mapping:
2026-02-22T05:05:44.734186Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]:   Stream #0:0 -> #0:0 (hevc (native) -> h264 (libx264))
2026-02-22T05:05:44.734194Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]:   Stream #0:1 -> #0:1 (eac3 (native) -> aac (native))
2026-02-22T05:05:44.736386Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]: Stream mapping:
2026-02-22T05:05:44.736409Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]:   Stream #0:0 -> #0:0 (hevc (native) -> h264 (libx264))
2026-02-22T05:05:44.736413Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]:   Stream #0:1 -> #0:1 (eac3 (native) -> aac (native))
2026-02-22T05:05:44.737923Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]: Stream mapping:
2026-02-22T05:05:44.737941Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]:   Stream #0:0 -> #0:0 (hevc (native) -> h264 (libx264))
2026-02-22T05:05:44.737945Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]:   Stream #0:1 -> #0:1 (eac3 (native) -> aac (native))
2026-02-22T05:05:44.823529Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]: [libx264 @ 0x558ceb74c4c0] using SAR=1/1
2026-02-22T05:05:44.823546Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]: [libx264 @ 0x558ceb74c4c0] using cpu capabilities: MMX2 SSE2Fast SSSE3 SSE4.2 AVX FMA3 BMI2 AVX2
2026-02-22T05:05:44.823550Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]: [libx264 @ 0x558ceb74c4c0] profile High, level 4.1, 4:2:0, 8-bit
2026-02-22T05:05:44.823555Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]: [libx264 @ 0x558ceb74c4c0] 264 - core 164 - H.264/MPEG-4 AVC codec - Copyleft 2003-2024 - http://www.videolan.org/x264.html - options: cabac=1 ref=1 deblock=1:0:0 analyse=0x3:0x113 me=hex subme=2 psy=1 psy_rd=1.00:0.00 mixed_ref=0 me_range=16 chroma_me=1 trellis=0 8x8dct=1 cqm=0 deadzone=21,11 fast_pskip=1 chroma_qp_offset=0 threads=12 lookahead_threads=4 sliced_threads=0 nr=0 decimate=1 interlaced=0 bluray_compat=0 constrained_intra=0 bframes=3 b_pyramid=2 b_adapt=1 b_bias=0 direct=1 weightb=1 open_gop=0 weightp=1 keyint=180 keyint_min=91 scenecut=40 intra_refresh=0 rc_lookahead=10 rc=crf mbtree=1 crf=23.0 qcomp=0.60 qpmin=0 qpmax=69 qpstep=4 ip_ratio=1.40 aq=1:1.00
2026-02-22T05:05:44.823565Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]: [hls @ 0x558ceb773340] Opening 'init.mp4' for writing
2026-02-22T05:05:44.823568Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]: Output #0, hls, to 'playlist.m3u8':
2026-02-22T05:05:44.823571Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]:   Metadata:
2026-02-22T05:05:44.823574Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]:     encoder         : Lavf61.7.100
2026-02-22T05:05:44.823580Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]:   Stream #0:0: Video: h264, yuv420p(tv, bt709, progressive), 1920x1080 [SAR 1:1 DAR 16:9], q=2-31, 30 fps, 15360 tbn (default)
2026-02-22T05:05:44.823584Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]:       Metadata:
2026-02-22T05:05:44.823586Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]:         DURATION        : 00:58:00.053000000
2026-02-22T05:05:44.823589Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]:         encoder         : Lavc61.19.101 libx264
2026-02-22T05:05:44.823592Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]:       Side data:
2026-02-22T05:05:44.823594Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]:         cpb: bitrate max/min/avg: 0/0/0 buffer size: 0 vbv_delay: N/A
2026-02-22T05:05:44.823598Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]:   Stream #0:1(eng): Audio: aac (LC), 48000 Hz, stereo, fltp, 192 kb/s (default)
2026-02-22T05:05:44.823601Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]:       Metadata:
2026-02-22T05:05:44.823603Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]:         DURATION        : 00:58:00.000000000
2026-02-22T05:05:44.823606Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]:         encoder         : Lavc61.19.101 aac
2026-02-22T05:05:44.841496Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]: [libx264 @ 0x55f4f3400580] using SAR=1280/1281
2026-02-22T05:05:44.842185Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]: [libx264 @ 0x55f4f3400580] using cpu capabilities: MMX2 SSE2Fast SSSE3 SSE4.2 AVX FMA3 BMI2 AVX2
2026-02-22T05:05:44.846885Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]: [libx264 @ 0x55844040d580] using SAR=1/1
2026-02-22T05:05:44.847067Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]: [libx264 @ 0x55f4f3400580] profile High, level 4.1, 4:2:0, 8-bit
2026-02-22T05:05:44.847146Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]: [libx264 @ 0x55844040d580] using cpu capabilities: MMX2 SSE2Fast SSSE3 SSE4.2 AVX FMA3 BMI2 AVX2
2026-02-22T05:05:44.847187Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]: [libx264 @ 0x55f4f3400580] 264 - core 164 - H.264/MPEG-4 AVC codec - Copyleft 2003-2024 - http://www.videolan.org/x264.html - options: cabac=1 ref=1 deblock=1:0:0 analyse=0x3:0x113 me=hex subme=2 psy=1 psy_rd=1.00:0.00 mixed_ref=0 me_range=16 chroma_me=1 trellis=0 8x8dct=1 cqm=0 deadzone=21,11 fast_pskip=1 chroma_qp_offset=0 threads=12 lookahead_threads=3 sliced_threads=0 nr=0 decimate=1 interlaced=0 bluray_compat=0 constrained_intra=0 bframes=3 b_pyramid=2 b_adapt=1 b_bias=0 direct=1 weightb=1 open_gop=0 weightp=1 keyint=180 keyint_min=91 scenecut=40 intra_refresh=0 rc_lookahead=10 rc=crf mbtree=1 crf=23.0 qcomp=0.60 qpmin=0 qpmax=69 qpstep=4 vbv_maxrate=2100 vbv_bufsize=2800 crf_max=0.0 nal_hrd=none filler=0 ip_ratio=1.40 aq=1:1.00
2026-02-22T05:05:44.847333Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]: [hls @ 0x55f4f34273c0] Opening 'init.mp4' for writing
2026-02-22T05:05:44.847477Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]: Output #0, hls, to 'playlist.m3u8':
2026-02-22T05:05:44.847505Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]:   Metadata:
2026-02-22T05:05:44.847546Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]:     encoder         : Lavf61.7.100
2026-02-22T05:05:44.847661Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]:   Stream #0:0: Video: h264, yuv420p(tv, bt709, progressive), 854x480 [SAR 1280:1281 DAR 16:9], q=2-31, 1400 kb/s, 30 fps, 15360 tbn (default)
2026-02-22T05:05:44.847687Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]:       Metadata:
2026-02-22T05:05:44.847725Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]:         DURATION        : 00:58:00.053000000
2026-02-22T05:05:44.847765Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]:         encoder         : Lavc61.19.101 libx264
2026-02-22T05:05:44.847787Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]:       Side data:
2026-02-22T05:05:44.847844Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]:         cpb: bitrate max/min/avg: 2100000/0/1400000 buffer size: 2800000 vbv_delay: N/A
2026-02-22T05:05:44.847929Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]:   Stream #0:1(eng): Audio: aac (LC), 48000 Hz, stereo, fltp, 128 kb/s (default)
2026-02-22T05:05:44.847953Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]:       Metadata:
2026-02-22T05:05:44.847992Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]:         DURATION        : 00:58:00.000000000
2026-02-22T05:05:44.848031Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]:         encoder         : Lavc61.19.101 aac
2026-02-22T05:05:44.857399Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]: [libx264 @ 0x55844040d580] profile High, level 4.1, 4:2:0, 8-bit
2026-02-22T05:05:44.857421Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]: [libx264 @ 0x55844040d580] 264 - core 164 - H.264/MPEG-4 AVC codec - Copyleft 2003-2024 - http://www.videolan.org/x264.html - options: cabac=1 ref=1 deblock=1:0:0 analyse=0x3:0x113 me=hex subme=2 psy=1 psy_rd=1.00:0.00 mixed_ref=0 me_range=16 chroma_me=1 trellis=0 8x8dct=1 cqm=0 deadzone=21,11 fast_pskip=1 chroma_qp_offset=0 threads=11 lookahead_threads=2 sliced_threads=0 nr=0 decimate=1 interlaced=0 bluray_compat=0 constrained_intra=0 bframes=3 b_pyramid=2 b_adapt=1 b_bias=0 direct=1 weightb=1 open_gop=0 weightp=1 keyint=180 keyint_min=91 scenecut=40 intra_refresh=0 rc_lookahead=10 rc=crf mbtree=1 crf=23.0 qcomp=0.60 qpmin=0 qpmax=69 qpstep=4 vbv_maxrate=1200 vbv_bufsize=1600 crf_max=0.0 nal_hrd=none filler=0 ip_ratio=1.40 aq=1:1.00
2026-02-22T05:05:44.857500Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]: [hls @ 0x5584404343c0] Opening 'init.mp4' for writing
2026-02-22T05:05:44.857865Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]: Output #0, hls, to 'playlist.m3u8':
2026-02-22T05:05:44.857871Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]:   Metadata:
2026-02-22T05:05:44.857874Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]:     encoder         : Lavf61.7.100
2026-02-22T05:05:44.857877Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]:   Stream #0:0: Video: h264, yuv420p(tv, bt709, progressive), 640x360 [SAR 1:1 DAR 16:9], q=2-31, 800 kb/s, 30 fps, 15360 tbn (default)
2026-02-22T05:05:44.857881Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]:       Metadata:
2026-02-22T05:05:44.857884Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]:         DURATION        : 00:58:00.053000000
2026-02-22T05:05:44.857887Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]:         encoder         : Lavc61.19.101 libx264
2026-02-22T05:05:44.857890Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]:       Side data:
2026-02-22T05:05:44.857893Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]:         cpb: bitrate max/min/avg: 1200000/0/800000 buffer size: 1600000 vbv_delay: N/A
2026-02-22T05:05:44.857896Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]:   Stream #0:1(eng): Audio: aac (LC), 48000 Hz, stereo, fltp, 96 kb/s (default)
2026-02-22T05:05:44.857899Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]:       Metadata:
2026-02-22T05:05:44.857902Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]:         DURATION        : 00:58:00.000000000
2026-02-22T05:05:44.857905Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]:         encoder         : Lavc61.19.101 aac
2026-02-22T05:05:44.975594Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]: [libx264 @ 0x55a034710580] using SAR=1/1
2026-02-22T05:05:44.975617Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]: [libx264 @ 0x55a034710580] using cpu capabilities: MMX2 SSE2Fast SSSE3 SSE4.2 AVX FMA3 BMI2 AVX2
2026-02-22T05:05:44.989614Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]: [libx264 @ 0x55a034710580] profile High, level 4.1, 4:2:0, 8-bit
2026-02-22T05:05:44.989636Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]: [libx264 @ 0x55a034710580] 264 - core 164 - H.264/MPEG-4 AVC codec - Copyleft 2003-2024 - http://www.videolan.org/x264.html - options: cabac=1 ref=1 deblock=1:0:0 analyse=0x3:0x113 me=hex subme=2 psy=1 psy_rd=1.00:0.00 mixed_ref=0 me_range=16 chroma_me=1 trellis=0 8x8dct=1 cqm=0 deadzone=21,11 fast_pskip=1 chroma_qp_offset=0 threads=12 lookahead_threads=4 sliced_threads=0 nr=0 decimate=1 interlaced=0 bluray_compat=0 constrained_intra=0 bframes=3 b_pyramid=2 b_adapt=1 b_bias=0 direct=1 weightb=1 open_gop=0 weightp=1 keyint=180 keyint_min=91 scenecut=40 intra_refresh=0 rc_lookahead=10 rc=crf mbtree=1 crf=23.0 qcomp=0.60 qpmin=0 qpmax=69 qpstep=4 vbv_maxrate=4200 vbv_bufsize=5600 crf_max=0.0 nal_hrd=none filler=0 ip_ratio=1.40 aq=1:1.00
2026-02-22T05:05:44.989645Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]: [hls @ 0x55a0347373c0] Opening 'init.mp4' for writing
2026-02-22T05:05:44.989649Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]: Output #0, hls, to 'playlist.m3u8':
2026-02-22T05:05:44.989652Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]:   Metadata:
2026-02-22T05:05:44.989655Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]:     encoder         : Lavf61.7.100
2026-02-22T05:05:44.989658Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]:   Stream #0:0: Video: h264, yuv420p(tv, bt709, progressive), 1280x720 [SAR 1:1 DAR 16:9], q=2-31, 2800 kb/s, 30 fps, 15360 tbn (default)
2026-02-22T05:05:44.989663Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]:       Metadata:
2026-02-22T05:05:44.989665Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]:         DURATION        : 00:58:00.053000000
2026-02-22T05:05:44.989668Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]:         encoder         : Lavc61.19.101 libx264
2026-02-22T05:05:44.989672Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]:       Side data:
2026-02-22T05:05:44.989675Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]:         cpb: bitrate max/min/avg: 4200000/0/2800000 buffer size: 5600000 vbv_delay: N/A
2026-02-22T05:05:44.989679Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]:   Stream #0:1(eng): Audio: aac (LC), 48000 Hz, stereo, fltp, 128 kb/s (default)
2026-02-22T05:05:44.989682Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]:       Metadata:
2026-02-22T05:05:44.989685Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]:         DURATION        : 00:58:00.000000000
2026-02-22T05:05:44.989688Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]:         encoder         : Lavc61.19.101 aac
2026-02-22T05:05:47.574940Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]: frame=    5 fps=0.0 [hls @ 0x55f4f34273c0] Opening 'seg_000.m4s.tmp' for writing=N/A dup=2 drop=0 speed=1.98x
2026-02-22T05:05:47.575574Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]: [hls @ 0x55f4f34273c0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:05:47.588891Z  INFO http_request{method=GET uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/master.m3u8?token=[REDACTED]&playback_session_id=4a6b450a%2D1400%2D47b9%2Db0c0%2D71a1ced8fcd4}: ferrite_stream::hls: HLS session 87dd079e-9778-479b-a821-43cc5589274c ready (first segment generated in 2.9s)
2026-02-22T05:05:47.712807Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]: frame=    0 fps=0.0 [hls @ 0x5584404343c0] Opening 'seg_000.m4s.tmp' for writing=N/A dup=2 drop=0 speed=1.83x
2026-02-22T05:05:47.713696Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]: [hls @ 0x5584404343c0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:05:47.799544Z  INFO http_request{method=GET uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/master.m3u8?token=[REDACTED]&playback_session_id=4a6b450a%2D1400%2D47b9%2Db0c0%2D71a1ced8fcd4}: ferrite_stream::hls: HLS session 7890035a-d09f-48e6-b100-3ef031e401fe ready (first segment generated in 3.1s)
2026-02-22T05:05:48.117862Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]: frame=    0 fps=0.0 [hls @ 0x55a0347373c0] Opening 'seg_000.m4s.tmp' for writing=N/A dup=2 drop=0 speed=1.72x
2026-02-22T05:05:48.117888Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]: [hls @ 0x55a0347373c0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:05:48.197185Z  INFO http_request{method=GET uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/master.m3u8?token=[REDACTED]&playback_session_id=4a6b450a%2D1400%2D47b9%2Db0c0%2D71a1ced8fcd4}: ferrite_stream::hls: HLS session 4ad4c5ee-cc0e-4d4d-b078-3158e049d70f ready (first segment generated in 3.5s)
2026-02-22T05:05:50.224337Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]: frame=    0 fps=0.0 [hls @ 0x558ceb773340] Opening 'seg_000.m4s.tmp' for writing=N/A dup=2 drop=0 speed=1.08x
2026-02-22T05:05:50.224369Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]: [hls @ 0x558ceb773340] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:05:50.331050Z  INFO http_request{method=GET uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/master.m3u8?token=[REDACTED]&playback_session_id=4a6b450a%2D1400%2D47b9%2Db0c0%2D71a1ced8fcd4}: ferrite_stream::hls: HLS session 18766485-3d70-4989-94a0-0cdfd785aacc ready (first segment generated in 5.6s)
2026-02-22T05:05:50.331108Z  INFO http_request{method=GET uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/master.m3u8?token=[REDACTED]&playback_session_id=4a6b450a%2D1400%2D47b9%2Db0c0%2D71a1ced8fcd4}: ferrite_api::handlers::stream: HLS master playlist for ca663e67-38d2-4f47-9008-ed2de1f68a50: variants=4 reused=false mode=fast seek_source=none db=0ms seek=0ms session=5624ms total=5624ms
2026-02-22T05:05:50.601362Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]: frame=  196 fps= 65 [hls @ 0x55f4f34273c0] Opening 'seg_001.m4s.tmp' for writing=N/A dup=4 drop=0 speed=2.37x
2026-02-22T05:05:50.601471Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]: [hls @ 0x55f4f34273c0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:05:50.643825Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]: frame=  182 fps= 60 [hls @ 0x5584404343c0] Opening 'seg_001.m4s.tmp' for writing=N/A dup=4 drop=0 speed=2.34x
2026-02-22T05:05:50.643853Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]: [hls @ 0x5584404343c0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:05:51.743798Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]: frame=  181 fps= 51 [hls @ 0x55a0347373c0] Opening 'seg_001.m4s.tmp' for writing=N/A dup=4 drop=0 speed=2.01x
2026-02-22T05:05:51.744152Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]: [hls @ 0x55a0347373c0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:05:53.391436Z  INFO http_request{method=POST uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/seek?start=700.496&playback_session_id=4a6b450a-1400-47b9-b0c0-71a1ced8fcd4}: ferrite_api::handlers::stream: Transcode permit acquired: op=hls-seek media_id=ca663e67-38d2-4f47-9008-ed2de1f68a50 wait_ms=0.0
2026-02-22T05:05:53.606594Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]: frame=  443 fps= 74 [hls @ 0x55f4f34273c0] Opening 'seg_002.m4s.tmp' for writing=N/A dup=8 drop=0 speed=2.71x
2026-02-22T05:05:53.606900Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]: [hls @ 0x55f4f34273c0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:05:54.094281Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]: frame=  432 fps= 72 [hls @ 0x5584404343c0] Opening 'seg_002.m4s.tmp' for writing=N/A dup=8 drop=0 speed= 2.5x
2026-02-22T05:05:54.096556Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]: [hls @ 0x5584404343c0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:05:54.146163Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]: [hls @ 0x5584404343c0] Opening 'seg_003.m4s.tmp' for writing
2026-02-22T05:05:54.146183Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]: [hls @ 0x5584404343c0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:05:54.146187Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]: [out#0/hls @ 0x558440435540] video:1169KiB audio:288KiB subtitle:0KiB other streams:0KiB global headers:0KiB muxing overhead: unknown
2026-02-22T05:05:54.146192Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]: frame=  736 fps= 78 q=-1.0 Lsize=N/A time=00:00:24.38 bitrate=N/A dup=11 drop=0 speed=2.59x
2026-02-22T05:05:54.158525Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]: [libx264 @ 0x55844040d580] frame I:8     Avg QP:20.50  size: 24356
2026-02-22T05:05:54.158546Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]: [libx264 @ 0x55844040d580] frame P:218   Avg QP:24.26  size:  3484
2026-02-22T05:05:54.158549Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]: [libx264 @ 0x55844040d580] frame B:510   Avg QP:27.88  size:   475
2026-02-22T05:05:54.158552Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]: [libx264 @ 0x55844040d580] consecutive B-frames:  2.3% 10.3% 16.7% 70.7%
2026-02-22T05:05:54.158556Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]: [libx264 @ 0x55844040d580] mb I  I16..4:  7.0% 38.4% 54.6%
2026-02-22T05:05:54.158559Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]: [libx264 @ 0x55844040d580] mb P  I16..4:  3.1%  9.7%  1.9%  P16..4: 25.5% 11.8%  6.1%  0.0%  0.0%    skip:41.9%
2026-02-22T05:05:54.158563Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]: [libx264 @ 0x55844040d580] mb B  I16..4:  0.4%  0.9%  0.0%  B16..8:  8.7%  2.9%  0.3%  direct: 1.4%  skip:85.4%  L0:38.9% L1:43.5% BI:17.6%
2026-02-22T05:05:54.158567Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]: [libx264 @ 0x55844040d580] 8x8 transform intra:61.5% inter:39.9%
2026-02-22T05:05:54.158571Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]: [libx264 @ 0x55844040d580] coded y,uvDC,uvAC intra: 58.6% 54.5% 20.3% inter: 4.9% 3.4% 0.1%
2026-02-22T05:05:54.158575Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]: [libx264 @ 0x55844040d580] i16 v,h,dc,p: 51% 19% 17% 12%
2026-02-22T05:05:54.158578Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]: [libx264 @ 0x55844040d580] i8 v,h,dc,ddl,ddr,vr,hd,vl,hu: 31% 19% 15%  4%  7%  7%  7%  4%  5%
2026-02-22T05:05:54.158582Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]: [libx264 @ 0x55844040d580] i4 v,h,dc,ddl,ddr,vr,hd,vl,hu: 26% 19% 11%  6%  9%  8%  9%  6%  7%
2026-02-22T05:05:54.158585Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]: [libx264 @ 0x55844040d580] i8c dc,h,v,p: 45% 21% 27%  7%
2026-02-22T05:05:54.158588Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]: [libx264 @ 0x55844040d580] Weighted P-Frames: Y:0.0% UV:0.0%
2026-02-22T05:05:54.158591Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]: [libx264 @ 0x55844040d580] kb/s:390.18
2026-02-22T05:05:54.171237Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]: frame=  428 fps= 61 [hls @ 0x55a0347373c0] Opening 'seg_002.m4s.tmp' for writing=N/A dup=5 drop=0 speed=1.95x
2026-02-22T05:05:54.171258Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]: [hls @ 0x55a0347373c0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:05:54.171263Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]: [out#0/hls @ 0x55a034738540] video:2540KiB audio:311KiB subtitle:0KiB other streams:0KiB global headers:0KiB muxing overhead: unknown
2026-02-22T05:05:54.171268Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]: frame=  602 fps= 64 q=-1.0 Lsize=N/A time=00:00:19.84 bitrate=N/A dup=11 drop=0 speed= 2.1x
2026-02-22T05:05:54.174659Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]: [aac @ 0x558440bdeb40] Qavg: 420.952
2026-02-22T05:05:54.175513Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]: [libx264 @ 0x55a034710580] frame I:6     Avg QP:19.52  size: 44654
2026-02-22T05:05:54.185924Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]: [libx264 @ 0x55a034710580] frame P:165   Avg QP:22.88  size:  9674
2026-02-22T05:05:54.185945Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]: [libx264 @ 0x55a034710580] frame B:431   Avg QP:24.72  size:  1708
2026-02-22T05:05:54.185949Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]: [libx264 @ 0x55a034710580] consecutive B-frames:  1.7%  4.3% 13.0% 81.1%
2026-02-22T05:05:54.185953Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]: [libx264 @ 0x55a034710580] mb I  I16..4: 17.7% 52.6% 29.8%
2026-02-22T05:05:54.185957Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]: [libx264 @ 0x55a034710580] mb P  I16..4:  7.5% 15.6%  1.5%  P16..4: 19.7%  8.7%  3.2%  0.0%  0.0%    skip:43.8%
2026-02-22T05:05:54.185961Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]: [libx264 @ 0x55a034710580] mb B  I16..4:  0.9%  1.7%  0.0%  B16..8:  9.0%  2.1%  0.1%  direct: 1.9%  skip:84.2%  L0:40.0% L1:50.0% BI:10.0%
2026-02-22T05:05:54.185965Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]: [libx264 @ 0x55a034710580] 8x8 transform intra:62.3% inter:48.6%
2026-02-22T05:05:54.185969Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]: [libx264 @ 0x55a034710580] coded y,uvDC,uvAC intra: 42.1% 41.0% 9.2% inter: 2.7% 3.6% 0.0%
2026-02-22T05:05:54.185972Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]: [libx264 @ 0x55a034710580] i16 v,h,dc,p: 49% 22% 14% 15%
2026-02-22T05:05:54.185975Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]: [libx264 @ 0x55a034710580] i8 v,h,dc,ddl,ddr,vr,hd,vl,hu: 27% 22% 17%  4%  7%  6%  7%  4%  5%
2026-02-22T05:05:54.185979Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]: [libx264 @ 0x55a034710580] i4 v,h,dc,ddl,ddr,vr,hd,vl,hu: 26% 20% 10%  6%  9%  7%  9%  6%  7%
2026-02-22T05:05:54.185982Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]: [libx264 @ 0x55a034710580] i8c dc,h,v,p: 50% 20% 24%  6%
2026-02-22T05:05:54.185985Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]: [libx264 @ 0x55a034710580] Weighted P-Frames: Y:0.0% UV:0.0%
2026-02-22T05:05:54.185988Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]: [libx264 @ 0x55a034710580] kb/s:1036.62
2026-02-22T05:05:54.202403Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]: [aac @ 0x55a034ee1b40] Qavg: 349.674
2026-02-22T05:05:54.208157Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]: [hls @ 0x55f4f34273c0] Opening 'seg_003.m4s.tmp' for writing
2026-02-22T05:05:54.208725Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]: [hls @ 0x55f4f34273c0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:05:54.208891Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]: [out#0/hls @ 0x55f4f3428540] video:1940KiB audio:399KiB subtitle:0KiB other streams:0KiB global headers:0KiB muxing overhead: unknown
2026-02-22T05:05:54.208901Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]: frame=  767 fps= 81 q=-1.0 Lsize=N/A time=00:00:25.37 bitrate=N/A dup=12 drop=0 speed=2.68x
2026-02-22T05:05:54.215113Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]: [libx264 @ 0x55f4f3400580] frame I:7     Avg QP:20.18  size: 38881
2026-02-22T05:05:54.215127Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]: [libx264 @ 0x55f4f3400580] frame P:222   Avg QP:23.82  size:  5773
2026-02-22T05:05:54.215131Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]: [libx264 @ 0x55f4f3400580] frame B:538   Avg QP:26.63  size:   803
2026-02-22T05:05:54.215134Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]: [libx264 @ 0x55f4f3400580] consecutive B-frames:  1.6% 10.2% 13.7% 74.6%
2026-02-22T05:05:54.215137Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]: [libx264 @ 0x55f4f3400580] mb I  I16..4:  8.0% 39.5% 52.6%
2026-02-22T05:05:54.215140Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]: [libx264 @ 0x55f4f3400580] mb P  I16..4:  4.2% 11.1%  1.8%  P16..4: 23.8% 11.2%  5.8%  0.0%  0.0%    skip:42.1%
2026-02-22T05:05:54.215145Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]: [libx264 @ 0x55f4f3400580] mb B  I16..4:  0.5%  1.1%  0.0%  B16..8:  8.4%  2.6%  0.3%  direct: 1.6%  skip:85.6%  L0:38.3% L1:45.1% BI:16.6%
2026-02-22T05:05:54.215148Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]: [libx264 @ 0x55f4f3400580] 8x8 transform intra:62.0% inter:40.1%
2026-02-22T05:05:54.215152Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]: [libx264 @ 0x55f4f3400580] coded y,uvDC,uvAC intra: 52.6% 48.9% 15.9% inter: 4.4% 3.5% 0.1%
2026-02-22T05:05:54.215156Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]: [libx264 @ 0x55f4f3400580] i16 v,h,dc,p: 50% 21% 16% 14%
2026-02-22T05:05:54.215159Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]: [libx264 @ 0x55f4f3400580] i8 v,h,dc,ddl,ddr,vr,hd,vl,hu: 27% 22% 16%  4%  7%  7%  7%  4%  5%
2026-02-22T05:05:54.215163Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]: [libx264 @ 0x55f4f3400580] i4 v,h,dc,ddl,ddr,vr,hd,vl,hu: 24% 20% 10%  6%  9%  8%  9%  6%  7%
2026-02-22T05:05:54.215166Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]: [libx264 @ 0x55f4f3400580] i8c dc,h,v,p: 47% 21% 26%  7%
2026-02-22T05:05:54.215169Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]: [libx264 @ 0x55f4f3400580] Weighted P-Frames: Y:0.0% UV:0.0%
2026-02-22T05:05:54.215173Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]: [libx264 @ 0x55f4f3400580] kb/s:621.39
2026-02-22T05:05:54.237158Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]: [aac @ 0x55f4f3bd1b40] Qavg: 431.291
2026-02-22T05:05:54.239345Z  WARN ferrite_stream::hls: ffmpeg HLS [4ad4c5ee-cc0e-4d4d-b078-3158e049d70f]: Exiting normally, received signal 15.
2026-02-22T05:05:54.261817Z  WARN ferrite_stream::hls: ffmpeg HLS [7890035a-d09f-48e6-b100-3ef031e401fe]: Exiting normally, received signal 15.
2026-02-22T05:05:54.276994Z  WARN ferrite_stream::hls: ffmpeg HLS [87dd079e-9778-479b-a821-43cc5589274c]: Exiting normally, received signal 15.
2026-02-22T05:05:54.325757Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]: frame=  181 fps= 33 [hls @ 0x558ceb773340] Opening 'seg_001.m4s.tmp' for writing=N/A dup=3 drop=0 speed=1.05x
2026-02-22T05:05:54.326946Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]: [hls @ 0x558ceb773340] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:05:54.327020Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]: [out#0/hls @ 0x558ceb774480] video:2415KiB audio:281KiB subtitle:0KiB other streams:0KiB global headers:0KiB muxing overhead: unknown
2026-02-22T05:05:54.327029Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]: frame=  360 fps= 38 q=-1.0 Lsize=N/A time=00:00:11.90 bitrate=N/A dup=4 drop=0 speed=1.24x
2026-02-22T05:05:54.345849Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]: [libx264 @ 0x558ceb74c4c0] frame I:4     Avg QP:19.22  size: 72714
2026-02-22T05:05:54.345870Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]: [libx264 @ 0x558ceb74c4c0] frame P:96    Avg QP:22.03  size: 15025
2026-02-22T05:05:54.345874Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]: [libx264 @ 0x558ceb74c4c0] frame B:260   Avg QP:23.56  size:  2842
2026-02-22T05:05:54.345877Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]: [libx264 @ 0x558ceb74c4c0] consecutive B-frames:  1.7%  3.9%  6.7% 87.8%
2026-02-22T05:05:54.345881Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]: [libx264 @ 0x558ceb74c4c0] mb I  I16..4:  8.6% 73.1% 18.4%
2026-02-22T05:05:54.345884Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]: [libx264 @ 0x558ceb74c4c0] mb P  I16..4:  5.6% 24.2%  0.6%  P16..4: 14.4%  5.4%  1.8%  0.0%  0.0%    skip:47.9%
2026-02-22T05:05:54.345889Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]: [libx264 @ 0x558ceb74c4c0] mb B  I16..4:  0.6%  2.5%  0.0%  B16..8:  7.8%  1.2%  0.1%  direct: 1.9%  skip:85.9%  L0:41.2% L1:54.0% BI: 4.8%
2026-02-22T05:05:54.345894Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]: [libx264 @ 0x558ceb74c4c0] 8x8 transform intra:78.8% inter:65.5%
2026-02-22T05:05:54.345898Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]: [libx264 @ 0x558ceb74c4c0] coded y,uvDC,uvAC intra: 34.8% 31.3% 3.4% inter: 1.2% 3.4% 0.0%
2026-02-22T05:05:54.345902Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]: [libx264 @ 0x558ceb74c4c0] i16 v,h,dc,p: 56% 19% 12% 14%
2026-02-22T05:05:54.345906Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]: [libx264 @ 0x558ceb74c4c0] i8 v,h,dc,ddl,ddr,vr,hd,vl,hu: 40% 22% 20%  2%  4%  3%  4%  2%  3%
2026-02-22T05:05:54.345910Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]: [libx264 @ 0x558ceb74c4c0] i4 v,h,dc,ddl,ddr,vr,hd,vl,hu: 33% 19%  7%  6%  8%  8%  7%  7%  5%
2026-02-22T05:05:54.345914Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]: [libx264 @ 0x558ceb74c4c0] i8c dc,h,v,p: 54% 18% 25%  3%
2026-02-22T05:05:54.345917Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]: [libx264 @ 0x558ceb74c4c0] Weighted P-Frames: Y:0.0% UV:0.0%
2026-02-22T05:05:54.345922Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]: [libx264 @ 0x558ceb74c4c0] kb/s:1648.13
2026-02-22T05:05:54.381724Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]: [aac @ 0x558cebf1d140] Qavg: 1043.592
2026-02-22T05:05:54.420930Z  WARN ferrite_stream::hls: ffmpeg HLS [18766485-3d70-4989-94a0-0cdfd785aacc]: Exiting normally, received signal 15.
2026-02-22T05:05:55.393294Z  INFO http_request{method=POST uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/seek?start=700.496&playback_session_id=4a6b450a-1400-47b9-b0c0-71a1ced8fcd4}: ferrite_stream::hls: Destroyed HLS session 18766485-3d70-4989-94a0-0cdfd785aacc
2026-02-22T05:05:55.393457Z  INFO http_request{method=POST uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/seek?start=700.496&playback_session_id=4a6b450a-1400-47b9-b0c0-71a1ced8fcd4}: ferrite_stream::hls: Destroyed HLS session 4ad4c5ee-cc0e-4d4d-b078-3158e049d70f
2026-02-22T05:05:55.393611Z  INFO http_request{method=POST uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/seek?start=700.496&playback_session_id=4a6b450a-1400-47b9-b0c0-71a1ced8fcd4}: ferrite_stream::hls: Destroyed HLS session 87dd079e-9778-479b-a821-43cc5589274c
2026-02-22T05:05:55.393892Z  INFO http_request{method=POST uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/seek?start=700.496&playback_session_id=4a6b450a-1400-47b9-b0c0-71a1ced8fcd4}: ferrite_stream::hls: Destroyed HLS session 7890035a-d09f-48e6-b100-3ef031e401fe
2026-02-22T05:05:55.393929Z  INFO http_request{method=POST uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/seek?start=700.496&playback_session_id=4a6b450a-1400-47b9-b0c0-71a1ced8fcd4}: ferrite_stream::hls: Creating single HLS session for seek: media ca663e67-38d2-4f47-9008-ed2de1f68a50 at 698.2s variant=1080p
2026-02-22T05:05:55.394092Z  INFO http_request{method=POST uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/seek?start=700.496&playback_session_id=4a6b450a-1400-47b9-b0c0-71a1ced8fcd4}: ferrite_stream::hls: Creating HLS session 3ad6788f-04fe-412e-9dbc-32d109b7ed09 for media ca663e67-38d2-4f47-9008-ed2de1f68a50 at 698.2s variant=1080p (/home/badchiefy/files/tv/Asia (2024)/Season 1/Asia.S01E01.Beneath.the.Waves.1080p.WEBRip.10bit.EAC3.2.0.x265-iVy.mkv)
2026-02-22T05:05:55.394120Z  INFO http_request{method=POST uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/seek?start=700.496&playback_session_id=4a6b450a-1400-47b9-b0c0-71a1ced8fcd4}: ferrite_stream::hls: 10-bit SDR detected (pix=yuv420p10le, transfer=Some("bt709")), applying bit-depth conversion only
2026-02-22T05:05:55.394153Z  INFO http_request{method=POST uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/seek?start=700.496&playback_session_id=4a6b450a-1400-47b9-b0c0-71a1ced8fcd4}: ferrite_stream::hls: HLS ffmpeg args: ["-hide_banner", "-nostdin", "-ss", "698.167", "-i", "/home/badchiefy/files/tv/Asia (2024)/Season 1/Asia.S01E01.Beneath.the.Waves.1080p.WEBRip.10bit.EAC3.2.0.x265-iVy.mkv", "-ss", "2.329", "-map", "0:v:0", "-map", "0:a:0", "-vf", "format=yuv420p", "-c:v", "libx264", "-preset", "veryfast", "-crf", "23", "-profile:v", "high", "-level", "4.1", "-g", "180", "-keyint_min", "180", "-c:a", "aac", "-b:a", "192k", "-ac", "2", "-f", "hls", "-hls_time", "6", "-hls_list_size", "30", "-hls_segment_type", "fmp4", "-hls_fmp4_init_filename", "init.mp4", "-hls_segment_filename", "seg_%03d.m4s", "-hls_flags", "independent_segments+delete_segments+temp_file", "playlist.m3u8"]
2026-02-22T05:05:55.407545Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]: Input #0, matroska,webm, from '/home/badchiefy/files/tv/Asia (2024)/Season 1/Asia.S01E01.Beneath.the.Waves.1080p.WEBRip.10bit.EAC3.2.0.x265-iVy.mkv':
2026-02-22T05:05:55.407565Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]:   Metadata:
2026-02-22T05:05:55.407568Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]:     creation_time   : 2025-09-05T17:37:40.000000Z
2026-02-22T05:05:55.407572Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]:     ENCODER         : Lavf61.7.100
2026-02-22T05:05:55.407576Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]:   Duration: 00:58:00.05, start: -0.005000, bitrate: 4284 kb/s
2026-02-22T05:05:55.407580Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]:   Stream #0:0: Video: hevc (Main 10), yuv420p10le(tv, bt709), 1920x1080 [SAR 1:1 DAR 16:9], 30 fps, 30 tbr, 1k tbn (default)
2026-02-22T05:05:55.407585Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]:       Metadata:
2026-02-22T05:05:55.407588Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]:         DURATION        : 00:58:00.053000000
2026-02-22T05:05:55.407591Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]:   Stream #0:1(eng): Audio: eac3, 48000 Hz, stereo, fltp, 640 kb/s (default)
2026-02-22T05:05:55.407594Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]:       Metadata:
2026-02-22T05:05:55.407596Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]:         DURATION        : 00:58:00.000000000
2026-02-22T05:05:55.407600Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]:   Stream #0:2(eng): Subtitle: ass (ssa)
2026-02-22T05:05:55.407603Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]:       Metadata:
2026-02-22T05:05:55.407606Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]:         DURATION        : 00:57:24.800000000
2026-02-22T05:05:55.407608Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]:   Stream #0:3(eng): Subtitle: ass (ssa)
2026-02-22T05:05:55.407612Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]:       Metadata:
2026-02-22T05:05:55.407615Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]:         title           : SDH
2026-02-22T05:05:55.407617Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]:         DURATION        : 00:57:24.800000000
2026-02-22T05:05:55.410042Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]: Stream mapping:
2026-02-22T05:05:55.410081Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]:   Stream #0:0 -> #0:0 (hevc (native) -> h264 (libx264))
2026-02-22T05:05:55.410112Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]:   Stream #0:1 -> #0:1 (eac3 (native) -> aac (native))
2026-02-22T05:05:56.101288Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]: [libx264 @ 0x55f3dc88b500] using SAR=1/1
2026-02-22T05:05:56.101848Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]: [libx264 @ 0x55f3dc88b500] using cpu capabilities: MMX2 SSE2Fast SSSE3 SSE4.2 AVX FMA3 BMI2 AVX2
2026-02-22T05:05:56.118255Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]: [libx264 @ 0x55f3dc88b500] profile High, level 4.1, 4:2:0, 8-bit
2026-02-22T05:05:56.118275Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]: [libx264 @ 0x55f3dc88b500] 264 - core 164 - H.264/MPEG-4 AVC codec - Copyleft 2003-2024 - http://www.videolan.org/x264.html - options: cabac=1 ref=1 deblock=1:0:0 analyse=0x3:0x113 me=hex subme=2 psy=1 psy_rd=1.00:0.00 mixed_ref=0 me_range=16 chroma_me=1 trellis=0 8x8dct=1 cqm=0 deadzone=21,11 fast_pskip=1 chroma_qp_offset=0 threads=12 lookahead_threads=4 sliced_threads=0 nr=0 decimate=1 interlaced=0 bluray_compat=0 constrained_intra=0 bframes=3 b_pyramid=2 b_adapt=1 b_bias=0 direct=1 weightb=1 open_gop=0 weightp=1 keyint=180 keyint_min=91 scenecut=40 intra_refresh=0 rc_lookahead=10 rc=crf mbtree=1 crf=23.0 qcomp=0.60 qpmin=0 qpmax=69 qpstep=4 ip_ratio=1.40 aq=1:1.00
2026-02-22T05:05:56.118284Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]: [hls @ 0x55f3dc899340] Opening 'init.mp4' for writing
2026-02-22T05:05:56.118288Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]: Output #0, hls, to 'playlist.m3u8':
2026-02-22T05:05:56.118291Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]:   Metadata:
2026-02-22T05:05:56.118295Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]:     encoder         : Lavf61.7.100
2026-02-22T05:05:56.118301Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]:   Stream #0:0: Video: h264, yuv420p(tv, bt709, progressive), 1920x1080 [SAR 1:1 DAR 16:9], q=2-31, 30 fps, 15360 tbn (default)
2026-02-22T05:05:56.118306Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]:       Metadata:
2026-02-22T05:05:56.118308Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]:         DURATION        : 00:58:00.053000000
2026-02-22T05:05:56.118311Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]:         encoder         : Lavc61.19.101 libx264
2026-02-22T05:05:56.118314Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]:       Side data:
2026-02-22T05:05:56.118317Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]:         cpb: bitrate max/min/avg: 0/0/0 buffer size: 0 vbv_delay: N/A
2026-02-22T05:05:56.118320Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]:   Stream #0:1(eng): Audio: aac (LC), 48000 Hz, stereo, fltp, 192 kb/s (default)
2026-02-22T05:05:56.118323Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]:       Metadata:
2026-02-22T05:05:56.118326Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]:         DURATION        : 00:58:00.000000000
2026-02-22T05:05:56.118330Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]:         encoder         : Lavc61.19.101 aac
2026-02-22T05:05:59.505095Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]: frame=    2 fps=2.0 [hls @ 0x55f3dc899340] Opening 'seg_000.m4s.tmp' for writing=N/A dup=3 drop=0 speed=1.75x
2026-02-22T05:05:59.513686Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]: [hls @ 0x55f3dc899340] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:05:59.588458Z  INFO http_request{method=POST uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/seek?start=700.496&playback_session_id=4a6b450a-1400-47b9-b0c0-71a1ced8fcd4}: ferrite_stream::hls: HLS session 3ad6788f-04fe-412e-9dbc-32d109b7ed09 ready (first segment generated in 4.2s)
2026-02-22T05:05:59.588497Z  INFO http_request{method=POST uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/seek?start=700.496&playback_session_id=4a6b450a-1400-47b9-b0c0-71a1ced8fcd4}: ferrite_api::handlers::stream: HLS seek for ca663e67-38d2-4f47-9008-ed2de1f68a50: start=698.2s mode=fast seek_source=index db=0ms seek=1ms session=6197ms total=6199ms
2026-02-22T05:05:59.658035Z  INFO http_request{method=GET uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/master.m3u8?start=700.496&token=[REDACTED]&playback_session_id=4a6b450a%2D1400%2D47b9%2Db0c0%2D71a1ced8fcd4}: ferrite_api::handlers::stream: HLS master playlist for ca663e67-38d2-4f47-9008-ed2de1f68a50: variants=1 reused=true mode=fast seek_source=index db=1ms seek=0ms session=0ms total=1ms
2026-02-22T05:06:02.006256Z  INFO http_request{method=POST uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/seek?start=1432.653&playback_session_id=4a6b450a-1400-47b9-b0c0-71a1ced8fcd4}: ferrite_api::handlers::stream: Transcode permit acquired: op=hls-seek media_id=ca663e67-38d2-4f47-9008-ed2de1f68a50 wait_ms=0.0
2026-02-22T05:06:02.456516Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]: frame=  251 fps= 56 [hls @ 0x55f3dc899340] Opening 'seg_001.m4s.tmp' for writing=N/A dup=4 drop=0 speed=1.96x
2026-02-22T05:06:02.460444Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]: [hls @ 0x55f3dc899340] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:06:02.910501Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]: [hls @ 0x55f3dc899340] Opening 'seg_002.m4s.tmp' for writing
2026-02-22T05:06:02.912527Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]: [hls @ 0x55f3dc899340] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:06:02.912678Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]: [out#0/hls @ 0x55f3dc8aa300] video:14917KiB audio:387KiB subtitle:0KiB other streams:0KiB global headers:0KiB muxing overhead: unknown
2026-02-22T05:06:02.912701Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]: frame=  490 fps= 65 q=-1.0 Lsize=N/A time=00:00:16.17 bitrate=N/A dup=6 drop=0 speed=2.16x
2026-02-22T05:06:02.917114Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]: [libx264 @ 0x55f3dc88b500] frame I:7     Avg QP:21.96  size: 69481
2026-02-22T05:06:02.917131Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]: [libx264 @ 0x55f3dc88b500] frame P:126   Avg QP:24.76  size: 48334
2026-02-22T05:06:02.917137Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]: [libx264 @ 0x55f3dc88b500] frame B:357   Avg QP:25.79  size: 24363
2026-02-22T05:06:02.917141Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]: [libx264 @ 0x55f3dc88b500] consecutive B-frames:  2.2%  1.2%  1.8% 94.7%
2026-02-22T05:06:02.917144Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]: [libx264 @ 0x55f3dc88b500] mb I  I16..4:  7.5% 79.2% 13.2%
2026-02-22T05:06:02.917155Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]: [libx264 @ 0x55f3dc88b500] mb P  I16..4:  7.0% 72.2%  4.8%  P16..4:  5.8%  3.0%  1.0%  0.0%  0.0%    skip: 6.1%
2026-02-22T05:06:02.917160Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]: [libx264 @ 0x55f3dc88b500] mb B  I16..4:  1.8% 22.9%  0.8%  B16..8: 20.5%  7.3%  0.7%  direct: 6.5%  skip:39.5%  L0:45.2% L1:43.2% BI:11.7%
2026-02-22T05:06:02.917165Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]: [libx264 @ 0x55f3dc88b500] 8x8 transform intra:87.5% inter:62.6%
2026-02-22T05:06:02.917170Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]: [libx264 @ 0x55f3dc88b500] coded y,uvDC,uvAC intra: 60.6% 44.9% 5.0% inter: 8.9% 10.9% 0.0%
2026-02-22T05:06:02.917174Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]: [libx264 @ 0x55f3dc88b500] i16 v,h,dc,p: 31% 26% 25% 18%
2026-02-22T05:06:02.917179Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]: [libx264 @ 0x55f3dc88b500] i8 v,h,dc,ddl,ddr,vr,hd,vl,hu: 18% 23% 18%  5%  8%  6%  9%  4%  8%
2026-02-22T05:06:02.917184Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]: [libx264 @ 0x55f3dc88b500] i4 v,h,dc,ddl,ddr,vr,hd,vl,hu: 15% 18% 11%  6% 15% 10% 12%  6%  6%
2026-02-22T05:06:02.917188Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]: [libx264 @ 0x55f3dc88b500] i8c dc,h,v,p: 53% 22% 18%  7%
2026-02-22T05:06:02.917191Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]: [libx264 @ 0x55f3dc88b500] Weighted P-Frames: Y:0.0% UV:0.0%
2026-02-22T05:06:02.917195Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]: [libx264 @ 0x55f3dc88b500] kb/s:7481.20
2026-02-22T05:06:02.963871Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]: [aac @ 0x55f3dd06a240] Qavg: 616.081
2026-02-22T05:06:02.995461Z  WARN ferrite_stream::hls: ffmpeg HLS [3ad6788f-04fe-412e-9dbc-32d109b7ed09]: Exiting normally, received signal 15.
2026-02-22T05:06:04.009458Z  INFO http_request{method=POST uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/seek?start=1432.653&playback_session_id=4a6b450a-1400-47b9-b0c0-71a1ced8fcd4}: ferrite_stream::hls: Destroyed HLS session 3ad6788f-04fe-412e-9dbc-32d109b7ed09
2026-02-22T05:06:04.009492Z  INFO http_request{method=POST uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/seek?start=1432.653&playback_session_id=4a6b450a-1400-47b9-b0c0-71a1ced8fcd4}: ferrite_stream::hls: Creating single HLS session for seek: media ca663e67-38d2-4f47-9008-ed2de1f68a50 at 1431.8s variant=1080p
2026-02-22T05:06:04.009628Z  INFO http_request{method=POST uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/seek?start=1432.653&playback_session_id=4a6b450a-1400-47b9-b0c0-71a1ced8fcd4}: ferrite_stream::hls: Creating HLS session cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa for media ca663e67-38d2-4f47-9008-ed2de1f68a50 at 1431.8s variant=1080p (/home/badchiefy/files/tv/Asia (2024)/Season 1/Asia.S01E01.Beneath.the.Waves.1080p.WEBRip.10bit.EAC3.2.0.x265-iVy.mkv)
2026-02-22T05:06:04.009650Z  INFO http_request{method=POST uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/seek?start=1432.653&playback_session_id=4a6b450a-1400-47b9-b0c0-71a1ced8fcd4}: ferrite_stream::hls: 10-bit SDR detected (pix=yuv420p10le, transfer=Some("bt709")), applying bit-depth conversion only
2026-02-22T05:06:04.009680Z  INFO http_request{method=POST uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/seek?start=1432.653&playback_session_id=4a6b450a-1400-47b9-b0c0-71a1ced8fcd4}: ferrite_stream::hls: HLS ffmpeg args: ["-hide_banner", "-nostdin", "-ss", "1431.800", "-i", "/home/badchiefy/files/tv/Asia (2024)/Season 1/Asia.S01E01.Beneath.the.Waves.1080p.WEBRip.10bit.EAC3.2.0.x265-iVy.mkv", "-ss", "0.853", "-map", "0:v:0", "-map", "0:a:0", "-vf", "format=yuv420p", "-c:v", "libx264", "-preset", "veryfast", "-crf", "23", "-profile:v", "high", "-level", "4.1", "-g", "180", "-keyint_min", "180", "-c:a", "aac", "-b:a", "192k", "-ac", "2", "-f", "hls", "-hls_time", "6", "-hls_list_size", "30", "-hls_segment_type", "fmp4", "-hls_fmp4_init_filename", "init.mp4", "-hls_segment_filename", "seg_%03d.m4s", "-hls_flags", "independent_segments+delete_segments+temp_file", "playlist.m3u8"]
2026-02-22T05:06:04.023566Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]: Input #0, matroska,webm, from '/home/badchiefy/files/tv/Asia (2024)/Season 1/Asia.S01E01.Beneath.the.Waves.1080p.WEBRip.10bit.EAC3.2.0.x265-iVy.mkv':
2026-02-22T05:06:04.024497Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]:   Metadata:
2026-02-22T05:06:04.024505Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]:     creation_time   : 2025-09-05T17:37:40.000000Z
2026-02-22T05:06:04.024509Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]:     ENCODER         : Lavf61.7.100
2026-02-22T05:06:04.024513Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]:   Duration: 00:58:00.05, start: -0.005000, bitrate: 4284 kb/s
2026-02-22T05:06:04.024518Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]:   Stream #0:0: Video: hevc (Main 10), yuv420p10le(tv, bt709), 1920x1080 [SAR 1:1 DAR 16:9], 30 fps, 30 tbr, 1k tbn (default)
2026-02-22T05:06:04.024522Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]:       Metadata:
2026-02-22T05:06:04.024525Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]:         DURATION        : 00:58:00.053000000
2026-02-22T05:06:04.024529Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]:   Stream #0:1(eng): Audio: eac3, 48000 Hz, stereo, fltp, 640 kb/s (default)
2026-02-22T05:06:04.024533Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]:       Metadata:
2026-02-22T05:06:04.024536Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]:         DURATION        : 00:58:00.000000000
2026-02-22T05:06:04.024539Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]:   Stream #0:2(eng): Subtitle: ass (ssa)
2026-02-22T05:06:04.024542Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]:       Metadata:
2026-02-22T05:06:04.024544Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]:         DURATION        : 00:57:24.800000000
2026-02-22T05:06:04.024547Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]:   Stream #0:3(eng): Subtitle: ass (ssa)
2026-02-22T05:06:04.024550Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]:       Metadata:
2026-02-22T05:06:04.024553Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]:         title           : SDH
2026-02-22T05:06:04.024556Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]:         DURATION        : 00:57:24.800000000
2026-02-22T05:06:04.028039Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]: Stream mapping:
2026-02-22T05:06:04.028082Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]:   Stream #0:0 -> #0:0 (hevc (native) -> h264 (libx264))
2026-02-22T05:06:04.028115Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]:   Stream #0:1 -> #0:1 (eac3 (native) -> aac (native))
2026-02-22T05:06:04.605599Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]: [libx264 @ 0x55e02179d500] using SAR=1/1
2026-02-22T05:06:04.605620Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]: [libx264 @ 0x55e02179d500] using cpu capabilities: MMX2 SSE2Fast SSSE3 SSE4.2 AVX FMA3 BMI2 AVX2
2026-02-22T05:06:04.605625Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]: [libx264 @ 0x55e02179d500] profile High, level 4.1, 4:2:0, 8-bit
2026-02-22T05:06:04.605629Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]: [libx264 @ 0x55e02179d500] 264 - core 164 - H.264/MPEG-4 AVC codec - Copyleft 2003-2024 - http://www.videolan.org/x264.html - options: cabac=1 ref=1 deblock=1:0:0 analyse=0x3:0x113 me=hex subme=2 psy=1 psy_rd=1.00:0.00 mixed_ref=0 me_range=16 chroma_me=1 trellis=0 8x8dct=1 cqm=0 deadzone=21,11 fast_pskip=1 chroma_qp_offset=0 threads=12 lookahead_threads=4 sliced_threads=0 nr=0 decimate=1 interlaced=0 bluray_compat=0 constrained_intra=0 bframes=3 b_pyramid=2 b_adapt=1 b_bias=0 direct=1 weightb=1 open_gop=0 weightp=1 keyint=180 keyint_min=91 scenecut=40 intra_refresh=0 rc_lookahead=10 rc=crf mbtree=1 crf=23.0 qcomp=0.60 qpmin=0 qpmax=69 qpstep=4 ip_ratio=1.40 aq=1:1.00
2026-02-22T05:06:04.605638Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]: [hls @ 0x55e0217ab340] Opening 'init.mp4' for writing
2026-02-22T05:06:04.607457Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]: Output #0, hls, to 'playlist.m3u8':
2026-02-22T05:06:04.607474Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]:   Metadata:
2026-02-22T05:06:04.607507Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]:     encoder         : Lavf61.7.100
2026-02-22T05:06:04.607608Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]:   Stream #0:0: Video: h264, yuv420p(tv, bt709, progressive), 1920x1080 [SAR 1:1 DAR 16:9], q=2-31, 30 fps, 15360 tbn (default)
2026-02-22T05:06:04.607623Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]:       Metadata:
2026-02-22T05:06:04.607653Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]:         DURATION        : 00:58:00.053000000
2026-02-22T05:06:04.607685Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]:         encoder         : Lavc61.19.101 libx264
2026-02-22T05:06:04.607697Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]:       Side data:
2026-02-22T05:06:04.607744Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]:         cpb: bitrate max/min/avg: 0/0/0 buffer size: 0 vbv_delay: N/A
2026-02-22T05:06:04.607812Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]:   Stream #0:1(eng): Audio: aac (LC), 48000 Hz, stereo, fltp, 192 kb/s (default)
2026-02-22T05:06:04.607825Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]:       Metadata:
2026-02-22T05:06:04.607854Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]:         DURATION        : 00:58:00.000000000
2026-02-22T05:06:04.607882Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]:         encoder         : Lavc61.19.101 aac
2026-02-22T05:06:06.242194Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]: frame=   22 fps= 22 [hls @ 0x55e0217ab340] Opening 'seg_000.m4s.tmp' for writing=N/A dup=1 drop=0 speed=2.46x
2026-02-22T05:06:06.244460Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]: [hls @ 0x55e0217ab340] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:06:06.269140Z  INFO http_request{method=POST uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/seek?start=1432.653&playback_session_id=4a6b450a-1400-47b9-b0c0-71a1ced8fcd4}: ferrite_stream::hls: HLS session cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa ready (first segment generated in 2.3s)
2026-02-22T05:06:06.269184Z  INFO http_request{method=POST uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/seek?start=1432.653&playback_session_id=4a6b450a-1400-47b9-b0c0-71a1ced8fcd4}: ferrite_api::handlers::stream: HLS seek for ca663e67-38d2-4f47-9008-ed2de1f68a50: start=1431.8s mode=fast seek_source=index db=0ms seek=0ms session=4263ms total=4263ms
2026-02-22T05:06:06.336746Z  INFO http_request{method=GET uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/master.m3u8?start=1432.653&token=[REDACTED]&playback_session_id=4a6b450a%2D1400%2D47b9%2Db0c0%2D71a1ced8fcd4}: ferrite_api::handlers::stream: HLS master playlist for ca663e67-38d2-4f47-9008-ed2de1f68a50: variants=1 reused=true mode=fast seek_source=index db=0ms seek=0ms session=0ms total=0ms
2026-02-22T05:06:07.756459Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]: frame=  218 fps= 87 [hls @ 0x55e0217ab340] Opening 'seg_001.m4s.tmp' for writing=N/A dup=1 drop=0 speed=3.15x
2026-02-22T05:06:07.757377Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]: [hls @ 0x55e0217ab340] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:06:09.356005Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]: frame=  394 fps= 98 [hls @ 0x55e0217ab340] Opening 'seg_002.m4s.tmp' for writing=N/A dup=1 drop=0 speed=3.36x
2026-02-22T05:06:09.359041Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]: [hls @ 0x55e0217ab340] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:06:09.411251Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]: [hls @ 0x55e0217ab340] Opening 'seg_003.m4s.tmp' for writing
2026-02-22T05:06:09.411600Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]: [hls @ 0x55e0217ab340] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:06:09.411681Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]: [out#0/hls @ 0x55e0217bc300] video:3549KiB audio:442KiB subtitle:0KiB other streams:0KiB global headers:0KiB muxing overhead: unknown
2026-02-22T05:06:09.411691Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]: frame=  561 fps=104 q=-1.0 Lsize=N/A time=00:00:18.51 bitrate=N/A dup=4 drop=0 speed=3.44x
2026-02-22T05:06:09.414746Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]: [libx264 @ 0x55e02179d500] frame I:4     Avg QP:19.01  size:119766
2026-02-22T05:06:09.414751Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]: [libx264 @ 0x55e02179d500] frame P:140   Avg QP:22.13  size: 17006
2026-02-22T05:06:09.414758Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]: [libx264 @ 0x55e02179d500] frame B:417   Avg QP:22.94  size:  1854
2026-02-22T05:06:09.414762Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]: [libx264 @ 0x55e02179d500] consecutive B-frames:  0.7%  0.0%  1.6% 97.7%
2026-02-22T05:06:09.414766Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]: [libx264 @ 0x55e02179d500] mb I  I16..4: 14.7% 43.3% 42.1%
2026-02-22T05:06:09.414770Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]: [libx264 @ 0x55e02179d500] mb P  I16..4:  1.6%  7.0%  0.1%  P16..4: 35.7% 13.6%  4.0%  0.0%  0.0%    skip:38.0%
2026-02-22T05:06:09.414777Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]: [libx264 @ 0x55e02179d500] mb B  I16..4:  0.1%  0.1%  0.0%  B16..8: 10.0%  1.1%  0.1%  direct: 0.9%  skip:87.7%  L0:33.2% L1:61.3% BI: 5.5%
2026-02-22T05:06:09.414782Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]: [libx264 @ 0x55e02179d500] 8x8 transform intra:70.8% inter:70.2%
2026-02-22T05:06:09.414786Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]: [libx264 @ 0x55e02179d500] coded y,uvDC,uvAC intra: 48.9% 37.9% 4.7% inter: 3.2% 2.4% 0.0%
2026-02-22T05:06:09.414791Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]: [libx264 @ 0x55e02179d500] i16 v,h,dc,p: 46% 30% 15%  9%
2026-02-22T05:06:09.414795Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]: [libx264 @ 0x55e02179d500] i8 v,h,dc,ddl,ddr,vr,hd,vl,hu: 19% 31% 27%  4%  3%  2%  5%  3%  7%
2026-02-22T05:06:09.414802Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]: [libx264 @ 0x55e02179d500] i4 v,h,dc,ddl,ddr,vr,hd,vl,hu: 17% 22% 10%  9%  9%  6% 10%  7% 10%
2026-02-22T05:06:09.414806Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]: [libx264 @ 0x55e02179d500] i8c dc,h,v,p: 60% 24% 13%  4%
2026-02-22T05:06:09.414810Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]: [libx264 @ 0x55e02179d500] Weighted P-Frames: Y:0.0% UV:0.0%
2026-02-22T05:06:09.414814Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]: [libx264 @ 0x55e02179d500] kb/s:1554.27
2026-02-22T05:06:09.457348Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]: [aac @ 0x55e021f7c240] Qavg: 546.794
2026-02-22T05:06:09.487939Z  WARN ferrite_stream::hls: ffmpeg HLS [cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa]: Exiting normally, received signal 15.
2026-02-22T05:06:10.621584Z  INFO http_request{method=DELETE uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/session/stop?playback_session_id=4a6b450a-1400-47b9-b0c0-71a1ced8fcd4}: ferrite_stream::hls: Destroyed HLS session cfdd2adb-9d30-41f2-b3e0-49ef1552f3aa
2026-02-22T05:06:11.198821Z  INFO http_request{method=GET uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/master.m3u8?start=1434.564&token=[REDACTED]&playback_session_id=6b348531%2D9998%2D4e25%2D9ca7%2D3a154a80388a}: ferrite_api::handlers::stream: Transcode permit acquired: op=hls-master media_id=ca663e67-38d2-4f47-9008-ed2de1f68a50 wait_ms=0.0
2026-02-22T05:06:11.198870Z  INFO http_request{method=GET uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/master.m3u8?start=1434.564&token=[REDACTED]&playback_session_id=6b348531%2D9998%2D4e25%2D9ca7%2D3a154a80388a}: ferrite_stream::hls: Creating 4 ABR variant sessions for media ca663e67-38d2-4f47-9008-ed2de1f68a50 (source=1920x1080)
2026-02-22T05:06:11.199033Z  INFO http_request{method=GET uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/master.m3u8?start=1434.564&token=[REDACTED]&playback_session_id=6b348531%2D9998%2D4e25%2D9ca7%2D3a154a80388a}: ferrite_stream::hls: Creating HLS session 542e39ca-1b17-4d1c-89cc-8b1b8537f863 for media ca663e67-38d2-4f47-9008-ed2de1f68a50 at 1431.8s variant=1080p (/home/badchiefy/files/tv/Asia (2024)/Season 1/Asia.S01E01.Beneath.the.Waves.1080p.WEBRip.10bit.EAC3.2.0.x265-iVy.mkv)
2026-02-22T05:06:11.199059Z  INFO http_request{method=GET uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/master.m3u8?start=1434.564&token=[REDACTED]&playback_session_id=6b348531%2D9998%2D4e25%2D9ca7%2D3a154a80388a}: ferrite_stream::hls: 10-bit SDR detected (pix=yuv420p10le, transfer=Some("bt709")), applying bit-depth conversion only
2026-02-22T05:06:11.199086Z  INFO http_request{method=GET uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/master.m3u8?start=1434.564&token=[REDACTED]&playback_session_id=6b348531%2D9998%2D4e25%2D9ca7%2D3a154a80388a}: ferrite_stream::hls: HLS ffmpeg args: ["-hide_banner", "-nostdin", "-ss", "1431.800", "-i", "/home/badchiefy/files/tv/Asia (2024)/Season 1/Asia.S01E01.Beneath.the.Waves.1080p.WEBRip.10bit.EAC3.2.0.x265-iVy.mkv", "-ss", "2.764", "-map", "0:v:0", "-map", "0:a:0", "-vf", "format=yuv420p", "-c:v", "libx264", "-preset", "veryfast", "-crf", "23", "-profile:v", "high", "-level", "4.1", "-g", "180", "-keyint_min", "180", "-c:a", "aac", "-b:a", "192k", "-ac", "2", "-f", "hls", "-hls_time", "6", "-hls_list_size", "30", "-hls_segment_type", "fmp4", "-hls_fmp4_init_filename", "init.mp4", "-hls_segment_filename", "seg_%03d.m4s", "-hls_flags", "independent_segments+delete_segments+temp_file", "playlist.m3u8"]
2026-02-22T05:06:11.199751Z  INFO http_request{method=GET uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/master.m3u8?start=1434.564&token=[REDACTED]&playback_session_id=6b348531%2D9998%2D4e25%2D9ca7%2D3a154a80388a}: ferrite_stream::hls: Creating HLS session 37ad02fb-0347-420f-a211-796301266068 for media ca663e67-38d2-4f47-9008-ed2de1f68a50 at 1431.8s variant=720p (/home/badchiefy/files/tv/Asia (2024)/Season 1/Asia.S01E01.Beneath.the.Waves.1080p.WEBRip.10bit.EAC3.2.0.x265-iVy.mkv)
2026-02-22T05:06:11.199772Z  INFO http_request{method=GET uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/master.m3u8?start=1434.564&token=[REDACTED]&playback_session_id=6b348531%2D9998%2D4e25%2D9ca7%2D3a154a80388a}: ferrite_stream::hls: 10-bit SDR detected (pix=yuv420p10le, transfer=Some("bt709")), applying bit-depth conversion only
2026-02-22T05:06:11.199798Z  INFO http_request{method=GET uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/master.m3u8?start=1434.564&token=[REDACTED]&playback_session_id=6b348531%2D9998%2D4e25%2D9ca7%2D3a154a80388a}: ferrite_stream::hls: HLS ffmpeg args: ["-hide_banner", "-nostdin", "-ss", "1431.800", "-i", "/home/badchiefy/files/tv/Asia (2024)/Season 1/Asia.S01E01.Beneath.the.Waves.1080p.WEBRip.10bit.EAC3.2.0.x265-iVy.mkv", "-ss", "2.764", "-map", "0:v:0", "-map", "0:a:0", "-vf", "format=yuv420p,scale=-2:720", "-c:v", "libx264", "-preset", "veryfast", "-crf", "23", "-profile:v", "high", "-level", "4.1", "-b:v", "2800k", "-maxrate", "4200k", "-bufsize", "5600k", "-g", "180", "-keyint_min", "180", "-c:a", "aac", "-b:a", "128k", "-ac", "2", "-f", "hls", "-hls_time", "6", "-hls_list_size", "30", "-hls_segment_type", "fmp4", "-hls_fmp4_init_filename", "init.mp4", "-hls_segment_filename", "seg_%03d.m4s", "-hls_flags", "independent_segments+delete_segments+temp_file", "playlist.m3u8"]
2026-02-22T05:06:11.200419Z  INFO http_request{method=GET uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/master.m3u8?start=1434.564&token=[REDACTED]&playback_session_id=6b348531%2D9998%2D4e25%2D9ca7%2D3a154a80388a}: ferrite_stream::hls: Creating HLS session dfc2bf90-4262-4793-abb8-b3112f03b9cb for media ca663e67-38d2-4f47-9008-ed2de1f68a50 at 1431.8s variant=480p (/home/badchiefy/files/tv/Asia (2024)/Season 1/Asia.S01E01.Beneath.the.Waves.1080p.WEBRip.10bit.EAC3.2.0.x265-iVy.mkv)
2026-02-22T05:06:11.200444Z  INFO http_request{method=GET uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/master.m3u8?start=1434.564&token=[REDACTED]&playback_session_id=6b348531%2D9998%2D4e25%2D9ca7%2D3a154a80388a}: ferrite_stream::hls: 10-bit SDR detected (pix=yuv420p10le, transfer=Some("bt709")), applying bit-depth conversion only
2026-02-22T05:06:11.200469Z  INFO http_request{method=GET uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/master.m3u8?start=1434.564&token=[REDACTED]&playback_session_id=6b348531%2D9998%2D4e25%2D9ca7%2D3a154a80388a}: ferrite_stream::hls: HLS ffmpeg args: ["-hide_banner", "-nostdin", "-ss", "1431.800", "-i", "/home/badchiefy/files/tv/Asia (2024)/Season 1/Asia.S01E01.Beneath.the.Waves.1080p.WEBRip.10bit.EAC3.2.0.x265-iVy.mkv", "-ss", "2.764", "-map", "0:v:0", "-map", "0:a:0", "-vf", "format=yuv420p,scale=-2:480", "-c:v", "libx264", "-preset", "veryfast", "-crf", "23", "-profile:v", "high", "-level", "4.1", "-b:v", "1400k", "-maxrate", "2100k", "-bufsize", "2800k", "-g", "180", "-keyint_min", "180", "-c:a", "aac", "-b:a", "128k", "-ac", "2", "-f", "hls", "-hls_time", "6", "-hls_list_size", "30", "-hls_segment_type", "fmp4", "-hls_fmp4_init_filename", "init.mp4", "-hls_segment_filename", "seg_%03d.m4s", "-hls_flags", "independent_segments+delete_segments+temp_file", "playlist.m3u8"]
2026-02-22T05:06:11.201104Z  INFO http_request{method=GET uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/master.m3u8?start=1434.564&token=[REDACTED]&playback_session_id=6b348531%2D9998%2D4e25%2D9ca7%2D3a154a80388a}: ferrite_stream::hls: Creating HLS session df7cb4a0-be8a-4e62-bc3b-d778f2b03cba for media ca663e67-38d2-4f47-9008-ed2de1f68a50 at 1431.8s variant=360p (/home/badchiefy/files/tv/Asia (2024)/Season 1/Asia.S01E01.Beneath.the.Waves.1080p.WEBRip.10bit.EAC3.2.0.x265-iVy.mkv)
2026-02-22T05:06:11.201151Z  INFO http_request{method=GET uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/master.m3u8?start=1434.564&token=[REDACTED]&playback_session_id=6b348531%2D9998%2D4e25%2D9ca7%2D3a154a80388a}: ferrite_stream::hls: 10-bit SDR detected (pix=yuv420p10le, transfer=Some("bt709")), applying bit-depth conversion only
2026-02-22T05:06:11.201175Z  INFO http_request{method=GET uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/master.m3u8?start=1434.564&token=[REDACTED]&playback_session_id=6b348531%2D9998%2D4e25%2D9ca7%2D3a154a80388a}: ferrite_stream::hls: HLS ffmpeg args: ["-hide_banner", "-nostdin", "-ss", "1431.800", "-i", "/home/badchiefy/files/tv/Asia (2024)/Season 1/Asia.S01E01.Beneath.the.Waves.1080p.WEBRip.10bit.EAC3.2.0.x265-iVy.mkv", "-ss", "2.764", "-map", "0:v:0", "-map", "0:a:0", "-vf", "format=yuv420p,scale=-2:360", "-c:v", "libx264", "-preset", "veryfast", "-crf", "23", "-profile:v", "high", "-level", "4.1", "-b:v", "800k", "-maxrate", "1200k", "-bufsize", "1600k", "-g", "180", "-keyint_min", "180", "-c:a", "aac", "-b:a", "96k", "-ac", "2", "-f", "hls", "-hls_time", "6", "-hls_list_size", "30", "-hls_segment_type", "fmp4", "-hls_fmp4_init_filename", "init.mp4", "-hls_segment_filename", "seg_%03d.m4s", "-hls_flags", "independent_segments+delete_segments+temp_file", "playlist.m3u8"]
2026-02-22T05:06:11.214444Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]: Input #0, matroska,webm, from '/home/badchiefy/files/tv/Asia (2024)/Season 1/Asia.S01E01.Beneath.the.Waves.1080p.WEBRip.10bit.EAC3.2.0.x265-iVy.mkv':
2026-02-22T05:06:11.214454Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]: Input #0, matroska,webm, from '/home/badchiefy/files/tv/Asia (2024)/Season 1/Asia.S01E01.Beneath.the.Waves.1080p.WEBRip.10bit.EAC3.2.0.x265-iVy.mkv':
2026-02-22T05:06:11.214492Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]:   Metadata:
2026-02-22T05:06:11.214500Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]:   Metadata:
2026-02-22T05:06:11.214502Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]:     creation_time   : 2025-09-05T17:37:40.000000Z
2026-02-22T05:06:11.214508Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]:     creation_time   : 2025-09-05T17:37:40.000000Z
2026-02-22T05:06:11.214513Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]:     ENCODER         : Lavf61.7.100
2026-02-22T05:06:11.214516Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]:     ENCODER         : Lavf61.7.100
2026-02-22T05:06:11.214526Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]:   Duration: 00:58:00.05, start: -0.005000, bitrate: 4284 kb/s
2026-02-22T05:06:11.214526Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]:   Duration: 00:58:00.05, start: -0.005000, bitrate: 4284 kb/s
2026-02-22T05:06:11.214534Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]:   Stream #0:0: Video: hevc (Main 10), yuv420p10le(tv, bt709), 1920x1080 [SAR 1:1 DAR 16:9], 30 fps, 30 tbr, 1k tbn (default)
2026-02-22T05:06:11.214542Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]:       Metadata:
2026-02-22T05:06:11.214540Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]:   Stream #0:0: Video: hevc (Main 10), yuv420p10le(tv, bt709), 1920x1080 [SAR 1:1 DAR 16:9], 30 fps, 30 tbr, 1k tbn (default)
2026-02-22T05:06:11.214550Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]:         DURATION        : 00:58:00.053000000
2026-02-22T05:06:11.214552Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]:       Metadata:
2026-02-22T05:06:11.214557Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]:   Stream #0:1(eng): Audio: eac3, 48000 Hz, stereo, fltp, 640 kb/s (default)
2026-02-22T05:06:11.214560Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]:         DURATION        : 00:58:00.053000000
2026-02-22T05:06:11.214564Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]:       Metadata:
2026-02-22T05:06:11.214570Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]:         DURATION        : 00:58:00.000000000
2026-02-22T05:06:11.214570Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]:   Stream #0:1(eng): Audio: eac3, 48000 Hz, stereo, fltp, 640 kb/s (default)
2026-02-22T05:06:11.214576Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]:   Stream #0:2(eng): Subtitle: ass (ssa)
2026-02-22T05:06:11.214580Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]:       Metadata:
2026-02-22T05:06:11.214586Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]:       Metadata:
2026-02-22T05:06:11.214588Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]:         DURATION        : 00:58:00.000000000
2026-02-22T05:06:11.214592Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]:         DURATION        : 00:57:24.800000000
2026-02-22T05:06:11.214599Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]:   Stream #0:3(eng): Subtitle: ass (ssa)
2026-02-22T05:06:11.214598Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]:   Stream #0:2(eng): Subtitle: ass (ssa)
2026-02-22T05:06:11.214606Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]:       Metadata:
2026-02-22T05:06:11.214608Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]:       Metadata:
2026-02-22T05:06:11.214612Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]:         title           : SDH
2026-02-22T05:06:11.214616Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]:         DURATION        : 00:57:24.800000000
2026-02-22T05:06:11.214618Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]:         DURATION        : 00:57:24.800000000
2026-02-22T05:06:11.214625Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]:   Stream #0:3(eng): Subtitle: ass (ssa)
2026-02-22T05:06:11.214634Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]:       Metadata:
2026-02-22T05:06:11.214642Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]:         title           : SDH
2026-02-22T05:06:11.214651Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]:         DURATION        : 00:57:24.800000000
2026-02-22T05:06:11.216810Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]: Input #0, matroska,webm, from '/home/badchiefy/files/tv/Asia (2024)/Season 1/Asia.S01E01.Beneath.the.Waves.1080p.WEBRip.10bit.EAC3.2.0.x265-iVy.mkv':
2026-02-22T05:06:11.216829Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]:   Metadata:
2026-02-22T05:06:11.216836Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]:     creation_time   : 2025-09-05T17:37:40.000000Z
2026-02-22T05:06:11.216842Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]:     ENCODER         : Lavf61.7.100
2026-02-22T05:06:11.216849Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]:   Duration: 00:58:00.05, start: -0.005000, bitrate: 4284 kb/s
2026-02-22T05:06:11.216862Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]:   Stream #0:0: Video: hevc (Main 10), yuv420p10le(tv, bt709), 1920x1080 [SAR 1:1 DAR 16:9], 30 fps, 30 tbr, 1k tbn (default)
2026-02-22T05:06:11.216870Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]:       Metadata:
2026-02-22T05:06:11.216876Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]:         DURATION        : 00:58:00.053000000
2026-02-22T05:06:11.216882Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]:   Stream #0:1(eng): Audio: eac3, 48000 Hz, stereo, fltp, 640 kb/s (default)
2026-02-22T05:06:11.216889Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]:       Metadata:
2026-02-22T05:06:11.216895Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]:         DURATION        : 00:58:00.000000000
2026-02-22T05:06:11.216908Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]:   Stream #0:2(eng): Subtitle: ass (ssa)
2026-02-22T05:06:11.216914Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]:       Metadata:
2026-02-22T05:06:11.216920Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]:         DURATION        : 00:57:24.800000000
2026-02-22T05:06:11.216926Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]:   Stream #0:3(eng): Subtitle: ass (ssa)
2026-02-22T05:06:11.216932Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]:       Metadata:
2026-02-22T05:06:11.216938Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]:         title           : SDH
2026-02-22T05:06:11.216944Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]:         DURATION        : 00:57:24.800000000
2026-02-22T05:06:11.219048Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]: Stream mapping:
2026-02-22T05:06:11.219068Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]:   Stream #0:0 -> #0:0 (hevc (native) -> h264 (libx264))
2026-02-22T05:06:11.219090Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]:   Stream #0:1 -> #0:1 (eac3 (native) -> aac (native))
2026-02-22T05:06:11.219823Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]: Stream mapping:
2026-02-22T05:06:11.219844Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]:   Stream #0:0 -> #0:0 (hevc (native) -> h264 (libx264))
2026-02-22T05:06:11.219850Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]:   Stream #0:1 -> #0:1 (eac3 (native) -> aac (native))
2026-02-22T05:06:11.221132Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]: Stream mapping:
2026-02-22T05:06:11.221141Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]:   Stream #0:0 -> #0:0 (hevc (native) -> h264 (libx264))
2026-02-22T05:06:11.221145Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]:   Stream #0:1 -> #0:1 (eac3 (native) -> aac (native))
2026-02-22T05:06:11.223847Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]: Input #0, matroska,webm, from '/home/badchiefy/files/tv/Asia (2024)/Season 1/Asia.S01E01.Beneath.the.Waves.1080p.WEBRip.10bit.EAC3.2.0.x265-iVy.mkv':
2026-02-22T05:06:11.224284Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]:   Metadata:
2026-02-22T05:06:11.224291Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]:     creation_time   : 2025-09-05T17:37:40.000000Z
2026-02-22T05:06:11.224295Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]:     ENCODER         : Lavf61.7.100
2026-02-22T05:06:11.224299Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]:   Duration: 00:58:00.05, start: -0.005000, bitrate: 4284 kb/s
2026-02-22T05:06:11.224303Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]:   Stream #0:0: Video: hevc (Main 10), yuv420p10le(tv, bt709), 1920x1080 [SAR 1:1 DAR 16:9], 30 fps, 30 tbr, 1k tbn (default)
2026-02-22T05:06:11.224307Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]:       Metadata:
2026-02-22T05:06:11.224311Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]:         DURATION        : 00:58:00.053000000
2026-02-22T05:06:11.224314Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]:   Stream #0:1(eng): Audio: eac3, 48000 Hz, stereo, fltp, 640 kb/s (default)
2026-02-22T05:06:11.224318Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]:       Metadata:
2026-02-22T05:06:11.224321Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]:         DURATION        : 00:58:00.000000000
2026-02-22T05:06:11.224324Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]:   Stream #0:2(eng): Subtitle: ass (ssa)
2026-02-22T05:06:11.224327Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]:       Metadata:
2026-02-22T05:06:11.224329Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]:         DURATION        : 00:57:24.800000000
2026-02-22T05:06:11.224332Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]:   Stream #0:3(eng): Subtitle: ass (ssa)
2026-02-22T05:06:11.224335Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]:       Metadata:
2026-02-22T05:06:11.224338Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]:         title           : SDH
2026-02-22T05:06:11.224340Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]:         DURATION        : 00:57:24.800000000
2026-02-22T05:06:11.228521Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]: Stream mapping:
2026-02-22T05:06:11.228529Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]:   Stream #0:0 -> #0:0 (hevc (native) -> h264 (libx264))
2026-02-22T05:06:11.228533Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]:   Stream #0:1 -> #0:1 (eac3 (native) -> aac (native))
2026-02-22T05:06:12.791182Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]: [libx264 @ 0x559eae7e3500] using SAR=1/1
2026-02-22T05:06:12.791752Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]: [libx264 @ 0x559eae7e3500] using cpu capabilities: MMX2 SSE2Fast SSSE3 SSE4.2 AVX FMA3 BMI2 AVX2
2026-02-22T05:06:12.800514Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]: [libx264 @ 0x559eae7e3500] profile High, level 4.1, 4:2:0, 8-bit
2026-02-22T05:06:12.800952Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]: [libx264 @ 0x559eae7e3500] 264 - core 164 - H.264/MPEG-4 AVC codec - Copyleft 2003-2024 - http://www.videolan.org/x264.html - options: cabac=1 ref=1 deblock=1:0:0 analyse=0x3:0x113 me=hex subme=2 psy=1 psy_rd=1.00:0.00 mixed_ref=0 me_range=16 chroma_me=1 trellis=0 8x8dct=1 cqm=0 deadzone=21,11 fast_pskip=1 chroma_qp_offset=0 threads=12 lookahead_threads=4 sliced_threads=0 nr=0 decimate=1 interlaced=0 bluray_compat=0 constrained_intra=0 bframes=3 b_pyramid=2 b_adapt=1 b_bias=0 direct=1 weightb=1 open_gop=0 weightp=1 keyint=180 keyint_min=91 scenecut=40 intra_refresh=0 rc_lookahead=10 rc=crf mbtree=1 crf=23.0 qcomp=0.60 qpmin=0 qpmax=69 qpstep=4 ip_ratio=1.40 aq=1:1.00
2026-02-22T05:06:12.801044Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]: [hls @ 0x559eae7f1340] Opening 'init.mp4' for writing
2026-02-22T05:06:12.801153Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]: Output #0, hls, to 'playlist.m3u8':
2026-02-22T05:06:12.801158Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]:   Metadata:
2026-02-22T05:06:12.801165Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]:     encoder         : Lavf61.7.100
2026-02-22T05:06:12.801189Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]:   Stream #0:0: Video: h264, yuv420p(tv, bt709, progressive), 1920x1080 [SAR 1:1 DAR 16:9], q=2-31, 30 fps, 15360 tbn (default)
2026-02-22T05:06:12.801198Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]:       Metadata:
2026-02-22T05:06:12.801201Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]:         DURATION        : 00:58:00.053000000
2026-02-22T05:06:12.801205Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]:         encoder         : Lavc61.19.101 libx264
2026-02-22T05:06:12.801207Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]:       Side data:
2026-02-22T05:06:12.801213Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]:         cpb: bitrate max/min/avg: 0/0/0 buffer size: 0 vbv_delay: N/A
2026-02-22T05:06:12.801223Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]:   Stream #0:1(eng): Audio: aac (LC), 48000 Hz, stereo, fltp, 192 kb/s (default)
2026-02-22T05:06:12.801228Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]:       Metadata:
2026-02-22T05:06:12.801233Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]:         DURATION        : 00:58:00.000000000
2026-02-22T05:06:12.801237Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]:         encoder         : Lavc61.19.101 aac
2026-02-22T05:06:13.290459Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]: [libx264 @ 0x5621128ff5c0] using SAR=1/1
2026-02-22T05:06:13.290479Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]: [libx264 @ 0x5621128ff5c0] using cpu capabilities: MMX2 SSE2Fast SSSE3 SSE4.2 AVX FMA3 BMI2 AVX2
2026-02-22T05:06:13.290485Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]: [libx264 @ 0x5621128ff5c0] profile High, level 4.1, 4:2:0, 8-bit
2026-02-22T05:06:13.290498Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]: [libx264 @ 0x5621128ff5c0] 264 - core 164 - H.264/MPEG-4 AVC codec - Copyleft 2003-2024 - http://www.videolan.org/x264.html - options: cabac=1 ref=1 deblock=1:0:0 analyse=0x3:0x113 me=hex subme=2 psy=1 psy_rd=1.00:0.00 mixed_ref=0 me_range=16 chroma_me=1 trellis=0 8x8dct=1 cqm=0 deadzone=21,11 fast_pskip=1 chroma_qp_offset=0 threads=11 lookahead_threads=2 sliced_threads=0 nr=0 decimate=1 interlaced=0 bluray_compat=0 constrained_intra=0 bframes=3 b_pyramid=2 b_adapt=1 b_bias=0 direct=1 weightb=1 open_gop=0 weightp=1 keyint=180 keyint_min=91 scenecut=40 intra_refresh=0 rc_lookahead=10 rc=crf mbtree=1 crf=23.0 qcomp=0.60 qpmin=0 qpmax=69 qpstep=4 vbv_maxrate=1200 vbv_bufsize=1600 crf_max=0.0 nal_hrd=none filler=0 ip_ratio=1.40 aq=1:1.00
2026-02-22T05:06:13.290552Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]: [hls @ 0x562112917140] Opening 'init.mp4' for writing
2026-02-22T05:06:13.290773Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]: Output #0, hls, to 'playlist.m3u8':
2026-02-22T05:06:13.290780Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]:   Metadata:
2026-02-22T05:06:13.290783Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]:     encoder         : Lavf61.7.100
2026-02-22T05:06:13.290786Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]:   Stream #0:0: Video: h264, yuv420p(tv, bt709, progressive), 640x360 [SAR 1:1 DAR 16:9], q=2-31, 800 kb/s, 30 fps, 15360 tbn (default)
2026-02-22T05:06:13.290791Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]:       Metadata:
2026-02-22T05:06:13.290793Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]:         DURATION        : 00:58:00.053000000
2026-02-22T05:06:13.290796Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]:         encoder         : Lavc61.19.101 libx264
2026-02-22T05:06:13.290800Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]:       Side data:
2026-02-22T05:06:13.290803Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]:         cpb: bitrate max/min/avg: 1200000/0/800000 buffer size: 1600000 vbv_delay: N/A
2026-02-22T05:06:13.290807Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]:   Stream #0:1(eng): Audio: aac (LC), 48000 Hz, stereo, fltp, 96 kb/s (default)
2026-02-22T05:06:13.290811Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]:       Metadata:
2026-02-22T05:06:13.290814Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]:         DURATION        : 00:58:00.000000000
2026-02-22T05:06:13.290817Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]:         encoder         : Lavc61.19.101 aac
2026-02-22T05:06:13.384412Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]: [libx264 @ 0x56050a8365c0] using SAR=1/1
2026-02-22T05:06:13.385117Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]: [libx264 @ 0x56050a8365c0] using cpu capabilities: MMX2 SSE2Fast SSSE3 SSE4.2 AVX FMA3 BMI2 AVX2
2026-02-22T05:06:13.392461Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]: [libx264 @ 0x56050a8365c0] profile High, level 4.1, 4:2:0, 8-bit
2026-02-22T05:06:13.392484Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]: [libx264 @ 0x56050a8365c0] 264 - core 164 - H.264/MPEG-4 AVC codec - Copyleft 2003-2024 - http://www.videolan.org/x264.html - options: cabac=1 ref=1 deblock=1:0:0 analyse=0x3:0x113 me=hex subme=2 psy=1 psy_rd=1.00:0.00 mixed_ref=0 me_range=16 chroma_me=1 trellis=0 8x8dct=1 cqm=0 deadzone=21,11 fast_pskip=1 chroma_qp_offset=0 threads=12 lookahead_threads=4 sliced_threads=0 nr=0 decimate=1 interlaced=0 bluray_compat=0 constrained_intra=0 bframes=3 b_pyramid=2 b_adapt=1 b_bias=0 direct=1 weightb=1 open_gop=0 weightp=1 keyint=180 keyint_min=91 scenecut=40 intra_refresh=0 rc_lookahead=10 rc=crf mbtree=1 crf=23.0 qcomp=0.60 qpmin=0 qpmax=69 qpstep=4 vbv_maxrate=4200 vbv_bufsize=5600 crf_max=0.0 nal_hrd=none filler=0 ip_ratio=1.40 aq=1:1.00
2026-02-22T05:06:13.392518Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]: [hls @ 0x56050a84e140] Opening 'init.mp4' for writing
2026-02-22T05:06:13.392523Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]: Output #0, hls, to 'playlist.m3u8':
2026-02-22T05:06:13.392526Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]:   Metadata:
2026-02-22T05:06:13.392529Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]:     encoder         : Lavf61.7.100
2026-02-22T05:06:13.392532Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]:   Stream #0:0: Video: h264, yuv420p(tv, bt709, progressive), 1280x720 [SAR 1:1 DAR 16:9], q=2-31, 2800 kb/s, 30 fps, 15360 tbn (default)
2026-02-22T05:06:13.392536Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]:       Metadata:
2026-02-22T05:06:13.392539Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]:         DURATION        : 00:58:00.053000000
2026-02-22T05:06:13.392542Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]:         encoder         : Lavc61.19.101 libx264
2026-02-22T05:06:13.392548Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]:       Side data:
2026-02-22T05:06:13.392551Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]:         cpb: bitrate max/min/avg: 4200000/0/2800000 buffer size: 5600000 vbv_delay: N/A
2026-02-22T05:06:13.392555Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]:   Stream #0:1(eng): Audio: aac (LC), 48000 Hz, stereo, fltp, 128 kb/s (default)
2026-02-22T05:06:13.392558Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]:       Metadata:
2026-02-22T05:06:13.392561Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]:         DURATION        : 00:58:00.000000000
2026-02-22T05:06:13.392563Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]:         encoder         : Lavc61.19.101 aac
2026-02-22T05:06:13.465590Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]: [libx264 @ 0x564a819285c0] using SAR=1280/1281
2026-02-22T05:06:13.465615Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]: [libx264 @ 0x564a819285c0] using cpu capabilities: MMX2 SSE2Fast SSSE3 SSE4.2 AVX FMA3 BMI2 AVX2
2026-02-22T05:06:13.473609Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]: [libx264 @ 0x564a819285c0] profile High, level 4.1, 4:2:0, 8-bit
2026-02-22T05:06:13.474045Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]: [libx264 @ 0x564a819285c0] 264 - core 164 - H.264/MPEG-4 AVC codec - Copyleft 2003-2024 - http://www.videolan.org/x264.html - options: cabac=1 ref=1 deblock=1:0:0 analyse=0x3:0x113 me=hex subme=2 psy=1 psy_rd=1.00:0.00 mixed_ref=0 me_range=16 chroma_me=1 trellis=0 8x8dct=1 cqm=0 deadzone=21,11 fast_pskip=1 chroma_qp_offset=0 threads=12 lookahead_threads=3 sliced_threads=0 nr=0 decimate=1 interlaced=0 bluray_compat=0 constrained_intra=0 bframes=3 b_pyramid=2 b_adapt=1 b_bias=0 direct=1 weightb=1 open_gop=0 weightp=1 keyint=180 keyint_min=91 scenecut=40 intra_refresh=0 rc_lookahead=10 rc=crf mbtree=1 crf=23.0 qcomp=0.60 qpmin=0 qpmax=69 qpstep=4 vbv_maxrate=2100 vbv_bufsize=2800 crf_max=0.0 nal_hrd=none filler=0 ip_ratio=1.40 aq=1:1.00
2026-02-22T05:06:13.474093Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]: [hls @ 0x564a81940140] Opening 'init.mp4' for writing
2026-02-22T05:06:13.474110Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]: Output #0, hls, to 'playlist.m3u8':
2026-02-22T05:06:13.474124Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]:   Metadata:
2026-02-22T05:06:13.474138Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]:     encoder         : Lavf61.7.100
2026-02-22T05:06:13.474152Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]:   Stream #0:0: Video: h264, yuv420p(tv, bt709, progressive), 854x480 [SAR 1280:1281 DAR 16:9], q=2-31, 1400 kb/s, 30 fps, 15360 tbn (default)
2026-02-22T05:06:13.474169Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]:       Metadata:
2026-02-22T05:06:13.474186Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]:         DURATION        : 00:58:00.053000000
2026-02-22T05:06:13.474200Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]:         encoder         : Lavc61.19.101 libx264
2026-02-22T05:06:13.474214Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]:       Side data:
2026-02-22T05:06:13.474227Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]:         cpb: bitrate max/min/avg: 2100000/0/1400000 buffer size: 2800000 vbv_delay: N/A
2026-02-22T05:06:13.474244Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]:   Stream #0:1(eng): Audio: aac (LC), 48000 Hz, stereo, fltp, 128 kb/s (default)
2026-02-22T05:06:13.474273Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]:       Metadata:
2026-02-22T05:06:13.474285Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]:         DURATION        : 00:58:00.000000000
2026-02-22T05:06:13.474298Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]:         encoder         : Lavc61.19.101 aac
2026-02-22T05:06:16.300371Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]: frame=    3 fps=1.2 [hls @ 0x562112917140] Opening 'seg_000.m4s.tmp' for writing=N/A speed=1.17x
2026-02-22T05:06:16.300795Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]: [hls @ 0x562112917140] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:06:16.335670Z  INFO http_request{method=GET uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/master.m3u8?start=1434.564&token=[REDACTED]&playback_session_id=6b348531%2D9998%2D4e25%2D9ca7%2D3a154a80388a}: ferrite_stream::hls: HLS session df7cb4a0-be8a-4e62-bc3b-d778f2b03cba ready (first segment generated in 5.1s)
2026-02-22T05:06:16.340883Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]: frame=    0 fps=0.0 [hls @ 0x56050a84e140] Opening 'seg_000.m4s.tmp' for writing=N/A speed=1.16x
2026-02-22T05:06:16.341168Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]: [hls @ 0x56050a84e140] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:06:16.532890Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]: frame=    0 fps=0.0 [hls @ 0x564a81940140] Opening 'seg_000.m4s.tmp' for writing=N/A speed=1.08x
2026-02-22T05:06:16.533644Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]: [hls @ 0x564a81940140] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:06:16.600682Z  INFO http_request{method=GET uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/master.m3u8?start=1434.564&token=[REDACTED]&playback_session_id=6b348531%2D9998%2D4e25%2D9ca7%2D3a154a80388a}: ferrite_stream::hls: HLS session 37ad02fb-0347-420f-a211-796301266068 ready (first segment generated in 5.4s)
2026-02-22T05:06:16.604140Z  INFO http_request{method=GET uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/master.m3u8?start=1434.564&token=[REDACTED]&playback_session_id=6b348531%2D9998%2D4e25%2D9ca7%2D3a154a80388a}: ferrite_stream::hls: HLS session dfc2bf90-4262-4793-abb8-b3112f03b9cb ready (first segment generated in 5.4s)
2026-02-22T05:06:16.966521Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]: frame=    2 fps=1.0 [hls @ 0x559eae7f1340] Opening 'seg_000.m4s.tmp' for writing=N/A speed=1.03x
2026-02-22T05:06:16.968993Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]: [hls @ 0x559eae7f1340] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:06:17.092494Z  INFO http_request{method=GET uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/master.m3u8?start=1434.564&token=[REDACTED]&playback_session_id=6b348531%2D9998%2D4e25%2D9ca7%2D3a154a80388a}: ferrite_stream::hls: HLS session 542e39ca-1b17-4d1c-89cc-8b1b8537f863 ready (first segment generated in 5.9s)
2026-02-22T05:06:17.092607Z  INFO http_request{method=GET uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/master.m3u8?start=1434.564&token=[REDACTED]&playback_session_id=6b348531%2D9998%2D4e25%2D9ca7%2D3a154a80388a}: ferrite_api::handlers::stream: HLS master playlist for ca663e67-38d2-4f47-9008-ed2de1f68a50: variants=4 reused=false mode=fast seek_source=index db=0ms seek=0ms session=5894ms total=5895ms
2026-02-22T05:06:18.907988Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]: frame=  211 fps= 38 [hls @ 0x56050a84e140] Opening 'seg_001.m4s.tmp' for writing=N/A speed=1.55x
2026-02-22T05:06:18.908510Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]: [hls @ 0x56050a84e140] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:06:19.088628Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]: frame=  191 fps= 35 [hls @ 0x564a81940140] Opening 'seg_001.m4s.tmp' for writing=N/A speed=1.49x
2026-02-22T05:06:19.088654Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]: [hls @ 0x564a81940140] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:06:19.323607Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]: frame=  206 fps= 37 [hls @ 0x562112917140] Opening 'seg_001.m4s.tmp' for writing=N/A speed=1.47x
2026-02-22T05:06:19.323635Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]: [hls @ 0x562112917140] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:06:20.088124Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]: [hls @ 0x562112917140] Opening 'seg_002.m4s.tmp' for writing
2026-02-22T05:06:20.090799Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]: [hls @ 0x562112917140] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:06:20.090841Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]: [out#0/hls @ 0x562112917040] video:541KiB audio:169KiB subtitle:0KiB other streams:0KiB global headers:0KiB muxing overhead: unknown
2026-02-22T05:06:20.090956Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]: frame=  429 fps= 48 q=-1.0 Lsize=N/A time=00:00:14.20 bitrate=N/A dup=1 drop=0 speed= 1.6x
2026-02-22T05:06:20.101916Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]: [libx264 @ 0x5621128ff5c0] frame I:3     Avg QP:20.80  size: 29846
2026-02-22T05:06:20.101937Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]: [libx264 @ 0x5621128ff5c0] frame P:112   Avg QP:23.82  size:  3474
2026-02-22T05:06:20.101940Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]: [libx264 @ 0x5621128ff5c0] frame B:314   Avg QP:24.82  size:   236
2026-02-22T05:06:20.101944Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]: [libx264 @ 0x5621128ff5c0] consecutive B-frames:  0.7%  2.3%  8.4% 88.6%
2026-02-22T05:06:20.101947Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]: [libx264 @ 0x5621128ff5c0] mb I  I16..4: 12.9% 17.3% 69.8%
2026-02-22T05:06:20.101950Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]: [libx264 @ 0x5621128ff5c0] mb P  I16..4:  1.2%  1.4%  0.2%  P16..4: 47.6% 19.3%  7.0%  0.0%  0.0%    skip:23.4%
2026-02-22T05:06:20.101955Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]: [libx264 @ 0x5621128ff5c0] mb B  I16..4:  0.0%  0.0%  0.0%  B16..8:  7.2%  1.4%  0.2%  direct: 0.8%  skip:90.3%  L0:35.9% L1:44.1% BI:20.0%
2026-02-22T05:06:20.101960Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]: [libx264 @ 0x5621128ff5c0] 8x8 transform intra:33.8% inter:49.2%
2026-02-22T05:06:20.101964Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]: [libx264 @ 0x5621128ff5c0] coded y,uvDC,uvAC intra: 63.5% 59.4% 22.0% inter: 6.4% 3.2% 0.2%
2026-02-22T05:06:20.101968Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]: [libx264 @ 0x5621128ff5c0] i16 v,h,dc,p: 27% 43% 22%  8%
2026-02-22T05:06:20.101972Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]: [libx264 @ 0x5621128ff5c0] i8 v,h,dc,ddl,ddr,vr,hd,vl,hu: 26% 25% 21%  5%  3%  2%  5%  4% 10%
2026-02-22T05:06:20.101976Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]: [libx264 @ 0x5621128ff5c0] i4 v,h,dc,ddl,ddr,vr,hd,vl,hu: 14% 21% 14%  9%  8%  5% 10%  6% 13%
2026-02-22T05:06:20.101981Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]: [libx264 @ 0x5621128ff5c0] i8c dc,h,v,p: 49% 30% 14%  7%
2026-02-22T05:06:20.101985Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]: [libx264 @ 0x5621128ff5c0] Weighted P-Frames: Y:0.0% UV:0.0%
2026-02-22T05:06:20.101991Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]: [libx264 @ 0x5621128ff5c0] kb/s:309.26
2026-02-22T05:06:20.114572Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]: frame=  377 fps= 47 [hls @ 0x564a81940140] Opening 'seg_002.m4s.tmp' for writingeed=1.56x
2026-02-22T05:06:20.115226Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]: [hls @ 0x564a81940140] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:06:20.115236Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]: [out#0/hls @ 0x564a81940040] video:847KiB audio:249KiB subtitle:0KiB other streams:0KiB global headers:0KiB muxing overhead: unknown
2026-02-22T05:06:20.115240Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]: frame=  474 fps= 53 q=-1.0 Lsize=N/A time=00:00:15.64 bitrate=N/A dup=3 drop=0 speed=1.76x
2026-02-22T05:06:20.116924Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]: [libx264 @ 0x564a819285c0] frame I:3     Avg QP:20.28  size: 45145
2026-02-22T05:06:20.116933Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]: [libx264 @ 0x564a819285c0] frame P:120   Avg QP:23.24  size:  5103
2026-02-22T05:06:20.116937Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]: [libx264 @ 0x564a819285c0] frame B:351   Avg QP:21.97  size:   337
2026-02-22T05:06:20.116941Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]: [libx264 @ 0x564a819285c0] consecutive B-frames:  0.6%  0.4%  4.4% 94.5%
2026-02-22T05:06:20.116944Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]: [libx264 @ 0x564a819285c0] mb I  I16..4: 14.9% 19.9% 65.2%
2026-02-22T05:06:20.116950Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]: [libx264 @ 0x564a819285c0] mb P  I16..4:  1.5%  1.9%  0.1%  P16..4: 46.2% 18.3%  6.0%  0.0%  0.0%    skip:26.0%
2026-02-22T05:06:20.116954Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]: [libx264 @ 0x564a819285c0] mb B  I16..4:  0.0%  0.0%  0.0%  B16..8:  8.6%  1.2%  0.0%  direct: 0.4%  skip:89.6%  L0:35.2% L1:50.4% BI:14.5%
2026-02-22T05:06:20.116957Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]: [libx264 @ 0x564a819285c0] 8x8 transform intra:38.8% inter:53.7%
2026-02-22T05:06:20.116961Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]: [libx264 @ 0x564a819285c0] coded y,uvDC,uvAC intra: 56.4% 53.8% 15.1% inter: 5.4% 2.9% 0.1%
2026-02-22T05:06:20.116964Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]: [libx264 @ 0x564a819285c0] i16 v,h,dc,p: 33% 38% 18% 10%
2026-02-22T05:06:20.116968Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]: [libx264 @ 0x564a819285c0] i8 v,h,dc,ddl,ddr,vr,hd,vl,hu: 11% 38% 21%  5%  4%  3%  7%  3%  9%
2026-02-22T05:06:20.116971Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]: [libx264 @ 0x564a819285c0] i4 v,h,dc,ddl,ddr,vr,hd,vl,hu: 11% 24% 12%  9%  8%  5% 10%  6% 13%
2026-02-22T05:06:20.116974Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]: [libx264 @ 0x564a819285c0] i8c dc,h,v,p: 50% 32% 13%  5%
2026-02-22T05:06:20.116977Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]: [libx264 @ 0x564a819285c0] Weighted P-Frames: Y:0.0% UV:0.0%
2026-02-22T05:06:20.116980Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]: [libx264 @ 0x564a819285c0] kb/s:438.52
2026-02-22T05:06:20.119490Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]: [aac @ 0x5621130df080] Qavg: 450.015
2026-02-22T05:06:20.131931Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]: [aac @ 0x564a82108080] Qavg: 627.555
2026-02-22T05:06:20.194284Z  WARN ferrite_stream::hls: ffmpeg HLS [dfc2bf90-4262-4793-abb8-b3112f03b9cb]: Exiting normally, received signal 15.
2026-02-22T05:06:20.214610Z  WARN ferrite_stream::hls: ffmpeg HLS [df7cb4a0-be8a-4e62-bc3b-d778f2b03cba]: Exiting normally, received signal 15.
2026-02-22T05:06:20.249905Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]: frame=  389 fps= 48 [hls @ 0x56050a84e140] Opening 'seg_002.m4s.tmp' for writingeed=1.61x
2026-02-22T05:06:20.249925Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]: [hls @ 0x56050a84e140] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:06:20.249930Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]: [out#0/hls @ 0x56050a84e040] video:1559KiB audio:260KiB subtitle:0KiB other streams:0KiB global headers:0KiB muxing overhead: unknown
2026-02-22T05:06:20.249934Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]: frame=  494 fps= 55 q=-1.0 Lsize=N/A time=00:00:16.34 bitrate=N/A dup=2 drop=0 speed=1.81x
2026-02-22T05:06:20.255028Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]: [libx264 @ 0x56050a8365c0] frame I:3     Avg QP:19.67  size: 74539
2026-02-22T05:06:20.255044Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]: [libx264 @ 0x56050a8365c0] frame P:124   Avg QP:22.63  size:  8928
2026-02-22T05:06:20.255048Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]: [libx264 @ 0x56050a8365c0] frame B:367   Avg QP:21.07  size:   721
2026-02-22T05:06:20.255051Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]: [libx264 @ 0x56050a8365c0] consecutive B-frames:  0.6%  0.4%  1.8% 97.2%
2026-02-22T05:06:20.255054Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]: [libx264 @ 0x56050a8365c0] mb I  I16..4: 17.8% 26.1% 56.1%
2026-02-22T05:06:20.255064Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]: [libx264 @ 0x56050a8365c0] mb P  I16..4:  2.3%  2.6%  0.0%  P16..4: 42.1% 16.8%  5.2%  0.0%  0.0%    skip:31.0%
2026-02-22T05:06:20.255068Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]: [libx264 @ 0x56050a8365c0] mb B  I16..4:  0.1%  0.0%  0.0%  B16..8:  9.5%  1.1%  0.0%  direct: 0.5%  skip:88.8%  L0:35.2% L1:55.5% BI: 9.2%
2026-02-22T05:06:20.255072Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]: [libx264 @ 0x56050a8365c0] 8x8 transform intra:43.0% inter:60.0%
2026-02-22T05:06:20.255075Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]: [libx264 @ 0x56050a8365c0] coded y,uvDC,uvAC intra: 47.1% 45.8% 8.6% inter: 4.3% 2.7% 0.0%
2026-02-22T05:06:20.255079Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]: [libx264 @ 0x56050a8365c0] i16 v,h,dc,p: 37% 35% 18% 10%
2026-02-22T05:06:20.255082Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]: [libx264 @ 0x56050a8365c0] i8 v,h,dc,ddl,ddr,vr,hd,vl,hu: 12% 32% 25%  5%  4%  3%  6%  3%  9%
2026-02-22T05:06:20.255085Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]: [libx264 @ 0x56050a8365c0] i4 v,h,dc,ddl,ddr,vr,hd,vl,hu: 13% 22% 12% 10%  9%  6% 10%  7% 12%
2026-02-22T05:06:20.255088Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]: [libx264 @ 0x56050a8365c0] i8c dc,h,v,p: 55% 28% 13%  5%
2026-02-22T05:06:20.255091Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]: [libx264 @ 0x56050a8365c0] Weighted P-Frames: Y:0.0% UV:0.0%
2026-02-22T05:06:20.255095Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]: [libx264 @ 0x56050a8365c0] kb/s:775.13
2026-02-22T05:06:20.278720Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]: [aac @ 0x56050b016080] Qavg: 624.540
2026-02-22T05:06:20.370816Z  WARN ferrite_stream::hls: ffmpeg HLS [37ad02fb-0347-420f-a211-796301266068]: Exiting normally, received signal 15.
2026-02-22T05:06:20.562366Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]: frame=  194 fps= 32 [hls @ 0x559eae7f1340] Opening 'seg_001.m4s.tmp' for writing=N/A speed=1.16x
2026-02-22T05:06:20.563478Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]: [hls @ 0x559eae7f1340] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:06:20.613045Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]: [hls @ 0x559eae7f1340] Opening 'seg_002.m4s.tmp' for writing
2026-02-22T05:06:20.613111Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]: [hls @ 0x559eae7f1340] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:06:20.613128Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]: [out#0/hls @ 0x559eae802300] video:2337KiB audio:291KiB subtitle:0KiB other streams:0KiB global headers:0KiB muxing overhead: unknown
2026-02-22T05:06:20.613142Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]: frame=  374 fps= 40 q=-1.0 Lsize=N/A time=00:00:12.25 bitrate=N/A dup=4 drop=0 speed= 1.3x
2026-02-22T05:06:20.626603Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]: [libx264 @ 0x559eae7e3500] frame I:3     Avg QP:19.21  size:120396
2026-02-22T05:06:20.626625Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]: [libx264 @ 0x559eae7e3500] frame P:94    Avg QP:22.16  size: 16816
2026-02-22T05:06:20.626629Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]: [libx264 @ 0x559eae7e3500] frame B:277   Avg QP:22.91  size:  1628
2026-02-22T05:06:20.626632Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]: [libx264 @ 0x559eae7e3500] consecutive B-frames:  0.8%  0.5%  2.4% 96.3%
2026-02-22T05:06:20.626636Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]: [libx264 @ 0x559eae7e3500] mb I  I16..4: 14.9% 42.1% 43.0%
2026-02-22T05:06:20.626639Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]: [libx264 @ 0x559eae7e3500] mb P  I16..4:  1.5%  5.7%  0.1%  P16..4: 36.0% 13.9%  4.1%  0.0%  0.0%    skip:38.9%
2026-02-22T05:06:20.626643Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]: [libx264 @ 0x559eae7e3500] mb B  I16..4:  0.1%  0.1%  0.0%  B16..8:  9.2%  1.0%  0.1%  direct: 0.7%  skip:89.0%  L0:33.8% L1:60.3% BI: 5.9%
2026-02-22T05:06:20.626647Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]: [libx264 @ 0x559eae7e3500] 8x8 transform intra:67.1% inter:69.9%
2026-02-22T05:06:20.626651Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]: [libx264 @ 0x559eae7e3500] coded y,uvDC,uvAC intra: 50.6% 41.4% 5.9% inter: 3.2% 2.4% 0.0%
2026-02-22T05:06:20.626654Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]: [libx264 @ 0x559eae7e3500] i16 v,h,dc,p: 48% 31% 14%  8%
2026-02-22T05:06:20.626657Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]: [libx264 @ 0x559eae7e3500] i8 v,h,dc,ddl,ddr,vr,hd,vl,hu: 18% 33% 27%  4%  3%  2%  5%  2%  7%
2026-02-22T05:06:20.626660Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]: [libx264 @ 0x559eae7e3500] i4 v,h,dc,ddl,ddr,vr,hd,vl,hu: 16% 22% 10%  9%  9%  6% 10%  7% 10%
2026-02-22T05:06:20.626663Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]: [libx264 @ 0x559eae7e3500] i8c dc,h,v,p: 58% 26% 13%  4%
2026-02-22T05:06:20.626666Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]: [libx264 @ 0x559eae7e3500] Weighted P-Frames: Y:0.0% UV:0.0%
2026-02-22T05:06:20.626670Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]: [libx264 @ 0x559eae7e3500] kb/s:1535.49
2026-02-22T05:06:20.663483Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]: [aac @ 0x559eaefc2240] Qavg: 595.573
2026-02-22T05:06:20.701448Z  WARN ferrite_stream::hls: ffmpeg HLS [542e39ca-1b17-4d1c-89cc-8b1b8537f863]: Exiting normally, received signal 15.
2026-02-22T05:06:21.610267Z  INFO http_request{method=DELETE uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/session/stop?playback_session_id=6b348531-9998-4e25-9ca7-3a154a80388a}: ferrite_stream::hls: Destroyed HLS session dfc2bf90-4262-4793-abb8-b3112f03b9cb
2026-02-22T05:06:21.610582Z  INFO http_request{method=DELETE uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/session/stop?playback_session_id=6b348531-9998-4e25-9ca7-3a154a80388a}: ferrite_stream::hls: Destroyed HLS session df7cb4a0-be8a-4e62-bc3b-d778f2b03cba
2026-02-22T05:06:21.610628Z  INFO http_request{method=DELETE uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/session/stop?playback_session_id=6b348531-9998-4e25-9ca7-3a154a80388a}: ferrite_stream::hls: Destroyed HLS session 542e39ca-1b17-4d1c-89cc-8b1b8537f863
2026-02-22T05:06:21.610637Z  INFO http_request{method=DELETE uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/session/stop?playback_session_id=6b348531-9998-4e25-9ca7-3a154a80388a}: ferrite_stream::hls: Destroyed HLS session 37ad02fb-0347-420f-a211-796301266068
