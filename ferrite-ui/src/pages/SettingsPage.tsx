import { createSignal, For, Show, onMount, onCleanup, createEffect } from 'solid-js';
import type { ScanProgress, UpdateCheckResult, UpdateProgress, UpdateHistoryEntry } from '../api';
import { Settings, FolderPlus, Trash2, RefreshCw, Server, HardDrive, Users, UserPlus, KeyRound, ShieldCheck, Shield, Sliders, Download, ExternalLink, RotateCcw, AlertTriangle, History } from 'lucide-solid';
import { libraries, loadLibraries, addLibrary, deleteLibrary, refreshAll, scanning, statusMessage } from '../stores/media';
import { api } from '../api';
import type { User, UserPreferences } from '../api';

export default function SettingsPage() {
  const [showAddDialog, setShowAddDialog] = createSignal(false);
  const [serverInfo, setServerInfo] = createSignal<{ name: string; version: string } | null>(null);
  const [users, setUsers] = createSignal<User[]>([]);
  const [currentUser, setCurrentUser] = createSignal<User | null>(null);
  const [showCreateUser, setShowCreateUser] = createSignal(false);
  const [resetTarget, setResetTarget] = createSignal<User | null>(null);
  const [prefs, setPrefs] = createSignal<UserPreferences>({});
  const [prefsSaving, setPrefsSaving] = createSignal(false);
  const [prefsSaved, setPrefsSaved] = createSignal(false);
  const [scanProgress, setScanProgress] = createSignal<Record<string, ScanProgress>>({});
  const [scanningLibs, setScanningLibs] = createSignal<Set<string>>(new Set());
  const [updateInfo, setUpdateInfo] = createSignal<UpdateCheckResult | null>(null);
  const [updateChecking, setUpdateChecking] = createSignal(false);
  const [updateProgress, setUpdateProgress] = createSignal<UpdateProgress | null>(null);
  const [updateApplying, setUpdateApplying] = createSignal(false);
  const [reconnecting, setReconnecting] = createSignal(false);
  const [updateHistoryList, setUpdateHistoryList] = createSignal<UpdateHistoryEntry[]>([]);
  let pollInterval: ReturnType<typeof setInterval> | null = null;
  let updatePollInterval: ReturnType<typeof setInterval> | null = null;

  async function pollScanStatus() {
    const libs = libraries();
    if (libs.length === 0) return;
    const updates: Record<string, ScanProgress> = {};
    await Promise.all(libs.map(async (lib) => {
      try {
        const p = await api.scanStatus(lib.id);
        updates[lib.id] = p;
      } catch { /* ignore */ }
    }));
    setScanProgress(updates);
    const anyActive = Object.values(updates).some(p => p.scanning);
    if (!anyActive && pollInterval) {
      clearInterval(pollInterval);
      pollInterval = null;
    }
  }

  function startPolling() {
    if (pollInterval) return;
    pollInterval = setInterval(pollScanStatus, 2000);
  }

  async function triggerScan(libId: string) {
    setScanningLibs(prev => new Set([...prev, libId]));
    try {
      await api.scanLibrary(libId);
      startPolling();
      pollScanStatus();
    } catch (err: any) {
      alert(err.message || 'Failed to start scan');
    } finally {
      setScanningLibs(prev => { const s = new Set(prev); s.delete(libId); return s; });
    }
  }

  onMount(async () => {
    if (libraries().length === 0) await loadLibraries();
    try {
      const info = await api.info();
      setServerInfo(info);
    } catch { /* ignore */ }
    try {
      const status = await api.authStatus();
      if (status.auth_required) {
        const me = await api.getCurrentUser();
        setCurrentUser(me);
        if (me.is_admin === 1) {
          const userList = await api.listUsers();
          setUsers(userList);
        }
      } else {
        // No auth configured — everyone has full access, treat as admin
        setCurrentUser({ id: '', username: 'admin', is_admin: 1 } as any);
      }
    } catch { /* ignore */ }
    try {
      const p = await api.getPreferences();
      setPrefs(p);
    } catch { /* ignore */ }
    // Check for updates and load history (admin only, non-blocking)
    if (currentUser()?.is_admin === 1) {
      await handleCheckForUpdate();
    }
    // Check if any scans are already running
    await pollScanStatus();
    const anyActive = Object.values(scanProgress()).some(p => p.scanning);
    if (anyActive) startPolling();
  });

  onCleanup(() => {
    if (pollInterval) clearInterval(pollInterval);
    if (updatePollInterval) clearInterval(updatePollInterval);
  });

  async function handleCheckForUpdate(force = false) {
    if (updateChecking()) return;
    setUpdateChecking(true);
    try {
      const info = await api.checkForUpdate(force);
      setUpdateInfo(info);
    } catch { /* ignore — update check is best-effort */ }
    setUpdateChecking(false);
    try {
      const history = await api.updateHistory();
      setUpdateHistoryList(history);
    } catch { /* ignore */ }
  }

  async function handleApplyUpdate() {
    if (updateApplying()) return;
    if (!confirm('Apply update now? The server will restart automatically.')) return;
    setUpdateApplying(true);
    try {
      await api.applyUpdate();
      startUpdatePolling();
    } catch (err: any) {
      alert(err.message || 'Failed to start update');
      setUpdateApplying(false);
    }
  }

  function startUpdatePolling() {
    if (updatePollInterval) return;
    updatePollInterval = setInterval(async () => {
      try {
        const status = await api.updateStatus();
        setUpdateProgress(status);
        if (status.state === 'restarting') {
          if (updatePollInterval) clearInterval(updatePollInterval);
          updatePollInterval = null;
          setReconnecting(true);
          // Wait 5s then start polling health
          setTimeout(pollUntilReconnected, 5000);
        } else if (status.state === 'failed') {
          if (updatePollInterval) clearInterval(updatePollInterval);
          updatePollInterval = null;
          setUpdateApplying(false);
        }
      } catch {
        // Server may be shutting down — switch to reconnect mode
        if (updatePollInterval) clearInterval(updatePollInterval);
        updatePollInterval = null;
        setReconnecting(true);
        setTimeout(pollUntilReconnected, 5000);
      }
    }, 1000);
  }

  async function pollUntilReconnected() {
    let attempts = 0;
    const maxAttempts = 60;
    const poll = setInterval(async () => {
      attempts++;
      try {
        await api.info();
        clearInterval(poll);
        setReconnecting(false);
        setUpdateApplying(false);
        setUpdateProgress(null);
        // Refresh version info
        const info = await api.info();
        setServerInfo(info);
        setUpdateInfo(null);
        // Re-check for updates
        try {
          const check = await api.checkForUpdate();
          setUpdateInfo(check);
        } catch { /* ignore */ }
      } catch {
        if (attempts >= maxAttempts) {
          clearInterval(poll);
          setReconnecting(false);
          setUpdateApplying(false);
          alert('Server did not come back after update. Check server logs.');
        }
      }
    }, 2000);
  }

  async function handleRollback() {
    if (!confirm('Roll back to the previous version? The server will restart.')) return;
    try {
      await api.rollbackUpdate();
      setReconnecting(true);
      setTimeout(pollUntilReconnected, 5000);
    } catch (err: any) {
      alert(err.message || 'Rollback failed');
    }
  }

  function updatePhaseLabel(state: string): string {
    switch (state) {
      case 'downloading': return 'Downloading update…';
      case 'verifying': return 'Verifying checksum…';
      case 'extracting': return 'Extracting files…';
      case 'swapping': return 'Installing update…';
      case 'restarting': return 'Restarting server…';
      case 'failed': return 'Update failed';
      default: return 'Idle';
    }
  }

  async function savePrefs(updates: Partial<UserPreferences>) {
    const next = { ...prefs(), ...updates };
    setPrefs(next);
    setPrefsSaving(true);
    setPrefsSaved(false);
    try {
      await api.setPreferences(updates);
      setPrefsSaved(true);
      setTimeout(() => setPrefsSaved(false), 2000);
    } catch { /* ignore */ } finally {
      setPrefsSaving(false);
    }
  }

  async function handleDeleteUser(user: User) {
    if (!confirm(`Delete user "${user.username}"? This cannot be undone.`)) return;
    try {
      await api.deleteUser(user.id);
      setUsers(users().filter(u => u.id !== user.id));
    } catch (err: any) {
      alert(err.message || 'Failed to delete user');
    }
  }

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

      {/* System Update — admin only */}
      <Show when={currentUser()?.is_admin === 1}>
        <section class="mb-8">
          <div class="flex items-center justify-between mb-3">
            <h2 class="text-sm font-semibold text-surface-800 uppercase tracking-wider">System Update</h2>
            <button
              class="btn-ghost text-xs flex items-center gap-1.5"
              onClick={() => handleCheckForUpdate(true)}
              disabled={updateChecking() || updateApplying()}
            >
              <RefreshCw class={`w-3.5 h-3.5 ${updateChecking() ? 'animate-spin' : ''}`} />
              {updateChecking() ? 'Checking…' : 'Check for Updates'}
            </button>
          </div>
          <div class="card p-4">
            {/* Reconnecting overlay */}
            <Show when={reconnecting()}>
              <div class="flex items-center gap-3 text-sm text-surface-700">
                <div class="w-4 h-4 border-2 border-surface-400 border-t-ferrite-500 rounded-full animate-spin" />
                Waiting for server to restart…
              </div>
            </Show>

            {/* Update in progress */}
            <Show when={updateApplying() && !reconnecting()}>
              {(() => {
                const prog = updateProgress();
                const phase = prog?.state ?? 'downloading';
                const pct = prog?.progress_pct ?? 0;
                const downloaded = prog?.downloaded_bytes ?? 0;
                const total = prog?.total_bytes ?? 0;
                const failed = phase === 'failed';
                return (
                  <div class="space-y-3">
                    <div class="flex items-center gap-3">
                      <div class={`w-10 h-10 rounded-xl flex items-center justify-center ${
                        failed ? 'bg-red-500/10' : 'bg-blue-500/10'
                      }`}>
                        {failed
                          ? <AlertTriangle class="w-5 h-5 text-red-400" />
                          : <div class="w-5 h-5 border-2 border-blue-300 border-t-blue-500 rounded-full animate-spin" />
                        }
                      </div>
                      <div class="flex-1">
                        <div class="text-sm font-medium text-white">{updatePhaseLabel(phase)}</div>
                        <Show when={phase === 'downloading' && total > 0}>
                          <div class="text-xs text-surface-700">
                            {(downloaded / 1048576).toFixed(1)} / {(total / 1048576).toFixed(1)} MB
                          </div>
                        </Show>
                        <Show when={failed && prog?.error}>
                          <div class="text-xs text-red-400 mt-1">{prog!.error}</div>
                        </Show>
                      </div>
                    </div>
                    <Show when={!failed}>
                      <div class="w-full h-1.5 bg-surface-300 rounded-full overflow-hidden">
                        <div
                          class="h-full bg-blue-500 rounded-full transition-all duration-500"
                          style={{ width: `${phase === 'downloading' ? pct : 100}%` }}
                        />
                      </div>
                    </Show>
                    <Show when={failed}>
                      <button class="btn-secondary text-xs mt-2" onClick={() => { setUpdateApplying(false); setUpdateProgress(null); }}>
                        Dismiss
                      </button>
                    </Show>
                  </div>
                );
              })()}
            </Show>

            {/* Checking spinner */}
            <Show when={updateChecking() && !updateInfo() && !updateApplying() && !reconnecting()}>
              <div class="flex items-center gap-3 text-sm text-surface-700">
                <div class="w-4 h-4 border-2 border-surface-400 border-t-ferrite-500 rounded-full animate-spin" />
                Checking for updates…
              </div>
            </Show>

            {/* Idle — no check done yet */}
            <Show when={!updateChecking() && !updateInfo() && !updateApplying() && !reconnecting()}>
              <div class="flex items-center gap-3 text-sm text-surface-700">
                <div class="w-10 h-10 rounded-xl flex items-center justify-center bg-surface-200">
                  <Download class="w-5 h-5 text-surface-700" />
                </div>
                <div class="text-surface-600">Click "Check for Updates" to see if a newer version is available.</div>
              </div>
            </Show>

            {/* Check result */}
            <Show when={updateInfo() && !updateApplying() && !reconnecting()}>
              {(() => {
                const info = updateInfo()!;
                return (
                  <div class="space-y-3">
                    <div class="flex items-center justify-between">
                      <div class="flex items-center gap-3">
                        <div class={`w-10 h-10 rounded-xl flex items-center justify-center ${
                          info.update_available ? 'bg-green-500/10' : 'bg-surface-200'
                        }`}>
                          <Download class={`w-5 h-5 ${
                            info.update_available ? 'text-green-400' : 'text-surface-700'
                          }`} />
                        </div>
                        <div>
                          <div class="text-sm font-medium text-white">
                            {info.update_available
                              ? `Update available: v${info.latest_version}`
                              : 'Up to date'}
                          </div>
                          <div class="text-xs text-surface-700">
                            Current: v{info.current_version}
                            {info.update_available && info.published_at && (
                              <> · Released {new Date(info.published_at).toLocaleDateString()}</>
                            )}
                            {info.update_available && info.download_size_bytes && (
                              <> · {(info.download_size_bytes / 1048576).toFixed(1)} MB</>
                            )}
                          </div>
                        </div>
                      </div>
                      <div class="flex items-center gap-2">
                        <Show when={info.update_available && info.release_url}>
                          <a
                            href={info.release_url}
                            target="_blank"
                            rel="noopener noreferrer"
                            class="btn-ghost text-xs flex items-center gap-1.5"
                          >
                            <ExternalLink class="w-3.5 h-3.5" /> Release Notes
                          </a>
                        </Show>
                        <Show when={info.update_available}>
                          <button
                            class="btn-primary text-xs py-1.5 px-3 flex items-center gap-1.5"
                            onClick={handleApplyUpdate}
                          >
                            <Download class="w-3.5 h-3.5" /> Update Now
                          </button>
                        </Show>
                      </div>
                    </div>
                    <Show when={info.update_available && info.release_notes}>
                      <details class="text-xs text-surface-700">
                        <summary class="cursor-pointer hover:text-surface-800 transition-colors">What's new in v{info.latest_version}</summary>
                        <pre class="mt-2 whitespace-pre-wrap text-surface-600 bg-surface-100 rounded-lg p-3 max-h-48 overflow-y-auto">{info.release_notes}</pre>
                      </details>
                    </Show>
                  </div>
                );
              })()}
            </Show>
          </div>

          {/* Update History */}
          <Show when={updateHistoryList().length > 0}>
            <details class="mt-3 text-xs text-surface-700">
              <summary class="cursor-pointer hover:text-surface-800 transition-colors flex items-center gap-1.5">
                <History class="w-3.5 h-3.5" /> Update History ({updateHistoryList().length})
              </summary>
              <div class="mt-2 space-y-1.5">
                <For each={[...updateHistoryList()].reverse()}>
                  {(entry) => (
                    <div class="flex items-center justify-between bg-surface-100 rounded-lg px-3 py-2">
                      <div class="flex items-center gap-2">
                        <span class={`w-1.5 h-1.5 rounded-full ${entry.success ? 'bg-green-400' : 'bg-red-400'}`} />
                        <span class="text-surface-600">
                          v{entry.from_version} → v{entry.to_version}
                        </span>
                      </div>
                      <span class="text-surface-500">
                        {new Date(entry.applied_at).toLocaleDateString()}{' '}
                        {new Date(entry.applied_at).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}
                      </span>
                    </div>
                  )}
                </For>
              </div>
            </details>
          </Show>
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
            {(lib) => {
              const progress = () => scanProgress()[lib.id];
              const isActive = () => progress()?.scanning ?? false;
              const isTriggeringThis = () => scanningLibs().has(lib.id);
              return (
                <div class="card p-4 group">
                  <div class="flex items-center justify-between">
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
                    <div class="flex items-center gap-2">
                      <button
                        class="btn-ghost text-xs py-1 px-2"
                        onClick={() => triggerScan(lib.id)}
                        disabled={isActive() || isTriggeringThis()}
                        title="Scan library"
                      >
                        <RefreshCw class={`w-3.5 h-3.5 ${isActive() || isTriggeringThis() ? 'animate-spin' : ''}`} />
                        {isActive() ? 'Scanning…' : 'Scan'}
                      </button>
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
                  </div>
                  <Show when={isActive()}>
                    <div class="mt-3 space-y-1.5">
                      <div class="flex items-center justify-between text-xs text-surface-600">
                        <span class="truncate max-w-xs">{progress()?.current_item || 'Scanning…'}</span>
                        <span class="ml-2 flex-shrink-0">
                          {progress()?.files_probed ?? 0} / {progress()?.total_files ?? 0}
                          {' '}({progress()?.percent ?? 0}%)
                        </span>
                      </div>
                      <div class="w-full h-1.5 bg-surface-300 rounded-full overflow-hidden">
                        <div
                          class="h-full bg-ferrite-500 rounded-full transition-all duration-500"
                          style={{ width: `${progress()?.percent ?? 0}%` }}
                        />
                      </div>
                      <div class="flex items-center gap-3 text-xs text-surface-600">
                        <Show when={(progress()?.items_enriched ?? 0) > 0}>
                          <span>{progress()!.items_enriched} enriched</span>
                        </Show>
                        <Show when={(progress()?.subtitles_extracted ?? 0) > 0}>
                          <span>{progress()!.subtitles_extracted} subtitles</span>
                        </Show>
                        <Show when={(progress()?.errors ?? 0) > 0}>
                          <span class="text-red-400">{progress()!.errors} errors</span>
                        </Show>
                        <span class="ml-auto">
                          {Math.floor((progress()?.elapsed_seconds ?? 0) / 60)}m {(progress()?.elapsed_seconds ?? 0) % 60}s
                          {progress()?.estimated_remaining_seconds != null && (
                            <span class="text-surface-500"> — ~{Math.floor(progress()!.estimated_remaining_seconds! / 60)}m remaining</span>
                          )}
                        </span>
                      </div>
                    </div>
                  </Show>
                </div>
              );
            }}
          </For>
        </div>
      </section>

      {/* User Management — admin only */}
      <Show when={currentUser()?.is_admin === 1}>
        <section class="mb-8">
          <div class="flex items-center justify-between mb-3">
            <h2 class="text-sm font-semibold text-surface-800 uppercase tracking-wider">Users</h2>
            <button class="btn-primary text-xs py-1.5 px-3" onClick={() => setShowCreateUser(true)}>
              <UserPlus class="w-3.5 h-3.5" /> Add User
            </button>
          </div>

          <div class="space-y-2">
            <For each={users()} fallback={
              <div class="card p-6 text-center">
                <Users class="w-8 h-8 text-surface-500 mx-auto mb-2" />
                <p class="text-sm text-surface-700">No users found</p>
              </div>
            }>
              {(user) => (
                <div class="card p-4 flex items-center justify-between group">
                  <div class="flex items-center gap-3">
                    <div class="w-9 h-9 rounded-full bg-surface-200 flex items-center justify-center text-sm font-bold text-surface-700">
                      {(user.display_name || user.username).charAt(0).toUpperCase()}
                    </div>
                    <div>
                      <div class="flex items-center gap-2">
                        <span class="text-sm font-medium text-gray-300">{user.display_name || user.username}</span>
                        <Show when={user.display_name}>
                          <span class="text-xs text-surface-600">@{user.username}</span>
                        </Show>
                        <Show when={user.is_admin === 1}>
                          <span class="flex items-center gap-0.5 text-[0.65rem] font-semibold text-amber-400 bg-amber-400/10 px-1.5 py-0.5 rounded">
                            <ShieldCheck class="w-3 h-3" /> Admin
                          </span>
                        </Show>
                        <Show when={user.id === currentUser()?.id}>
                          <span class="text-[0.65rem] text-ferrite-400 bg-ferrite-500/10 px-1.5 py-0.5 rounded">You</span>
                        </Show>
                      </div>
                      <div class="text-xs text-surface-600">
                        {user.last_login_at
                          ? `Last login: ${new Date(user.last_login_at).toLocaleDateString()}`
                          : `Joined: ${new Date(user.created_at).toLocaleDateString()}`}
                      </div>
                    </div>
                  </div>
                  <div class="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
                    <button
                      class="btn-icon text-surface-600 hover:text-blue-400"
                      onClick={() => setResetTarget(user)}
                      title="Reset password"
                    >
                      <KeyRound class="w-4 h-4" />
                    </button>
                    <Show when={user.id !== currentUser()?.id}>
                      <button
                        class="btn-icon text-surface-600 hover:text-red-400"
                        onClick={() => handleDeleteUser(user)}
                        title="Delete user"
                      >
                        <Trash2 class="w-4 h-4" />
                      </button>
                    </Show>
                  </div>
                </div>
              )}
            </For>
          </div>
        </section>
      </Show>

      {/* Playback Preferences */}
      <section class="mb-8">
        <div class="flex items-center gap-2 mb-3">
          <h2 class="text-sm font-semibold text-surface-800 uppercase tracking-wider">Playback Preferences</h2>
          <Show when={prefsSaving()}>
            <span class="text-xs text-surface-600 ml-auto">Saving…</span>
          </Show>
          <Show when={prefsSaved()}>
            <span class="text-xs text-green-400 ml-auto">Saved</span>
          </Show>
        </div>
        <div class="card p-4 space-y-5">
          <div class="flex items-center gap-3">
            <div class="w-9 h-9 rounded-lg bg-surface-200 flex items-center justify-center flex-shrink-0">
              <Sliders class="w-4 h-4 text-surface-700" />
            </div>
            <div class="flex-1 grid grid-cols-1 sm:grid-cols-3 gap-4">
              <div>
                <label class="block text-xs font-medium text-gray-400 mb-1.5">Default Subtitle Language</label>
                <select
                  class="input-field text-sm"
                  value={prefs().default_subtitle_language ?? ''}
                  onChange={(e) => savePrefs({ default_subtitle_language: e.currentTarget.value || undefined })}
                >
                  <option value="">None (off by default)</option>
                  <option value="en">English</option>
                  <option value="es">Spanish</option>
                  <option value="fr">French</option>
                  <option value="de">German</option>
                  <option value="ja">Japanese</option>
                  <option value="ko">Korean</option>
                  <option value="zh">Chinese</option>
                  <option value="pt">Portuguese</option>
                  <option value="it">Italian</option>
                  <option value="ru">Russian</option>
                </select>
              </div>
              <div>
                <label class="block text-xs font-medium text-gray-400 mb-1.5">Default Audio Language</label>
                <select
                  class="input-field text-sm"
                  value={prefs().default_audio_language ?? ''}
                  onChange={(e) => savePrefs({ default_audio_language: e.currentTarget.value || undefined })}
                >
                  <option value="">Original (default track)</option>
                  <option value="en">English</option>
                  <option value="es">Spanish</option>
                  <option value="fr">French</option>
                  <option value="de">German</option>
                  <option value="ja">Japanese</option>
                  <option value="ko">Korean</option>
                  <option value="zh">Chinese</option>
                  <option value="pt">Portuguese</option>
                  <option value="it">Italian</option>
                  <option value="ru">Russian</option>
                </select>
              </div>
              <div>
                <label class="block text-xs font-medium text-gray-400 mb-1.5">Max Streaming Quality</label>
                <select
                  class="input-field text-sm"
                  value={prefs().max_quality ?? ''}
                  onChange={(e) => savePrefs({ max_quality: e.currentTarget.value || undefined })}
                >
                  <option value="">No limit (best available)</option>
                  <option value="480p">480p</option>
                  <option value="720p">720p</option>
                  <option value="1080p">1080p</option>
                  <option value="4k">4K</option>
                </select>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* Add Library Dialog */}
      <Show when={showAddDialog()}>
        <AddLibraryDialog onClose={() => setShowAddDialog(false)} />
      </Show>

      {/* Create User Dialog */}
      <Show when={showCreateUser()}>
        <CreateUserDialog
          onClose={() => setShowCreateUser(false)}
          onCreated={(user) => { setUsers([...users(), user]); setShowCreateUser(false); }}
        />
      </Show>

      {/* Reset Password Dialog */}
      <Show when={resetTarget()}>
        <ResetPasswordDialog
          user={resetTarget()!}
          onClose={() => setResetTarget(null)}
        />
      </Show>
    </div>
  );
}

