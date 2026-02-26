import { createSignal, onMount, onCleanup, Show, For } from 'solid-js';
import Hls from 'hls.js';
import {
  Play, Pause, Volume2, VolumeX, Volume1,
  Maximize, Minimize, ArrowLeft, SkipBack, SkipForward,
  Settings, Loader2, PictureInPicture2, Languages, Captions, SlidersHorizontal,
} from 'lucide-solid';
import type { MediaItem, MediaStream, ExternalSubtitle, NextEpisode, Chapter } from '../api';
import { api, authUrl, getToken } from '../api';
import { getDisplayTitle, getStreamType, fmtTime } from '../utils';
import { perf } from '../lib/perf';
import PerfOverlay from './PerfOverlay';

interface PlayerProps {
  item: MediaItem;
  resumePosition: number | null;
  isEpisode?: boolean;
  onClose: () => void;
  onNextEpisode?: (mediaItemId: string) => void;
}

const SPEED_OPTIONS = [0.5, 0.75, 1, 1.25, 1.5, 2];

// ---- Playback preference persistence (sessionStorage, scoped to library) ----
interface PlaybackPrefs {
  audioTrackIndex?: number;
  subtitleTrackId?: number | null;
  qualityHeight?: number;
}

function prefsKey(libraryId: string): string {
  return `ferrite-prefs-${libraryId}`;
}

function loadPrefs(libraryId: string): PlaybackPrefs {
  try {
    const raw = sessionStorage.getItem(prefsKey(libraryId));
    return raw ? JSON.parse(raw) : {};
  } catch { return {}; }
}

function savePrefs(libraryId: string, prefs: PlaybackPrefs): void {
  try {
    const existing = loadPrefs(libraryId);
    sessionStorage.setItem(prefsKey(libraryId), JSON.stringify({ ...existing, ...prefs }));
  } catch {}
}

/** HLS.js config tuned for fast TTFF and smooth playback from a local server */
const HLS_CONFIG = {
  maxBufferLength: 30,
  maxMaxBufferLength: 60,
  maxBufferSize: 60 * 1000 * 1000,
  backBufferLength: 15,
  maxBufferHole: 0.5,
  highBufferWatchdogPeriod: 3,
  detectStallWithCurrentTimeMs: 2500,
  lowLatencyMode: false,
  startFragPrefetch: true,
  testBandwidth: false,
  abrEwmaDefaultEstimate: 10_000_000,
  enableWorker: true,
};

function createHls(): Hls {
  return new Hls({
    ...HLS_CONFIG,
    xhrSetup: (xhr: XMLHttpRequest) => {
      const t = getToken();
      if (t) xhr.setRequestHeader('Authorization', `Bearer ${t}`);
    },
  });
}

function createPlaybackSessionId(): string {
  if (typeof crypto !== 'undefined' && typeof crypto.randomUUID === 'function') {
    return crypto.randomUUID();
  }
  return `ps-${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 10)}`;
}

