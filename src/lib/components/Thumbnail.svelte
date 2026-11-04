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
    'relative grid place-items-center overflow-hidden rounded-lg bg-gradient-to-br to-surface-sunken',
    well,
    className
  )}
>
  {#if thumbnail}
    <video poster={thumbnail} class="absolute inset-0 h-full w-full object-cover"
    ></video>
  {/if}

  {#if onplay}
    <button
      type="button"
      aria-label="Lire la vidéo"
      onclick={onplay}
      class="relative z-10 grid size-12 place-items-center rounded-full bg-brand text-on-brand shadow-lg outline-none transition-transform hover:scale-105 hover:bg-brand-strong focus-visible:ring-2 focus-visible:ring-on-media"
    >
      <Play class="size-[22px] translate-x-0.5 fill-current" />
    </button>
  {/if}

  {#if duration}
    <div
      class="absolute bottom-2 right-2 z-10 flex items-center gap-1 rounded-sm bg-overlay px-[7px] py-0.5 font-mono text-[11px] text-on-media"
    >
      <Clock class="size-3" />
      {duration}
    </div>
  {/if}
</div>
