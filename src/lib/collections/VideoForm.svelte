<script lang="ts">
  import EditableInput from '../components/EditableInput.svelte';
  import { type Style, STYLES, type Video } from './video';
  import EditableMultiSelectInput from '../components/EditableMultiSelectInput.svelte';
  import EditableTagInput from '../components/EditableTagInput.svelte';
  import Badge from '../components/Badge.svelte';
  import { updateVideo } from './video-operations';
  import { collectionStore } from './collection.store';
  import { get } from 'svelte/store';
  import { invoke } from '@tauri-apps/api/core';

  interface VideoFormProps {
    video?: Video | null;
    collectionId: string;
  }

  let { video }: VideoFormProps = $props();

  let videoEl: HTMLVideoElement | null = null;

  let editName = $state(false);
  let editArtistName = $state(false);
  let editSongName = $state(false);
  let editSongStyle = $state(false);
  let editSongTags = $state(false);

  $effect(() => {
    if (!open) {
      videoEl?.pause();
    }
  });

  const removeTag = async (tag: string) => {
    if (!video) return;
    await collectionStore.removeTag(video, tag, async (video) => {
      const { path, thumbnail, size, name, artist, song, style, tags } = video;
      const collection = get(collectionStore);
      await invoke<void>('update_video', {
        video: {
          collection_id: collection.id,
          video: {
            path,
            thumbnail,
            size_bytes: size,
            name,
            artist,
            song,
            style,
            tags,
          },
        },
      });
    });
  };

  const updateVideoName = async (event: Event) => {
    if (!video) return;
    const value = (event.target as HTMLInputElement).value;
    await updateVideo(video, 'name', value);
  };
  const updateArtistName = async (event: Event) => {
    if (!video) return;
    const value = (event.target as HTMLInputElement).value;
    await updateVideo(video, 'artist', value);
  };
  const updateSongName = async (event: Event) => {
    if (!video) return;
    const value = (event.target as HTMLInputElement).value;
    await updateVideo(video, 'song', value);
  };

  const addVideoTag = async (value: string) => {
    if (!video) return;
    await updateVideo(video, 'tags', [value]);
  };
  const addvideoStyle = async (style: unknown | Style) => {
    if (!video) return;
    await updateVideo(video, 'style', [style as Style]);
  };
</script>

