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

<div class="w-64 bg-white border-r border-gray-200 flex flex-col h-full">
  <div class="p-4 border-b border-gray-200">
    <h2 class="font-semibold text-lg mb-3">Collections</h2>
    <Button onclick={() => goToCreateCollection()}>
      <Plus class="w-4 h-4 mr-2" />
      New Collection
    </Button>
  </div>

  <div class="flex-1">
    <div class="p-2">
      <div class="space-y-1">
        {#each $collectionsStore as collection}
          <div
            class={`group relative rounded-lg transition-colors cursor-pointer ${$selectedCollectionId === collection.id ? 'bg-blue-50 text-blue-700' : 'hover:bg-gray-100'}`}
          >
            <div class="flex items-center gap-3 p-3 pr-10">
              <Folder class="w-5 h-5 flex-shrink-0" />
              <div class="flex-1 min-w-0" onclick={() => goToCollection(collection)}>
                <p class="font-medium truncate">{collection.title}</p>
                <p class="text-xs text-muted-foreground">
                  {collection.videos.length} video{collection.videos.length !== 1
                    ? 's'
                    : ''}
                </p>
              </div>
            </div>
            <!--                <button-->
            <!--                  onClick={(e) => {-->
            <!--                    e.preventDefault();-->
            <!--                    if (confirm(`Delete collection "${collection.name}"?`)) {-->
            <!--                      deleteCollection(collection.id);-->
            <!--                    }-->
            <!--                  }}-->
            <!--                  class="absolute right-2 top-1/2 -translate-y-1/2 p-1.5 rounded opacity-0 group-hover:opacity-100 hover:bg-red-50 transition-opacity"-->
            <!--                >-->
            <!--                  <Trash2 class="w-4 h-4 text-red-500" />-->
            <!--                </button>-->
          </div>
        {/each}
      </div>
    </div>
  </div>
</div>
