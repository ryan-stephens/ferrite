-- Add color metadata columns to media_streams for HDR vs 10-bit SDR detection.
-- FFprobe provides color_space, color_transfer, and color_primaries which are
-- needed to distinguish true HDR (BT.2020/PQ) from 10-bit SDR (BT.709).

ALTER TABLE media_streams ADD COLUMN color_space TEXT;       -- e.g. 'bt2020nc', 'bt709'
ALTER TABLE media_streams ADD COLUMN color_transfer TEXT;    -- e.g. 'smpte2084' (PQ), 'arib-std-b67' (HLG), 'bt709'
ALTER TABLE media_streams ADD COLUMN color_primaries TEXT;   -- e.g. 'bt2020', 'bt709'
