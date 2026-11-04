<script lang="ts">
  import { Clock, Play, X } from '@lucide/svelte';
  import type { VideoData } from './collectionCreation.store';

  interface Props {
    onRemoveFile: (video: VideoData) => void;
    play: (video: VideoData) => void;
    video: VideoData;
  }

  const { onRemoveFile, play, video }: Props = $props();
</script>

<div class="flex items-center gap-3 p-3 bg-gray-50 rounded-lg group">
  <div
    class="flex-shrink-0 w-40 h-24 bg-gradient-to-br from-blue-50 to-indigo-50 rounded-lg flex items-center justify-center overflow-hidden relative group cursor-pointer"
    onclick={() => play(video)}
  >
    <video poster={video.thumbnail ?? ''} class="w-full h-full object-cover" />
    <div
      class="absolute inset-0 bg-black/40 flex items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity"
    >
      <Play class="w-12 h-12 text-white fill-white" />
    </div>
    <div
      class="absolute bottom-2 right-2 bg-black/75 text-white text-xs px-2 py-1 rounded flex items-center gap-1"
    >
      <Clock class="w-3 h-3" />
      {video.duration.toHumanReadable()}
    </div>
  </div>
  <div class="flex-1 min-w-0">
    <p class="font-medium truncate">{video.name}</p>
    <p class="text-xs text-muted-foreground">
      {video.size.toHumanReadable()} MB
    </p>
    <p class="text-xs text-muted-foreground">
      {video.duration.toHumanReadable()}
    </p>
  </div>
  <button
    type="button"
    onclick={() => onRemoveFile(video)}
    class="p-1 hover:bg-red-50 rounded opacity-0 group-hover:opacity-100 transition-opacity"
  >
    <X class="w-4 h-4 text-red-500" />
  </button>
</div>
