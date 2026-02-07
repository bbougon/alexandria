import { writable } from 'svelte/store';
import type { Video } from '../collections/video';

const { subscribe, update, set } = writable<Video[]>([]);

export const searchResultStore = {
  subscribe,
  addVideo: (video: Video) => update((videos) => [...videos, video]),
  initialize: () => set([]),
};
