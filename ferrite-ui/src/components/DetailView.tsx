import { Show } from 'solid-js';
import type { MediaItem } from '../api';
import { authUrl } from '../api';
import { getDisplayTitle, getDisplayYear, formatDuration, getResLabel, getStreamType, fmtTime } from '../utils';

interface DetailViewProps {
  item: MediaItem;
  onClose: () => void;
  onPlay: (id: string, resumePos: number | null) => void;
}

export default function DetailView(props: DetailViewProps) {
  const item = () => props.item;
  const title = () => getDisplayTitle(item());
  const year = () => getDisplayYear(item());
  const stream = () => getStreamType(item().container_format, item().video_codec, item().audio_codec);
  const res = () => getResLabel(item().width, item().height);

  const metaParts = () => {
    const parts: string[] = [];
    const y = year(); if (y) parts.push(String(y));
    if (item().content_rating) parts.push(item().content_rating!);
    const d = formatDuration(item().duration_ms); if (d) parts.push(d);
    return parts.join(' • ');
  };

  const genres = () => {
    if (!item().genres) return [];
    try { return JSON.parse(item().genres!) as string[]; } catch { return []; }
  };

  const hasResume = () =>
    item().position_ms && item().position_ms! > 0 && !item().completed && item().duration_ms;

  const resumeTime = () => fmtTime((item().position_ms || 0) / 1000);

  return (
    <div class="fixed inset-0 bg-black/95 z-[100] overflow-y-auto" onClick={e => { if (e.target === e.currentTarget) props.onClose(); }}>
      <button
        class="absolute top-4 right-6 text-3xl text-white hover:text-gray-300 bg-transparent border-none cursor-pointer z-[110]"
        onClick={props.onClose}
      >
        ×
      </button>

      <div class="max-w-[900px] mx-auto flex gap-8 p-8 mt-12">
        {/* Poster */}
        <div class="flex-shrink-0">
          <Show when={item().poster_path} fallback={
            <div class="w-[300px] h-[450px] bg-surface-200 rounded-xl flex items-center justify-center text-5xl text-surface-400">
              🎬
            </div>
          }>
            <img
              src={authUrl(`/api/images/${item().poster_path}`)}
              class="w-[300px] rounded-xl shadow-2xl"
            />
          </Show>
        </div>

        {/* Info */}
        <div class="flex-1">
          <h2 class="text-3xl font-bold mb-1">{title()}</h2>
          <div class="text-gray-500 mb-2">{metaParts()}</div>

          <Show when={item().rating}>
            <div class="text-amber-400 mb-3">★ {item().rating} / 10</div>
          </Show>

          <Show when={genres().length > 0}>
            <div class="flex gap-1.5 flex-wrap mb-4">
              {genres().map(g => (
                <span class="bg-surface-300 px-2.5 py-1 rounded-full text-xs text-gray-400">{g}</span>
              ))}
            </div>
          </Show>

          <Show when={item().overview}>
            <p class="text-gray-400 leading-relaxed mb-6 text-sm">{item().overview}</p>
          </Show>

          {/* Badges */}
          <div class="flex gap-1.5 flex-wrap mb-6">
            <span class={`inline-block px-2 py-1 rounded text-xs font-semibold uppercase ${
              stream() === 'direct' ? 'bg-green-900/50 text-green-400' : stream() === 'remux' ? 'bg-blue-900/50 text-blue-400' : 'bg-amber-900/50 text-amber-400'
            }`}>
              {stream() === 'direct' ? 'Direct Play' : stream() === 'remux' ? 'Remux' : stream() === 'audio-transcode' ? 'Audio Transcode' : 'Full Transcode'}
            </span>
            <Show when={res()}>
              <span class="inline-block px-2 py-1 rounded text-xs font-semibold uppercase bg-purple-900/50 text-purple-300">{res()}</span>
            </Show>
            <Show when={item().video_codec}>
              <span class="inline-block px-2 py-1 rounded text-xs font-semibold uppercase bg-surface-200 text-blue-300">{item().video_codec}</span>
            </Show>
            <Show when={item().audio_codec}>
              <span class="inline-block px-2 py-1 rounded text-xs font-semibold uppercase bg-surface-200 text-blue-300">{item().audio_codec}</span>
            </Show>
          </div>

          {/* Actions */}
          <div class="flex gap-3 items-center flex-wrap">
            <Show when={hasResume()} fallback={
              <button
                class="bg-ferrite-500 hover:bg-ferrite-600 text-white font-medium px-5 py-2.5 rounded-md transition-colors"
                onClick={() => props.onPlay(item().id, 0)}
              >
                ▶ Play
              </button>
            }>
              <button
                class="bg-ferrite-500 hover:bg-ferrite-600 text-white font-medium px-5 py-2.5 rounded-md transition-colors"
                onClick={() => props.onPlay(item().id, null)}
              >
                ▶ Resume from {resumeTime()}
              </button>
              <button
                class="bg-surface-300 hover:bg-surface-400 text-gray-300 font-medium px-5 py-2.5 rounded-md transition-colors"
                onClick={() => props.onPlay(item().id, 0)}
              >
                Play from Start
              </button>
            </Show>
          </div>
        </div>
      </div>
    </div>
  );
}
