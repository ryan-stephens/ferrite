import { createSignal, onCleanup, Show, For } from 'solid-js';
import { perf } from '../lib/perf';
import type { PerfSpan } from '../lib/perf';

/**
 * Toggleable performance overlay for the video player.
 * Shows real-time timing metrics for playback operations.
 * Toggle with the `P` key.
 */
export default function PerfOverlay(props: { visible: boolean }) {
  const [entries, setEntries] = createSignal(perf.getEntries());
  const [tab, setTab] = createSignal<'timeline' | 'summary'>('timeline');

  const unsub = perf.subscribe(() => {
    setEntries(perf.getEntries());
  });

  onCleanup(unsub);

  const spans = () =>
    entries().filter(
      (e): e is PerfSpan => 'startMs' in e && (e as PerfSpan).durationMs !== null,
    ) as PerfSpan[];

  const summary = () => perf.getSummary();

  const categoryColor = (cat: string): string => {
    switch (cat) {
      case 'frontend':
        return 'text-blue-400';
      case 'backend':
        return 'text-amber-400';
      case 'network':
        return 'text-green-400';
      default:
        return 'text-gray-400';
    }
  };

  const durationColor = (ms: number): string => {
    if (ms < 100) return 'text-green-400';
    if (ms < 500) return 'text-yellow-400';
    if (ms < 2000) return 'text-orange-400';
    return 'text-red-400';
  };

  return (
    <Show when={props.visible}>
      <div
        class="absolute top-14 right-4 z-50 w-[420px] max-h-[70vh] rounded-xl bg-black/85 backdrop-blur-xl border border-white/10 shadow-2xl shadow-black/60 overflow-hidden font-mono text-xs select-text"
        onClick={(e) => e.stopPropagation()}
      >
        {/* Header */}
        <div class="flex items-center justify-between px-4 py-2.5 border-b border-white/10">
          <div class="flex items-center gap-2">
            <div class="w-2 h-2 rounded-full bg-green-500 animate-pulse" />
            <span class="text-white/90 font-semibold text-xs uppercase tracking-wider">
              Perf Monitor
            </span>
          </div>
          <div class="flex items-center gap-1">
            <button
              class={`px-2.5 py-1 rounded text-[10px] uppercase tracking-wider font-semibold transition-colors ${
                tab() === 'timeline'
                  ? 'bg-white/15 text-white'
                  : 'text-white/50 hover:text-white/80'
              }`}
              onClick={() => setTab('timeline')}
            >
              Timeline
            </button>
            <button
              class={`px-2.5 py-1 rounded text-[10px] uppercase tracking-wider font-semibold transition-colors ${
                tab() === 'summary'
                  ? 'bg-white/15 text-white'
                  : 'text-white/50 hover:text-white/80'
              }`}
              onClick={() => setTab('summary')}
            >
              Summary
            </button>
            <button
              class="ml-2 px-2 py-1 rounded text-[10px] uppercase tracking-wider text-red-400 hover:bg-red-500/20 transition-colors"
              onClick={() => perf.reset()}
            >
              Clear
            </button>
          </div>
        </div>

        {/* Content */}
        <div class="overflow-y-auto max-h-[calc(70vh-44px)] scrollbar-thin">
          <Show when={tab() === 'timeline'}>
            <Show
              when={spans().length > 0}
              fallback={
                <div class="px-4 py-8 text-center text-white/30">
                  No timing data yet. Play, seek, or resume to see metrics.
                </div>
              }
            >
              <table class="w-full">
                <thead>
                  <tr class="text-white/40 text-[10px] uppercase tracking-wider">
                    <th class="text-left px-4 py-2 font-medium">Operation</th>
                    <th class="text-left px-2 py-2 font-medium">Source</th>
                    <th class="text-right px-4 py-2 font-medium">Duration</th>
                  </tr>
                </thead>
                <tbody>
                  <For each={spans().slice(0, 50)}>
                    {(span) => (
                      <tr class="border-t border-white/5 hover:bg-white/5 transition-colors">
                        <td class="px-4 py-1.5 text-white/80 truncate max-w-[200px]">
                          {span.label}
                          <Show when={span.meta}>
                            <span class="text-white/30 ml-1">
                              {Object.entries(span.meta!)
                                .map(([k, v]) => `${k}=${v}`)
                                .join(' ')}
                            </span>
                          </Show>
                        </td>
                        <td class={`px-2 py-1.5 ${categoryColor(span.category)}`}>
                          {span.category}
                        </td>
                        <td
                          class={`px-4 py-1.5 text-right tabular-nums font-semibold ${durationColor(
                            span.durationMs!,
                          )}`}
                        >
                          {span.durationMs! < 1
                            ? '<1ms'
                            : span.durationMs! >= 1000
                              ? `${(span.durationMs! / 1000).toFixed(2)}s`
                              : `${Math.round(span.durationMs!)}ms`}
                        </td>
                      </tr>
                    )}
                  </For>
                </tbody>
              </table>
            </Show>
          </Show>

          <Show when={tab() === 'summary'}>
            <Show
              when={summary().length > 0}
              fallback={
                <div class="px-4 py-8 text-center text-white/30">
                  No timing data yet.
                </div>
              }
            >
              <table class="w-full">
                <thead>
                  <tr class="text-white/40 text-[10px] uppercase tracking-wider">
                    <th class="text-left px-4 py-2 font-medium">Operation</th>
                    <th class="text-right px-2 py-2 font-medium">Count</th>
                    <th class="text-right px-2 py-2 font-medium">Avg</th>
                    <th class="text-right px-4 py-2 font-medium">Last</th>
                  </tr>
                </thead>
                <tbody>
                  <For each={summary()}>
                    {(row) => (
                      <tr class="border-t border-white/5 hover:bg-white/5 transition-colors">
                        <td class="px-4 py-1.5">
                          <span class="text-white/80">{row.label}</span>
                          <span class={`ml-1.5 ${categoryColor(row.category)}`}>
                            [{row.category.charAt(0)}]
                          </span>
                        </td>
                        <td class="px-2 py-1.5 text-right text-white/50 tabular-nums">
                          {row.count}×
                        </td>
                        <td
                          class={`px-2 py-1.5 text-right tabular-nums ${durationColor(
                            row.avgMs,
                          )}`}
                        >
                          {row.avgMs}ms
                        </td>
                        <td
                          class={`px-4 py-1.5 text-right tabular-nums font-semibold ${durationColor(
                            row.lastMs,
                          )}`}
                        >
                          {row.lastMs >= 1000
                            ? `${(row.lastMs / 1000).toFixed(2)}s`
                            : `${row.lastMs}ms`}
                        </td>
                      </tr>
                    )}
                  </For>
                </tbody>
              </table>
            </Show>
          </Show>
        </div>

        {/* Footer legend */}
        <div class="flex items-center gap-4 px-4 py-2 border-t border-white/10 text-[10px]">
          <span class="text-blue-400">● frontend</span>
          <span class="text-amber-400">● backend</span>
          <span class="text-green-400">● network</span>
          <span class="text-white/30 ml-auto">Press P to toggle</span>
        </div>
      </div>
    </Show>
  );
}
