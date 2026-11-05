<script lang="ts">
  import type { HTMLInputAttributes } from 'svelte/elements';
  import { cn } from './classCombinator';

  interface Props extends HTMLInputAttributes {
    /** Libellé/placeholder du champ. */
    label: string;
    value: string;
    /** Variante de style reprise de la maquette (titre / artiste / morceau). */
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

  // Habillage « édition au survol » en utilitaires Tailwind. Le champ `<input>`
  // n'hérite pas de la police par variante : on fixe donc `font-*` explicitement
  // (`[font:inherit]` pour la variante neutre). `[field-sizing:content]` fait
  // épouser la largeur au contenu.
  const base =
    '[field-sizing:content] -mx-1 -my-0.5 min-w-[3ch] cursor-text rounded-sm border border-transparent bg-transparent px-1.5 py-0.5 outline-none placeholder:text-ink-subtle hover:border-border hover:bg-surface-sunken focus:border-focus focus:bg-surface-raised focus:ring-[3px] focus:ring-focus/25';
  const variants = {
    title: 'block w-full font-display text-base font-bold leading-[21px] text-ink',
    artist: 'font-sans text-[13px] font-semibold text-ink',
    song: 'font-sans text-[13px] text-ink-muted',
    plain: '[font:inherit] text-inherit',
  } as const;

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
  class={cn(base, variants[variant], className)}
  {...props}
/>
