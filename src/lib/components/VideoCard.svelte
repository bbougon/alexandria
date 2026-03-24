<script lang="ts">
  import Card from '../components/Card.svelte';
  import { Play, Clock } from '@lucide/svelte';
  import EditableInput from '../components/EditableInput.svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { selectedCollection } from '../collections/collection.store';
  import VideoPlayer from '../components/VideoPlayer.svelte';
  import TagsEdit from '../components/TagsEdit.svelte';
  import MusicStyleEdit from '../components/MusicStyleEdit.svelte';
  import type { Video } from '../collections/video';

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
</script>

<div>
  <Card class="p-4 bg-white hover:shadow-md transition-shadow">
    <div class="flex gap-4">
      <div
        class="flex-shrink-0 w-40 h-24 bg-gradient-to-br from-blue-50 to-indigo-50 rounded-lg flex items-center justify-center overflow-hidden relative group cursor-pointer"
        onclick={() => videoPlayer?.play(video)}
      >
        <video poster={video.thumbnail ?? ''} class="w-full h-full object-cover" />
        <div
          class="absolute inset-0 bg-black/40 flex items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity"
        >
          <Play class="w-12 h-12 text-white fill-white" />
        </div>
        <div
          class="absolute bottom-2 right-2 bg-black/75 text-white text-xs px-2 py-1 rounded flex items-center gap-1"
        >
          <Clock class="w-3 h-3" />
          {video.duration.toHumanReadable()}
        </div>
      </div>

      <div class="flex-1 space-y-3">
        <div>
          <label class="text-xs text-muted-foreground block mb-1">File Name</label>
          <EditableInput
            label="Name"
            bind:value={videoName}
            onblur={onUpdateVideoName}
          />
        </div>

        <div class="grid grid-cols-2 gap-4">
          <div>
            <label class="text-xs text-muted-foreground block mb-1">Artist</label>
            <EditableInput
              label="Artist name"
              bind:value={artistName}
              onblur={onUpdateArtistName}
            />
          </div>
          <div>
            <label class="text-xs text-muted-foreground block mb-1">Song</label>
            <EditableInput
              label="Song name"
              bind:value={songName}
              onblur={onUpdateSongName}
            />
          </div>
        </div>

        <div>
          <label class="text-xs text-muted-foreground block mb-1">Music Style</label>
          <MusicStyleEdit
            bind:style
            onupdatestyle={onUpdateVideoStyle}
            onremovestyle={(style) => onRemoveStyle(style)}
          />
        </div>

        <div>
          <label class="text-xs text-muted-foreground block mb-1">Tags</label>
          <TagsEdit
            bind:tags
            onaddvideotag={onUpdateVideoTag}
            onremovevideotag={(tag) => onRemoveVideoTag(tag)}
          />
        </div>
      </div>
    </div>
  </Card>

  <VideoPlayer bind:this={videoPlayer} />
</div>
