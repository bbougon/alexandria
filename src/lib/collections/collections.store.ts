import { get, writable } from 'svelte/store';
import type { Collection } from './collection';
import { type Video } from './video';

const { subscribe, set, update } = writable<Collection[]>();

const addVideo = (collection: Collection, video: Video) => {
  update((collections) => {
    const collectionFound = collections.find(
      (c: Collection) => c.id === collection.id
    );
    if (!collectionFound) return collections;
    collections.splice(collections.indexOf(collectionFound), 1);
    collectionFound.videos.push(video);
    return [...collections, collectionFound];
  });
};

type ArrayVideoField = 'tags' | 'style';
const isArrayVideoField = (field: keyof Video): field is ArrayVideoField =>
  field === 'tags' || field === 'style';

const updateVideo = async <K extends keyof Video>(
  collection: Collection,
  video: Video,
  field: K,
  value: Video[K],
  eventEmitter: (video: Video) => Promise<void>
): Promise<void> => {
  const backedUpCollections = get(collectionsStore);
  try {
    const nextValueForEmitter: Video[K] = isArrayVideoField(field)
      ? ([
          ...(video[field] as unknown as Array<unknown>),
          ...((Array.isArray(value) ? value : [value]) as Array<unknown>).filter(
            (x) => !(video[field] as unknown as Array<unknown>).includes(x)
          ),
        ] as unknown as Video[K])
      : value;

    await eventEmitter({ ...video, [field]: nextValueForEmitter });

    update((collections) => {
      const collectionFound = collections.find(
        (c: Collection) => c.id === collection.id
      );
      if (!collectionFound) return collections;
      collections.splice(collections.indexOf(collectionFound), 1);
      collectionFound.videos = collectionFound.videos.map((v) => {
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
      });

      return [...collections, collectionFound];
    });
  } catch {
    set(backedUpCollections);
  }
};

const removeTag = async (
  collection: Collection,
  video: Video,
  tag: string,
  eventEmitter: (video: Video) => Promise<void>
) => {
  const backedUpCollections = get(collectionsStore);
  try {
    await eventEmitter({ ...video, tags: video.tags.filter((t) => t !== tag) });
    update((collections) => {
      const collectionFound = collections.find(
        (c: Collection) => c.id === collection.id
      );
      if (!collectionFound) return collections;
      collections.splice(collections.indexOf(collectionFound), 1);
      collectionFound.videos = collectionFound.videos.map((v) => {
        if (v.path !== video.path) return v;
        return { ...v, tags: v.tags.filter((t) => t !== tag) };
      });
      return [...collections, collectionFound];
    });
  } catch {
    set(backedUpCollections);
  }
};

const addCollection = (collection: Collection) => {
  update((collections) => [...collections, collection]);
};

export const collectionsStore = {
  subscribe,
  initialize: set,
  addCollection,
  updateVideo,
  addVideo,
  removeTag,
};
