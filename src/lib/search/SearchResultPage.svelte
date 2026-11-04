<script lang="ts">
  import { searchResultStore } from './searchResult.store';
  import { Search } from '@lucide/svelte';
  import VideoCard from '../components/VideoCard.svelte';
</script>

<div>
  <div class="mb-8">
    <div class="mb-2 flex items-center gap-3">
      <Search class="h-8 w-8 text-brand" />
      <h1 class="text-3xl font-semibold">Search results</h1>
    </div>
    <p class="text-ink-muted">
      {$searchResultStore.length}
      {$searchResultStore.length === 1 ? 'result' : 'results'} found
    </p>
  </div>

  {#if $searchResultStore.length === 0}
    <div class="py-16 text-center">
      <Search class="mx-auto mb-4 h-16 w-16 text-ink-subtle" />
      <p class="text-lg text-ink-muted">No videos match your search</p>
    </div>
  {:else}
    <div
      class="grid gap-4 [grid-template-columns:repeat(auto-fill,minmax(360px,1fr))]"
    >
      {#each $searchResultStore as video}
        <VideoCard {video} />
      {/each}
    </div>
  {/if}
</div>
