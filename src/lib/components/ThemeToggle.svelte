<script lang="ts">
  import { onMount } from 'svelte';
  import { Sun, Moon } from '@lucide/svelte';

  let dark = $state(false);

  const apply = (value: boolean) => {
    document.documentElement.classList.toggle('dark', value);
  };

  onMount(() => {
    let stored: string | null = null;
    try {
      stored = localStorage.getItem('theme');
    } catch {
      stored = null;
    }
    dark = stored
      ? stored === 'dark'
      : window.matchMedia?.('(prefers-color-scheme: dark)').matches;
    apply(dark);
  });

  const toggle = () => {
    dark = !dark;
    apply(dark);
    try {
      localStorage.setItem('theme', dark ? 'dark' : 'light');
    } catch {
      // stockage indisponible : on garde juste l'état en mémoire
    }
  };
</script>

<button
  type="button"
  aria-label={dark ? 'Passer en thème clair' : 'Passer en thème sombre'}
  onclick={toggle}
  class="grid size-[34px] place-items-center rounded-md border border-border-strong bg-surface-raised text-ink-muted transition-colors hover:border-brand hover:text-ink"
>
  {#if dark}
    <Sun class="size-4" />
  {:else}
    <Moon class="size-4" />
  {/if}
</button>
