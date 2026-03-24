import { toVideo } from '../../../src/lib/collections/video.tauri.ts';

describe('Tauri video', () => {
  it('should have video size human readable (KB)', function () {
    const video = toVideo({
      artist: 'An artist',
      collection_id: '1',
      duration_seconds: 0,
      name: 'Video name',
      path: 'Video path',
      size_bytes: 1_000,
      song: 'Song name',
      style: [],
      tags: [],
      thumbnail: 'thumbnail.jpg',
    });

    expect(video.size.toHumanReadable()).toBe('1 kB');
  });

  it('should have video size human readable (MB)', function () {
    const video = toVideo({
      artist: 'An artist',
      collection_id: '1',
      duration_seconds: 0,
      name: 'Video name',
      path: 'Video path',
      size_bytes: 10_500_000,
      song: 'Song name',
      style: [],
      tags: [],
      thumbnail: 'thumbnail.jpg',
    });

    expect(video.size.toHumanReadable()).toBe('10.50 MB');
  });

  it('should have video size human readable (Bytes)', function () {
    const video = toVideo({
      artist: 'An artist',
      collection_id: '1',
      duration_seconds: 0,
      name: 'Video name',
      path: 'Video path',
      size_bytes: 500,
      song: 'Song name',
      style: [],
      tags: [],
      thumbnail: 'thumbnail.jpg',
    });

    expect(video.size.toHumanReadable()).toBe('500 byte');
  });

  it('should have video duration human readable', function () {
    const video = toVideo({
      artist: 'An artist',
      collection_id: '1',
      duration_seconds: 354,
      name: 'Video name',
      path: 'Video path',
      size_bytes: 1_000,
      song: 'Song name',
      style: [],
      tags: [],
      thumbnail: 'thumbnail.jpg',
    });

    expect(video.duration.toHumanReadable()).toBe('05:54');
  });

  it('should have video duration human readable with hours', function () {
    const video = toVideo({
      artist: 'An artist',
      collection_id: '1',
      duration_seconds: 3661,
      name: 'Video name',
      path: 'Video path',
      size_bytes: 1_000,
      song: 'Song name',
      style: [],
      tags: [],
      thumbnail: 'thumbnail.jpg',
    });

    expect(video.duration.toHumanReadable()).toBe('01:01:01');
  });
});
