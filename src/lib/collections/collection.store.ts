import { writable } from 'svelte/store';
import { type Collection } from './collection';
import {
  type Style,
  toVideo,
  type Video,
  type VideoAddedToCollection,
} from './video';

export type VideoFromCollection = {
  path: string;
  name: string;
  artist: string;
  song: string;
  style: Style[];
  tags: string[];
  thumbnail?: string;
  size_bytes?: number;
};

export type CollectionCreated = {
  collection_id: string;
  title: string;
  videos: VideoFromCollection[];
};

const { subscribe, set, update } = writable<Collection>();

export const toCollection = (collectionCreated: CollectionCreated): Collection => ({
  id: collectionCreated.collection_id,
  title: collectionCreated.title,
  videos: collectionCreated.videos.map(toVideo),
});
const initialize = (collection: CollectionCreated) => {
  set(toCollection(collection));
};
export const addVideo = (video: VideoAddedToCollection) => {
  update((collection) => ({
    ...collection,
    videos: [...collection.videos, toVideo(video)],
  }));
};

type ArrayVideoField = 'tags' | 'style';
const isArrayVideoField = (field: keyof Video): field is ArrayVideoField =>
  field === 'tags' || field === 'style';

export const updateVideo = async <K extends keyof Video>(
  video: Video,
  field: K,
  value: Video[K],
  eventEmitter: (video: Video) => Promise<void>
): Promise<void> => {
  const nextValueForEmitter: Video[K] = isArrayVideoField(field)
    ? ([
        ...(video[field] as unknown as Array<unknown>),
        ...((Array.isArray(value) ? value : [value]) as Array<unknown>).filter(
          (x) => !(video[field] as unknown as Array<unknown>).includes(x)
        ),
      ] as unknown as Video[K])
    : value;

  await eventEmitter({ ...video, [field]: nextValueForEmitter });
  update((collection) => {
    return {
      ...collection,
      videos: collection.videos.map((v) => {
        if (v.path !== video.path) return v;

        if (isArrayVideoField(field)) {
          const current = v[field];
          const incoming = (
            Array.isArray(value) ? value : [value]
          ) as typeof current;
          const merged = [
            ...current,
            ...incoming.filter((x) => !(current as string[]).includes(x as string)),
          ] as Video[K];
          return { ...v, [field]: merged };
        }

        return { ...v, [field]: value };
      }),
    };
  });
};

const removeTag = async (
  video: Video,
  tag: string,
  eventEmitter: (video: Video) => Promise<void>
) => {
  await eventEmitter({ ...video, tags: video.tags.filter((t) => t !== tag) });
  update((collection) => {
    return {
      ...collection,
      videos: collection.videos.map((v) => {
        if (v.path !== video.path) return v;
        return { ...v, tags: v.tags.filter((t) => t !== tag) };
      }),
    };
  });
};

export const collectionStore = {
  subscribe,
  initialize,
  addVideo,
  updateVideo,
  removeTag,
};
