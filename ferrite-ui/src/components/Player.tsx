import { createSignal, onMount, onCleanup, Show, For } from 'solid-js';
import Hls from 'hls.js';
import {
  Play, Pause, Volume2, VolumeX, Volume1,
  Maximize, Minimize, ArrowLeft, SkipBack, SkipForward,
  Settings, Loader2, PictureInPicture2, Languages,
} from 'lucide-solid';
import type { MediaItem, MediaStream } from '../api';
import { api, authUrl, getToken } from '../api';
import { getDisplayTitle, getStreamType, fmtTime } from '../utils';
import { perf } from '../lib/perf';
import PerfOverlay from './PerfOverlay';

interface PlayerProps {
  item: MediaItem;
  resumePosition: number | null;
  onClose: () => void;
}

const SPEED_OPTIONS = [0.5, 0.75, 1, 1.25, 1.5, 2];

/** HLS.js config tuned for low-latency live-like playback from a local server */
const HLS_CONFIG = {
  maxBufferLength: 30,
  maxMaxBufferLength: 60,
  maxBufferSize: 60 * 1000 * 1000,
  maxBufferHole: 0.5,
  lowLatencyMode: false,
  startFragPrefetch: true,
  testBandwidth: false,
  abrEwmaDefaultEstimate: 10_000_000,
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

  const item = () => props.item;
  const stream = () => getStreamType(item().container_format, item().video_codec, item().audio_codec);
  const isTranscoded = () => stream() !== 'direct';
  const isFullTranscode = () => stream() === 'full-transcode';
  const knownDuration = () => item().duration_ms ? item().duration_ms! / 1000 : 0;

  let hlsInstance: Hls | null = null;
  let hlsSessionId: string | null = null;
  let seekOffset = 0;
  let hlsStartOffset = 0;
  let isSeeking = false;
  let lastProgressReport = 0;
  let isHls = false;
  let seekIndicatorTimer: ReturnType<typeof setTimeout> | null = null;
  // The last confirmed media-time position (survives seeks)
  let lastConfirmedTime = 0;
  // Monotonically increasing counter to detect stale seek callbacks
  let seekGeneration = 0;

  // ---- Core helpers ----
  function actualTime(): number {
    if (!videoRef) return 0;
    return isHls ? hlsStartOffset + videoRef.currentTime : seekOffset + videoRef.currentTime;
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

  // ---- Controls visibility ----
  function showControls() {
    setControlsVisible(true);
    if (controlsTimeout) clearTimeout(controlsTimeout);
    if (playing() && !showSettings() && !showAudioMenu()) {
      controlsTimeout = setTimeout(() => setControlsVisible(false), 3000);
    }
  }

  function hideControlsDelayed() {
    if (controlsTimeout) clearTimeout(controlsTimeout);
    if (playing() && !showSettings() && !showAudioMenu()) {
      controlsTimeout = setTimeout(() => setControlsVisible(false), 3000);
    }
  }

  // ---- HLS management ----
  function destroyHlsLocal() {
    if (hlsInstance) { hlsInstance.destroy(); hlsInstance = null; }
  }

  function destroyHls() {
    destroyHlsLocal();
    if (hlsSessionId && item()) {
      api.hlsStop(item().id, hlsSessionId);
      hlsSessionId = null;
    }
  }

  function handleClose() {
    // Report the last known-good position (not mid-seek garbage)
    if (!isSeeking) {
      reportProgress();
    } else if (lastConfirmedTime > 0) {
      api.updateProgress(item().id, lastConfirmedTime);
    }
    destroyHls();
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
  onMount(() => {
    perf.reset();
    perf.startSpan('init/total', 'frontend', { stream: stream() });

    const video = videoRef;
    const id = item().id;

    video.volume = volume() / 100;

    // Fetch audio tracks for multi-audio selection
    api.getStreams(id).then(streams => {
      const audio = streams.filter(s => s.stream_type === 'audio');
      setAudioTracks(audio);
      // Default to the stream marked as default, or first
      const defaultIdx = audio.findIndex(s => s.is_default === 1);
      if (defaultIdx > 0) setSelectedAudioTrack(defaultIdx);
    }).catch(() => {});

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
      perf.endSpan('init/total');
      perf.endSpan('init/first-frame');
      video.removeEventListener('playing', onFirstPlay);
    };
    video.addEventListener('playing', onFirstPlay);
    perf.startSpan('init/first-frame', 'frontend');

    const useHlsJs = isTranscoded() && Hls.isSupported();
    const useNativeHls = isTranscoded() && !useHlsJs && video.canPlayType('application/vnd.apple.mpegurl') !== '';

    if (useHlsJs) {
      isHls = true;
      hlsStartOffset = startAt;
      const startParam = startAt > 0.5 ? `&start=${startAt.toFixed(3)}` : '';
      const masterUrl = `/api/stream/${id}/hls/master.m3u8?_=1${startParam}`;

      perf.startSpan('init/hls-setup', 'frontend');
      const hls = createHls();
      hlsInstance = hls;

      hls.on(Hls.Events.MANIFEST_PARSED, () => {
        perf.endSpan('init/hls-setup');
        if (hls.levels && hls.levels.length > 0) {
          const lvlUrl = (hls.levels[0] as any).url;
          const match = lvlUrl?.match(/\/hls\/([^/]+)\/playlist\.m3u8/);
          if (match) hlsSessionId = match[1];
        }
        video.play();
      });

      hls.on(Hls.Events.ERROR, (_event: any, data: any) => {
        if (data.fatal) {
          if (hlsInstance !== hls) return;
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

      hls.loadSource(authUrl(masterUrl));
      hls.attachMedia(video);
    } else if (useNativeHls) {
      isHls = true;
      hlsStartOffset = startAt;
      video.src = authUrl(`/api/stream/${id}/hls/master.m3u8`);
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
        video.src = authUrl(`/api/stream/${id}?start=${seekOffset.toFixed(3)}`);
        video.addEventListener('canplay', function onCan() {
          perf.endSpan('init/stream-load');
          video.removeEventListener('canplay', onCan);
        }, { once: true });
        video.play();
      })();
    } else {
      perf.startSpan('init/stream-load', 'network');
      video.src = authUrl(`/api/stream/${id}`);
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
    setCurrentTime(t);
    setPlaying(!videoRef.paused);

    const now = Date.now();
    if (now - lastProgressReport > 10000) {
      lastProgressReport = now;
      reportProgress();
    }
  }

  function onProgress() {
    if (!knownDuration() || !videoRef.buffered.length) return;
    const buffEnd = videoRef.buffered.end(videoRef.buffered.length - 1);
    const total = isHls ? hlsStartOffset + buffEnd : seekOffset + buffEnd;
    setBufferedPct(Math.min(100, (total / knownDuration()) * 100));
  }

  function onEnded() { api.markCompleted(item().id); }
  function onPlay() { setPlaying(true); setBuffering(false); hideControlsDelayed(); }
  function onPause() { setPlaying(false); showControls(); }
  function onWaiting() { setBuffering(true); }
  function onCanPlay() { setBuffering(false); }

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

  function changeAudioTrack(trackIndex: number) {
    if (trackIndex === selectedAudioTrack()) { setShowAudioMenu(false); return; }
    setSelectedAudioTrack(trackIndex);
    setShowAudioMenu(false);
    // For HLS streams, trigger a seek to the current position with the new audio track.
    // The audio_stream param will be picked up by the backend on the next HLS session.
    if (isHls) {
      const pos = currentTime();
      hlsSeekTo(pos);
    }
    // For non-HLS transcoded streams, the audio_stream param would need to be
    // added to the stream URL. For now, HLS is the primary use case.
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
    // Bump generation so any in-flight seek callbacks become stale
    const gen = ++seekGeneration;

    perf.startSpan('seek/hls-total', 'frontend', { target: Math.round(targetTime) });
    isSeeking = true;
    setBuffering(true);
    videoRef.pause();
    setCurrentTime(targetTime);

    destroyHlsLocal();
    hlsSessionId = null;

    try {
      perf.startSpan('seek/hls-api', 'network');
      const audioIdx = selectedAudioTrack();
      const seekRes = await api.hlsSeek(item().id, targetTime, audioIdx > 0 ? audioIdx : undefined);

      // If another seek was initiated while we were waiting, abandon this one
      if (gen !== seekGeneration) return;

      perf.endSpan('seek/hls-api');
      if (seekRes.timing_ms) perf.ingestBackendTiming('seek/hls', seekRes.timing_ms);

      // The server returns the actual start_secs (which is the time FFmpeg
      // was told to seek to). HLS segments start at t=0 relative to this offset,
      // so we add it to video.currentTime to get absolute media time.
      hlsStartOffset = seekRes.start_secs ?? targetTime;
      hlsSessionId = seekRes.session_id;
      isHls = true;

      perf.startSpan('seek/hls-manifest', 'network');
      const hls = createHls();
      hlsInstance = hls;

      hls.on(Hls.Events.MANIFEST_PARSED, () => {
        // Stale seek — a newer one has taken over
        if (gen !== seekGeneration) { hls.destroy(); return; }
        perf.endSpan('seek/hls-manifest');
        videoRef.play();
        setTimeout(() => {
          if (gen !== seekGeneration) return;
          isSeeking = false;
          setBuffering(false);
          perf.endSpan('seek/hls-total');
          // Update confirmed time now that we're playing from the new position
          lastConfirmedTime = Math.floor(hlsStartOffset * 1000);
        }, 200);
      });

      hls.on(Hls.Events.ERROR, (_e: any, d: any) => {
        if (d.fatal) {
          if (gen !== seekGeneration) return;
          perf.event('seek/hls-error', 'frontend', { type: d.type, details: d.details });
          console.error('HLS seek error:', d.type, d.details);
          isSeeking = false;
          setBuffering(false);
          perf.endSpan('seek/hls-total');
        }
      });

      hls.loadSource(authUrl(seekRes.master_url));
      hls.attachMedia(videoRef);
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
      setCurrentTime(actualStart);
      lastConfirmedTime = Math.floor(actualStart * 1000);
      perf.startSpan('seek/stream-load', 'network');
      videoRef.src = authUrl(`/api/stream/${item().id}?start=${actualStart.toFixed(3)}`);
      videoRef.addEventListener('canplay', function onCan() {
        perf.endSpan('seek/stream-load');
        perf.endSpan('seek/transcode-total');
        videoRef.removeEventListener('canplay', onCan);
      }, { once: true });
      videoRef.play();
      setTimeout(() => { isSeeking = false; }, 500);
    } else {
      perf.startSpan('seek/direct', 'frontend', { target: Math.round(targetTime) });
      videoRef.currentTime = targetTime;
      videoRef.addEventListener('seeked', function onSeeked() {
        perf.endSpan('seek/direct');
        videoRef.removeEventListener('seeked', onSeeked);
      }, { once: true });
    }
  }

  async function seekRelative(deltaSec: number) {
    if (!knownDuration()) return;
    // Use the signal-based currentTime() instead of actualTime() because during
    // an active seek, videoRef.currentTime is stale (from the destroyed session).
    // currentTime() is always updated immediately when a seek starts.
    const base = isSeeking ? currentTime() : actualTime();
    const target = Math.max(0, Math.min(knownDuration(), base + deltaSec));
    flashSeekIndicator(deltaSec > 0 ? `+${deltaSec}s` : `${deltaSec}s`);
    await seekToTime(target);
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
      onClick={() => { if (showSettings()) setShowSettings(false); if (showAudioMenu()) setShowAudioMenu(false); }}
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
        onClick={(e) => { e.stopPropagation(); togglePlay(); showControls(); }}
        onDblClick={(e) => { e.stopPropagation(); handleDoubleClick(e); }}
      />

      {/* Performance overlay (toggle with P key) */}
      <PerfOverlay visible={showPerf()} />

      {/* Buffering spinner */}
      <Show when={buffering() && !isDragging()}>
        <div class="absolute inset-0 flex items-center justify-center pointer-events-none z-10">
          <Loader2 class="w-12 h-12 text-white/80 animate-spin" />
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
              <h2 class="text-white font-semibold text-lg truncate drop-shadow-lg">
                {getDisplayTitle(item())}
              </h2>
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
            {/* Hover time tooltip */}
            <Show when={hoverTime() !== null}>
              <div
                class="absolute -top-10 transform -translate-x-1/2 px-2.5 py-1 rounded-lg bg-black/80 backdrop-blur-sm text-white text-xs font-medium whitespace-nowrap pointer-events-none z-30"
                style={{ left: `${hoverX() - (timelineRef?.getBoundingClientRect().left || 0)}px` }}
              >
                {fmtTime(hoverTime()!)}
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
                <button class="btn-icon text-white/70 hover:text-white hover:bg-white/10" onClick={toggleMute} title="Mute (M)">
                  <VolumeIcon />
                </button>
                <div class="w-0 group-hover/vol:w-24 overflow-hidden transition-all duration-200">
                  <input
                    type="range"
                    min="0"
                    max="100"
                    step="1"
                    value={volume()}
                    onInput={e => handleVolumeChange(parseInt(e.currentTarget.value))}
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

              {/* Audio track selector — only show when multiple audio tracks exist */}
              <Show when={audioTracks().length > 1}>
                <div class="relative">
                  <button
                    class={`btn-icon text-white/70 hover:text-white hover:bg-white/10 ${showAudioMenu() ? 'text-white bg-white/10' : ''}`}
                    onClick={(e) => { e.stopPropagation(); setShowAudioMenu(!showAudioMenu()); setShowSettings(false); }}
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

              {/* Settings */}
              <div class="relative">
                <button
                  class={`btn-icon text-white/70 hover:text-white hover:bg-white/10 ${showSettings() ? 'text-white bg-white/10' : ''}`}
                  onClick={(e) => { e.stopPropagation(); setShowSettings(!showSettings()); setShowAudioMenu(false); }}
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
