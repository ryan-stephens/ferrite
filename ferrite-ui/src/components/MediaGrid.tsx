import { For, Show } from 'solid-js';
import type { MediaItem } from '../api';
import MediaCard from './MediaCard';

interface MediaGridProps {
  items: MediaItem[];
  searchText: string;
  hasAnyMedia: boolean;
  onShowDetail: (id: string) => void;
}

export default function MediaGrid(props: MediaGridProps) {
  return (
    <>
      <Show when={props.items.length === 0}>
        <div class="text-center py-16 text-gray-500">
          <h2 class="text-xl font-semibold mb-2">
            {props.searchText
              ? 'No results'
              : props.hasAnyMedia
                ? 'No items in this library'
                : 'No media yet'}
          </h2>
          <p class="text-sm">
            {props.searchText
              ? `No items match "${props.searchText}". Try a different search.`
              : props.hasAnyMedia
                ? 'Try scanning the library or adding media files to its folder.'
                : 'Add a library to get started. Point it at a folder containing your media files.'}
          </p>
        </div>
      </Show>

      <Show when={props.items.length > 0}>
        <div class="grid grid-cols-[repeat(auto-fill,minmax(180px,1fr))] gap-5">
          <For each={props.items}>
            {item => <MediaCard item={item} onClick={() => props.onShowDetail(item.id)} />}
          </For>
        </div>
      </Show>
    </>
  );
}
