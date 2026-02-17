import { For } from 'solid-js';
import type { Library } from '../api';

interface LibraryChipsProps {
  libraries: Library[];
  currentLibrary: string | null;
  onSelect: (id: string) => void;
  onDelete: (id: string) => void;
}

export default function LibraryChips(props: LibraryChipsProps) {
  return (
    <div class="flex gap-2 flex-wrap mb-6">
      <For each={props.libraries}>
        {lib => (
          <div
            class={`flex items-center gap-2 px-3 py-1.5 rounded-full text-sm cursor-pointer transition-colors ${
              props.currentLibrary === lib.id
                ? 'bg-ferrite-500 text-white'
                : 'bg-surface-300 text-gray-300 hover:bg-surface-400'
            }`}
            onClick={() => props.onSelect(lib.id)}
          >
            {lib.name}
            <span
              class="opacity-60 hover:opacity-100 text-base leading-none"
              onClick={e => { e.stopPropagation(); props.onDelete(lib.id); }}
            >
              ×
            </span>
          </div>
        )}
      </For>
    </div>
  );
}
