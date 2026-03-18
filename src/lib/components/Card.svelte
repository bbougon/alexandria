<script lang="ts">
  import { cn } from './classCombinator';

  let { children, class: className, ...props } = $props();
</script>

<div
  class={cn(
    'bg-card text-card-foreground flex flex-col gap-6 rounded-xl border',
    className
  )}
  {...props}
>
  {@render children?.()}
</div>
