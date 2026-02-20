import { createSignal, createEffect, For, Show, onMount } from 'solid-js';
import { useNavigate, useSearchParams } from '@solidjs/router';
import { Search, Play, Star, SlidersHorizontal } from 'lucide-solid';
import { allMedia, loadMedia, libraries, loadLibraries } from '../stores/media';
import { authUrl } from '../api';
import type { MediaItem } from '../api';
import { getDisplayTitle, getDisplayYear, formatDuration, getResLabel, formatSize } from '../utils';

type ViewMode = 'grid' | 'list';
type SortKey = 'title-asc' | 'title-desc' | 'year-desc' | 'year-asc' | 'rating-desc' | 'added-desc' | 'played-desc';

export default function SearchPage() {
  const navigate = useNavigate();
  const [searchParams] = useSearchParams();

  // Derive query directly from the URL param so header-bar typing updates results live
  const query = () => searchParams.q || '';
  const [sort, setSort] = createSignal<SortKey>((localStorage.getItem('ferrite-sort') as SortKey) || 'title-asc');
  const [viewMode, setViewMode] = createSignal<ViewMode>((localStorage.getItem('ferrite-view') as ViewMode) || 'grid');
  const [showFilters, setShowFilters] = createSignal(false);
  const [results, setResults] = createSignal<MediaItem[]>([]);

  onMount(async () => {
    if (allMedia().length === 0) await loadMedia();
    if (libraries().length === 0) await loadLibraries();
  });

  createEffect(() => {
    const q = query().toLowerCase().trim();
    const s = sort();
    localStorage.setItem('ferrite-sort', s);
    localStorage.setItem('ferrite-view', viewMode());

    let items = allMedia();
    if (q) {
      items = items.filter(item => {
        const title = getDisplayTitle(item).toLowerCase();
        const overview = (item.overview || '').toLowerCase();
        const genres = (item.genres || '').toLowerCase();
        return title.includes(q) || overview.includes(q) || genres.includes(q);
      });
    }

    items = [...items].sort((a, b) => {
      switch (s) {
        case 'title-asc': return getDisplayTitle(a).localeCompare(getDisplayTitle(b));
        case 'title-desc': return getDisplayTitle(b).localeCompare(getDisplayTitle(a));
        case 'year-desc': return (getDisplayYear(b) || 0) - (getDisplayYear(a) || 0);
        case 'year-asc': return (getDisplayYear(a) || 0) - (getDisplayYear(b) || 0);
        case 'rating-desc': return (b.rating || 0) - (a.rating || 0);
        case 'added-desc': return (b.added_at || '').localeCompare(a.added_at || '');
        case 'played-desc': return (b.last_played_at || '').localeCompare(a.last_played_at || '');
        default: return 0;
      }
    });

    setResults(items);
  });

  return (
    <div class="px-6 py-6 animate-fade-in">
      {/* Header row */}
      <div class="flex items-center gap-3 mb-6">
        <div class="flex-1 min-w-0">
          <h1 class="text-xl font-semibold text-white truncate">
            <Show when={query()} fallback={<span class="text-surface-700">All media</span>}>
              Results for <span class="text-ferrite-400">&ldquo;{query()}&rdquo;</span>
            </Show>
          </h1>
        </div>
        <button
          class={`btn-icon flex-shrink-0 ${showFilters() ? 'bg-ferrite-500/15 text-ferrite-400' : ''}`}
          onClick={() => setShowFilters(!showFilters())}
          title="Sort &amp; View"
        >
          <SlidersHorizontal class="w-4.5 h-4.5" />
        </button>
      </div>

      {/* Filters bar */}
      <Show when={showFilters()}>
        <div class="flex items-center gap-4 mb-6 animate-slide-down">
          <div class="flex items-center gap-2">
            <label class="text-xs text-surface-700 font-medium">Sort:</label>
            <select
              class="input-field py-1.5 px-3 text-sm w-auto"
              value={sort()}
              onChange={(e) => setSort(e.currentTarget.value as SortKey)}
            >
              <option value="title-asc">Title A→Z</option>
              <option value="title-desc">Title Z→A</option>
              <option value="year-desc">Year (Newest)</option>
              <option value="year-asc">Year (Oldest)</option>
              <option value="rating-desc">Rating</option>
              <option value="added-desc">Recently Added</option>
              <option value="played-desc">Recently Played</option>
            </select>
          </div>

          <div class="flex items-center gap-1 ml-auto">
            <button
              class={`btn-icon w-8 h-8 ${viewMode() === 'grid' ? 'bg-white/10 text-white' : ''}`}
              onClick={() => setViewMode('grid')}
              title="Grid view"
            >
              <svg class="w-4 h-4" viewBox="0 0 16 16" fill="currentColor">
                <rect x="1" y="1" width="6" height="6" rx="1" />
                <rect x="9" y="1" width="6" height="6" rx="1" />
                <rect x="1" y="9" width="6" height="6" rx="1" />
                <rect x="9" y="9" width="6" height="6" rx="1" />
              </svg>
            </button>
            <button
              class={`btn-icon w-8 h-8 ${viewMode() === 'list' ? 'bg-white/10 text-white' : ''}`}
              onClick={() => setViewMode('list')}
              title="List view"
            >
              <svg class="w-4 h-4" viewBox="0 0 16 16" fill="currentColor">
                <rect x="1" y="2" width="14" height="2.5" rx="0.5" />
                <rect x="1" y="6.75" width="14" height="2.5" rx="0.5" />
                <rect x="1" y="11.5" width="14" height="2.5" rx="0.5" />
              </svg>
            </button>
          </div>
        </div>
      </Show>

      {/* Results count */}
      <div class="text-sm text-surface-700 mb-4">
        {results().length} {results().length === 1 ? 'item' : 'items'}
        {query() ? ` matching "${query()}"` : ''}
      </div>

      {/* Grid view */}
      <Show when={viewMode() === 'grid'}>
        <div class="grid grid-cols-[repeat(auto-fill,minmax(160px,1fr))] gap-4">
          <For each={results()}>
            {(item) => <GridCard item={item} onClick={() => navigate(`/media/${item.id}`)} />}
          </For>
        </div>
      </Show>

      {/* List view */}
      <Show when={viewMode() === 'list'}>
        <div class="space-y-1">
          <For each={results()}>
            {(item) => <ListRow item={item} onClick={() => navigate(`/media/${item.id}`)} />}
          </For>
        </div>
      </Show>

      {/* Empty state */}
      <Show when={results().length === 0 && query()}>
        <div class="flex flex-col items-center py-16 text-center">
          <Search class="w-12 h-12 text-surface-500 mb-4" />
          <h3 class="text-lg font-medium text-gray-400 mb-1">No results found</h3>
          <p class="text-sm text-surface-700">Try a different search term</p>
        </div>
      </Show>
    </div>
  );
}

