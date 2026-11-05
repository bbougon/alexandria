<script lang="ts">
  import { X } from '@lucide/svelte';

  interface Props {
    /** Libellé du tag libre. */
    label: string;
    /** Affiche la croix de suppression. */
    removable?: boolean;
    onremove?: () => void;
  }

  let { label, removable = false, onremove }: Props = $props();
</script>

<span
  class="inline-flex items-center rounded-sm border border-border bg-surface-sunken px-[9px] py-0.5 text-[12.5px] text-ink"
>
  {label}
  {#if removable}
    <button
      type="button"
      class="-mr-[3px] ml-0.5 inline-grid h-[15px] w-[15px] place-items-center rounded-pill p-0 opacity-65 hover:bg-danger-tint hover:text-danger hover:opacity-100"
      aria-label={`Retirer le tag ${label}`}
      onclick={onremove}
    >
      <X class="h-[10px] w-[10px]" />
    </button>
  {/if}
</span>
