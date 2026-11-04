<script lang="ts">
  import Input from './Input.svelte';
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
    {#each tags as tag}
      <Tag label={tag} removable={editing} onremove={() => handleRemoveTag(tag)} />
    {/each}
  {/snippet}

  {#snippet add()}
    <Input
      value={newTag}
      onchange={(e) => (newTag = e.currentTarget.value)}
      onblur={handleAddTag}
      onkeydown={handleKeyDown}
      placeholder="New tag..."
      class="w-24 h-7"
    />
  {/snippet}
</EditableChipRow>
