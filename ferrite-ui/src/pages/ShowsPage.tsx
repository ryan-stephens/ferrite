import { createSignal, createEffect, For, Show, onMount } from 'solid-js';
import { useParams, useNavigate } from '@solidjs/router';
import { Tv, FolderOpen, RefreshCw, Trash2 } from 'lucide-solid';
import { libraries, loadLibraries, deleteLibrary, refreshAll } from '../stores/media';
import { api, authUrl } from '../api';
import type { TvShow, Library } from '../api';

export default function ShowsPage() {
  const params = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [shows, setShows] = createSignal<TvShow[]>([]);
  const [loading, setLoading] = createSignal(true);
  const [sort, setSort] = createSignal<string>(localStorage.getItem('ferrite-shows-sort') || 'title-asc');

  const library = (): Library | undefined => libraries().find(l => l.id === params.id);

  onMount(async () => {
    if (libraries().length === 0) await loadLibraries();
    await loadShows();
  });

  async function loadShows() {
    setLoading(true);
    try {
      const data = await api.listShows(params.id);
      setShows(data);
    } catch {
      setShows([]);
    } finally {
      setLoading(false);
    }
  }

  const sortedShows = () => {
    const s = sort();
    localStorage.setItem('ferrite-shows-sort', s);
    return [...shows()].sort((a, b) => {
      switch (s) {
        case 'title-asc': return (a.sort_title || a.title).localeCompare(b.sort_title || b.title);
        case 'title-desc': return (b.sort_title || b.title).localeCompare(a.sort_title || a.title);
        case 'year-desc': return (b.year || 0) - (a.year || 0);
        case 'year-asc': return (a.year || 0) - (b.year || 0);
        default: return 0;
      }
    });
  };

  async function handleDelete() {
    const lib = library();
    if (!lib) return;
    if (!confirm(`Delete library "${lib.name}"? Media files on disk will not be affected.`)) return;
    await deleteLibrary(lib.id);
    navigate('/');
  }

  async function handleRefresh() {
    await refreshAll();
    await loadShows();
  }

  return (
    <div class="px-6 py-6 animate-fade-in">
      {/* Header */}
      <div class="flex items-center justify-between mb-6">
        <div class="flex items-center gap-3">
          <div class="w-10 h-10 rounded-xl bg-surface-200 flex items-center justify-center">
            <Tv class="w-5 h-5 text-surface-700" />
          </div>
          <div>
            <h1 class="text-xl font-bold text-white">{library()?.name || 'TV Shows'}</h1>
            <p class="text-sm text-surface-700">{shows().length} shows</p>
          </div>
        </div>

        <div class="flex items-center gap-2">
          <select
            class="input-field py-1.5 px-3 text-sm w-auto"
            value={sort()}
            onChange={(e) => setSort(e.currentTarget.value)}
          >
            <option value="title-asc">Title A→Z</option>
            <option value="title-desc">Title Z→A</option>
            <option value="year-desc">Year (Newest)</option>
            <option value="year-asc">Year (Oldest)</option>
          </select>
          <button class="btn-ghost" onClick={handleRefresh} title="Refresh library">
            <RefreshCw class="w-4 h-4" />
          </button>
          <button class="btn-ghost text-red-400 hover:text-red-300" onClick={handleDelete} title="Delete library">
            <Trash2 class="w-4 h-4" />
          </button>
        </div>
      </div>

      {/* Loading */}
      <Show when={loading()}>
        <div class="flex items-center justify-center py-16">
          <div class="w-8 h-8 border-2 border-surface-400 border-t-ferrite-500 rounded-full animate-spin" />
        </div>
      </Show>

      {/* Show grid */}
      <Show when={!loading()}>
        <div class="grid grid-cols-[repeat(auto-fill,minmax(160px,1fr))] gap-4">
          <For each={sortedShows()}>
            {(show) => (
              <div
                class="cursor-pointer group/card"
                onClick={() => navigate(`/shows/${show.id}`)}
              >
                <div class="relative aspect-[2/3] rounded-xl overflow-hidden bg-surface-200 mb-2 ring-1 ring-white/5 group-hover/card:ring-ferrite-500/30 transition-all duration-250">
                  <Show when={show.poster_path} fallback={
                    <div class="w-full h-full flex flex-col items-center justify-center gap-2 p-3">
                      <Tv class="w-8 h-8 text-surface-500" />
                      <span class="text-2xs text-surface-600 text-center leading-tight">{show.title}</span>
                    </div>
                  }>
                    <img
                      src={authUrl(`/api/images/${show.poster_path}`)}
                      alt={show.title}
                      class="w-full h-full object-cover transition-transform duration-300 group-hover/card:scale-105"
                      loading="lazy"
                    />
                  </Show>
                  <div class="absolute inset-0 bg-black/0 group-hover/card:bg-black/30 transition-all duration-250" />
                  {/* Episode/season count badge */}
                  <div class="absolute bottom-2 left-2 right-2 flex gap-1">
                    <span class="badge bg-black/70 text-white backdrop-blur-sm text-2xs">
                      {show.season_count} {show.season_count === 1 ? 'Season' : 'Seasons'}
                    </span>
                  </div>
                </div>
                <h3 class="text-sm font-medium text-gray-300 group-hover/card:text-white truncate transition-colors">
                  {show.title}
                </h3>
                <Show when={show.year}>
                  <p class="text-xs text-surface-700">{show.year}</p>
                </Show>
              </div>
            )}
          </For>
        </div>

        <Show when={shows().length === 0}>
          <div class="flex flex-col items-center py-16 text-center">
            <FolderOpen class="w-12 h-12 text-surface-500 mb-4" />
            <h3 class="text-lg font-medium text-gray-400 mb-1">No TV shows found</h3>
            <p class="text-sm text-surface-700">Try refreshing to scan for new media</p>
          </div>
        </Show>
      </Show>
    </div>
  );
}
