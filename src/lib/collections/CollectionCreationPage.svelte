<script lang="ts">
  import VideoInformation from './VideoInformation.svelte';
  import UploadVideos from './UploadVideos.svelte';
  import { open } from '@tauri-apps/plugin-dialog';
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { selectedCollection, selectedCollectionId } from './collection.store';
  import { collectionsStore } from './collections.store';
  import {
    toVideo,
    toVideoDataDTO,
    type VideoAddedToCollection,
    type VideoDataRetrievedDTO,
  } from './video.tauri';
  import { type CollectionCreated, toCollection } from './collection.tauri';
  import Button from '../components/Button.svelte';
  import { pageStore } from '../kit/pages/pageStore';
  import {
    collectionCreationStore,
    type VideoData,
  } from './collectionCreation.store';
  import VideoPlayer from '../components/VideoPlayer.svelte';
  import { fileConverter } from './file-converter';

  let videoPlayer: VideoPlayer | undefined = $state();

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

  $effect(() => {
    const unlisten = getCurrentWindow().listen<{ paths: string[] }>(
      'tauri://drag-drop',
      async (event) => {
        if (event.payload.paths.length > 0) {
          await retrieveVideosData(event.payload.paths);
        }
      }
    );

    return () => {
      unlisten.then((u) => u());
    };
  });

  const retrieveVideosData = async (paths: string[]) => {
    const unlistenToVideoRetrieved = await listen<VideoDataRetrievedDTO>(
      'video_data:retrieved',
      (e) => {
        const videoDataRetrieved = e.payload;
        collectionCreationStore.addVideo(videoDataRetrieved);
      }
    );
    await invoke<string[]>('retrieve_videos_data', { paths });
    unlistenToVideoRetrieved();
  };

  const createCollection = async () => {
    if ($collectionCreationStore.videos.length === 0) return;
    const unlistenToVideoAdded = await listen<VideoAddedToCollection>(
      'video:added',
      (e) => {
        const videoAdded = e.payload;

        if (videoAdded.thumbnail) {
          $selectedCollection.addVideo(toVideo(videoAdded));
        }
      }
    );
    const unlistenToCollectionCreated = await listen<CollectionCreated>(
      'collection:created',
      (e) => {
        const collectionCreated = e.payload;
        collectionsStore.addCollection(toCollection(collectionCreated));
        selectedCollectionId.initialize(collectionCreated.collection_id);
        collectionCreationStore.reset();
        pageStore.goTo('CollectionDetailsPage');
      }
    );

    await invoke<string[]>('create_collection', {
      videos: $collectionCreationStore.videos.map((video) => toVideoDataDTO(video)),
    });

    unlistenToVideoAdded();
    unlistenToCollectionCreated();
  };

  const onCancel = () => {
    collectionCreationStore.reset();
    pageStore.goTo('HomePage');
  };
  const onRemoveFile = (video: VideoData) => {
    collectionCreationStore.removeVideo(video);
  };
  const play = (video: VideoData) => {
    videoPlayer?.play({
      name: video.name,
      thumbnail: video.thumbnail,
      play: () => fileConverter.convertFile(video.path),
    });
  };
</script>

<div class="mx-auto max-w-4xl">
  <div class="mb-6">
    <h1 class="text-3xl font-semibold">New Collection</h1>
    <p class="mt-1 text-ink-muted">
      Import video files — the collection is named automatically by its creation
      date.
    </p>
  </div>

  <div class="space-y-4 rounded-2xl border border-border bg-surface-raised p-6">
    <UploadVideos {pickVideo} />

    {#if $collectionCreationStore.videos.length > 0}
      <div class="flex flex-col gap-2">
        {#each $collectionCreationStore.videos as video}
          <VideoInformation {onRemoveFile} {play} {video} />
        {/each}
      </div>
    {/if}

    <div class="flex items-center gap-3 border-t border-border pt-4">
      <span class="mr-auto text-sm text-ink-muted">
        {$collectionCreationStore.videos.length} video{$collectionCreationStore
          .videos.length !== 1
          ? 's'
          : ''} selected
      </span>
      <Button variant="outline" size="lg" onclick={onCancel}>Cancel</Button>
      <Button
        size="lg"
        enabled={$collectionCreationStore.videos.length > 0}
        onclick={createCollection}
      >
        Create collection
      </Button>
    </div>
  </div>

  <VideoPlayer bind:this={videoPlayer} />
</div>
