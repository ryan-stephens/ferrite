import { Show } from 'solid-js';
import type { MediaItem } from '../api';
import { authUrl } from '../api';
import { getDisplayTitle, getDisplayYear, formatSize, formatDuration, getResLabel, getStreamType } from '../utils';

interface MediaCardProps {
  item: MediaItem;
  onClick: () => void;
}

export default function MediaCard(props: MediaCardProps) {
  const item = () => props.item;
  const title = () => getDisplayTitle(item());
  const year = () => getDisplayYear(item());
  const dur = () => formatDuration(item().duration_ms);
  const res = () => getResLabel(item().width, item().height);
  const stream = () => getStreamType(item().container_format, item().video_codec, item().audio_codec);

  const metaParts = () => {
    const parts: string[] = [formatSize(item().file_size)];
    const d = dur(); if (d) parts.push(d);
    const y = year(); if (y) parts.push(String(y));
    if (item().rating) parts.push(`★ ${item().rating}`);
    return parts.join(' · ');
  };

  const progressPct = () => {
    if (item().completed) return -1; // sentinel for checkmark
    if (item().position_ms && item().position_ms! > 0 && item().duration_ms) {
      return Math.min(100, (item().position_ms! / item().duration_ms!) * 100);
    }
    return 0;
  };

  return (
    <div
      class="bg-surface-100 rounded-xl overflow-hidden cursor-pointer border border-surface-300 hover:-translate-y-1 hover:shadow-xl hover:shadow-black/40 transition-all group"
      onClick={props.onClick}
    >
      <div class="w-full aspect-[2/3] bg-surface-200 flex items-center justify-center text-4xl text-surface-400 relative">
        <Show when={item().poster_path} fallback={<span>▶</span>}>
          <img
            src={authUrl(`/api/images/${item().poster_path}`)}
            class="w-full h-full object-cover"
            loading="lazy"
          />
        </Show>

        {/* Completed checkmark */}
        <Show when={progressPct() === -1}>
          <div class="absolute top-1.5 right-1.5 bg-black/70 text-green-400 w-5 h-5 rounded-full flex items-center justify-center text-xs">
            ✓
          </div>
        </Show>

        {/* Progress bar */}
        <Show when={progressPct() > 0}>
          <div class="absolute bottom-0 left-0 right-0 h-[3px] bg-black/50">
            <div class="h-full bg-ferrite-500 rounded-tr-sm" style={{ width: `${progressPct()}%` }} />
          </div>
        </Show>

        {/* Duration hover */}
        <Show when={dur()}>
          <div class="absolute bottom-2 right-1.5 bg-black/80 text-white px-1.5 py-0.5 rounded text-[0.65rem] opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none">
            {dur()}
          </div>
        </Show>
      </div>

      <div class="p-3">
        <div class="font-semibold text-sm truncate">{title()}</div>
        <div class="text-xs text-gray-500 mt-1">{metaParts()}</div>
        <div class="flex gap-1 mt-1.5 flex-wrap">
          <span class={`inline-block px-1.5 py-0.5 rounded text-[0.65rem] font-semibold uppercase ${
            stream() === 'direct'
              ? 'bg-green-900/50 text-green-400'
              : stream() === 'remux' ? 'bg-blue-900/50 text-blue-400'
              : 'bg-amber-900/50 text-amber-400'
          }`}>
            {stream() === 'direct' ? 'Direct Play' : stream() === 'remux' ? 'Remux' : stream() === 'audio-transcode' ? 'Audio Transcode' : 'Full Transcode'}
          </span>
          <Show when={res()}>
            <span class="inline-block px-1.5 py-0.5 rounded text-[0.65rem] font-semibold uppercase bg-purple-900/50 text-purple-300">
              {res()}
            </span>
          </Show>
          <Show when={item().video_codec}>
            <span class="inline-block px-1.5 py-0.5 rounded text-[0.65rem] font-semibold uppercase bg-surface-200 text-blue-300">
              {item().video_codec}
            </span>
          </Show>
        </div>
      </div>
    </div>
  );
}
