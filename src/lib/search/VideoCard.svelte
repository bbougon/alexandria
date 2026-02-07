<script lang="ts">
  import Badge from '../components/Badge.svelte';
  import type { Video } from '../collections/video';

  interface VideoCardProps extends HTMLButtonElement {
    video?: Video | null;
    onVideo?: (video: Video) => void;
  }

  let { video, onVideo }: VideoCardProps = $props();
</script>

{#if video}
  <div
    class="bg-card text-card-foreground flex flex-col gap-6 rounded-xl border overflow-hidden hover:shadow-lg transition-shadow"
  >
    <button
      command="show-modal"
      commandfor="dialog"
      class="relative aspect-video bg-gray-100 cursor-pointer group"
      onclick={() => onVideo?.(video)}
    >
      <img
        src={video.thumbnail}
        alt={video.name}
        class="w-full h-full object-cover"
      />
      <div
        class="absolute inset-0 bg-black/40 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center"
      >
        <div class="bg-white rounded-full p-4">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            class="lucide lucide-circle-play-icon lucide-circle-play"
            ><path
              d="M9 9.003a1 1 0 0 1 1.517-.859l4.997 2.997a1 1 0 0 1 0 1.718l-4.997 2.997A1 1 0 0 1 9 14.996z"
            /><circle cx="12" cy="12" r="10" /></svg
          >
        </div>
      </div>
    </button>

    <div class="p-4 space-y-2">
      <div class="text-sm text-muted-foreground">{video.name}</div>
      <h3 class="font-semibold line-clamp-2">{video.song}</h3>
      <div class="text-sm text-muted-foreground">by {video.artist}</div>

      <div class="flex flex-wrap gap-2 pt-2">
        {#each video.tags as tag}
          <Badge value={tag} />
        {/each}
      </div>
    </div>
  </div>
{/if}