function GridCard(props: { item: MediaItem; onClick: () => void }) {
  const title = () => getDisplayTitle(props.item);
  const year = () => getDisplayYear(props.item);
  const posterUrl = () => props.item.poster_path ? authUrl(`/api/images/${props.item.poster_path}`) : null;
  const resLabel = () => getResLabel(props.item.width, props.item.height);
  const progressPct = () => {
    if (!props.item.position_ms || !props.item.duration_ms) return 0;
    return Math.min(100, (props.item.position_ms / props.item.duration_ms) * 100);
  };

  return (
    <div class="cursor-pointer group/card" onClick={props.onClick}>
      <div class="relative aspect-[2/3] rounded-xl overflow-hidden bg-surface-200 mb-2
                  ring-1 ring-white/5 group-hover/card:ring-ferrite-500/30 transition-all duration-250">
        <Show when={posterUrl()} fallback={
          <div class="w-full h-full flex items-center justify-center">
            <Play class="w-8 h-8 text-surface-500" />
          </div>
        }>
          <img src={posterUrl()!} alt={title()} class="w-full h-full object-cover transition-transform duration-300 group-hover/card:scale-105" loading="lazy" />
        </Show>
        <div class="absolute inset-0 bg-black/0 group-hover/card:bg-black/40 transition-all duration-250 flex items-center justify-center">
          <div class="w-12 h-12 rounded-full bg-ferrite-500/90 flex items-center justify-center opacity-0 group-hover/card:opacity-100 scale-75 group-hover/card:scale-100 transition-all duration-250 shadow-lg shadow-ferrite-500/30">
            <Play class="w-5 h-5 text-white fill-white ml-0.5" />
          </div>
        </div>
        <Show when={resLabel()}>
          <span class="absolute top-2 right-2 badge bg-black/60 text-white backdrop-blur-sm text-2xs">{resLabel()}</span>
        </Show>
        <Show when={progressPct() > 0}>
          <div class="absolute bottom-0 left-0 right-0 h-1 bg-black/50">
            <div class="h-full bg-ferrite-500" style={{ width: `${progressPct()}%` }} />
          </div>
        </Show>
      </div>
      <h3 class="text-sm font-medium text-gray-300 group-hover/card:text-white truncate transition-colors">{title()}</h3>
      <Show when={year()}><p class="text-xs text-surface-700">{year()}</p></Show>
    </div>
  );
}

function ListRow(props: { item: MediaItem; onClick: () => void }) {
  const title = () => getDisplayTitle(props.item);
  const year = () => getDisplayYear(props.item);
  const posterUrl = () => props.item.poster_path ? authUrl(`/api/images/${props.item.poster_path}`) : null;

  return (
    <div
      class="flex items-center gap-4 px-4 py-3 rounded-xl hover:bg-surface-100 cursor-pointer transition-colors group"
      onClick={props.onClick}
    >
      <div class="w-10 h-14 rounded-lg overflow-hidden bg-surface-200 flex-shrink-0">
        <Show when={posterUrl()} fallback={<div class="w-full h-full flex items-center justify-center"><Play class="w-4 h-4 text-surface-500" /></div>}>
          <img src={posterUrl()!} alt="" class="w-full h-full object-cover" loading="lazy" />
        </Show>
      </div>
      <div class="flex-1 min-w-0">
        <h3 class="text-sm font-medium text-gray-300 group-hover:text-white truncate transition-colors">{title()}</h3>
        <div class="flex items-center gap-2 text-xs text-surface-700 mt-0.5">
          <Show when={year()}><span>{year()}</span></Show>
          <Show when={props.item.duration_ms}><span>{formatDuration(props.item.duration_ms)}</span></Show>
          <Show when={props.item.rating}>
            <span class="flex items-center gap-0.5"><Star class="w-3 h-3 text-yellow-500 fill-yellow-500" />{props.item.rating!.toFixed(1)}</span>
          </Show>
        </div>
      </div>
      <div class="text-xs text-surface-600 hidden md:block">{formatSize(props.item.file_size)}</div>
      <div class="text-xs text-surface-600 hidden lg:block">{getResLabel(props.item.width, props.item.height)}</div>
    </div>
  );
}
