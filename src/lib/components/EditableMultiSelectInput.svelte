<script lang="ts">
  import type { HTMLInputAttributes } from 'svelte/elements';
  import type { Snippet } from 'svelte';
  import MultiSelectCheckbox from './MultiSelectCheckbox.svelte';

  interface Props<T = unknown> extends HTMLInputAttributes {
    label: string;
    value: T[];
    options: T[];
    toggle: boolean;
    hideOnToggle: Snippet;
    displayChild?: Snippet;
  }

  let {
    value = $bindable(),
    label,
    toggle,
    hideOnToggle,
    displayChild,
    options,
  }: Props = $props();
  let visible = $state(false);

  const addOption = (option: (typeof options)[number]) => {
    console.log(option);
    if (!value.includes(option)) value.push(option);
    else value.splice(value.indexOf(option), 1);
  };
</script>

{#if !toggle}
  {@render hideOnToggle?.()}
  {@render displayChild?.()}
{:else}
  {#snippet dropDown()}
    <div
      id="menu"
      class={`${visible ? 'block' : 'hidden'} absolute z-10 mt-2 w-full rounded-lg border bg-white  outline-1 -outline-offset-1 outline-gray-300 placeholder:text-gray-400 focus:outline-2 focus:-outline-offset-2 focus:outline-indigo-600 sm:text-sm/6 dark:bg-gray-900 dark:text-white dark:outline-gray-600 dark:placeholder:text-gray-500 dark:focus:outline-indigo-500`}
    >
      {#each options as option}
        {@const checked = value.includes(option)}
        <MultiSelectCheckbox {checked} {option} onclick={() => addOption(option)} />
      {/each}
    </div>
  {/snippet}
  <div class="relative w-64">
    <label
      for="multi-select"
      class="absolute -top-2 left-2 inline-block rounded-lg bg-white px-1 text-xs font-medium text-gray-900 dark:bg-gray-900 dark:text-white"
      >{label}</label
    >
    <button
      onclick={() => (visible = !visible)}
      class="block w-full rounded-md bg-white px-3 py-1.5 text-base text-gray-900 outline-1 -outline-offset-1 outline-gray-300 placeholder:text-gray-400 focus:outline-2 focus:-outline-offset-2 focus:outline-indigo-600 sm:text-sm/6 dark:bg-gray-900 dark:text-white dark:outline-gray-600 dark:placeholder:text-gray-500 dark:focus:outline-indigo-500"
    >
      Choose options
    </button>
    {@render dropDown()}
  </div>

  {@render displayChild?.()}
{/if}
