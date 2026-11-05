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
      <span class="text-[12.5px] text-ink-subtle">Aucun tag</span>
    {/if}
  {/snippet}

  {#snippet add()}
    <input
      class="w-[108px] rounded-sm border border-dashed border-border-strong bg-surface-sunken px-2.5 py-1 font-sans text-[12.5px] text-ink outline-none placeholder:text-ink-subtle focus:border-solid focus:border-focus focus:ring-[3px] focus:ring-focus/25"
      bind:value={newTag}
      onblur={handleAddTag}
      onkeydown={handleKeyDown}
      placeholder="+ tag"
      aria-label="Ajouter un tag"
    />
  {/snippet}
</EditableChipRow>
