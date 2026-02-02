import { get } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import { collectionStore } from './collection.store';
import type { Video } from './video';

export async function updateVideo<K extends keyof Video>(
  video: Video,
  field: K,
  value: Video[K]
): Promise<void> {
  await collectionStore.updateVideo(video, field, value, async (video) => {
    const { path, thumbnail, size, name, artist, song, style, tags } = video;
    const collection = get(collectionStore);
    await invoke<void>('update_video', {
      video: {
        collection_id: collection.id,
        video: {
          path,
          thumbnail,
          size_bytes: size,
          name,
          artist,
          song,
          style,
          tags,
        },
      },
    });
  });
}
