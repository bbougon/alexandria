<script lang="ts">
  import { searchResultStore } from './searchResult.store';
  import VideoCard from '../components/VideoCard.svelte';
</script>

<div class="content">
  <div class="collhead">
    <div>
      <h2>Résultats de recherche</h2>
      <p class="sub">
        {$searchResultStore.length}
        {$searchResultStore.length === 1 ? 'résultat' : 'résultats'}
      </p>
    </div>
  </div>

  {#if $searchResultStore.length === 0}
    <div class="empty">
      <h3>Aucune vidéo trouvée</h3>
      <p>Essaie un autre morceau, artiste, style ou tag.</p>
    </div>
  {:else}
    <div class="grid">
      {#each $searchResultStore as video}
        <VideoCard {video} />
      {/each}
    </div>
  {/if}
</div>
