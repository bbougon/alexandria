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

<div>
  <div class="mb-8">
    <h1 class="mb-2 text-3xl font-semibold">Your library</h1>
    <p class="text-ink-muted">Manage your video collections and metadata</p>
  </div>

  <div class="mb-8 grid grid-cols-2 gap-4">
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
          <p class="text-sm text-ink-muted">Total Videos</p>
        </div>
      </div>
    </Card>
  </div>
</div>
