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

<div class="relative w-full max-w-md">
  <Search
    class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground"
  />
  <form action="#" method="GET" class="pl-10 bg-white">
    <input
      name="search"
      placeholder="Search"
      aria-label="Search"
      class="col-start-1 row-start-1 block size-full bg-white pl-8 text-base text-gray-900 outline-hidden placeholder:text-gray-400 sm:text-sm/6 dark:bg-gray-900 dark:text-white dark:placeholder:text-gray-500"
      onkeydown={(e) => search(e)}
    />
  </form>
</div>
