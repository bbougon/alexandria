import {
  selectedCollection,
  selectedCollectionId,
} from '../../../src/lib/collections/collection.store.ts';
import { get } from 'svelte/store';
import { type Video } from '../../../src/lib/collections/video.ts';
import { aCollection, aVideo } from './collectionBuilder.ts';
import { beforeEach, expect } from 'vitest';
import { collectionsStore } from '../../../src/lib/collections/collections.store.ts';

describe('Collection store', () => {
  beforeEach(() => {
    selectedCollectionId.initialize('');
    collectionsStore.initialize([]);
  });

  it('returns the collection corresponding to the selected collection id', () => {
    selectedCollectionId.initialize('2');
    collectionsStore.initialize([
      aCollection().build(),
      aCollection().withId('2').build(),
    ]);

    const { collection } = get(selectedCollection);

    expect(collection).toBeDefined();
    expect(collection.id).toBe('2');
  });

  it.each([
    {
      video: aVideo().withName('A name').build(),
      field: 'name',
      value: 'New name',
    },
    {
      video: aVideo().withArtist('Artist name').build(),
      field: 'artist',
      value: 'Interpol',
    },
    {
      video: aVideo().withSong('Song name').build(),
      field: 'song',
      value: 'Rest my chemistry',
    },
  ])(`can update a video $field`, async ({ video, field, value }) => {
    selectedCollectionId.initialize('4');
    collectionsStore.initialize([
      aCollection().withId('4').addVideo(video).build(),
      aCollection().withId('2').build(),
    ]);

    const { updateVideo } = get(selectedCollection);
    await updateVideo(video, field as keyof Video, value, () => Promise.resolve());

    const collections = get(collectionsStore);
    expect(collections).toHaveLength(2);
    const collection = collections[1];
    const expectedVideo = {
      ...video,
      [field]: value,
    };
    expect(collection.videos[0]).toStrictEqual<Video>({
      ...expectedVideo,
      toHumanReadable: expect.any(Function),
      play: expect.any(Function),
    });
  });

  it('can add video to the collection', () => {
    selectedCollectionId.initialize('2');
    collectionsStore.initialize([
      aCollection().withId('4').addVideo(aVideo().build()).build(),
      aCollection().withId('2').build(),
    ]);

    const { addVideo } = get(selectedCollection);
    addVideo(
      aVideo()
        .withPath('video.mp4')
        .withName('Video')
        .withArtist('Artist')
        .withSong('Song')
        .withTags([])
        .withStyles([])
        .withSize(0)
        .withThumbnail('thumbnail.jpg')
        .build()
    );

    const collections = get(collectionsStore);
    expect(collections).toHaveLength(2);
    const collection = collections[1];
    expect(collection.videos[0]).toStrictEqual<Video>({
      path: 'video.mp4',
      thumbnail: 'thumbnail.jpg',
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

  it('emits an event video_update', async () => {
    let emittedVideo: Video | undefined = undefined;
    const video = aVideo().withName('Old name').build();
    selectedCollectionId.initialize('2');
    collectionsStore.initialize([
      aCollection().withId('4').addVideo(aVideo().build()).build(),
      aCollection().withId('2').addVideo(aVideo().build()).addVideo(video).build(),
    ]);

    const { updateVideo } = get(selectedCollection);
    await updateVideo(video, 'name', 'New name', async (video) => {
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
    const video = aVideo().withName('Old name').build();
    selectedCollectionId.initialize('2');
    collectionsStore.initialize([
      aCollection().withId('4').addVideo(aVideo().build()).build(),
      aCollection().withId('2').addVideo(aVideo().build()).addVideo(video).build(),
    ]);

    const { updateVideo } = get(selectedCollection);
    await updateVideo(video, 'name', 'New name', async () => {
      return Promise.reject("I'm an error");
    });

    const collections = get(collectionsStore);
    expect(collections).toHaveLength(2);
    const backedUpCollection = collections[1];
    expect(backedUpCollection.videos[1]).toStrictEqual<Video>({
      path: video.path,
      thumbnail: video.thumbnail,
      size: video.size,
      name: 'Old name',
      artist: video.artist,
      song: video.song,
      style: video.style,
      tags: video.tags,
      toHumanReadable: expect.any(Function),
      play: expect.any(Function),
    });
  });

  describe('for tags and style', () => {
    it.each([
      {
        video: aVideo().withTags(['rock', 'pop-rock']).build(),
        field: 'tags' as const,
        value: ['intro'],
        expectedValue: ['rock', 'pop-rock', 'intro'],
      },
      {
        video: aVideo().withStyles(['Hard Rock', 'Rock']).build(),
        field: 'style' as const,
        value: ['Metal'],
        expectedValue: ['Hard Rock', 'Rock', 'Metal'],
      },
    ])(
      `can add $field in a video $field`,
      async ({ video, field, value, expectedValue }) => {
        selectedCollectionId.initialize('2');
        collectionsStore.initialize([
          aCollection().withId('4').addVideo(aVideo().build()).build(),
          aCollection()
            .withId('2')
            .addVideo(aVideo().build())
            .addVideo(video)
            .build(),
        ]);

        const { updateVideo } = get(selectedCollection);
        await updateVideo(video, field, value, () => Promise.resolve());

        const collections = get(collectionsStore);
        expect(collections).toHaveLength(2);
        const collection = collections[1];
        expect(collection.videos[1][field]).toStrictEqual<string[]>(expectedValue);
      }
    );

    it.each([
      {
        video: aVideo().withTags(['rock', 'pop-rock', 'intro']).build(),
        field: 'tags' as const,
        value: ['intro'],
        expectedValue: ['rock', 'pop-rock', 'intro'],
      },
      {
        video: aVideo().withStyles(['Hard Rock', 'Rock', 'Metal']).build(),
        field: 'style' as const,
        value: ['Metal'],
        expectedValue: ['Hard Rock', 'Rock', 'Metal'],
      },
    ])(
      `do not add $field in a video $field if already present`,
      async ({ video, field, value, expectedValue }) => {
        selectedCollectionId.initialize('2');
        collectionsStore.initialize([
          aCollection().withId('4').addVideo(aVideo().build()).build(),
          aCollection()
            .withId('2')
            .addVideo(aVideo().build())
            .addVideo(video)
            .build(),
        ]);

        const { updateVideo } = get(selectedCollection);
        await updateVideo(video, field, value, () => Promise.resolve());

        const collections = get(collectionsStore);
        expect(collections).toHaveLength(2);
        const collection = collections[1];
        expect(collection.videos[1][field]).toStrictEqual<string[]>(expectedValue);
      }
    );

    it('can remove a tag from a video', async () => {
      const video = aVideo().withTags(['rock', 'punk', 'intro']).build();
      selectedCollectionId.initialize('2');
      collectionsStore.initialize([
        aCollection().withId('4').addVideo(aVideo().build()).build(),
        aCollection().withId('2').addVideo(aVideo().build()).addVideo(video).build(),
      ]);

      const { removeTag } = get(selectedCollection);
      await removeTag(video, 'punk', () => Promise.resolve());

      const collections = get(collectionsStore);
      expect(collections).toHaveLength(2);
      const collection = collections[1];
      expect(collection.videos[1].tags).toHaveLength(2);
      expect(collection.videos[1].tags).toStrictEqual(['rock', 'intro']);
    });

    it('emits a video_update when removing a tag from a video', async () => {
      let emittedVideo: Video | undefined = undefined;
      const video = aVideo().withTags(['rock', 'punk', 'intro']).build();
      selectedCollectionId.initialize('2');
      collectionsStore.initialize([
        aCollection().withId('4').addVideo(aVideo().build()).build(),
        aCollection().withId('2').addVideo(aVideo().build()).addVideo(video).build(),
      ]);

      const { removeTag } = get(selectedCollection);
      await removeTag(video, 'punk', (video) => {
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
      const video = aVideo().withTags(['rock', 'punk', 'intro']).build();
      selectedCollectionId.initialize('2');
      collectionsStore.initialize([
        aCollection().withId('4').addVideo(aVideo().build()).build(),
        aCollection().withId('2').addVideo(aVideo().build()).addVideo(video).build(),
      ]);

      const { removeTag } = get(selectedCollection);
      await removeTag(video, 'punk', async () => Promise.reject("I'm an error"));

      const collections = get(collectionsStore);
      expect(collections).toHaveLength(2);
      const collection = collections[1];
      expect(collection.videos[1]).toStrictEqual<Video>({
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
});
