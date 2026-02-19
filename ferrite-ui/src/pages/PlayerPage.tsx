import { createSignal, createEffect, Show, Switch, Match } from 'solid-js';
import { useParams, useNavigate, useSearchParams } from '@solidjs/router';
import { api } from '../api';
import type { MediaItem } from '../api';
import Player from '../components/Player';

export default function PlayerPage() {
  const params = useParams<{ id: string }>();
  const [searchParams] = useSearchParams();
  const navigate = useNavigate();
  const [item, setItem] = createSignal<MediaItem | null>(null);
  const [loadedId, setLoadedId] = createSignal<string | null>(null);

  const resumePos = () => {
    const r = searchParams.resume;
    if (r) return parseFloat(r);
    return null;
  };

  const isEpisode = () => (item()?.is_episode ?? 0) === 1;

  // Re-fetch whenever the route param changes (handles Up Next navigation)
  createEffect(() => {
    const id = params.id;
    setItem(null);
    setLoadedId(null);
    api.getMedia(id).then(data => {
      setItem(data);
      setLoadedId(id);
    }).catch(() => navigate('/'));
  });

  function handleClose() {
    if (window.history.length > 1) {
      navigate(-1);
    } else {
      navigate('/');
    }
  }

  function handleNextEpisode(mediaItemId: string) {
    navigate(`/player/${mediaItemId}`);
  }

  // Use Switch/Match keyed on loadedId so Player fully unmounts+remounts on episode change
  return (
    <Switch fallback={
      <div class="fixed inset-0 bg-black flex items-center justify-center z-[100]">
        <div class="w-8 h-8 border-2 border-surface-400 border-t-ferrite-500 rounded-full animate-spin" />
      </div>
    }>
      <Match when={item() && loadedId() === params.id && loadedId()}>
        {(id) => (
          <Player
            item={item()!}
            resumePosition={resumePos()}
            isEpisode={isEpisode()}
            onClose={handleClose}
            onNextEpisode={handleNextEpisode}
          />
        )}
      </Match>
    </Switch>
  );
}
