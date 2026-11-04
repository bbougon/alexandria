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
  import Card from '../components/Card.svelte';
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

<div class="max-w-4xl mx-auto">
  <h1 class="text-3xl font-semibold mb-6">Create New Collection</h1>

  <div class="space-y-6">
    <Card class="bg-surface-raised p-6">
      <p class="mb-4 font-medium">Video Files</p>

      <UploadVideos {pickVideo} />

      {#if $collectionCreationStore.videos.length > 0}
        <div class="mt-6 space-y-2">
          <p class="text-sm font-medium mb-3">
            Selected Files ({$collectionCreationStore.videos.length})
          </p>
          {#each $collectionCreationStore.videos as video}
            <VideoInformation {onRemoveFile} {play} {video} />
          {/each}
        </div>
      {/if}
    </Card>

    <div class="flex gap-3">
      <Button
        type="submit"
        size="lg"
        enabled={$selectedCollection.collection !== undefined &&
          $selectedCollection?.collection.videos.length > 0}
        onclick={createCollection}
      >
        Create collection
      </Button>
      <Button type="button" variant="outline" size="lg" onclick={onCancel}>
        Cancel
      </Button>
    </div>
  </div>
  <VideoPlayer bind:this={videoPlayer} />
</div>
