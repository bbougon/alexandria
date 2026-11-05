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
  class="drop"
>
  <Upload />
  <span class="big">Déposez vos vidéos ici</span>
  <span class="sm">ou cliquez pour parcourir — mp4, mov, mkv, webm</span>
</div>