export default function Player(props: PlayerProps) {
  let playerRef!: HTMLDivElement;
  let videoRef!: HTMLVideoElement;
  let timelineRef!: HTMLDivElement;
  let controlsTimeout: ReturnType<typeof setTimeout> | null = null;

  // ---- Signals ----
  const [playing, setPlaying] = createSignal(false);
  const [currentTime, setCurrentTime] = createSignal(0);
  const [bufferedPct, setBufferedPct] = createSignal(0);
  const [volume, setVolume] = createSignal(
    parseInt(localStorage.getItem('ferrite-volume') || '100')
  );
  const [lastVolume, setLastVolume] = createSignal(100);
  const [isFullscreen, setIsFullscreen] = createSignal(false);
  const [controlsVisible, setControlsVisible] = createSignal(true);
  const [buffering, setBuffering] = createSignal(true);
  const [seekIndicator, setSeekIndicator] = createSignal<string | null>(null);
  const [hoverTime, setHoverTime] = createSignal<number | null>(null);
  const [hoverX, setHoverX] = createSignal(0);
  const [showSettings, setShowSettings] = createSignal(false);
  const [playbackSpeed, setPlaybackSpeed] = createSignal(1);
  const [isDragging, setIsDragging] = createSignal(false);
  const [dragPct, setDragPct] = createSignal(0);
  const [showPerf, setShowPerf] = createSignal(false);
  const [audioTracks, setAudioTracks] = createSignal<MediaStream[]>([]);
  const [selectedAudioTrack, setSelectedAudioTrack] = createSignal(0);
  const [showAudioMenu, setShowAudioMenu] = createSignal(false);
  const [subtitleTracks, setSubtitleTracks] = createSignal<ExternalSubtitle[]>([]);
  const [selectedSubtitle, setSelectedSubtitle] = createSignal<number | null>(null); // null = off
  const [showSubtitleMenu, setShowSubtitleMenu] = createSignal(false);
  const [qualityLevels, setQualityLevels] = createSignal<{ index: number; height: number; bitrate: number; label: string }[]>([]);
  const [selectedQuality, setSelectedQuality] = createSignal(-1); // -1 = auto
  const [showQualityMenu, setShowQualityMenu] = createSignal(false);
  const [currentQualityLabel, setCurrentQualityLabel] = createSignal('Auto');
  const [activeCue, setActiveCue] = createSignal<string | null>(null);
  const [chapters, setChapters] = createSignal<Chapter[]>([]);
  const [nextEpisode, setNextEpisode] = createSignal<NextEpisode | null>(null);
  const [upNextVisible, setUpNextVisible] = createSignal(false);
  const [upNextCountdown, setUpNextCountdown] = createSignal(15);
  let upNextTimer: ReturnType<typeof setInterval> | null = null;
  let upNextStarted = false; // plain flag — safe to read in non-reactive contexts
  let upNextNavigating = false; // set when navigation to next episode is in flight

  const item = () => props.item;
  const stream = () => getStreamType(item().container_format, item().video_codec, item().audio_codec);
  const isTranscoded = () => stream() !== 'direct';
  const isFullTranscode = () => stream() === 'full-transcode';
  const knownDuration = () => item().duration_ms ? item().duration_ms! / 1000 : 0;
  let playbackSessionId: string | null = null;

  let hlsInstance: Hls | null = null;
  let hlsSessionId: string | null = null;
  let hlsHeartbeatTimer: ReturnType<typeof setInterval> | null = null;
  let seekOffset = 0;
  let hlsStartOffset = 0;
  let isSeeking = false;
  let lastProgressReport = 0;
  let isHls = false;
  let seekIndicatorTimer: ReturnType<typeof setTimeout> | null = null;
  interface SubtitleCue { start: number; end: number; text: string; }
  let subtitleCues: SubtitleCue[] = [];
  // The last confirmed media-time position (survives seeks)
  let lastConfirmedTime = 0;
  // Monotonically increasing counter to detect stale seek callbacks
  let seekGeneration = 0;
  // Debounce timer for single-click play/pause to avoid conflict with double-click
  let clickDebounceTimer: ReturnType<typeof setTimeout> | null = null;
  // Debounce rapid seek-relative presses (e.g. +10s spam) so we only spawn one
  // FFmpeg session for the final destination, not one per button press.
  let seekDebounceTimer: ReturnType<typeof setTimeout> | null = null;
  let pendingSeekTarget: number | null = null;
  let firstPlaybackStarted = false;
  let rebufferStartPerfMs: number | null = null;
  // AbortController for in-flight subtitle fetch — aborted on each new applySubtitle() call
  let subtitleFetchController: AbortController | null = null;
  // Session-expired recovery: shared across initial playback and post-seek HLS instances
  // so that the attempt counter persists across recovery cycles.
  let sessionExpiredRecoveryAttempts = 0;
  const MAX_SESSION_RECOVERY_ATTEMPTS = 3;
  // ---- Core helpers ----
  function actualTime(): number {
    if (!videoRef) return 0;
    return isHls ? hlsStartOffset + videoRef.currentTime : seekOffset + videoRef.currentTime;
  }

  function recordSeekMetric(mode: string, durationMs: number) {
    if (durationMs <= 0) return;
    api.trackPlaybackMetric({
      metric: 'seek_latency_ms',
      value_ms: durationMs,
      labels: {
        path: 'player_seek',
        mode,
        stream: isHls ? 'hls' : stream(),
      },
    });
  }

  function beginRebuffer() {
    if (!firstPlaybackStarted || isSeeking || rebufferStartPerfMs !== null) return;
    rebufferStartPerfMs = performance.now();
  }

  function endRebuffer() {
    if (rebufferStartPerfMs === null) return;
    const rebufferMs = performance.now() - rebufferStartPerfMs;
    rebufferStartPerfMs = null;
    if (rebufferMs <= 0) return;

    api.trackPlaybackMetric({
      metric: 'rebuffer_count',
      increment: 1,
      labels: { path: 'player', stream: isHls ? 'hls' : stream() },
    });
    api.trackPlaybackMetric({
      metric: 'rebuffer_ms',
      value_ms: rebufferMs,
      labels: { path: 'player', stream: isHls ? 'hls' : stream() },
    });
  }

  function progressPct(): number {
    if (isDragging()) return dragPct();
    const dur = knownDuration();
    if (dur <= 0) return 0;
    return Math.min(100, (currentTime() / dur) * 100);
  }

  function reportProgress() {
    // Don't report during active seeks — the position is unreliable
    if (isSeeking) return;
    const posMs = Math.floor(actualTime() * 1000);
    if (posMs > 0) {
      lastConfirmedTime = posMs;
      api.updateProgress(item().id, posMs);
    }
  }

  function authMasterUrl(url: string): string {
    // hls_session_start may already include ?token=... when called with auth headers.
    // Avoid adding a duplicate token query parameter.
    return url.includes('token=') ? url : authUrl(url);
  }

  // ---- Controls visibility ----
  function showControls() {
    setControlsVisible(true);
    if (controlsTimeout) clearTimeout(controlsTimeout);
    if (playing() && !showSettings() && !showAudioMenu() && !showQualityMenu() && !showSubtitleMenu()) {
      controlsTimeout = setTimeout(() => setControlsVisible(false), 3000);
    }
  }

  function hideControlsDelayed() {
    if (controlsTimeout) clearTimeout(controlsTimeout);
    if (playing() && !showSettings() && !showAudioMenu() && !showQualityMenu() && !showSubtitleMenu()) {
      controlsTimeout = setTimeout(() => setControlsVisible(false), 3000);
    }
  }

  // ---- HLS management ----
  function destroyHlsLocal() {
    if (hlsInstance) {
      hlsInstance.stopLoad();
      hlsInstance.detachMedia();
      hlsInstance.destroy();
      hlsInstance = null;
    }
  }

  /**
   * Reset the video element so a fresh HLS.js instance can attach cleanly.
   * After hls.destroy(), the video element's internal MediaSource binding is
   * revoked, but the element can still be in a state (e.g. stale src or
   * pending load) that prevents a new MediaSource from attaching. Clearing
   * the src and calling load() forces the element back to HAVE_NOTHING,
   * which is the clean state HLS.js expects for attachMedia().
   */
  function resetVideoElement() {
    videoRef.pause();
    videoRef.removeAttribute('src');
    videoRef.load();
  }

  function stopHlsHeartbeat() {
    if (hlsHeartbeatTimer) {
      clearInterval(hlsHeartbeatTimer);
      hlsHeartbeatTimer = null;
    }
  }

  function startHlsHeartbeat(mediaId: string) {
    stopHlsHeartbeat();
    if (!playbackSessionId) return;

    api.hlsSessionHeartbeat(mediaId, playbackSessionId);
    hlsHeartbeatTimer = setInterval(() => {
      if (!playbackSessionId) return;
      api.hlsSessionHeartbeat(mediaId, playbackSessionId);
    }, 15000);
  }

  async function initializeHlsLifecycle(mediaId: string, startAt: number): Promise<string> {
    const requestedStart = startAt > 0.5 ? startAt : undefined;

    try {
      const started = await api.hlsSessionStart(mediaId, requestedStart);
      playbackSessionId = started.playback_session_id;
      return started.master_url;
    } catch {
      // Fallback for older servers that don't expose explicit lifecycle endpoints yet.
      const fallbackPlaybackSessionId = createPlaybackSessionId();
      playbackSessionId = fallbackPlaybackSessionId;
      const startParam = requestedStart != null ? `&start=${requestedStart.toFixed(3)}` : '';
      return `/api/stream/${mediaId}/hls/master.m3u8?_=1${startParam}&playback_session_id=${encodeURIComponent(fallbackPlaybackSessionId)}`;
    }
  }

  function destroyHls() {
    destroyHlsLocal();
    stopHlsHeartbeat();
    const wasHls = isHls;
    isHls = false;

    const mediaId = item()?.id;
    if (!mediaId) return;

    if (playbackSessionId) {
      api.hlsSessionStop(mediaId, playbackSessionId);
      api.hlsStopMedia(mediaId, playbackSessionId);
      playbackSessionId = null;
      hlsSessionId = null;
      return;
    }

    if (hlsSessionId) {
      api.hlsStop(mediaId, hlsSessionId);
      hlsSessionId = null;
    } else if (wasHls) {
      // Native HLS may not expose session-id headers to JS.
      // Fall back to media-level cleanup endpoint.
      api.hlsStopMedia(mediaId);
    }
  }

  function handleClose() {
    // Report the best known position: take the max of lastConfirmedTime and
    // the live actualTime() reading so we never regress to an earlier position.
    if (!isSeeking) {
      const liveMs = Math.floor(actualTime() * 1000);
      const bestMs = Math.max(lastConfirmedTime, liveMs);
      if (bestMs > 0) {
        lastConfirmedTime = bestMs;
        api.updateProgress(item().id, bestMs);
      }
    } else if (lastConfirmedTime > 0) {
      api.updateProgress(item().id, lastConfirmedTime);
    }
    destroyHls();
    subtitleCues = [];
    if (videoRef) { videoRef.pause(); videoRef.removeAttribute('src'); videoRef.load(); }
    props.onClose();
  }

  // ---- Seek indicator flash ----
  function flashSeekIndicator(text: string) {
    setSeekIndicator(text);
    if (seekIndicatorTimer) clearTimeout(seekIndicatorTimer);
    seekIndicatorTimer = setTimeout(() => setSeekIndicator(null), 800);
  }

  // ---- Playback initialization ----
  onMount(async () => {
    perf.reset();
    perf.startSpan('init/total', 'frontend', { stream: stream() });

    const video = videoRef;
    const id = item().id;

    video.volume = volume() / 100;

    const prefs = loadPrefs(item().library_id);

    // Fetch server-side user preferences (language defaults) — used as fallback
    let serverPrefs: { default_subtitle_language?: string; default_audio_language?: string } = {};
    try { serverPrefs = await api.getPreferences(); } catch { /* ignore */ }

    // Fetch audio tracks for multi-audio selection
    api.getStreams(id).then(streams => {
      const audio = streams.filter(s => s.stream_type === 'audio');
      setAudioTracks(audio);
      if (prefs.audioTrackIndex != null && prefs.audioTrackIndex < audio.length) {
        setSelectedAudioTrack(prefs.audioTrackIndex);
      } else if (serverPrefs.default_audio_language) {
        const lang = serverPrefs.default_audio_language;
        const langIdx = audio.findIndex(s => s.language?.toLowerCase().startsWith(lang));
        if (langIdx >= 0) {
          setSelectedAudioTrack(langIdx);
        } else {
          const defaultIdx = audio.findIndex(s => s.is_default === 1);
          if (defaultIdx > 0) setSelectedAudioTrack(defaultIdx);
        }
      } else {
        const defaultIdx = audio.findIndex(s => s.is_default === 1);
        if (defaultIdx > 0) setSelectedAudioTrack(defaultIdx);
      }
    }).catch(() => {});

    // Fetch external subtitle tracks, then restore saved preference
    api.listSubtitles(id).then(subs => {
      setSubtitleTracks(subs);
      if (prefs.subtitleTrackId != null) {
        const match = subs.find(s => s.id === prefs.subtitleTrackId);
        if (match) {
          setSelectedSubtitle(match.id);
          setTimeout(() => applySubtitle(match.id), 500);
        }
      } else if (serverPrefs.default_subtitle_language) {
        const lang = serverPrefs.default_subtitle_language;
        const match = subs.find(s => s.language?.toLowerCase().startsWith(lang));
        if (match) {
          setSelectedSubtitle(match.id);
          setTimeout(() => applySubtitle(match.id), 500);
        }
      }
    }).catch(() => {});

    // Fetch chapter markers
    api.listChapters(id).then(ch => {
      setChapters(ch);
    }).catch(() => {});

    // Fetch next episode if this is a TV episode
    if (props.isEpisode) {
      api.nextEpisode(id).then(res => {
        setNextEpisode(res.next);
      }).catch(() => {});
    }

    let startAt: number;
    if (props.resumePosition != null) {
      startAt = props.resumePosition;
      perf.event('init/mode', 'frontend', { mode: 'resume', position: startAt });
    } else if (item().position_ms && item().position_ms! > 0 && !item().completed) {
      startAt = item().position_ms! / 1000;
      perf.event('init/mode', 'frontend', { mode: 'continue', position: startAt });
    } else {
      startAt = 0;
      perf.event('init/mode', 'frontend', { mode: 'start' });
    }

    setCurrentTime(startAt);
    lastConfirmedTime = Math.floor(startAt * 1000);

    // Track time-to-first-frame
    const onFirstPlay = () => {
      const ttffMs = perf.endSpan('init/first-frame');
      perf.endSpan('init/total');
      firstPlaybackStarted = true;
      if (ttffMs > 0) {
        api.trackPlaybackMetric({
          metric: 'playback_ttff_ms',
          value_ms: ttffMs,
          labels: { path: 'player', stream: stream() },
        });
      }
      video.removeEventListener('playing', onFirstPlay);
    };
    video.addEventListener('playing', onFirstPlay);
    perf.startSpan('init/first-frame', 'frontend');

    const useHlsJs = isTranscoded() && Hls.isSupported();
    const useNativeHls = isTranscoded() && !useHlsJs && video.canPlayType('application/vnd.apple.mpegurl') !== '';

    if (useHlsJs) {
      isHls = true;
      hlsStartOffset = startAt;
      const masterUrl = await initializeHlsLifecycle(id, startAt);
      startHlsHeartbeat(id);

      perf.startSpan('init/hls-setup', 'frontend');
      const hls = createHls();
      hlsInstance = hls;

      // Read x-hls-session-ids and x-hls-start-secs from the XHR that HLS.js
      // already performed — no redundant fetch() needed.
      hls.on(Hls.Events.MANIFEST_LOADED, (_e: any, data: any) => {
        const xhr: XMLHttpRequest | undefined = data.networkDetails;
        if (xhr) {
          const ids = xhr.getResponseHeader('x-hls-session-ids');
          if (ids) hlsSessionId = ids.split(',')[0];
          const startHdr = xhr.getResponseHeader('x-hls-requested-start') ?? xhr.getResponseHeader('x-hls-start-secs');
          if (startHdr) hlsStartOffset = parseFloat(startHdr);
        }
      });

      hls.on(Hls.Events.MANIFEST_PARSED, () => {
        perf.endSpan('init/hls-setup');
        if (hls.levels && hls.levels.length > 0) {
          // Fallback: extract session ID from variant playlist URL if
          // MANIFEST_LOADED didn't fire or networkDetails was unavailable.
          if (!hlsSessionId) {
            const lvlUrl = (hls.levels[0] as any).url;
            const match = lvlUrl?.match(/\/hls\/([^/]+)\/playlist\.m3u8/);
            if (match) hlsSessionId = match[1];
          }

          // Populate quality levels for the quality picker
          const levels = hls.levels.map((lvl: any, i: number) => ({
            index: i,
            height: lvl.height || 0,
            bitrate: lvl.bitrate || 0,
            label: lvl.height ? `${lvl.height}p` : `Level ${i + 1}`,
          }));
          // Sort by height descending so highest quality is first in menu
          levels.sort((a: any, b: any) => b.height - a.height);
          setQualityLevels(levels);

          // Restore saved quality preference
          const savedQuality = prefs.qualityHeight;
          if (savedQuality != null && savedQuality !== -1) {
            const idx = hls.levels.findIndex((lvl: any) => lvl.height === savedQuality);
            if (idx >= 0) {
              hls.currentLevel = idx;
              setSelectedQuality(savedQuality);
              setCurrentQualityLabel(`${savedQuality}p`);
            }
          }
        }
        video.play();
      });

      // Track quality level switches (from ABR auto-switching or manual selection)
      hls.on(Hls.Events.LEVEL_SWITCHED, (_e: any, data: any) => {
        const lvl = hls.levels[data.level];
        if (lvl) {
          const label = selectedQuality() === -1
            ? `Auto (${lvl.height}p)`
            : `${lvl.height}p`;
          setCurrentQualityLabel(label);
        }
      });

      hls.on(Hls.Events.ERROR, (_event: any, data: any) => {
        if (hlsInstance !== hls) return;

        // Detect session-expired: 404 on a fragment or playlist load means the
        // backend cleaned up the HLS session while we were paused. Restart the
        // session at the current playback position.
        const is404 = data.response?.code === 404;
        const isFragOrPlaylist =
          data.details === Hls.ErrorDetails.FRAG_LOAD_ERROR ||
          data.details === Hls.ErrorDetails.LEVEL_LOAD_ERROR ||
          data.details === Hls.ErrorDetails.MANIFEST_LOAD_ERROR;
        if (is404 && isFragOrPlaylist && !isSeeking && sessionExpiredRecoveryAttempts < MAX_SESSION_RECOVERY_ATTEMPTS) {
          sessionExpiredRecoveryAttempts++;
          // Use the display time signal, not actualTime() — when the buffer is
          // exhausted, video.currentTime stalls at the buffer edge which may be
          // earlier than where the user paused. currentTime() reflects the last
          // known good position from onTimeUpdate.
          const resumeAt = currentTime();
          console.warn(`[hls] Session expired (404), recovery attempt ${sessionExpiredRecoveryAttempts}/${MAX_SESSION_RECOVERY_ATTEMPTS} at position:`, resumeAt);
          hlsSeekTo(resumeAt).then(() => {
            sessionExpiredRecoveryAttempts = 0;
          }).catch(() => {});
          return;
        }

        if (data.fatal) {
          perf.event('init/hls-error', 'frontend', { type: data.type, details: data.details });
          console.error('HLS fatal error:', data.type, data.details);
          hls.destroy();
          hlsInstance = null;
          isHls = false;
          seekOffset = startAt > 0.5 ? startAt : 0;
          video.src = authUrl(startAt > 0.5 ? `/api/stream/${id}?start=${startAt.toFixed(3)}` : `/api/stream/${id}`);
          video.play();
        }
      });

      hls.loadSource(authMasterUrl(masterUrl));
      hls.attachMedia(video);
    } else if (useNativeHls) {
      isHls = true;
      hlsStartOffset = startAt;
      const masterUrl = await initializeHlsLifecycle(id, startAt);
      startHlsHeartbeat(id);
      video.src = authMasterUrl(masterUrl);
      video.addEventListener('loadedmetadata', function onMeta() {
        if (startAt > 1) video.currentTime = startAt;
        video.removeEventListener('loadedmetadata', onMeta);
      });
      video.play();
    } else if (isTranscoded() && startAt > 1) {
      // For transcoded non-HLS, the backend snaps to the nearest keyframe.
      // Query the keyframe endpoint first (lightweight ffprobe call) to get the
      // actual position, then start the stream from that exact keyframe.
      seekOffset = startAt;
      (async () => {
        perf.startSpan('init/keyframe-lookup', 'network');
        try {
          const kfRes = await fetch(
            authUrl(`/api/stream/${id}/keyframe?time=${startAt.toFixed(3)}`),
          );
          if (kfRes.ok) {
            const d = await kfRes.json();
            perf.endSpan('init/keyframe-lookup');
            if (d.timing_ms) perf.ingestBackendTiming('init/keyframe', d.timing_ms);
            const kf = d.keyframe as number;
            if (kf > 0) {
              seekOffset = kf;
              setCurrentTime(kf);
              lastConfirmedTime = Math.floor(kf * 1000);
            }
          } else {
            perf.endSpan('init/keyframe-lookup');
          }
        } catch {
          perf.endSpan('init/keyframe-lookup');
        }
        perf.startSpan('init/stream-load', 'network');
        const audioParam = selectedAudioTrack() > 0 ? `&audio_stream=${selectedAudioTrack()}` : '';
        video.src = authUrl(`/api/stream/${id}?start=${seekOffset.toFixed(3)}${audioParam}`);
        video.addEventListener('canplay', function onCan() {
          perf.endSpan('init/stream-load');
          video.removeEventListener('canplay', onCan);
        }, { once: true });
        video.play();
      })();
    } else {
      perf.startSpan('init/stream-load', 'network');
      const audioParam = selectedAudioTrack() > 0 ? `?audio_stream=${selectedAudioTrack()}` : '';
      video.src = authUrl(`/api/stream/${id}${audioParam}`);
      video.addEventListener('canplay', function onCan() {
        perf.endSpan('init/stream-load');
        video.removeEventListener('canplay', onCan);
      }, { once: true });
      if (!isTranscoded() && startAt > 1) {
        video.addEventListener('loadedmetadata', function onMeta() {
          video.currentTime = startAt;
          video.removeEventListener('loadedmetadata', onMeta);
        });
      }
      video.play();
    }

    hideControlsDelayed();
  });

  // ---- Video event handlers ----
  function onTimeUpdate() {
    if (isSeeking) return;
    const t = actualTime();
    const dur = knownDuration();
    setCurrentTime(dur > 0 ? Math.min(t, dur) : t);
    setPlaying(!videoRef.paused);

    // Custom subtitle renderer: binary-search for the cue whose [start, end]
    // brackets actualTime(). Cues are sorted by start time after parsing.
    if (subtitleCues.length > 0) {
      let lo = 0, hi = subtitleCues.length - 1, found: string | null = null;
      while (lo <= hi) {
        const mid = (lo + hi) >>> 1;
        const c = subtitleCues[mid];
        if (t < c.start) {
          hi = mid - 1;
        } else if (t >= c.end) {
          lo = mid + 1;
        } else {
          found = c.text;
          break;
        }
      }
      setActiveCue(found);
    }

    const now = Date.now();
    if (now - lastProgressReport > 30000) {
      lastProgressReport = now;
      reportProgress();
    }

    // Show Up Next overlay in the final 30 seconds
    if (dur > 0 && nextEpisode() && props.onNextEpisode && !upNextStarted && !upNextNavigating) {
      const remaining = dur - t;
      if (remaining > 0 && remaining <= 30) {
        startUpNextCountdown();
      }
    }
  }

  function onProgress() {
    if (!knownDuration() || !videoRef.buffered.length) return;
    const buffEnd = videoRef.buffered.end(videoRef.buffered.length - 1);
    const total = isHls ? hlsStartOffset + buffEnd : seekOffset + buffEnd;
    setBufferedPct(Math.min(100, (total / knownDuration()) * 100));
  }

  function startUpNextCountdown() {
    if (!nextEpisode() || !props.onNextEpisode) return;
    if (upNextStarted) return;
    upNextStarted = true;
    if (upNextTimer) clearInterval(upNextTimer);
    let remaining = 15;
    setUpNextCountdown(remaining);
    setUpNextVisible(true);
    upNextTimer = setInterval(() => {
      remaining -= 1;
      setUpNextCountdown(remaining);
      if (remaining <= 0) {
        clearInterval(upNextTimer!);
        upNextTimer = null;
        setTimeout(() => playNextEpisode(), 0);
      }
    }, 1000);
  }

  function cancelUpNext() {
    if (upNextTimer) { clearInterval(upNextTimer); upNextTimer = null; }
    upNextStarted = false;
    upNextNavigating = false;
    setUpNextVisible(false);
  }

  function playNextEpisode() {
    if (upNextNavigating) return;
    upNextNavigating = true;
    if (upNextTimer) { clearInterval(upNextTimer); upNextTimer = null; }
    setUpNextVisible(false);
    const next = nextEpisode();
    if (next && props.onNextEpisode) {
      api.markCompleted(item().id);
      props.onNextEpisode(next.media_item_id);
    }
  }

  function onEnded() {
    if (nextEpisode() && props.onNextEpisode) {
      startUpNextCountdown();
    } else {
      api.markCompleted(item().id);
    }
  }
  function onPlay() {
    firstPlaybackStarted = true;
    setPlaying(true);
    setBuffering(false);
    hideControlsDelayed();
  }
  function onPause() {
    endRebuffer();
    setPlaying(false);
    showControls();
  }
  function onWaiting() {
    setBuffering(true);
    beginRebuffer();
  }
  function onCanPlay() {
    setBuffering(false);
    endRebuffer();
  }

  // ---- Controls ----
  function togglePlay() {
    if (videoRef.paused) videoRef.play(); else videoRef.pause();
  }

  function toggleFullscreen() {
    if (document.fullscreenElement) {
      document.exitFullscreen();
    } else {
      playerRef?.requestFullscreen().catch(() => {});
    }
  }

  function onFullscreenChange() {
    setIsFullscreen(!!document.fullscreenElement);
  }

  function handleVolumeChange(val: number) {
    setVolume(val);
    videoRef.volume = val / 100;
    localStorage.setItem('ferrite-volume', String(val));
    if (val > 0) setLastVolume(val);
  }

  function toggleMute() {
    if (volume() > 0) {
      setLastVolume(volume());
      handleVolumeChange(0);
    } else {
      handleVolumeChange(lastVolume() || 50);
    }
  }

  function changeSpeed(speed: number) {
    setPlaybackSpeed(speed);
    videoRef.playbackRate = speed;
    setShowSettings(false);
  }

  /**
   * Parse a VTT timestamp string (hh:mm:ss.mmm or mm:ss.mmm) into seconds.
   */
  function vttTimeToSecs(ts: string): number {
    const parts = ts.trim().split(':');
    if (parts.length === 3) {
      return parseFloat(parts[0]) * 3600 + parseFloat(parts[1]) * 60 + parseFloat(parts[2]);
    } else if (parts.length === 2) {
      return parseFloat(parts[0]) * 60 + parseFloat(parts[1]);
    }
    return 0;
  }

  /**
   * Parse a VTT string into an array of cue objects with absolute timestamps.
   * No shifting — cues retain their original times so we can compare against actualTime().
   */
  function parseVttCues(vtt: string): SubtitleCue[] {
    const cues: SubtitleCue[] = [];
    const blocks = vtt.split(/\n\n+/);
    const timingRe = /^(\d{1,2}:\d{2}:\d{2}\.\d{1,3}|\d{2}:\d{2}\.\d{1,3})\s+-->\s+(\d{1,2}:\d{2}:\d{2}\.\d{1,3}|\d{2}:\d{2}\.\d{1,3})/;
    for (const block of blocks) {
      const lines = block.trim().split('\n');
      for (let i = 0; i < lines.length; i++) {
        const m = lines[i].match(timingRe);
        if (m) {
          const text = lines.slice(i + 1).join('\n').trim();
          if (text) cues.push({ start: vttTimeToSecs(m[1]), end: vttTimeToSecs(m[2]), text });
          break;
        }
      }
    }
    cues.sort((a, b) => a.start - b.start);
    return cues;
  }

  /**
   * Fetch a subtitle track and parse its cues into subtitleCues[].
   * Active cue selection is driven by onTimeUpdate() using actualTime(),
   * so there is no native <track> element and no browser cue accumulation.
   */
  async function applySubtitle(subtitleId: number) {
    // Abort any in-flight subtitle fetch from a previous call (rapid track changes)
    if (subtitleFetchController) {
      subtitleFetchController.abort();
    }
    subtitleFetchController = new AbortController();
    const signal = subtitleFetchController.signal;

    subtitleCues = [];
    setActiveCue(null);
    try {
      const res = await fetch(authUrl(`/api/subtitles/${subtitleId}/serve`), { signal });
      if (!res.ok) return;
      const vttText = await res.text();
      subtitleCues = parseVttCues(vttText);
    } catch (e: any) {
      if (e?.name === 'AbortError') return;
      // Subtitle fetch failed — silently ignore
    }
  }

  function changeSubtitle(subtitleId: number | null) {
    setSelectedSubtitle(subtitleId);
    setShowSubtitleMenu(false);
    savePrefs(item().library_id, { subtitleTrackId: subtitleId });
    if (subtitleId === null) {
      subtitleCues = [];
      setActiveCue(null);
      return;
    }
    applySubtitle(subtitleId);
  }

  function changeAudioTrack(trackIndex: number) {
    if (trackIndex === selectedAudioTrack()) { setShowAudioMenu(false); return; }
    setSelectedAudioTrack(trackIndex);
    setShowAudioMenu(false);
    savePrefs(item().library_id, { audioTrackIndex: trackIndex });
    const pos = actualTime();
    if (isHls) {
      // For HLS: trigger a seek at the current position — the new audio_stream
      // index is picked up by the backend when it creates the new HLS session.
      hlsSeekTo(pos);
    } else if (isTranscoded()) {
      // For progressive transcode: reload the stream URL with the new audio_stream
      // param at the current playback position.
      seekOffset = pos;
      setCurrentTime(pos);
      const audioParam = trackIndex > 0 ? `&audio_stream=${trackIndex}` : '';
      videoRef.src = authUrl(`/api/stream/${item().id}?start=${pos.toFixed(3)}${audioParam}`);
      videoRef.play();
    }
  }

  function changeQuality(levelHeight: number) {
    // levelHeight: -1 for auto, or the height (e.g. 1080, 720) for a specific level
    setSelectedQuality(levelHeight);
    setShowQualityMenu(false);
    savePrefs(item().library_id, { qualityHeight: levelHeight });
    if (!hlsInstance) return;

    if (levelHeight === -1) {
      // Auto mode — let HLS.js ABR decide
      hlsInstance.currentLevel = -1;
      setCurrentQualityLabel('Auto');
    } else {
      // Find the HLS.js level index matching this height
      const idx = hlsInstance.levels.findIndex((lvl: any) => lvl.height === levelHeight);
      if (idx >= 0) {
        hlsInstance.currentLevel = idx;
        setCurrentQualityLabel(`${levelHeight}p`);
      }
    }
  }

  function togglePiP() {
    if (document.pictureInPictureElement) {
      document.exitPictureInPicture();
    } else {
      videoRef.requestPictureInPicture?.().catch(() => {});
    }
  }

  // ---- Seeking ----
  async function hlsSeekTo(targetTime: number) {
    // Cancel Up Next if seeking backward out of the final 30s window
    const dur = knownDuration();
    if (upNextStarted && dur > 0 && (dur - targetTime) > 30) {
      cancelUpNext();
    }

    // Bump generation so any in-flight seek callbacks become stale
    const gen = ++seekGeneration;

    perf.startSpan('seek/hls-total', 'frontend', { target: Math.round(targetTime) });
    isSeeking = true;
    setBuffering(true);
    setCurrentTime(targetTime);

    // Stop the old HLS instance from fetching segments immediately — before
    // the seek API call. The backend will destroy the old FFmpeg session, so
    // any requests from the old instance would 404.
    if (hlsInstance) {
      hlsInstance.stopLoad();
    }

    try {
      perf.startSpan('seek/hls-api', 'network');
      const audioIdx = selectedAudioTrack();
      console.log('[seek] calling hlsSeek API, target:', targetTime);
      const seekRes = await api.hlsSeek(
        item().id,
        targetTime,
        audioIdx > 0 ? audioIdx : undefined,
        playbackSessionId ?? undefined,
      );

      // If another seek was initiated while we were waiting, abandon this one
      if (gen !== seekGeneration) return;

      console.log('[seek] API returned:', JSON.stringify(seekRes));
      perf.endSpan('seek/hls-api');
      if (seekRes.timing_ms) perf.ingestBackendTiming('seek/hls', seekRes.timing_ms);

      // Fast path: backend confirmed the target is within the already-buffered range.
      // Skip destroying/recreating the HLS instance — just seek within the existing stream.
      if (seekRes.reused && hlsInstance && hlsSessionId === seekRes.session_id) {
        const seekPos = targetTime - hlsStartOffset;
        console.log('[seek] reusing session, seeking to videoRef.currentTime =', seekPos);
        videoRef.currentTime = seekPos;
        videoRef.play().catch(() => {});
        setTimeout(() => {
          if (gen !== seekGeneration) return;
          isSeeking = false;
          setBuffering(false);
          const seekMs = perf.endSpan('seek/hls-total');
          recordSeekMetric('hls-reused', seekMs);
          lastConfirmedTime = Math.floor(targetTime * 1000);
        }, 100);
        return;
      }

      // Slow path: new session — destroy the old HLS instance now that we know
      // the backend has already destroyed the old FFmpeg session.
      destroyHlsLocal();
      resetVideoElement();
      hlsSessionId = null;

      // The server returns the actual start_secs (which is the time FFmpeg
      // was told to seek to). HLS.js normalizes currentTime to 0 relative to
      // the playlist start, so we add this offset to get absolute media time.
      // However, if the video is copied, fmp4 segments retain their original PTS
      // and videoRef.currentTime is ALREADY absolute. In this case, we set
      // hlsStartOffset to 0 so we don't double-count the time offset.
      hlsStartOffset = seekRes.video_copied ? 0 : (seekRes.requested_start ?? seekRes.start_secs ?? targetTime);
      hlsSessionId = seekRes.session_id;
      isHls = true;

      const activeSub = selectedSubtitle();

      // The master_url from the backend already contains the auth token
      // as a query param — do NOT wrap in authUrl() or it will be doubled,
      // causing a 400 Bad Request.
      const sourceUrl = seekRes.master_url;
      console.log('[seek] creating new HLS instance, source:', sourceUrl);

      perf.startSpan('seek/hls-manifest', 'network');
      const hls = createHls();
      hlsInstance = hls;

      hls.on(Hls.Events.MEDIA_ATTACHED, () => {
        console.log('[seek] MEDIA_ATTACHED fired');
      });

      hls.on(Hls.Events.MANIFEST_LOADING, () => {
        console.log('[seek] MANIFEST_LOADING fired');
      });

      hls.on(Hls.Events.MANIFEST_LOADED, (_e: any, data: any) => {
        console.log('[seek] MANIFEST_LOADED fired');

        const xhr: XMLHttpRequest | undefined = data.networkDetails;
        if (xhr) {
          const ids = xhr.getResponseHeader('x-hls-session-ids');
          if (ids) hlsSessionId = ids.split(',')[0];

          const startHdr = xhr.getResponseHeader('x-hls-requested-start') ?? xhr.getResponseHeader('x-hls-start-secs');
          if (startHdr) hlsStartOffset = parseFloat(startHdr);
        }
      });

      hls.on(Hls.Events.MANIFEST_PARSED, (_e: any, data: any) => {
        // Stale seek — a newer one has taken over
        if (gen !== seekGeneration) { hls.destroy(); return; }
        console.log('[seek] MANIFEST_PARSED fired, levels:', data.levels?.length);
        perf.endSpan('seek/hls-manifest');

        // Re-apply subtitle now that HLS.js has finished resetting the video element.
        // Doing this earlier (before attachMedia) causes stale cues to flash because
        // HLS.js calls videoRef.load() internally during attachment.
        if (activeSub !== null) applySubtitle(activeSub, hlsStartOffset);

        // Re-populate quality levels and re-apply user's quality preference
        if (hls.levels && hls.levels.length > 0) {
          const levels = hls.levels.map((lvl: any, i: number) => ({
            index: i,
            height: lvl.height || 0,
            bitrate: lvl.bitrate || 0,
            label: lvl.height ? `${lvl.height}p` : `Level ${i + 1}`,
          }));
          levels.sort((a: any, b: any) => b.height - a.height);
          setQualityLevels(levels);

          // Re-apply selected quality after seek
          const userQuality = selectedQuality();
          if (userQuality !== -1) {
            // Find matching height in new levels
            const match = hls.levels.findIndex((lvl: any) => lvl.height === userQuality);
            if (match >= 0) {
              hls.currentLevel = match;
            }
          }
        }

        videoRef.play().then(() => {
          console.log('[seek] play() resolved');
        }).catch((err: any) => {
          console.error('[seek] play() rejected:', err);
        });
        setTimeout(() => {
          if (gen !== seekGeneration) return;
          isSeeking = false;
          setBuffering(false);
          const seekMs = perf.endSpan('seek/hls-total');
          recordSeekMetric('hls-new', seekMs);
          // Use targetTime (the user's requested position), not hlsStartOffset
          // (the keyframe-snapped start). The keyframe can be 10-12s earlier,
          // which would cause the resume position to regress on close.
          lastConfirmedTime = Math.floor(targetTime * 1000);
        }, 200);
      });

      // Track quality level switches after seek
      hls.on(Hls.Events.LEVEL_SWITCHED, (_e: any, data: any) => {
        if (gen !== seekGeneration) return;
        const lvl = hls.levels[data.level];
        if (lvl) {
          const label = selectedQuality() === -1
            ? `Auto (${lvl.height}p)`
            : `${lvl.height}p`;
          setCurrentQualityLabel(label);
        }
      });

      hls.on(Hls.Events.ERROR, (_e: any, d: any) => {
        if (gen !== seekGeneration) return;
        // Non-fatal bufferStalledError is a known false positive: HLS.js starts
        // its stall detector on MEDIA_ATTACHED before play() resolves, sees
        // currentTime=0 with buffer available, and reports a stall. Ignore it.
        if (!d.fatal && d.details === 'bufferStalledError') return;
        console.error('[seek] HLS error:', d.type, d.details, 'fatal:', d.fatal, d);

        // Detect session-expired 404 on the post-seek HLS instance — same
        // recovery logic as the initial playback error handler.
        const is404 = d.response?.code === 404;
        const isFragOrPlaylist =
          d.details === Hls.ErrorDetails.FRAG_LOAD_ERROR ||
          d.details === Hls.ErrorDetails.LEVEL_LOAD_ERROR ||
          d.details === Hls.ErrorDetails.MANIFEST_LOAD_ERROR;
        if (is404 && isFragOrPlaylist && !isSeeking && sessionExpiredRecoveryAttempts < MAX_SESSION_RECOVERY_ATTEMPTS) {
          sessionExpiredRecoveryAttempts++;
          const resumeAt = currentTime();
          console.warn(`[seek] Session expired (404), recovery attempt ${sessionExpiredRecoveryAttempts}/${MAX_SESSION_RECOVERY_ATTEMPTS} at position:`, resumeAt);
          hlsSeekTo(resumeAt).then(() => {
            sessionExpiredRecoveryAttempts = 0;
          }).catch(() => {});
          return;
        }

        if (d.fatal) {
          perf.event('seek/hls-error', 'frontend', { type: d.type, details: d.details });
          isSeeking = false;
          setBuffering(false);
          perf.endSpan('seek/hls-total');
        }
      });

      // loadSource first, then attachMedia — matches the working initial-playback
      // order. HLS.js queues the source URL and starts fetching once MEDIA_ATTACHED
      // fires internally after attachMedia.
      console.log('[seek] calling loadSource + attachMedia');
      hls.loadSource(sourceUrl);
      hls.attachMedia(videoRef);
      console.log('[seek] loadSource + attachMedia called');
    } catch (e) {
      if (gen !== seekGeneration) return;
      console.error('HLS seek failed:', e);
      perf.event('seek/hls-failed', 'frontend');
      perf.endSpan('seek/hls-api');
      perf.endSpan('seek/hls-total');
      isSeeking = false;
      setBuffering(false);
    }
  }

  async function seekToTime(targetTime: number) {
    if (isHls) {
      await hlsSeekTo(targetTime);
    } else if (isTranscoded()) {
      // Cancel Up Next if seeking backward out of the final 30s window
      const dur = knownDuration();
      if (upNextStarted && dur > 0 && (dur - targetTime) > 30) {
        cancelUpNext();
      }
      perf.startSpan('seek/transcode-total', 'frontend', { target: Math.round(targetTime) });
      isSeeking = true;
      setBuffering(true);
      videoRef.pause();
      setCurrentTime(targetTime);

      // Query the keyframe endpoint to get the actual keyframe-snapped position,
      // then start the stream from that exact keyframe for accurate seeking.
      perf.startSpan('seek/keyframe-lookup', 'network');
      let actualStart = targetTime;
      try {
        const res = await fetch(
          authUrl(`/api/stream/${item().id}/keyframe?time=${targetTime.toFixed(3)}`),
        );
        if (res.ok) {
          const d = await res.json();
          perf.endSpan('seek/keyframe-lookup');
          if (d.timing_ms) perf.ingestBackendTiming('seek/keyframe', d.timing_ms);
          actualStart = d.keyframe ?? targetTime;
        } else {
          perf.endSpan('seek/keyframe-lookup');
        }
      } catch {
        perf.endSpan('seek/keyframe-lookup');
      }

      seekOffset = actualStart;
      setCurrentTime(targetTime);
      lastConfirmedTime = Math.floor(targetTime * 1000);

      // Re-apply subtitle with updated offset so cues stay in sync
      const activeSub = selectedSubtitle();
      if (activeSub !== null) applySubtitle(activeSub, seekOffset);

      perf.startSpan('seek/stream-load', 'network');
      const audioParam = selectedAudioTrack() > 0 ? `&audio_stream=${selectedAudioTrack()}` : '';
      videoRef.src = authUrl(`/api/stream/${item().id}?start=${actualStart.toFixed(3)}${audioParam}`);
      videoRef.addEventListener('canplay', function onCan() {
        perf.endSpan('seek/stream-load');
        const seekMs = perf.endSpan('seek/transcode-total');
        recordSeekMetric('transcode', seekMs);
        videoRef.removeEventListener('canplay', onCan);
      }, { once: true });
      videoRef.play();
      setTimeout(() => { isSeeking = false; }, 500);
    } else {
      perf.startSpan('seek/direct', 'frontend', { target: Math.round(targetTime) });
      videoRef.currentTime = targetTime;
      videoRef.addEventListener('seeked', function onSeeked() {
        const seekMs = perf.endSpan('seek/direct');
        recordSeekMetric('direct', seekMs);
        videoRef.removeEventListener('seeked', onSeeked);
      }, { once: true });
    }
  }

  function seekRelative(deltaSec: number) {
    if (!knownDuration()) return;
    // Use the signal-based currentTime() instead of actualTime() because during
    // an active seek, videoRef.currentTime is stale (from the destroyed session).
    // currentTime() is always updated immediately when a seek starts.
    const base = pendingSeekTarget ?? (isSeeking ? currentTime() : actualTime());
    const target = Math.max(0, Math.min(knownDuration(), base + deltaSec));
    flashSeekIndicator(deltaSec > 0 ? `+${deltaSec}s` : `${deltaSec}s`);

    if (isHls) {
      // For HLS, debounce: update display immediately but only fire the
      // expensive FFmpeg seek after the user stops pressing for 400ms.
      pendingSeekTarget = target;
      setCurrentTime(target);
      if (seekDebounceTimer !== null) clearTimeout(seekDebounceTimer);
      seekDebounceTimer = setTimeout(() => {
        seekDebounceTimer = null;
        const finalTarget = pendingSeekTarget!;
        pendingSeekTarget = null;
        seekToTime(finalTarget);
      }, 400);
    } else {
      seekToTime(target);
    }
  }

  // ---- Timeline interaction ----
  function getTimelineTime(clientX: number): number {
    if (!timelineRef || !knownDuration()) return 0;
    const rect = timelineRef.getBoundingClientRect();
    const fraction = Math.max(0, Math.min(1, (clientX - rect.left) / rect.width));
    return fraction * knownDuration();
  }

  function onTimelineHover(e: MouseEvent) {
    const t = getTimelineTime(e.clientX);
    setHoverTime(t);
    setHoverX(e.clientX);
  }

  function onTimelineLeave() {
    if (!isDragging()) setHoverTime(null);
  }

  function onTimelineDown(e: MouseEvent) {
    e.preventDefault();
    setIsDragging(true);
    const t = getTimelineTime(e.clientX);
    setDragPct((t / knownDuration()) * 100);
    setCurrentTime(t);

    const onMove = (ev: MouseEvent) => {
      const mt = getTimelineTime(ev.clientX);
      setDragPct((mt / knownDuration()) * 100);
      setCurrentTime(mt);
      setHoverTime(mt);
      setHoverX(ev.clientX);
    };

    const onUp = async (ev: MouseEvent) => {
      document.removeEventListener('mousemove', onMove);
      document.removeEventListener('mouseup', onUp);
      setIsDragging(false);
      setHoverTime(null);
      const finalTime = getTimelineTime(ev.clientX);
      await seekToTime(finalTime);
    };

    document.addEventListener('mousemove', onMove);
    document.addEventListener('mouseup', onUp);
  }

  // ---- Keyboard shortcuts ----
  function handleKeyDown(e: KeyboardEvent) {
    const isInput = ['INPUT', 'SELECT', 'TEXTAREA'].includes((document.activeElement?.tagName || ''));
    if (isInput) return;

    showControls();

    switch (e.key) {
      case 'Escape':
        if (showQualityMenu()) { setShowQualityMenu(false); return; }
        if (showSubtitleMenu()) { setShowSubtitleMenu(false); return; }
        if (showAudioMenu()) { setShowAudioMenu(false); return; }
        if (showSettings()) { setShowSettings(false); return; }
        if (isFullscreen()) { document.exitFullscreen(); return; }
        handleClose();
        return;
      case ' ':
      case 'k':
      case 'K':
        e.preventDefault();
        togglePlay();
        return;
      case 'ArrowLeft':
        e.preventDefault();
        seekRelative(-10);
        return;
      case 'ArrowRight':
        e.preventDefault();
        seekRelative(10);
        return;
      case 'j':
      case 'J':
        e.preventDefault();
        seekRelative(-10);
        return;
      case 'l':
      case 'L':
        e.preventDefault();
        seekRelative(10);
        return;
      case 'ArrowUp':
        e.preventDefault();
        handleVolumeChange(Math.min(100, volume() + 5));
        return;
      case 'ArrowDown':
        e.preventDefault();
        handleVolumeChange(Math.max(0, volume() - 5));
        return;
      case 'm':
      case 'M':
        toggleMute();
        return;
      case 'f':
      case 'F':
        toggleFullscreen();
        return;
      case '<':
        changeSpeed(Math.max(0.25, playbackSpeed() - 0.25));
        return;
      case '>':
        changeSpeed(Math.min(3, playbackSpeed() + 0.25));
        return;
      case 'p':
      case 'P':
        setShowPerf(!showPerf());
        return;
    }
  }

  function handleDoubleClick(e: MouseEvent) {
    const rect = playerRef.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const third = rect.width / 3;

    if (x < third) {
      seekRelative(-10);
    } else if (x > third * 2) {
      seekRelative(10);
    } else {
      toggleFullscreen();
    }
  }

  // ---- Lifecycle ----
  onMount(() => {
    document.addEventListener('keydown', handleKeyDown);
    document.addEventListener('fullscreenchange', onFullscreenChange);
  });

  onCleanup(() => {
    document.removeEventListener('keydown', handleKeyDown);
    document.removeEventListener('fullscreenchange', onFullscreenChange);
    if (controlsTimeout) clearTimeout(controlsTimeout);
    if (seekIndicatorTimer) clearTimeout(seekIndicatorTimer);
    if (clickDebounceTimer) { clearTimeout(clickDebounceTimer); clickDebounceTimer = null; }
    if (upNextTimer) { clearInterval(upNextTimer); upNextTimer = null; }
    stopHlsHeartbeat();
    if (subtitleFetchController) { subtitleFetchController.abort(); subtitleFetchController = null; }
    rebufferStartPerfMs = null;
    subtitleCues = [];
    destroyHls();
  });

  // ---- Volume icon ----
  const VolumeIcon = () => {
    const v = volume();
    if (v === 0) return <VolumeX class="w-5 h-5" />;
    if (v < 50) return <Volume1 class="w-5 h-5" />;
    return <Volume2 class="w-5 h-5" />;
  };

  // ---- Render ----
  return (
    <div
      ref={playerRef!}
      class="fixed inset-0 bg-black z-[100] select-none"
      onMouseMove={showControls}
      onClick={() => { if (showSettings()) setShowSettings(false); if (showAudioMenu()) setShowAudioMenu(false); if (showQualityMenu()) setShowQualityMenu(false); if (showSubtitleMenu()) setShowSubtitleMenu(false); }}
      style={{ cursor: controlsVisible() ? 'default' : 'none' }}
    >
      {/* Video element — fills entire viewport */}
      <video
        ref={videoRef!}
        class="absolute inset-0 w-full h-full object-contain"
        autoplay
        onTimeUpdate={onTimeUpdate}
        onProgress={onProgress}
        onEnded={onEnded}
        onPlay={onPlay}
        onPause={onPause}
        onWaiting={onWaiting}
        onCanPlay={onCanPlay}
        onClick={(e) => {
          e.stopPropagation();
          showControls();
          // Debounce: wait 250ms before toggling play/pause so a double-click
          // can cancel it before it fires, preventing play→pause→play flicker.
          if (clickDebounceTimer) clearTimeout(clickDebounceTimer);
          clickDebounceTimer = setTimeout(() => {
            clickDebounceTimer = null;
            togglePlay();
          }, 250);
        }}
        onDblClick={(e) => {
          e.stopPropagation();
          // Cancel the pending single-click play/pause toggle
          if (clickDebounceTimer) { clearTimeout(clickDebounceTimer); clickDebounceTimer = null; }
          handleDoubleClick(e);
        }}
      />

      {/* Performance overlay (toggle with P key) */}
      <PerfOverlay visible={showPerf()} />

      {/* Buffering spinner */}
      <Show when={buffering() && !isDragging()}>
        <div class="absolute inset-0 flex items-center justify-center pointer-events-none z-10">
          <Loader2 class="w-12 h-12 text-white/80 animate-spin" />
        </div>
      </Show>

      {/* Custom subtitle overlay — driven by actualTime() in onTimeUpdate, no native <track> */}
      <Show when={activeCue()}>
        <div class="absolute bottom-[10%] left-0 right-0 flex justify-center pointer-events-none z-20 px-12">
          <div
            class="text-white text-2xl font-medium text-center leading-snug px-3 py-1 rounded"
            style={{ "background": "rgba(0,0,0,0.55)", "text-shadow": "0 1px 3px rgba(0,0,0,0.8)" }}
            innerHTML={activeCue()!.replace(/\n/g, '<br>')}
          />
        </div>
      </Show>

      {/* Seek indicator (e.g. "+10s") */}
      <Show when={seekIndicator()}>
        <div class="absolute inset-0 flex items-center justify-center pointer-events-none z-10">
          <div class="px-6 py-3 rounded-2xl bg-black/60 backdrop-blur-sm text-white text-2xl font-bold animate-fade-in">
            {seekIndicator()}
          </div>
        </div>
      </Show>

      {/* Center play button when paused */}
      <Show when={!playing() && !buffering()}>
        <div
          class="absolute inset-0 flex items-center justify-center pointer-events-none z-10"
        >
          <div class="w-20 h-20 rounded-full bg-black/40 backdrop-blur-sm flex items-center justify-center">
            <Play class="w-10 h-10 text-white fill-white ml-1" />
          </div>
        </div>
      </Show>

      {/* Up Next overlay — shown in final 30 seconds */}
      <Show when={upNextVisible() && nextEpisode()}>
        <div
          class="absolute bottom-28 right-6 z-30 w-80 rounded-2xl bg-surface-100/95 backdrop-blur-xl border border-white/10 shadow-2xl shadow-black/60 overflow-hidden animate-scale-in"
          onClick={(e) => e.stopPropagation()}
        >
          {/* Countdown progress bar */}
          <div class="h-1 bg-surface-300">
            <div
              class="h-full bg-ferrite-500 transition-all duration-1000 ease-linear"
              style={{ width: `${(upNextCountdown() / 15) * 100}%` }}
            />
          </div>
          <div class="p-4">
            <div class="flex items-start gap-3">
              {/* Thumbnail */}
              <Show when={nextEpisode()!.still_path}>
                <img
                  src={authUrl(`/api/images/${nextEpisode()!.still_path}`)}
                  alt=""
                  class="w-24 h-14 object-cover rounded-lg flex-shrink-0 bg-surface-300"
                />
              </Show>
              <Show when={!nextEpisode()!.still_path && nextEpisode()!.show_poster_path}>
                <img
                  src={authUrl(`/api/images/${nextEpisode()!.show_poster_path}`)}
                  alt=""
                  class="w-24 h-14 object-cover rounded-lg flex-shrink-0 bg-surface-300"
                />
              </Show>
              <div class="flex-1 min-w-0">
                <p class="text-2xs text-surface-700 uppercase tracking-wider font-medium mb-0.5">Up Next</p>
                <p class="text-sm font-semibold text-white leading-tight truncate">
                  {nextEpisode()!.show_title}
                </p>
                <p class="text-xs text-surface-800 truncate">
                  S{String(nextEpisode()!.season_number).padStart(2, '0')}E{String(nextEpisode()!.episode_number).padStart(2, '0')}
                  <Show when={nextEpisode()!.episode_title}>
                    {' · '}{nextEpisode()!.episode_title}
                  </Show>
                </p>
              </div>
            </div>
            <div class="flex items-center gap-2 mt-3">
              <button
                class="flex-1 btn-primary py-2 text-sm justify-center"
                onClick={() => playNextEpisode()}
              >
                <Play class="w-4 h-4 fill-current" /> Play Now
              </button>
              <button
                class="btn-secondary py-2 text-sm"
                onClick={() => cancelUpNext()}
              >
                Cancel ({upNextCountdown()})
              </button>
            </div>
          </div>
        </div>
      </Show>

      {/* Top gradient + title bar */}
      <div
        class={`absolute top-0 left-0 right-0 z-20 transition-opacity duration-300
                ${controlsVisible() ? 'opacity-100' : 'opacity-0 pointer-events-none'}`}
      >
        <div class="bg-gradient-to-b from-black/80 via-black/40 to-transparent px-6 pt-5 pb-16">
          <div class="flex items-center gap-4">
            <button class="btn-icon text-white hover:bg-white/10" onClick={handleClose}>
              <ArrowLeft class="w-5 h-5" />
            </button>
            <div class="flex-1 min-w-0">
              <Show when={item().is_episode && item().season_number != null} fallback={
                <h2 class="text-white font-semibold text-lg truncate drop-shadow-lg">
                  {getDisplayTitle(item())}
                </h2>
              }>
                <h2 class="text-white font-semibold text-lg truncate drop-shadow-lg">
                  {item().show_title || getDisplayTitle(item())}
                </h2>
                <p class="text-white/60 text-sm truncate drop-shadow">
                  S{String(item().season_number!).padStart(2, '0')}E{String(item().episode_number!).padStart(2, '0')}
                  <Show when={item().episode_title}>
                    {' — '}{item().episode_title}
                  </Show>
                </p>
              </Show>
            </div>
          </div>
        </div>
      </div>

      {/* Bottom controls */}
      <div
        class={`absolute bottom-0 left-0 right-0 z-20 transition-opacity duration-300
                ${controlsVisible() ? 'opacity-100' : 'opacity-0 pointer-events-none'}`}
      >
        <div class="bg-gradient-to-t from-black/90 via-black/60 to-transparent px-6 pb-5 pt-20">
          {/* Timeline scrubber */}
          <div class="mb-3 relative group/timeline">
            {/* Hover time tooltip — shows time + chapter name if available */}
            <Show when={hoverTime() !== null}>
              <div
                class="absolute -top-10 transform -translate-x-1/2 px-2.5 py-1 rounded-lg bg-black/80 backdrop-blur-sm text-white text-xs font-medium whitespace-nowrap pointer-events-none z-30"
                style={{ left: `${hoverX() - (timelineRef?.getBoundingClientRect().left || 0)}px` }}
              >
                {(() => {
                  const t = hoverTime()! * 1000;
                  const ch = chapters().find(c => t >= c.start_time_ms && t < c.end_time_ms);
                  return ch?.title ? `${ch.title}  ·  ${fmtTime(hoverTime()!)}` : fmtTime(hoverTime()!);
                })()}
              </div>
            </Show>

            <div
              ref={timelineRef!}
              class="relative h-1.5 group-hover/timeline:h-3 rounded-full bg-white/20 cursor-pointer transition-all duration-150"
              onMouseMove={onTimelineHover}
              onMouseLeave={onTimelineLeave}
              onMouseDown={onTimelineDown}
            >
              {/* Buffered */}
              <div
                class="absolute top-0 left-0 h-full bg-white/20 rounded-full pointer-events-none"
                style={{ width: `${bufferedPct()}%` }}
              />
              {/* Progress */}
              <div
                class="absolute top-0 left-0 h-full bg-ferrite-500 rounded-full pointer-events-none transition-[width] duration-75"
                style={{ width: `${progressPct()}%` }}
              />
              {/* Chapter markers */}
              <For each={chapters()}>{(ch) => {
                const pct = knownDuration() > 0 ? (ch.start_time_ms / 1000 / knownDuration()) * 100 : 0;
                if (pct <= 0 || pct >= 100) return null;
                return (
                  <div
                    class="absolute top-0 h-full w-0.5 bg-white/40 pointer-events-none z-10"
                    style={{ left: `${pct}%` }}
                  />
                );
              }}</For>
              {/* Scrub handle */}
              <div
                class={`absolute top-1/2 -translate-y-1/2 -translate-x-1/2 w-4 h-4 rounded-full bg-ferrite-500 shadow-lg shadow-ferrite-500/30
                        transition-all duration-150
                        ${isDragging() ? 'scale-125' : 'scale-0 group-hover/timeline:scale-100'}`}
                style={{ left: `${progressPct()}%` }}
              />
            </div>
          </div>

          {/* Controls row */}
          <div class="flex items-center gap-2">
            {/* Left controls */}
            <div class="flex items-center gap-1">
              {/* Play/Pause */}
              <button class="btn-icon text-white hover:bg-white/10 w-10 h-10" onClick={togglePlay}>
                {playing() ? <Pause class="w-6 h-6" /> : <Play class="w-6 h-6 fill-current ml-0.5" />}
              </button>

              {/* Skip back/forward */}
              <button class="btn-icon text-white/70 hover:text-white hover:bg-white/10" onClick={() => seekRelative(-10)} title="Back 10s (←)">
                <SkipBack class="w-5 h-5" />
              </button>
              <button class="btn-icon text-white/70 hover:text-white hover:bg-white/10" onClick={() => seekRelative(10)} title="Forward 10s (→)">
                <SkipForward class="w-5 h-5" />
              </button>

              {/* Volume */}
              <div class="flex items-center gap-1 group/vol ml-1">
                <button class="btn-icon text-white/70 hover:text-white hover:bg-white/10" onClick={toggleMute} title="Mute (M)" aria-label={`${volume() === 0 ? 'Unmute' : 'Mute'} (M)`}>
                  <VolumeIcon />
                </button>
                <div class="w-0 group-hover/vol:w-24 focus-within:w-24 overflow-hidden transition-all duration-200">
                  <input
                    type="range"
                    min="0"
                    max="100"
                    step="1"
                    value={volume()}
                    onInput={e => handleVolumeChange(parseInt(e.currentTarget.value))}
                    aria-label={`Volume: ${volume()}%`}
                    aria-valuemin={0}
                    aria-valuemax={100}
                    aria-valuenow={volume()}
                    class="w-24 h-1 bg-white/20 rounded-full appearance-none cursor-pointer accent-ferrite-500
                           [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:w-3 [&::-webkit-slider-thumb]:h-3
                           [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-white [&::-webkit-slider-thumb]:shadow"
                  />
                </div>
              </div>

              {/* Time display */}
              <span class="text-sm text-white/70 ml-2 tabular-nums whitespace-nowrap">
                {fmtTime(currentTime())} <span class="text-white/40">/</span> {fmtTime(knownDuration())}
              </span>
            </div>

            {/* Spacer */}
            <div class="flex-1" />

            {/* Right controls */}
            <div class="flex items-center gap-1">
              {/* Speed indicator */}
              <Show when={playbackSpeed() !== 1}>
                <span class="text-xs text-ferrite-400 font-medium mr-1">{playbackSpeed()}x</span>
              </Show>

              {/* Subtitle track selector — only show when external subtitles exist */}
              <Show when={subtitleTracks().length > 0}>
                <div class="relative">
                  <button
                    class={`btn-icon text-white/70 hover:text-white hover:bg-white/10 ${showSubtitleMenu() ? 'text-white bg-white/10' : ''} ${selectedSubtitle() !== null ? 'text-ferrite-400' : ''}`}
                    onClick={(e) => { e.stopPropagation(); setShowSubtitleMenu(!showSubtitleMenu()); setShowSettings(false); setShowAudioMenu(false); setShowQualityMenu(false); }}
                    title="Subtitles"
                  >
                    <Captions class="w-5 h-5" />
                  </button>

                  <Show when={showSubtitleMenu()}>
                    <div
                      class="absolute bottom-12 right-0 w-64 rounded-xl bg-surface-100/95 backdrop-blur-xl border border-white/10 shadow-2xl shadow-black/50 overflow-hidden animate-scale-in z-50"
                      onClick={(e) => e.stopPropagation()}
                    >
                      <div class="px-4 py-3 border-b border-white/5">
                        <span class="text-xs font-semibold text-surface-800 uppercase tracking-wider">Subtitles</span>
                      </div>
                      <div class="p-2 max-h-64 overflow-y-auto">
                        <button
                          class={`w-full text-left px-3 py-2 rounded-lg text-sm transition-colors
                                  ${selectedSubtitle() === null
                                    ? 'bg-ferrite-500/15 text-ferrite-400 font-medium'
                                    : 'text-gray-300 hover:bg-white/5'}`}
                          onClick={() => changeSubtitle(null)}
                        >
                          Off
                        </button>
                        <For each={subtitleTracks()}>
                          {(sub) => {
                            const label = () => {
                              const parts: string[] = [];
                              if (sub.title) parts.push(sub.title);
                              else if (sub.language) parts.push(sub.language.toUpperCase());
                              else parts.push(`Track ${sub.id}`);
                              if (sub.language && sub.title) parts.push(`(${sub.language.toUpperCase()})`);
                              if (sub.is_forced) parts.push('Forced');
                              if (sub.is_sdh) parts.push('SDH');
                              return parts.join(' · ');
                            };
                            return (
                              <button
                                class={`w-full text-left px-3 py-2 rounded-lg text-sm transition-colors
                                        ${selectedSubtitle() === sub.id
                                          ? 'bg-ferrite-500/15 text-ferrite-400 font-medium'
                                          : 'text-gray-300 hover:bg-white/5'}`}
                                onClick={() => changeSubtitle(sub.id)}
                              >
                                {label()}
                              </button>
                            );
                          }}
                        </For>
                      </div>
                    </div>
                  </Show>
                </div>
              </Show>

              {/* Audio track selector — only show when multiple audio tracks exist */}
              <Show when={audioTracks().length > 1}>
                <div class="relative">
                  <button
                    class={`btn-icon text-white/70 hover:text-white hover:bg-white/10 ${showAudioMenu() ? 'text-white bg-white/10' : ''}`}
                    onClick={(e) => { e.stopPropagation(); setShowAudioMenu(!showAudioMenu()); setShowSettings(false); setShowQualityMenu(false); }}
                    title="Audio Track"
                  >
                    <Languages class="w-5 h-5" />
                  </button>

                  <Show when={showAudioMenu()}>
                    <div
                      class="absolute bottom-12 right-0 w-64 rounded-xl bg-surface-100/95 backdrop-blur-xl border border-white/10 shadow-2xl shadow-black/50 overflow-hidden animate-scale-in z-50"
                      onClick={(e) => e.stopPropagation()}
                    >
                      <div class="px-4 py-3 border-b border-white/5">
                        <span class="text-xs font-semibold text-surface-800 uppercase tracking-wider">Audio Track</span>
                      </div>
                      <div class="p-2 max-h-64 overflow-y-auto">
                        <For each={audioTracks()}>
                          {(track, idx) => {
                            const label = () => {
                              const parts: string[] = [];
                              if (track.title) parts.push(track.title);
                              else if (track.language) parts.push(track.language.toUpperCase());
                              else parts.push(`Track ${idx() + 1}`);
                              if (track.language && track.title) parts.push(`(${track.language.toUpperCase()})`);
                              if (track.channels) parts.push(`${track.channels}ch`);
                              if (track.codec_name) parts.push(track.codec_name.toUpperCase());
                              return parts.join(' · ');
                            };
                            return (
                              <button
                                class={`w-full text-left px-3 py-2 rounded-lg text-sm transition-colors
                                        ${selectedAudioTrack() === idx()
                                          ? 'bg-ferrite-500/15 text-ferrite-400 font-medium'
                                          : 'text-gray-300 hover:bg-white/5'}`}
                                onClick={() => changeAudioTrack(idx())}
                              >
                                {label()}
                              </button>
                            );
                          }}
                        </For>
                      </div>
                    </div>
                  </Show>
                </div>
              </Show>

              {/* Quality selector — only show when HLS has multiple quality levels */}
              <Show when={qualityLevels().length > 1}>
                <div class="relative">
                  <button
                    class={`btn-icon text-white/70 hover:text-white hover:bg-white/10 ${showQualityMenu() ? 'text-white bg-white/10' : ''}`}
                    onClick={(e) => { e.stopPropagation(); setShowQualityMenu(!showQualityMenu()); setShowSettings(false); setShowAudioMenu(false); }}
                    title="Quality"
                  >
                    <SlidersHorizontal class="w-5 h-5" />
                  </button>

                  {/* Current quality badge */}
                  <Show when={isHls && qualityLevels().length > 0}>
                    <span class="absolute -top-1 -right-1 text-[0.55rem] font-bold bg-ferrite-500/80 text-white px-1 rounded pointer-events-none">
                      {selectedQuality() === -1 ? 'A' : `${selectedQuality()}p`}
                    </span>
                  </Show>

                  <Show when={showQualityMenu()}>
                    <div
                      class="absolute bottom-12 right-0 w-64 rounded-xl bg-surface-100/95 backdrop-blur-xl border border-white/10 shadow-2xl shadow-black/50 overflow-hidden animate-scale-in z-50"
                      onClick={(e) => e.stopPropagation()}
                    >
                      <div class="px-4 py-3 border-b border-white/5">
                        <span class="text-xs font-semibold text-surface-800 uppercase tracking-wider">Quality</span>
                        <span class="text-xs text-gray-500 ml-2">{currentQualityLabel()}</span>
                      </div>
                      <div class="p-2 max-h-64 overflow-y-auto">
                        {/* Auto option */}
                        <button
                          class={`w-full text-left px-3 py-2 rounded-lg text-sm transition-colors
                                  ${selectedQuality() === -1
                                    ? 'bg-ferrite-500/15 text-ferrite-400 font-medium'
                                    : 'text-gray-300 hover:bg-white/5'}`}
                          onClick={() => changeQuality(-1)}
                        >
                          Auto
                        </button>
                        <For each={qualityLevels()}>
                          {(level) => {
                            const bitrateLabel = () => {
                              if (level.bitrate > 1_000_000) return `${(level.bitrate / 1_000_000).toFixed(1)} Mbps`;
                              if (level.bitrate > 1_000) return `${Math.round(level.bitrate / 1_000)} Kbps`;
                              return '';
                            };
                            return (
                              <button
                                class={`w-full text-left px-3 py-2 rounded-lg text-sm transition-colors
                                        ${selectedQuality() === level.height
                                          ? 'bg-ferrite-500/15 text-ferrite-400 font-medium'
                                          : 'text-gray-300 hover:bg-white/5'}`}
                                onClick={() => changeQuality(level.height)}
                              >
                                {level.label}
                                <Show when={bitrateLabel()}>
                                  <span class="text-xs text-gray-500 ml-2">{bitrateLabel()}</span>
                                </Show>
                              </button>
                            );
                          }}
                        </For>
                      </div>
                    </div>
                  </Show>
                </div>
              </Show>

              {/* Settings */}
              <div class="relative">
                <button
                  class={`btn-icon text-white/70 hover:text-white hover:bg-white/10 ${showSettings() ? 'text-white bg-white/10' : ''}`}
                  onClick={(e) => { e.stopPropagation(); setShowSettings(!showSettings()); setShowAudioMenu(false); setShowQualityMenu(false); }}
                  title="Settings"
                >
                  <Settings class="w-5 h-5" />
                </button>

                {/* Settings panel */}
                <Show when={showSettings()}>
                  <div
                    class="absolute bottom-12 right-0 w-56 rounded-xl bg-surface-100/95 backdrop-blur-xl border border-white/10 shadow-2xl shadow-black/50 overflow-hidden animate-scale-in z-50"
                    onClick={(e) => e.stopPropagation()}
                  >
                    <div class="px-4 py-3 border-b border-white/5">
                      <span class="text-xs font-semibold text-surface-800 uppercase tracking-wider">Playback Speed</span>
                    </div>
                    <div class="p-2">
                      {SPEED_OPTIONS.map(speed => (
                        <button
                          class={`w-full text-left px-3 py-2 rounded-lg text-sm transition-colors
                                  ${playbackSpeed() === speed
                                    ? 'bg-ferrite-500/15 text-ferrite-400 font-medium'
                                    : 'text-gray-300 hover:bg-white/5'}`}
                          onClick={() => changeSpeed(speed)}
                        >
                          {speed === 1 ? 'Normal' : `${speed}x`}
                        </button>
                      ))}
                    </div>
                  </div>
                </Show>
              </div>

              {/* PiP */}
              <Show when={'pictureInPictureEnabled' in document}>
                <button class="btn-icon text-white/70 hover:text-white hover:bg-white/10" onClick={togglePiP} title="Picture in Picture">
                  <PictureInPicture2 class="w-5 h-5" />
                </button>
              </Show>

              {/* Fullscreen */}
              <button class="btn-icon text-white/70 hover:text-white hover:bg-white/10" onClick={toggleFullscreen} title="Fullscreen (F)">
                {isFullscreen() ? <Minimize class="w-5 h-5" /> : <Maximize class="w-5 h-5" />}
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
