import { derived, get, writable } from 'svelte/store';
import { type Collection } from './collection';
import { type Video } from './video';
import { collectionsStore } from './collections.store';

const selectionId = writable<string>('');

export const selectedCollectionId = {
  subscribe: selectionId.subscribe,
  initialize: (collectionId: string) => selectionId.set(collectionId),
  is(id: string) {
    return get(selectionId) === id;
  },
};

type SelectedCollection = {
  collection: Collection | undefined;
  updateVideo: <K extends keyof Video>(
    video: Video,
    field: K,
    value: Video[K],
    eventEmitter: (video: Video) => Promise<void>
  ) => Promise<void>;
  addVideo: (video: Video) => void;
  removeTag: (
    video: Video,
    tag: string,
    eventEmitter: (video: Video) => Promise<void>
  ) => Promise<void>;
};

export const selectedCollection = derived(
  [collectionsStore, selectedCollectionId],
  ([$collections]): SelectedCollection => {
    const collectionFound = $collections.find(({ id }) =>
      selectedCollectionId.is(id)
    );
    return {
      collection: collectionFound,
      updateVideo: (video, field, value, eventEmitter) =>
        collectionsStore.updateVideo(
          collectionFound!,
          video,
          field,
          value,
          eventEmitter
        ),
      addVideo: (video) => collectionsStore.addVideo(collectionFound!, video),
      removeTag: (video, tag, eventEmitter) =>
        collectionsStore.removeTag(collectionFound!, video, tag, eventEmitter),
    };
  }
);
