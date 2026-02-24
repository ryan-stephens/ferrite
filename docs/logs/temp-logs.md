badchiefy@charon ~/ferrite $ ./ferrite
2026-02-24T21:10:31.175413Z  INFO ferrite: Loaded config from config/ferrite.toml
2026-02-24T21:10:31.175605Z  INFO ferrite_core::config: Data directory: /mnt/mpathj/badchiefy/ferrite/data
2026-02-24T21:10:31.186840Z  INFO ferrite_transcode::hwaccel: HW encoder detection: nvenc=false, qsv=false, vaapi=false
2026-02-24T21:10:31.186876Z  INFO ferrite_transcode::hwaccel: No HW encoders available, using software (libx264)
2026-02-24T21:10:31.186886Z  INFO ferrite: Video encoder: libx264 (backend=software)
2026-02-24T21:10:31.189761Z  INFO ferrite_db: Database connected at /mnt/mpathj/badchiefy/ferrite/data/ferrite.db
2026-02-24T21:10:31.190094Z  INFO ferrite_db: Database migrations applied
2026-02-24T21:10:31.212971Z  INFO ferrite_scanner::watcher: Watching 1 library directories for changes
2026-02-24T21:10:31.213144Z  INFO ferrite: Filesystem watcher started
2026-02-24T21:10:31.215198Z  INFO ferrite_api::router: Serving SPA from: /mnt/mpathj/badchiefy/ferrite/static
2026-02-24T21:10:31.216313Z  INFO ferrite: DLNA server enabled (Ferrite Media Server)
2026-02-24T21:10:31.216326Z  INFO ferrite: Ferrite starting on http://0.0.0.0:12335
2026-02-24T21:10:31.216334Z  INFO ferrite: Open http://localhost:12335 in your browser
2026-02-24T21:10:31.216396Z  WARN ferrite: SSDP server stopped (DLNA discovery unavailable): Operation not permitted (os error 1). On Linux, binding port 1900 requires CAP_NET_BIND_SERVICE or running as root.
2026-02-24T21:10:43.276055Z  INFO ferrite_scanner: Scanning library 'TV Shows' at /home/badchiefy/files/tv
2026-02-24T21:10:43.496014Z  INFO ferrite_scanner: Found 3319 media files in 'TV Shows'
2026-02-24T21:10:43.668044Z  INFO ferrite_scanner: Phase 1 complete for 'TV Shows': 0 new items indexed
2026-02-24T21:10:43.668278Z  INFO ferrite_metadata::enrichment: Enriching metadata for 0 TV shows in library 7c1c6368-910f-4080-97b8-87eaf4fe10f9
2026-02-24T21:10:43.668297Z  INFO ferrite_metadata::enrichment: TV metadata enrichment complete: 0/0 shows enriched
2026-02-24T21:10:43.671072Z  INFO ferrite_metadata::enrichment: Backfilling episode metadata for 7 show(s) in library 7c1c6368-910f-4080-97b8-87eaf4fe10f9
2026-02-24T21:10:44.817528Z  WARN sqlx::query: slow statement: execution time exceeded alert threshold summary="UPDATE episodes SET title …" db.statement="\n\nUPDATE episodes\n           SET title      = COALESCE(?, title),\n               overview   = COALESCE(?, overview),\n               air_date   = COALESCE(?, air_date),\n               still_path = COALESCE(?, still_path)\n           WHERE season_id = ? AND episode_number = ?\n" rows_affected=1 rows_returned=0 elapsed=1.132428608s elapsed_secs=1.1324286080000001 slow_threshold=1s
2026-02-24T21:10:49.264310Z  INFO ferrite_scanner: Scan complete for 'TV Shows': 0 new items indexed
2026-02-24T21:10:49.265919Z  INFO ferrite_api::handlers::library: Scan complete for library 7c1c6368-910f-4080-97b8-87eaf4fe10f9: 0 new items
2026-02-24T21:10:59.505564Z  INFO http_request{method=GET uri=/api/stream/bcde264b-883a-41a3-a729-1582138e780b/hls/master.m3u8?token=[REDACTED]&playback_session_id=2506bf64%2Dc1d4%2D4c3f%2Db1d1%2D8b61f4d2b8f0}: ferrite_api::handlers::stream: Transcode permit acquired: op=hls-master media_id=bcde264b-883a-41a3-a729-1582138e780b wait_ms=0.0
2026-02-24T21:10:59.505614Z  INFO http_request{method=GET uri=/api/stream/bcde264b-883a-41a3-a729-1582138e780b/hls/master.m3u8?token=[REDACTED]&playback_session_id=2506bf64%2Dc1d4%2D4c3f%2Db1d1%2D8b61f4d2b8f0}: ferrite_stream::hls: Creating single HLS session for media bcde264b-883a-41a3-a729-1582138e780b at 0.0s variant=1080p awaiting_promotion=true
2026-02-24T21:10:59.505803Z  INFO http_request{method=GET uri=/api/stream/bcde264b-883a-41a3-a729-1582138e780b/hls/master.m3u8?token=[REDACTED]&playback_session_id=2506bf64%2Dc1d4%2D4c3f%2Db1d1%2D8b61f4d2b8f0}: ferrite_stream::hls: Creating HLS session d80190e3-efd7-460a-bad6-78fd15d8046e for media bcde264b-883a-41a3-a729-1582138e780b at 0.0s variant=1080p (/home/badchiefy/files/tv/The Magicians/Season 1/The Magicians - S01E03 - Consequences of Advanced Spellcasting Bluray-1080p.mkv)
2026-02-24T21:10:59.505894Z  INFO http_request{method=GET uri=/api/stream/bcde264b-883a-41a3-a729-1582138e780b/hls/master.m3u8?token=[REDACTED]&playback_session_id=2506bf64%2Dc1d4%2D4c3f%2Db1d1%2D8b61f4d2b8f0}: ferrite_stream::hls: HLS video passthrough: source is H.264 with no filters, using -c:v copy
2026-02-24T21:10:59.505913Z  INFO http_request{method=GET uri=/api/stream/bcde264b-883a-41a3-a729-1582138e780b/hls/master.m3u8?token=[REDACTED]&playback_session_id=2506bf64%2Dc1d4%2D4c3f%2Db1d1%2D8b61f4d2b8f0}: ferrite_stream::hls: HLS ffmpeg args: ["-hide_banner", "-nostdin", "-i", "/home/badchiefy/files/tv/The Magicians/Season 1/The Magicians - S01E03 - Consequences of Advanced Spellcasting Bluray-1080p.mkv", "-map", "0:v:0", "-map", "0:a:0", "-c:v", "copy", "-c:a", "aac", "-b:a", "192k", "-ac", "2", "-f", "hls", "-hls_time", "6", "-hls_list_size", "0", "-hls_segment_type", "fmp4", "-hls_fmp4_init_filename", "init.mp4", "-hls_segment_filename", "seg_%03d.m4s", "-hls_flags", "independent_segments+temp_file", "playlist.m3u8"]
2026-02-24T21:10:59.586703Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [matroska,webm @ 0x5597934d0240] Could not find codec parameters for stream 2 (Subtitle: hdmv_pgs_subtitle (pgssub)): unspecified size
2026-02-24T21:10:59.586733Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: Consider increasing the value for the 'analyzeduration' (0) and 'probesize' (5000000) options
2026-02-24T21:10:59.588729Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: Input #0, matroska,webm, from '/home/badchiefy/files/tv/The Magicians/Season 1/The Magicians - S01E03 - Consequences of Advanced Spellcasting Bluray-1080p.mkv':
2026-02-24T21:10:59.588744Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:   Metadata:
2026-02-24T21:10:59.588754Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:     title           : The.Magicians.S01E03.Consequences.of.Advanced.Spellcasting.1080p.BluRay.Dts-HDMa5.1.AVC-PiR8
2026-02-24T21:10:59.588767Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:     encoder         : libebml v1.4.2 + libmatroska v1.6.3
2026-02-24T21:10:59.588787Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:     creation_time   : 2022-01-29T03:50:30.000000Z
2026-02-24T21:10:59.588798Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:   Duration: 00:43:02.66, start: 0.000000, bitrate: 13324 kb/s
2026-02-24T21:10:59.588809Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:   Chapters:
2026-02-24T21:10:59.588818Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:     Chapter #0:0: start 0.000000, end 161.328000
2026-02-24T21:10:59.588829Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:       Metadata:
2026-02-24T21:10:59.588838Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         title           : Chapter 1
2026-02-24T21:10:59.588848Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:     Chapter #0:1: start 161.328000, end 1283.574000
2026-02-24T21:10:59.588858Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:       Metadata:
2026-02-24T21:10:59.588867Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         title           : Chapter 2
2026-02-24T21:10:59.588877Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:     Chapter #0:2: start 1283.574000, end 2069.150000
2026-02-24T21:10:59.588887Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:       Metadata:
2026-02-24T21:10:59.588896Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         title           : Chapter 3
2026-02-24T21:10:59.588905Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:     Chapter #0:3: start 2069.150000, end 2582.664000
2026-02-24T21:10:59.588915Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:       Metadata:
2026-02-24T21:10:59.588924Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         title           : Chapter 4
2026-02-24T21:10:59.588942Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:   Stream #0:0: Video: h264 (High), yuv420p(tv, bt709, progressive), 1920x1080 [SAR 1:1 DAR 16:9], 23.98 fps, 23.98 tbr, 1k tbn (default)
2026-02-24T21:10:59.588955Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:       Metadata:
2026-02-24T21:10:59.588964Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         BPS             : 10081748
2026-02-24T21:10:59.588974Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         BPS-eng         : 10081748
2026-02-24T21:10:59.588984Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         DURATION        : 00:43:02.664000000
2026-02-24T21:10:59.588994Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         DURATION-eng    : 00:43:02.664000000
2026-02-24T21:10:59.589004Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         NUMBER_OF_FRAMES: 61922
2026-02-24T21:10:59.589014Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         NUMBER_OF_FRAMES-eng: 61922
2026-02-24T21:10:59.589024Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         NUMBER_OF_BYTES : 3254721176
2026-02-24T21:10:59.589033Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         NUMBER_OF_BYTES-eng: 3254721176
2026-02-24T21:10:59.589044Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         _STATISTICS_WRITING_APP: DVDFab 12.0.6.0
2026-02-24T21:10:59.589054Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         _STATISTICS_WRITING_APP-eng: DVDFab 12.0.6.0
2026-02-24T21:10:59.589064Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         _STATISTICS_WRITING_DATE_UTC: 2022-01-29 03:50:30
2026-02-24T21:10:59.589075Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         _STATISTICS_WRITING_DATE_UTC-eng: 2022-01-29 03:50:30
2026-02-24T21:10:59.589085Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-24T21:10:59.589096Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         _STATISTICS_TAGS-eng: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-24T21:10:59.589110Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:   Stream #0:1(eng): Audio: dts (dca) (DTS-HD MA), 48000 Hz, 5.1(side), s32p (24 bit) (default)
2026-02-24T21:10:59.589121Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:       Metadata:
2026-02-24T21:10:59.589131Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         BPS             : 3190345
2026-02-24T21:10:59.589140Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         BPS-eng         : 3190345
2026-02-24T21:10:59.589150Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         DURATION        : 00:43:02.624000000
2026-02-24T21:10:59.589160Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         DURATION-eng    : 00:43:02.624000000
2026-02-24T21:10:59.589170Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         NUMBER_OF_FRAMES: 242121
2026-02-24T21:10:59.589180Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         NUMBER_OF_FRAMES-eng: 242121
2026-02-24T21:10:59.589190Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         NUMBER_OF_BYTES : 1029932972
2026-02-24T21:10:59.589199Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         NUMBER_OF_BYTES-eng: 1029932972
2026-02-24T21:10:59.589209Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         _STATISTICS_WRITING_APP: DVDFab 12.0.6.0
2026-02-24T21:10:59.589219Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         _STATISTICS_WRITING_APP-eng: DVDFab 12.0.6.0
2026-02-24T21:10:59.589230Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         _STATISTICS_WRITING_DATE_UTC: 2022-01-29 03:50:30
2026-02-24T21:10:59.589241Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         _STATISTICS_WRITING_DATE_UTC-eng: 2022-01-29 03:50:30
2026-02-24T21:10:59.589252Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-24T21:10:59.589262Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         _STATISTICS_TAGS-eng: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-24T21:10:59.589273Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:   Stream #0:2(eng): Subtitle: hdmv_pgs_subtitle (pgssub) (default)
2026-02-24T21:10:59.589283Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:       Metadata:
2026-02-24T21:10:59.589292Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         BPS             : 49454
2026-02-24T21:10:59.589302Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         BPS-eng         : 49454
2026-02-24T21:10:59.589312Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         DURATION        : 00:42:30.422000000
2026-02-24T21:10:59.589322Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         DURATION-eng    : 00:42:30.422000000
2026-02-24T21:10:59.589332Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         NUMBER_OF_FRAMES: 1507
2026-02-24T21:10:59.589342Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         NUMBER_OF_FRAMES-eng: 1507
2026-02-24T21:10:59.589352Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         NUMBER_OF_BYTES : 15766292
2026-02-24T21:10:59.589361Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         NUMBER_OF_BYTES-eng: 15766292
2026-02-24T21:10:59.589371Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         _STATISTICS_WRITING_APP: DVDFab 12.0.6.0
2026-02-24T21:10:59.589382Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         _STATISTICS_WRITING_APP-eng: DVDFab 12.0.6.0
2026-02-24T21:10:59.589392Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         _STATISTICS_WRITING_DATE_UTC: 2022-01-29 03:50:30
2026-02-24T21:10:59.589403Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         _STATISTICS_WRITING_DATE_UTC-eng: 2022-01-29 03:50:30
2026-02-24T21:10:59.589413Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-24T21:10:59.589432Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         _STATISTICS_TAGS-eng: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-24T21:10:59.589443Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: Stream mapping:
2026-02-24T21:10:59.589452Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:   Stream #0:0 -> #0:0 (copy)
2026-02-24T21:10:59.589462Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:   Stream #0:1 -> #0:1 (dts (dca) -> aac (native))
2026-02-24T21:10:59.592107Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'init.mp4' for writing
2026-02-24T21:10:59.592198Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: Output #0, hls, to 'playlist.m3u8':
2026-02-24T21:10:59.592202Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:   Metadata:
2026-02-24T21:10:59.592210Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:     title           : The.Magicians.S01E03.Consequences.of.Advanced.Spellcasting.1080p.BluRay.Dts-HDMa5.1.AVC-PiR8
2026-02-24T21:10:59.592216Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:     encoder         : Lavf61.7.100
2026-02-24T21:10:59.592220Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:   Chapters:
2026-02-24T21:10:59.592229Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:     Chapter #0:0: start 0.000000, end 161.328000
2026-02-24T21:10:59.592234Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:       Metadata:
2026-02-24T21:10:59.592237Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         title           : Chapter 1
2026-02-24T21:10:59.592243Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:     Chapter #0:1: start 161.328000, end 1283.574000
2026-02-24T21:10:59.592248Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:       Metadata:
2026-02-24T21:10:59.592252Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         title           : Chapter 2
2026-02-24T21:10:59.592256Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:     Chapter #0:2: start 1283.574000, end 2069.150000
2026-02-24T21:10:59.592265Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:       Metadata:
2026-02-24T21:10:59.592271Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         title           : Chapter 3
2026-02-24T21:10:59.592275Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:     Chapter #0:3: start 2069.150000, end 2582.664000
2026-02-24T21:10:59.592280Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:       Metadata:
2026-02-24T21:10:59.592284Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         title           : Chapter 4
2026-02-24T21:10:59.592295Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:   Stream #0:0: Video: h264 (High), yuv420p(tv, bt709, progressive), 1920x1080 [SAR 1:1 DAR 16:9], q=2-31, 23.98 fps, 23.98 tbr, 16k tbn (default)
2026-02-24T21:10:59.592301Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:       Metadata:
2026-02-24T21:10:59.592304Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         BPS             : 10081748
2026-02-24T21:10:59.592309Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         BPS-eng         : 10081748
2026-02-24T21:10:59.592313Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         DURATION        : 00:43:02.664000000
2026-02-24T21:10:59.592317Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         DURATION-eng    : 00:43:02.664000000
2026-02-24T21:10:59.592321Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         NUMBER_OF_FRAMES: 61922
2026-02-24T21:10:59.592329Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         NUMBER_OF_FRAMES-eng: 61922
2026-02-24T21:10:59.592337Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         NUMBER_OF_BYTES : 3254721176
2026-02-24T21:10:59.592342Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         NUMBER_OF_BYTES-eng: 3254721176
2026-02-24T21:10:59.592346Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         _STATISTICS_WRITING_APP: DVDFab 12.0.6.0
2026-02-24T21:10:59.592352Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         _STATISTICS_WRITING_APP-eng: DVDFab 12.0.6.0
2026-02-24T21:10:59.592359Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         _STATISTICS_WRITING_DATE_UTC: 2022-01-29 03:50:30
2026-02-24T21:10:59.592368Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         _STATISTICS_WRITING_DATE_UTC-eng: 2022-01-29 03:50:30
2026-02-24T21:10:59.592373Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-24T21:10:59.592378Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         _STATISTICS_TAGS-eng: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-24T21:10:59.592383Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:   Stream #0:1(eng): Audio: aac (LC), 48000 Hz, stereo, fltp, 192 kb/s (default)
2026-02-24T21:10:59.592388Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:       Metadata:
2026-02-24T21:10:59.592392Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         BPS             : 3190345
2026-02-24T21:10:59.592401Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         BPS-eng         : 3190345
2026-02-24T21:10:59.592405Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         DURATION        : 00:43:02.624000000
2026-02-24T21:10:59.592410Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         DURATION-eng    : 00:43:02.624000000
2026-02-24T21:10:59.592416Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         NUMBER_OF_FRAMES: 242121
2026-02-24T21:10:59.592420Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         NUMBER_OF_FRAMES-eng: 242121
2026-02-24T21:10:59.592426Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         NUMBER_OF_BYTES : 1029932972
2026-02-24T21:10:59.592431Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         NUMBER_OF_BYTES-eng: 1029932972
2026-02-24T21:10:59.592436Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         _STATISTICS_WRITING_APP: DVDFab 12.0.6.0
2026-02-24T21:10:59.592441Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         _STATISTICS_WRITING_APP-eng: DVDFab 12.0.6.0
2026-02-24T21:10:59.592446Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         _STATISTICS_WRITING_DATE_UTC: 2022-01-29 03:50:30
2026-02-24T21:10:59.592454Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         _STATISTICS_WRITING_DATE_UTC-eng: 2022-01-29 03:50:30
2026-02-24T21:10:59.592461Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-24T21:10:59.592468Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         _STATISTICS_TAGS-eng: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-24T21:10:59.592475Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]:         encoder         : Lavc61.19.101 aac
2026-02-24T21:10:59.745697Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_000.m4s.tmp' for writing
2026-02-24T21:10:59.761175Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:10:59.762427Z  INFO http_request{method=GET uri=/api/stream/bcde264b-883a-41a3-a729-1582138e780b/hls/master.m3u8?token=[REDACTED]&playback_session_id=2506bf64%2Dc1d4%2D4c3f%2Db1d1%2D8b61f4d2b8f0}: ferrite_stream::hls: HLS session d80190e3-efd7-460a-bad6-78fd15d8046e ready (first segment generated in 0.3s)
2026-02-24T21:10:59.762493Z  INFO http_request{method=GET uri=/api/stream/bcde264b-883a-41a3-a729-1582138e780b/hls/master.m3u8?token=[REDACTED]&playback_session_id=2506bf64%2Dc1d4%2D4c3f%2Db1d1%2D8b61f4d2b8f0}: ferrite_api::handlers::stream: HLS master playlist for bcde264b-883a-41a3-a729-1582138e780b: variants=1 reused=false mode=fast seek_source=none db=0ms seek=0ms session=257ms total=257ms
2026-02-24T21:10:59.952650Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_001.m4s.tmp' for writing
2026-02-24T21:10:59.955065Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:00.131790Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: frame=  402 fps=0.0 q=-1.0 size=N/A time=00:00:16.40 bitrate=N/A sp[hls @ 0x55979351ecc0] Opening 'seg_002.m4s.tmp' for writing
2026-02-24T21:11:00.133758Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:00.258221Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_003.m4s.tmp' for writing
2026-02-24T21:11:00.260702Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:00.421059Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_004.m4s.tmp' for writing
2026-02-24T21:11:00.423350Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:00.639981Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: frame=  844 fps=844 q=-1.0 size=N/A time=00:00:34.75 bitrate=N/A sp[hls @ 0x55979351ecc0] Opening 'seg_005.m4s.tmp' for writing
2026-02-24T21:11:00.642255Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:00.642406Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [mp4 @ 0x7f2f8c0624c0] Packet duration: -16 / dts: 1733608 is out of range
2026-02-24T21:11:00.795862Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_006.m4s.tmp' for writing
2026-02-24T21:11:00.802208Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:00.922877Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_007.m4s.tmp' for writing
2026-02-24T21:11:00.924147Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:01.071784Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_008.m4s.tmp' for writing
2026-02-24T21:11:01.081894Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:01.186666Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: frame= 1322 fps=881 q=-1.0 size=N/A time=00:00:54.72 bitrate=N/A sp[hls @ 0x55979351ecc0] Opening 'seg_009.m4s.tmp' for writing
2026-02-24T21:11:01.188689Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:01.361353Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_010.m4s.tmp' for writing
2026-02-24T21:11:01.363179Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:01.735129Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: frame= 1648 fps=824 q=-1.0 size=N/A time=00:01:08.73 bitrate=N/A sp[hls @ 0x55979351ecc0] Opening 'seg_011.m4s.tmp' for writing
2026-02-24T21:11:01.738023Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:01.738434Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [mp4 @ 0x7f2f8c0624c0] Packet duration: -16 / dts: 3463144 is out of range
2026-02-24T21:11:01.841123Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_012.m4s.tmp' for writing
2026-02-24T21:11:01.849153Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:02.001318Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_013.m4s.tmp' for writing
2026-02-24T21:11:02.008561Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:02.171786Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: frame= 2100 fps=840 q=-1.0 size=N/A time=00:01:27.12 bitrate=N/A sp[hls @ 0x55979351ecc0] Opening 'seg_014.m4s.tmp' for writing
2026-02-24T21:11:02.178586Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:02.178802Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [mp4 @ 0x7f2f8c0624c0] Packet duration: -16 / dts: 4327416 is out of range
2026-02-24T21:11:02.331233Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_015.m4s.tmp' for writing
2026-02-24T21:11:02.338073Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:02.486569Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_016.m4s.tmp' for writing
2026-02-24T21:11:02.488802Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:02.645856Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: frame= 2532 fps=844 q=-1.0 size=N/A time=00:01:45.23 bitrate=N/A sp[hls @ 0x55979351ecc0] Opening 'seg_017.m4s.tmp' for writing
2026-02-24T21:11:02.647996Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:02.790865Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_018.m4s.tmp' for writing
2026-02-24T21:11:02.792747Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:02.997199Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_019.m4s.tmp' for writing
2026-02-24T21:11:03.000677Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:03.540397Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: frame= 2890 fps=826 q=-1.0 size=N/A time=00:02:00.53 bitrate=N/A sp[hls @ 0x55979351ecc0] Opening 'seg_020.m4s.tmp' for writing
2026-02-24T21:11:03.542643Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:03.742116Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: frame= 3076 fps=769 q=-1.0 size=N/A time=00:02:07.89 bitrate=N/A sp[hls @ 0x55979351ecc0] Opening 'seg_021.m4s.tmp' for writing
2026-02-24T21:11:03.749789Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:03.878396Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_022.m4s.tmp' for writing
2026-02-24T21:11:03.888460Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:04.118825Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: frame= 3434 fps=763 q=-1.0 size=N/A time=00:02:23.08 bitrate=N/A sp[hls @ 0x55979351ecc0] Opening 'seg_023.m4s.tmp' for writing
2026-02-24T21:11:04.128899Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:04.251182Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_024.m4s.tmp' for writing
2026-02-24T21:11:04.260154Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:04.260373Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [mp4 @ 0x7f2f8c0624c0] Packet duration: -16 / dts: 7210984 is out of range
2026-02-24T21:11:04.506036Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_025.m4s.tmp' for writing
2026-02-24T21:11:04.507970Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:04.665747Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: frame= 3846 fps=769 q=-1.0 size=N/A time=00:02:40.31 bitrate=N/A sp[hls @ 0x55979351ecc0] Opening 'seg_026.m4s.tmp' for writing
2026-02-24T21:11:04.667752Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:04.827015Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_027.m4s.tmp' for writing
2026-02-24T21:11:04.835637Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:05.076192Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_028.m4s.tmp' for writing
2026-02-24T21:11:05.083087Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:05.318467Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: frame= 4197 fps=763 q=-1.0 size=N/A time=00:02:54.61 bitrate=N/A sp[hls @ 0x55979351ecc0] Opening 'seg_029.m4s.tmp' for writing
2026-02-24T21:11:05.325771Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:05.559739Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_030.m4s.tmp' for writing
2026-02-24T21:11:05.562061Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:05.681536Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: frame= 4508 fps=751 q=-1.0 size=N/A time=00:03:07.69 bitrate=N/A sp[hls @ 0x55979351ecc0] Opening 'seg_031.m4s.tmp' for writing
2026-02-24T21:11:05.683476Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:05.838685Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_032.m4s.tmp' for writing
2026-02-24T21:11:05.840821Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:06.131236Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: frame= 4874 fps=750 q=-1.0 size=N/A time=00:03:22.87 bitrate=N/A sp[hls @ 0x55979351ecc0] Opening 'seg_033.m4s.tmp' for writing
2026-02-24T21:11:06.133277Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:06.309375Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_034.m4s.tmp' for writing
2026-02-24T21:11:06.317681Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:06.457417Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_035.m4s.tmp' for writing
2026-02-24T21:11:06.460156Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:06.627954Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: frame= 5308 fps=758 q=-1.0 size=N/A time=00:03:40.94 bitrate=N/A sp[hls @ 0x55979351ecc0] Opening 'seg_036.m4s.tmp' for writing
2026-02-24T21:11:06.636870Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:06.805962Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_037.m4s.tmp' for writing
2026-02-24T21:11:06.814793Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:06.966146Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_038.m4s.tmp' for writing
2026-02-24T21:11:06.968713Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:07.139482Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: frame= 5742 fps=765 q=-1.0 size=N/A time=00:03:59.03 bitrate=N/A sp[hls @ 0x55979351ecc0] Opening 'seg_039.m4s.tmp' for writing
2026-02-24T21:11:07.142122Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:07.284999Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_040.m4s.tmp' for writing
2026-02-24T21:11:07.287196Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:07.287427Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [mp4 @ 0x7f2f8c0624c0] Packet duration: -16 / dts: 11823096 is out of range
2026-02-24T21:11:07.444555Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_041.m4s.tmp' for writing
2026-02-24T21:11:07.451741Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:08.191900Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: frame= 6126 fps=766 q=-1.0 size=N/A time=00:04:15.50 bitrate=N/A sp[hls @ 0x55979351ecc0] Opening 'seg_042.m4s.tmp' for writing=N/A speed=30.1x
2026-02-24T21:11:08.199961Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:08.439983Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_043.m4s.tmp' for writing
2026-02-24T21:11:08.447111Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:08.604045Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: frame= 6484 fps=720 q=-1.0 size=N/A time=00:04:30.03 bitrate=N/A sp[hls @ 0x55979351ecc0] Opening 'seg_044.m4s.tmp' for writing
2026-02-24T21:11:08.610625Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:08.773296Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_045.m4s.tmp' for writing
2026-02-24T21:11:08.781008Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:09.114410Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: frame= 6764 fps=712 q=-1.0 size=N/A time=00:04:41.70 bitrate=N/A sp[hls @ 0x55979351ecc0] Opening 'seg_046.m4s.tmp' for writing
2026-02-24T21:11:09.116380Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:09.239842Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_047.m4s.tmp' for writing
2026-02-24T21:11:09.241287Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:09.463414Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_048.m4s.tmp' for writing
2026-02-24T21:11:09.465643Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:09.592013Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: frame= 7212 fps=721 q=-1.0 size=N/A time=00:05:00.50 bitrate=N/A sp[hls @ 0x55979351ecc0] Opening 'seg_049.m4s.tmp' for writing
2026-02-24T21:11:09.603160Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:09.730840Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_050.m4s.tmp' for writing
2026-02-24T21:11:09.737821Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:09.945007Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_051.m4s.tmp' for writing
2026-02-24T21:11:09.946892Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:10.067463Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_052.m4s.tmp' for writing
2026-02-24T21:11:10.069263Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:10.194629Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: frame= 7668 fps=730 q=-1.0 size=N/A time=00:05:19.42 bitrate=N/A sp[hls @ 0x55979351ecc0] Opening 'seg_053.m4s.tmp' for writing
2026-02-24T21:11:10.202832Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:10.203103Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [mp4 @ 0x7f2f8c0624c0] Packet duration: -16 / dts: 15570936 is out of range
2026-02-24T21:11:10.395085Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_054.m4s.tmp' for writing
2026-02-24T21:11:10.396880Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:10.515747Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_055.m4s.tmp' for writing
2026-02-24T21:11:10.517935Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:10.663990Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: frame= 8128 fps=739 q=-1.0 size=N/A time=00:05:38.62 bitrate=N/A sp[hls @ 0x55979351ecc0] Opening 'seg_056.m4s.tmp' for writing
2026-02-24T21:11:10.672249Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:10.672520Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [mp4 @ 0x7f2f8c0624c0] Packet duration: -16 / dts: 16436200 is out of range
2026-02-24T21:11:10.803330Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_057.m4s.tmp' for writing
2026-02-24T21:11:10.811881Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:10.998220Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_058.m4s.tmp' for writing
2026-02-24T21:11:11.005476Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:11.627063Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: frame= 8548 fps=743 q=-1.0 size=N/A time=00:05:56.52 bitrate=N/A sp[hls @ 0x55979351ecc0] Opening 'seg_059.m4s.tmp' for writing=N/A speed=  30x
2026-02-24T21:11:11.629419Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:11.753635Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_060.m4s.tmp' for writing
2026-02-24T21:11:11.755732Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:11.896157Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_061.m4s.tmp' for writing
2026-02-24T21:11:11.898540Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:12.117492Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: frame= 9049 fps=724 q=-1.0 size=N/A time=00:06:17.13 bitrate=N/A sp[hls @ 0x55979351ecc0] Opening 'seg_062.m4s.tmp' for writing
2026-02-24T21:11:12.119729Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:12.119975Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [mp4 @ 0x7f2f8c0624c0] Packet duration: -16 / dts: 18165736 is out of range
2026-02-24T21:11:12.281601Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_063.m4s.tmp' for writing
2026-02-24T21:11:12.289603Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:12.402137Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_064.m4s.tmp' for writing
2026-02-24T21:11:12.409199Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:12.512513Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_065.m4s.tmp' for writing
2026-02-24T21:11:12.513988Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:12.641442Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: frame= 9606 fps=739 q=-1.0 size=N/A time=00:06:40.21 bitrate=N/A sp[hls @ 0x55979351ecc0] Opening 'seg_066.m4s.tmp' for writing
2026-02-24T21:11:12.649892Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:12.650446Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [mp4 @ 0x7f2f8c0624c0] Packet duration: -16 / dts: 19318776 is out of range
2026-02-24T21:11:13.052927Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_067.m4s.tmp' for writing
2026-02-24T21:11:13.060174Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:13.210437Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: frame= 9842 fps=729 q=-1.0 size=N/A time=00:06:50.15 bitrate=N/A sp[hls @ 0x55979351ecc0] Opening 'seg_068.m4s.tmp' for writing
2026-02-24T21:11:13.217408Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:13.389039Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_069.m4s.tmp' for writing
2026-02-24T21:11:13.391200Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:13.499315Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_070.m4s.tmp' for writing
2026-02-24T21:11:13.505885Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:13.615936Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: frame=10349 fps=739 q=-1.0 size=N/A time=00:07:11.23 bitrate=N/A sp[hls @ 0x55979351ecc0] Opening 'seg_071.m4s.tmp' for writing
2026-02-24T21:11:13.618765Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:13.719582Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_072.m4s.tmp' for writing
2026-02-24T21:11:13.721434Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:13.721674Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [mp4 @ 0x7f2f8c0624c0] Packet duration: -16 / dts: 21048312 is out of range
2026-02-24T21:11:13.833673Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_073.m4s.tmp' for writing
2026-02-24T21:11:13.843666Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:14.385729Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: frame=10797 fps=744 q=-1.0 size=N/A time=00:07:30.23 bitrate=N/A sp[hls @ 0x55979351ecc0] Opening 'seg_074.m4s.tmp' for writing
2026-02-24T21:11:14.392903Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:14.531246Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_075.m4s.tmp' for writing
2026-02-24T21:11:14.538530Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:14.655285Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: frame=11044 fps=736 q=-1.0 size=N/A time=00:07:40.20 bitrate=N/A sp[hls @ 0x55979351ecc0] Opening 'seg_076.m4s.tmp' for writing
2026-02-24T21:11:14.663214Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:14.762522Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_077.m4s.tmp' for writing
2026-02-24T21:11:14.764909Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:15.045974Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_078.m4s.tmp' for writing
2026-02-24T21:11:15.048455Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:15.172839Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: frame=11440 fps=738 q=-1.0 size=N/A time=00:07:56.73 bitrate=N/A sp[hls @ 0x55979351ecc0] Opening 'seg_079.m4s.tmp' for writing
2026-02-24T21:11:15.174958Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:15.425314Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_080.m4s.tmp' for writing
2026-02-24T21:11:15.433292Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:15.575320Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_081.m4s.tmp' for writing
2026-02-24T21:11:15.577578Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:15.690074Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: frame=11844 fps=740 q=-1.0 size=N/A time=00:08:13.58 bitrate=N/A sp[hls @ 0x55979351ecc0] Opening 'seg_082.m4s.tmp' for writing
2026-02-24T21:11:15.692171Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:15.804316Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_083.m4s.tmp' for writing
2026-02-24T21:11:15.806733Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:15.912034Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_084.m4s.tmp' for writing
2026-02-24T21:11:15.913753Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:16.134628Z  INFO http_request{method=DELETE uri=/api/stream/bcde264b-883a-41a3-a729-1582138e780b/hls/session/stop?playback_session_id=2506bf64-c1d4-4c3f-b1d1-8b61f4d2b8f0}: ferrite_stream::hls: Destroyed HLS session d80190e3-efd7-460a-bad6-78fd15d8046e
2026-02-24T21:11:16.163612Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'seg_085.m4s.tmp' for writing
2026-02-24T21:11:16.163648Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Failed to open file 'seg_085.m4s.tmp'
2026-02-24T21:11:16.166731Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] failed to rename file seg_085.m4s.tmp to seg_085.m4s: No such file or directory
2026-02-24T21:11:16.166764Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: fatal error detected, marking session failed
2026-02-24T21:11:16.166788Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:16.166802Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [hls @ 0x55979351ecc0] failed to rename file playlist.m3u8.tmp to playlist.m3u8: No such file or directory
2026-02-24T21:11:16.166813Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: fatal error detected, marking session failed
2026-02-24T21:11:16.166830Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [out#0/hls @ 0x559793526000] video:635064KiB audio:12250KiB subtitle:0KiB other streams:0KiB global headers:0KiB muxing overhead: unknown
2026-02-24T21:11:16.166844Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: frame=12336 fps=744 q=-1.0 Lsize=N/A time=00:08:34.51 bitrate=N/A speed=  31x
2026-02-24T21:11:16.166863Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: [aac @ 0x5597934d5ac0] Qavg: 860.921
2026-02-24T21:11:16.167300Z  WARN ferrite_stream::hls: ffmpeg HLS [d80190e3-efd7-460a-bad6-78fd15d8046e]: Exiting normally, received signal 15.
2026-02-24T21:11:52.314268Z  INFO http_request{method=POST uri=/api/stream/bcde264b-883a-41a3-a729-1582138e780b/hls/seek?start=1356.244&playback_session_id=2506bf64-c1d4-4c3f-b1d1-8b61f4d2b8f0}: ferrite_api::handlers::stream: Transcode permit acquired: op=hls-seek media_id=bcde264b-883a-41a3-a729-1582138e780b wait_ms=0.0
2026-02-24T21:11:52.314310Z  INFO http_request{method=POST uri=/api/stream/bcde264b-883a-41a3-a729-1582138e780b/hls/seek?start=1356.244&playback_session_id=2506bf64-c1d4-4c3f-b1d1-8b61f4d2b8f0}: ferrite_stream::hls: Creating single HLS session for media bcde264b-883a-41a3-a729-1582138e780b at 1355.4s variant=1080p awaiting_promotion=false
2026-02-24T21:11:52.314674Z  INFO http_request{method=POST uri=/api/stream/bcde264b-883a-41a3-a729-1582138e780b/hls/seek?start=1356.244&playback_session_id=2506bf64-c1d4-4c3f-b1d1-8b61f4d2b8f0}: ferrite_stream::hls: Creating HLS session f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a for media bcde264b-883a-41a3-a729-1582138e780b at 1355.4s variant=1080p (/home/badchiefy/files/tv/The Magicians/Season 1/The Magicians - S01E03 - Consequences of Advanced Spellcasting Bluray-1080p.mkv)
2026-02-24T21:11:52.314745Z  INFO http_request{method=POST uri=/api/stream/bcde264b-883a-41a3-a729-1582138e780b/hls/seek?start=1356.244&playback_session_id=2506bf64-c1d4-4c3f-b1d1-8b61f4d2b8f0}: ferrite_stream::hls: HLS ffmpeg args: ["-hide_banner", "-nostdin", "-ss", "1355.396", "-i", "/home/badchiefy/files/tv/The Magicians/Season 1/The Magicians - S01E03 - Consequences of Advanced Spellcasting Bluray-1080p.mkv", "-ss", "0.848", "-map", "0:v:0", "-map", "0:a:0", "-c:v", "libx264", "-preset", "veryfast", "-crf", "23", "-profile:v", "high", "-level", "4.1", "-pix_fmt", "yuv420p", "-g", "144", "-keyint_min", "144", "-c:a", "aac", "-b:a", "192k", "-ac", "2", "-f", "hls", "-hls_time", "6", "-hls_list_size", "30", "-hls_segment_type", "fmp4", "-hls_fmp4_init_filename", "init.mp4", "-hls_segment_filename", "seg_%03d.m4s", "-hls_flags", "independent_segments+delete_segments+temp_file", "playlist.m3u8"]
2026-02-24T21:11:52.340754Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [matroska,webm @ 0x561ef0ab8500] Could not find codec parameters for stream 2 (Subtitle: hdmv_pgs_subtitle (pgssub)): unspecified size
2026-02-24T21:11:52.340821Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: Consider increasing the value for the 'analyzeduration' (0) and 'probesize' (5000000) options
2026-02-24T21:11:52.343699Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: Input #0, matroska,webm, from '/home/badchiefy/files/tv/The Magicians/Season 1/The Magicians - S01E03 - Consequences of Advanced Spellcasting Bluray-1080p.mkv':
2026-02-24T21:11:52.343721Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:   Metadata:
2026-02-24T21:11:52.343737Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:     title           : The.Magicians.S01E03.Consequences.of.Advanced.Spellcasting.1080p.BluRay.Dts-HDMa5.1.AVC-PiR8
2026-02-24T21:11:52.343755Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:     encoder         : libebml v1.4.2 + libmatroska v1.6.3
2026-02-24T21:11:52.343769Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:     creation_time   : 2022-01-29T03:50:30.000000Z
2026-02-24T21:11:52.343794Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:   Duration: 00:43:02.66, start: 0.000000, bitrate: 13324 kb/s
2026-02-24T21:11:52.343810Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:   Chapters:
2026-02-24T21:11:52.343820Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:     Chapter #0:0: start 0.000000, end 161.328000
2026-02-24T21:11:52.343831Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:       Metadata:
2026-02-24T21:11:52.343842Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         title           : Chapter 1
2026-02-24T21:11:52.343854Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:     Chapter #0:1: start 161.328000, end 1283.574000
2026-02-24T21:11:52.343865Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:       Metadata:
2026-02-24T21:11:52.343874Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         title           : Chapter 2
2026-02-24T21:11:52.343884Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:     Chapter #0:2: start 1283.574000, end 2069.150000
2026-02-24T21:11:52.343896Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:       Metadata:
2026-02-24T21:11:52.343905Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         title           : Chapter 3
2026-02-24T21:11:52.343916Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:     Chapter #0:3: start 2069.150000, end 2582.664000
2026-02-24T21:11:52.343927Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:       Metadata:
2026-02-24T21:11:52.343936Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         title           : Chapter 4
2026-02-24T21:11:52.343946Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:   Stream #0:0: Video: h264 (High), yuv420p(tv, bt709, progressive), 1920x1080 [SAR 1:1 DAR 16:9], 23.98 fps, 23.98 tbr, 1k tbn (default)
2026-02-24T21:11:52.343959Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:       Metadata:
2026-02-24T21:11:52.343968Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         BPS             : 10081748
2026-02-24T21:11:52.343978Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         BPS-eng         : 10081748
2026-02-24T21:11:52.343989Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         DURATION        : 00:43:02.664000000
2026-02-24T21:11:52.344001Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         DURATION-eng    : 00:43:02.664000000
2026-02-24T21:11:52.344012Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         NUMBER_OF_FRAMES: 61922
2026-02-24T21:11:52.344024Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         NUMBER_OF_FRAMES-eng: 61922
2026-02-24T21:11:52.344036Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         NUMBER_OF_BYTES : 3254721176
2026-02-24T21:11:52.344045Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         NUMBER_OF_BYTES-eng: 3254721176
2026-02-24T21:11:52.344062Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         _STATISTICS_WRITING_APP: DVDFab 12.0.6.0
2026-02-24T21:11:52.344093Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         _STATISTICS_WRITING_APP-eng: DVDFab 12.0.6.0
2026-02-24T21:11:52.344143Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         _STATISTICS_WRITING_DATE_UTC: 2022-01-29 03:50:30
2026-02-24T21:11:52.344156Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         _STATISTICS_WRITING_DATE_UTC-eng: 2022-01-29 03:50:30
2026-02-24T21:11:52.344168Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-24T21:11:52.344183Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         _STATISTICS_TAGS-eng: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-24T21:11:52.344200Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:   Stream #0:1(eng): Audio: dts (dca) (DTS-HD MA), 48000 Hz, 5.1(side), s32p (24 bit) (default)
2026-02-24T21:11:52.344277Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:       Metadata:
2026-02-24T21:11:52.344290Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         BPS             : 3190345
2026-02-24T21:11:52.344301Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         BPS-eng         : 3190345
2026-02-24T21:11:52.344316Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         DURATION        : 00:43:02.624000000
2026-02-24T21:11:52.344331Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         DURATION-eng    : 00:43:02.624000000
2026-02-24T21:11:52.344344Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         NUMBER_OF_FRAMES: 242121
2026-02-24T21:11:52.344392Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         NUMBER_OF_FRAMES-eng: 242121
2026-02-24T21:11:52.344404Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         NUMBER_OF_BYTES : 1029932972
2026-02-24T21:11:52.344418Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         NUMBER_OF_BYTES-eng: 1029932972
2026-02-24T21:11:52.344430Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         _STATISTICS_WRITING_APP: DVDFab 12.0.6.0
2026-02-24T21:11:52.344445Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         _STATISTICS_WRITING_APP-eng: DVDFab 12.0.6.0
2026-02-24T21:11:52.344460Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         _STATISTICS_WRITING_DATE_UTC: 2022-01-29 03:50:30
2026-02-24T21:11:52.344474Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         _STATISTICS_WRITING_DATE_UTC-eng: 2022-01-29 03:50:30
2026-02-24T21:11:52.344489Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-24T21:11:52.344502Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         _STATISTICS_TAGS-eng: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-24T21:11:52.344518Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:   Stream #0:2(eng): Subtitle: hdmv_pgs_subtitle (pgssub) (default)
2026-02-24T21:11:52.344531Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:       Metadata:
2026-02-24T21:11:52.344545Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         BPS             : 49454
2026-02-24T21:11:52.344560Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         BPS-eng         : 49454
2026-02-24T21:11:52.344572Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         DURATION        : 00:42:30.422000000
2026-02-24T21:11:52.344587Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         DURATION-eng    : 00:42:30.422000000
2026-02-24T21:11:52.344602Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         NUMBER_OF_FRAMES: 1507
2026-02-24T21:11:52.344616Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         NUMBER_OF_FRAMES-eng: 1507
2026-02-24T21:11:52.344629Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         NUMBER_OF_BYTES : 15766292
2026-02-24T21:11:52.344642Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         NUMBER_OF_BYTES-eng: 15766292
2026-02-24T21:11:52.344656Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         _STATISTICS_WRITING_APP: DVDFab 12.0.6.0
2026-02-24T21:11:52.344671Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         _STATISTICS_WRITING_APP-eng: DVDFab 12.0.6.0
2026-02-24T21:11:52.344683Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         _STATISTICS_WRITING_DATE_UTC: 2022-01-29 03:50:30
2026-02-24T21:11:52.344698Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         _STATISTICS_WRITING_DATE_UTC-eng: 2022-01-29 03:50:30
2026-02-24T21:11:52.344712Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-24T21:11:52.344726Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         _STATISTICS_TAGS-eng: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-24T21:11:52.348876Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: Stream mapping:
2026-02-24T21:11:52.348899Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:   Stream #0:0 -> #0:0 (h264 (native) -> h264 (libx264))
2026-02-24T21:11:52.348918Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:   Stream #0:1 -> #0:1 (dts (dca) -> aac (native))
2026-02-24T21:11:52.416571Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [libx264 @ 0x561ef0abdd80] using SAR=1/1
2026-02-24T21:11:52.417933Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [libx264 @ 0x561ef0abdd80] using cpu capabilities: MMX2 SSE2Fast SSSE3 SSE4.2 AVX FMA3 BMI2 AVX2
2026-02-24T21:11:52.441472Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [libx264 @ 0x561ef0abdd80] profile High, level 4.1, 4:2:0, 8-bit
2026-02-24T21:11:52.441960Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [libx264 @ 0x561ef0abdd80] 264 - core 164 - H.264/MPEG-4 AVC codec - Copyleft 2003-2024 - http://www.videolan.org/x264.html - options: cabac=1 ref=1 deblock=1:0:0 analyse=0x3:0x113 me=hex subme=2 psy=1 psy_rd=1.00:0.00 mixed_ref=0 me_range=16 chroma_me=1 trellis=0 8x8dct=1 cqm=0 deadzone=21,11 fast_pskip=1 chroma_qp_offset=0 threads=34 lookahead_threads=8 sliced_threads=0 nr=0 decimate=1 interlaced=0 bluray_compat=0 constrained_intra=0 bframes=3 b_pyramid=2 b_adapt=1 b_bias=0 direct=1 weightb=1 open_gop=0 weightp=1 keyint=144 keyint_min=73 scenecut=40 intra_refresh=0 rc_lookahead=10 rc=crf mbtree=1 crf=23.0 qcomp=0.60 qpmin=0 qpmax=69 qpstep=4 ip_ratio=1.40 aq=1:1.00
2026-02-24T21:11:52.442123Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'init.mp4' for writing
2026-02-24T21:11:52.442269Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: Output #0, hls, to 'playlist.m3u8':
2026-02-24T21:11:52.442282Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:   Metadata:
2026-02-24T21:11:52.442305Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:     title           : The.Magicians.S01E03.Consequences.of.Advanced.Spellcasting.1080p.BluRay.Dts-HDMa5.1.AVC-PiR8
2026-02-24T21:11:52.442319Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:     encoder         : Lavf61.7.100
2026-02-24T21:11:52.442333Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:   Chapters:
2026-02-24T21:11:52.442346Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:     Chapter #0:0: start 0.000000, end 712.906000
2026-02-24T21:11:52.442361Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:       Metadata:
2026-02-24T21:11:52.442414Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         title           : Chapter 3
2026-02-24T21:11:52.442427Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:     Chapter #0:1: start 712.906000, end 1226.420000
2026-02-24T21:11:52.442440Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:       Metadata:
2026-02-24T21:11:52.442462Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         title           : Chapter 4
2026-02-24T21:11:52.442498Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:   Stream #0:0: Video: h264, yuv420p(tv, bt709, progressive), 1920x1080 [SAR 1:1 DAR 16:9], q=2-31, 23.98 fps, 24k tbn (default)
2026-02-24T21:11:52.442513Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:       Metadata:
2026-02-24T21:11:52.442535Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         BPS             : 10081748
2026-02-24T21:11:52.442547Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         BPS-eng         : 10081748
2026-02-24T21:11:52.442558Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         DURATION        : 00:43:02.664000000
2026-02-24T21:11:52.442573Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         DURATION-eng    : 00:43:02.664000000
2026-02-24T21:11:52.442588Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         NUMBER_OF_FRAMES: 61922
2026-02-24T21:11:52.442612Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         NUMBER_OF_FRAMES-eng: 61922
2026-02-24T21:11:52.442624Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         NUMBER_OF_BYTES : 3254721176
2026-02-24T21:11:52.442639Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         NUMBER_OF_BYTES-eng: 3254721176
2026-02-24T21:11:52.442651Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         _STATISTICS_WRITING_APP: DVDFab 12.0.6.0
2026-02-24T21:11:52.442665Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         _STATISTICS_WRITING_APP-eng: DVDFab 12.0.6.0
2026-02-24T21:11:52.442680Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         _STATISTICS_WRITING_DATE_UTC: 2022-01-29 03:50:30
2026-02-24T21:11:52.442693Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         _STATISTICS_WRITING_DATE_UTC-eng: 2022-01-29 03:50:30
2026-02-24T21:11:52.442708Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-24T21:11:52.442722Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         _STATISTICS_TAGS-eng: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-24T21:11:52.442737Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         encoder         : Lavc61.19.101 libx264
2026-02-24T21:11:52.442749Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:       Side data:
2026-02-24T21:11:52.442762Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         cpb: bitrate max/min/avg: 0/0/0 buffer size: 0 vbv_delay: N/A
2026-02-24T21:11:52.442784Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:   Stream #0:1(eng): Audio: aac (LC), 48000 Hz, stereo, fltp, 192 kb/s (default)
2026-02-24T21:11:52.442798Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:       Metadata:
2026-02-24T21:11:52.442810Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         BPS             : 3190345
2026-02-24T21:11:52.442825Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         BPS-eng         : 3190345
2026-02-24T21:11:52.442838Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         DURATION        : 00:43:02.624000000
2026-02-24T21:11:52.442853Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         DURATION-eng    : 00:43:02.624000000
2026-02-24T21:11:52.442866Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         NUMBER_OF_FRAMES: 242121
2026-02-24T21:11:52.442905Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         NUMBER_OF_FRAMES-eng: 242121
2026-02-24T21:11:52.442916Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         NUMBER_OF_BYTES : 1029932972
2026-02-24T21:11:52.442925Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         NUMBER_OF_BYTES-eng: 1029932972
2026-02-24T21:11:52.442937Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         _STATISTICS_WRITING_APP: DVDFab 12.0.6.0
2026-02-24T21:11:52.442967Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         _STATISTICS_WRITING_APP-eng: DVDFab 12.0.6.0
2026-02-24T21:11:52.442978Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         _STATISTICS_WRITING_DATE_UTC: 2022-01-29 03:50:30
2026-02-24T21:11:52.442989Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         _STATISTICS_WRITING_DATE_UTC-eng: 2022-01-29 03:50:30
2026-02-24T21:11:52.443000Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         _STATISTICS_TAGS: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-24T21:11:52.443011Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         _STATISTICS_TAGS-eng: BPS DURATION NUMBER_OF_FRAMES NUMBER_OF_BYTES
2026-02-24T21:11:52.443022Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]:         encoder         : Lavc61.19.101 aac
2026-02-24T21:11:53.544191Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame=   69 fps=0.0 q=28.0 size=N/A time=00:00:02.79 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_000.m4s.tmp' for writing=N/A dup=1 drop=0 speed=9.38x
2026-02-24T21:11:53.550327Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:53.550567Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [mp4 @ 0x7fd5644a1000] Packet duration: -16 / dts: 567172 is out of range
2026-02-24T21:11:53.639868Z  INFO http_request{method=POST uri=/api/stream/bcde264b-883a-41a3-a729-1582138e780b/hls/seek?start=1356.244&playback_session_id=2506bf64-c1d4-4c3f-b1d1-8b61f4d2b8f0}: ferrite_stream::hls: HLS session f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a ready (first segment generated in 1.3s)
2026-02-24T21:11:53.639928Z  INFO http_request{method=POST uri=/api/stream/bcde264b-883a-41a3-a729-1582138e780b/hls/seek?start=1356.244&playback_session_id=2506bf64-c1d4-4c3f-b1d1-8b61f4d2b8f0}: ferrite_api::handlers::stream: HLS seek for bcde264b-883a-41a3-a729-1582138e780b: start=1355.4s mode=fast seek_source=index-lazy db=1ms seek=42758ms session=1326ms total=44085ms
2026-02-24T21:11:53.696021Z  INFO http_request{method=GET uri=/api/stream/bcde264b-883a-41a3-a729-1582138e780b/hls/master.m3u8?start=1356.244&token=[REDACTED]&playback_session_id=2506bf64%2Dc1d4%2D4c3f%2Db1d1%2D8b61f4d2b8f0}: ferrite_api::handlers::stream: HLS master playlist for bcde264b-883a-41a3-a729-1582138e780b: variants=1 reused=true mode=fast seek_source=index db=0ms seek=0ms session=0ms total=1ms
2026-02-24T21:11:54.095648Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame=  347 fps=231 q=28.0 size=N/A time=00:00:14.38 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_001.m4s.tmp' for writing
2026-02-24T21:11:54.096097Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:54.330948Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'seg_002.m4s.tmp' for writing
2026-02-24T21:11:54.331293Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:54.783075Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame=  494 fps=247 q=28.0 size=N/A time=00:00:20.52 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_003.m4s.tmp' for writing
2026-02-24T21:11:54.783387Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:54.783614Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [mp4 @ 0x7fd5644a1000] Packet duration: -16 / dts: 1209220 is out of range
2026-02-24T21:11:55.233178Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame=  622 fps=249 q=28.0 size=N/A time=00:00:25.85 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_004.m4s.tmp' for writing
2026-02-24T21:11:55.234048Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:56.281010Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame=  752 fps=251 q=28.0 size=N/A time=00:00:31.28 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_005.m4s.tmp' for writing=N/A dup=1 drop=0 speed=9.72x
2026-02-24T21:11:56.281885Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:56.284963Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [mp4 @ 0x7fd5644a1000] Packet duration: -16 / dts: 1856404 is out of range
2026-02-24T21:11:56.826988Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame=  942 fps=235 q=28.0 size=N/A time=00:00:39.20 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_006.m4s.tmp' for writing
2026-02-24T21:11:56.827636Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:58.586366Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame= 1027 fps=228 q=28.0 size=N/A time=00:00:42.75 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_007.m4s.tmp' for writing=N/A dup=1 drop=0 speed= 8.2x
2026-02-24T21:11:58.589846Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:58.593632Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [mp4 @ 0x7fd5644a1000] Packet duration: -16 / dts: 2453380 is out of range
2026-02-24T21:11:58.890087Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame= 1302 fps=200 q=28.0 size=N/A time=00:00:54.22 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_008.m4s.tmp' for writing
2026-02-24T21:11:58.890459Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:11:59.715081Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame= 1415 fps=202 q=28.0 size=N/A time=00:00:58.93 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_009.m4s.tmp' for writing
2026-02-24T21:11:59.716806Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:00.220919Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame= 1544 fps=206 q=28.0 size=N/A time=00:01:04.31 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_010.m4s.tmp' for writing
2026-02-24T21:12:00.224549Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:00.660951Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame= 1672 fps=209 q=28.0 size=N/A time=00:01:09.65 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_011.m4s.tmp' for writing
2026-02-24T21:12:00.662962Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:01.463407Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame= 1803 fps=212 q=28.0 size=N/A time=00:01:15.11 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_012.m4s.tmp' for writing=N/A dup=1 drop=0 speed=8.85x
2026-02-24T21:12:01.466142Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:02.015597Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame= 2011 fps=212 q=28.0 size=N/A time=00:01:23.79 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_013.m4s.tmp' for writing
2026-02-24T21:12:02.016017Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:02.023497Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [mp4 @ 0x7fd5644a1000] Packet duration: -16 / dts: 4094852 is out of range
2026-02-24T21:12:02.846604Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame= 2130 fps=213 q=28.0 size=N/A time=00:01:28.75 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_014.m4s.tmp' for writing
2026-02-24T21:12:02.849911Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:03.327975Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame= 2286 fps=218 q=28.0 size=N/A time=00:01:35.26 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_015.m4s.tmp' for writing
2026-02-24T21:12:03.328330Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:04.006262Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame= 2407 fps=219 q=28.0 size=N/A time=00:01:40.30 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_016.m4s.tmp' for writing=N/A dup=1 drop=0 speed=8.94x
2026-02-24T21:12:04.007004Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:04.430674Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame= 2604 fps=217 q=28.0 size=N/A time=00:01:48.52 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_017.m4s.tmp' for writing
2026-02-24T21:12:04.431213Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:04.875007Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame= 2743 fps=219 q=28.0 size=N/A time=00:01:54.32 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_018.m4s.tmp' for writing
2026-02-24T21:12:04.875688Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:04.876364Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [mp4 @ 0x7fd5644a1000] Packet duration: -16 / dts: 5499796 is out of range
2026-02-24T21:12:05.815049Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame= 2839 fps=218 q=28.0 size=N/A time=00:01:58.32 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_019.m4s.tmp' for writing
2026-02-24T21:12:05.815787Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:06.539869Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame= 2897 fps=215 q=28.0 size=N/A time=00:02:00.74 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_020.m4s.tmp' for writing=N/A dup=1 drop=0 speed=9.04x
2026-02-24T21:12:06.540622Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:06.540819Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [mp4 @ 0x7fd5644a1000] Packet duration: -16 / dts: 6184852 is out of range
2026-02-24T21:12:07.424053Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame= 3172 fps=219 q=28.0 size=N/A time=00:02:12.21 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_021.m4s.tmp' for writing=N/A dup=1 drop=0 speed=8.96x
2026-02-24T21:12:07.425020Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:08.269660Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame= 3288 fps=212 q=28.0 size=N/A time=00:02:17.05 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_022.m4s.tmp' for writing
2026-02-24T21:12:08.270495Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:08.292772Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [mp4 @ 0x7fd5644a1000] Packet duration: -16 / dts: 6689684 is out of range
2026-02-24T21:12:09.496331Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame= 3352 fps=209 q=28.0 size=N/A time=00:02:19.72 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_023.m4s.tmp' for writing=N/A dup=1 drop=0 speed=8.61x
2026-02-24T21:12:09.497543Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:09.879264Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame= 3682 fps=210 q=28.0 size=N/A time=00:02:33.48 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_024.m4s.tmp' for writing
2026-02-24T21:12:09.879693Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:10.124631Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'seg_025.m4s.tmp' for writing
2026-02-24T21:12:10.125008Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:10.743792Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame= 3848 fps=214 q=28.0 size=N/A time=00:02:40.41 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_026.m4s.tmp' for writing
2026-02-24T21:12:10.744402Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:11.180697Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame= 4004 fps=216 q=28.0 size=N/A time=00:02:46.91 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_027.m4s.tmp' for writing
2026-02-24T21:12:11.181076Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:11.625642Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame= 4172 fps=220 q=28.0 size=N/A time=00:02:53.92 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_028.m4s.tmp' for writing
2026-02-24T21:12:11.626138Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:11.906376Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame= 4319 fps=221 q=28.0 size=N/A time=00:03:00.05 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_029.m4s.tmp' for writing
2026-02-24T21:12:11.906635Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:11.907322Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [mp4 @ 0x7fd5644a1000] Packet duration: -16 / dts: 8679316 is out of range
2026-02-24T21:12:12.346420Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'seg_030.m4s.tmp' for writing
2026-02-24T21:12:12.346800Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:13.053758Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame= 4472 fps=224 q=28.0 size=N/A time=00:03:06.43 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_031.m4s.tmp' for writing=N/A dup=1 drop=0 speed= 9.4x
2026-02-24T21:12:13.054787Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:13.412076Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame= 4793 fps=228 q=28.0 size=N/A time=00:03:19.82 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_032.m4s.tmp' for writing
2026-02-24T21:12:13.412646Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:13.883534Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame= 4940 fps=230 q=28.0 size=N/A time=00:03:25.95 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_033.m4s.tmp' for writing
2026-02-24T21:12:13.884254Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:14.366251Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame= 5065 fps=230 q=28.0 size=N/A time=00:03:31.16 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_034.m4s.tmp' for writing
2026-02-24T21:12:14.366938Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:15.153356Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame= 5168 fps=230 q=28.0 size=N/A time=00:03:35.46 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_035.m4s.tmp' for writing
2026-02-24T21:12:15.157368Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:16.390113Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame= 5235 fps=228 q=28.0 size=N/A time=00:03:38.25 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_036.m4s.tmp' for writing=N/A dup=1 drop=0 speed=9.46x
2026-02-24T21:12:16.394177Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:16.394442Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [mp4 @ 0x7fd5644a1000] Packet duration: -16 / dts: 10909588 is out of range
2026-02-24T21:12:16.734431Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'seg_037.m4s.tmp' for writing
2026-02-24T21:12:16.734993Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:16.735114Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [mp4 @ 0x7fd5644a1000] Packet duration: -16 / dts: 11085700 is out of range
2026-02-24T21:12:17.026262Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame= 5571 fps=227 q=28.0 size=N/A time=00:03:52.27 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_038.m4s.tmp' for writing
2026-02-24T21:12:17.027399Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:18.448161Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame= 5676 fps=227 q=28.0 size=N/A time=00:03:56.65 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_039.m4s.tmp' for writing=N/A dup=1 drop=0 speed=9.34x
2026-02-24T21:12:18.451228Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:19.099598Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame= 5898 fps=223 q=28.0 size=N/A time=00:04:05.91 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_040.m4s.tmp' for writing
2026-02-24T21:12:19.100605Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:19.840938Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame= 6026 fps=223 q=28.0 size=N/A time=00:04:11.25 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_041.m4s.tmp' for writing
2026-02-24T21:12:19.842242Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:20.690007Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame= 6176 fps=225 q=28.0 size=N/A time=00:04:17.50 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_042.m4s.tmp' for writing=N/A dup=1 drop=0 speed=9.33x
2026-02-24T21:12:20.691005Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:20.703505Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [mp4 @ 0x7fd5644a1000] Packet duration: -16 / dts: 12635012 is out of range
2026-02-24T21:12:21.521890Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame= 6339 fps=222 q=28.0 size=N/A time=00:04:24.30 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_043.m4s.tmp' for writing=N/A dup=1 drop=0 speed=9.23x
2026-02-24T21:12:21.522826Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:21.523080Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [mp4 @ 0x7fd5644a1000] Packet duration: -16 / dts: 12884884 is out of range
2026-02-24T21:12:22.317459Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame= 6477 fps=220 q=28.0 size=N/A time=00:04:30.06 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_044.m4s.tmp' for writing
2026-02-24T21:12:22.318351Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:22.819445Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame= 6537 fps=218 q=28.0 size=N/A time=00:04:32.56 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_045.m4s.tmp' for writing
2026-02-24T21:12:22.820503Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:23.367619Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame= 6648 fps=218 q=28.0 size=N/A time=00:04:37.19 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_046.m4s.tmp' for writing=N/A dup=1 drop=0 speed=9.11x
2026-02-24T21:12:23.368438Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:23.971453Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame= 6890 fps=219 q=28.0 size=N/A time=00:04:47.28 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_047.m4s.tmp' for writing
2026-02-24T21:12:23.972510Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:24.521843Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame= 7033 fps=220 q=28.0 size=N/A time=00:04:53.25 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_048.m4s.tmp' for writing
2026-02-24T21:12:24.522567Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:24.533116Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [mp4 @ 0x7fd5644a1000] Packet duration: -16 / dts: 14129044 is out of range
2026-02-24T21:12:25.260740Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame= 7149 fps=220 q=28.0 size=N/A time=00:04:58.08 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_049.m4s.tmp' for writing
2026-02-24T21:12:25.261762Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:25.262085Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [mp4 @ 0x7fd5644a1000] Packet duration: -16 / dts: 14536612 is out of range
2026-02-24T21:12:25.587442Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame= 7283 fps=221 q=25.0 size=N/A time=00:05:03.67 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_050.m4s.tmp' for writing
2026-02-24T21:12:25.588251Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:25.588483Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [mp4 @ 0x7fd5644a1000] Packet duration: -16 / dts: 14733220 is out of range
2026-02-24T21:12:26.388227Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame= 7435 fps=222 q=28.0 size=N/A time=00:05:10.01 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_051.m4s.tmp' for writing=N/A dup=1 drop=0 speed=9.24x
2026-02-24T21:12:26.389563Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:26.947141Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame= 7614 fps=221 q=28.0 size=N/A time=00:05:17.48 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_052.m4s.tmp' for writing
2026-02-24T21:12:26.947971Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:26.948321Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [mp4 @ 0x7fd5644a1000] Packet duration: -16 / dts: 15265684 is out of range
2026-02-24T21:12:28.069286Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame= 7677 fps=219 q=28.0 size=N/A time=00:05:20.11 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_053.m4s.tmp' for writing=N/A dup=1 drop=0 speed=9.12x
2026-02-24T21:12:28.072241Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:28.491596Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame= 7881 fps=219 q=28.0 size=N/A time=00:05:28.61 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_054.m4s.tmp' for writing
2026-02-24T21:12:28.492206Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:29.374914Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame= 8020 fps=220 q=28.0 size=N/A time=00:05:34.41 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_055.m4s.tmp' for writing=N/A dup=1 drop=0 speed=9.18x
2026-02-24T21:12:29.375968Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:29.376364Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [mp4 @ 0x7fd5644a1000] Packet duration: -16 / dts: 16318340 is out of range
2026-02-24T21:12:29.950240Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame= 8259 fps=220 q=28.0 size=N/A time=00:05:44.38 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_056.m4s.tmp' for writing
2026-02-24T21:12:29.951291Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:29.951444Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [mp4 @ 0x7fd5644a1000] Packet duration: -16 / dts: 16563076 is out of range
2026-02-24T21:12:30.474596Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame= 8352 fps=220 q=28.0 size=N/A time=00:05:48.26 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_057.m4s.tmp' for writing
2026-02-24T21:12:30.476209Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:30.959591Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame= 8490 fps=220 q=25.0 size=N/A time=00:05:54.02 bitrate=N/A du[hls @ 0x561ef0af8d00] Opening 'seg_058.m4s.tmp' for writing
2026-02-24T21:12:30.960465Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:31.190898Z  INFO ferrite_stream::hls: Killing idle FFmpeg for session f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a (no segments requested for 30s)
2026-02-24T21:12:31.440413Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'seg_059.m4s.tmp' for writing
2026-02-24T21:12:31.441242Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:31.526760Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'seg_060.m4s.tmp' for writing
2026-02-24T21:12:31.528047Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [hls @ 0x561ef0af8d00] Opening 'playlist.m3u8.tmp' for writing
2026-02-24T21:12:31.529802Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [out#0/hls @ 0x561ef0b0e100] video:93878KiB audio:8600KiB subtitle:0KiB other streams:0KiB global headers:0KiB muxing overhead: unknown
2026-02-24T21:12:31.529825Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: frame= 8716 fps=222 q=-1.0 Lsize=N/A time=00:06:03.44 bitrate=N/A dup=1 drop=0 speed=9.28x
2026-02-24T21:12:31.531764Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [libx264 @ 0x561ef0abdd80] frame I:139   Avg QP:18.09  size: 64638
2026-02-24T21:12:31.531782Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [libx264 @ 0x561ef0abdd80] frame P:2654  Avg QP:21.11  size: 18847
2026-02-24T21:12:31.531793Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [libx264 @ 0x561ef0abdd80] frame B:5923  Avg QP:21.68  size:  6268
2026-02-24T21:12:31.531813Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [libx264 @ 0x561ef0abdd80] consecutive B-frames:  7.7%  1.9%  9.3% 81.1%
2026-02-24T21:12:31.531825Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [libx264 @ 0x561ef0abdd80] mb I  I16..4: 39.0% 52.8%  8.2%
2026-02-24T21:12:31.531837Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [libx264 @ 0x561ef0abdd80] mb P  I16..4: 19.2% 20.4%  0.4%  P16..4: 18.1%  5.3%  1.4%  0.0%  0.0%    skip:35.3%
2026-02-24T21:12:31.531849Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [libx264 @ 0x561ef0abdd80] mb B  I16..4:  2.8%  2.9%  0.0%  B16..8: 12.9%  2.1%  0.1%  direct: 8.5%  skip:70.7%  L0:46.2% L1:49.3% BI: 4.5%
2026-02-24T21:12:31.531862Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [libx264 @ 0x561ef0abdd80] 8x8 transform intra:51.1% inter:43.5%
2026-02-24T21:12:31.531873Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [libx264 @ 0x561ef0abdd80] coded y,uvDC,uvAC intra: 30.5% 48.0% 7.7% inter: 1.6% 13.2% 0.0%
2026-02-24T21:12:31.531895Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [libx264 @ 0x561ef0abdd80] i16 v,h,dc,p: 50% 22% 16% 12%
2026-02-24T21:12:31.531907Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [libx264 @ 0x561ef0abdd80] i8 v,h,dc,ddl,ddr,vr,hd,vl,hu: 28% 19% 24%  4%  5%  6%  5%  5%  4%
2026-02-24T21:12:31.531918Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [libx264 @ 0x561ef0abdd80] i4 v,h,dc,ddl,ddr,vr,hd,vl,hu: 26% 19% 11%  6%  9%  8%  8%  7%  6%
2026-02-24T21:12:31.531930Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [libx264 @ 0x561ef0abdd80] i8c dc,h,v,p: 52% 20% 25%  3%
2026-02-24T21:12:31.531941Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [libx264 @ 0x561ef0abdd80] Weighted P-Frames: Y:0.4% UV:0.3%
2026-02-24T21:12:31.531954Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [libx264 @ 0x561ef0abdd80] kb/s:2115.49
2026-02-24T21:12:31.620862Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: [aac @ 0x561ef0b5e2c0] Qavg: 965.438
2026-02-24T21:12:31.635299Z  WARN ferrite_stream::hls: ffmpeg HLS [f2fa5c07-d0bf-4cfa-b774-b05f51b0c11a]: Exiting normally, received signal 15.
