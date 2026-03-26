<script lang="ts">
  import { open } from '@tauri-apps/plugin-dialog';
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { selectedCollection, selectedCollectionId } from './collection.store';
  import { collectionsStore } from './collections.store';
  import {
    toVideo,
    type VideoAddedToCollection,
    type VideoDataRetrievedDTO,
  } from './video.tauri';
  import { type CollectionCreated, toCollection } from './collection.tauri';
  import Card from '../components/Card.svelte';
  import { FileVideo, Upload } from '@lucide/svelte';
  import Button from '../components/Button.svelte';
  import { pageStore } from '../kit/pages/pageStore';
  import { collectionCreationStore } from './collectionCreation.store';

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
        pageStore.goTo('CollectionDetailsPage');
      }
    );

    await invoke<string[]>('create_collection', {
      paths: $collectionCreationStore.videos.map((video) => video.path),
    });

    unlistenToVideoAdded();
    unlistenToCollectionCreated();
  };

  const onCancel = () => {
    collectionCreationStore.reset();
    pageStore.goTo('HomePage');
  };
</script>

<div class="max-w-4xl mx-auto">
  <h1 class="text-3xl font-semibold mb-6">Create New Collection</h1>

  <div class="space-y-6">
    <!--    <Card class="p-6 bg-white">-->
    <!--      <label class="block mb-2 font-medium">Collection Name</label>-->
    <!--      <input-->
    <!--        type="text"-->
    <!--        placeholder="Enter collection name..."-->
    <!--        class="max-w-md"-->
    <!--      />-->
    <!--    </Card>-->

    <Card class="p-6 bg-white">
      <label class="block mb-4 font-medium">Video Files</label>

      <div
        onclick={pickVideo}
        role="button"
        tabindex="0"
        onkeydown={(e) => e.key === 'Enter' && pickVideo()}
        aria-roledescription="video file picker"
        class="border-2 border-dashed rounded-lg p-12 text-center cursor-pointer transition-colors border-gray-300 hover:border-blue-400 hover:bg-blue-50/50"
      >
        <Upload class="w-12 h-12 mx-auto mb-4 text-gray-400" />
        <p class="text-lg mb-2">Click to upload video files</p>
        <p class="text-sm text-muted-foreground">or drag and drop</p>
      </div>

      {#if $collectionCreationStore.videos.length > 0}
        <div class="mt-6 space-y-2">
          <p class="text-sm font-medium mb-3">
            Selected Files ({$collectionCreationStore.videos.length})
          </p>
          {#each $collectionCreationStore.videos as video}
            <div class="flex items-center gap-3 p-3 bg-gray-50 rounded-lg group">
              <FileVideo class="w-5 h-5 text-blue-500 flex-shrink-0" />
              <div class="flex-1 min-w-0">
                <p class="font-medium truncate">{video.name}</p>
                <p class="text-xs text-muted-foreground">
                  {video.size.toHumanReadable()} MB
                </p>
                <p class="text-xs text-muted-foreground">
                  {video.duration.toHumanReadable()}
                </p>
              </div>
              <!--              <button-->
              <!--                type="button"-->
              <!--                // onClick={() => handleRemoveFile(index)}-->
              <!--                class="p-1 hover:bg-red-50 rounded opacity-0 group-hover:opacity-100 transition-opacity"-->
              <!--              >-->
              <!--                <X class="w-4 h-4 text-red-500" />-->
              <!--              </button>-->
            </div>
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
</div>
