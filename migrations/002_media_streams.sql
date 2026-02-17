-- Stores individual audio, video, and subtitle tracks discovered by ffprobe.
-- One media_item can have many streams (e.g. 1 video + 3 audio + 2 subtitle).

CREATE TABLE IF NOT EXISTS media_streams (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    media_item_id TEXT NOT NULL REFERENCES media_items(id) ON DELETE CASCADE,
    stream_index INTEGER NOT NULL,
    stream_type TEXT NOT NULL,          -- 'video', 'audio', 'subtitle'
    codec_name TEXT,                     -- e.g. 'h264', 'aac', 'srt'
    codec_long_name TEXT,                -- e.g. 'H.264 / AVC / MPEG-4 AVC / MPEG-4 part 10'
    profile TEXT,                        -- e.g. 'High', 'LC'
    language TEXT,                       -- ISO 639 code, e.g. 'eng', 'jpn'
    title TEXT,                          -- stream title tag, e.g. 'Commentary'
    is_default INTEGER NOT NULL DEFAULT 0,
    is_forced INTEGER NOT NULL DEFAULT 0,

    -- Video-specific
    width INTEGER,
    height INTEGER,
    frame_rate TEXT,                     -- e.g. '23.976', '29.97'
    pixel_format TEXT,                   -- e.g. 'yuv420p'
    bit_depth INTEGER,                   -- e.g. 8, 10

    -- Audio-specific
    channels INTEGER,                    -- e.g. 2, 6, 8
    channel_layout TEXT,                 -- e.g. 'stereo', '5.1(side)'
    sample_rate INTEGER,                 -- e.g. 48000, 44100

    -- Common
    bitrate_bps INTEGER,                 -- stream bitrate in bits/sec

    UNIQUE(media_item_id, stream_index)
);

CREATE INDEX IF NOT EXISTS idx_media_streams_item ON media_streams(media_item_id);
CREATE INDEX IF NOT EXISTS idx_media_streams_type ON media_streams(media_item_id, stream_type);
