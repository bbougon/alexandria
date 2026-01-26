<script lang="ts">
  import EditableInput from '../components/EditableInput.svelte';
  import { STYLES, type Video } from './video';
  import EditableMultiSelectInput from '../components/EditableMultiSelectInput.svelte';
  import EditableTagInput from '../components/EditableTagInput.svelte';
  import Badge from '../components/Badge.svelte';

  interface VideoFormProps {
    close: () => void;
    open: boolean;
    video?: Video | null;
  }

  let { close, open, video }: VideoFormProps = $props();

  let drawerEl: HTMLDialogElement | null = null;
  let videoEl: HTMLVideoElement | null = null;

  let editName = $state(false);
  let editArtistName = $state(false);
  let editSongName = $state(false);
  let editSongStyle = $state(false);
  let editSongTags = $state(false);

  $effect(() => {
    if (drawerEl) {
      if (open && !drawerEl.open) drawerEl.showModal();
      if (!open && drawerEl.open) drawerEl.close();
    }
  });

  $effect(() => {
    if (!open) {
      videoEl?.pause();
    }
  });

  const requestClose = () => {
    open = false;
    close();
  };

  const removeTag = (tag: string) => {
    console.log(`TAG : ${tag}`);
    video?.tags.splice(video.tags.indexOf(tag), 1);
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

<el-dialog>
  <dialog
    bind:this={drawerEl}
    id="drawer"
    class="fixed inset-0 size-auto max-h-none max-w-none overflow-hidden bg-transparent not-open:hidden backdrop:bg-transparent"
    onclose={requestClose}
  >
    <el-dialog-backdrop
      class="absolute inset-0 bg-gray-500/75 transition-opacity duration-500 ease-in-out data-closed:opacity-0 dark:bg-gray-900/50"
      onclick={requestClose}
    ></el-dialog-backdrop>

    <div class="absolute inset-0 pl-10 focus:outline-none sm:pl-16">
      <el-dialog-panel
        class="group/dialog-panel relative ml-auto block size-full sm:max-w-sm lg:max-w-2xl transform transition duration-500 ease-in-out data-closed:translate-x-full sm:duration-700"
      >
        <div
          class="absolute top-0 left-0 -ml-8 flex pt-4 pr-2 duration-500 ease-in-out group-data-closed/dialog-panel:opacity-0 sm:-ml-10 sm:pr-4"
        >
          <button
            type="button"
            class="relative rounded-md text-gray-300 hover:text-white focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600 dark:text-gray-400 dark:hover:text-white dark:focus-visible:outline-indigo-500"
            onclick={requestClose}
          >
            <span class="absolute -inset-2.5"></span>
            <span class="sr-only">Close panel</span>
            <svg
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="1.5"
              data-slot="icon"
              aria-hidden="true"
              class="size-6"
            >
              <path
                d="M6 18 18 6M6 6l12 12"
                stroke-linecap="round"
                stroke-linejoin="round"
              />
            </svg>
          </button>
        </div>

        <!-- Slide-over panel, show/hide based on slide-over state. -->
        <div
          class="relative h-full overflow-y-auto bg-white p-8 dark:bg-gray-800 dark:after:absolute dark:after:inset-y-0 dark:after:left-0 dark:after:w-px dark:after:bg-white/10"
        >
          <div class="space-y-6 pb-16">
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
                      <h2
                        class="text-base font-semibold text-gray-900 dark:text-white"
                      >
                        <span class="sr-only">Details for </span>{video?.name ?? ''}
                      </h2>
                    {/snippet}
                    <EditableInput
                      label="Name"
                      bind:value={video.name}
                      toggle={editName}
                      hideOnToggle={videoName}
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
                          {@render editButton(
                            () => (editArtistName = !editArtistName)
                          )}
                        </dd>
                      {/snippet}
                      <EditableInput
                        label="Artist name"
                        bind:value={video.artist}
                        toggle={editArtistName}
                        hideOnToggle={artistName}
                        displayChild={showEditButton}
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
                        bind:value={video.song}
                        toggle={editSongName}
                        hideOnToggle={songName}
                        displayChild={showEditButton}
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
                          {@render editButton(
                            () => (editSongStyle = !editSongStyle)
                          )}
                        </dd>
                      {/snippet}
                      <EditableMultiSelectInput
                        label="Song style"
                        bind:value={video.style}
                        toggle={editSongStyle}
                        hideOnToggle={songStyle}
                        displayChild={showEditButton}
                        options={[...STYLES]}
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
                      <Badge value={tag} remove={() => removeTag(tag)} />
                    {/each}
                  </div>
                </div>
                <EditableTagInput
                  label="Song tags"
                  bind:value={video.tags}
                  toggle={editSongTags}
                ></EditableTagInput>
              {/if}
            </div>

            <div class="flex">
              <button
                type="button"
                class="flex-1 rounded-md bg-gray-600 px-3 py-2 text-sm font-semibold text-white shadow-xs hover:bg-gray-500 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-gray-600 dark:bg-gray-500 dark:shadow-none dark:hover:bg-gray-400 dark:focus-visible:outline-indigo-gray"
              >
                Save
              </button>
            </div>
          </div>
        </div>
      </el-dialog-panel>
    </div>
  </dialog>
</el-dialog>
