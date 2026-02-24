/** Typed API client for Ferrite backend */

export function getToken(): string | null {
  return localStorage.getItem('ferrite-token');
}

export function setToken(token: string): void {
  localStorage.setItem('ferrite-token', token);
}

export function clearToken(): void {
  localStorage.removeItem('ferrite-token');
}

function authHeaders(): Record<string, string> {
  const h: Record<string, string> = { 'Content-Type': 'application/json' };
  const t = getToken();
  if (t) h['Authorization'] = `Bearer ${t}`;
  return h;
}

/** Append auth token as query param for src attributes (video, img) */
export function authUrl(url: string): string {
  const t = getToken();
  if (!t) return url;
  const sep = url.includes('?') ? '&' : '?';
  return `${url}${sep}token=${encodeURIComponent(t)}`;
}

/** Core fetch wrapper with auth + 401 handling */
async function apiFetch<T>(method: string, path: string, body?: unknown): Promise<T> {
  const opts: RequestInit = { method, headers: authHeaders() };
  if (body) opts.body = JSON.stringify(body);
  const res = await fetch(path, opts);
  if (res.status === 401) {
    clearToken();
    window.dispatchEvent(new CustomEvent('ferrite:unauthorized'));
    throw new Error('Unauthorized');
  }
  if (!res.ok) {
    const err = await res.json().catch(() => ({ error: res.statusText }));
    throw new Error(err.error || `${res.status} ${res.statusText}`);
  }
  if (res.status === 204) return undefined as T;
  return res.json();
}

/** Fire-and-forget API call (progress reporting, session cleanup).
 *  keepalive: true ensures the request completes even if the component
 *  unmounts or the user navigates away before the response arrives. */
export function apiQuiet(method: string, path: string, body?: unknown): void {
  const opts: RequestInit = { method, headers: authHeaders(), keepalive: true };
  if (body) opts.body = JSON.stringify(body);
  fetch(path, opts).catch(() => {});
}

// ---- Types ----

export interface Library {
  id: string;
  name: string;
  path: string;
  library_type: string;
}

export interface MediaItem {
  id: string;
  title: string;
  file_path: string;
  file_size: number;
  duration_ms: number | null;
  width: number | null;
  height: number | null;
  video_codec: string | null;
  audio_codec: string | null;
  container_format: string | null;
  poster_path: string | null;
  overview: string | null;
  genres: string | null;
  rating: number | null;
  content_rating: string | null;
  year: number | null;
  movie_title: string | null;
  movie_year: number | null;
  bitrate_kbps: number | null;
  position_ms: number | null;
  completed: boolean;
  last_played_at: string | null;
  added_at: string | null;
  library_id: string;
  is_episode: number;
  episode_number: number | null;
  episode_title: string | null;
  season_number: number | null;
  show_title: string | null;
  still_path: string | null;
}

export interface MediaListResponse {
  items: MediaItem[];
  total: number;
  page: number;
  per_page: number;
}

export interface MediaStream {
  id: number;
  media_item_id: string;
  stream_index: number;
  stream_type: string;
  codec_name: string | null;
  codec_long_name: string | null;
  profile: string | null;
  language: string | null;
  title: string | null;
  is_default: number;
  is_forced: number;
  width: number | null;
  height: number | null;
  frame_rate: string | null;
  pixel_format: string | null;
  bit_depth: number | null;
  channels: number | null;
  channel_layout: string | null;
  sample_rate: number | null;
  bitrate_bps: number | null;
}

export interface UserPreferences {
  default_subtitle_language?: string;
  default_audio_language?: string;
  max_quality?: string;
}

export interface ActiveStream {
  session_id: string;
  media_id: string;
  variant_label: string | null;
  start_secs: number;
  width: number | null;
  height: number | null;
  bitrate_kbps: number | null;
  idle_secs: number;
  age_secs: number;
}

export interface Chapter {
  id: number;
  media_item_id: string;
  chapter_index: number;
  title: string | null;
  start_time_ms: number;
  end_time_ms: number;
}

export interface ExternalSubtitle {
  id: number;
  media_item_id: string;
  file_path: string;
  format: string;
  language: string | null;
  title: string | null;
  is_forced: number;
  is_sdh: number;
  file_size: number;
}

export interface NextEpisode {
  media_item_id: string;
  season_id: string;
  episode_number: number;
  season_number: number;
  episode_title: string | null;
  overview: string | null;
  still_path: string | null;
  duration_ms: number | null;
  show_title: string;
  show_poster_path: string | null;
}

