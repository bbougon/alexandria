<script lang="ts">
  import type { HTMLAttributes } from 'svelte/elements';
  import type { Snippet } from 'svelte';
  import { getPopoverContext } from './popover-context';

  interface Props extends HTMLAttributes<HTMLButtonElement> {
    children: Snippet;
  }

  let { children, ...props }: Props = $props();

  const popoverState = getPopoverContext();

  const handleToggle = (e: MouseEvent) => {
    if (!popoverState.anchor) {
      popoverState.anchor = e.currentTarget as HTMLElement;
    }
    popoverState.toggle();
    props.onclick?.(e);
  };
</script>

<button
  bind:this={popoverState.anchor}
  {...props}
  onclick={handleToggle}
  aria-expanded={popoverState.open}
  aria-haspopup="dialog"
  aria-controls={popoverState.id}
  data-state={popoverState.open ? 'open' : 'closed'}
  data-slot="popover-trigger"
>
  {@render children?.()}
</button>
