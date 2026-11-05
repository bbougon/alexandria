<script lang="ts">
  import { Trash2 } from '@lucide/svelte';
  import type { VideoData } from './collectionCreation.store';
  import Thumbnail from '../components/Thumbnail.svelte';

  interface Props {
    onRemoveFile: (video: VideoData) => void;
    play: (video: VideoData) => void;
    video: VideoData;
  }

  const { onRemoveFile, play, video }: Props = $props();
</script>

<div
  class="flex items-center gap-3 rounded-md border border-border bg-surface-raised px-2.5 py-2"
>
  <Thumbnail
    variant="import"
    thumbnail={video.thumbnail}
    duration={video.duration.toHumanReadable()}
    onplay={() => play(video)}
  />
  <span class="min-w-0 flex-1 truncate text-sm font-semibold text-ink"
    >{video.name}</span
  >
  <span class="whitespace-nowrap font-mono text-[11px] text-ink-muted"
    >{video.size.toHumanReadable()}</span
  >
  <button
    type="button"
    class="grid h-[30px] w-[30px] cursor-pointer place-items-center rounded-md text-ink-muted transition hover:bg-danger-tint hover:text-danger"
    aria-label={`Retirer ${video.name}`}
    onclick={() => onRemoveFile(video)}
  >
    <Trash2 class="h-4 w-4" />
  </button>
</div>
