<script lang="ts">
  import Card from '../components/Card.svelte';
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

<div>
  <Card class="bg-surface-raised p-4 transition-shadow hover:shadow-md">
    <div class="flex gap-4">
      <Thumbnail
        class="h-28 w-48 flex-shrink-0"
        thumbnail={video.thumbnail}
        duration={video.duration.toHumanReadable()}
        styleName={video.style[0]}
        onplay={play}
      />

      <div class="min-w-0 flex-1 space-y-2">
        <EditableField
          label="Video title"
          bind:value={videoName}
          onblur={onUpdateVideoName}
          class="font-display text-lg font-semibold"
        />

        <div class="flex items-center gap-1 text-sm text-ink-muted">
          <EditableField
            label="Artist"
            bind:value={artistName}
            onblur={onUpdateArtistName}
            class="flex-1 text-sm"
          />
          <span class="text-ink-subtle">·</span>
          <EditableField
            label="Song"
            bind:value={songName}
            onblur={onUpdateSongName}
            class="flex-1 text-sm"
          />
        </div>

        <div class="flex items-start gap-2">
          <span class="mt-1.5 w-12 shrink-0 text-xs font-medium text-ink-subtle">
            Styles
          </span>
          <MusicStyleEdit
            bind:style
            onupdatestyle={onUpdateVideoStyle}
            onremovestyle={(style) => onRemoveStyle(style)}
          />
        </div>

        <div class="flex items-start gap-2">
          <span class="mt-1.5 w-12 shrink-0 text-xs font-medium text-ink-subtle">
            Tags
          </span>
          <TagsEdit
            bind:tags
            onaddvideotag={onUpdateVideoTag}
            onremovevideotag={(tag) => onRemoveVideoTag(tag)}
          />
        </div>

        <div class="pt-1 font-mono text-xs text-ink-subtle">
          {video.size.toHumanReadable()}
        </div>
      </div>
    </div>
  </Card>

  <VideoPlayer bind:this={videoPlayer} />
</div>
