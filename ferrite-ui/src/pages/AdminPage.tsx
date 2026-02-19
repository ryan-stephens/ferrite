import { createSignal, onMount, onCleanup, For, Show } from 'solid-js';
import { Activity, Monitor, Clock, Wifi, WifiOff, RefreshCw } from 'lucide-solid';
import { api } from '../api';
import type { ActiveStream } from '../api';
import { fmtTime } from '../utils';

function fmtAge(secs: number): string {
  if (secs < 60) return `${secs}s`;
  const m = Math.floor(secs / 60);
  const s = secs % 60;
  if (m < 60) return `${m}m ${s}s`;
  const h = Math.floor(m / 60);
  return `${h}h ${m % 60}m`;
}

function resLabel(s: ActiveStream): string {
  if (s.height) return `${s.height}p`;
  if (s.width && s.height) return `${s.width}×${s.height}`;
  return s.variant_label ?? 'native';
}

export default function AdminPage() {
  const [streams, setStreams] = createSignal<ActiveStream[]>([]);
  const [loading, setLoading] = createSignal(true);
  const [error, setError] = createSignal<string | null>(null);
  const [lastRefresh, setLastRefresh] = createSignal<Date | null>(null);
  let intervalId: ReturnType<typeof setInterval> | null = null;

  async function refresh() {
    try {
      const data = await api.listActiveStreams();
      setStreams(data.sessions);
      setLastRefresh(new Date());
      setError(null);
    } catch (e: any) {
      setError(e.message ?? 'Failed to load streams');
    } finally {
      setLoading(false);
    }
  }

  onMount(() => {
    refresh();
    intervalId = setInterval(refresh, 5000);
  });

  onCleanup(() => {
    if (intervalId) clearInterval(intervalId);
  });

  return (
    <div class="px-6 py-6 max-w-5xl mx-auto animate-fade-in">
      {/* Header */}
      <div class="flex items-center justify-between mb-8">
        <div class="flex items-center gap-3">
          <div class="w-10 h-10 rounded-xl bg-surface-200 flex items-center justify-center">
            <Activity class="w-5 h-5 text-surface-700" />
          </div>
          <div>
            <h1 class="text-xl font-bold text-white">Activity</h1>
            <p class="text-xs text-surface-700 mt-0.5">Active transcode sessions · refreshes every 5s</p>
          </div>
        </div>
        <button
          class="btn-icon text-surface-700 hover:text-white hover:bg-surface-200"
          onClick={refresh}
          title="Refresh now"
        >
          <RefreshCw class="w-4 h-4" />
        </button>
      </div>

      {/* Error */}
      <Show when={error()}>
        <div class="card p-4 mb-6 border border-red-500/20 bg-red-500/5">
          <p class="text-sm text-red-400">{error()}</p>
        </div>
      </Show>

      {/* Loading skeleton */}
      <Show when={loading()}>
        <div class="space-y-3">
          <For each={[1, 2, 3]}>{() =>
            <div class="card p-4 animate-pulse">
              <div class="h-4 bg-surface-200 rounded w-1/3 mb-2" />
              <div class="h-3 bg-surface-200 rounded w-1/2" />
            </div>
          }</For>
        </div>
      </Show>

      {/* Empty state */}
      <Show when={!loading() && streams().length === 0 && !error()}>
        <div class="card p-12 flex flex-col items-center gap-3 text-center">
          <WifiOff class="w-10 h-10 text-surface-600" />
          <p class="text-surface-700 text-sm">No active streams</p>
          <p class="text-surface-600 text-xs">Streams will appear here when someone is watching</p>
        </div>
      </Show>

      {/* Stream cards */}
      <Show when={!loading() && streams().length > 0}>
        <div class="mb-4 flex items-center gap-2">
          <div class="w-2 h-2 rounded-full bg-green-400 animate-pulse" />
          <span class="text-sm text-surface-700">
            {streams().length} active {streams().length === 1 ? 'stream' : 'streams'}
          </span>
          <Show when={lastRefresh()}>
            <span class="text-xs text-surface-600 ml-auto">
              Updated {lastRefresh()!.toLocaleTimeString()}
            </span>
          </Show>
        </div>

        <div class="space-y-3">
          <For each={streams()}>{(s) => (
            <div class="card p-4">
              <div class="flex items-start justify-between gap-4">
                {/* Left: media info */}
                <div class="flex items-center gap-3 min-w-0">
                  <div class="w-9 h-9 rounded-lg bg-ferrite-500/10 flex items-center justify-center flex-shrink-0">
                    <Monitor class="w-4 h-4 text-ferrite-400" />
                  </div>
                  <div class="min-w-0">
                    <p class="text-sm font-medium text-white truncate">
                      {s.media_id}
                    </p>
                    <div class="flex items-center gap-2 mt-0.5 flex-wrap">
                      <span class="text-xs text-surface-700">
                        {resLabel(s)}
                      </span>
                      <Show when={s.bitrate_kbps}>
                        <span class="text-surface-600 text-xs">·</span>
                        <span class="text-xs text-surface-700">
                          {s.bitrate_kbps! >= 1000
                            ? `${(s.bitrate_kbps! / 1000).toFixed(1)} Mbps`
                            : `${s.bitrate_kbps} kbps`}
                        </span>
                      </Show>
                      <span class="text-surface-600 text-xs">·</span>
                      <span class="text-xs text-surface-700">
                        at {fmtTime(s.start_secs)}
                      </span>
                    </div>
                  </div>
                </div>

                {/* Right: timing badges */}
                <div class="flex flex-col items-end gap-1 flex-shrink-0">
                  <div class="flex items-center gap-1.5 text-xs text-surface-700">
                    <Clock class="w-3 h-3" />
                    <span>Age: {fmtAge(s.age_secs)}</span>
                  </div>
                  <div class="flex items-center gap-1.5 text-xs">
                    <Wifi class={`w-3 h-3 ${s.idle_secs < 10 ? 'text-green-400' : s.idle_secs < 30 ? 'text-yellow-400' : 'text-red-400'}`} />
                    <span class={s.idle_secs < 10 ? 'text-green-400' : s.idle_secs < 30 ? 'text-yellow-400' : 'text-red-400'}>
                      Idle {fmtAge(s.idle_secs)}
                    </span>
                  </div>
                </div>
              </div>

              {/* Session ID */}
              <p class="mt-2 text-xs text-surface-600 font-mono truncate">
                {s.session_id}
              </p>
            </div>
          )}</For>
        </div>
      </Show>
    </div>
  );
}
