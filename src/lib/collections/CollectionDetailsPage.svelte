<script lang="ts">
  import { selectedCollection } from './collection.store';
  import { Folder } from '@lucide/svelte';
  import VideoCard from '../components/VideoCard.svelte';
</script>

<div>
  {#if $selectedCollection.collection}
    <div class="mb-8">
      <div class="flex items-center gap-3 mb-2">
        <Folder class="w-8 h-8 text-blue-500" />
        <h1 class="text-3xl font-semibold">
          {$selectedCollection.collection.title}
        </h1>
      </div>
      <p class="text-muted-foreground">
        {$selectedCollection.collection.videos.length} video{$selectedCollection
          .collection.videos.length !== 1
          ? 's'
          : ''}
      </p>
    </div>

    <div class="space-y-4">
      <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
        {#each $selectedCollection.collection.videos as video}
          <VideoCard bind:video />
        {/each}
      </div>
    </div>
  {/if}
</div>
