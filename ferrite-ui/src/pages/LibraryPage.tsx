import { createSignal, createEffect, createMemo, For, Show, onMount } from 'solid-js';
import { useParams, useNavigate } from '@solidjs/router';
import { Play, Star, FolderOpen, RefreshCw, Trash2, Filter, X } from 'lucide-solid';
import { allMedia, libraries, loadMedia, loadLibraries, deleteLibrary, refreshAll } from '../stores/media';
import { authUrl } from '../api';
import type { MediaItem, Library } from '../api';
import { getDisplayTitle, getDisplayYear, formatDuration, getResLabel } from '../utils';

export default function LibraryPage() {
  const params = useParams<{ id: string }>();
  const navigate = useNavigate();

  const [sort, setSort] = createSignal<string>(localStorage.getItem('ferrite-lib-sort') || 'title-asc');
  const [items, setItems] = createSignal<MediaItem[]>([]);
  const [filterGenre, setFilterGenre] = createSignal<string | null>(null);
  const [filterMinRating, setFilterMinRating] = createSignal<number>(0);
  const [filterMinYear, setFilterMinYear] = createSignal<number>(0);
  const [filterMaxYear, setFilterMaxYear] = createSignal<number>(0);
  const [showFilters, setShowFilters] = createSignal(false);

  const libItems = createMemo(() => allMedia().filter(i => i.library_id === params.id));

  const allGenres = createMemo(() => {
    const set = new Set<string>();
    libItems().forEach(item => {
      if (item.genres) item.genres.split(',').forEach(g => { const t = g.trim(); if (t) set.add(t); });
    });
    return [...set].sort();
  });

  const yearRange = createMemo(() => {
    const years = libItems().map(i => getDisplayYear(i)).filter(Boolean) as number[];
    if (years.length === 0) return { min: 1900, max: new Date().getFullYear() };
    return { min: Math.min(...years), max: Math.max(...years) };
  });

  const hasActiveFilters = createMemo(() =>
    filterGenre() !== null || filterMinRating() > 0 || filterMinYear() > 0 || filterMaxYear() > 0
  );

  function clearFilters() {
    setFilterGenre(null);
    setFilterMinRating(0);
    setFilterMinYear(0);
    setFilterMaxYear(0);
  }

  const library = (): Library | undefined => libraries().find(l => l.id === params.id);

  onMount(async () => {
    if (libraries().length === 0) await loadLibraries();
    // Redirect TV libraries to the shows page
    const lib = libraries().find(l => l.id === params.id);
    if (lib?.library_type === 'tv') {
      navigate(`/shows/library/${params.id}`, { replace: true });
      return;
    }
    if (allMedia().length === 0) await loadMedia();
  });

  createEffect(() => {
    const s = sort();
    localStorage.setItem('ferrite-lib-sort', s);

    const genre = filterGenre();
    const minRating = filterMinRating();
    const minYear = filterMinYear();
    const maxYear = filterMaxYear();

    let filtered = libItems().filter(item => {
      if (genre) {
        const genres = item.genres?.split(',').map(g => g.trim()) ?? [];
        if (!genres.includes(genre)) return false;
      }
      if (minRating > 0 && (item.rating == null || item.rating < minRating)) return false;
      const y = getDisplayYear(item);
      if (minYear > 0 && (y == null || y < minYear)) return false;
      if (maxYear > 0 && (y == null || y > maxYear)) return false;
      return true;
    });
    filtered = [...filtered].sort((a, b) => {
      switch (s) {
        case 'title-asc': return getDisplayTitle(a).localeCompare(getDisplayTitle(b));
        case 'title-desc': return getDisplayTitle(b).localeCompare(getDisplayTitle(a));
        case 'year-desc': return (getDisplayYear(b) || 0) - (getDisplayYear(a) || 0);
        case 'year-asc': return (getDisplayYear(a) || 0) - (getDisplayYear(b) || 0);
        case 'rating-desc': return (b.rating || 0) - (a.rating || 0);
        case 'added-desc': return (b.added_at || '').localeCompare(a.added_at || '');
        default: return 0;
      }
    });
    setItems(filtered);
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
            <p class="text-sm text-surface-700">{items().length} items</p>
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

          <button class="btn-ghost" onClick={refreshAll} title="Refresh library">
            <RefreshCw class="w-4 h-4" />
          </button>
          <button class="btn-ghost text-red-400 hover:text-red-300" onClick={handleDelete} title="Delete library">
            <Trash2 class="w-4 h-4" />
          </button>
        </div>
      </div>

      {/* Filter bar */}
      <Show when={showFilters()}>
        <div class="mb-5 p-4 rounded-xl bg-surface-100 border border-surface-300/50 space-y-4">
          <div class="flex items-center justify-between">
            <span class="text-xs font-semibold text-surface-700 uppercase tracking-wider">Filters</span>
            <Show when={hasActiveFilters()}>
              <button class="flex items-center gap-1 text-xs text-ferrite-400 hover:text-ferrite-300" onClick={clearFilters}>
                <X class="w-3 h-3" /> Clear all
              </button>
            </Show>
          </div>

          {/* Genre chips */}
          <Show when={allGenres().length > 0}>
            <div>
              <p class="text-xs text-surface-600 mb-2">Genre</p>
              <div class="flex flex-wrap gap-1.5">
                <button
                  class={`px-2.5 py-1 rounded-full text-xs font-medium transition-colors ${filterGenre() === null ? 'bg-ferrite-500 text-white' : 'bg-surface-200 text-surface-800 hover:bg-surface-300'}`}
                  onClick={() => setFilterGenre(null)}
                >All</button>
                <For each={allGenres()}>{(g) =>
                  <button
                    class={`px-2.5 py-1 rounded-full text-xs font-medium transition-colors ${filterGenre() === g ? 'bg-ferrite-500 text-white' : 'bg-surface-200 text-surface-800 hover:bg-surface-300'}`}
                    onClick={() => setFilterGenre(filterGenre() === g ? null : g)}
                  >{g}</button>
                }</For>
              </div>
            </div>
          </Show>

          <div class="flex flex-wrap gap-6">
            {/* Min rating */}
            <div>
              <p class="text-xs text-surface-600 mb-2">Min Rating</p>
              <div class="flex items-center gap-1.5">
                <For each={[0, 5, 6, 7, 8]}>{(r) =>
                  <button
                    class={`px-2.5 py-1 rounded-full text-xs font-medium transition-colors ${filterMinRating() === r ? 'bg-ferrite-500 text-white' : 'bg-surface-200 text-surface-800 hover:bg-surface-300'}`}
                    onClick={() => setFilterMinRating(r)}
                  >{r === 0 ? 'Any' : `${r}+`}</button>
                }</For>
              </div>
            </div>

            {/* Year range */}
            <Show when={yearRange().min !== yearRange().max}>
              <div>
                <p class="text-xs text-surface-600 mb-2">Year</p>
                <div class="flex items-center gap-2">
                  <input
                    type="number"
                    class="input-field py-1 px-2 text-xs w-20"
                    placeholder={String(yearRange().min)}
                    value={filterMinYear() || ''}
                    onInput={(e) => setFilterMinYear(parseInt(e.currentTarget.value) || 0)}
                  />
                  <span class="text-surface-600 text-xs">–</span>
                  <input
                    type="number"
                    class="input-field py-1 px-2 text-xs w-20"
                    placeholder={String(yearRange().max)}
                    value={filterMaxYear() || ''}
                    onInput={(e) => setFilterMaxYear(parseInt(e.currentTarget.value) || 0)}
                  />
                </div>
              </div>
            </Show>
          </div>
        </div>
      </Show>

      {/* Grid */}
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
    </div>
  );
}
