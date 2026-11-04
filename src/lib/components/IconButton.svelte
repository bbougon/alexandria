<script lang="ts">
  import type { HTMLButtonAttributes } from 'svelte/elements';
  import type { Snippet } from 'svelte';
  import { cva } from 'class-variance-authority';
  import { cn } from './classCombinator';

  type Variant = {
    ghost: string;
    brand: string;
    danger: string;
  };
  type Size = {
    sm: string;
    md: string;
  };

  const iconButtonVariants = cva(
    "inline-flex items-center justify-center rounded-full transition-colors shrink-0 outline-none focus-visible:ring-focus focus-visible:ring-2 focus-visible:ring-offset-1 focus-visible:ring-offset-surface disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-[18px]",
    {
      variants: {
        variant: {
          ghost: 'text-ink-muted hover:bg-brand-tint hover:text-brand',
          brand: 'bg-brand text-on-brand hover:bg-brand-strong',
          danger: 'text-ink-muted hover:bg-danger-tint hover:text-danger',
        },
        size: {
          sm: 'size-7',
          md: 'size-9',
        },
      },
      defaultVariants: {
        variant: 'ghost',
        size: 'md',
      },
    }
  );

  interface Props extends HTMLButtonAttributes {
    variant?: keyof Variant;
    size?: keyof Size;
    /** Libellé accessible : obligatoire car le bouton n'a pas de texte visible. */
    label: string;
    children: Snippet;
  }

  let {
    variant = 'ghost',
    size = 'md',
    label,
    class: className,
    children,
    ...props
  }: Props = $props();
</script>

<button
  {...props}
  type={props.type ?? 'button'}
  class={cn(iconButtonVariants({ variant, size, className }))}
  aria-label={label}
  title={props.title ?? label}
>
  {@render children()}
</button>
