<script lang="ts" module>
  export { default as Popover } from './PopOver.svelte';
  export { default as PopoverTrigger } from './PopOverTrigger.svelte';
  export { default as PopoverContent } from './PopOverContent.svelte';
</script>

<script lang="ts">
  import type { Snippet } from 'svelte';
  import { setPopoverContext } from './popover-context';

  interface Props {
    open?: boolean;
    onOpenChange?: (open: boolean) => void;
    children: Snippet;
  }

  let { open = $bindable(false), onOpenChange, children }: Props = $props();

  const id = crypto.randomUUID();
  let anchor = $state<HTMLElement | null>(null);

  setPopoverContext({
    get open() {
      return open;
    },
    set open(value: boolean) {
      open = value;
    },
    toggle: () => {
      open = !open;
      onOpenChange?.(open);
    },
    id,
    get anchor() {
      return anchor;
    },
    set anchor(value: HTMLElement | null) {
      anchor = value;
    },
  });
</script>

{@render children?.()}
