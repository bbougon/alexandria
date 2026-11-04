<script lang="ts">
  import type { HTMLInputAttributes } from 'svelte/elements';
  import { cn } from './classCombinator';

  interface Props extends HTMLInputAttributes {
    /** Libellé/placeholder du champ. */
    label: string;
    value: string;
  }

  let {
    value = $bindable(),
    label,
    class: className,
    onkeydown,
    ...props
  }: Props = $props();

  // Le champ reste toujours éditable ; l'affordance apparaît au survol et au
  // focus. Entrée valide la saisie en retirant le focus (déclenche onblur).
  const handleKeydown = (
    e: KeyboardEvent & { currentTarget: EventTarget & HTMLInputElement }
  ) => {
    onkeydown?.(e);
    if (e.key === 'Enter') {
      e.preventDefault();
      e.currentTarget.blur();
    }
  };
</script>

<input
  bind:value
  placeholder={label}
  aria-label={label}
  onkeydown={handleKeydown}
  class={cn(
    '-mx-2 w-full min-w-0 cursor-text rounded-md bg-transparent px-2 py-1 text-ink outline-none transition-colors',
    'placeholder:text-ink-subtle hover:bg-surface-sunken',
    'focus:bg-surface-raised focus:ring-2 focus:ring-focus/40',
    className
  )}
  {...props}
/>
