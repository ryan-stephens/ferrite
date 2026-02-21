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

export type ClientProfile = 'web-chrome' | 'safari-ios' | 'android' | 'tvos' | 'roku';

const PROFILE_CAPABILITIES: Record<ClientProfile, {
  audio: string[];
  video: string[];
  containers: string[];
}> = {
  'web-chrome': {
    audio: ['aac', 'mp3', 'opus', 'vorbis', 'flac', 'pcm_s16le', 'pcm_s24le', 'pcm_f32le'],
    video: ['h264', 'vp8', 'vp9', 'av1'],
    containers: ['mp4', 'mov', 'webm', 'ogg', 'flac', 'wav'],
  },
  'safari-ios': {
    audio: ['aac', 'mp3', 'alac'],
    video: ['h264', 'hevc'],
    containers: ['mp4', 'mov', 'm4v', 'm4a'],
  },
  android: {
    audio: ['aac', 'mp3', 'opus', 'vorbis', 'flac'],
    video: ['h264', 'vp8', 'vp9', 'av1', 'hevc'],
    containers: ['mp4', 'mov', 'webm'],
  },
  tvos: {
    audio: ['aac', 'mp3', 'alac', 'ac3', 'eac3'],
    video: ['h264', 'hevc'],
    containers: ['mp4', 'mov', 'm4v', 'm4a'],
  },
  roku: {
    audio: ['aac', 'mp3', 'ac3', 'eac3'],
    video: ['h264', 'hevc'],
    containers: ['mp4', 'mov', 'mkv', 'matroska'],
  },
};

function parseClientProfile(value?: string | null): ClientProfile | null {
  if (!value) return null;
  switch (value.trim().toLowerCase()) {
    case 'web-chrome':
    case 'chrome':
    case 'web':
    case 'default':
      return 'web-chrome';
    case 'safari-ios':
    case 'ios':
    case 'iphone':
    case 'ipad':
      return 'safari-ios';
    case 'android':
      return 'android';
    case 'tvos':
    case 'apple-tv':
    case 'appletv':
      return 'tvos';
    case 'roku':
      return 'roku';
    default:
      return null;
  }
}

function inferClientProfileFromNavigator(): ClientProfile {
  if (typeof navigator === 'undefined') return 'web-chrome';
  const ua = navigator.userAgent.toLowerCase();
  const nav = navigator as Navigator & { userAgentData?: { platform?: string } };
  const platform = [navigator.platform || '', nav.userAgentData?.platform || '']
    .join(' ')
    .toLowerCase();

  if (ua.includes('roku')) return 'roku';
  if (ua.includes('appletv') || ua.includes('apple tv') || ua.includes('tvos')) return 'tvos';
  if (ua.includes('iphone') || ua.includes('ipad') || ua.includes('ipod') || platform.includes('ios')) {
    return 'safari-ios';
  }
  if (ua.includes('android') || platform.includes('android')) return 'android';
  return 'web-chrome';
}

export function resolveClientProfile(explicitOverride?: string | null): ClientProfile {
  return parseClientProfile(explicitOverride) || inferClientProfileFromNavigator();
}

export type StreamType = 'direct' | 'remux' | 'audio-transcode' | 'full-transcode';

/** Determine stream compatibility */
export function getStreamType(
  container: string | null,
  videoCodec: string | null,
  audioCodec: string | null,
  explicitProfile?: string | null,
): StreamType {
  const profile = resolveClientProfile(explicitProfile);
  const caps = PROFILE_CAPABILITIES[profile];
  const cOk = container ? caps.containers.includes(container.toLowerCase()) : false;
  const vOk = !videoCodec || caps.video.includes(videoCodec.toLowerCase());
  const aOk = !audioCodec || caps.audio.includes(audioCodec.toLowerCase());
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
