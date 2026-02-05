import type { Collection } from './collection';
import { invoke } from '@tauri-apps/api/core';
import { toVideo } from './video.tauri';
import type { VideoFromCollection } from './collection.tauri';

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

const getCollections = async () => {
  const collections = await invoke<RetrievedCollection[]>('get_collections');
  return collections.map((c) => toCollection(c));
};

export const allCollections = await getCollections();
