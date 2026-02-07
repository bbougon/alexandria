<script lang="ts">
  import type { Video } from '../collections/video';
  import Badge from './Badge.svelte';

  interface VideoDialogProps {
    video?: Video | null;
  }

  let { video }: VideoDialogProps = $props();
</script>

<el-dialog>
  <dialog
    id="dialog"
    aria-labelledby="dialog-title"
    class="fixed inset-0 size-auto max-h-none max-w-none overflow-y-auto bg-transparent backdrop:bg-transparent"
  >
    <el-dialog-backdrop
      class="fixed inset-0 bg-gray-500/75 transition-opacity data-closed:opacity-0 data-enter:duration-300 data-enter:ease-out data-leave:duration-200 data-leave:ease-in dark:bg-gray-900/50"
    ></el-dialog-backdrop>
    <div
      tabindex="0"
      class="flex min-h-full items-end justify-center p-4 text-center focus:outline-none sm:items-center sm:p-0"
    >
      <el-dialog-panel
        class="relative transform overflow-hidden rounded-lg bg-white px-4 pt-5 pb-4 text-left shadow-xl transition-all data-closed:translate-y-4 data-closed:opacity-0 data-enter:duration-300 data-enter:ease-out data-leave:duration-200 data-leave:ease-in sm:my-8 sm:w-full sm:max-w-4xl sm:p-6 data-closed:sm:translate-y-0 data-closed:sm:scale-95 dark:bg-gray-800 dark:outline dark:-outline-offset-1 dark:outline-white/10"
      >
        {#if video}
          <div class="space-y-4">
            <div class="relative aspect-video bg-black rounded-lg overflow-hidden">
              <video
                controls
                class="block aspect-10/7 w-full rounded-lg bg-gray-100 object-cover outline -outline-offset-1 outline-black/5 dark:bg-gray-800 dark:outline-white/10"
                poster={video.thumbnail}
                src={video.play()}
              >
              </video>
            </div>

            <div class="space-y-3">
              <div>
                <div class="text-sm text-muted-foreground">Lesson Name</div>
                <div class="font-medium">{video.name}</div>
              </div>

              <div>
                <div class="text-sm text-muted-foreground">Artist</div>
                <div class="font-medium">{video.artist}</div>
              </div>

              <div>
                <div class="text-sm text-muted-foreground mb-2">Tags</div>
                <div class="flex flex-wrap gap-2">
                  {#each video.tags as tag}
                    <Badge value={tag} />
                  {/each}
                </div>
              </div>
            </div>
          </div>
        {/if}
      </el-dialog-panel>
    </div>
  </dialog>
</el-dialog>
