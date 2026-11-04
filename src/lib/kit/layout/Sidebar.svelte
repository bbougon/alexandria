<script lang="ts">
  import { collectionsStore } from '../../collections/collections.store';
  import { selectedCollectionId } from '../../collections/collection.store';
  import type { Collection } from '../../collections/collection';
  import { pageStore } from '../pages/pageStore';
  import { Folder, Plus } from '@lucide/svelte';
  import Button from '../../components/Button.svelte';

  const goToCollection = (collection: Collection) => {
    selectedCollectionId.initialize(collection.id);
    pageStore.goTo('CollectionDetailsPage');
  };

  const goToCreateCollection = () => {
    selectedCollectionId.initialize('');
    pageStore.goTo('CollectionCreationPage');
  };
</script>

<div class="flex h-full w-64 flex-col border-r border-border bg-surface-sunken">
  <div class="border-b border-border p-4">
    <h2 class="mb-3 font-display text-lg font-semibold">Collections</h2>
    <Button class="w-full" onclick={() => goToCreateCollection()}>
      <Plus class="mr-2 h-4 w-4" />
      New Collection
    </Button>
  </div>

  <div class="flex-1 overflow-y-auto p-2">
    <div class="space-y-1">
      {#each $collectionsStore as collection}
        <div
          class={`group relative cursor-pointer rounded-lg border-l-2 transition-colors ${
            $selectedCollectionId === collection.id
              ? 'border-brand bg-brand-tint text-brand'
              : 'border-transparent hover:bg-surface'
          }`}
        >
          <div class="flex items-center gap-3 p-3">
            <Folder class="h-5 w-5 flex-shrink-0" />
            <div class="min-w-0 flex-1" onclick={() => goToCollection(collection)}>
              <p class="truncate font-medium">{collection.title}</p>
              <p class="text-xs text-ink-subtle">
                {collection.videos.length} video{collection.videos.length !== 1
                  ? 's'
                  : ''}
              </p>
            </div>
          </div>
        </div>
      {/each}
    </div>
  </div>
</div>
