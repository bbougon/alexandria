<script lang="ts">
  import { searchResultStore } from './searchResult.store';
  import { Folder } from '@lucide/svelte';
  import VideoCard from '../components/VideoCard.svelte';
</script>

<div>
  <div class="mb-8">
    <div class="flex items-center gap-3 mb-2">
      <Folder class="w-8 h-8 text-blue-500" />
      <h1 class="text-3xl font-semibold">Search results</h1>
    </div>
    <p class="text-muted-foreground">
      {$searchResultStore.length}
      {$searchResultStore.length === 1 ? 'result' : 'results'} found
    </p>
  </div>

  {#if $searchResultStore.length === 0}
    <div class="text-center py-16">
      <Folder class="w-16 h-16 mx-auto mb-4 text-gray-300" />
      <p class="text-lg text-muted-foreground">No videos match your search</p>
    </div>
  {:else}
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
      {#each $searchResultStore as video}
        <VideoCard {video} />
      {/each}
    </div>
  {/if}
</div>
