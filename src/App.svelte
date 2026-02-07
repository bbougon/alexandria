<script lang="ts">
  import Searchbar from './lib/kit/layout/Searchbar.svelte';
  import Sidebar from './lib/kit/layout/Sidebar.svelte';
  import { collectionsStore } from './lib/collections/collections.store';
  import { allCollections } from './lib/collections/collections.tauri';
  import CollectionCreationPage from './lib/collections/CollectionCreationPage.svelte';
  import { type Page, pageStore } from './lib/kit/pages/pageStore';
  import CollectionDetailsPage from './lib/collections/CollectionDetailsPage.svelte';
  import type { Component } from 'svelte';
  import SearchResultPage from './lib/search/SearchResultPage.svelte';

  collectionsStore.initialize(allCollections);
  pageStore.initialize('CollectionCreationPage');

  const PAGES: Map<Page, Component> = new Map<Page, Component>([
    ['CollectionCreationPage', CollectionCreationPage],
    ['CollectionDetailsPage', CollectionDetailsPage],
    ['SearchResultPage', SearchResultPage],
  ]);
</script>

<Sidebar />

<div class="lg:pl-72">
  <Searchbar />
  <main class="py-10">
    <svelte:component this={PAGES.get($pageStore)} />
  </main>
</div>

<style>
</style>
