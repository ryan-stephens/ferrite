import { Show } from 'solid-js';

interface HeaderProps {
  authRequired: boolean;
  itemCount: number;
  onLogout: () => void;
}

export default function Header(props: HeaderProps) {
  return (
    <header class="bg-surface-100 border-b border-surface-300 px-6 py-4 flex items-center justify-between">
      <h1 class="text-ferrite-500 text-xl font-bold">Ferrite</h1>
      <div class="flex items-center gap-4">
        <Show when={props.itemCount > 0}>
          <span class="text-gray-500 text-sm">{props.itemCount} items</span>
        </Show>
        <Show when={props.authRequired}>
          <button
            class="bg-surface-300 hover:bg-surface-400 text-gray-300 text-sm px-3 py-1.5 rounded-md transition-colors"
            onClick={props.onLogout}
          >
            Logout
          </button>
        </Show>
      </div>
    </header>
  );
}
