<script lang="ts">
  import type { HTMLInputAttributes } from 'svelte/elements';
  import type { Snippet } from 'svelte';

  interface Props extends HTMLInputAttributes {
    label: string;
    toggle: boolean;
    hideOnToggle?: Snippet;
    displayChild?: Snippet;
    onAddTag: (tag: string) => void;
  }

  const availableKeysToPushTag = ['Enter', 'Tab', ',', ';', ' ', 'Semicolon'];

  let { label, toggle, hideOnToggle, displayChild, onAddTag }: Props = $props();
  const addTag = (e: KeyboardEvent) => {
    if (availableKeysToPushTag.includes(e.key)) {
      e.preventDefault();
      const inputTarget = e.currentTarget as HTMLInputElement;
      onAddTag(inputTarget.value);
      inputTarget.value = '';
    }
  };
</script>

{#if !toggle}
  {@render hideOnToggle?.()}
  {@render displayChild?.()}
{:else}
  <div class="relative">
    <label
      for={label}
      class="absolute -top-2 left-2 inline-block rounded-lg bg-white px-1 text-xs font-medium text-gray-900 dark:bg-gray-900 dark:text-white"
      >Name</label
    >
    <input
      id={label}
      type="text"
      name={label}
      onkeydown={(e) => addTag(e)}
      class="block w-full rounded-md bg-white px-3 py-1.5 text-base text-gray-900 outline-1 -outline-offset-1 outline-gray-300 placeholder:text-gray-400 focus:outline-2 focus:-outline-offset-2 focus:outline-indigo-600 sm:text-sm/6 dark:bg-gray-900 dark:text-white dark:outline-gray-600 dark:placeholder:text-gray-500 dark:focus:outline-indigo-500"
    />
  </div>
  {@render displayChild?.()}
{/if}
