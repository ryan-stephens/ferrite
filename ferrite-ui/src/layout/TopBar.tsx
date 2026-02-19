import { createSignal, createMemo, For, Show, onMount, onCleanup } from 'solid-js';
import { useNavigate, useLocation, useSearchParams } from '@solidjs/router';
import { Search, Bell, X, Play } from 'lucide-solid';
import { scanning, statusMessage, allMedia, loadMedia } from '../stores/media';
import { authUrl } from '../api';
import { getDisplayTitle, getDisplayYear, getEpisodeLabel, getResLabel } from '../utils';

export default function TopBar() {
  const navigate = useNavigate();
  const location = useLocation();
  const [searchParams, setSearchParams] = useSearchParams();
  let inputRef!: HTMLInputElement;
  let containerRef!: HTMLDivElement;

  const [query, setQuery] = createSignal('');
  const [open, setOpen] = createSignal(false);
  const [focused, setFocused] = createSignal(false);

  const onSearchPage = () => location.pathname === '/search';

  onMount(async () => {
    if (allMedia().length === 0) await loadMedia();

    // ⌘K / Ctrl+K focuses the search bar
    function onKey(e: KeyboardEvent) {
      if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
        e.preventDefault();
        inputRef?.focus();
      }
    }
    window.addEventListener('keydown', onKey);
    onCleanup(() => window.removeEventListener('keydown', onKey));
  });

  const quickResults = createMemo(() => {
    const q = query().toLowerCase().trim();
    if (!q) return [];
    return allMedia()
      .filter(item => getDisplayTitle(item).toLowerCase().includes(q))
      .slice(0, 8);
  });

  function handleInput(e: InputEvent) {
    const val = (e.currentTarget as HTMLInputElement).value;
    setQuery(val);
    if (onSearchPage()) {
      setSearchParams({ q: val || undefined });
      setOpen(false);
    } else {
      setOpen(true);
    }
  }

  function goToSearch() {
    const q = query().trim();
    if (!q) return;
    setOpen(false);
    inputRef.blur();
    // Always navigate — SolidJS router dedupes same-path navigations
    navigate(`/search?q=${encodeURIComponent(q)}`);
  }

  function handleKeyDown(e: KeyboardEvent) {
    if (e.key === 'Enter') {
      if (onSearchPage()) {
        // Already live-updating via handleInput; just close dropdown and blur
        setOpen(false);
        inputRef.blur();
      } else {
        goToSearch();
      }
    } else if (e.key === 'Escape') {
      setOpen(false);
      inputRef.blur();
    }
  }

  function handleBlur(e: FocusEvent) {
    // Delay so clicks inside the dropdown register first
    setTimeout(() => {
      if (!containerRef?.contains(document.activeElement)) {
        setOpen(false);
        setFocused(false);
      }
    }, 150);
  }

  function pickItem(id: string) {
    setOpen(false);
    setQuery('');
    inputRef.blur();
    navigate(`/media/${id}`);
  }

  function clearQuery() {
    setQuery('');
    setOpen(false);
    inputRef.focus();
  }

  return (
    <header class="h-16 flex items-center justify-between px-6 border-b border-surface-300/30 bg-surface-50/50 backdrop-blur-xl sticky top-0 z-30">
      {/* Search bar */}
      <div class="flex-1 max-w-xl relative" ref={containerRef!}>
        <div class={`flex items-center gap-3 px-4 py-2 rounded-xl bg-surface-200/60 border transition-all duration-200
          ${focused() ? 'border-ferrite-500/60' : 'border-surface-400/50 hover:border-surface-500'}`}>
          <Search class="w-4 h-4 text-surface-700 flex-shrink-0" />
          <input
            ref={inputRef!}
            type="text"
            class="flex-1 bg-transparent text-sm text-gray-200 placeholder-surface-700 min-w-0"
            style="outline: none;"
            placeholder="Search media..."
            value={query()}
            onInput={handleInput}
            onKeyDown={handleKeyDown}
            onFocus={() => { setFocused(true); if (query()) setOpen(true); }}
            onBlur={handleBlur}
          />
          <Show when={query()}>
            <button class="flex-shrink-0 text-surface-600 hover:text-surface-800 transition-colors" onClick={clearQuery}>
              <X class="w-3.5 h-3.5" />
            </button>
          </Show>
          <Show when={!query()}>
            <kbd class="flex-shrink-0 hidden sm:inline-flex items-center gap-0.5 px-1.5 py-0.5 rounded bg-surface-300/50 text-2xs text-surface-700 font-mono">
              ⌘K
            </kbd>
          </Show>
        </div>

        {/* Dropdown */}
        <Show when={open() && quickResults().length > 0}>
          <div class="absolute top-full left-0 right-0 mt-2 rounded-xl bg-surface-100 border border-surface-300/60 shadow-2xl shadow-black/40 overflow-hidden z-50">
            <For each={quickResults()}>
              {(item) => {
                const isEp = !!item.is_episode;
                const imgPath = item.still_path || item.poster_path;
                const epLabel = isEp ? getEpisodeLabel(item) : null;
                return (
                  <button
                    class="w-full flex items-center gap-3 px-3 py-2.5 hover:bg-surface-200 transition-colors text-left"
                    onMouseDown={(e) => { e.preventDefault(); pickItem(item.id); }}
                  >
                    {/* Thumbnail */}
                    <div class={`flex-shrink-0 rounded-md overflow-hidden bg-surface-300 ${
                      isEp ? 'w-16 h-9' : 'w-8 h-11'
                    }`}>
                      <Show when={imgPath} fallback={
                        <div class="w-full h-full flex items-center justify-center">
                          <Play class="w-3 h-3 text-surface-600" />
                        </div>
                      }>
                        <img src={authUrl(`/api/images/${imgPath}`)} alt="" class="w-full h-full object-cover" loading="lazy" />
                      </Show>
                    </div>
                    {/* Text */}
                    <div class="flex-1 min-w-0">
                      <p class="text-sm font-medium text-gray-200 truncate">{getDisplayTitle(item)}</p>
                      <p class="text-xs text-surface-700 truncate">
                        <Show when={isEp && epLabel}><span class="font-mono">{epLabel}</span></Show>
                        <Show when={isEp && item.episode_title}>
                          <span>{epLabel ? ' · ' : ''}{item.episode_title}</span>
                        </Show>
                        <Show when={!isEp}>
                          <span>{getDisplayYear(item)}</span>
                          <Show when={getDisplayYear(item) && getResLabel(item.width, item.height)}>
                            <span class="mx-1">·</span>
                          </Show>
                          <span>{getResLabel(item.width, item.height)}</span>
                        </Show>
                      </p>
                    </div>
                  </button>
                );
              }}
            </For>
            {/* Footer: press Enter for full results */}
            <div class="px-3 py-2 border-t border-surface-300/40 flex items-center justify-between">
              <span class="text-xs text-surface-600">Press <kbd class="px-1 py-0.5 rounded bg-surface-300/50 font-mono text-2xs">Enter</kbd> for all results</span>
              <button
                class="text-xs text-ferrite-400 hover:text-ferrite-300 transition-colors"
                onMouseDown={(e) => { e.preventDefault(); goToSearch(); }}
              >
                See all →
              </button>
            </div>
          </div>
        </Show>
      </div>

      {/* Right section */}
      <div class="flex items-center gap-2 ml-4">
        <Show when={scanning()}>
          <div class="flex items-center gap-2 px-3 py-1.5 rounded-lg bg-ferrite-500/10 border border-ferrite-500/20">
            <div class="w-2 h-2 rounded-full bg-ferrite-500 animate-pulse" />
            <span class="text-xs text-ferrite-400 font-medium">{statusMessage()}</span>
          </div>
        </Show>

        <button class="btn-icon" title="Notifications">
          <Bell class="w-4.5 h-4.5" />
        </button>
      </div>
    </header>
  );
}
