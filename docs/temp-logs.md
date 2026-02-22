c0580] using SAR=1280/1281
2026-02-22T05:33:57.405843Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]: [libx264 @ 0x55bcdb0c0580] using cpu capabilities: MMX2 SSE2Fast SSSE3 SSE4.2 AVX FMA3 BMI2 AVX2
2026-02-22T05:33:57.410112Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]: [libx264 @ 0x55bcdb0c0580] profile High, level 4.1, 4:2:0, 8-bit
2026-02-22T05:33:57.410157Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]: [libx264 @ 0x55bcdb0c0580] 264 - core 164 - H.264/MPEG-4 AVC codec - Copyleft 2003-2024 - http://www.videolan.org/x264.html - options: cabac=1 ref=1 deblock=1:0:0 analyse=0x3:0x113 me=hex subme=2 psy=1 psy_rd=1.00:0.00 mixed_ref=0 me_range=16 chroma_me=1 trellis=0 8x8dct=1 cqm=0 deadzone=21,11 fast_pskip=1 chroma_qp_offset=0 threads=15 lookahead_threads=3 sliced_threads=0 nr=0 decimate=1 interlaced=0 bluray_compat=0 constrained_intra=0 bframes=3 b_pyramid=2 b_adapt=1 b_bias=0 direct=1 weightb=1 open_gop=0 weightp=1 keyint=180 keyint_min=91 scenecut=40 intra_refresh=0 rc_lookahead=10 rc=crf mbtree=1 crf=23.0 qcomp=0.60 qpmin=0 qpmax=69 qpstep=4 vbv_maxrate=2100 vbv_bufsize=2800 crf_max=0.0 nal_hrd=none filler=0 ip_ratio=1.40 aq=1:1.00
2026-02-22T05:33:57.410371Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]: [hls @ 0x55bcdb0e73c0] Opening 'init.mp4' for writing
2026-02-22T05:33:57.410483Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]: Output #0, hls, to 'playlist.m3u8':
2026-02-22T05:33:57.410487Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]:   Metadata:
2026-02-22T05:33:57.410495Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]:     encoder         : Lavf61.7.100
2026-02-22T05:33:57.410539Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]:   Stream #0:0: Video: h264, yuv420p(tv, bt709, progressive), 854x480 [SAR 1280:1281 DAR 16:9], q=2-31, 1400 kb/s, 30 fps, 15360 tbn (default)
2026-02-22T05:33:57.410549Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]:       Metadata:
2026-02-22T05:33:57.410558Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]:         DURATION        : 00:58:00.053000000
2026-02-22T05:33:57.410565Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]:         encoder         : Lavc61.19.101 libx264
2026-02-22T05:33:57.410571Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]:       Side data:
2026-02-22T05:33:57.410577Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]:         cpb: bitrate max/min/avg: 2100000/0/1400000 buffer size: 2800000 vbv_delay: N/A
2026-02-22T05:33:57.410588Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]:   Stream #0:1(eng): Audio: aac (LC), 48000 Hz, stereo, fltp, 128 kb/s (default)
2026-02-22T05:33:57.410594Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]:       Metadata:
2026-02-22T05:33:57.410599Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]:         DURATION        : 00:58:00.000000000
2026-02-22T05:33:57.410605Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]:         encoder         : Lavc61.19.101 aac
2026-02-22T05:33:57.879706Z  WARN ferrite_stream::hls: ffmpeg HLS [af36ea4d-f4de-406e-bc47-a1437554f199]: frame=  136 fps=0.0 [hls @ 0x55a93670e3c0] Opening 'seg_000.m4s.tmp' for writingeed=8.93x
2026-02-22T05:33:57.881260Z  WARN ferrite_stream::hls: ffmpeg HLS [af36ea4d-f4de-406e-bc47-a1437554f199]: [hls @ 0x55a93670e3c0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:33:57.919947Z  INFO http_request{method=GET uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/master.m3u8?token=[REDACTED]&playback_session_id=1c801cec%2D91fa%2D430e%2Da022%2D96074d9cf284}: ferrite_stream::hls: HLS session af36ea4d-f4de-406e-bc47-a1437554f199 ready (first segment generated in 0.7s)
2026-02-22T05:33:57.972007Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]: frame=  109 fps=0.0 [hls @ 0x55bcdb0e73c0] Opening 'seg_000.m4s.tmp' for writingeed=7.13x
2026-02-22T05:33:57.973627Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]: [hls @ 0x55bcdb0e73c0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:33:58.021950Z  INFO http_request{method=GET uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/master.m3u8?token=[REDACTED]&playback_session_id=1c801cec%2D91fa%2D430e%2Da022%2D96074d9cf284}: ferrite_stream::hls: HLS session c31307b4-a071-4f42-aea5-8e9b56c5d033 ready (first segment generated in 0.8s)
2026-02-22T05:33:58.250522Z  WARN ferrite_stream::hls: ffmpeg HLS [cbfe9ceb-300b-48ed-b197-b02653f4d19e]: frame=   59 fps=0.0 [hls @ 0x55aa06f933c0] Opening 'seg_000.m4s.tmp' for writingeed= 3.8x
2026-02-22T05:33:58.253116Z  WARN ferrite_stream::hls: ffmpeg HLS [cbfe9ceb-300b-48ed-b197-b02653f4d19e]: [hls @ 0x55aa06f933c0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:33:58.277636Z  INFO http_request{method=GET uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/master.m3u8?token=[REDACTED]&playback_session_id=1c801cec%2D91fa%2D430e%2Da022%2D96074d9cf284}: ferrite_stream::hls: HLS session cbfe9ceb-300b-48ed-b197-b02653f4d19e ready (first segment generated in 1.0s)
2026-02-22T05:33:58.380592Z  WARN ferrite_stream::hls: ffmpeg HLS [af36ea4d-f4de-406e-bc47-a1437554f199]: frame=  381 fps=381 [hls @ 0x55a93670e3c0] Opening 'seg_001.m4s.tmp' for writingeed=12.6x
2026-02-22T05:33:58.381149Z  WARN ferrite_stream::hls: ffmpeg HLS [af36ea4d-f4de-406e-bc47-a1437554f199]: [hls @ 0x55a93670e3c0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:33:58.579483Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]: frame=  291 fps=291 [hls @ 0x55bcdb0e73c0] Opening 'seg_001.m4s.tmp' for writingeed=9.63x
2026-02-22T05:33:58.579668Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]: [hls @ 0x55bcdb0e73c0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:33:58.957683Z  WARN ferrite_stream::hls: ffmpeg HLS [7cddee04-df5a-4b9e-9c48-baf92bfc096a]: frame=    0 fps=0.0 [hls @ 0x55fd60cb2340] Opening 'seg_000.m4s.tmp' for writing=N/A dup=2 drop=0 speed=3.33x
2026-02-22T05:33:58.960568Z  WARN ferrite_stream::hls: ffmpeg HLS [7cddee04-df5a-4b9e-9c48-baf92bfc096a]: [hls @ 0x55fd60cb2340] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:33:58.984834Z  INFO http_request{method=GET uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/master.m3u8?token=[REDACTED]&playback_session_id=1c801cec%2D91fa%2D430e%2Da022%2D96074d9cf284}: ferrite_stream::hls: HLS session 7cddee04-df5a-4b9e-9c48-baf92bfc096a ready (first segment generated in 1.7s)
2026-02-22T05:33:58.984940Z  INFO http_request{method=GET uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/master.m3u8?token=[REDACTED]&playback_session_id=1c801cec%2D91fa%2D430e%2Da022%2D96074d9cf284}: ferrite_api::handlers::stream: HLS master playlist for ca663e67-38d2-4f47-9008-ed2de1f68a50: variants=4 reused=false mode=fast seek_source=none db=1ms seek=0ms session=1733ms total=1734ms
2026-02-22T05:33:59.108836Z  WARN ferrite_stream::hls: ffmpeg HLS [cbfe9ceb-300b-48ed-b197-b02653f4d19e]: frame=  188 fps=188 [hls @ 0x55aa06f933c0] Opening 'seg_001.m4s.tmp' for writing=N/A dup=4 drop=0 speed=7.33x
2026-02-22T05:33:59.109625Z  WARN ferrite_stream::hls: ffmpeg HLS [cbfe9ceb-300b-48ed-b197-b02653f4d19e]: [hls @ 0x55aa06f933c0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:33:59.128450Z  WARN ferrite_stream::hls: ffmpeg HLS [af36ea4d-f4de-406e-bc47-a1437554f199]: frame=  571 fps=381 [hls @ 0x55a93670e3c0] Opening 'seg_002.m4s.tmp' for writingeed=12.6x
2026-02-22T05:33:59.128750Z  WARN ferrite_stream::hls: ffmpeg HLS [af36ea4d-f4de-406e-bc47-a1437554f199]: [hls @ 0x55a93670e3c0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:33:59.265664Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]: frame=  495 fps=330 [hls @ 0x55bcdb0e73c0] Opening 'seg_002.m4s.tmp' for writingeed=  11x
2026-02-22T05:33:59.266487Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]: [hls @ 0x55bcdb0e73c0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:33:59.580737Z  WARN ferrite_stream::hls: ffmpeg HLS [af36ea4d-f4de-406e-bc47-a1437554f199]: frame=  744 fps=372 [hls @ 0x55a93670e3c0] Opening 'seg_003.m4s.tmp' for writingeed=12.4x
2026-02-22T05:33:59.581520Z  WARN ferrite_stream::hls: ffmpeg HLS [af36ea4d-f4de-406e-bc47-a1437554f199]: [hls @ 0x55a93670e3c0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:33:59.728767Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]: frame=  710 fps=355 [hls @ 0x55bcdb0e73c0] Opening 'seg_003.m4s.tmp' for writingeed=11.8x
2026-02-22T05:33:59.729194Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]: [hls @ 0x55bcdb0e73c0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:33:59.953346Z  WARN ferrite_stream::hls: ffmpeg HLS [cbfe9ceb-300b-48ed-b197-b02653f4d19e]: frame=  468 fps=234 [hls @ 0x55aa06f933c0] Opening 'seg_002.m4s.tmp' for writing=N/A dup=8 drop=0 speed=8.72x
2026-02-22T05:33:59.953902Z  WARN ferrite_stream::hls: ffmpeg HLS [cbfe9ceb-300b-48ed-b197-b02653f4d19e]: [hls @ 0x55aa06f933c0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:33:59.991649Z  WARN ferrite_stream::hls: ffmpeg HLS [af36ea4d-f4de-406e-bc47-a1437554f199]: frame=  891 fps=356 [hls @ 0x55a93670e3c0] Opening 'seg_004.m4s.tmp' for writingpeed=11.9x
2026-02-22T05:33:59.992013Z  WARN ferrite_stream::hls: ffmpeg HLS [af36ea4d-f4de-406e-bc47-a1437554f199]: [hls @ 0x55a93670e3c0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:34:00.127765Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]: frame=  838 fps=335 [hls @ 0x55bcdb0e73c0] Opening 'seg_004.m4s.tmp' for writingpeed=11.1x
2026-02-22T05:34:00.127995Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]: [hls @ 0x55bcdb0e73c0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:34:00.185340Z  WARN ferrite_stream::hls: ffmpeg HLS [7cddee04-df5a-4b9e-9c48-baf92bfc096a]: frame=  236 fps=118 [hls @ 0x55fd60cb2340] Opening 'seg_001.m4s.tmp' for writing=N/A dup=4 drop=0 speed=4.75x
2026-02-22T05:34:00.186277Z  WARN ferrite_stream::hls: ffmpeg HLS [7cddee04-df5a-4b9e-9c48-baf92bfc096a]: [hls @ 0x55fd60cb2340] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:34:00.435245Z  WARN ferrite_stream::hls: ffmpeg HLS [cbfe9ceb-300b-48ed-b197-b02653f4d19e]: frame=  775 fps=258 [hls @ 0x55aa06f933c0] Opening 'seg_003.m4s.tmp' for writingpeed=8.59x
2026-02-22T05:34:00.436382Z  WARN ferrite_stream::hls: ffmpeg HLS [cbfe9ceb-300b-48ed-b197-b02653f4d19e]: [hls @ 0x55aa06f933c0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:34:00.832492Z  WARN ferrite_stream::hls: ffmpeg HLS [cbfe9ceb-300b-48ed-b197-b02653f4d19e]: frame=  940 fps=269 [hls @ 0x55aa06f933c0] Opening 'seg_004.m4s.tmp' for writingpeed=8.93x
2026-02-22T05:34:00.832941Z  WARN ferrite_stream::hls: ffmpeg HLS [cbfe9ceb-300b-48ed-b197-b02653f4d19e]: [hls @ 0x55aa06f933c0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:34:01.077937Z  WARN ferrite_stream::hls: ffmpeg HLS [af36ea4d-f4de-406e-bc47-a1437554f199]: frame= 1047 fps=349 [hls @ 0x55a93670e3c0] Opening 'seg_005.m4s.tmp' for writing=N/A dup=12 drop=0 speed=10.9x
2026-02-22T05:34:01.078618Z  WARN ferrite_stream::hls: ffmpeg HLS [af36ea4d-f4de-406e-bc47-a1437554f199]: [hls @ 0x55a93670e3c0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:34:01.224095Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]: frame= 1004 fps=335 [hls @ 0x55bcdb0e73c0] Opening 'seg_005.m4s.tmp' for writing=N/A dup=12 drop=0 speed=10.6x
2026-02-22T05:34:01.224967Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]: [hls @ 0x55bcdb0e73c0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:34:01.292457Z  WARN ferrite_stream::hls: ffmpeg HLS [7cddee04-df5a-4b9e-9c48-baf92bfc096a]: frame=  439 fps=146 [hls @ 0x55fd60cb2340] Opening 'seg_002.m4s.tmp' for writing=N/A dup=9 drop=0 speed=5.87x
2026-02-22T05:34:01.292494Z  WARN ferrite_stream::hls: ffmpeg HLS [7cddee04-df5a-4b9e-9c48-baf92bfc096a]: [hls @ 0x55fd60cb2340] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:34:01.625207Z  WARN ferrite_stream::hls: ffmpeg HLS [af36ea4d-f4de-406e-bc47-a1437554f199]: frame= 1291 fps=323 [hls @ 0x55a93670e3c0] Opening 'seg_006.m4s.tmp' for writingpeed=10.7x
2026-02-22T05:34:01.625582Z  WARN ferrite_stream::hls: ffmpeg HLS [af36ea4d-f4de-406e-bc47-a1437554f199]: [hls @ 0x55a93670e3c0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:34:01.752856Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]: frame= 1242 fps=310 [hls @ 0x55bcdb0e73c0] Opening 'seg_006.m4s.tmp' for writingpeed=10.3x
2026-02-22T05:34:01.753385Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]: [hls @ 0x55bcdb0e73c0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:34:01.869392Z  WARN ferrite_stream::hls: ffmpeg HLS [7cddee04-df5a-4b9e-9c48-baf92bfc096a]: frame=  805 fps=179 [hls @ 0x55fd60cb2340] Opening 'seg_003.m4s.tmp' for writingpeed=5.95x
2026-02-22T05:34:01.872277Z  WARN ferrite_stream::hls: ffmpeg HLS [7cddee04-df5a-4b9e-9c48-baf92bfc096a]: [hls @ 0x55fd60cb2340] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:34:02.055134Z  WARN ferrite_stream::hls: ffmpeg HLS [cbfe9ceb-300b-48ed-b197-b02653f4d19e]: frame= 1063 fps=266 [hls @ 0x55aa06f933c0] Opening 'seg_005.m4s.tmp' for writing=N/A dup=13 drop=0 speed=8.58x
2026-02-22T05:34:02.057624Z  WARN ferrite_stream::hls: ffmpeg HLS [cbfe9ceb-300b-48ed-b197-b02653f4d19e]: [hls @ 0x55aa06f933c0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:34:02.137518Z  WARN ferrite_stream::hls: ffmpeg HLS [af36ea4d-f4de-406e-bc47-a1437554f199]: frame= 1453 fps=323 [hls @ 0x55a93670e3c0] Opening 'seg_007.m4s.tmp' for writingpeed=10.7x
2026-02-22T05:34:02.137776Z  WARN ferrite_stream::hls: ffmpeg HLS [af36ea4d-f4de-406e-bc47-a1437554f199]: [hls @ 0x55a93670e3c0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:34:02.287133Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]: frame= 1402 fps=311 [hls @ 0x55bcdb0e73c0] Opening 'seg_007.m4s.tmp' for writing=N/A dup=16 drop=0 speed=10.4x
2026-02-22T05:34:02.287714Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]: [hls @ 0x55bcdb0e73c0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:34:02.426669Z  WARN ferrite_stream::hls: ffmpeg HLS [7cddee04-df5a-4b9e-9c48-baf92bfc096a]: frame=  921 fps=184 [hls @ 0x55fd60cb2340] Opening 'seg_004.m4s.tmp' for writingpeed=6.13x
2026-02-22T05:34:02.427717Z  WARN ferrite_stream::hls: ffmpeg HLS [7cddee04-df5a-4b9e-9c48-baf92bfc096a]: [hls @ 0x55fd60cb2340] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:34:02.517936Z  WARN ferrite_stream::hls: ffmpeg HLS [af36ea4d-f4de-406e-bc47-a1437554f199]: frame= 1634 fps=327 [hls @ 0x55a93670e3c0] Opening 'seg_008.m4s.tmp' for writingpeed=10.9x
2026-02-22T05:34:02.518347Z  WARN ferrite_stream::hls: ffmpeg HLS [af36ea4d-f4de-406e-bc47-a1437554f199]: [hls @ 0x55a93670e3c0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:34:02.651779Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]: [hls @ 0x55bcdb0e73c0] Opening 'seg_008.m4s.tmp' for writing
2026-02-22T05:34:02.652392Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]: [hls @ 0x55bcdb0e73c0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:34:02.691237Z  WARN ferrite_stream::hls: ffmpeg HLS [cbfe9ceb-300b-48ed-b197-b02653f4d19e]: frame= 1280 fps=256 [hls @ 0x55aa06f933c0] Opening 'seg_006.m4s.tmp' for writingpeed=8.52x
2026-02-22T05:34:02.691960Z  WARN ferrite_stream::hls: ffmpeg HLS [cbfe9ceb-300b-48ed-b197-b02653f4d19e]: [hls @ 0x55aa06f933c0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:34:04.367451Z  WARN ferrite_stream::hls: ffmpeg HLS [af36ea4d-f4de-406e-bc47-a1437554f199]: frame= 1799 fps=327 [hls @ 0x55a93670e3c0] Opening 'seg_009.m4s.tmp' for writing=N/A dup=18 drop=0 speed=8.97x
2026-02-22T05:34:04.367614Z  WARN ferrite_stream::hls: ffmpeg HLS [af36ea4d-f4de-406e-bc47-a1437554f199]: [hls @ 0x55a93670e3c0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:34:04.689212Z  INFO http_request{method=POST uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/seek?start=1681.982&playback_session_id=1c801cec-91fa-430e-a022-96074d9cf284}: ferrite_api::handlers::stream: Transcode permit acquired: op=hls-seek media_id=ca663e67-38d2-4f47-9008-ed2de1f68a50 wait_ms=0.0
2026-02-22T05:34:05.184151Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]: frame= 1743 fps=317 [hls @ 0x55bcdb0e73c0] Opening 'seg_009.m4s.tmp' for writing=N/A dup=18 drop=0 speed=8.75x
2026-02-22T05:34:05.184178Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]: [hls @ 0x55bcdb0e73c0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:34:05.219643Z  WARN ferrite_stream::hls: ffmpeg HLS [cbfe9ceb-300b-48ed-b197-b02653f4d19e]: frame= 1423 fps=259 [hls @ 0x55aa06f933c0] Opening 'seg_007.m4s.tmp' for writing=N/A dup=16 drop=0 speed=7.19x
2026-02-22T05:34:05.220041Z  WARN ferrite_stream::hls: ffmpeg HLS [cbfe9ceb-300b-48ed-b197-b02653f4d19e]: [hls @ 0x55aa06f933c0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:34:05.376703Z  WARN ferrite_stream::hls: ffmpeg HLS [af36ea4d-f4de-406e-bc47-a1437554f199]: [hls @ 0x55a93670e3c0] Opening 'seg_010.m4s.tmp' for writing
2026-02-22T05:34:05.377115Z  WARN ferrite_stream::hls: ffmpeg HLS [af36ea4d-f4de-406e-bc47-a1437554f199]: [hls @ 0x55a93670e3c0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:34:05.377273Z  WARN ferrite_stream::hls: ffmpeg HLS [af36ea4d-f4de-406e-bc47-a1437554f199]: [out#0/hls @ 0x55a93670f540] video:4543KiB audio:781KiB subtitle:0KiB other streams:0KiB global headers:0KiB muxing overhead: unknown
2026-02-22T05:34:05.377283Z  WARN ferrite_stream::hls: ffmpeg HLS [af36ea4d-f4de-406e-bc47-a1437554f199]: frame= 1980 fps=244 q=-1.0 Lsize=N/A time=00:01:05.88 bitrate=N/A dup=19 drop=0 speed=8.13x
2026-02-22T05:34:05.384752Z  WARN ferrite_stream::hls: ffmpeg HLS [af36ea4d-f4de-406e-bc47-a1437554f199]: [libx264 @ 0x55a9366e7580] frame I:22    Avg QP:20.57  size: 27533
2026-02-22T05:34:05.384770Z  WARN ferrite_stream::hls: ffmpeg HLS [af36ea4d-f4de-406e-bc47-a1437554f199]: [libx264 @ 0x55a9366e7580] frame P:571   Avg QP:24.70  size:  5247
2026-02-22T05:34:05.384774Z  WARN ferrite_stream::hls: ffmpeg HLS [af36ea4d-f4de-406e-bc47-a1437554f199]: [libx264 @ 0x55a9366e7580] frame B:1387  Avg QP:27.95  size:   757
2026-02-22T05:34:05.384777Z  WARN ferrite_stream::hls: ffmpeg HLS [af36ea4d-f4de-406e-bc47-a1437554f199]: [libx264 @ 0x55a9366e7580] consecutive B-frames:  1.9%  9.9% 12.9% 75.4%
2026-02-22T05:34:05.384781Z  WARN ferrite_stream::hls: ffmpeg HLS [af36ea4d-f4de-406e-bc47-a1437554f199]: [libx264 @ 0x55a9366e7580] mb I  I16..4:  8.7% 31.9% 59.4%
2026-02-22T05:34:05.384784Z  WARN ferrite_stream::hls: ffmpeg HLS [af36ea4d-f4de-406e-bc47-a1437554f199]: [libx264 @ 0x55a9366e7580] mb P  I16..4:  2.7%  7.8%  2.1%  P16..4: 28.5% 15.0%  9.5%  0.0%  0.0%    skip:34.4%
2026-02-22T05:34:05.384797Z  WARN ferrite_stream::hls: ffmpeg HLS [af36ea4d-f4de-406e-bc47-a1437554f199]: [libx264 @ 0x55a9366e7580] mb B  I16..4:  0.2%  0.5%  0.0%  B16..8:  9.3%  3.8%  0.7%  direct: 2.3%  skip:83.2%  L0:32.3% L1:38.0% BI:29.7%
2026-02-22T05:34:05.384801Z  WARN ferrite_stream::hls: ffmpeg HLS [af36ea4d-f4de-406e-bc47-a1437554f199]: [libx264 @ 0x55a9366e7580] 8x8 transform intra:56.3% inter:34.1%
2026-02-22T05:34:05.384804Z  WARN ferrite_stream::hls: ffmpeg HLS [af36ea4d-f4de-406e-bc47-a1437554f199]: [libx264 @ 0x55a9366e7580] coded y,uvDC,uvAC intra: 63.7% 51.9% 18.6% inter: 8.3% 3.9% 0.3%
2026-02-22T05:34:05.384808Z  WARN ferrite_stream::hls: ffmpeg HLS [af36ea4d-f4de-406e-bc47-a1437554f199]: [libx264 @ 0x55a9366e7580] i16 v,h,dc,p: 40% 26% 20% 14%
2026-02-22T05:34:05.384812Z  WARN ferrite_stream::hls: ffmpeg HLS [af36ea4d-f4de-406e-bc47-a1437554f199]: [libx264 @ 0x55a9366e7580] i8 v,h,dc,ddl,ddr,vr,hd,vl,hu: 25% 21% 17%  6%  6%  6%  7%  6%  7%
2026-02-22T05:34:05.384816Z  WARN ferrite_stream::hls: ffmpeg HLS [af36ea4d-f4de-406e-bc47-a1437554f199]: [libx264 @ 0x55a9366e7580] i4 v,h,dc,ddl,ddr,vr,hd,vl,hu: 19% 20% 12%  8%  9%  7% 10%  7%  9%
2026-02-22T05:34:05.384819Z  WARN ferrite_stream::hls: ffmpeg HLS [af36ea4d-f4de-406e-bc47-a1437554f199]: [libx264 @ 0x55a9366e7580] i8c dc,h,v,p: 50% 22% 21%  7%
2026-02-22T05:34:05.384822Z  WARN ferrite_stream::hls: ffmpeg HLS [af36ea4d-f4de-406e-bc47-a1437554f199]: [libx264 @ 0x55a9366e7580] Weighted P-Frames: Y:0.2% UV:0.0%
2026-02-22T05:34:05.384825Z  WARN ferrite_stream::hls: ffmpeg HLS [af36ea4d-f4de-406e-bc47-a1437554f199]: [libx264 @ 0x55a9366e7580] kb/s:563.81
2026-02-22T05:34:05.419188Z  WARN ferrite_stream::hls: ffmpeg HLS [cbfe9ceb-300b-48ed-b197-b02653f4d19e]: [hls @ 0x55aa06f933c0] Opening 'seg_008.m4s.tmp' for writing
2026-02-22T05:34:05.419590Z  WARN ferrite_stream::hls: ffmpeg HLS [cbfe9ceb-300b-48ed-b197-b02653f4d19e]: [hls @ 0x55aa06f933c0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:34:05.421232Z  WARN ferrite_stream::hls: ffmpeg HLS [cbfe9ceb-300b-48ed-b197-b02653f4d19e]: [out#0/hls @ 0x55aa06f94540] video:10607KiB audio:848KiB subtitle:0KiB other streams:0KiB global headers:0KiB muxing overhead: unknown
2026-02-22T05:34:05.421241Z  WARN ferrite_stream::hls: ffmpeg HLS [cbfe9ceb-300b-48ed-b197-b02653f4d19e]: frame= 1615 fps=198 q=-1.0 Lsize=N/A time=00:00:53.76 bitrate=N/A dup=16 drop=0 speed= 6.6x
2026-02-22T05:34:05.426144Z  WARN ferrite_stream::hls: ffmpeg HLS [cbfe9ceb-300b-48ed-b197-b02653f4d19e]: [libx264 @ 0x55aa06f6c580] frame I:17    Avg QP:19.87  size: 68598
2026-02-22T05:34:05.426178Z  WARN ferrite_stream::hls: ffmpeg HLS [cbfe9ceb-300b-48ed-b197-b02653f4d19e]: [libx264 @ 0x55aa06f6c580] frame P:433   Avg QP:23.73  size: 14968
2026-02-22T05:34:05.426204Z  WARN ferrite_stream::hls: ffmpeg HLS [cbfe9ceb-300b-48ed-b197-b02653f4d19e]: [libx264 @ 0x55aa06f6c580] frame B:1165  Avg QP:26.53  size:  2758
2026-02-22T05:34:05.426233Z  WARN ferrite_stream::hls: ffmpeg HLS [cbfe9ceb-300b-48ed-b197-b02653f4d19e]: [libx264 @ 0x55aa06f6c580] consecutive B-frames:  1.6%  4.3%  6.9% 87.2%
2026-02-22T05:34:05.426258Z  WARN ferrite_stream::hls: ffmpeg HLS [cbfe9ceb-300b-48ed-b197-b02653f4d19e]: [libx264 @ 0x55aa06f6c580] mb I  I16..4: 15.5% 39.4% 45.1%
2026-02-22T05:34:05.426284Z  WARN ferrite_stream::hls: ffmpeg HLS [cbfe9ceb-300b-48ed-b197-b02653f4d19e]: [libx264 @ 0x55aa06f6c580] mb P  I16..4:  5.6% 12.4%  2.3%  P16..4: 23.8% 12.2%  6.6%  0.0%  0.0%    skip:37.1%
2026-02-22T05:34:05.426312Z  WARN ferrite_stream::hls: ffmpeg HLS [cbfe9ceb-300b-48ed-b197-b02653f4d19e]: [libx264 @ 0x55aa06f6c580] mb B  I16..4:  0.5%  1.0%  0.1%  B16..8: 10.0%  3.3%  0.5%  direct: 2.7%  skip:82.0%  L0:34.3% L1:44.3% BI:21.4%
2026-02-22T05:34:05.426336Z  WARN ferrite_stream::hls: ffmpeg HLS [cbfe9ceb-300b-48ed-b197-b02653f4d19e]: [libx264 @ 0x55aa06f6c580] 8x8 transform intra:58.3% inter:39.3%
2026-02-22T05:34:05.426360Z  WARN ferrite_stream::hls: ffmpeg HLS [cbfe9ceb-300b-48ed-b197-b02653f4d19e]: [libx264 @ 0x55aa06f6c580] coded y,uvDC,uvAC intra: 51.1% 42.3% 11.1% inter: 6.1% 4.0% 0.1%
2026-02-22T05:34:05.426384Z  WARN ferrite_stream::hls: ffmpeg HLS [cbfe9ceb-300b-48ed-b197-b02653f4d19e]: [libx264 @ 0x55aa06f6c580] i16 v,h,dc,p: 44% 25% 16% 15%
2026-02-22T05:34:05.426409Z  WARN ferrite_stream::hls: ffmpeg HLS [cbfe9ceb-300b-48ed-b197-b02653f4d19e]: [libx264 @ 0x55aa06f6c580] i8 v,h,dc,ddl,ddr,vr,hd,vl,hu: 21% 23% 18%  5%  7%  6%  7%  5%  7%
2026-02-22T05:34:05.426433Z  WARN ferrite_stream::hls: ffmpeg HLS [cbfe9ceb-300b-48ed-b197-b02653f4d19e]: [libx264 @ 0x55aa06f6c580] i4 v,h,dc,ddl,ddr,vr,hd,vl,hu: 18% 19% 11%  8% 10%  8% 10%  7%  9%
2026-02-22T05:34:05.426457Z  WARN ferrite_stream::hls: ffmpeg HLS [cbfe9ceb-300b-48ed-b197-b02653f4d19e]: [libx264 @ 0x55aa06f6c580] i8c dc,h,v,p: 53% 21% 20%  6%
2026-02-22T05:34:05.426481Z  WARN ferrite_stream::hls: ffmpeg HLS [cbfe9ceb-300b-48ed-b197-b02653f4d19e]: [libx264 @ 0x55aa06f6c580] Weighted P-Frames: Y:0.2% UV:0.0%
2026-02-22T05:34:05.429312Z  WARN ferrite_stream::hls: ffmpeg HLS [cbfe9ceb-300b-48ed-b197-b02653f4d19e]: [libx264 @ 0x55aa06f6c580] kb/s:1613.95
2026-02-22T05:34:05.430587Z  WARN ferrite_stream::hls: ffmpeg HLS [af36ea4d-f4de-406e-bc47-a1437554f199]: [aac @ 0x55a9373ed400] Qavg: 470.926
2026-02-22T05:34:05.505976Z  WARN ferrite_stream::hls: ffmpeg HLS [cbfe9ceb-300b-48ed-b197-b02653f4d19e]: [aac @ 0x55aa07c72400] Qavg: 517.410
2026-02-22T05:34:05.506979Z  WARN ferrite_stream::hls: ffmpeg HLS [af36ea4d-f4de-406e-bc47-a1437554f199]: Exiting normally, received signal 15.
2026-02-22T05:34:05.507853Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]: [hls @ 0x55bcdb0e73c0] Opening 'seg_010.m4s.tmp' for writing
2026-02-22T05:34:05.507863Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]: [hls @ 0x55bcdb0e73c0] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:34:05.507867Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]: [out#0/hls @ 0x55bcdb0e8540] video:6967KiB audio:1018KiB subtitle:0KiB other streams:0KiB global headers:0KiB muxing overhead: unknown
2026-02-22T05:34:05.507872Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]: frame= 1939 fps=235 q=-1.0 Lsize=N/A time=00:01:04.51 bitrate=N/A dup=19 drop=0 speed=7.83x
2026-02-22T05:34:05.508510Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]: [libx264 @ 0x55bcdb0c0580] frame I:21    Avg QP:20.21  size: 43005
2026-02-22T05:34:05.508547Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]: [libx264 @ 0x55bcdb0c0580] frame P:541   Avg QP:24.20  size:  8196
2026-02-22T05:34:05.508573Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]: [libx264 @ 0x55bcdb0c0580] frame B:1377  Avg QP:27.34  size:  1305
2026-02-22T05:34:05.508598Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]: [libx264 @ 0x55bcdb0c0580] consecutive B-frames:  1.4%  8.4% 10.2% 80.0%
2026-02-22T05:34:05.508624Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]: [libx264 @ 0x55bcdb0c0580] mb I  I16..4: 10.6% 33.3% 56.1%
2026-02-22T05:34:05.508651Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]: [libx264 @ 0x55bcdb0c0580] mb P  I16..4:  3.7%  9.1%  2.2%  P16..4: 26.8% 14.0%  8.4%  0.0%  0.0%    skip:35.7%
2026-02-22T05:34:05.508678Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]: [libx264 @ 0x55bcdb0c0580] mb B  I16..4:  0.3%  0.7%  0.0%  B16..8:  9.6%  3.5%  0.6%  direct: 2.4%  skip:82.9%  L0:33.1% L1:40.1% BI:26.8%
2026-02-22T05:34:05.515649Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]: [libx264 @ 0x55bcdb0c0580] 8x8 transform intra:56.8% inter:35.1%
2026-02-22T05:34:05.515675Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]: [libx264 @ 0x55bcdb0c0580] coded y,uvDC,uvAC intra: 58.4% 47.4% 15.2% inter: 7.4% 3.9% 0.2%
2026-02-22T05:34:05.515683Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]: [libx264 @ 0x55bcdb0c0580] i16 v,h,dc,p: 41% 27% 17% 14%
2026-02-22T05:34:05.515690Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]: [libx264 @ 0x55bcdb0c0580] i8 v,h,dc,ddl,ddr,vr,hd,vl,hu: 21% 24% 17%  6%  7%  6%  7%  6%  7%
2026-02-22T05:34:05.515697Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]: [libx264 @ 0x55bcdb0c0580] i4 v,h,dc,ddl,ddr,vr,hd,vl,hu: 16% 22% 11%  8%  9%  7% 10%  7%  9%
2026-02-22T05:34:05.515703Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]: [libx264 @ 0x55bcdb0c0580] i8c dc,h,v,p: 51% 23% 20%  6%
2026-02-22T05:34:05.515709Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]: [libx264 @ 0x55bcdb0c0580] Weighted P-Frames: Y:0.2% UV:0.0%
2026-02-22T05:34:05.515715Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]: [libx264 @ 0x55bcdb0c0580] kb/s:882.94
2026-02-22T05:34:05.529624Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]: [aac @ 0x55bcdbdc6400] Qavg: 527.154
2026-02-22T05:34:05.554625Z  WARN ferrite_stream::hls: ffmpeg HLS [cbfe9ceb-300b-48ed-b197-b02653f4d19e]: Exiting normally, received signal 15.
2026-02-22T05:34:05.609237Z  WARN ferrite_stream::hls: ffmpeg HLS [7cddee04-df5a-4b9e-9c48-baf92bfc096a]: frame= 1040 fps=189 [hls @ 0x55fd60cb2340] Opening 'seg_005.m4s.tmp' for writing=N/A dup=12 drop=0 speed= 5.3x
2026-02-22T05:34:05.616289Z  WARN ferrite_stream::hls: ffmpeg HLS [7cddee04-df5a-4b9e-9c48-baf92bfc096a]: [hls @ 0x55fd60cb2340] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:34:05.616381Z  WARN ferrite_stream::hls: ffmpeg HLS [7cddee04-df5a-4b9e-9c48-baf92bfc096a]: [out#0/hls @ 0x55fd60cb3480] video:14113KiB audio:962KiB subtitle:0KiB other streams:0KiB global headers:0KiB muxing overhead: unknown
2026-02-22T05:34:05.616390Z  WARN ferrite_stream::hls: ffmpeg HLS [7cddee04-df5a-4b9e-9c48-baf92bfc096a]: frame= 1223 fps=147 q=-1.0 Lsize=N/A time=00:00:40.67 bitrate=N/A dup=13 drop=0 speed=4.87x
2026-02-22T05:34:05.620704Z  WARN ferrite_stream::hls: ffmpeg HLS [7cddee04-df5a-4b9e-9c48-baf92bfc096a]: [libx264 @ 0x55fd60c8b4c0] frame I:12    Avg QP:19.56  size:119802
2026-02-22T05:34:05.620722Z  WARN ferrite_stream::hls: ffmpeg HLS [7cddee04-df5a-4b9e-9c48-baf92bfc096a]: [libx264 @ 0x55fd60c8b4c0] frame P:338   Avg QP:23.07  size: 26197
2026-02-22T05:34:05.620725Z  WARN ferrite_stream::hls: ffmpeg HLS [7cddee04-df5a-4b9e-9c48-baf92bfc096a]: [libx264 @ 0x55fd60c8b4c0] frame B:873   Avg QP:25.78  size:  4764
2026-02-22T05:34:05.620729Z  WARN ferrite_stream::hls: ffmpeg HLS [7cddee04-df5a-4b9e-9c48-baf92bfc096a]: [libx264 @ 0x55fd60c8b4c0] consecutive B-frames:  2.4%  4.6%  8.3% 84.7%
2026-02-22T05:34:05.620733Z  WARN ferrite_stream::hls: ffmpeg HLS [7cddee04-df5a-4b9e-9c48-baf92bfc096a]: [libx264 @ 0x55fd60c8b4c0] mb I  I16..4:  6.5% 60.5% 33.0%
2026-02-22T05:34:05.620737Z  WARN ferrite_stream::hls: ffmpeg HLS [7cddee04-df5a-4b9e-9c48-baf92bfc096a]: [libx264 @ 0x55fd60c8b4c0] mb P  I16..4:  3.2% 17.8%  1.5%  P16..4: 20.4%  9.9%  5.2%  0.0%  0.0%    skip:41.9%
2026-02-22T05:34:05.620750Z  WARN ferrite_stream::hls: ffmpeg HLS [7cddee04-df5a-4b9e-9c48-baf92bfc096a]: [libx264 @ 0x55fd60c8b4c0] mb B  I16..4:  0.3%  1.6%  0.0%  B16..8:  9.2%  2.4%  0.3%  direct: 2.3%  skip:83.7%  L0:36.4% L1:47.2% BI:16.4%
2026-02-22T05:34:05.620754Z  WARN ferrite_stream::hls: ffmpeg HLS [7cddee04-df5a-4b9e-9c48-baf92bfc096a]: [libx264 @ 0x55fd60c8b4c0] 8x8 transform intra:77.5% inter:53.9%
2026-02-22T05:34:05.620758Z  WARN ferrite_stream::hls: ffmpeg HLS [7cddee04-df5a-4b9e-9c48-baf92bfc096a]: [libx264 @ 0x55fd60c8b4c0] coded y,uvDC,uvAC intra: 47.0% 39.6% 7.3% inter: 4.6% 3.6% 0.0%
2026-02-22T05:34:05.620762Z  WARN ferrite_stream::hls: ffmpeg HLS [7cddee04-df5a-4b9e-9c48-baf92bfc096a]: [libx264 @ 0x55fd60c8b4c0] i16 v,h,dc,p: 46% 20% 13% 21%
2026-02-22T05:34:05.620769Z  WARN ferrite_stream::hls: ffmpeg HLS [7cddee04-df5a-4b9e-9c48-baf92bfc096a]: [libx264 @ 0x55fd60c8b4c0] i8 v,h,dc,ddl,ddr,vr,hd,vl,hu: 30% 24% 19%  3%  6%  5%  6%  3%  4%
2026-02-22T05:34:05.620777Z  WARN ferrite_stream::hls: ffmpeg HLS [7cddee04-df5a-4b9e-9c48-baf92bfc096a]: [libx264 @ 0x55fd60c8b4c0] i4 v,h,dc,ddl,ddr,vr,hd,vl,hu: 19% 18%  9%  8% 10%  8% 10%  8%  9%
2026-02-22T05:34:05.620784Z  WARN ferrite_stream::hls: ffmpeg HLS [7cddee04-df5a-4b9e-9c48-baf92bfc096a]: [libx264 @ 0x55fd60c8b4c0] i8c dc,h,v,p: 52% 21% 21%  6%
2026-02-22T05:34:05.620791Z  WARN ferrite_stream::hls: ffmpeg HLS [7cddee04-df5a-4b9e-9c48-baf92bfc096a]: [libx264 @ 0x55fd60c8b4c0] Weighted P-Frames: Y:0.0% UV:0.0%
2026-02-22T05:34:05.620799Z  WARN ferrite_stream::hls: ffmpeg HLS [7cddee04-df5a-4b9e-9c48-baf92bfc096a]: [libx264 @ 0x55fd60c8b4c0] kb/s:2835.91
2026-02-22T05:34:05.625589Z  WARN ferrite_stream::hls: ffmpeg HLS [c31307b4-a071-4f42-aea5-8e9b56c5d033]: Exiting normally, received signal 15.
2026-02-22T05:34:05.702041Z  WARN ferrite_stream::hls: ffmpeg HLS [7cddee04-df5a-4b9e-9c48-baf92bfc096a]: [aac @ 0x55fd61990940] Qavg: 759.475
2026-02-22T05:34:05.739268Z  WARN ferrite_stream::hls: ffmpeg HLS [7cddee04-df5a-4b9e-9c48-baf92bfc096a]: Exiting normally, received signal 15.
2026-02-22T05:34:06.699312Z  INFO http_request{method=POST uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/seek?start=1681.982&playback_session_id=1c801cec-91fa-430e-a022-96074d9cf284}: ferrite_stream::hls: Destroyed HLS session c31307b4-a071-4f42-aea5-8e9b56c5d033
2026-02-22T05:34:06.699440Z  INFO http_request{method=POST uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/seek?start=1681.982&playback_session_id=1c801cec-91fa-430e-a022-96074d9cf284}: ferrite_stream::hls: Destroyed HLS session 7cddee04-df5a-4b9e-9c48-baf92bfc096a
2026-02-22T05:34:06.700236Z  INFO http_request{method=POST uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/seek?start=1681.982&playback_session_id=1c801cec-91fa-430e-a022-96074d9cf284}: ferrite_stream::hls: Destroyed HLS session af36ea4d-f4de-406e-bc47-a1437554f199
2026-02-22T05:34:06.702484Z  INFO http_request{method=POST uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/seek?start=1681.982&playback_session_id=1c801cec-91fa-430e-a022-96074d9cf284}: ferrite_stream::hls: Destroyed HLS session cbfe9ceb-300b-48ed-b197-b02653f4d19e
2026-02-22T05:34:06.702513Z  INFO http_request{method=POST uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/seek?start=1681.982&playback_session_id=1c801cec-91fa-430e-a022-96074d9cf284}: ferrite_stream::hls: Creating single HLS session for seek: media ca663e67-38d2-4f47-9008-ed2de1f68a50 at 1673.0s variant=1080p
2026-02-22T05:34:06.702613Z  INFO http_request{method=POST uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/seek?start=1681.982&playback_session_id=1c801cec-91fa-430e-a022-96074d9cf284}: ferrite_stream::hls: Creating HLS session b1b19145-70cf-4d73-a3c9-31f6bc27810e for media ca663e67-38d2-4f47-9008-ed2de1f68a50 at 1673.0s variant=1080p (/home/badchiefy/files/tv/Asia (2024)/Season 1/Asia.S01E01.Beneath.the.Waves.1080p.WEBRip.10bit.EAC3.2.0.x265-iVy.mkv)
2026-02-22T05:34:06.702671Z  INFO http_request{method=POST uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/seek?start=1681.982&playback_session_id=1c801cec-91fa-430e-a022-96074d9cf284}: ferrite_stream::hls: 10-bit SDR detected (pix=yuv420p10le, transfer=Some("bt709")), applying bit-depth conversion only
2026-02-22T05:34:06.702706Z  INFO http_request{method=POST uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/seek?start=1681.982&playback_session_id=1c801cec-91fa-430e-a022-96074d9cf284}: ferrite_stream::hls: HLS ffmpeg args: ["-hide_banner", "-nostdin", "-ss", "1672.973", "-i", "/home/badchiefy/files/tv/Asia (2024)/Season 1/Asia.S01E01.Beneath.the.Waves.1080p.WEBRip.10bit.EAC3.2.0.x265-iVy.mkv", "-ss", "9.009", "-map", "0:v:0", "-map", "0:a:0", "-vf", "format=yuv420p", "-c:v", "libx264", "-preset", "veryfast", "-crf", "23", "-profile:v", "high", "-level", "4.1", "-g", "180", "-keyint_min", "180", "-c:a", "aac", "-b:a", "192k", "-ac", "2", "-f", "hls", "-hls_time", "6", "-hls_list_size", "30", "-hls_segment_type", "fmp4", "-hls_fmp4_init_filename", "init.mp4", "-hls_segment_filename", "seg_%03d.m4s", "-hls_flags", "independent_segments+delete_segments+temp_file", "playlist.m3u8"]
2026-02-22T05:34:06.716902Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]: Input #0, matroska,webm, from '/home/badchiefy/files/tv/Asia (2024)/Season 1/Asia.S01E01.Beneath.the.Waves.1080p.WEBRip.10bit.EAC3.2.0.x265-iVy.mkv':
2026-02-22T05:34:06.716922Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]:   Metadata:
2026-02-22T05:34:06.716926Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]:     creation_time   : 2025-09-05T17:37:40.000000Z
2026-02-22T05:34:06.716930Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]:     ENCODER         : Lavf61.7.100
2026-02-22T05:34:06.716933Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]:   Duration: 00:58:00.05, start: -0.005000, bitrate: 4284 kb/s
2026-02-22T05:34:06.716937Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]:   Stream #0:0: Video: hevc (Main 10), yuv420p10le(tv, bt709), 1920x1080 [SAR 1:1 DAR 16:9], 30 fps, 30 tbr, 1k tbn (default)
2026-02-22T05:34:06.716941Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]:       Metadata:
2026-02-22T05:34:06.716944Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]:         DURATION        : 00:58:00.053000000
2026-02-22T05:34:06.716947Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]:   Stream #0:1(eng): Audio: eac3, 48000 Hz, stereo, fltp, 640 kb/s (default)
2026-02-22T05:34:06.716951Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]:       Metadata:
2026-02-22T05:34:06.716953Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]:         DURATION        : 00:58:00.000000000
2026-02-22T05:34:06.716956Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]:   Stream #0:2(eng): Subtitle: ass (ssa)
2026-02-22T05:34:06.716959Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]:       Metadata:
2026-02-22T05:34:06.716962Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]:         DURATION        : 00:57:24.800000000
2026-02-22T05:34:06.716965Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]:   Stream #0:3(eng): Subtitle: ass (ssa)
2026-02-22T05:34:06.716968Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]:       Metadata:
2026-02-22T05:34:06.716971Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]:         title           : SDH
2026-02-22T05:34:06.716974Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]:         DURATION        : 00:57:24.800000000
2026-02-22T05:34:06.720512Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]: Stream mapping:
2026-02-22T05:34:06.720522Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]:   Stream #0:0 -> #0:0 (hevc (native) -> h264 (libx264))
2026-02-22T05:34:06.720526Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]:   Stream #0:1 -> #0:1 (eac3 (native) -> aac (native))
2026-02-22T05:34:08.952838Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]: [libx264 @ 0x55ec6349e500] using SAR=1/1
2026-02-22T05:34:08.954891Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]: [libx264 @ 0x55ec6349e500] using cpu capabilities: MMX2 SSE2Fast SSSE3 SSE4.2 AVX FMA3 BMI2 AVX2
2026-02-22T05:34:08.961862Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]: [libx264 @ 0x55ec6349e500] profile High, level 4.1, 4:2:0, 8-bit
2026-02-22T05:34:08.962846Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]: [libx264 @ 0x55ec6349e500] 264 - core 164 - H.264/MPEG-4 AVC codec - Copyleft 2003-2024 - http://www.videolan.org/x264.html - options: cabac=1 ref=1 deblock=1:0:0 analyse=0x3:0x113 me=hex subme=2 psy=1 psy_rd=1.00:0.00 mixed_ref=0 me_range=16 chroma_me=1 trellis=0 8x8dct=1 cqm=0 deadzone=21,11 fast_pskip=1 chroma_qp_offset=0 threads=12 lookahead_threads=4 sliced_threads=0 nr=0 decimate=1 interlaced=0 bluray_compat=0 constrained_intra=0 bframes=3 b_pyramid=2 b_adapt=1 b_bias=0 direct=1 weightb=1 open_gop=0 weightp=1 keyint=180 keyint_min=91 scenecut=40 intra_refresh=0 rc_lookahead=10 rc=crf mbtree=1 crf=23.0 qcomp=0.60 qpmin=0 qpmax=69 qpstep=4 ip_ratio=1.40 aq=1:1.00
2026-02-22T05:34:08.962858Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]: [hls @ 0x55ec634ac340] Opening 'init.mp4' for writing
2026-02-22T05:34:08.962863Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]: Output #0, hls, to 'playlist.m3u8':
2026-02-22T05:34:08.962866Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]:   Metadata:
2026-02-22T05:34:08.962869Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]:     encoder         : Lavf61.7.100
2026-02-22T05:34:08.962872Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]:   Stream #0:0: Video: h264, yuv420p(tv, bt709, progressive), 1920x1080 [SAR 1:1 DAR 16:9], q=2-31, 30 fps, 15360 tbn (default)
2026-02-22T05:34:08.962876Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]:       Metadata:
2026-02-22T05:34:08.962880Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]:         DURATION        : 00:58:00.053000000
2026-02-22T05:34:08.962883Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]:         encoder         : Lavc61.19.101 libx264
2026-02-22T05:34:08.962887Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]:       Side data:
2026-02-22T05:34:08.962891Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]:         cpb: bitrate max/min/avg: 0/0/0 buffer size: 0 vbv_delay: N/A
2026-02-22T05:34:08.962895Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]:   Stream #0:1(eng): Audio: aac (LC), 48000 Hz, stereo, fltp, 192 kb/s (default)
2026-02-22T05:34:08.962899Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]:       Metadata:
2026-02-22T05:34:08.962903Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]:         DURATION        : 00:58:00.000000000
2026-02-22T05:34:08.962906Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]:         encoder         : Lavc61.19.101 aac
2026-02-22T05:34:11.265586Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]: frame=    6 fps=2.4 [hls @ 0x55ec634ac340] Opening 'seg_000.m4s.tmp' for writing=N/A dup=2 drop=0 speed=1.28x
2026-02-22T05:34:11.267175Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]: [hls @ 0x55ec634ac340] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:34:11.282365Z  INFO http_request{method=POST uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/seek?start=1681.982&playback_session_id=1c801cec-91fa-430e-a022-96074d9cf284}: ferrite_stream::hls: HLS session b1b19145-70cf-4d73-a3c9-31f6bc27810e ready (first segment generated in 4.6s)
2026-02-22T05:34:11.282445Z  INFO http_request{method=POST uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/seek?start=1681.982&playback_session_id=1c801cec-91fa-430e-a022-96074d9cf284}: ferrite_api::handlers::stream: HLS seek for ca663e67-38d2-4f47-9008-ed2de1f68a50: start=1673.0s mode=fast seek_source=index db=1ms seek=12ms session=6593ms total=6607ms
2026-02-22T05:34:11.342592Z  INFO http_request{method=GET uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/master.m3u8?start=1681.982&token=[REDACTED]&playback_session_id=1c801cec%2D91fa%2D430e%2Da022%2D96074d9cf284}: ferrite_api::handlers::stream: HLS master playlist for ca663e67-38d2-4f47-9008-ed2de1f68a50: variants=1 reused=true mode=fast seek_source=index db=1ms seek=0ms session=0ms total=1ms
2026-02-22T05:34:12.867935Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]: frame=  220 fps= 44 [hls @ 0x55ec634ac340] Opening 'seg_001.m4s.tmp' for writing=N/A dup=4 drop=0 speed=1.89x
2026-02-22T05:34:12.870466Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]: [hls @ 0x55ec634ac340] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:34:15.039036Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]: frame=  400 fps= 61 [hls @ 0x55ec634ac340] Opening 'seg_002.m4s.tmp' for writing=N/A dup=5 drop=0 speed=2.15x
2026-02-22T05:34:15.039891Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]: [hls @ 0x55ec634ac340] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:34:19.125617Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]: frame=  556 fps= 65 [hls @ 0x55ec634ac340] Opening 'seg_003.m4s.tmp' for writing=N/A dup=9 drop=0 speed=2.34x
2026-02-22T05:34:19.125643Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]: [hls @ 0x55ec634ac340] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:34:21.190166Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]: frame=  882 fps= 70 [hls @ 0x55ec634ac340] Opening 'seg_004.m4s.tmp' for writing=N/A dup=11 drop=0 speed=2.43x
2026-02-22T05:34:21.191147Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]: [hls @ 0x55ec634ac340] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:34:23.567046Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]: frame= 1062 fps= 73 [hls @ 0x55ec634ac340] Opening 'seg_005.m4s.tmp' for writing=N/A dup=12 drop=0 speed=2.45x
2026-02-22T05:34:23.568012Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]: [hls @ 0x55ec634ac340] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:34:25.460262Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]: frame= 1253 fps= 74 [hls @ 0x55ec634ac340] Opening 'seg_006.m4s.tmp' for writing=N/A dup=13 drop=0 speed=2.45x
2026-02-22T05:34:25.460294Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]: [hls @ 0x55ec634ac340] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:34:27.282618Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]: frame= 1424 fps= 75 [hls @ 0x55ec634ac340] Opening 'seg_007.m4s.tmp' for writing=N/A dup=15 drop=0 speed=2.53x
2026-02-22T05:34:27.282643Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]: [hls @ 0x55ec634ac340] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:34:28.498318Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]: frame= 1597 fps= 76 [hls @ 0x55ec634ac340] Opening 'seg_008.m4s.tmp' for writing=N/A dup=16 drop=0 speed=2.54x
2026-02-22T05:34:28.498476Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]: [hls @ 0x55ec634ac340] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:34:29.803008Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]: frame= 1687 fps= 77 [hls @ 0x55ec634ac340] Opening 'seg_009.m4s.tmp' for writingpeed=2.55x
2026-02-22T05:34:29.805263Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]: [hls @ 0x55ec634ac340] Opening 'playlist.m3u8.tmp' for writing
2026-02-22T05:34:29.805350Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]: [out#0/hls @ 0x55ec634bd300] video:31878KiB audio:1418KiB subtitle:0KiB other streams:0KiB global headers:0KiB muxing overhead: unknown
2026-02-22T05:34:29.805377Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]: frame= 1802 fps= 78 q=-1.0 Lsize=N/A time=00:00:59.87 bitrate=N/A dup=21 drop=0 speed=2.59x
2026-02-22T05:34:29.808760Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]: [libx264 @ 0x55ec6349e500] frame I:22    Avg QP:20.43  size:121015
2026-02-22T05:34:29.808772Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]: [libx264 @ 0x55ec6349e500] frame P:496   Avg QP:24.26  size: 39408
2026-02-22T05:34:29.808775Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]: [libx264 @ 0x55ec6349e500] frame B:1284  Avg QP:27.58  size:  8126
2026-02-22T05:34:29.808778Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]: [libx264 @ 0x55ec6349e500] consecutive B-frames:  2.8%  3.8%  8.2% 85.2%
2026-02-22T05:34:29.808789Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]: [libx264 @ 0x55ec6349e500] mb I  I16..4: 10.6% 46.1% 43.2%
2026-02-22T05:34:29.808792Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]: [libx264 @ 0x55ec6349e500] mb P  I16..4:  2.6% 22.1%  2.9%  P16..4: 24.7% 15.6%  9.1%  0.0%  0.0%    skip:23.1%
2026-02-22T05:34:29.808796Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]: [libx264 @ 0x55ec6349e500] mb B  I16..4:  0.4%  3.0%  0.1%  B16..8: 16.9%  4.8%  0.6%  direct: 2.9%  skip:71.3%  L0:36.5% L1:45.4% BI:18.1%
2026-02-22T05:34:29.808800Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]: [libx264 @ 0x55ec6349e500] 8x8 transform intra:77.8% inter:50.8%
2026-02-22T05:34:29.808804Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]: [libx264 @ 0x55ec6349e500] coded y,uvDC,uvAC intra: 60.1% 19.3% 1.1% inter: 7.6% 2.0% 0.0%
2026-02-22T05:34:29.808808Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]: [libx264 @ 0x55ec6349e500] i16 v,h,dc,p: 40% 18% 12% 29%
2026-02-22T05:34:29.808813Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]: [libx264 @ 0x55ec6349e500] i8 v,h,dc,ddl,ddr,vr,hd,vl,hu: 23% 23% 16%  6%  7%  6%  7%  6%  7%
2026-02-22T05:34:29.808817Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]: [libx264 @ 0x55ec6349e500] i4 v,h,dc,ddl,ddr,vr,hd,vl,hu: 19% 17% 10%  8% 10% 10%  9%  9%  8%
2026-02-22T05:34:29.808820Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]: [libx264 @ 0x55ec6349e500] i8c dc,h,v,p: 69% 14% 15%  2%
2026-02-22T05:34:29.808828Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]: [libx264 @ 0x55ec6349e500] Weighted P-Frames: Y:0.2% UV:0.0%
2026-02-22T05:34:29.808831Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]: [libx264 @ 0x55ec6349e500] kb/s:4347.47
2026-02-22T05:34:29.854497Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]: [aac @ 0x55ec63c7d240] Qavg: 588.713
2026-02-22T05:34:29.887420Z  WARN ferrite_stream::hls: ffmpeg HLS [b1b19145-70cf-4d73-a3c9-31f6bc27810e]: Exiting normally, received signal 15.
2026-02-22T05:34:30.820550Z  INFO http_request{method=DELETE uri=/api/stream/ca663e67-38d2-4f47-9008-ed2de1f68a50/hls/session/stop?playback_session_id=1c801cec-91fa-430e-a022-96074d9cf284}: ferrite_stream::hls: Destroyed HLS session b1b19145-70cf-4d73-a3c9-31f6bc27810e
