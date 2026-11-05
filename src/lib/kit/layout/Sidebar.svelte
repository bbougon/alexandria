<script lang="ts">
  import { collectionsStore } from '../../collections/collections.store';
  import { selectedCollectionId } from '../../collections/collection.store';
  import type { Collection } from '../../collections/collection';
  import { pageStore } from '../pages/pageStore';
  import { Folder, Plus, Library } from '@lucide/svelte';
  import ThemeToggle from '../../components/ThemeToggle.svelte';

  const goToCollection = (collection: Collection) => {
    selectedCollectionId.initialize(collection.id);
    pageStore.goTo('CollectionDetailsPage');
  };

  const goToCreateCollection = () => {
    selectedCollectionId.initialize('');
    pageStore.goTo('CollectionCreationPage');
  };
</script>

<aside class="side">
  <div class="brandmark">
    <div class="dot"><Library /></div>
    <h1>Alexandria</h1>
  </div>

  <button type="button" class="newbtn" onclick={() => goToCreateCollection()}>
    <Plus />
    <span>Nouvelle collection</span>
  </button>

  <p class="side-h">Collections</p>
  <ul class="collections">
    {#each $collectionsStore as collection}
      {@const active = $selectedCollectionId === collection.id}
      <li>
        <button
          type="button"
          class="coll"
          aria-current={active ? 'true' : 'false'}
          onclick={() => goToCollection(collection)}
        >
          <Folder class="ci" />
          <span class="cname">{collection.title}</span>
          <span class="count">{collection.videos.length}</span>
        </button>
      </li>
    {/each}
  </ul>

  <div class="side-foot">
    <span class="who">Thème</span>
    <ThemeToggle />
  </div>
</aside>
