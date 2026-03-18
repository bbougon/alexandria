<script lang="ts">
  import type { HTMLAttributes } from 'svelte/elements';
  import type { Snippet } from 'svelte';
  import { cn } from '../classCombinator';

  interface Props extends HTMLAttributes<HTMLDivElement> {
    children: Snippet;
  }

  let { children, class: classe, ...props }: Props = $props();
</script>

<div
  class={cn('flex flex-col-reverse gap-2 sm:flex-row sm:justify-end', classe)}
  {...props}
>
  {@render children?.()}
</div>
