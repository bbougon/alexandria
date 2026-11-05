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

<div class="improw">
  <Thumbnail
    variant="import"
    thumbnail={video.thumbnail}
    duration={video.duration.toHumanReadable()}
    onplay={() => play(video)}
  />
  <span class="impname">{video.name}</span>
  <span class="impsize">{video.size.toHumanReadable()}</span>
  <button
    type="button"
    class="iconsm danger"
    aria-label={`Retirer ${video.name}`}
    onclick={() => onRemoveFile(video)}
  >
    <Trash2 />
  </button>
</div>
