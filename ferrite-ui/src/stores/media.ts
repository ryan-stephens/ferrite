import { createSignal } from 'solid-js';
import { api, type Library, type MediaItem } from '../api';

const [libraries, setLibraries] = createSignal<Library[]>([]);
const [allMedia, setAllMedia] = createSignal<MediaItem[]>([]);
const [totalCount, setTotalCount] = createSignal(0);
const [loading, setLoading] = createSignal(false);
const [scanning, setScanning] = createSignal(false);
const [statusMessage, setStatusMessage] = createSignal('Ready');

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
  await api.createLibrary(name, path, type);
  await loadLibraries();
}

async function deleteLibrary(id: string): Promise<void> {
  await api.deleteLibrary(id);
  await loadLibraries();
  await loadMedia();
}

async function refreshAll(): Promise<void> {
  setScanning(true);
  setStatusMessage('Triggering scans...');
  const libs = await api.listLibraries();
  await Promise.all(libs.map(lib => api.scanLibrary(lib.id).catch(() => {})));
  setStatusMessage('Scans started');
  setTimeout(() => { setScanning(false); setStatusMessage('Ready'); }, 3000);
}

export {
  libraries, allMedia, totalCount, loading, scanning, statusMessage,
  setStatusMessage, loadLibraries, loadMedia, addLibrary, deleteLibrary, refreshAll,
};
