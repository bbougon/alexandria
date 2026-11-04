<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { type VideoFromCollection } from '../../collections/collection.tauri';
  import { listen } from '@tauri-apps/api/event';
  import { toVideo } from '../../collections/video.tauri';
  import { pageStore } from '../pages/pageStore';
  import { searchResultStore } from '../../search/searchResult.store';
  import { Search } from '@lucide/svelte';

  const search = async (e: KeyboardEvent) => {
    if (e.key === 'Enter') {
      e.preventDefault();
      searchResultStore.initialize();
      pageStore.goTo('SearchResultPage');
      const unlistenToCollectionCreated = await listen<VideoFromCollection>(
        'video:selected',
        (e) => {
          const videoFromCollection = e.payload;
          searchResultStore.addVideo(toVideo(videoFromCollection));
        }
      );
      await invoke<VideoFromCollection[]>('search_videos', {
        query: (e.target as HTMLInputElement).value,
      });

      unlistenToCollectionCreated();
    }
  };
</script>

<div class="relative w-full max-w-2xl">
  <Search
    class="pointer-events-none absolute left-3.5 top-1/2 size-4 -translate-y-1/2 text-brand"
  />
  <input
    name="search"
    placeholder="Search by title, artist, song…"
    aria-label="Search"
    class="h-10 w-full rounded-full border border-border-strong bg-surface-raised pl-10 pr-4 text-sm text-ink outline-none transition-colors placeholder:text-ink-subtle focus-visible:border-focus focus-visible:ring-2 focus-visible:ring-focus/40"
    onkeydown={(e) => search(e)}
  />
</div>