function CreateUserDialog(props: { onClose: () => void; onCreated: (user: User) => void }) {
  const [username, setUsername] = createSignal('');
  const [displayName, setDisplayName] = createSignal('');
  const [password, setPassword] = createSignal('');
  const [isAdmin, setIsAdmin] = createSignal(false);
  const [loading, setLoading] = createSignal(false);
  const [error, setError] = createSignal('');

  async function handleSubmit(e: Event) {
    e.preventDefault();
    if (!username().trim() || !password().trim()) return;
    setLoading(true);
    setError('');
    try {
      const user = await api.createUser(
        username().trim(),
        password(),
        displayName().trim() || null,
        isAdmin(),
      );
      props.onCreated(user);
    } catch (err: any) {
      setError(err.message || 'Failed to create user');
    } finally {
      setLoading(false);
    }
  }

  return (
    <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm animate-fade-in" onClick={props.onClose}>
      <form
        class="card p-6 w-full max-w-md space-y-4 animate-scale-in"
        onClick={(e) => e.stopPropagation()}
        onSubmit={handleSubmit}
      >
        <h2 class="text-lg font-semibold text-white">Create User</h2>

        <Show when={error()}>
          <div class="text-sm text-red-400 bg-red-400/10 border border-red-400/20 rounded-lg px-3 py-2">{error()}</div>
        </Show>

        <div>
          <label class="block text-sm font-medium text-gray-400 mb-1.5">Username</label>
          <input class="input-field" placeholder="username" value={username()} onInput={(e) => setUsername(e.currentTarget.value)} autofocus required />
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-400 mb-1.5">Display Name <span class="text-surface-600">(optional)</span></label>
          <input class="input-field" placeholder="Full Name" value={displayName()} onInput={(e) => setDisplayName(e.currentTarget.value)} />
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-400 mb-1.5">Password</label>
          <input class="input-field" type="password" placeholder="••••••••" value={password()} onInput={(e) => setPassword(e.currentTarget.value)} required />
        </div>

        <label class="flex items-center gap-3 cursor-pointer">
          <input type="checkbox" class="w-4 h-4 rounded accent-ferrite-500" checked={isAdmin()} onChange={(e) => setIsAdmin(e.currentTarget.checked)} />
          <span class="text-sm text-gray-300 flex items-center gap-1.5">
            <Shield class="w-4 h-4 text-amber-400" /> Admin privileges
          </span>
        </label>

        <div class="flex justify-end gap-3 pt-2">
          <button type="button" class="btn-secondary" onClick={props.onClose}>Cancel</button>
          <button type="submit" class="btn-primary" disabled={loading()}>
            {loading() ? <div class="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin" /> : 'Create User'}
          </button>
        </div>
      </form>
    </div>
  );
}

