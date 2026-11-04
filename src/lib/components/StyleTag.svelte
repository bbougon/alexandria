<script lang="ts">
  import { X } from '@lucide/svelte';
  import { cn } from './classCombinator';
  import { styleColor, type StyleColor } from './styleColor';

  interface Props {
    /** Nom du style musical ; détermine la couleur du chip. */
    name: string;
    /** Affiche la croix de suppression. */
    removable?: boolean;
    onremove?: () => void;
    class?: string;
  }

  let { name, removable = false, onremove, class: className }: Props = $props();

  // Classes statiques pour que Tailwind les conserve au build.
  const CHIP: Record<StyleColor, string> = {
    rose: 'bg-tag-rose-bg text-tag-rose-fg',
    amber: 'bg-tag-amber-bg text-tag-amber-fg',
    lime: 'bg-tag-lime-bg text-tag-lime-fg',
    teal: 'bg-tag-teal-bg text-tag-teal-fg',
    sky: 'bg-tag-sky-bg text-tag-sky-fg',
    violet: 'bg-tag-violet-bg text-tag-violet-fg',
  };

  let color = $derived(styleColor(name));
</script>

<span
  class={cn(
    'inline-flex items-center gap-1 whitespace-nowrap rounded-full px-2.5 py-0.5 text-xs font-medium',
    CHIP[color],
    className
  )}
>
  {name}
  {#if removable}
    <button
      type="button"
      aria-label={`Retirer le style ${name}`}
      onclick={onremove}
      class="-mr-1 rounded-full p-0.5 hover:bg-black/10 dark:hover:bg-white/10"
    >
      <X class="size-3" />
    </button>
  {/if}
</span>
