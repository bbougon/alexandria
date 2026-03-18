<script lang="ts">
  import Dialog from './dialog/Dialog.svelte';
  import DialogHeader from './dialog/DialogHeader.svelte';
  import DialogTitle from './dialog/DialogTitle.svelte';
  import DialogContent from './dialog/DialogContent.svelte';
  import type { Video } from '../collections/video';

  let elementDialog: Dialog | undefined = $state();
  let video: Video | undefined = $state();
  let videoEl: HTMLVideoElement | null = null;

  export const play = (videoToPlay: Video) => {
    video = videoToPlay;
    elementDialog?.display();
  };

  const close = () => {
    videoEl?.pause();
  };
</script>

<Dialog bind:this={elementDialog}>
  <DialogContent class="max-w-[90%] sm:max-w-4xl" onclose={close}>
    <DialogHeader>
      <DialogTitle>{video?.name}</DialogTitle>
    </DialogHeader>
    <div class="w-full">
      <video
        bind:this={videoEl}
        controls
        autoPlay
        class="w-full rounded-lg"
        src={video?.play() ?? ''}
        poster={video?.thumbnail ?? ''}
      >
      </video>
      <div class="mt-4 text-sm text-muted-foreground">
        <p>
          <span class="font-medium">{video?.artist}</span> - {video?.song}
        </p>
        <!--{video.musicStyle && <p class="mt-1">Style: {video.musicStyle}</p>}-->
      </div>
    </div>
  </DialogContent>
</Dialog>
