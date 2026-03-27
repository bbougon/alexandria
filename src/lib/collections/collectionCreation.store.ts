import { writable } from 'svelte/store';
import { toVideoData, type VideoDataRetrievedDTO } from './video.tauri';
import type { HumanReadable } from '../components/types';

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

type CollectionCreation = {
  name: string;
  videos: VideoData[];
};

const { subscribe, set, update } = writable<CollectionCreation>();
set({ name: '', videos: [] });

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

export const collectionCreationStore = {
  subscribe,
  addVideo,
  reset: () => set({ name: '', videos: [] }),
  removeVideo,
};
