<script lang="ts">
  import type { Style } from '../collections/video';
  import { STYLES } from '../collections/video';
  import StyleTag from './StyleTag.svelte';
  import EditableChipRow from './EditableChipRow.svelte';

  interface Props {
    style: Style[];
    onupdatestyle: () => void;
    onremovestyle: (style: Style) => void;
  }

  let { style = $bindable(), onupdatestyle, onremovestyle }: Props = $props();

  let available = $derived(STYLES.filter((s) => !style.includes(s)));

  const handleAddStyle = (styleToAdd: Style) => {
    if (!style.some((s) => s === styleToAdd)) {
      style = [...style, styleToAdd];
      onupdatestyle();
    }
  };

  const handleRemoveStyle = (styleToRemove: Style) => {
    onremovestyle(styleToRemove);
  };

  const onSelect = (
    e: Event & { currentTarget: EventTarget & HTMLSelectElement }
  ) => {
    const value = e.currentTarget.value as Style;
    e.currentTarget.value = '';
    if (value) handleAddStyle(value);
  };

  $effect(() => {
    style = style;
  });
</script>

<EditableChipRow editLabel="Modifier les styles">
  {#snippet children(editing)}
    {#if style.length}
      {#each style as s}
        <StyleTag
          name={s}
          removable={editing}
          onremove={() => handleRemoveStyle(s)}
        />
      {/each}
    {:else if !editing}
      <span class="rownote">Aucun style</span>
    {/if}
  {/snippet}

  {#snippet add()}
    {#if available.length}
      <select class="addsel" aria-label="Ajouter un style" onchange={onSelect}>
        <option value="" selected>+ Style</option>
        {#each available as availableStyle}
          <option>{availableStyle}</option>
        {/each}
      </select>
    {/if}
  {/snippet}
</EditableChipRow>
