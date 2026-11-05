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
  class="flex cursor-pointer flex-col items-center gap-1.5 rounded-lg border-[1.5px] border-dashed border-border-strong bg-surface-sunken p-[30px] text-center text-ink-muted transition hover:border-brand hover:bg-brand-tint hover:text-brand-strong"
>
  <Upload class="h-7 w-7" />
  <span class="text-[15px] font-semibold text-ink">Déposez vos vidéos ici</span>
  <span class="text-[12.5px]">ou cliquez pour parcourir — mp4, mov, mkv, webm</span>
</div>
