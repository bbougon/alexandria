import type { VideoFromCollection } from './collection.store';
import type { Collection } from './collection';
import { toVideo } from './video';
import { invoke } from '@tauri-apps/api/core';

type RetrievedCollection = {
  id: string;
  title: string;
  videos: VideoFromCollection[];
};
const toCollection = (collection: RetrievedCollection): Collection => {
  return {
    id: collection.id,
    title: collection.title,
    videos: collection.videos.map(toVideo),
  };
};

async function getCollections() {
  const collections = await invoke<RetrievedCollection[]>('get_collections');
  return collections.map((c) => toCollection(c));
}

export const allCollections = await getCollections();
