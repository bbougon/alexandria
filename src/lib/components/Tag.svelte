<script lang="ts">
  import { X } from '@lucide/svelte';
  import { cn } from './classCombinator';

  interface Props {
    /** Libellé du tag libre. */
    label: string;
    /** Affiche la croix de suppression. */
    removable?: boolean;
    onremove?: () => void;
    class?: string;
  }

  let { label, removable = false, onremove, class: className }: Props = $props();
</script>

<span
  class={cn(
    'inline-flex items-center whitespace-nowrap rounded-sm border border-border bg-surface-sunken px-2.5 py-[3px] text-[12.5px] text-ink',
    className
  )}
>
  {label}
  {#if removable}
    <button
      type="button"
      aria-label={`Retirer le tag ${label}`}
      onclick={onremove}
      class="-mr-1 ml-1 grid size-[15px] place-items-center rounded-full opacity-70 hover:bg-danger-tint hover:text-danger hover:opacity-100"
    >
      <X class="size-2.5" />
    </button>
  {/if}
</span>
