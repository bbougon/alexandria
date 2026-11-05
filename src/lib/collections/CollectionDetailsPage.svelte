<script lang="ts">
  import { selectedCollection } from './collection.store';
  import { Plus, X } from '@lucide/svelte';
  import VideoCard from '../components/VideoCard.svelte';
  import UploadVideos from './UploadVideos.svelte';
  import Dialog from '../components/dialog/Dialog.svelte';
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
      path: video.path,
      size: video.size,
      duration: video.duration,
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

<div
  class="flex flex-col gap-[18px] pb-[calc(32px_+_env(safe-area-inset-bottom,0px))] pl-[max(20px,env(safe-area-inset-left,0px))] pr-[max(20px,env(safe-area-inset-right,0px))] pt-[22px]"
>
  {#if $selectedCollection.collection}
    <div class="flex flex-wrap items-end gap-[14px]">
      <div>
        <h2 class="font-display text-[26px] font-bold tracking-[-0.02em]">
          {$selectedCollection.collection.title}
        </h2>
        <p class="mt-1 text-sm text-ink-muted">
          {$selectedCollection.collection.videos.length} vidéo{$selectedCollection
            .collection.videos.length !== 1
            ? 's'
            : ''}
        </p>
      </div>
      <div class="flex-1"></div>
      <button
        type="button"
        class="inline-flex cursor-pointer items-center gap-2 rounded-md border border-border-strong bg-surface-raised px-[14px] py-[9px] text-sm font-semibold text-ink hover:border-brand hover:bg-brand-tint"
        onclick={openAddVideos}
      >
        <Plus class="h-4 w-4" />
        Ajouter des vidéos
      </button>
    </div>

    <div
      class="grid grid-cols-[repeat(auto-fill,minmax(380px,1fr))] gap-4 max-[760px]:grid-cols-1"
    >
      {#each $selectedCollection.collection.videos as video, i (video)}
        <VideoCard bind:video={$selectedCollection.collection.videos[i]} />
      {/each}
    </div>
  {/if}
</div>

<Dialog bind:this={addModal}>
  <!-- Largeur en `vw` (et non en `%` d'un parent dimensionné au contenu) pour que
       la grille 2 colonnes ne fasse pas varier la largeur selon la longueur des
       noms de fichiers ; la hauteur reste adaptative. -->
  <div
    class="max-h-full w-[min(880px,92vw)] overflow-auto rounded-xl border border-border bg-surface-raised shadow-lg"
  >
    <div class="flex items-center justify-between gap-3 px-[22px] pt-[22px]">
      <h3 class="font-display text-[21px] font-bold tracking-[-0.01em]">
        Ajouter des vidéos
      </h3>
      <button
        type="button"
        class="grid h-[34px] w-[34px] cursor-pointer place-items-center rounded-pill bg-surface-sunken text-ink-muted hover:bg-danger-tint hover:text-danger"
        aria-label="Fermer"
        onclick={closeAddVideos}
      >
        <X class="h-4 w-4" />
      </button>
    </div>

    <div class="flex flex-col gap-[14px] px-[22px] py-[18px]">
      <UploadVideos {pickVideo} />

      {#if $collectionUpdateStore.videos.length > 0}
        <div class="grid grid-cols-2 gap-2 max-[620px]:grid-cols-1">
          {#each $collectionUpdateStore.videos as video}
            <VideoInformation {onRemoveFile} {play} {video} />
          {/each}
        </div>
      {/if}
    </div>

    <div class="flex items-center justify-end gap-2.5 px-[22px] pb-[22px] pt-1">
      <span class="mr-auto text-[13px] text-ink-muted">
        {$collectionUpdateStore.videos.length} vidéo{$collectionUpdateStore.videos
          .length !== 1
          ? 's'
          : ''} sélectionnée{$collectionUpdateStore.videos.length !== 1 ? 's' : ''}
      </span>
      <button
        type="button"
        class="inline-flex cursor-pointer items-center gap-2 rounded-md border border-border-strong bg-surface-raised px-[14px] py-[9px] text-sm font-semibold text-ink hover:border-brand hover:bg-brand-tint"
        onclick={closeAddVideos}
      >
        Annuler
      </button>
      <button
        type="button"
        class="inline-flex cursor-pointer items-center gap-2 rounded-md bg-brand px-4 py-2.5 text-sm font-semibold text-on-brand shadow-sm hover:bg-brand-strong disabled:cursor-not-allowed disabled:opacity-50"
        disabled={$collectionUpdateStore.videos.length === 0}
        onclick={addVideosToCollection}
      >
        Ajouter à la collection
      </button>
    </div>
  </div>
</Dialog>

<VideoPlayer bind:this={videoPlayer} />
