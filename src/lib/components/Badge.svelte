<script lang="ts">
  import { cn } from './classCombinator';
  import type { HTMLAttributes } from 'svelte/elements';
  import { cva } from 'class-variance-authority';

  type Variant = {
    default: string;
    destructive: string;
    outline: string;
    secondary: string;
  };

  interface Props extends HTMLAttributes<HTMLSpanElement> {
    variant?: keyof Variant;
  }

  let {
    variant = 'default',
    class: className,
    children,
    ...props
  }: Props = $props();

  const badgeVariants = cva(
    'inline-flex items-center justify-center rounded-md border px-2 py-0.5 text-xs font-medium w-fit whitespace-nowrap shrink-0 [&>svg]:size-3 gap-1 [&>svg]:pointer-events-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive transition-[color,box-shadow] overflow-hidden',
    {
      variants: {
        variant: {
          default:
            'border-transparent bg-primary text-primary-foreground [a&]:hover:bg-primary/90',
          secondary:
            'border-transparent bg-secondary text-secondary-foreground [a&]:hover:bg-secondary/90',
          destructive:
            'border-transparent bg-destructive text-white [a&]:hover:bg-destructive/90 focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40 dark:bg-destructive/60',
          outline:
            'text-foreground [a&]:hover:bg-accent [a&]:hover:text-accent-foreground',
        },
      },
      defaultVariants: {
        variant: 'default',
      },
    }
  );

  type ColourName = 'purple' | 'yellow' | 'green' | 'blue' | 'indigo' | 'pink';
  type ColourClasses = string[];
  type ColourEntry = Record<ColourName, ColourClasses>;

  const colours: ColourEntry = {
    purple: [
      'bg-purple-100',
      'text-purple-700',
      'dark:bg-purple-400/10',
      'dark:text-purple-400',
    ],
    yellow: [
      'bg-yellow-100',
      'text-yellow-800',
      'dark:bg-yellow-400/10',
      'dark:text-yellow-500',
    ],
    green: [
      'bg-green-100',
      'text-green-700',
      'dark:bg-green-400/10',
      'dark:text-green-400',
    ],
    blue: [
      'bg-blue-100',
      'text-blue-700',
      'dark:bg-blue-400/10',
      'dark:text-blue-400',
    ],
    indigo: [
      'bg-indigo-100',
      'text-indigo-700',
      'dark:bg-indigo-400/10',
      'dark:text-indigo-400',
    ],
    pink: [
      'bg-pink-100',
      'text-pink-700',
      'dark:bg-pink-400/10',
      'dark:text-pink-400',
    ],
  };

  const pickRandomColour = (colours: ColourEntry) => {
    const allColours = Object.values(colours);
    return allColours[Math.floor(Math.random() * allColours.length)];
  };

  const activeColour = pickRandomColour(colours);
</script>

<span
  data-slot="badge"
  class={cn(badgeVariants({ variant, className }), activeColour)}
  {...props}
>
  {@render children?.()}
</span>
