<script lang="ts">
  import Searchbar from './lib/kit/layout/Searchbar.svelte';
  import Sidebar from './lib/kit/layout/Sidebar.svelte';
  import { collectionsStore } from './lib/collections/collections.store';
  import { allCollections } from './lib/collections/collections.tauri';
  import CollectionCreationPage from './lib/collections/CollectionCreationPage.svelte';

  collectionsStore.initialize(allCollections);
</script>

<Sidebar />

<div class="lg:pl-72">
  <Searchbar />
  <main class="py-10">
    <CollectionCreationPage />
  </main>
</div>

<style>
</style>
