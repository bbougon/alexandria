<script lang="ts">
  import type { HTMLAttributes } from 'svelte/elements';
  import type { Snippet } from 'svelte';
  import { cn } from '../classCombinator';

  interface Props extends HTMLAttributes<HTMLDivElement> {
    children: Snippet;
  }

  let { children, class: classe, ...props }: Props = $props();
</script>

<div class={cn('flex flex-col gap-2 text-center sm:text-left', classe)} {...props}>
  {@render children?.()}
</div>
