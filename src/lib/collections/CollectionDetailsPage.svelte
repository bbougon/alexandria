<script lang="ts">
  import { selectedCollection } from './collection.store';
  import Badge from '../components/Badge.svelte';
  import type { Video } from './video';
  import VideoForm from './VideoForm.svelte';

  let selectedVideoPath: string | undefined = $state();
  let listElement: HTMLElement | undefined = $state();
  let selectedElementOffset = $state(0);

  const selectVideo = (video: Video, event: MouseEvent) => {
    selectedVideoPath = video.path;
    const target = event.currentTarget as HTMLElement;
    const listItem = target.closest('li');
    if (listItem && listElement) {
      selectedElementOffset = listItem.offsetTop - 40;
    }
  };

  const activeVideo = $derived(() => {
    const c = $selectedCollection.collection;
    if (!c || !selectedVideoPath) return null;
    return c.videos.find((v) => v.path === selectedVideoPath) ?? null;
  });
</script>

<div class="px-4 sm:px-6 lg:px-8">
  {#if $selectedCollection.collection}
    <div class="border-b border-gray-200 pb-5 dark:border-white/10">
      <div class="mt-2 -ml-2 flex flex-wrap items-baseline">
        <h1 class="text-6xl font-bold tracking-tight text-gray-900 sm:text-4xl">
          {$selectedCollection.collection.title}
        </h1>
        <p class="mt-1 ml-2 truncate text-sm text-gray-500 dark:text-gray-400">
          Number of videos {$selectedCollection.collection.videos.length}
        </p>
      </div>
      <div class="max-w-2xl px-4 pt-16 pb-24 sm:px-6 lg:max-w-7xl lg:px-8">
        <div
          class="mt-12 lg:grid lg:grid-cols-12 lg:items-start lg:gap-x-12 xl:gap-x-16 relative"
        >
          <section
            aria-labelledby="collection-heading"
            class="lg:col-span-7 border-t border-b border-gray-200"
          >
            <h2 id="collection-heading" class="sr-only">Videos in collection</h2>

            <ul bind:this={listElement} role="list" class="divide-y divide-gray-200">
              {#each $selectedCollection.collection.videos as video}
                <li class="flex py-6 sm:py-10">
                  <div
                    class="shrink-0 cursor-pointer"
                    onclick={(e) => selectVideo(video, e)}
                  >
                    <img
                      src={video.thumbnail}
                      alt={video.name}
                      class="size-24 aspect-10/7 rounded-md object-cover sm:size-48"
                    />
                  </div>

                  <div class="ml-4 flex flex-1 flex-col justify-between sm:ml-6">
                    <div
                      class="relative pr-9 sm:grid sm:grid-cols-2 sm:gap-x-6 sm:pr-0"
                    >
                      <div>
                        <div class="flex justify-between">
                          <h3 class="text-sm">
                            <button
                              type="button"
                              onclick={(e) => selectVideo(video, e)}
                              class="cursor-pointer font-medium text-gray-700 hover:text-gray-800 text-left"
                              >{video.name}</button
                            >
                          </h3>
                        </div>
                        <p class="mt-1 text-xl font-medium text-gray-900">
                          {video.artist}
                        </p>
                        <p class="mt-1 text-lg font-medium text-gray-900">
                          {video.song}
                        </p>
                        <div class="mt-1 flex text-sm">
                          {#each video.style as style}
                            <p
                              class="style mr-4 border-r border-gray-200 pr-4 text-gray-500"
                            >
                              {style}
                            </p>
                          {/each}
                        </div>
                      </div>
                    </div>

                    <div class="flex">
                      <div class="mt-2 mb-4 flex gap-2 items-center flex-wrap">
                        {#each video.tags as tag}
                          <Badge value={tag} />
                        {/each}
                      </div>
                    </div>
                  </div>
                </li>
              {/each}
            </ul>
          </section>

          {#if selectedVideoPath}
            <section
              aria-labelledby="video-section"
              class="lg:block mt-16 lg:col-span-5 lg:mt-0 transition-all duration-300"
              style="margin-top: {selectedElementOffset}px"
            >
              <div class="flex justify-end items-center mb-4">
                <button
                  type="button"
                  class="cursor-pointer text-gray-400 hover:text-gray-500"
                  onclick={() => (selectedVideoPath = undefined)}
                >
                  <span class="sr-only">Close</span>
                  <svg
                    class="h-6 w-6"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke-width="1.5"
                    stroke="currentColor"
                  >
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      d="M6 18L18 6M6 6l12 12"
                    />
                  </svg>
                </button>
              </div>
              <VideoForm
                video={activeVideo()}
                collectionId={$selectedCollection.collection.id}
              />
            </section>
          {/if}
        </div>
      </div>
    </div>
  {/if}
</div>

<style lang="scss">
  .style {
    white-space: nowrap;
  }
</style>
