import { createSignal, createEffect, For, Show, onMount } from 'solid-js';
import { useParams, useNavigate } from '@solidjs/router';
import { Play, FolderOpen, RefreshCw, Trash2, Filter, X, ChevronLeft, ChevronRight } from 'lucide-solid';
import { libraries, loadLibraries, deleteLibrary, refreshAll } from '../stores/media';
import { api, authUrl } from '../api';
import type { MediaItem, Library } from '../api';
import { getDisplayTitle, getDisplayYear, getResLabel } from '../utils';

const PER_PAGE = 50;

export default function LibraryPage() {
  const params = useParams<{ id: string }>();
  const navigate = useNavigate();

  const [sort, setSort] = createSignal<string>(localStorage.getItem('ferrite-lib-sort') || 'title-asc');
  const [items, setItems] = createSignal<MediaItem[]>([]);
  const [total, setTotal] = createSignal(0);
  const [page, setPage] = createSignal(1);
  const [loading, setLoading] = createSignal(false);
  const [filterGenre, setFilterGenre] = createSignal<string | null>(null);
  const [showFilters, setShowFilters] = createSignal(false);

  const totalPages = () => Math.max(1, Math.ceil(total() / PER_PAGE));
  const hasActiveFilters = () => filterGenre() !== null;
  const library = (): Library | undefined => libraries().find(l => l.id === params.id);

  function clearFilters() {
    setFilterGenre(null);
  }

  async function fetchPage() {
    setLoading(true);
    try {
      const [sortBy, sortDir] = sort().split('-');
      const params_: Record<string, string> = {
        library_id: params.id,
        page: String(page()),
        per_page: String(PER_PAGE),
        sort: sortBy,
        dir: sortDir,
      };
      if (filterGenre()) params_['genre'] = filterGenre()!;
      const data = await api.listMedia(params_);
      setItems(data.items);
      setTotal(data.total);
    } catch { /* ignore */ }
    setLoading(false);
  }

  onMount(async () => {
    if (libraries().length === 0) await loadLibraries();
    const lib = libraries().find(l => l.id === params.id);
    if (lib?.library_type === 'tv') {
      navigate(`/shows/library/${params.id}`, { replace: true });
      return;
    }
    await fetchPage();
  });

  createEffect(() => {
    localStorage.setItem('ferrite-lib-sort', sort());
    setPage(1);
    fetchPage();
  });

  createEffect(() => {
    void filterGenre();
    setPage(1);
    fetchPage();
  });

  createEffect(() => {
    void page();
    fetchPage();
  });

  async function handleDelete() {
    const lib = library();
    if (!lib) return;
    if (!confirm(`Delete library "${lib.name}"? Media files on disk will not be affected.`)) return;
    await deleteLibrary(lib.id);
    navigate('/');
  }

  return (
    <div class="px-6 py-6 animate-fade-in">
      {/* Header */}
      <div class="flex items-center justify-between mb-6">
        <div class="flex items-center gap-3">
          <div class="w-10 h-10 rounded-xl bg-surface-200 flex items-center justify-center">
            <FolderOpen class="w-5 h-5 text-surface-700" />
          </div>
          <div>
            <h1 class="text-xl font-bold text-white">{library()?.name || 'Library'}</h1>
            <p class="text-sm text-surface-700">{total()} items</p>
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
            <option value="rating-desc">Rating</option>
            <option value="added-desc">Recently Added</option>
          </select>

          <button
            class={`btn-ghost ${hasActiveFilters() ? 'text-ferrite-400' : ''}`}
            onClick={() => setShowFilters(v => !v)}
            title="Filters"
          >
            <Filter class="w-4 h-4" />
            <Show when={hasActiveFilters()}>
              <span class="ml-1 text-xs font-semibold">On</span>
            </Show>
          </button>

          <button class="btn-ghost" onClick={async () => { await refreshAll(); await fetchPage(); }} title="Refresh library">
            <RefreshCw class="w-4 h-4" />
          </button>
          <button class="btn-ghost text-red-400 hover:text-red-300" onClick={handleDelete} title="Delete library">
            <Trash2 class="w-4 h-4" />
          </button>
        </div>
      </div>

      {/* Filter bar */}
      <Show when={showFilters()}>
        <div class="mb-5 p-4 rounded-xl bg-surface-100 border border-surface-300/50 space-y-3">
          <div class="flex items-center justify-between">
            <span class="text-xs font-semibold text-surface-700 uppercase tracking-wider">Filters</span>
            <Show when={hasActiveFilters()}>
              <button class="flex items-center gap-1 text-xs text-ferrite-400 hover:text-ferrite-300" onClick={clearFilters}>
                <X class="w-3 h-3" /> Clear all
              </button>
            </Show>
          </div>
          <div>
            <p class="text-xs text-surface-600 mb-2">Genre</p>
            <input
              class="input-field py-1 px-2 text-sm w-48"
              placeholder="e.g. Action"
              value={filterGenre() ?? ''}
              onInput={(e) => setFilterGenre(e.currentTarget.value || null)}
            />
          </div>
        </div>
      </Show>

      {/* Grid */}
      <Show when={loading()}>
        <div class="flex justify-center py-16">
          <div class="w-8 h-8 border-2 border-ferrite-500/30 border-t-ferrite-500 rounded-full animate-spin" />
        </div>
      </Show>

      <Show when={!loading()}>
        <div class="grid grid-cols-[repeat(auto-fill,minmax(160px,1fr))] gap-4">
          <For each={items()}>
            {(item) => (
              <div class="cursor-pointer group/card" onClick={() => navigate(`/media/${item.id}`)}>
                <div class="relative aspect-[2/3] rounded-xl overflow-hidden bg-surface-200 mb-2 ring-1 ring-white/5 group-hover/card:ring-ferrite-500/30 transition-all duration-250">
                  <Show when={item.poster_path} fallback={
                    <div class="w-full h-full flex items-center justify-center"><Play class="w-8 h-8 text-surface-500" /></div>
                  }>
                    <img src={authUrl(`/api/images/${item.poster_path}`)} alt={getDisplayTitle(item)} class="w-full h-full object-cover transition-transform duration-300 group-hover/card:scale-105" loading="lazy" />
                  </Show>
                  <div class="absolute inset-0 bg-black/0 group-hover/card:bg-black/40 transition-all duration-250 flex items-center justify-center">
                    <div class="w-12 h-12 rounded-full bg-ferrite-500/90 flex items-center justify-center opacity-0 group-hover/card:opacity-100 scale-75 group-hover/card:scale-100 transition-all duration-250">
                      <Play class="w-5 h-5 text-white fill-white ml-0.5" />
                    </div>
                  </div>
                  <Show when={getResLabel(item.width, item.height)}>
                    <span class="absolute top-2 right-2 badge bg-black/60 text-white backdrop-blur-sm text-2xs">{getResLabel(item.width, item.height)}</span>
                  </Show>
                  <Show when={item.position_ms && item.duration_ms && !item.completed}>
                    <div class="absolute bottom-0 left-0 right-0 h-1 bg-black/50">
                      <div class="h-full bg-ferrite-500" style={{ width: `${Math.min(100, (item.position_ms! / item.duration_ms!) * 100)}%` }} />
                    </div>
                  </Show>
                </div>
                <h3 class="text-sm font-medium text-gray-300 group-hover/card:text-white truncate transition-colors">{getDisplayTitle(item)}</h3>
                <Show when={getDisplayYear(item)}><p class="text-xs text-surface-700">{getDisplayYear(item)}</p></Show>
              </div>
            )}
          </For>
        </div>

        <Show when={items().length === 0}>
          <div class="flex flex-col items-center py-16 text-center">
            <FolderOpen class="w-12 h-12 text-surface-500 mb-4" />
            <h3 class="text-lg font-medium text-gray-400 mb-1">Library is empty</h3>
            <p class="text-sm text-surface-700">Try refreshing to scan for new media</p>
          </div>
        </Show>

        {/* Pagination */}
        <Show when={totalPages() > 1}>
          <div class="flex items-center justify-center gap-2 mt-8">
            <button
              class="btn-ghost py-1.5 px-3 text-sm"
              disabled={page() <= 1}
              onClick={() => setPage(p => p - 1)}
            >
              <ChevronLeft class="w-4 h-4" />
            </button>
            <span class="text-sm text-surface-700">
              Page {page()} of {totalPages()}
            </span>
            <button
              class="btn-ghost py-1.5 px-3 text-sm"
              disabled={page() >= totalPages()}
              onClick={() => setPage(p => p + 1)}
            >
              <ChevronRight class="w-4 h-4" />
            </button>
          </div>
        </Show>
      </Show>
    </div>
  );
}
