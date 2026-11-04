<script lang="ts">
  import { selectedCollection } from './collection.store';
  import { ChevronUp, Folder, Plus } from '@lucide/svelte';
  import VideoCard from '../components/VideoCard.svelte';
  import Button from '../components/Button.svelte';
  import UploadVideos from './UploadVideos.svelte';
  import Card from '../components/Card.svelte';
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
  let isAddingVideos = $state(false);

  const openAddVideosPanel = () => {
    isAddingVideos = !isAddingVideos;
    if (isAddingVideos) {
      collectionUpdateStore.initialize({
        id: $selectedCollection.collection!.id,
        videos: [],
      });
    }
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
        isAddingVideos = false;
      }
    );

    await invoke('add_videos_to_collection', {
      collectionId: $selectedCollection.collection!.id,
      videos: $collectionUpdateStore.videos.map((video) => toVideoDataDTO(video)),
    });

    unlistenToVideoAdded();
    unlistenToCollectionUpdated();
  };

  const onCancel = () => {
    collectionUpdateStore.reset();
  };
</script>

<div>
  {#if $selectedCollection.collection}
    <div class="mb-8 flex items-start justify-between gap-4">
      <div>
        <div class="flex items-center gap-3 mb-2">
          <Folder class="w-8 h-8 text-blue-500" />
          <h1 class="text-3xl font-semibold">
            {$selectedCollection.collection.title}
          </h1>
        </div>
        <p class="text-muted-foreground">
          {$selectedCollection.collection.videos.length} video{$selectedCollection
            .collection.videos.length !== 1
            ? 's'
            : ''}
        </p>
      </div>

      <Button
        onclick={openAddVideosPanel}
        class={`shrink-0 {isAddingVideos ? 'outline' : 'default'}`}
      >
        {#if isAddingVideos}
          <ChevronUp className="w-4 h-4" />
          Hide
        {:else}
          <Plus className="w-4 h-4" />
          Add Videos
        {/if}
      </Button>
    </div>

    {#if isAddingVideos}
      <Card class="mb-8 p-6 bg-white">
        <label class="block mb-4 font-medium">Add videos to this collection</label>
        <UploadVideos {pickVideo} />

        {#if $collectionUpdateStore.videos.length > 0}
          <Card class="p-6 mb-8 bg-white">
            <div class="mt-6 space-y-2">
              <p class="text-sm font-medium mb-3">
                Selected Files ({$collectionUpdateStore.videos.length})
              </p>
              {#each $collectionUpdateStore.videos as video}
                <VideoInformation {onRemoveFile} {play} {video} />
              {/each}
            </div>
            <VideoPlayer bind:this={videoPlayer} />
          </Card>
          <div class="flex gap-3">
            <Button
              type="submit"
              size="lg"
              enabled={$selectedCollection.collection !== undefined &&
                $selectedCollection?.collection.videos.length > 0}
              onclick={addVideosToCollection}
            >
              Add videos to collection
            </Button>
            <Button type="button" variant="outline" size="lg" onclick={onCancel}>
              Cancel
            </Button>
          </div>
        {/if}
      </Card>
    {/if}

    <div class="mt-8 space-y-4">
      <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
        {#each $selectedCollection.collection.videos as video, i (video)}
          <VideoCard bind:video={$selectedCollection.collection.videos[i]} />
        {/each}
      </div>
    </div>
  {/if}
</div>