function ResetPasswordDialog(props: { user: User; onClose: () => void }) {
  const [newPassword, setNewPassword] = createSignal('');
  const [confirm, setConfirm] = createSignal('');
  const [loading, setLoading] = createSignal(false);
  const [error, setError] = createSignal('');
  const [done, setDone] = createSignal(false);

  async function handleSubmit(e: Event) {
    e.preventDefault();
    if (newPassword() !== confirm()) { setError('Passwords do not match'); return; }
    if (newPassword().length < 4) { setError('Password must be at least 4 characters'); return; }
    setLoading(true);
    setError('');
    try {
      await api.adminResetPassword(props.user.id, newPassword());
      setDone(true);
    } catch (err: any) {
      setError(err.message || 'Failed to reset password');
    } finally {
      setLoading(false);
    }
  }

  return (
    <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm animate-fade-in" onClick={props.onClose}>
      <form
        class="card p-6 w-full max-w-sm space-y-4 animate-scale-in"
        onClick={(e) => e.stopPropagation()}
        onSubmit={handleSubmit}
      >
        <h2 class="text-lg font-semibold text-white">Reset Password</h2>
        <p class="text-sm text-surface-700">Set a new password for <span class="text-gray-300 font-medium">{props.user.display_name || props.user.username}</span>.</p>

        <Show when={error()}>
          <div class="text-sm text-red-400 bg-red-400/10 border border-red-400/20 rounded-lg px-3 py-2">{error()}</div>
        </Show>

        <Show when={done()}>
          <div class="text-sm text-green-400 bg-green-400/10 border border-green-400/20 rounded-lg px-3 py-2">Password reset successfully.</div>
        </Show>

        <Show when={!done()}>
          <div>
            <label class="block text-sm font-medium text-gray-400 mb-1.5">New Password</label>
            <input class="input-field" type="password" placeholder="••••••••" value={newPassword()} onInput={(e) => setNewPassword(e.currentTarget.value)} autofocus required />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-400 mb-1.5">Confirm Password</label>
            <input class="input-field" type="password" placeholder="••••••••" value={confirm()} onInput={(e) => setConfirm(e.currentTarget.value)} required />
          </div>
        </Show>

        <div class="flex justify-end gap-3 pt-2">
          <button type="button" class="btn-secondary" onClick={props.onClose}>{done() ? 'Close' : 'Cancel'}</button>
          <Show when={!done()}>
            <button type="submit" class="btn-primary" disabled={loading()}>
              {loading() ? <div class="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin" /> : 'Reset Password'}
            </button>
          </Show>
        </div>
      </form>
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
