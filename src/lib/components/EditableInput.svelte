<script lang="ts">
  import type { HTMLInputAttributes } from 'svelte/elements';
  import { Pencil } from '@lucide/svelte';
  import Input from './Input.svelte';

  interface Props extends HTMLInputAttributes {
    label: string;
    value: string;
  }

  let { value = $bindable(), label, onblur, ...props }: Props = $props();

  let isEditing = $state(false);
  let isHovered = $state(false);

  const update = (
    event: FocusEvent & {
      currentTarget: EventTarget & HTMLInputElement;
    }
  ) => {
    if (onblur) onblur(event);
    isEditing = false;
  };
</script>

{#snippet displayInput()}
  <Input
    bind:value
    onblur={(event) => update(event)}
    {...props}
    placeholder={label}
  />
{/snippet}

{#snippet displayValue()}
  <div
    class="group relative cursor-pointer transition-all"
    onclick={() => (isEditing = true)}
    onmouseenter={() => (isHovered = true)}
    onmouseleave={() => (isHovered = false)}
  >
    <span class={value ? '' : 'text-muted-foreground'}>
      {value || label}
    </span>
    {#if isHovered}
      <Pencil class="inline-block ml-2 w-3 h-3 text-muted-foreground opacity-50" />
    {/if}
  </div>
{/snippet}

{#if isEditing}
  {@render displayInput()}
{:else}
  {@render displayValue()}
{/if}
