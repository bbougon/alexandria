<script lang="ts">
  import type { PlayableVideo } from '../collections/video';
  import StyleTag from './StyleTag.svelte';
  import Tag from './Tag.svelte';

  interface Props {
    video: PlayableVideo;
  }

  let { video }: Props = $props();
</script>

<div class="space-y-4">
  <div>
    <h3 class="font-display text-lg font-semibold text-ink">{video.name}</h3>
    {#if video.artist || video.song}
      <p class="text-sm text-ink-muted">
        {#if video.artist}<span class="font-medium text-ink">{video.artist}</span
          >{/if}
        {#if video.artist && video.song}
          ·
        {/if}
        {video.song}
      </p>
    {/if}
  </div>

  {#if video.style?.length}
    <div>
      <p class="mb-1.5 text-xs font-medium text-ink-subtle">Styles</p>
      <div class="flex flex-wrap gap-2">
        {#each video.style as styleName}
          <StyleTag name={styleName} />
        {/each}
      </div>
    </div>
  {/if}

  {#if video.tags?.length}
    <div>
      <p class="mb-1.5 text-xs font-medium text-ink-subtle">Tags</p>
      <div class="flex flex-wrap gap-2">
        {#each video.tags as tag}
          <Tag label={tag} />
        {/each}
      </div>
    </div>
  {/if}

  {#if video.duration || video.size || video.path}
    <dl class="space-y-2 border-t border-border pt-4 text-sm">
      {#if video.duration}
        <div class="flex justify-between gap-4">
          <dt class="text-ink-subtle">Duration</dt>
          <dd class="font-mono text-ink">{video.duration.toHumanReadable()}</dd>
        </div>
      {/if}
      {#if video.size}
        <div class="flex justify-between gap-4">
          <dt class="text-ink-subtle">Size</dt>
          <dd class="font-mono text-ink">{video.size.toHumanReadable()}</dd>
        </div>
      {/if}
      {#if video.path}
        <div>
          <dt class="text-ink-subtle">Location</dt>
          <dd class="mt-1 break-all font-mono text-xs text-ink-muted">
            {video.path}
          </dd>
        </div>
      {/if}
    </dl>
  {/if}
</div>
