<script lang="ts">
  import Tag from './Tag.svelte';
  import EditableChipRow from './EditableChipRow.svelte';

  interface Props {
    tags: string[];
    onaddvideotag: () => void;
    onremovevideotag: (tag: string) => void;
  }

  let { tags = $bindable(), onaddvideotag, onremovevideotag }: Props = $props();

  let newTag = $state('');

  $effect(() => {
    tags = tags;
  });

  const handleAddTag = () => {
    if (!newTag.trim()) return;
    tags = [...tags, newTag];
    onaddvideotag();
    newTag = '';
  };

  const handleRemoveTag = (tag: string) => {
    onremovevideotag(tag);
  };

  const handleKeyDown = (
    e: KeyboardEvent & {
      currentTarget: EventTarget & HTMLInputElement;
    }
  ) => {
    if (e.key === 'Enter') {
      newTag = e.currentTarget.value;
      e.preventDefault();
      handleAddTag();
    } else if (e.key === 'Escape') {
      newTag = '';
    }
  };
</script>

<EditableChipRow editLabel="Modifier les tags">
  {#snippet children(editing)}
    {#if tags.length}
      {#each tags as tag}
        <Tag label={tag} removable={editing} onremove={() => handleRemoveTag(tag)} />
      {/each}
    {:else if !editing}
      <span class="rownote">Aucun tag</span>
    {/if}
  {/snippet}

  {#snippet add()}
    <input
      class="addtag"
      bind:value={newTag}
      onblur={handleAddTag}
      onkeydown={handleKeyDown}
      placeholder="+ tag"
      aria-label="Ajouter un tag"
    />
  {/snippet}
</EditableChipRow>
