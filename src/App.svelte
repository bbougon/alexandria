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
  import HomePage from './lib/HomePage.svelte';

  collectionsStore.initialize(allCollections);
  pageStore.initialize('CollectionCreationPage');

  const PAGES: Map<Page, Component> = new Map<Page, Component>([
    ['CollectionCreationPage', CollectionCreationPage],
    ['CollectionDetailsPage', CollectionDetailsPage],
    ['SearchResultPage', SearchResultPage],
    ['HomePage', HomePage],
  ]);
</script>

<div class="flex h-screen bg-surface">
  <Sidebar />

  <div class="flex min-w-0 flex-1 flex-col">
    <header
      class="sticky top-0 z-10 flex items-center border-b border-border bg-surface/85 px-5 py-3.5 backdrop-blur"
    >
      <Searchbar />
    </header>
    <main class="flex-1 overflow-y-auto p-6">
      <svelte:component this={PAGES.get($pageStore)} />
    </main>
  </div>
</div>

<style>
</style>
