<script lang="ts">
  import type { HTMLButtonAttributes } from 'svelte/elements';
  import { cva } from 'class-variance-authority';
  import { cn } from './classCombinator';

  type Variant = {
    default: string;
    destructive: string;
    outline: string;
    secondary: string;
    ghost: string;
    link: string;
  };
  type Size = {
    default: string;
    sm: string;
    lg: string;
    icon: string;
  };

  const buttonVariants = cva(
    "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-all disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 shrink-0 [&_svg]:shrink-0 outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive",
    {
      variants: {
        variant: {
          default: 'bg-primary text-primary-foreground hover:bg-brand-strong',
          destructive:
            'bg-destructive text-on-danger hover:bg-destructive/90 focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40',
          outline:
            'border border-border-strong bg-background text-foreground hover:bg-accent hover:text-accent-foreground',
          secondary: 'bg-secondary text-secondary-foreground hover:bg-secondary/80',
          ghost:
            'hover:bg-accent hover:text-accent-foreground dark:hover:bg-accent/50',
          link: 'text-primary underline-offset-4 hover:underline',
        },
        size: {
          default: 'h-9 px-4 py-2 has-[>svg]:px-3',
          sm: 'h-8 rounded-md gap-1.5 px-3 has-[>svg]:px-2.5',
          lg: 'h-10 rounded-md px-6 has-[>svg]:px-4',
          icon: 'size-9 rounded-md',
        },
      },
      defaultVariants: {
        variant: 'default',
        size: 'default',
      },
    }
  );

  interface Props extends HTMLButtonAttributes {
    variant?: keyof Variant;
    size?: keyof Size;
    enabled?: boolean;
  }

  let {
    variant = 'default',
    size = 'default',
    enabled,
    class: className,
    children,
    ...props
  }: Props = $props();
</script>

<button
  {...props}
  class={cn(buttonVariants({ variant, size, className }))}
  disabled={enabled === false ? true : props.disabled}
>
  {@render children?.()}
</button>
