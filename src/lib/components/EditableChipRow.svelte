<script lang="ts">
  import type { Snippet } from 'svelte';
  import { Pencil, Check } from '@lucide/svelte';

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

<div class="trow">
  {@render children(editing)}

  {#if editing && add}
    {@render add()}
  {/if}

  <button
    type="button"
    class="rowic"
    class:done={editing}
    aria-label={editing ? 'Terminer la modification' : editLabel}
    title={editing ? 'Terminer' : editLabel}
    onclick={() => (editing = !editing)}
  >
    {#if editing}
      <Check />
    {:else}
      <Pencil />
    {/if}
  </button>
</div>
