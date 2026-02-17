import { createSignal, createEffect, For, Show, onMount } from 'solid-js';
import { useNavigate } from '@solidjs/router';
import { Play, Clock, ChevronLeft, ChevronRight, Star, Info } from 'lucide-solid';
import { allMedia, libraries, loadMedia, loadLibraries } from '../stores/media';
import { authUrl } from '../api';
import type { MediaItem } from '../api';
import { getDisplayTitle, getDisplayYear, formatDuration, getResLabel, getStreamType } from '../utils';

export default function HomePage() {
  const navigate = useNavigate();

  onMount(async () => {
    if (libraries().length === 0) await loadLibraries();
    if (allMedia().length === 0) await loadMedia();
  });

  // Continue watching: items with progress, not completed
  const continueWatching = () =>
    allMedia()
      .filter(i => i.position_ms && i.position_ms > 0 && !i.completed && i.last_played_at)
      .sort((a, b) => (b.last_played_at || '').localeCompare(a.last_played_at || ''))
      .slice(0, 20);

  // Recently added
  const recentlyAdded = () =>
    [...allMedia()]
      .sort((a, b) => (b.added_at || '').localeCompare(a.added_at || ''))
      .slice(0, 20);

  // Hero item: most recently added with a poster
  const heroItem = () => {
    const items = allMedia().filter(i => i.poster_path);
    return items.length > 0
      ? [...items].sort((a, b) => (b.added_at || '').localeCompare(a.added_at || ''))[0]
      : null;
  };

  return (
    <div class="animate-fade-in">
      {/* Hero Banner */}
      <Show when={heroItem()}>
        {(item) => <HeroBanner item={item()} onPlay={() => navigate(`/player/${item().id}`)} onInfo={() => navigate(`/media/${item().id}`)} />}
      </Show>

      <div class="px-6 py-6 space-y-8">
        {/* Continue Watching */}
        <Show when={continueWatching().length > 0}>
          <MediaRow
            title="Continue Watching"
            items={continueWatching()}
            onItemClick={(id) => navigate(`/media/${id}`)}
            showProgress
          />
        </Show>

        {/* Recently Added */}
        <Show when={recentlyAdded().length > 0}>
          <MediaRow
            title="Recently Added"
            items={recentlyAdded()}
            onItemClick={(id) => navigate(`/media/${id}`)}
          />
        </Show>

        {/* Per-library rows */}
        <For each={libraries()}>
          {(lib) => {
            const libItems = () =>
              allMedia()
                .filter(i => i.library_id === lib.id)
                .sort((a, b) => (b.added_at || '').localeCompare(a.added_at || ''))
                .slice(0, 20);
            return (
              <Show when={libItems().length > 0}>
                <MediaRow
                  title={lib.name}
                  items={libItems()}
                  onItemClick={(id) => navigate(`/media/${id}`)}
                  onSeeAll={() => navigate(`/library/${lib.id}`)}
                />
              </Show>
            );
          }}
        </For>

        {/* Empty state */}
        <Show when={allMedia().length === 0 && !libraries().length}>
          <div class="flex flex-col items-center justify-center py-24 text-center">
            <div class="w-16 h-16 rounded-2xl bg-surface-200 flex items-center justify-center mb-4">
              <Play class="w-7 h-7 text-surface-600" />
            </div>
            <h2 class="text-xl font-semibold text-gray-300 mb-2">Welcome to Ferrite</h2>
            <p class="text-surface-700 max-w-md mb-6">
              Get started by adding a media library in Settings.
            </p>
            <button class="btn-primary" onClick={() => navigate('/settings')}>
              Add Library
            </button>
          </div>
        </Show>
      </div>
    </div>
  );
}

