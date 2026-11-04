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
  class="group cursor-pointer rounded-xl border-2 border-dashed border-border-strong p-12 text-center transition-colors hover:border-brand hover:bg-brand-tint/40"
>
  <Upload
    class="mx-auto mb-4 h-12 w-12 text-ink-subtle transition-colors group-hover:text-brand"
  />
  <p class="mb-2 text-lg">Click to upload video files</p>
  <p class="text-sm text-ink-subtle">or drag and drop</p>
</div>
