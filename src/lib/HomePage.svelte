<script lang="ts">
  import Card from './components/Card.svelte';
  import { Folder, Video } from '@lucide/svelte';
  import { collectionsStore } from './collections/collections.store';

  let totalVideos = $derived(
    $collectionsStore.reduce(
      (count, collection) => count + collection.videos.length,
      0
    )
  );
</script>

<div class="content">
  <div class="collhead">
    <div>
      <h2>Votre bibliothèque</h2>
      <p class="sub">Gérez vos collections de vidéos et leurs métadonnées.</p>
    </div>
  </div>

  <div class="grid grid-cols-2 gap-4">
    <Card class="bg-brand-tint p-6">
      <div class="flex items-center gap-4">
        <div class="rounded-lg bg-brand p-3">
          <Folder class="h-6 w-6 text-on-brand" />
        </div>
        <div>
          <p class="text-2xl font-semibold">{$collectionsStore.length}</p>
          <p class="text-sm text-ink-muted">Collections</p>
        </div>
      </div>
    </Card>

    <Card class="bg-surface-sunken p-6">
      <div class="flex items-center gap-4">
        <div class="rounded-lg bg-brand p-3">
          <Video class="h-6 w-6 text-on-brand" />
        </div>
        <div>
          <p class="text-2xl font-semibold">{totalVideos}</p>
          <p class="text-sm text-ink-muted">Vidéos au total</p>
        </div>
      </div>
    </Card>
  </div>
</div>