// ---- Hero Banner ----
function HeroBanner(props: { item: MediaItem; onPlay: () => void; onInfo: () => void }) {
  const title = () => getDisplayTitle(props.item);
  const year = () => getDisplayYear(props.item);
  const posterUrl = () => props.item.poster_path ? authUrl(`/api/images/${props.item.poster_path}`) : null;

  return (
    <div class="relative h-[420px] overflow-hidden">
      {/* Backdrop */}
      <Show when={posterUrl()}>
        <img
          src={posterUrl()!}
          alt=""
          class="absolute inset-0 w-full h-full object-cover opacity-30 blur-2xl scale-110"
        />
      </Show>
      <div class="absolute inset-0 bg-gradient-to-t from-surface via-surface/80 to-surface/30" />
      <div class="absolute inset-0 bg-gradient-to-r from-surface/90 via-surface/50 to-transparent" />

      {/* Content */}
      <div class="relative h-full flex items-end px-8 pb-10">
        <div class="flex gap-6 items-end max-w-3xl">
          <Show when={posterUrl()}>
            <img
              src={posterUrl()!}
              alt={title()}
              class="w-36 h-52 object-cover rounded-xl shadow-2xl shadow-black/50 flex-shrink-0 border border-white/10"
            />
          </Show>
          <div class="space-y-3 pb-1">
            <div class="flex items-center gap-2 text-sm text-surface-800">
              <Show when={year()}><span>{year()}</span></Show>
              <Show when={props.item.content_rating}>
                <span class="badge bg-surface-400/50 text-surface-900">{props.item.content_rating}</span>
              </Show>
              <Show when={props.item.duration_ms}>
                <span>{formatDuration(props.item.duration_ms)}</span>
              </Show>
              <Show when={props.item.rating}>
                <span class="flex items-center gap-1"><Star class="w-3 h-3 text-yellow-500 fill-yellow-500" />{props.item.rating!.toFixed(1)}</span>
              </Show>
            </div>
            <h1 class="text-3xl font-bold text-white leading-tight text-balance">{title()}</h1>
            <Show when={props.item.overview}>
              <p class="text-sm text-surface-800 line-clamp-2 max-w-lg">{props.item.overview}</p>
            </Show>
            <div class="flex items-center gap-3 pt-1">
              <button class="btn-primary" onClick={props.onPlay}>
                <Play class="w-4 h-4 fill-current" /> Play
              </button>
              <button class="btn-secondary" onClick={props.onInfo}>
                <Info class="w-4 h-4" /> More Info
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

// ---- Horizontal Media Row ----
function MediaRow(props: {
  title: string;
  items: MediaItem[];
  onItemClick: (id: string) => void;
  onSeeAll?: () => void;
  showProgress?: boolean;
}) {
  let scrollRef!: HTMLDivElement;
  const [canScrollLeft, setCanScrollLeft] = createSignal(false);
  const [canScrollRight, setCanScrollRight] = createSignal(true);

  function updateScrollState() {
    if (!scrollRef) return;
    setCanScrollLeft(scrollRef.scrollLeft > 10);
    setCanScrollRight(scrollRef.scrollLeft < scrollRef.scrollWidth - scrollRef.clientWidth - 10);
  }

  function scroll(dir: 'left' | 'right') {
    if (!scrollRef) return;
    const amount = scrollRef.clientWidth * 0.75;
    scrollRef.scrollBy({ left: dir === 'left' ? -amount : amount, behavior: 'smooth' });
  }

  onMount(() => {
    if (scrollRef) {
      scrollRef.addEventListener('scroll', updateScrollState, { passive: true });
      updateScrollState();
    }
  });

  return (
    <section>
      <div class="flex items-center justify-between mb-3">
        <h2 class="text-lg font-semibold text-gray-200">{props.title}</h2>
        <Show when={props.onSeeAll}>
          <button class="btn-ghost text-xs" onClick={props.onSeeAll}>
            See All <ChevronRight class="w-3.5 h-3.5" />
          </button>
        </Show>
      </div>

      <div class="relative group/row">
        {/* Scroll buttons */}
        <Show when={canScrollLeft()}>
          <button
            class="absolute left-0 top-0 bottom-0 w-12 z-10 flex items-center justify-center
                   bg-gradient-to-r from-surface/90 to-transparent opacity-0 group-hover/row:opacity-100 transition-opacity"
            onClick={() => scroll('left')}
          >
            <ChevronLeft class="w-6 h-6 text-white" />
          </button>
        </Show>
        <Show when={canScrollRight()}>
          <button
            class="absolute right-0 top-0 bottom-0 w-12 z-10 flex items-center justify-center
                   bg-gradient-to-l from-surface/90 to-transparent opacity-0 group-hover/row:opacity-100 transition-opacity"
            onClick={() => scroll('right')}
          >
            <ChevronRight class="w-6 h-6 text-white" />
          </button>
        </Show>

        <div
          ref={scrollRef!}
          class="flex gap-3 overflow-x-auto scrollbar-hide pb-2 -mx-1 px-1"
        >
          <For each={props.items}>
            {(item) => (
              <PosterCard
                item={item}
                onClick={() => props.onItemClick(item.id)}
                showProgress={props.showProgress}
              />
            )}
          </For>
        </div>
      </div>
    </section>
  );
}

// ---- Poster Card ----
function PosterCard(props: { item: MediaItem; onClick: () => void; showProgress?: boolean }) {
  const title = () => getDisplayTitle(props.item);
  const year = () => getDisplayYear(props.item);
  const posterUrl = () => props.item.poster_path ? authUrl(`/api/images/${props.item.poster_path}`) : null;
  const progressPct = () => {
    if (!props.item.position_ms || !props.item.duration_ms) return 0;
    return Math.min(100, (props.item.position_ms / props.item.duration_ms) * 100);
  };
  const resLabel = () => getResLabel(props.item.width, props.item.height);

  return (
    <div
      class="flex-shrink-0 w-[160px] cursor-pointer group/card"
      onClick={props.onClick}
    >
      {/* Poster */}
      <div class="relative aspect-[2/3] rounded-xl overflow-hidden bg-surface-200 mb-2
                  ring-1 ring-white/5 group-hover/card:ring-ferrite-500/30
                  transition-all duration-250">
        <Show when={posterUrl()} fallback={
          <div class="w-full h-full flex items-center justify-center bg-surface-200">
            <Play class="w-8 h-8 text-surface-500" />
          </div>
        }>
          <img
            src={posterUrl()!}
            alt={title()}
            class="w-full h-full object-cover transition-transform duration-300 group-hover/card:scale-105"
            loading="lazy"
          />
        </Show>

        {/* Hover overlay */}
        <div class="absolute inset-0 bg-black/0 group-hover/card:bg-black/40 transition-all duration-250 flex items-center justify-center">
          <div class="w-12 h-12 rounded-full bg-ferrite-500/90 flex items-center justify-center
                      opacity-0 group-hover/card:opacity-100 scale-75 group-hover/card:scale-100
                      transition-all duration-250 shadow-lg shadow-ferrite-500/30">
            <Play class="w-5 h-5 text-white fill-white ml-0.5" />
          </div>
        </div>

        {/* Resolution badge */}
        <Show when={resLabel()}>
          <span class="absolute top-2 right-2 badge bg-black/60 text-white backdrop-blur-sm text-2xs">
            {resLabel()}
          </span>
        </Show>

        {/* Progress bar */}
        <Show when={props.showProgress && progressPct() > 0}>
          <div class="absolute bottom-0 left-0 right-0 h-1 bg-black/50">
            <div class="h-full bg-ferrite-500 transition-all" style={{ width: `${progressPct()}%` }} />
          </div>
        </Show>
      </div>

      {/* Title */}
      <h3 class="text-sm font-medium text-gray-300 group-hover/card:text-white truncate transition-colors">
        {title()}
      </h3>
      <Show when={year()}>
        <p class="text-xs text-surface-700">{year()}</p>
      </Show>
    </div>
  );
}
