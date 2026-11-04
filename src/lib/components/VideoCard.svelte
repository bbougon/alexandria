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
    videoPlayer?.play({
      name: video.name,
      artist: video.artist,
      song: video.song,
      thumbnail: video.thumbnail,
      play: () => video.play(),
    });
  };
</script>

<div>
  <Card class="p-4 bg-white hover:shadow-md transition-shadow">
    <div class="flex gap-4">
      <Thumbnail
        class="h-24 w-40 flex-shrink-0"
        thumbnail={video.thumbnail}
        duration={video.duration.toHumanReadable()}
        styleName={video.style[0]}
        onplay={play}
      />

      <div class="flex-1 space-y-3">
        <div>
          <label class="text-xs text-muted-foreground block mb-1">File Name</label>
          <EditableField
            label="Name"
            bind:value={videoName}
            onblur={onUpdateVideoName}
          />
        </div>

        <div class="grid grid-cols-2 gap-4">
          <div>
            <label class="text-xs text-muted-foreground block mb-1">Artist</label>
            <EditableField
              label="Artist name"
              bind:value={artistName}
              onblur={onUpdateArtistName}
            />
          </div>
          <div>
            <label class="text-xs text-muted-foreground block mb-1">Song</label>
            <EditableField
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
