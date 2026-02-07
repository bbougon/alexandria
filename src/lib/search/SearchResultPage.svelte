<script lang="ts">
  import { searchResultStore } from './searchResult.store';
  import VideoCard from './VideoCard.svelte';
  import VideoDialog from '../components/VideoDialog.svelte';
  import type { Video } from '../collections/video';

  let activeVideo = $state<Video | null>(null);
</script>

<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
  <div class="mb-6">
    <p class="text-muted-foreground">
      {$searchResultStore.length}
      {$searchResultStore.length === 1 ? 'result' : 'results'} found
    </p>
  </div>

  {#if $searchResultStore.length > 0}
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      {#each $searchResultStore as video}
        <VideoCard {video} onVideo={() => (activeVideo = video)} />
      {/each}
    </div>
    <VideoDialog video={activeVideo} />
  {:else}
    <div class="text-center py-12">
      <p class="text-muted-foreground">No videos found matching your search.</p>
    </div>
  {/if}
</div>
