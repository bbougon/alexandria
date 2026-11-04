<script lang="ts">
  import { collectionsStore } from '../../collections/collections.store';
  import { selectedCollectionId } from '../../collections/collection.store';
  import type { Collection } from '../../collections/collection';
  import { pageStore } from '../pages/pageStore';
  import { Folder, Plus, Library } from '@lucide/svelte';
  import Button from '../../components/Button.svelte';
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
  class="flex h-full w-[270px] flex-col gap-5 overflow-y-auto border-r border-border bg-surface-sunken px-4 py-6"
>
  <div class="flex items-center gap-2.5 px-2">
    <div
      class="grid size-[26px] place-items-center rounded-lg bg-brand text-on-brand"
    >
      <Library class="size-4" />
    </div>
    <h1 class="font-display text-xl font-bold tracking-tight">Alexandria</h1>
  </div>

  <Button class="w-full" onclick={() => goToCreateCollection()}>
    <Plus class="mr-2 h-4 w-4" />
    New Collection
  </Button>

  <div class="flex min-h-0 flex-1 flex-col gap-1">
    <p
      class="px-2 text-[11px] font-semibold uppercase tracking-wider text-ink-muted"
    >
      Collections
    </p>
    <ul class="flex flex-col gap-0.5">
      {#each $collectionsStore as collection}
        {@const active = $selectedCollectionId === collection.id}
        <li>
          <button
            type="button"
            aria-current={active ? 'true' : undefined}
            onclick={() => goToCollection(collection)}
            class={`relative flex w-full items-center gap-2.5 rounded-md px-2.5 py-2 text-left text-[14.5px] transition-colors ${
              active
                ? 'bg-brand-tint font-semibold before:absolute before:bottom-2 before:left-0 before:top-2 before:w-[3px] before:rounded before:bg-brand before:content-[""]'
                : 'font-medium hover:bg-brand-tint'
            }`}
          >
            <Folder
              class={`size-[18px] flex-none ${active ? 'text-brand-strong' : 'text-ink-muted'}`}
            />
            <span class="min-w-0 flex-1 truncate">{collection.title}</span>
            <span
              class="rounded-full border border-border bg-surface px-[7px] py-px font-mono text-[11px] text-ink-muted"
            >
              {collection.videos.length}
            </span>
          </button>
        </li>
      {/each}
    </ul>
  </div>

  <div
    class="mt-auto flex items-center justify-between border-t border-border px-2 pt-3"
  >
    <span class="text-[13px] text-ink-muted">Theme</span>
    <ThemeToggle />
  </div>
</aside>
