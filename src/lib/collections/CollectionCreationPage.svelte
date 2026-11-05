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
      path: video.path,
      size: video.size,
      duration: video.duration,
      play: () => fileConverter.convertFile(video.path),
    });
  };
</script>

<div
  class="flex flex-col gap-[18px] pb-[calc(32px_+_env(safe-area-inset-bottom,0px))] pl-[max(20px,env(safe-area-inset-left,0px))] pr-[max(20px,env(safe-area-inset-right,0px))] pt-[22px]"
>
  <div class="flex flex-wrap items-end gap-[14px]">
    <div>
      <h2 class="font-display text-[26px] font-bold tracking-[-0.02em]">
        Nouvelle collection
      </h2>
      <p class="mt-1 text-sm text-ink-muted">
        Importez des fichiers vidéo — la collection est nommée automatiquement par sa
        date de création.
      </p>
    </div>
  </div>

  <UploadVideos {pickVideo} />

  {#if $collectionCreationStore.videos.length > 0}
    <div class="grid grid-cols-2 gap-2 max-[620px]:grid-cols-1">
      {#each $collectionCreationStore.videos as video}
        <VideoInformation {onRemoveFile} {play} {video} />
      {/each}
    </div>
  {/if}

  <div class="flex items-center justify-end gap-2.5">
    <span class="mr-auto text-[13px] text-ink-muted">
      {$collectionCreationStore.videos.length} vidéo{$collectionCreationStore.videos
        .length !== 1
        ? 's'
        : ''} sélectionnée{$collectionCreationStore.videos.length !== 1 ? 's' : ''}
    </span>
    <button
      type="button"
      class="inline-flex cursor-pointer items-center gap-2 rounded-md border border-border-strong bg-surface-raised px-[14px] py-[9px] text-sm font-semibold text-ink hover:border-brand hover:bg-brand-tint"
      onclick={onCancel}>Annuler</button
    >
    <button
      type="button"
      class="inline-flex cursor-pointer items-center gap-2 rounded-md bg-brand px-4 py-2.5 text-sm font-semibold text-on-brand shadow-sm hover:bg-brand-strong disabled:cursor-not-allowed disabled:opacity-50"
      disabled={$collectionCreationStore.videos.length === 0}
      onclick={createCollection}
    >
      Créer la collection
    </button>
  </div>

  <VideoPlayer bind:this={videoPlayer} />
</div>
