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
    class="impthumb"
    style="background:{background}"
    aria-label="Lire l'aperçu"
    onclick={onplay}
  >
    {#if thumbnail}
      <video poster={thumbnail}></video>
    {:else}
      <Film />
    {/if}
  </button>
{:else}
  <div class="thumb" style="background:{background}">
    {#if thumbnail}
      <video poster={thumbnail}></video>
    {/if}

    {#if onplay}
      <button type="button" class="play" aria-label="Lire la vidéo" onclick={onplay}>
        <Play fill="currentColor" />
      </button>
    {/if}

    {#if duration}
      <span class="dur">
        <Clock />
        {duration}
      </span>
    {/if}
  </div>
{/if}
