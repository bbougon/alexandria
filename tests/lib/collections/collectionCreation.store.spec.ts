import { collectionCreationStore } from '../../../src/lib/collections/collectionCreation.store.ts';
import { get } from 'svelte/store';
import { expect } from 'vitest';

describe('Collection creation store', () => {
  it('should add a video', () => {
    collectionCreationStore.addVideo({
      duration_seconds: 63,
      path: 'a/path/VIDEO.mp4',
      size_bytes: 1234,
      thumbnail: 'thumbnail',
    });

    const store = get(collectionCreationStore);

    expect(store.videos.length).toBe(1);
    expect(store.videos[0]).toStrictEqual({
      duration: { seconds: 63, toHumanReadable: expect.any(Function) },
      path: 'a/path/VIDEO.mp4',
      size: { bytes: 1234, toHumanReadable: expect.any(Function) },
      thumbnail: 'thumbnail',
      name: 'VIDEO.mp4',
    });
    expect(store.videos[0].duration.toHumanReadable()).toBe('01:03');
    expect(store.videos[0].size.toHumanReadable()).toBe('1 kB');
  });
});
