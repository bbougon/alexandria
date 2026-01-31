<script lang="ts">
  import { open } from '@tauri-apps/plugin-dialog';
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import { videos } from './videos.svelte';
  import { type Style, toVideo, type Video } from './video';
  import VideoForm from './VideoForm.svelte';

  type VideoAddedToCollection = {
    collection_id: string;
    path: string;
    name: string;
    artist: string;
    song: string;
    style: Style[];
    tags: string[];
    thumbnail?: string;
    size_bytes?: number;
    error?: string | null;
  };

  let selectedPath: string[] = $state([]);
  let drawerOpen = $state(false);
  let activeVideo: Video | null = $state(null);

  const openDetails = (v: Video) => {
    activeVideo = v;
    drawerOpen = true;
  };

  $effect(() => {
    console.log(`VIDEO en édition : ${JSON.stringify(activeVideo)}`);
  });

  const pickVideo = async () => {
    const result = await open({
      multiple: true,
      directory: false,
      filters: [{ name: 'Videos', extensions: ['mp4', 'mov', 'mkv', 'webm'] }],
    });

    if (result !== null) {
      if (Array.isArray(result)) {
        selectedPath = result;
      } else {
        selectedPath = [result];
      }
    }
    await createCollection();
  };

  const createCollection = async () => {
    if (selectedPath.length === 0) return;
    const unlisten = await listen<VideoAddedToCollection>('video:added', (e) => {
      const p = e.payload;

      if (p.thumbnail) {
        videos.push(
          toVideo({
            videoPath: p.path,
            thumbnail: p.thumbnail,
            size: p.size_bytes || 0,
          })
        );
      }
    });
    await invoke<string[]>('create_collection', { paths: selectedPath });
    unlisten();
  };
</script>

<div class="px-4 sm:px-6 lg:px-8">
  <button
    type="button"
    onclick={pickVideo}
    class="inline-flex items-center gap-x-2 rounded-md bg-gray-600 px-3.5 py-2.5 text-sm font-semibold text-white shadow-xs hover:bg-gray-500 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-gray-600 dark:bg-gray-500 dark:shadow-none dark:hover:bg-gray-400 dark:focus-visible:outline-gray-500"
  >
    <svg
      xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 24 24"
      fill="currentColor"
      data-slot="icon"
      aria-hidden="true"
      class="-ml-0.5 size-5"
    >
      <path
        d="M3 3.9934C3 3.44476 3.44495 3 3.9934 3H20.0066C20.5552 3 21 3.44495 21 3.9934V20.0066C21 20.5552 20.5551 21 20.0066 21H3.9934C3.44476 21 3 20.5551 3 20.0066V3.9934ZM5 5V19H19V5H5ZM10.6219 8.41459L15.5008 11.6672C15.6846 11.7897 15.7343 12.0381 15.6117 12.2219C15.5824 12.2658 15.5447 12.3035 15.5008 12.3328L10.6219 15.5854C10.4381 15.708 10.1897 15.6583 10.0672 15.4745C10.0234 15.4088 10 15.3316 10 15.2526V8.74741C10 8.52649 10.1791 8.34741 10.4 8.34741C10.479 8.34741 10.5562 8.37078 10.6219 8.41459Z"
      ></path>
    </svg>
    Add videos
  </button>

  <div class="mt-4">
    <ul
      role="list"
      class="grid grid-cols-2 gap-x-4 gap-y-8 sm:grid-cols-3 sm:gap-x-6 lg:grid-cols-4 xl:gap-x-8"
    >
      {#each videos as video}
        <li class="relative">
          <div
            class="group overflow-hidden rounded-lg bg-gray-100 focus-within:outline-2 focus-within:outline-offset-2 focus-within:outline-indigo-600 dark:bg-gray-800 dark:focus-within:outline-indigo-500"
          >
            <img
              src={video.thumbnail}
              alt=""
              class="pointer-events-none aspect-10/7 rounded-lg object-cover outline -outline-offset-1 outline-black/5 group-hover:opacity-75 dark:outline-white/10"
            />
            <button
              type="button"
              class="absolute inset-0 focus:outline-hidden"
              onclick={() => openDetails(video)}
            >
              <span class="sr-only">View details for IMG_4985.HEIC</span>
            </button>
          </div>
          <p
            class="pointer-events-none mt-2 block truncate text-sm font-medium text-gray-900 dark:text-white"
          >
            {video.name}
          </p>
          <p
            class="pointer-events-none block text-sm font-medium text-gray-500 dark:text-gray-400"
          >
            {video.toHumanReadable()}
            MB
          </p>
        </li>
      {/each}
    </ul>
    <VideoForm
      open={drawerOpen}
      video={activeVideo}
      close={() => (drawerOpen = false)}
    />
  </div>
</div>
