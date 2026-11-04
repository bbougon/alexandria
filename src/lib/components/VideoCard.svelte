<script lang="ts">
  import Thumbnail from '../components/Thumbnail.svelte';
  import EditableField from '../components/EditableField.svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { selectedCollection } from '../collections/collection.store';
  import VideoPlayer from '../components/VideoPlayer.svelte';
  import TagsEdit from '../components/TagsEdit.svelte';
  import MusicStyleEdit from '../components/MusicStyleEdit.svelte';
  import type { Style, Video } from '../collections/video';

  interface Props {
    video: Video;
  }

  let { video = $bindable() }: Props = $props();

  let videoName = $state(video.name);
  let artistName = $state(video.artist);
  let songName = $state(video.song);
  let tags = $state(video.tags);
  let style = $state(video.style);

  $effect(() => {
    videoName = video.name;
    artistName = video.artist;
    songName = video.song;
    tags = video.tags;
    style = video.style;
  });

  let videoPlayer: VideoPlayer | undefined = $state();

  const updateVideoInvoker = async (video: Video) => {
    const { path, thumbnail, size, name, artist, song, style, tags, duration } =
      video;
    await invoke<void>('update_video', {
      video: {
        collection_id: $selectedCollection.collection?.id,
        video: {
          path,
          thumbnail,
          size_bytes: size.bytes,
          duration_seconds: duration.seconds,
          name,
          artist,
          song,
          style,
          tags,
        },
      },
    });
  };

  async function updateVideo<K extends keyof Video>(
    field: K,
    value: Video[K]
  ): Promise<void> {
    await $selectedCollection.updateVideo(video, field, value, async (video) => {
      await updateVideoInvoker(video);
    });
  }

  const removeTag = async (tag: string) => {
    if (!video) return;
    await $selectedCollection.removeTag(
      video,
      tag,
      async (video) => await updateVideoInvoker(video)
    );
  };

  const removeStyle = async (style: Style) => {
    if (!video) return;
    await $selectedCollection.removeStyle(video, style, async (video) => {
      await updateVideoInvoker(video);
    });
  };

  const onUpdateVideoName = async () => {
    if (!video) return;
    await updateVideo('name', videoName);
  };
  const onUpdateArtistName = async () => {
    if (!video) return;
    await updateVideo('artist', artistName);
  };
  const onUpdateSongName = async () => {
    if (!video) return;
    await updateVideo('song', songName);
  };

  const onUpdateVideoTag = async () => {
    if (!video) return;
    await updateVideo('tags', tags);
  };

  const onRemoveVideoTag = async (tag: string) => {
    if (!video) return;
    await removeTag(tag);
  };

  const onUpdateVideoStyle = async () => {
    if (!video) return;
    await updateVideo('style', style);
  };

  const onRemoveStyle = async (style: Style) => {
    if (!video) return;
    await removeStyle(style);
  };

  const play = () => {
    videoPlayer?.play(video);
  };
</script>

<div class="h-full">
  <article
    class="flex h-full gap-[18px] rounded-2xl border border-border bg-surface-raised p-3.5 transition hover:-translate-y-0.5 hover:border-border-strong hover:shadow-md"
  >
    <Thumbnail
      class="h-28 w-44 flex-shrink-0"
      thumbnail={video.thumbnail}
      duration={video.duration.toHumanReadable()}
      styleName={video.style[0]}
      onplay={play}
    />

    <div class="flex min-w-0 flex-1 flex-col gap-2">
      <EditableField
        label="Video title"
        bind:value={videoName}
        onblur={onUpdateVideoName}
        class="font-display text-base font-bold leading-snug"
      />

      <div class="flex flex-wrap items-baseline gap-x-1 text-[13px] text-ink-muted">
        <EditableField
          label="Artist"
          bind:value={artistName}
          onblur={onUpdateArtistName}
          class="w-auto min-w-[3ch] flex-none text-[13px] font-semibold text-ink [field-sizing:content]"
        />
        <span class="text-ink-subtle">·</span>
        <EditableField
          label="Song"
          bind:value={songName}
          onblur={onUpdateSongName}
          class="w-auto min-w-[3ch] flex-none text-[13px] [field-sizing:content]"
        />
      </div>

      <MusicStyleEdit
        bind:style
        onupdatestyle={onUpdateVideoStyle}
        onremovestyle={(style) => onRemoveStyle(style)}
      />

      <TagsEdit
        bind:tags
        onaddvideotag={onUpdateVideoTag}
        onremovevideotag={(tag) => onRemoveVideoTag(tag)}
      />

      <div class="mt-auto pt-0.5 font-mono text-[11px] text-ink-muted">
        {video.size.toHumanReadable()}
      </div>
    </div>
  </article>

  <VideoPlayer bind:this={videoPlayer} />
</div>
