<script lang="ts">
  import Button from './Button.svelte';
  import Input from './Input.svelte';
  import { Plus } from '@lucide/svelte';
  import Tag from './Tag.svelte';

  interface Props {
    tags: string[];
    onaddvideotag: () => void;
    onremovevideotag: (tag: string) => void;
  }

  let { tags = $bindable(), onaddvideotag, onremovevideotag }: Props = $props();

  let newTag = $state('');
  let isEditing = $state(false);

  $effect(() => {
    tags = tags;
  });

  const handleAddTag = () => {
    tags = [...tags, newTag];
    onaddvideotag();
    isEditing = false;
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
      isEditing = false;
    }
  };
</script>

<div class="flex flex-wrap gap-2 items-center">
  {#each tags as tag}
    <Tag label={tag} removable onremove={() => handleRemoveTag(tag)} />
  {/each}

  {#snippet showAddTag()}
    <Input
      value={newTag}
      onchange={(e) => (newTag = e.currentTarget.value)}
      onblur={handleAddTag}
      onkeydown={handleKeyDown}
      placeholder="New tag..."
      class="w-24 h-7"
      autofocus
    />
  {/snippet}

  {#if isEditing}
    {@render showAddTag()}
  {/if}
  <Button
    variant="outline"
    size="sm"
    class="h-7 px-2"
    onclick={() => (isEditing = true)}
  >
    <Plus class="w-3 h-3 mr-1" />
    Add tag
  </Button>
</div>