export interface TvShow {
  id: string;
  library_id: string;
  title: string;
  sort_title: string | null;
  year: number | null;
  overview: string | null;
  status: string | null;
  tmdb_id: number | null;
  tvdb_id: number | null;
  poster_path: string | null;
  backdrop_path: string | null;
  genres: string | null;
  fetched_at: string | null;
  season_count: number;
  episode_count: number;
}

export interface Season {
  id: string;
  tv_show_id: string;
  season_number: number;
  title: string | null;
  overview: string | null;
  poster_path: string | null;
  episode_count: number;
}

export interface Episode {
  media_item_id: string;
  season_id: string;
  episode_number: number;
  episode_title: string | null;
  overview: string | null;
  air_date: string | null;
  still_path: string | null;
  file_path: string;
  file_size: number;
  duration_ms: number | null;
  video_codec: string | null;
  audio_codec: string | null;
  width: number | null;
  height: number | null;
  position_ms: number | null;
  completed: number | null;
  last_played_at: string | null;
}

export interface ScanProgress {
  scanning: boolean;
  status: 'scanning' | 'enriching' | 'complete' | 'failed';
  total_files: number;
  files_probed: number;
  files_inserted: number;
  subtitles_extracted: number;
  items_enriched: number;
  errors: number;
  current_item: string;
  elapsed_seconds: number;
  percent: number;
}

export interface AuthStatus {
  auth_required: boolean;
  has_users: boolean;
}

export interface UpdateCheckResult {
  current_version: string;
  latest_version: string;
  update_available: boolean;
  release_url: string;
  release_notes: string;
  published_at: string;
  download_url: string | null;
  download_size_bytes: number | null;
}

export interface UpdateProgress {
  state: 'idle' | 'downloading' | 'verifying' | 'extracting' | 'swapping' | 'restarting' | 'failed';
  progress_pct: number;
  downloaded_bytes: number;
  total_bytes: number;
  error: string | null;
}

export interface UpdateHistoryEntry {
  from_version: string;
  to_version: string;
  applied_at: string;
  success: boolean;
  error: string | null;
}

export interface PlaybackMetricTrackRequest {
  metric: 'playback_ttff_ms' | 'seek_latency_ms' | 'rebuffer_count' | 'rebuffer_ms';
  value_ms?: number;
  increment?: number;
  labels?: Record<string, string>;
}

export interface HlsSessionStartResponse {
  playback_session_id: string;
  master_url: string;
}

export interface User {
  id: string;
  username: string;
  display_name: string | null;
  is_admin: number;
  created_at: string;
  last_login_at: string | null;
}

// ---- API functions ----

