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

<div
  class="flex min-w-0 flex-1 items-center gap-3 rounded-pill border border-border-strong bg-surface-raised px-[18px] py-[11px] shadow-sm transition focus-within:border-focus focus-within:ring-[3px] focus-within:ring-focus/25"
>
  <Search class="h-[19px] w-[19px] shrink-0 text-brand" />
  <input
    name="search"
    placeholder="Rechercher par morceau, artiste, style ou tag…"
    aria-label="Rechercher"
    class="min-w-0 flex-1 border-0 bg-transparent font-sans text-[15.5px] text-ink outline-none placeholder:text-ink-subtle"
    onkeydown={(e) => search(e)}
  />
</div>
