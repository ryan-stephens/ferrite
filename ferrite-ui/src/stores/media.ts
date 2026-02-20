import { createSignal } from 'solid-js';
import { api, type Library, type MediaItem } from '../api';

const [libraries, setLibraries] = createSignal<Library[]>([]);
const [allMedia, setAllMedia] = createSignal<MediaItem[]>([]);
const [totalCount, setTotalCount] = createSignal(0);
const [loading, setLoading] = createSignal(false);
const [scanning, setScanning] = createSignal(false);
const [statusMessage, setStatusMessage] = createSignal('Ready');

let _scanPollTimer: ReturnType<typeof setInterval> | null = null;

function _stopScanPoll() {
  if (_scanPollTimer) { clearInterval(_scanPollTimer); _scanPollTimer = null; }
}

async function _checkScansDone(libs: Library[]): Promise<void> {
  try {
    const statuses = await Promise.all(libs.map(lib => api.scanStatus(lib.id).catch(() => null)));
    const anyActive = statuses.some(s => s?.scanning);
    if (anyActive) {
      setStatusMessage('Scanning…');
    } else {
      _stopScanPoll();
      setScanning(false);
      setStatusMessage('Ready');
    }
  } catch {
    _stopScanPoll();
    setScanning(false);
    setStatusMessage('Ready');
  }
}

async function loadLibraries(): Promise<void> {
  try {
    const libs = await api.listLibraries();
    setLibraries(libs);
  } catch { /* handled by 401 listener */ }
}

async function loadMedia(libraryId?: string | null): Promise<void> {
  setLoading(true);
  try {
    const params: Record<string, string> = {};
    if (libraryId) params.library_id = libraryId;
    const data = await api.listMedia(params);
    setAllMedia(data.items);
    setTotalCount(data.total);
  } catch { /* handled by 401 listener */ }
  setLoading(false);
}

async function addLibrary(name: string, path: string, type: string): Promise<void> {
  const lib = await api.createLibrary(name, path, type);
  await api.scanLibrary(lib.id);
  await loadLibraries();
}

async function deleteLibrary(id: string): Promise<void> {
  await api.deleteLibrary(id);
  await loadLibraries();
  await loadMedia();
}

async function refreshAll(): Promise<void> {
  setScanning(true);
  setStatusMessage('Triggering scans…');
  const libs = await api.listLibraries();
  await Promise.all(libs.map(lib => api.scanLibrary(lib.id).catch(() => {})));
  setStatusMessage('Scanning…');
  _stopScanPoll();
  _scanPollTimer = setInterval(() => _checkScansDone(libs), 2000);
}

export {
  libraries, allMedia, totalCount, loading, scanning, statusMessage,
  setStatusMessage, loadLibraries, loadMedia, addLibrary, deleteLibrary, refreshAll,
};
