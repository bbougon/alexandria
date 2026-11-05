<script lang="ts">
  import type { HTMLInputAttributes } from 'svelte/elements';
  import { cn } from './classCombinator';

  interface Props extends HTMLInputAttributes {
    /** Libellé/placeholder du champ. */
    label: string;
    value: string;
    /** Variante de style reprise de la maquette (`hedit e-*`). */
    variant?: 'title' | 'artist' | 'song' | 'plain';
  }

  let {
    value = $bindable(),
    label,
    variant = 'plain',
    class: className,
    onkeydown,
    ...props
  }: Props = $props();

  // Le champ reste toujours éditable ; l'affordance (`hedit`) apparaît au survol
  // et au focus. Entrée valide la saisie en retirant le focus (déclenche onblur).
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
  class={cn('hedit', variant !== 'plain' && `e-${variant}`, className)}
  {...props}
/>
