import { Show } from 'solid-js';
import { useNavigate } from '@solidjs/router';
import { Search, Bell } from 'lucide-solid';
import { scanning, statusMessage } from '../stores/media';

export default function TopBar() {
  const navigate = useNavigate();

  function handleSearchFocus() {
    navigate('/search');
  }

  return (
    <header class="h-16 flex items-center justify-between px-6 border-b border-surface-300/30 bg-surface-50/50 backdrop-blur-xl sticky top-0 z-30">
      {/* Search bar */}
      <div class="flex-1 max-w-xl">
        <div
          class="flex items-center gap-3 px-4 py-2 rounded-xl bg-surface-200/60 border border-surface-400/50
                 hover:border-surface-500 cursor-text transition-all duration-200 group"
          onClick={handleSearchFocus}
        >
          <Search class="w-4 h-4 text-surface-700 group-hover:text-surface-800 transition-colors" />
          <span class="text-sm text-surface-700 group-hover:text-surface-800 transition-colors">
            Search media...
          </span>
          <kbd class="ml-auto hidden sm:inline-flex items-center gap-0.5 px-1.5 py-0.5 rounded bg-surface-300/50 text-2xs text-surface-700 font-mono">
            ⌘K
          </kbd>
        </div>
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
