import { createSignal, Show, onMount } from 'solid-js';
import { useParams, useNavigate } from '@solidjs/router';
import { Play, ArrowLeft, Star, Clock, HardDrive, Film, Music, Monitor, ChevronRight, CheckCircle, Circle } from 'lucide-solid';
import { api, authUrl } from '../api';
import type { MediaItem } from '../api';
import { getDisplayTitle, getDisplayYear, formatDuration, formatSize, getResLabel, getStreamType } from '../utils';

export default function MediaDetailPage() {
  const params = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [item, setItem] = createSignal<MediaItem | null>(null);
  const [showTechInfo, setShowTechInfo] = createSignal(false);
  const [togglingWatched, setTogglingWatched] = createSignal(false);

  onMount(async () => {
    try {
      const data = await api.getMedia(params.id);
      setItem(data);
    } catch {
      navigate('/');
    }
  });

  const title = () => item() ? getDisplayTitle(item()!) : '';
  const year = () => item() ? getDisplayYear(item()!) : null;
  const posterUrl = () => item()?.poster_path ? authUrl(`/api/images/${item()!.poster_path}`) : null;
  const streamType = () => item() ? getStreamType(item()!.container_format, item()!.video_codec, item()!.audio_codec) : 'direct';
  const hasProgress = () => item()?.position_ms && item()!.position_ms! > 0 && !item()!.completed;
  const progressPct = () => {
    const i = item();
    if (!i?.position_ms || !i?.duration_ms) return 0;
    return Math.min(100, (i.position_ms / i.duration_ms) * 100);
  };
  const resumeTime = () => {
    const i = item();
    if (!i?.position_ms) return 0;
    return i.position_ms / 1000;
  };

  const streamLabel = () => {
    switch (streamType()) {
      case 'direct': return 'Direct Play';
      case 'remux': return 'Remux';
      case 'audio-transcode': return 'Audio Transcode';
      case 'full-transcode': return 'Full Transcode';
    }
  };

  const streamColor = () => {
    switch (streamType()) {
      case 'direct': return 'bg-green-500/15 text-green-400 border-green-500/20';
      case 'remux': return 'bg-blue-500/15 text-blue-400 border-blue-500/20';
      case 'audio-transcode': return 'bg-yellow-500/15 text-yellow-400 border-yellow-500/20';
      case 'full-transcode': return 'bg-red-500/15 text-red-400 border-red-500/20';
    }
  };

  async function handleToggleWatched() {
    if (!item() || togglingWatched()) return;
    setTogglingWatched(true);
    try {
      if (item()!.completed) {
        await api.resetProgress(item()!.id);
        setItem({ ...item()!, completed: false, position_ms: 0 });
      } else {
        await api.markCompleted(item()!.id);
        setItem({ ...item()!, completed: true, position_ms: 0 });
      }
    } finally {
      setTogglingWatched(false);
    }
  }

  function handlePlay(resumePos: number | null = null) {
    if (!item()) return;
    if (resumePos !== null) {
      navigate(`/player/${item()!.id}?resume=${resumePos}`);
    } else {
      navigate(`/player/${item()!.id}`);
    }
  }

  return (
    <Show when={item()} fallback={
      <div class="flex items-center justify-center h-96">
        <div class="w-8 h-8 border-2 border-surface-400 border-t-ferrite-500 rounded-full animate-spin" />
      </div>
    }>
      <div class="animate-fade-in">
        {/* Backdrop */}
        <div class="relative h-[480px] overflow-hidden">
          <Show when={posterUrl()}>
            <img src={posterUrl()!} alt="" class="absolute inset-0 w-full h-full object-cover opacity-20 blur-3xl scale-125" />
          </Show>
          <div class="absolute inset-0 bg-gradient-to-t from-surface via-surface/80 to-surface/20" />
          <div class="absolute inset-0 bg-gradient-to-r from-surface/90 via-surface/40 to-transparent" />

          {/* Back button */}
          <button
            class="absolute top-6 left-6 btn-ghost text-white z-10"
            onClick={() => window.history.length > 1 ? navigate(-1) : navigate('/')}
          >
            <ArrowLeft class="w-5 h-5" /> Back
          </button>

          {/* Content */}
          <div class="relative h-full flex items-end px-8 pb-10">
            <div class="flex gap-8 items-end max-w-5xl w-full">
              {/* Poster */}
              <Show when={posterUrl()}>
                <div class="flex-shrink-0">
                  <img
                    src={posterUrl()!}
                    alt={title()}
                    class="w-48 h-72 object-cover rounded-xl shadow-2xl shadow-black/60 border border-white/10"
                  />
                  {/* Progress bar under poster */}
                  <Show when={hasProgress()}>
                    <div class="mt-2 h-1.5 rounded-full bg-surface-300 overflow-hidden">
                      <div class="h-full bg-ferrite-500 rounded-full" style={{ width: `${progressPct()}%` }} />
                    </div>
                  </Show>
                </div>
              </Show>

              {/* Info */}
              <div class="flex-1 space-y-4 pb-2">
                {/* Meta badges */}
                <div class="flex items-center gap-2 flex-wrap text-sm text-surface-800">
                  <Show when={year()}>
                    <span class="font-medium">{year()}</span>
                  </Show>
                  <Show when={item()!.content_rating}>
                    <span class="badge bg-surface-400/50 text-surface-900 border border-surface-500/30">{item()!.content_rating}</span>
                  </Show>
                  <Show when={item()!.duration_ms}>
                    <span class="flex items-center gap-1"><Clock class="w-3.5 h-3.5" />{formatDuration(item()!.duration_ms)}</span>
                  </Show>
                  <Show when={item()!.rating}>
                    <span class="flex items-center gap-1"><Star class="w-3.5 h-3.5 text-yellow-500 fill-yellow-500" />{item()!.rating!.toFixed(1)}</span>
                  </Show>
                  <span class={`badge border ${streamColor()}`}>{streamLabel()}</span>
                </div>

                {/* Title */}
                <h1 class="text-4xl font-bold text-white leading-tight text-balance">{title()}</h1>

                {/* Genres */}
                <Show when={item()!.genres}>
                  <div class="flex items-center gap-2 flex-wrap">
                    {item()!.genres!.split(',').map(g => g.trim()).filter(Boolean).map(genre => (
                      <span class="badge bg-surface-300/50 text-surface-900 border border-surface-400/30">{genre}</span>
                    ))}
                  </div>
                </Show>

                {/* Overview */}
                <Show when={item()!.overview}>
                  <p class="text-sm text-surface-800 leading-relaxed max-w-2xl">{item()!.overview}</p>
                </Show>

                {/* Action buttons */}
                <div class="flex items-center gap-3 pt-2">
                  <Show when={hasProgress()} fallback={
                    <button class="btn-primary text-base px-7 py-3" onClick={() => handlePlay()}>
                      <Play class="w-5 h-5 fill-current" /> {item()!.completed ? 'Play Again' : 'Play'}
                    </button>
                  }>
                    <button class="btn-primary text-base px-7 py-3" onClick={() => handlePlay(resumeTime())}>
                      <Play class="w-5 h-5 fill-current" /> Resume
                    </button>
                    <button class="btn-secondary" onClick={() => handlePlay(0)}>
                      Play from Start
                    </button>
                  </Show>
                  <button
                    class={`btn-ghost text-sm flex items-center gap-1.5 ${item()!.completed ? 'text-green-400 hover:text-gray-300' : 'text-surface-800 hover:text-gray-300'}`}
                    onClick={handleToggleWatched}
                    disabled={togglingWatched()}
                    title={item()!.completed ? 'Mark as unwatched' : 'Mark as watched'}
                  >
                    <Show when={item()!.completed} fallback={<Circle class="w-4 h-4" />}>
                      <CheckCircle class="w-4 h-4" />
                    </Show>
                    {item()!.completed ? 'Watched' : 'Mark Watched'}
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>

        {/* Technical Info */}
        <div class="px-8 py-6">
          <button
            class="flex items-center gap-2 text-sm font-medium text-surface-800 hover:text-gray-300 transition-colors"
            onClick={() => setShowTechInfo(!showTechInfo())}
          >
            <ChevronRight class={`w-4 h-4 transition-transform ${showTechInfo() ? 'rotate-90' : ''}`} />
            Technical Details
          </button>

          <Show when={showTechInfo()}>
            <div class="mt-4 grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 animate-slide-down">
              <TechCard icon={Film} label="Video" value={`${item()!.video_codec || 'Unknown'} · ${getResLabel(item()!.width, item()!.height) || `${item()!.width}×${item()!.height}`}`} />
              <TechCard icon={Music} label="Audio" value={item()!.audio_codec || 'Unknown'} />
              <TechCard icon={HardDrive} label="File Size" value={formatSize(item()!.file_size)} />
              <TechCard icon={Monitor} label="Container" value={item()!.container_format || 'Unknown'} />
              <Show when={item()!.bitrate_kbps}>
                <TechCard icon={HardDrive} label="Bitrate" value={`${item()!.bitrate_kbps} kbps`} />
              </Show>
            </div>
          </Show>
        </div>
      </div>
    </Show>
  );
}

function TechCard(props: { icon: any; label: string; value: string }) {
  const Icon = props.icon;
  return (
    <div class="flex items-center gap-3 px-4 py-3 rounded-xl bg-surface-100 border border-surface-300/50">
      <Icon class="w-4.5 h-4.5 text-surface-700 flex-shrink-0" />
      <div>
        <div class="text-2xs text-surface-700 uppercase tracking-wider font-medium">{props.label}</div>
        <div class="text-sm text-gray-300">{props.value}</div>
      </div>
    </div>
  );
}
