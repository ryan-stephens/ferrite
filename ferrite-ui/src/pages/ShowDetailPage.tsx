import { createSignal, For, Show, onMount } from 'solid-js';
import { useParams, useNavigate } from '@solidjs/router';
import { Play, ArrowLeft, Star, Clock, ChevronDown, CheckCircle2, Tv } from 'lucide-solid';
import { api, authUrl } from '../api';
import type { TvShow, Season, Episode } from '../api';
import { formatDuration, getResLabel, getStreamType } from '../utils';

export default function ShowDetailPage() {
  const params = useParams<{ id: string }>();
  const navigate = useNavigate();

  const [show, setShow] = createSignal<TvShow | null>(null);
  const [seasons, setSeasons] = createSignal<Season[]>([]);
  const [episodesBySeason, setEpisodesBySeason] = createSignal<Record<string, Episode[]>>({});
  const [expandedSeason, setExpandedSeason] = createSignal<string | null>(null);
  const [loadingEpisodes, setLoadingEpisodes] = createSignal<Record<string, boolean>>({});

  onMount(async () => {
    try {
      const [showData, seasonsData] = await Promise.all([
        api.getShow(params.id),
        api.listSeasons(params.id),
      ]);
      setShow(showData);
      setSeasons(seasonsData);
      // Auto-expand the first season
      if (seasonsData.length > 0) {
        expandSeason(seasonsData[0].id);
      }
    } catch {
      navigate(-1);
    }
  });

  async function expandSeason(seasonId: string) {
    setExpandedSeason(seasonId);
    if (episodesBySeason()[seasonId]) return; // already loaded
    setLoadingEpisodes(prev => ({ ...prev, [seasonId]: true }));
    try {
      const eps = await api.listEpisodes(seasonId);
      setEpisodesBySeason(prev => ({ ...prev, [seasonId]: eps }));
    } catch {
      setEpisodesBySeason(prev => ({ ...prev, [seasonId]: [] }));
    } finally {
      setLoadingEpisodes(prev => ({ ...prev, [seasonId]: false }));
    }
  }

  function toggleSeason(seasonId: string) {
    if (expandedSeason() === seasonId) {
      setExpandedSeason(null);
    } else {
      expandSeason(seasonId);
    }
  }

  function playEpisode(ep: Episode, resume = false) {
    const pos = resume && ep.position_ms ? ep.position_ms / 1000 : null;
    if (pos !== null) {
      navigate(`/player/${ep.media_item_id}?resume=${pos}`);
    } else {
      navigate(`/player/${ep.media_item_id}`);
    }
  }

  const posterUrl = () => show()?.poster_path ? authUrl(`/api/images/${show()!.poster_path}`) : null;
  const backdropUrl = () => show()?.backdrop_path ? authUrl(`/api/images/${show()!.backdrop_path}`) : null;

  return (
    <Show when={show()} fallback={
      <div class="flex items-center justify-center h-96">
        <div class="w-8 h-8 border-2 border-surface-400 border-t-ferrite-500 rounded-full animate-spin" />
      </div>
    }>
      <div class="animate-fade-in">
        {/* Backdrop hero */}
        <div class="relative h-[420px] overflow-hidden">
          <Show when={backdropUrl() || posterUrl()}>
            <img
              src={(backdropUrl() || posterUrl())!}
              alt=""
              class="absolute inset-0 w-full h-full object-cover opacity-25 blur-2xl scale-110"
            />
          </Show>
          <div class="absolute inset-0 bg-gradient-to-t from-surface via-surface/70 to-surface/10" />
          <div class="absolute inset-0 bg-gradient-to-r from-surface/80 via-surface/30 to-transparent" />

          {/* Back button */}
          <button
            class="absolute top-6 left-6 btn-ghost text-white z-10"
            onClick={() => window.history.length > 1 ? navigate(-1) : navigate('/')}
          >
            <ArrowLeft class="w-5 h-5" /> Back
          </button>

          {/* Show info */}
          <div class="relative h-full flex items-end px-8 pb-10">
            <div class="flex gap-8 items-end max-w-5xl w-full">
              {/* Poster */}
              <Show when={posterUrl()}>
                <div class="flex-shrink-0">
                  <img
                    src={posterUrl()!}
                    alt={show()!.title}
                    class="w-40 h-60 object-cover rounded-xl shadow-2xl shadow-black/60 border border-white/10"
                  />
                </div>
              </Show>

              <div class="flex-1 space-y-3 pb-2">
                {/* Meta */}
                <div class="flex items-center gap-2 flex-wrap text-sm text-surface-800">
                  <Show when={show()!.year}>
                    <span class="font-medium">{show()!.year}</span>
                  </Show>
                  <Show when={show()!.status}>
                    <span class="badge bg-surface-400/50 text-surface-900 border border-surface-500/30">{show()!.status}</span>
                  </Show>
                  <span class="flex items-center gap-1">
                    <Tv class="w-3.5 h-3.5" />
                    {show()!.season_count} {show()!.season_count === 1 ? 'Season' : 'Seasons'} · {show()!.episode_count} Episodes
                  </span>
                </div>

                <h1 class="text-4xl font-bold text-white leading-tight">{show()!.title}</h1>

                {/* Genres */}
                <Show when={show()!.genres}>
                  <div class="flex items-center gap-2 flex-wrap">
                    <For each={show()!.genres!.split(',').map(g => g.trim()).filter(Boolean)}>
                      {(genre) => (
                        <span class="badge bg-surface-300/50 text-surface-900 border border-surface-400/30">{genre}</span>
                      )}
                    </For>
                  </div>
                </Show>

                {/* Overview */}
                <Show when={show()!.overview}>
                  <p class="text-sm text-surface-800 leading-relaxed max-w-2xl line-clamp-3">{show()!.overview}</p>
                </Show>
              </div>
            </div>
          </div>
        </div>

        {/* Seasons + Episodes */}
        <div class="px-8 py-6 space-y-3 max-w-5xl">
          <For each={seasons()}>
            {(season) => {
              const isExpanded = () => expandedSeason() === season.id;
              const episodes = () => episodesBySeason()[season.id] || [];
              const isLoading = () => loadingEpisodes()[season.id];
              const seasonLabel = () => season.title || `Season ${season.season_number}`;

              return (
                <div class="rounded-xl border border-white/8 bg-surface-100/50 overflow-hidden">
                  {/* Season header */}
                  <button
                    class="w-full flex items-center justify-between px-5 py-4 hover:bg-white/5 transition-colors text-left"
                    onClick={() => toggleSeason(season.id)}
                  >
                    <div class="flex items-center gap-3">
                      <Show when={season.poster_path}>
                        <img
                          src={authUrl(`/api/images/${season.poster_path}`)}
                          alt=""
                          class="w-10 h-14 object-cover rounded-lg"
                        />
                      </Show>
                      <div>
                        <h2 class="text-base font-semibold text-white">{seasonLabel()}</h2>
                        <p class="text-xs text-surface-700">{season.episode_count} episodes</p>
                      </div>
                    </div>
                    <ChevronDown
                      class={`w-5 h-5 text-surface-700 transition-transform duration-200 ${isExpanded() ? 'rotate-180' : ''}`}
                    />
                  </button>

                  {/* Episode list */}
                  <Show when={isExpanded()}>
                    <div class="border-t border-white/5">
                      <Show when={isLoading()}>
                        <div class="flex items-center justify-center py-8">
                          <div class="w-6 h-6 border-2 border-surface-400 border-t-ferrite-500 rounded-full animate-spin" />
                        </div>
                      </Show>
                      <Show when={!isLoading()}>
                        <For each={episodes()}>
                          {(ep) => {
                            const hasProgress = () => ep.position_ms && ep.position_ms > 0 && !ep.completed;
                            const progressPct = () => {
                              if (!ep.position_ms || !ep.duration_ms) return 0;
                              return Math.min(100, (ep.position_ms / ep.duration_ms) * 100);
                            };
                            const isCompleted = () => ep.completed === 1;
                            const stillUrl = () => ep.still_path ? authUrl(`/api/images/${ep.still_path}`) : null;
                            const streamType = () => getStreamType(null, ep.video_codec, ep.audio_codec);

                            return (
                              <div
                                class="flex items-start gap-4 px-5 py-4 hover:bg-white/5 transition-colors cursor-pointer group/ep border-b border-white/5 last:border-0"
                                onClick={() => playEpisode(ep, !!hasProgress())}
                              >
                                {/* Thumbnail */}
                                <div class="relative flex-shrink-0 w-36 aspect-video rounded-lg overflow-hidden bg-surface-300">
                                  <Show when={stillUrl()} fallback={
                                    <div class="w-full h-full flex items-center justify-center">
                                      <Play class="w-6 h-6 text-surface-500" />
                                    </div>
                                  }>
                                    <img src={stillUrl()!} alt="" class="w-full h-full object-cover" loading="lazy" />
                                  </Show>
                                  {/* Play overlay */}
                                  <div class="absolute inset-0 bg-black/0 group-hover/ep:bg-black/40 transition-all flex items-center justify-center">
                                    <div class="w-9 h-9 rounded-full bg-ferrite-500/90 flex items-center justify-center opacity-0 group-hover/ep:opacity-100 scale-75 group-hover/ep:scale-100 transition-all">
                                      <Play class="w-4 h-4 text-white fill-white ml-0.5" />
                                    </div>
                                  </div>
                                  {/* Progress bar */}
                                  <Show when={hasProgress()}>
                                    <div class="absolute bottom-0 left-0 right-0 h-1 bg-black/50">
                                      <div class="h-full bg-ferrite-500" style={{ width: `${progressPct()}%` }} />
                                    </div>
                                  </Show>
                                  {/* Completed check */}
                                  <Show when={isCompleted()}>
                                    <div class="absolute top-1 right-1 w-5 h-5 rounded-full bg-black/70 flex items-center justify-center">
                                      <CheckCircle2 class="w-3.5 h-3.5 text-green-400" />
                                    </div>
                                  </Show>
                                </div>

                                {/* Episode info */}
                                <div class="flex-1 min-w-0 pt-0.5">
                                  <div class="flex items-start justify-between gap-2">
                                    <div class="min-w-0">
                                      <p class="text-xs text-surface-700 mb-0.5">
                                        S{String(seasons().find(s => s.id === ep.season_id)?.season_number ?? 1).padStart(2, '0')}E{String(ep.episode_number).padStart(2, '0')}
                                        <Show when={ep.air_date}>
                                          {' · '}{ep.air_date}
                                        </Show>
                                      </p>
                                      <h3 class="text-sm font-semibold text-gray-200 group-hover/ep:text-white transition-colors truncate">
                                        {ep.episode_title || `Episode ${ep.episode_number}`}
                                      </h3>
                                    </div>
                                    <div class="flex items-center gap-2 flex-shrink-0">
                                      <Show when={ep.duration_ms}>
                                        <span class="text-xs text-surface-700 flex items-center gap-1">
                                          <Clock class="w-3 h-3" />{formatDuration(ep.duration_ms)}
                                        </span>
                                      </Show>
                                      <Show when={getResLabel(ep.width, ep.height)}>
                                        <span class="badge bg-surface-300/50 text-surface-800 text-2xs">
                                          {getResLabel(ep.width, ep.height)}
                                        </span>
                                      </Show>
                                    </div>
                                  </div>
                                  <Show when={ep.overview}>
                                    <p class="text-xs text-surface-700 mt-1.5 leading-relaxed line-clamp-2">{ep.overview}</p>
                                  </Show>
                                  {/* Resume hint */}
                                  <Show when={hasProgress()}>
                                    <p class="text-xs text-ferrite-400 mt-1">
                                      Resume from {formatDuration(ep.position_ms)}
                                    </p>
                                  </Show>
                                </div>
                              </div>
                            );
                          }}
                        </For>
                        <Show when={episodes().length === 0}>
                          <p class="text-sm text-surface-700 px-5 py-4">No episodes found for this season.</p>
                        </Show>
                      </Show>
                    </div>
                  </Show>
                </div>
              );
            }}
          </For>
        </div>
      </div>
    </Show>
  );
}
