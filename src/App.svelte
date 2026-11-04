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

<div class="flex h-screen flex-col bg-surface">
  <header
    class="flex flex-shrink-0 items-center gap-6 border-b border-border bg-surface-raised px-6 py-4"
  >
    <div class="flex items-center gap-3">
      <div class="rounded-lg bg-brand p-2">
        <Library class="h-6 w-6 text-on-brand" />
      </div>
      <h1 class="font-display text-xl font-semibold">Alexandria</h1>
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
