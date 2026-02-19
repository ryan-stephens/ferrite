/** Format bytes to human-readable size */
export function formatSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1048576) return `${(bytes / 1024).toFixed(1)} KB`;
  if (bytes < 1073741824) return `${(bytes / 1048576).toFixed(1)} MB`;
  return `${(bytes / 1073741824).toFixed(2)} GB`;
}

/** Format milliseconds to human-readable duration (e.g. "1h 30m") */
export function formatDuration(ms: number | null): string {
  if (!ms) return '';
  const s = Math.floor(ms / 1000);
  const h = Math.floor(s / 3600);
  const m = Math.floor((s % 3600) / 60);
  if (h > 0) return `${h}h ${m}m`;
  return `${m}m`;
}

/** Format seconds to player time (e.g. "1:05:30") */
export function fmtTime(totalSec: number): string {
  if (!totalSec || !isFinite(totalSec)) return '0:00';
  const h = Math.floor(totalSec / 3600);
  const m = Math.floor((totalSec % 3600) / 60);
  const s = Math.floor(totalSec % 60);
  if (h > 0) return `${h}:${m.toString().padStart(2, '0')}:${s.toString().padStart(2, '0')}`;
  return `${m}:${s.toString().padStart(2, '0')}`;
}

/** Get resolution label from dimensions */
export function getResLabel(w: number | null, h: number | null): string {
  if (!w || !h) return '';
  if (h >= 2160) return '4K';
  if (h >= 1080) return '1080p';
  if (h >= 720) return '720p';
  if (h >= 480) return '480p';
  return `${h}p`;
}

const COMPAT_AUDIO = ['aac', 'mp3', 'opus', 'vorbis', 'flac', 'pcm_s16le', 'pcm_s24le', 'pcm_f32le'];
const COMPAT_VIDEO = ['h264', 'vp8', 'vp9', 'av1'];
const COMPAT_CONTAINER = ['mp4', 'mov', 'webm', 'ogg', 'flac', 'wav'];

export type StreamType = 'direct' | 'remux' | 'audio-transcode' | 'full-transcode';

/** Determine stream compatibility */
export function getStreamType(
  container: string | null,
  videoCodec: string | null,
  audioCodec: string | null,
): StreamType {
  const cOk = container ? COMPAT_CONTAINER.includes(container.toLowerCase()) : false;
  const vOk = !videoCodec || COMPAT_VIDEO.includes(videoCodec.toLowerCase());
  const aOk = !audioCodec || COMPAT_AUDIO.includes(audioCodec.toLowerCase());
  if (cOk && vOk && aOk) return 'direct';
  if (vOk && aOk && !cOk) return 'remux';
  if (vOk && !aOk) return 'audio-transcode';
  return 'full-transcode';
}

/** Get display title for a media item */
export function getDisplayTitle(item: {
  movie_title?: string | null;
  show_title?: string | null;
  is_episode?: number;
  title: string;
  file_path: string;
}): string {
  if (item.is_episode) return item.show_title || item.title || item.file_path.split(/[/\\]/).pop() || 'Unknown';
  return item.movie_title || item.title || item.file_path.split(/[/\\]/).pop() || 'Unknown';
}

/** Get episode label e.g. "S01E04" */
export function getEpisodeLabel(item: { season_number?: number | null; episode_number?: number | null }): string | null {
  if (item.season_number == null || item.episode_number == null) return null;
  return `S${String(item.season_number).padStart(2, '0')}E${String(item.episode_number).padStart(2, '0')}`;
}

/** Get display year for a media item */
export function getDisplayYear(item: { movie_year?: number | null; year?: number | null }): number | null {
  return item.movie_year || item.year || null;
}
