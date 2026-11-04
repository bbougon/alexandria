<script lang="ts">
  import Dialog from './dialog/Dialog.svelte';
  import DialogHeader from './dialog/DialogHeader.svelte';
  import DialogTitle from './dialog/DialogTitle.svelte';
  import DialogContent from './dialog/DialogContent.svelte';
  import VideoInfoPanel from './VideoInfoPanel.svelte';
  import type { PlayableVideo } from '../collections/video';

  let elementDialog: Dialog | undefined = $state();
  let video: PlayableVideo | undefined = $state();
  let videoEl: HTMLVideoElement | null = null;

  export const play = (videoToPlay: PlayableVideo) => {
    video = videoToPlay;
    elementDialog?.display();
  };

  const close = () => {
    videoEl?.pause();
    video = undefined;
  };
</script>

<Dialog bind:this={elementDialog}>
  <DialogContent class="max-w-[95%] sm:max-w-5xl" onclose={close}>
    <DialogHeader>
      <DialogTitle>{video?.name}</DialogTitle>
    </DialogHeader>
    <div class="grid gap-6 md:grid-cols-[minmax(0,1fr)_320px]">
      <div class="min-w-0">
        <video
          bind:this={videoEl}
          controls
          autoPlay
          class="w-full rounded-lg bg-black"
          src={video?.play() ?? ''}
          poster={video?.thumbnail ?? ''}
        >
        </video>
      </div>
      {#if video}
        <VideoInfoPanel {video} />
      {/if}
    </div>
  </DialogContent>
</Dialog>
