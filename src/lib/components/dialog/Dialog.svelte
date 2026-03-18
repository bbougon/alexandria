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
