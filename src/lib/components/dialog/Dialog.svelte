<script lang="ts">
  import type { Snippet } from 'svelte';

  let elementModale: HTMLDialogElement | undefined = $state();

  interface Props {
    children: Snippet;
  }

  let { children }: Props = $props();

  export const display = () => {
    elementModale?.showModal();
  };

  export const close = () => {
    elementModale?.close();
  };
</script>

<dialog
  bind:this={elementModale}
  class="bg-transparent border-none p-0 outline-none max-w-full max-h-full"
>
  {@render children?.()}
</dialog>

<style lang="scss">
  /* Le preflight de Tailwind v4 pose `margin: 0` sur `*`, ce qui écrase le
     `margin: auto` que le navigateur applique aux `<dialog>` modaux et les
     renvoie en haut à gauche. On le rétablit pour recentrer la modale. */
  dialog {
    margin: auto;
  }

  ::backdrop {
    animation: backdrop-fade var(--default-transition-duration) ease forwards;
  }

  @keyframes backdrop-fade {
    from {
      background: transparent;
    }
    to {
      background: color-mix(in oklab, black 50%, transparent);
    }
  }
</style>
