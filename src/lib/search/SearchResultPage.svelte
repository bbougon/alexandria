<script lang="ts">
  import { searchResultStore } from './searchResult.store';
  import VideoCard from '../components/VideoCard.svelte';
</script>

<div
  class="flex flex-col gap-[18px] pb-[calc(32px_+_env(safe-area-inset-bottom,0px))] pl-[max(20px,env(safe-area-inset-left,0px))] pr-[max(20px,env(safe-area-inset-right,0px))] pt-[22px]"
>
  <div class="flex flex-wrap items-end gap-[14px]">
    <div>
      <h2 class="font-display text-[26px] font-bold tracking-[-0.02em]">
        Résultats de recherche
      </h2>
      <p class="mt-1 text-sm text-ink-muted">
        {$searchResultStore.length}
        {$searchResultStore.length === 1 ? 'résultat' : 'résultats'}
      </p>
    </div>
  </div>

  {#if $searchResultStore.length === 0}
    <div class="px-5 py-[60px] text-center text-ink-muted">
      <h3 class="mb-1.5 font-display text-[22px] text-ink">Aucune vidéo trouvée</h3>
      <p>Essaie un autre morceau, artiste, style ou tag.</p>
    </div>
  {:else}
    <div
      class="grid grid-cols-[repeat(auto-fill,minmax(380px,1fr))] gap-4 max-[760px]:grid-cols-1"
    >
      {#each $searchResultStore as video}
        <VideoCard {video} />
      {/each}
    </div>
  {/if}
</div>
