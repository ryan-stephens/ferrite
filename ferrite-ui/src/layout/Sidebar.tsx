import { createSignal, For, Show } from 'solid-js';
import { A, useLocation } from '@solidjs/router';
import {
  Home, Library, PlayCircle, FolderOpen, Settings, ChevronLeft,
  ChevronRight, Search, LogOut, Flame,
} from 'lucide-solid';
import { libraries } from '../stores/media';
import { authState, logout } from '../stores/auth';

const [collapsed, setCollapsed] = createSignal(
  localStorage.getItem('ferrite-sidebar-collapsed') === 'true'
);

function toggleCollapsed() {
  const next = !collapsed();
  setCollapsed(next);
  localStorage.setItem('ferrite-sidebar-collapsed', String(next));
}

export { collapsed };

export default function Sidebar() {
  const location = useLocation();

  const isActive = (path: string) => {
    if (path === '/') return location.pathname === '/';
    return location.pathname.startsWith(path);
  };

  const linkClass = (path: string) =>
    `flex items-center gap-3 px-3 py-2.5 rounded-xl text-sm font-medium transition-all duration-200 group
     ${isActive(path)
       ? 'bg-ferrite-500/15 text-ferrite-400'
       : 'text-surface-800 hover:text-gray-200 hover:bg-white/5'}`;

  const iconClass = (path: string) =>
    `w-5 h-5 flex-shrink-0 transition-colors ${isActive(path) ? 'text-ferrite-400' : 'text-surface-700 group-hover:text-gray-300'}`;

  return (
    <aside
      class={`fixed top-0 left-0 h-full z-40 flex flex-col
              bg-surface-50 border-r border-surface-300/50
              transition-all duration-300 ease-out
              ${collapsed() ? 'w-sidebar-collapsed' : 'w-sidebar'}`}
    >
      {/* Logo / Brand */}
      <div class="flex items-center gap-3 px-4 h-16 border-b border-surface-300/30 flex-shrink-0">
        <div class="w-8 h-8 rounded-lg bg-gradient-to-br from-ferrite-500 to-ferrite-700 flex items-center justify-center flex-shrink-0">
          <Flame class="w-4.5 h-4.5 text-white" />
        </div>
        <Show when={!collapsed()}>
          <span class="text-lg font-bold text-white tracking-tight">Ferrite</span>
        </Show>
      </div>

      {/* Navigation */}
      <nav class="flex-1 px-3 py-4 space-y-1 overflow-y-auto scrollbar-hide">
        <A href="/" class={linkClass('/')}>
          <Home class={iconClass('/')} />
          <Show when={!collapsed()}><span>Home</span></Show>
        </A>

        <A href="/search" class={linkClass('/search')}>
          <Search class={iconClass('/search')} />
          <Show when={!collapsed()}><span>Search</span></Show>
        </A>

        <Show when={!collapsed()}>
          <div class="pt-4 pb-2 px-3">
            <span class="text-2xs font-semibold uppercase tracking-widest text-surface-600">Libraries</span>
          </div>
        </Show>
        <Show when={collapsed()}>
          <div class="pt-3" />
        </Show>

        <For each={libraries()}>
          {(lib) => (
            <A href={`/library/${lib.id}`} class={linkClass(`/library/${lib.id}`)}>
              <FolderOpen class={iconClass(`/library/${lib.id}`)} />
              <Show when={!collapsed()}>
                <span class="truncate">{lib.name}</span>
              </Show>
            </A>
          )}
        </For>

        <Show when={!collapsed()}>
          <div class="pt-4 pb-2 px-3">
            <span class="text-2xs font-semibold uppercase tracking-widest text-surface-600">Activity</span>
          </div>
        </Show>
        <Show when={collapsed()}>
          <div class="pt-3" />
        </Show>

        <A href="/continue" class={linkClass('/continue')}>
          <PlayCircle class={iconClass('/continue')} />
          <Show when={!collapsed()}><span>Continue Watching</span></Show>
        </A>

        <A href="/collections" class={linkClass('/collections')}>
          <Library class={iconClass('/collections')} />
          <Show when={!collapsed()}><span>Collections</span></Show>
        </A>
      </nav>

      {/* Bottom section */}
      <div class="px-3 py-3 border-t border-surface-300/30 space-y-1 flex-shrink-0">
        <A href="/settings" class={linkClass('/settings')}>
          <Settings class={iconClass('/settings')} />
          <Show when={!collapsed()}><span>Settings</span></Show>
        </A>

        <Show when={authState().authRequired}>
          <button onClick={logout} class={`${linkClass('/logout')} w-full`}>
            <LogOut class="w-5 h-5 flex-shrink-0 text-surface-700 group-hover:text-gray-300" />
            <Show when={!collapsed()}><span>Log Out</span></Show>
          </button>
        </Show>

        {/* Collapse toggle */}
        <button
          onClick={toggleCollapsed}
          class="flex items-center justify-center w-full py-2 rounded-lg text-surface-700 hover:text-gray-300 hover:bg-white/5 transition-all"
          title={collapsed() ? 'Expand sidebar' : 'Collapse sidebar'}
        >
          {collapsed() ? <ChevronRight class="w-4 h-4" /> : <ChevronLeft class="w-4 h-4" />}
        </button>
      </div>
    </aside>
  );
}
