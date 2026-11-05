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
  class="inline-flex items-center gap-1.5 whitespace-nowrap rounded-sm px-2.5 py-[3px] text-[12.5px] font-semibold"
  style="background:var(--tag-{color}-bg);color:var(--tag-{color}-fg)"
>
  <span class="h-[7px] w-[7px] rounded-full bg-current opacity-85" aria-hidden="true"
  ></span>
  {name}
  {#if removable}
    <button
      type="button"
      class="-mr-[3px] ml-0.5 inline-grid h-[15px] w-[15px] place-items-center rounded-pill p-0 opacity-65 hover:bg-black/[0.14] hover:opacity-100"
      aria-label={`Retirer le style ${name}`}
      onclick={onremove}
    >
      <X class="h-[10px] w-[10px]" />
    </button>
  {/if}
</span>
