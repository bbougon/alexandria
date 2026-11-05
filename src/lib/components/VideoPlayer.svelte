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
    <section
      class="grid max-h-full w-[min(980px,100%)] grid-cols-[1.6fr_1fr] overflow-auto rounded-xl border border-border bg-surface-raised shadow-lg max-[720px]:grid-cols-1"
    >
      <div
        class="relative grid aspect-video place-items-center overflow-hidden rounded-bl-xl rounded-tl-xl bg-black max-[720px]:rounded-bl-none max-[720px]:rounded-tr-xl"
      >
        <!-- svelte-ignore a11y_media_has_caption -->
        <video
          bind:this={videoEl}
          controls
          autoplay
          src={video.play()}
          poster={video.thumbnail ?? ''}
          class="absolute inset-0 h-full w-full bg-black object-contain"
        ></video>
      </div>
      <VideoInfoPanel {video} onclose={close} />
    </section>
  {/if}
</Dialog>