{#snippet editButton(onClick: () => void)}
  <button
    type="button"
    onclick={onClick}
    class="relative ml-4 flex size-8 items-center justify-center rounded-full text-gray-400 hover:bg-gray-100 hover:text-gray-500 focus-visible:outline-2 focus-visible:outline-indigo-600 dark:hover:bg-white/5 dark:hover:text-white dark:focus-visible:outline-indigo-500"
  >
    <span class="absolute -inset-1.5"></span>
    <svg
      viewBox="0 0 20 20"
      fill="currentColor"
      data-slot="icon"
      aria-hidden="true"
      class="size-5"
    >
      <path
        d="m2.695 14.762-1.262 3.155a.5.5 0 0 0 .65.65l3.155-1.262a4 4 0 0 0 1.343-.886L17.5 5.501a2.121 2.121 0 0 0-3-3L3.58 13.419a4 4 0 0 0-.885 1.343Z"
      />
    </svg>
    <span class="sr-only">Edit</span>
  </button>
{/snippet}

<div>
  <div>
    <video
      bind:this={videoEl}
      controls
      class="block aspect-10/7 w-full rounded-lg bg-gray-100 object-cover outline -outline-offset-1 outline-black/5 dark:bg-gray-800 dark:outline-white/10"
      src={video?.play() ?? ''}
      poster={video?.thumbnail ?? ''}
    ></video>
    <div class="mt-4 flex items-start justify-between">
      <div>
        {#if video}
          {#snippet videoName()}
            <h2 class="text-base font-semibold text-gray-900 dark:text-white">
              <span class="sr-only">Details for </span>{video?.name ?? ''}
            </h2>
          {/snippet}
          <EditableInput
            label="Name"
            value={video.name}
            toggle={editName}
            hideOnToggle={videoName}
            onblur={(value) => updateVideoName(value)}
          ></EditableInput>
        {/if}
        <p class="text-sm font-medium text-gray-500 dark:text-gray-400">
          {video ? video.toHumanReadable() : ''}
        </p>
      </div>
      <div>
        {@render editButton(() => (editName = !editName))}
      </div>
    </div>
  </div>
  <div>
    <h3 class="font-medium text-gray-900 dark:text-white">Information</h3>
    <dl
      class="mt-2 divide-y divide-gray-200 border-t border-b border-gray-200 dark:divide-white/10 dark:border-white/10"
    >
      <div class="flex justify-between py-3 text-sm font-medium">
        <dt class="text-gray-500 dark:text-gray-400">Artist</dt>
        <div class="flex items-center space-x-3">
          {#if video}
            {#snippet artistName()}
              <dd class="text-gray-900 dark:text-white">{video.artist}</dd>
            {/snippet}
            {#snippet showEditButton()}
              <dd>
                {@render editButton(() => (editArtistName = !editArtistName))}
              </dd>
            {/snippet}
            <EditableInput
              label="Artist name"
              value={video.artist}
              toggle={editArtistName}
              hideOnToggle={artistName}
              displayChild={showEditButton}
              onblur={(value) => updateArtistName(value)}
            ></EditableInput>
          {/if}
        </div>
      </div>
      <div class="flex justify-between py-3 text-sm font-medium">
        <dt class="text-gray-500 dark:text-gray-400">Song name</dt>
        <div class="flex items-center space-x-3">
          {#if video}
            {#snippet songName()}
              <dd class="text-gray-900 dark:text-white">{video.song}</dd>
            {/snippet}
            {#snippet showEditButton()}
              <dd>
                {@render editButton(() => (editSongName = !editSongName))}
              </dd>
            {/snippet}
            <EditableInput
              label="Song name"
              value={video.song}
              toggle={editSongName}
              hideOnToggle={songName}
              displayChild={showEditButton}
              onblur={(value) => updateSongName(value)}
            ></EditableInput>
          {/if}
        </div>
      </div>
      <div class="flex justify-between py-3 text-sm font-medium">
        <dt class="text-gray-500 dark:text-gray-400">Style</dt>
        <div class="flex items-center space-x-3">
          {#if video}
            {#snippet songStyle()}
              <dd class="text-gray-900 dark:text-white">
                {video.style.join(', ')}
              </dd>
            {/snippet}
            {#snippet showEditButton()}
              <dd>
                {@render editButton(() => (editSongStyle = !editSongStyle))}
              </dd>
            {/snippet}
            <EditableMultiSelectInput
              label="Song style"
              value={video.style}
              toggle={editSongStyle}
              hideOnToggle={songStyle}
              displayChild={showEditButton}
              options={[...STYLES]}
              addOption={(style) => addvideoStyle(style)}
            ></EditableMultiSelectInput>
          {/if}
        </div>
      </div>
    </dl>
  </div>
  <div>
    <div class="flex justify-between py-3 text-sm font-medium">
      <h3 class="font-medium text-gray-900 dark:text-white">Tags</h3>
      {@render editButton(() => (editSongTags = !editSongTags))}
    </div>
    {#if video}
      <div class="flex">
        <div class="mt-2 mb-4 flex gap-2 items-center flex-wrap">
          {#each video.tags as tag}
            <Badge value={tag} onclick={() => removeTag(tag)} />
          {/each}
        </div>
      </div>
      <EditableTagInput
        label="Song tags"
        toggle={editSongTags}
        onAddTag={(value) => addVideoTag(value)}
      ></EditableTagInput>
    {/if}
  </div>
</div>
