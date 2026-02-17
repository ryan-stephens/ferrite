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

/** Fire-and-forget API call (progress reporting) */
export function apiQuiet(method: string, path: string, body?: unknown): void {
  const opts: RequestInit = { method, headers: authHeaders() };
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

export interface AuthStatus {
  auth_required: boolean;
  has_users: boolean;
}

// ---- API functions ----

export const api = {
  // Auth
  authStatus: () => apiFetch<AuthStatus>('GET', '/api/auth/status'),
  login: (username: string, password: string) =>
    apiFetch<{ token: string }>('POST', '/api/auth/login', { username, password }),

  // System
  info: () => apiFetch<{ name: string; version: string }>('GET', '/api/system/info'),

  // Libraries
  listLibraries: () => apiFetch<Library[]>('GET', '/api/libraries'),
  createLibrary: (name: string, path: string, library_type: string) =>
    apiFetch<Library>('POST', '/api/libraries', { name, path, library_type }),
  deleteLibrary: (id: string) => apiFetch<void>('DELETE', `/api/libraries/${id}`),
  scanLibrary: (id: string) => apiFetch<void>('POST', `/api/libraries/${id}/scan`),

  // Media
  listMedia: (params?: Record<string, string>) => {
    const qs = new URLSearchParams({ page: '1', per_page: '500', ...params });
    return apiFetch<MediaListResponse>('GET', `/api/media?${qs}`);
  },
  getMedia: (id: string) => apiFetch<MediaItem>('GET', `/api/media/${id}`),
  getStreams: (id: string) => apiFetch<MediaStream[]>('GET', `/api/media/${id}/streams`),

  // Progress
  updateProgress: (mediaId: string, positionMs: number) =>
    apiQuiet('PUT', `/api/progress/${mediaId}`, { position_ms: positionMs }),
  markCompleted: (mediaId: string) =>
    apiQuiet('POST', `/api/progress/${mediaId}/complete`),

  // HLS
  hlsSeek: (id: string, start: number, audioStream?: number) => {
    const params = new URLSearchParams({ start: start.toFixed(3) });
    if (audioStream != null) params.set('audio_stream', String(audioStream));
    return apiFetch<{ session_id: string; master_url: string; start_secs: number; variant_count: number; timing_ms?: Record<string, number> }>(
      'POST', `/api/stream/${id}/hls/seek?${params}`
    );
  },
  hlsStop: (id: string, sessionId: string) =>
    apiQuiet('DELETE', `/api/stream/${id}/hls/${sessionId}`),

  // Users
  createUser: (username: string, password: string) =>
    apiFetch<{ id: string }>('POST', '/api/users', { username, password }),
  setupStatus: () => apiFetch<{ has_users: boolean }>('GET', '/api/users/setup'),
};
