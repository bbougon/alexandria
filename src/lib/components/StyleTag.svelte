<script lang="ts">
  import { X } from '@lucide/svelte';
  import { styleColor } from './styleColor';

  interface Props {
    /** Nom du style musical ; détermine la couleur du chip. */
    name: string;
    /** Affiche la croix de suppression. */
    removable?: boolean;
    onremove?: () => void;
  }

  let { name, removable = false, onremove }: Props = $props();

  let color = $derived(styleColor(name));
</script>

<span
  class="st"
  style="background:var(--tag-{color}-bg);color:var(--tag-{color}-fg)"
>
  {name}
  {#if removable}
    <button
      type="button"
      class="chipx"
      aria-label={`Retirer le style ${name}`}
      onclick={onremove}
    >
      <X />
    </button>
  {/if}
</span>
