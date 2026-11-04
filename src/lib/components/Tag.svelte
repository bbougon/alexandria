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
    'inline-flex items-center gap-1 whitespace-nowrap rounded-full bg-surface-sunken px-2.5 py-0.5 text-xs font-medium text-ink',
    className
  )}
>
  {label}
  {#if removable}
    <button
      type="button"
      aria-label={`Retirer le tag ${label}`}
      onclick={onremove}
      class="-mr-1 rounded-full p-0.5 hover:bg-black/10 dark:hover:bg-white/10"
    >
      <X class="size-3" />
    </button>
  {/if}
</span>
