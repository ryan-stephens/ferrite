import { createSignal, For, Show, onMount } from 'solid-js';
import { Settings, FolderPlus, Trash2, RefreshCw, Server, Cpu, HardDrive } from 'lucide-solid';
import { libraries, loadLibraries, addLibrary, deleteLibrary, refreshAll, scanning, statusMessage } from '../stores/media';
import { api } from '../api';

export default function SettingsPage() {
  const [showAddDialog, setShowAddDialog] = createSignal(false);
  const [serverInfo, setServerInfo] = createSignal<{ name: string; version: string } | null>(null);

  onMount(async () => {
    if (libraries().length === 0) await loadLibraries();
    try {
      const info = await api.info();
      setServerInfo(info);
    } catch { /* ignore */ }
  });

  return (
    <div class="px-6 py-6 max-w-4xl mx-auto animate-fade-in">
      <div class="flex items-center gap-3 mb-8">
        <div class="w-10 h-10 rounded-xl bg-surface-200 flex items-center justify-center">
          <Settings class="w-5 h-5 text-surface-700" />
        </div>
        <h1 class="text-xl font-bold text-white">Settings</h1>
      </div>

      {/* Server Info */}
      <Show when={serverInfo()}>
        <section class="mb-8">
          <h2 class="text-sm font-semibold text-surface-800 uppercase tracking-wider mb-3">Server</h2>
          <div class="card p-4">
            <div class="flex items-center gap-4">
              <div class="w-10 h-10 rounded-xl bg-ferrite-500/10 flex items-center justify-center">
                <Server class="w-5 h-5 text-ferrite-400" />
              </div>
              <div>
                <div class="text-sm font-medium text-white">{serverInfo()!.name}</div>
                <div class="text-xs text-surface-700">Version {serverInfo()!.version}</div>
              </div>
            </div>
          </div>
        </section>
      </Show>

      {/* Libraries */}
      <section class="mb-8">
        <div class="flex items-center justify-between mb-3">
          <h2 class="text-sm font-semibold text-surface-800 uppercase tracking-wider">Libraries</h2>
          <div class="flex items-center gap-2">
            <button class="btn-ghost text-xs" onClick={refreshAll} disabled={scanning()}>
              <RefreshCw class={`w-3.5 h-3.5 ${scanning() ? 'animate-spin' : ''}`} />
              {scanning() ? statusMessage() : 'Refresh All'}
            </button>
            <button class="btn-primary text-xs py-1.5 px-3" onClick={() => setShowAddDialog(true)}>
              <FolderPlus class="w-3.5 h-3.5" /> Add Library
            </button>
          </div>
        </div>

        <div class="space-y-2">
          <For each={libraries()} fallback={
            <div class="card p-8 text-center">
              <HardDrive class="w-8 h-8 text-surface-500 mx-auto mb-2" />
              <p class="text-sm text-surface-700">No libraries configured</p>
              <button class="btn-primary text-xs mt-3" onClick={() => setShowAddDialog(true)}>
                <FolderPlus class="w-3.5 h-3.5" /> Add Your First Library
              </button>
            </div>
          }>
            {(lib) => (
              <div class="card p-4 flex items-center justify-between group">
                <div class="flex items-center gap-3">
                  <div class="w-9 h-9 rounded-lg bg-surface-200 flex items-center justify-center">
                    <HardDrive class="w-4 h-4 text-surface-700" />
                  </div>
                  <div>
                    <div class="text-sm font-medium text-gray-300">{lib.name}</div>
                    <div class="text-xs text-surface-700 font-mono">{lib.path}</div>
                  </div>
                  <span class="badge bg-surface-300/50 text-surface-800 ml-2">{lib.library_type}</span>
                </div>
                <button
                  class="btn-icon text-surface-600 hover:text-red-400 opacity-0 group-hover:opacity-100 transition-all"
                  onClick={async () => {
                    if (confirm(`Delete library "${lib.name}"?`)) await deleteLibrary(lib.id);
                  }}
                  title="Delete library"
                >
                  <Trash2 class="w-4 h-4" />
                </button>
              </div>
            )}
          </For>
        </div>
      </section>

      {/* Add Library Dialog */}
      <Show when={showAddDialog()}>
        <AddLibraryDialog onClose={() => setShowAddDialog(false)} />
      </Show>
    </div>
  );
}

function AddLibraryDialog(props: { onClose: () => void }) {
  const [name, setName] = createSignal('');
  const [path, setPath] = createSignal('');
  const [type, setType] = createSignal('movies');
  const [loading, setLoading] = createSignal(false);

  async function handleSubmit(e: Event) {
    e.preventDefault();
    if (!name().trim() || !path().trim()) return;
    setLoading(true);
    try {
      await addLibrary(name(), path(), type());
      props.onClose();
    } catch (err: any) {
      alert(err.message || 'Failed to add library');
    }
    setLoading(false);
  }

  return (
    <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm animate-fade-in" onClick={props.onClose}>
      <form
        class="card p-6 w-full max-w-md space-y-4 animate-scale-in"
        onClick={(e) => e.stopPropagation()}
        onSubmit={handleSubmit}
      >
        <h2 class="text-lg font-semibold text-white">Add Library</h2>

        <div>
          <label class="block text-sm font-medium text-gray-400 mb-1.5">Name</label>
          <input class="input-field" placeholder="My Movies" value={name()} onInput={(e) => setName(e.currentTarget.value)} autofocus />
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-400 mb-1.5">Path</label>
          <input class="input-field font-mono text-sm" placeholder="/path/to/media" value={path()} onInput={(e) => setPath(e.currentTarget.value)} />
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-400 mb-1.5">Type</label>
          <select class="input-field" value={type()} onChange={(e) => setType(e.currentTarget.value)}>
            <option value="movies">Movies</option>
            <option value="tv">TV Shows</option>
            <option value="music">Music</option>
            <option value="other">Other</option>
          </select>
        </div>

        <div class="flex justify-end gap-3 pt-2">
          <button type="button" class="btn-secondary" onClick={props.onClose}>Cancel</button>
          <button type="submit" class="btn-primary" disabled={loading()}>
            {loading() ? <div class="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin" /> : 'Add Library'}
          </button>
        </div>
      </form>
    </div>
  );
}
