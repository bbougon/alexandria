<script lang="ts">
  import type { HTMLAttributes } from 'svelte/elements';
  import type { Snippet } from 'svelte';
  import { getPopoverContext } from './popover-context';
  import { cn } from '../classCombinator';

  interface Props extends HTMLAttributes<HTMLDivElement> {
    children: Snippet;
    align?: 'start' | 'center' | 'end';
    sideOffset?: number;
  }

  let {
    children,
    class: className,
    align = 'center',
    sideOffset = 4,
    ...props
  }: Props = $props();

  const popoverState = getPopoverContext();

  let popoverElement: HTMLElement | undefined = $state();
  let coords = $state({ top: 0, left: 0 });

  function portal(node: HTMLElement) {
    document.body.appendChild(node);
    return {
      destroy() {
        if (node.parentNode) {
          node.parentNode.removeChild(node);
        }
      },
    };
  }

  $effect(() => {
    if (popoverState.open && popoverState.anchor && popoverElement) {
      const updatePosition = () => {
        const anchorRect = popoverState.anchor!.getBoundingClientRect();
        let top = anchorRect.bottom + sideOffset + window.scrollY;
        let left = anchorRect.left + window.scrollX;

        if (align === 'center') {
          left =
            anchorRect.left +
            anchorRect.width / 2 -
            popoverElement!.offsetWidth / 2 +
            window.scrollX;
        } else if (align === 'end') {
          left = anchorRect.right - popoverElement!.offsetWidth + window.scrollX;
        }

        // Basic viewport boundary check
        if (left < 0) left = 0;
        if (
          left + popoverElement!.offsetWidth >
          window.innerWidth + window.scrollX
        ) {
          left = window.innerWidth + window.scrollX - popoverElement!.offsetWidth;
        }
        if (
          top + popoverElement!.offsetHeight >
          window.innerHeight + window.scrollY
        ) {
          top =
            anchorRect.top +
            window.scrollY -
            popoverElement!.offsetHeight -
            sideOffset;
        }

        coords = { top, left };
      };

      const handleOutsideClick = (event: MouseEvent) => {
        if (
          popoverElement &&
          !popoverElement.contains(event.target as Node) &&
          popoverState.anchor &&
          !popoverState.anchor.contains(event.target as Node)
        ) {
          popoverState.open = false;
        }
      };

      updatePosition();
      window.addEventListener('scroll', updatePosition);
      window.addEventListener('resize', updatePosition);
      window.addEventListener('mousedown', handleOutsideClick);

      return () => {
        window.removeEventListener('scroll', updatePosition);
        window.removeEventListener('resize', updatePosition);
        window.removeEventListener('mousedown', handleOutsideClick);
      };
    }
  });
</script>

{#if popoverState.open}
  <div
    use:portal
    bind:this={popoverElement}
    id={popoverState.id}
    data-slot="popover-content"
    data-state={popoverState.open ? 'open' : 'closed'}
    data-align={align}
    data-side-offset={sideOffset}
    class={cn(
      'bg-popover text-popover-foreground data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 z-50 w-72 origin-(--radix-popover-content-transform-origin) rounded-md border p-4 shadow-md outline-hidden',
      'absolute',
      className
    )}
    style:top="{coords.top}px"
    style:left="{coords.left}px"
    style:--radix-popover-content-transform-origin="var(--radix-popover-content-transform-origin)"
    {...props}
  >
    {@render children?.()}
  </div>
{/if}
