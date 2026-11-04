import { writable } from 'svelte/store';
import type { HumanReadable } from '../components/types';
import { toVideoData, type VideoDataRetrievedDTO } from './video.tauri';

type VideoSize = HumanReadable & {
  bytes: number;
};

type VideoDuration = HumanReadable & {
  seconds: number;
};

export type VideoData = {
  path: string;
  thumbnail: string;
  size: VideoSize;
  duration: VideoDuration;
  name: string;
};

type CollectionUpdate = {
  id: string;
  videos: VideoData[];
};

const { subscribe, set, update } = writable<CollectionUpdate>();
set({ id: '', videos: [] });

const addVideo = (video: VideoDataRetrievedDTO) => {
  update((collection) => {
    if (collection.videos.some((v) => v.path === video.path)) return collection;
    return { ...collection, videos: [...collection.videos, toVideoData(video)] };
  });
};

const removeVideo = (video: VideoData) => {
  update((collection) => {
    return {
      ...collection,
      videos: collection.videos.filter((v) => v.path !== video.path),
    };
  });
};

const reset = () => set({ id: '', videos: [] });

export const collectionUpdateStore = {
  subscribe,
  addVideo,
  removeVideo,
  reset,
  initialize: set,
};
