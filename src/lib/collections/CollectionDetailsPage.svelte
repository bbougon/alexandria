<script lang="ts">
  import { selectedCollection } from './collection.store';
  import { Folder, Plus } from '@lucide/svelte';
  import VideoCard from '../components/VideoCard.svelte';
  import Button from '../components/Button.svelte';
  import UploadVideos from './UploadVideos.svelte';
  import Dialog from '../components/dialog/Dialog.svelte';
  import DialogContent from '../components/dialog/DialogContent.svelte';
  import DialogHeader from '../components/dialog/DialogHeader.svelte';
  import DialogTitle from '../components/dialog/DialogTitle.svelte';
  import { open } from '@tauri-apps/plugin-dialog';
  import { listen } from '@tauri-apps/api/event';
  import {
    toVideo,
    toVideoDataDTO,
    type VideoAddedToCollection,
    type VideoDataRetrievedDTO,
  } from './video.tauri';
  import { invoke } from '@tauri-apps/api/core';
  import { collectionUpdateStore } from './collectionUpdate.store';
  import VideoInformation from './VideoInformation.svelte';
  import { type VideoData } from './collectionCreation.store';
  import { fileConverter } from './file-converter';
  import VideoPlayer from '../components/VideoPlayer.svelte';
  import type { CollectionUpdated } from './collection.tauri';

  let videoPlayer: VideoPlayer | undefined = $state();
  let addModal: Dialog | undefined = $state();

  const openAddVideos = () => {
    collectionUpdateStore.initialize({
      id: $selectedCollection.collection!.id,
      videos: [],
    });
    addModal?.display();
  };

  const closeAddVideos = () => {
    collectionUpdateStore.reset();
    addModal?.close();
  };

  const pickVideo = async () => {
    const result = await open({
      multiple: true,
      directory: false,
      filters: [{ name: 'Videos', extensions: ['mp4', 'mov', 'mkv', 'webm'] }],
    });
    const paths = [];
    if (result !== null) {
      if (Array.isArray(result)) {
        paths.push(...result);
      } else {
        paths.push(result);
      }
    }
    await retrieveVideosData(paths);
  };

  const retrieveVideosData = async (paths: string[]) => {
    const unlistenToVideoRetrieved = await listen<VideoDataRetrievedDTO>(
      'video_data:retrieved',
      (e) => {
        const videoDataRetrieved = e.payload;
        collectionUpdateStore.addVideo(videoDataRetrieved);
      }
    );
    await invoke<string[]>('retrieve_videos_data', { paths });
    unlistenToVideoRetrieved();
  };

  const onRemoveFile = (video: VideoData) => {
    collectionUpdateStore.removeVideo(video);
  };

  const play = (video: VideoData) => {
    videoPlayer?.play({
      name: video.name,
      thumbnail: video.thumbnail,
      play: () => fileConverter.convertFile(video.path),
    });
  };

  const addVideosToCollection = async (): Promise<void> => {
    const unlistenToVideoAdded = await listen<VideoAddedToCollection>(
      'video:added',
      (e) => {
        const videoAdded = e.payload;

        if (videoAdded.thumbnail) {
          $selectedCollection.addVideo(toVideo(videoAdded));
        }
      }
    );
    const unlistenToCollectionUpdated = await listen<CollectionUpdated>(
      'collection:updated',
      () => {
        collectionUpdateStore.reset();
        addModal?.close();
      }
    );

    await invoke('add_videos_to_collection', {
      collectionId: $selectedCollection.collection!.id,
      videos: $collectionUpdateStore.videos.map((video) => toVideoDataDTO(video)),
    });

    unlistenToVideoAdded();
    unlistenToCollectionUpdated();
  };
</script>

<div>
  {#if $selectedCollection.collection}
    <div class="mb-8 flex items-start justify-between gap-4">
      <div>
        <div class="mb-2 flex items-center gap-3">
          <Folder class="h-8 w-8 text-brand" />
          <h1 class="text-3xl font-semibold">
            {$selectedCollection.collection.title}
          </h1>
        </div>
        <p class="text-ink-muted">
          {$selectedCollection.collection.videos.length} video{$selectedCollection
            .collection.videos.length !== 1
            ? 's'
            : ''}
        </p>
      </div>

      <Button onclick={openAddVideos} class="shrink-0">
        <Plus class="mr-2 h-4 w-4" />
        Add Videos
      </Button>
    </div>

    <div
      class="grid gap-4 [grid-template-columns:repeat(auto-fill,minmax(360px,1fr))]"
    >
      {#each $selectedCollection.collection.videos as video, i (video)}
        <VideoCard bind:video={$selectedCollection.collection.videos[i]} />
      {/each}
    </div>
  {/if}
</div>

<Dialog bind:this={addModal}>
  <DialogContent
    class="max-h-[85vh] gap-4 overflow-y-auto sm:max-w-[560px]"
    onclose={closeAddVideos}
  >
    <DialogHeader>
      <DialogTitle>Add videos</DialogTitle>
    </DialogHeader>

    <UploadVideos {pickVideo} />

    {#if $collectionUpdateStore.videos.length > 0}
      <div class="flex flex-col gap-2">
        {#each $collectionUpdateStore.videos as video}
          <VideoInformation {onRemoveFile} {play} {video} />
        {/each}
      </div>
    {/if}

    <div class="flex items-center gap-3 border-t border-border pt-4">
      <span class="mr-auto text-sm text-ink-muted">
        {$collectionUpdateStore.videos.length} video{$collectionUpdateStore.videos
          .length !== 1
          ? 's'
          : ''} selected
      </span>
      <Button variant="outline" onclick={closeAddVideos}>Cancel</Button>
      <Button
        enabled={$collectionUpdateStore.videos.length > 0}
        onclick={addVideosToCollection}
      >
        Add to collection
      </Button>
    </div>
  </DialogContent>
</Dialog>

<VideoPlayer bind:this={videoPlayer} />
