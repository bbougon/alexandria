<script lang="ts">
  import { X } from '@lucide/svelte';
  import type { VideoData } from './collectionCreation.store';
  import Thumbnail from '../components/Thumbnail.svelte';
  import IconButton from '../components/IconButton.svelte';

  interface Props {
    onRemoveFile: (video: VideoData) => void;
    play: (video: VideoData) => void;
    video: VideoData;
  }

  const { onRemoveFile, play, video }: Props = $props();
</script>

<div class="group flex items-center gap-3 rounded-lg bg-surface-sunken p-3">
  <Thumbnail
    variant="import"
    thumbnail={video.thumbnail}
    duration={video.duration.toHumanReadable()}
    onplay={() => play(video)}
  />
  <div class="min-w-0 flex-1">
    <p class="truncate font-medium">{video.name}</p>
    <p class="font-mono text-xs text-ink-subtle">
      {video.size.toHumanReadable()}
    </p>
  </div>
  <IconButton
    variant="danger"
    size="sm"
    label={`Retirer ${video.name}`}
    onclick={() => onRemoveFile(video)}
    class="opacity-0 transition-opacity group-hover:opacity-100"
  >
    <X class="size-4" />
  </IconButton>
</div>
