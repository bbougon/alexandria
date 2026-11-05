<script lang="ts">
  import { Play, Clock, Film } from '@lucide/svelte';
  import { styleColor } from './styleColor';

  interface Props {
    thumbnail?: string;
    /** Durée déjà formatée (ex. « 12:04 »). */
    duration?: string;
    /** Style musical : teinte le puits de la vignette de façon déterministe. */
    styleName?: string;
    onplay?: () => void;
    /** `card` = vignette de carte (176×112) ; `import` = miniature de ligne d'import. */
    variant?: 'card' | 'import';
  }

  let { thumbnail, duration, styleName, onplay, variant = 'card' }: Props = $props();

  let color = $derived(styleName ? styleColor(styleName) : null);
  // Puits dégradé repris de la maquette (teinté par le 1er style, sinon brand).
  let background = $derived(
    color
      ? `radial-gradient(120% 120% at 20% 0%, color-mix(in srgb, var(--tag-${color}-fg) 32%, var(--surface-sunken)) 0%, var(--surface-sunken) 62%)`
      : `radial-gradient(120% 120% at 20% 0%, color-mix(in srgb, var(--brand) 22%, var(--surface-sunken)) 0%, var(--surface-sunken) 62%)`
  );
</script>

{#if variant === 'import'}
  <button
    type="button"
    class="relative grid h-[34px] w-[52px] shrink-0 grow-0 place-items-center overflow-hidden rounded-sm text-on-brand"
    style="background:{background}"
    aria-label="Lire l'aperçu"
    onclick={onplay}
  >
    {#if thumbnail}
      <video class="absolute inset-0 h-full w-full object-cover" poster={thumbnail}
      ></video>
    {:else}
      <Film class="h-4 w-4 opacity-85" />
    {/if}
  </button>
{:else}
  <div
    class="relative grid h-28 w-44 shrink-0 grow-0 place-items-center overflow-hidden rounded-lg max-[440px]:h-[150px] max-[440px]:w-full"
    style="background:{background}"
  >
    {#if thumbnail}
      <video class="absolute inset-0 h-full w-full object-cover" poster={thumbnail}
      ></video>
    {/if}

    {#if onplay}
      <button
        type="button"
        class="relative z-[1] grid h-12 w-12 cursor-pointer place-items-center rounded-pill bg-brand text-on-brand shadow-[0_6px_18px_rgba(54,38,12,0.28)] transition hover:scale-[1.06] hover:bg-brand-strong"
        aria-label="Lire la vidéo"
        onclick={onplay}
      >
        <Play fill="currentColor" class="ml-0.5 h-[22px] w-[22px]" />
      </button>
    {/if}

    {#if duration}
      <span
        class="absolute bottom-[7px] right-[7px] z-[1] inline-flex items-center gap-1 rounded-sm bg-overlay px-[7px] py-0.5 font-mono text-[11px] font-medium text-on-media"
      >
        <Clock class="h-[11px] w-[11px]" />
        {duration}
      </span>
    {/if}
  </div>
{/if}
