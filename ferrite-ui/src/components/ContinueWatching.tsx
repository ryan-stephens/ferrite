import { For, Show } from 'solid-js';
import type { MediaItem } from '../api';
import { authUrl } from '../api';
import { getDisplayTitle, formatDuration } from '../utils';

interface ContinueWatchingProps {
  items: MediaItem[];
  onShowDetail: (id: string) => void;
}

export default function ContinueWatching(props: ContinueWatchingProps) {
  const inProgress = () =>
    props.items
      .filter(item => item.position_ms && item.position_ms > 0 && !item.completed && item.duration_ms)
      .sort((a, b) => (b.last_played_at || '').localeCompare(a.last_played_at || ''))
      .slice(0, 10);

  return (
    <Show when={inProgress().length > 0}>
      <div class="mb-6">
        <h3 class="text-sm text-gray-400 mb-3">Continue Watching</h3>
        <div class="flex gap-3 overflow-x-auto pb-2 scrollbar-thin">
          <For each={inProgress()}>
            {item => {
              const pct = () => Math.min(100, ((item.position_ms || 0) / (item.duration_ms || 1)) * 100);
              const remaining = () => formatDuration((item.duration_ms || 0) - (item.position_ms || 0));
              return (
                <div
                  class="flex-shrink-0 w-[140px] bg-surface-100 rounded-lg overflow-hidden cursor-pointer border border-surface-300 hover:-translate-y-0.5 transition-transform"
                  onClick={() => props.onShowDetail(item.id)}
                >
                  <div class="w-full aspect-[2/3] bg-surface-200 flex items-center justify-center text-2xl text-surface-400 relative">
                    {item.poster_path ? (
                      <img src={authUrl(`/api/images/${item.poster_path}`)} class="w-full h-full object-cover" loading="lazy" />
                    ) : '▶'}
                    <div class="absolute bottom-0 left-0 right-0 h-[3px] bg-black/50">
                      <div class="h-full bg-ferrite-500" style={{ width: `${pct()}%` }} />
                    </div>
                  </div>
                  <div class="p-2">
                    <div class="font-semibold text-xs truncate">{getDisplayTitle(item)}</div>
                    <div class="text-[0.65rem] text-gray-500 mt-0.5">{remaining()} left</div>
                  </div>
                </div>
              );
            }}
          </For>
        </div>
      </div>
    </Show>
  );
}
