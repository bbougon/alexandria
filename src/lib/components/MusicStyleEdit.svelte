<script lang="ts">
  import { Popover, PopoverContent, PopoverTrigger } from './popover/PopOver.svelte';
  import Button from './Button.svelte';
  import { Plus, X } from '@lucide/svelte';
  import type { Style } from '../collections/video';
  import { STYLES } from '../collections/video';
  import Badge from './Badge.svelte';

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

<div class="flex flex-wrap gap-2 items-center">
  {#each style as style}
    <Badge variant="secondary" class="gap-1 pr-1">
      {style}
      <button
        onclick={() => handleRemoveStyle(style)}
        class="ml-1 hover:bg-muted rounded-full p-0.5"
      >
        <X class="w-3 h-3" />
      </button>
    </Badge>
  {/each}

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
                      ? 'bg-blue-100 text-blue-700 font-medium'
                      : 'hover:bg-gray-100'
                  }
                `}
          >
            {availableStyle}
          </button>
        {/each}
      </div>
    </PopoverContent>
  </Popover>
</div>