export const api = {
  // Auth
  authStatus: () => apiFetch<AuthStatus>('GET', '/api/auth/status'),
  login: (username: string, password: string) =>
    apiFetch<{ token: string }>('POST', '/api/auth/login', { username, password }),

  // System
  info: () => apiFetch<{ name: string; version: string }>('GET', '/api/system/info'),
  playbackMetrics: () => apiFetch<{ timings: unknown[]; counters: unknown[] }>('GET', '/api/system/metrics'),
  resetPlaybackMetrics: () => apiFetch<void>('DELETE', '/api/system/metrics'),
  trackPlaybackMetric: (payload: PlaybackMetricTrackRequest) =>
    apiQuiet('POST', '/api/system/metrics/track', payload),

  // Users
  listUsers: () => apiFetch<User[]>('GET', '/api/users'),
  createUser: (username: string, password: string, displayName: string | null, isAdmin: boolean) =>
    apiFetch<User>('POST', '/api/users', { username, password, display_name: displayName, is_admin: isAdmin }),
  deleteUser: (id: string) => apiFetch<void>('DELETE', `/api/users/${id}`),
  adminResetPassword: (id: string, newPassword: string) =>
    apiFetch<void>('PUT', `/api/users/${id}/password`, { new_password: newPassword }),
  getCurrentUser: () => apiFetch<User>('GET', '/api/users/me'),

  // Libraries
  listLibraries: () => apiFetch<Library[]>('GET', '/api/libraries'),
  createLibrary: (name: string, path: string, library_type: string) =>
    apiFetch<Library>('POST', '/api/libraries', { name, path, library_type }),
  deleteLibrary: (id: string) => apiFetch<void>('DELETE', `/api/libraries/${id}`),
  scanLibrary: (id: string) => apiFetch<void>('POST', `/api/libraries/${id}/scan`),
  scanStatus: (id: string) => apiFetch<ScanProgress>('GET', `/api/libraries/${id}/scan/status`),

  // Media
  listMedia: (params?: Record<string, string>) => {
    const qs = new URLSearchParams({ page: '1', per_page: '500', ...params });
    return apiFetch<MediaListResponse>('GET', `/api/media?${qs}`);
  },
  getMedia: (id: string) => apiFetch<MediaItem>('GET', `/api/media/${id}`),
  getStreams: (id: string) => apiFetch<MediaStream[]>('GET', `/api/media/${id}/streams`),
  listSubtitles: (id: string) => apiFetch<ExternalSubtitle[]>('GET', `/api/media/${id}/subtitles`),
  listChapters: (id: string) => apiFetch<Chapter[]>('GET', `/api/media/${id}/chapters`),
  listActiveStreams: () => apiFetch<{ sessions: ActiveStream[]; count: number }>('GET', '/api/admin/streams'),
  getPreferences: () => apiFetch<UserPreferences>('GET', '/api/preferences'),
  setPreferences: (prefs: Partial<UserPreferences>) =>
    apiFetch<void>('PUT', '/api/preferences', { preferences: prefs }),
  nextEpisode: (mediaItemId: string) =>
    apiFetch<{ next: NextEpisode | null }>('GET', `/api/episodes/${mediaItemId}/next`),

  // Progress
  updateProgress: (mediaId: string, positionMs: number) =>
    apiQuiet('PUT', `/api/progress/${mediaId}`, { position_ms: positionMs }),
  markCompleted: (mediaId: string) =>
    apiQuiet('POST', `/api/progress/${mediaId}/complete`),
  resetProgress: (mediaId: string) =>
    apiQuiet('DELETE', `/api/progress/${mediaId}`),

  // HLS
  hlsSessionStart: (id: string, start?: number, playbackSessionId?: string) => {
    const params = new URLSearchParams();
    if (start != null) params.set('start', start.toFixed(3));
    if (playbackSessionId) params.set('playback_session_id', playbackSessionId);
    const suffix = params.toString();
    return apiFetch<HlsSessionStartResponse>(
      'POST',
      `/api/stream/${id}/hls/session/start${suffix ? `?${suffix}` : ''}`,
    );
  },
  hlsSessionHeartbeat: (id: string, playbackSessionId: string) =>
    apiQuiet(
      'POST',
      `/api/stream/${id}/hls/session/heartbeat?playback_session_id=${encodeURIComponent(playbackSessionId)}`,
    ),
  hlsSessionStop: (id: string, playbackSessionId: string) =>
    apiQuiet(
      'DELETE',
      `/api/stream/${id}/hls/session/stop?playback_session_id=${encodeURIComponent(playbackSessionId)}`,
    ),
  hlsSeek: (id: string, start: number, audioStream?: number, playbackSessionId?: string) => {
    const params = new URLSearchParams({ start: start.toFixed(3) });
    if (audioStream != null) params.set('audio_stream', String(audioStream));
    if (playbackSessionId) params.set('playback_session_id', playbackSessionId);
    return apiFetch<{
      session_id: string;
      master_url: string;
      start_secs: number;
      variant_count: number;
      reused: boolean;
      video_copied?: boolean;
      timing_ms?: Record<string, number>;
    }>(
      'POST', `/api/stream/${id}/hls/seek?${params}`
    );
  },
  hlsStop: (id: string, sessionId: string) =>
    apiQuiet('DELETE', `/api/stream/${id}/hls/${sessionId}`),
  hlsStopMedia: (id: string, playbackSessionId?: string) => {
    const suffix = playbackSessionId
      ? `?playback_session_id=${encodeURIComponent(playbackSessionId)}`
      : '';
    apiQuiet('DELETE', `/api/stream/${id}/hls${suffix}`);
  },

  // TV Shows
  listShows: (libraryId: string) => apiFetch<TvShow[]>('GET', `/api/shows?library_id=${libraryId}`),
  getShow: (id: string) => apiFetch<TvShow>('GET', `/api/shows/${id}`),
  listSeasons: (showId: string) => apiFetch<Season[]>('GET', `/api/shows/${showId}/seasons`),
  listEpisodes: (seasonId: string) => apiFetch<Episode[]>('GET', `/api/seasons/${seasonId}/episodes`),

  setupStatus: () => apiFetch<{ has_users: boolean }>('GET', '/api/users/setup'),

  // System Update
  checkForUpdate: () => apiFetch<UpdateCheckResult>('GET', '/api/system/update/check'),
  applyUpdate: () => apiFetch<{ status: string; message: string }>('POST', '/api/system/update/apply'),
  updateStatus: () => apiFetch<UpdateProgress>('GET', '/api/system/update/status'),
  rollbackUpdate: () => apiFetch<{ status: string; message: string }>('POST', '/api/system/update/rollback'),
  updateHistory: () => apiFetch<UpdateHistoryEntry[]>('GET', '/api/system/update/history'),
};
