<script lang="ts">
  import { X } from '@lucide/svelte';
  import type { PlayableVideo } from '../collections/video';
  import StyleTag from './StyleTag.svelte';
  import Tag from './Tag.svelte';

  interface Props {
    video: PlayableVideo;
    onclose: () => void;
  }

  let { video, onclose }: Props = $props();
</script>

<div class="flex min-w-0 flex-col gap-4 p-[22px]">
  <div class="flex items-center justify-between gap-2">
    <span></span>
    <div class="flex gap-1">
      <button
        type="button"
        class="grid h-[34px] w-[34px] cursor-pointer place-items-center rounded-pill bg-surface-sunken text-ink-muted hover:bg-danger-tint hover:text-danger"
        aria-label="Fermer"
        onclick={onclose}
      >
        <X class="h-4 w-4" />
      </button>
    </div>
  </div>

  <div>
    <h3
      class="mt-0.5 font-display text-[22px] font-bold leading-[1.15] tracking-[-0.01em] text-ink"
    >
      {video.name}
    </h3>
    {#if video.artist || video.song}
      <div
        class="mt-1.5 flex flex-wrap items-baseline gap-x-1 gap-y-0.5 text-sm text-ink-muted"
      >
        {#if video.artist}
          <span class="font-semibold text-ink">{video.artist}</span>
        {/if}
        {#if video.artist && video.song}
          <span class="text-ink-subtle">·</span>
        {/if}
        {#if video.song}
          <span>{video.song}</span>
        {/if}
      </div>
    {/if}
  </div>

  {#if video.style?.length}
    <div>
      <p
        class="mb-1.5 text-[11px] font-semibold uppercase tracking-[0.08em] text-ink-muted"
      >
        Styles
      </p>
      <div class="flex flex-wrap items-center gap-1.5">
        {#each video.style as styleName}
          <StyleTag name={styleName} />
        {/each}
      </div>
    </div>
  {/if}

  {#if video.tags?.length}
    <div>
      <p
        class="mb-1.5 text-[11px] font-semibold uppercase tracking-[0.08em] text-ink-muted"
      >
        Tags
      </p>
      <div class="flex flex-wrap items-center gap-1.5">
        {#each video.tags as tag}
          <Tag label={tag} />
        {/each}
      </div>
    </div>
  {/if}

  {#if video.duration || video.size}
    <div>
      <p
        class="mb-1.5 text-[11px] font-semibold uppercase tracking-[0.08em] text-ink-muted"
      >
        Détails
      </p>
      <div class="grid grid-cols-2 gap-2.5">
        {#if video.duration}
          <div class="rounded-md border border-border bg-surface-sunken px-3 py-2.5">
            <div class="text-[11px] text-ink-muted">Durée</div>
            <div class="mt-0.5 font-mono text-sm text-ink">
              {video.duration.toHumanReadable()}
            </div>
          </div>
        {/if}
        {#if video.size}
          <div class="rounded-md border border-border bg-surface-sunken px-3 py-2.5">
            <div class="text-[11px] text-ink-muted">Taille</div>
            <div class="mt-0.5 font-mono text-sm text-ink">
              {video.size.toHumanReadable()}
            </div>
          </div>
        {/if}
      </div>
    </div>
  {/if}

  {#if video.path}
    <div>
      <p
        class="mb-1.5 text-[11px] font-semibold uppercase tracking-[0.08em] text-ink-muted"
      >
        Emplacement
      </p>
      <div
        class="break-all rounded-sm border border-border bg-surface-sunken px-2.5 py-2 font-mono text-[11px] text-ink-muted"
      >
        {video.path}
      </div>
    </div>
  {/if}
</div>
