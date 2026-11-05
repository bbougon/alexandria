<script lang="ts">
  import type { Snippet } from 'svelte';
  import { Pencil, Check } from '@lucide/svelte';
  import { cn } from './classCombinator';

  interface Props {
    /** Rendu des chips ; reçoit l'état d'édition pour afficher ou non la croix. */
    children: Snippet<[boolean]>;
    /** Contrôle d'ajout (popover, champ…), affiché uniquement en édition. */
    add?: Snippet;
    /** Libellé accessible de l'icône d'édition au repos. */
    editLabel: string;
  }

  let { children, add, editLabel }: Props = $props();

  let editing = $state(false);
</script>

<div class="flex flex-wrap items-center gap-1.5">
  {@render children(editing)}

  {#if editing && add}
    {@render add()}
  {/if}

  <button
    type="button"
    class={cn(
      'inline-grid h-6 w-6 cursor-pointer place-items-center rounded-sm bg-transparent transition-colors hover:bg-surface-sunken',
      editing ? 'text-success' : 'text-ink-subtle hover:text-brand-strong'
    )}
    aria-label={editing ? 'Terminer la modification' : editLabel}
    title={editing ? 'Terminer' : editLabel}
    onclick={() => (editing = !editing)}
  >
    {#if editing}
      <Check class="h-3.5 w-3.5" />
    {:else}
      <Pencil class="h-3.5 w-3.5" />
    {/if}
  </button>
</div>
