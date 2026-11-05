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

<div class="content">
  {#if $selectedCollection.collection}
    <div class="collhead">
      <div>
        <h2>{$selectedCollection.collection.title}</h2>
        <p class="sub">
          {$selectedCollection.collection.videos.length} vidéo{$selectedCollection
            .collection.videos.length !== 1
            ? 's'
            : ''}
        </p>
      </div>
      <div class="spacer"></div>
      <button type="button" class="ghostbtn" onclick={openAddVideos}>
        <Plus />
        Ajouter des vidéos
      </button>
    </div>

    <div class="grid">
      {#each $selectedCollection.collection.videos as video, i (video)}
        <VideoCard bind:video={$selectedCollection.collection.videos[i]} />
      {/each}
    </div>
  {/if}
</div>

<Dialog bind:this={addModal}>
  <div class="modal modal--wide">
    <div class="modal-h">
      <h3>Ajouter des vidéos</h3>
      <button
        type="button"
        class="close"
        aria-label="Fermer"
        onclick={closeAddVideos}
      >
        <X />
      </button>
    </div>

    <div class="modal-b">
      <UploadVideos {pickVideo} />

      {#if $collectionUpdateStore.videos.length > 0}
        <div class="implist">
          {#each $collectionUpdateStore.videos as video}
            <VideoInformation {onRemoveFile} {play} {video} />
          {/each}
        </div>
      {/if}
    </div>

    <div class="modal-f">
      <span class="count">
        {$collectionUpdateStore.videos.length} vidéo{$collectionUpdateStore.videos
          .length !== 1
          ? 's'
          : ''} sélectionnée{$collectionUpdateStore.videos.length !== 1 ? 's' : ''}
      </span>
      <button type="button" class="ghostbtn" onclick={closeAddVideos}>
        Annuler
      </button>
      <button
        type="button"
        class="pbtn"
        disabled={$collectionUpdateStore.videos.length === 0}
        onclick={addVideosToCollection}
      >
        Ajouter à la collection
      </button>
    </div>
  </div>
</Dialog>

<VideoPlayer bind:this={videoPlayer} />

<style>
  /* Modale d'ajout : plus large, hauteur stable (elle ne grandit plus avec le
     contenu) — en-tête et pied figés, seul le corps défile — et liste des vidéos
     sélectionnées sur 2 colonnes (sans toucher à la liste de l'écran de création). */
  .modal--wide {
    width: min(880px, 100%);
    height: min(680px, 85vh);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }
  .modal--wide :global(.modal-b) {
    flex: 1;
    overflow-y: auto;
  }
  .modal--wide :global(.implist) {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }
  @media (max-width: 620px) {
    .modal--wide :global(.implist) {
      grid-template-columns: 1fr;
    }
  }
</style>
