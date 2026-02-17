import { createSignal, Show, onMount } from 'solid-js';
import { useParams, useNavigate, useSearchParams } from '@solidjs/router';
import { api } from '../api';
import type { MediaItem } from '../api';
import Player from '../components/Player';

export default function PlayerPage() {
  const params = useParams<{ id: string }>();
  const [searchParams] = useSearchParams();
  const navigate = useNavigate();
  const [item, setItem] = createSignal<MediaItem | null>(null);

  const resumePos = () => {
    const r = searchParams.resume;
    if (r) return parseFloat(r);
    return null;
  };

  onMount(async () => {
    try {
      const data = await api.getMedia(params.id);
      setItem(data);
    } catch {
      navigate('/');
    }
  });

  function handleClose() {
    if (window.history.length > 1) {
      navigate(-1);
    } else {
      navigate('/');
    }
  }

  return (
    <Show when={item()} fallback={
      <div class="fixed inset-0 bg-black flex items-center justify-center z-[100]">
        <div class="w-8 h-8 border-2 border-surface-400 border-t-ferrite-500 rounded-full animate-spin" />
      </div>
    }>
      <Player
        item={item()!}
        resumePosition={resumePos()}
        onClose={handleClose}
      />
    </Show>
  );
}
