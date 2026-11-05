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
  import DesignSystemPage from './lib/designsystem/DesignSystemPage.svelte';

  collectionsStore.initialize(allCollections);
  pageStore.initialize('CollectionCreationPage');

  const PAGES: Map<Page, Component> = new Map<Page, Component>([
    ['CollectionCreationPage', CollectionCreationPage],
    ['CollectionDetailsPage', CollectionDetailsPage],
    ['SearchResultPage', SearchResultPage],
    ['HomePage', HomePage],
    ['DesignSystemPage', DesignSystemPage],
  ]);
</script>

<div
  class="grid h-screen grid-cols-[270px_1fr] max-[920px]:grid-cols-[220px_1fr] max-[760px]:grid-cols-1 max-[760px]:grid-rows-[auto_1fr]"
>
  <Sidebar />

  <main class="flex min-h-0 min-w-0 flex-col overflow-y-auto">
    <div
      class="sticky top-0 z-[5] flex items-center gap-[14px] border-b border-border bg-[color-mix(in_srgb,var(--surface)_86%,transparent)] pb-[14px] pl-[max(20px,env(safe-area-inset-left,0px))] pr-[max(16px,env(safe-area-inset-right,0px))] pt-[18px] backdrop-blur-[8px]"
    >
      <Searchbar />
    </div>
    <svelte:component this={PAGES.get($pageStore)} />
  </main>
</div>
