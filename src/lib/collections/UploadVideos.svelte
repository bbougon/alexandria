<script lang="ts">
  import { Upload } from '@lucide/svelte';

  interface Props {
    pickVideo: () => void;
  }

  const { pickVideo }: Props = $props();
</script>

<div
  onclick={pickVideo}
  role="button"
  tabindex="0"
  onkeydown={(e) => e.key === 'Enter' && pickVideo()}
  aria-roledescription="video file picker"
  class="border-2 border-dashed rounded-lg p-12 text-center cursor-pointer transition-colors border-gray-300 hover:border-blue-400 hover:bg-blue-50/50"
>
  <Upload class="w-12 h-12 mx-auto mb-4 text-gray-400" />
  <p class="text-lg mb-2">Click to upload video files</p>
  <p class="text-sm text-muted-foreground">or drag and drop</p>
</div>
