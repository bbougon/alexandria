<script lang="ts">
  import { Play, Clock } from '@lucide/svelte';
  import { cn } from './classCombinator';
  import { styleColor, type StyleColor } from './styleColor';

  interface Props {
    thumbnail?: string;
    /** Durée déjà formatée (ex. « 12:04 »). */
    duration?: string;
    /** Style musical : teinte le puits de la vignette de façon déterministe. */
    styleName?: string;
    onplay?: () => void;
    class?: string;
  }

  let { thumbnail, duration, styleName, onplay, class: className }: Props = $props();

  // Classes statiques pour que Tailwind conserve chaque dégradé au build.
  const WELL: Record<StyleColor, string> = {
    rose: 'from-tag-rose-bg',
    amber: 'from-tag-amber-bg',
    lime: 'from-tag-lime-bg',
    teal: 'from-tag-teal-bg',
    sky: 'from-tag-sky-bg',
    violet: 'from-tag-violet-bg',
  };

  let well = $derived(styleName ? WELL[styleColor(styleName)] : 'from-brand-tint');
</script>

<div
  class={cn(
    'group relative overflow-hidden rounded-lg bg-gradient-to-br to-surface-sunken',
    well,
    className
  )}
>
  {#if thumbnail}
    <video poster={thumbnail} class="h-full w-full object-cover"></video>
  {/if}

  {#if onplay}
    <button
      type="button"
      aria-label="Lire la vidéo"
      onclick={onplay}
      class="absolute inset-0 flex items-center justify-center bg-overlay opacity-0 outline-none transition-opacity group-hover:opacity-100 focus-visible:opacity-100"
    >
      <span
        class="flex size-11 items-center justify-center rounded-full bg-brand text-on-brand shadow-md"
      >
        <Play class="size-5 fill-current" />
      </span>
    </button>
  {/if}

  {#if duration}
    <div
      class="absolute bottom-2 right-2 flex items-center gap-1 rounded-md bg-overlay px-2 py-1 font-mono text-xs text-on-media"
    >
      <Clock class="size-3" />
      {duration}
    </div>
  {/if}
</div>
