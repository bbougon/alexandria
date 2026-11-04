<script lang="ts">
  import { Popover, PopoverContent, PopoverTrigger } from './popover/PopOver.svelte';
  import Button from './Button.svelte';
  import { Plus } from '@lucide/svelte';
  import type { Style } from '../collections/video';
  import { STYLES } from '../collections/video';
  import StyleTag from './StyleTag.svelte';
  import EditableChipRow from './EditableChipRow.svelte';

  interface Props {
    style: Style[];
    onupdatestyle: () => void;
    onremovestyle: (style: Style) => void;
  }

  let isOpen = $state(false);

  let { style = $bindable(), onupdatestyle, onremovestyle }: Props = $props();

  const handleToggleStyle = (styleToAdd: Style) => {
    if (style.some((s) => s === styleToAdd)) {
      onremovestyle(styleToAdd);
    } else {
      style = [...style, styleToAdd];
      onupdatestyle();
    }
  };

  const handleRemoveStyle = (style: Style) => {
    onremovestyle(style);
  };

  $effect(() => {
    style = style;
  });
</script>

<EditableChipRow editLabel="Modifier les styles">
  {#snippet children(editing)}
    {#each style as style}
      <StyleTag
        name={style}
        removable={editing}
        onremove={() => handleRemoveStyle(style)}
      />
    {/each}
  {/snippet}

  {#snippet add()}
    <Popover open={isOpen} onOpenChange={() => (isOpen = !isOpen)}>
      <PopoverTrigger>
        <Button variant="outline" size="sm" class="h-7 px-2">
          <Plus class="w-3 h-3 mr-1" />
          Add style
        </Button>
      </PopoverTrigger>
      <PopoverContent class="w-64 p-2" align="start">
        <div class="grid grid-cols-2 gap-1 max-h-64 overflow-y-auto">
          {#each STYLES as availableStyle}
            <button
              onclick={(e) => {
                e.preventDefault();
                handleToggleStyle(availableStyle);
              }}
              class={`
                    text-left px-3 py-2 rounded text-sm transition-colors
                    ${
                      style.includes(availableStyle)
                        ? 'bg-brand-tint text-brand font-medium'
                        : 'hover:bg-surface-sunken'
                    }
                  `}
            >
              {availableStyle}
            </button>
          {/each}
        </div>
      </PopoverContent>
    </Popover>
  {/snippet}
</EditableChipRow>
