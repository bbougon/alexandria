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

<aside
  class="flex min-h-0 flex-col gap-5 overflow-y-auto border-r border-border bg-surface-sunken pb-[calc(20px_+_env(safe-area-inset-bottom,0px))] pl-[max(16px,env(safe-area-inset-left,0px))] pr-4 pt-6 max-[760px]:flex-row max-[760px]:items-center max-[760px]:gap-3 max-[760px]:overflow-x-auto max-[760px]:overflow-y-hidden max-[760px]:border-b max-[760px]:border-r-0 max-[760px]:px-[max(16px,env(safe-area-inset-left,0px))] max-[760px]:py-3"
>
  <div class="flex items-center gap-2.5 px-2 max-[760px]:shrink-0">
    <div
      class="grid h-[26px] w-[26px] shrink-0 place-items-center rounded-[8px] bg-brand text-on-brand"
    >
      <Library class="h-4 w-4" />
    </div>
    <h1
      class="font-display text-[21px] font-bold tracking-[-0.02em] max-[760px]:hidden"
    >
      Alexandria
    </h1>
  </div>

  <button
    type="button"
    class="flex w-full cursor-pointer items-center justify-center gap-2 rounded-md bg-brand px-[14px] py-[11px] text-sm font-semibold text-on-brand shadow-sm hover:bg-brand-strong max-[760px]:w-auto max-[760px]:shrink-0 max-[760px]:whitespace-nowrap"
    onclick={() => goToCreateCollection()}
  >
    <Plus class="h-4 w-4" />
    <span class="max-[760px]:hidden">Nouvelle collection</span>
  </button>

  <p
    class="mb-0.5 mt-1 px-2 text-[11px] font-semibold uppercase tracking-[0.08em] text-ink-muted max-[760px]:hidden"
  >
    Collections
  </p>
  <ul
    class="m-0 flex list-none flex-col gap-0.5 p-0 max-[760px]:flex-1 max-[760px]:flex-row"
  >
    {#each $collectionsStore as collection}
      {@const active = $selectedCollectionId === collection.id}
      <li>
        <button
          type="button"
          class="relative flex w-full cursor-pointer items-center gap-2.5 rounded-md px-2.5 py-[9px] text-left text-[14.5px] font-medium text-ink hover:bg-brand-tint max-[760px]:whitespace-nowrap {active
            ? 'bg-brand-tint font-semibold'
            : ''}"
          aria-current={active ? 'true' : 'false'}
          onclick={() => goToCollection(collection)}
        >
          {#if active}
            <span
              class="absolute bottom-2 left-0 top-2 w-[3px] rounded-[3px] bg-brand max-[760px]:hidden"
            ></span>
          {/if}
          <Folder
            class="h-[18px] w-[18px] shrink-0 {active
              ? 'text-brand-strong'
              : 'text-ink-muted'}"
          />
          <span class="min-w-0 flex-1 truncate">{collection.title}</span>
          <span
            class="rounded-pill border border-border bg-surface px-[7px] py-px font-mono text-[11px] text-ink-muted"
            >{collection.videos.length}</span
          >
        </button>
      </li>
    {/each}
  </ul>

  <div
    class="mt-auto flex items-center justify-between border-t border-border p-2 max-[760px]:hidden"
  >
    <span class="text-[13px] text-ink-muted">Thème</span>
    <ThemeToggle />
  </div>
</aside>
