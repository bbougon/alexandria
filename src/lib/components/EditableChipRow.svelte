<script lang="ts">
  import type { Snippet } from 'svelte';
  import { Pencil, Check } from '@lucide/svelte';
  import IconButton from './IconButton.svelte';

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

<div class="flex flex-wrap items-center gap-2">
  {@render children(editing)}

  {#if editing && add}
    {@render add()}
  {/if}

  <IconButton
    size="sm"
    label={editing ? 'Terminer la modification' : editLabel}
    onclick={() => (editing = !editing)}
  >
    {#if editing}
      <Check class="size-4" />
    {:else}
      <Pencil class="size-4" />
    {/if}
  </IconButton>
</div>
