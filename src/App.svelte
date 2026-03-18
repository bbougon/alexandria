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
  import { Library } from '@lucide/svelte';
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

<div class="h-screen flex flex-col bg-gray-50">
  <header
    class="bg-white border-b border-gray-200 px-6 py-4 flex items-center gap-6 flex-shrink-0"
  >
    <div class="flex items-center gap-3">
      <div class="p-2 bg-blue-500 rounded-lg">
        <Library class="w-6 h-6 text-white" />
      </div>
      <h1 class="text-xl font-semibold">Video Library</h1>
    </div>
    <Searchbar />
  </header>

  <div class="flex-1 flex overflow-hidden">
    <Sidebar />
    <main class="flex-1 overflow-y-auto p-8">
      <svelte:component this={PAGES.get($pageStore)} />
    </main>
  </div>
</div>

<style>
</style>
