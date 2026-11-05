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

<div class="info">
  <div class="phead">
    <span></span>
    <div class="pactions">
      <button type="button" class="close" aria-label="Fermer" onclick={onclose}>
        <X />
      </button>
    </div>
  </div>

  <div>
    <h3>{video.name}</h3>
    {#if video.artist || video.song}
      <div class="by">
        {#if video.artist}
          <span style="color:var(--ink);font-weight:600">{video.artist}</span>
        {/if}
        {#if video.artist && video.song}
          <span class="sep">·</span>
        {/if}
        {#if video.song}
          <span>{video.song}</span>
        {/if}
      </div>
    {/if}
  </div>

  {#if video.style?.length}
    <div>
      <p class="ptitle">Styles</p>
      <div class="trow">
        {#each video.style as styleName}
          <StyleTag name={styleName} />
        {/each}
      </div>
    </div>
  {/if}

  {#if video.tags?.length}
    <div>
      <p class="ptitle">Tags</p>
      <div class="trow">
        {#each video.tags as tag}
          <Tag label={tag} />
        {/each}
      </div>
    </div>
  {/if}

  {#if video.duration || video.size}
    <div>
      <p class="ptitle">Détails</p>
      <div class="facts">
        {#if video.duration}
          <div class="fact">
            <div class="k">Durée</div>
            <div class="v">{video.duration.toHumanReadable()}</div>
          </div>
        {/if}
        {#if video.size}
          <div class="fact">
            <div class="k">Taille</div>
            <div class="v">{video.size.toHumanReadable()}</div>
          </div>
        {/if}
      </div>
    </div>
  {/if}

  {#if video.path}
    <div>
      <p class="ptitle">Emplacement</p>
      <div class="path">{video.path}</div>
    </div>
  {/if}
</div>
