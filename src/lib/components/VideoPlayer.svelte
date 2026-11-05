<script lang="ts">
  import Dialog from './dialog/Dialog.svelte';
  import VideoInfoPanel from './VideoInfoPanel.svelte';
  import type { PlayableVideo } from '../collections/video';

  let elementDialog: Dialog | undefined = $state();
  let video: PlayableVideo | undefined = $state();
  let videoEl: HTMLVideoElement | null = $state(null);

  export const play = (videoToPlay: PlayableVideo) => {
    video = videoToPlay;
    elementDialog?.display();
  };

  const close = () => {
    videoEl?.pause();
    video = undefined;
    elementDialog?.close();
  };
</script>

<Dialog bind:this={elementDialog}>
  {#if video}
    <section class="player">
      <div class="stage">
        <!-- svelte-ignore a11y_media_has_caption -->
        <video
          bind:this={videoEl}
          controls
          autoplay
          src={video.play()}
          poster={video.thumbnail ?? ''}
        ></video>
      </div>
      <VideoInfoPanel {video} onclose={close} />
    </section>
  {/if}
</Dialog>
