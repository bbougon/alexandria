import type { Collection } from '../../../src/lib/collections/collection.ts';
import { collectionStore } from '../../../src/lib/collections/collection.store.ts';
import { get } from 'svelte/store';
import { toVideo, type Video } from '../../../src/lib/collections/video.ts';
import { aVideoFromCollection } from './collectionBuilder.ts';
import { expect } from 'vitest';

describe('Collection store', () => {
  it('can be initialized', () => {
    collectionStore.initialize({
      collection_id: '123',
      title: 'My collection',
      videos: [],
    });

    const collection = get(collectionStore);

    expect(collection).toStrictEqual<Collection>({
      id: '123',
      title: 'My collection',
      videos: [],
    });
  });

  it('can add video to the collection', () => {
    collectionStore.initialize({
      collection_id: '123',
      title: 'My collection',
      videos: [],
    });

    collectionStore.addVideo({
      collection_id: '1234',
      path: 'video.mp4',
      name: 'Video',
      artist: 'Artist',
      song: 'Song',
      style: [],
      tags: [],
    });
    const collection = get(collectionStore);

    expect(collection.videos[0]).toStrictEqual<Video>({
      path: 'video.mp4',
      thumbnail: '',
      size: 0,
      name: 'Video',
      artist: 'Artist',
      song: 'Song',
      style: [],
      tags: [],
      toHumanReadable: expect.any(Function),
      play: expect.any(Function),
    });
  });

  it.each([
    {
      videoFromCollection: aVideoFromCollection().withName('A name').build(),
      field: 'name',
      value: 'New name',
    },
    {
      videoFromCollection: aVideoFromCollection().withArtist('Artist name').build(),
      field: 'artist',
      value: 'Interpol',
    },
    {
      videoFromCollection: aVideoFromCollection().withSong('Song name').build(),
      field: 'song',
      value: 'Rest my chemistry',
    },
  ])(`can update a video $field`, async ({ videoFromCollection, field, value }) => {
    collectionStore.initialize({
      collection_id: '123',
      title: 'My collection',
      videos: [aVideoFromCollection().build(), videoFromCollection],
    });
    const video = toVideo(videoFromCollection);

    await collectionStore.updateVideo(video, field as keyof Video, value, () =>
      Promise.resolve()
    );

    const collection = get(collectionStore);
    const expectedVideo = {
      ...video,
      [field]: value,
    };
    expect(collection.videos[1]).toStrictEqual<Video>({
      ...expectedVideo,
      toHumanReadable: expect.any(Function),
      play: expect.any(Function),
    });
  });

  describe('for tags and style', () => {
    it.each([
      {
        videoFromCollection: aVideoFromCollection()
          .withTags(['rock', 'pop-rock'])
          .build(),
        field: 'tags' as const,
        value: ['intro'],
        expectedValue: ['rock', 'pop-rock', 'intro'],
      },
      {
        videoFromCollection: aVideoFromCollection()
          .withStyles(['Hard Rock', 'Rock'])
          .build(),
        field: 'style' as const,
        value: ['Metal'],
        expectedValue: ['Hard Rock', 'Rock', 'Metal'],
      },
    ])(
      `can add $field in a video $field`,
      async ({ videoFromCollection, field, value, expectedValue }) => {
        collectionStore.initialize({
          collection_id: '123',
          title: 'My collection',
          videos: [aVideoFromCollection().build(), videoFromCollection],
        });
        const video = toVideo(videoFromCollection);

        await collectionStore.updateVideo(video, field, value, () =>
          Promise.resolve()
        );

        const collection = get(collectionStore);
        expect(collection.videos[1][field]).toStrictEqual<string[]>(expectedValue);
      }
    );

    it.each([
      {
        videoFromCollection: aVideoFromCollection()
          .withTags(['rock', 'pop-rock', 'intro'])
          .build(),
        field: 'tags' as const,
        value: ['intro'],
        expectedValue: ['rock', 'pop-rock', 'intro'],
      },
      {
        videoFromCollection: aVideoFromCollection()
          .withStyles(['Hard Rock', 'Rock', 'Metal'])
          .build(),
        field: 'style' as const,
        value: ['Metal'],
        expectedValue: ['Hard Rock', 'Rock', 'Metal'],
      },
    ])(
      `do not add $field in a video $field if already present`,
      async ({ videoFromCollection, field, value, expectedValue }) => {
        collectionStore.initialize({
          collection_id: '123',
          title: 'My collection',
          videos: [aVideoFromCollection().build(), videoFromCollection],
        });
        const video = toVideo(videoFromCollection);

        await collectionStore.updateVideo(video, field as keyof Video, value, () =>
          Promise.resolve()
        );

        const collection = get(collectionStore);
        expect(collection.videos[1][field]).toStrictEqual<string[]>(expectedValue);
      }
    );

    it('can remove a tag from a video', async () => {
      const video = aVideoFromCollection()
        .withTags(['rock', 'punk', 'intro'])
        .build();
      collectionStore.initialize({
        collection_id: '123',
        title: 'My collection',
        videos: [aVideoFromCollection().build(), video],
      });

      await collectionStore.removeTag(toVideo(video), 'punk', () =>
        Promise.resolve()
      );

      const collection = get(collectionStore);
      expect(collection.videos[1].tags).toHaveLength(2);
      expect(collection.videos[1].tags).toStrictEqual(['rock', 'intro']);
    });

    it('emits a video_update when removing a tag from a video', async () => {
      let emittedVideo: Video | undefined = undefined;
      const videoFromCollection = aVideoFromCollection()
        .withTags(['rock', 'punk', 'intro'])
        .build();
      collectionStore.initialize({
        collection_id: '123',
        title: 'My collection',
        videos: [videoFromCollection],
      });
      const video = toVideo(videoFromCollection);

      await collectionStore.removeTag(video, 'punk', async (video) => {
        emittedVideo = video;
        return Promise.resolve();
      });

      expect(emittedVideo).toStrictEqual<Video>({
        path: video.path,
        thumbnail: video.thumbnail,
        size: video.size,
        name: video.name,
        artist: video.artist,
        song: video.song,
        style: video.style,
        tags: ['rock', 'intro'],
        toHumanReadable: expect.any(Function),
        play: expect.any(Function),
      });
    });

    it('the collection is backed up if removing a tag from a video fails', async () => {
      const videoFromCollection = aVideoFromCollection()
        .withTags(['rock', 'punk', 'intro'])
        .build();
      collectionStore.initialize({
        collection_id: '123',
        title: 'My collection',
        videos: [videoFromCollection],
      });
      const video = toVideo(videoFromCollection);

      await collectionStore.removeTag(video, 'punk', async () =>
        Promise.reject("I'm an error")
      );

      const collection = get(collectionStore);
      expect(collection.videos[0]).toStrictEqual<Video>({
        path: video.path,
        thumbnail: video.thumbnail,
        size: video.size,
        name: video.name,
        artist: video.artist,
        song: video.song,
        style: video.style,
        tags: ['rock', 'punk', 'intro'],
        toHumanReadable: expect.any(Function),
        play: expect.any(Function),
      });
    });
  });

  it('emits an event video_update', async () => {
    let emittedVideo: Video | undefined = undefined;
    const videoFromCollection = aVideoFromCollection().withName('Old name').build();
    collectionStore.initialize({
      collection_id: '123',
      title: 'My collection',
      videos: [videoFromCollection],
    });
    const video = toVideo(videoFromCollection);

    await collectionStore.updateVideo(video, 'name', 'New name', async (video) => {
      emittedVideo = video;
      return Promise.resolve();
    });

    expect(emittedVideo).toStrictEqual<Video>({
      path: video.path,
      thumbnail: video.thumbnail,
      size: video.size,
      name: 'New name',
      artist: video.artist,
      song: video.song,
      style: video.style,
      tags: video.tags,
      toHumanReadable: expect.any(Function),
      play: expect.any(Function),
    });
  });

  it('the collection is backed up if an error occurres during emission', async () => {
    const videoFromCollection = aVideoFromCollection().withName('Old name').build();
    collectionStore.initialize({
      collection_id: '123',
      title: 'My collection',
      videos: [videoFromCollection],
    });
    const video = toVideo(videoFromCollection);

    await collectionStore.updateVideo(video, 'name', 'New name', async () => {
      return Promise.reject("I'm an error");
    });

    const backedUpCollection = get(collectionStore);
    expect(backedUpCollection.videos[0]).toStrictEqual<Video>({
      path: video.path,
      thumbnail: video.thumbnail,
      size: video.size,
      name: video.name,
      artist: video.artist,
      song: video.song,
      style: video.style,
      tags: video.tags,
      toHumanReadable: expect.any(Function),
      play: expect.any(Function),
    });
  });
});
