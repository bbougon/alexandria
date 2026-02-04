import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import type { Collection } from './collection';
import type { VideoFromCollection } from './collection.store';
import { toVideo } from './video';

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

const { subscribe, set } = writable<Collection[]>();

const collections = await invoke<RetrievedCollection[]>('get_collections');
set(collections.map((c) => toCollection(c)));

export const collectionsStore = { subscribe };
