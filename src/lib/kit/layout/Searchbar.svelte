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

<div class="search">
  <Search class="s" />
  <input
    name="search"
    placeholder="Rechercher par morceau, artiste, style ou tag…"
    aria-label="Rechercher"
    onkeydown={(e) => search(e)}
  />
</div>
