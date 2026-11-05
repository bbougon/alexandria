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

<span class="tg">
  {label}
  {#if removable}
    <button
      type="button"
      class="chipx"
      aria-label={`Retirer le tag ${label}`}
      onclick={onremove}
    >
      <X />
    </button>
  {/if}
</span>
