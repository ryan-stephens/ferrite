import { createSignal } from 'solid-js';

interface AddLibraryDialogProps {
  onAdd: (name: string, path: string, type: string) => void;
  onCancel: () => void;
}

export default function AddLibraryDialog(props: AddLibraryDialogProps) {
  const [name, setName] = createSignal('');
  const [path, setPath] = createSignal('');
  const [type, setType] = createSignal('movie');

  function handleAdd() {
    if (!name().trim() || !path().trim()) {
      alert('Name and path are required');
      return;
    }
    props.onAdd(name().trim(), path().trim(), type());
  }

  return (
    <div
      class="fixed inset-0 bg-black/70 z-[200] flex items-center justify-center"
      onClick={e => { if (e.target === e.currentTarget) props.onCancel(); }}
    >
      <div class="bg-surface-100 border border-surface-300 rounded-xl p-8 min-w-[400px]">
        <h3 class="text-lg font-semibold mb-4">Add Library</h3>

        <label class="block text-sm text-gray-400 mb-1">Name</label>
        <input
          type="text"
          class="w-full bg-surface-200 border border-surface-300 rounded-md px-3 py-2 text-gray-200 mb-4 focus:border-ferrite-500 focus:ring-1 focus:ring-ferrite-500"
          placeholder="Movies"
          value={name()}
          onInput={e => setName(e.currentTarget.value)}
          autofocus
        />

        <label class="block text-sm text-gray-400 mb-1">Path</label>
        <input
          type="text"
          class="w-full bg-surface-200 border border-surface-300 rounded-md px-3 py-2 text-gray-200 mb-4 focus:border-ferrite-500 focus:ring-1 focus:ring-ferrite-500"
          placeholder="/media/movies"
          value={path()}
          onInput={e => setPath(e.currentTarget.value)}
        />

        <label class="block text-sm text-gray-400 mb-1">Type</label>
        <select
          class="w-full bg-surface-200 border border-surface-300 rounded-md px-3 py-2 text-gray-200 mb-6 focus:border-ferrite-500 focus:ring-1 focus:ring-ferrite-500"
          value={type()}
          onChange={e => setType(e.currentTarget.value)}
        >
          <option value="movie">Movies</option>
          <option value="tv">TV Shows</option>
          <option value="music">Music</option>
        </select>

        <div class="flex gap-2 justify-end">
          <button
            class="bg-surface-300 hover:bg-surface-400 text-gray-300 font-medium px-4 py-2 rounded-md transition-colors"
            onClick={props.onCancel}
          >
            Cancel
          </button>
          <button
            class="bg-ferrite-500 hover:bg-ferrite-600 text-white font-medium px-4 py-2 rounded-md transition-colors"
            onClick={handleAdd}
          >
            Add & Scan
          </button>
        </div>
      </div>
    </div>
  );
}
