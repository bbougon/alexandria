import { toVideo, Video } from '../../../src/lib/collections/video.ts';
import { fileConverter } from '../../../src/lib/collections/file-converter.ts';
import { expect } from 'vitest';

describe('Video', () => {
  it('should be mapped', () => {
    fileConverter.convertFile = () => 'fooBar';

    const video = toVideo({ videoPath: 'foo', thumbnailPath: 'bar', size: 1000 });

    expect(video).toStrictEqual<Video>({
      path: 'foo',
      thumbnail: 'fooBar',
      size: 1000,
      name: 'foo',
      artist: 'Artist name',
      song: 'Song name',
      style: ['Rock', 'Hard Rock'],
      tags: [],
      toHumanReadable: expect.any(Function),
      play: expect.any(Function),
    });
  });

  it('should be mapped with a name', () => {
    fileConverter.convertFile = () => 'fooBar';

    const video = toVideo({
      videoPath: '/path/to/my/video.mp4',
      thumbnailPath: 'bar',
      size: 1000,
    });

    expect(video).toStrictEqual<Video>({
      path: '/path/to/my/video.mp4',
      thumbnail: 'fooBar',
      size: 1000,
      name: 'video.mp4',
      artist: 'Artist name',
      song: 'Song name',
      style: ['Rock', 'Hard Rock'],
      tags: [],
      toHumanReadable: expect.any(Function),
      play: expect.any(Function),
    });
  });

  it('should give the human readable size', () => {
    fileConverter.convertFile = () => 'fooBar';

    const video = toVideo({
      videoPath: '/path/to/my/video.mp4',
      thumbnailPath: 'bar',
      size: 1000000,
    });

    expect(video.toHumanReadable()).toBe('1,00 MB');
  });

  it('should play a video', () => {
    const videoPath = '/path/to/my/video.mp4';
    let isPlayed = false;
    fileConverter.convertFile = (videoFile: string) => {
      isPlayed = videoFile === videoFile;
      return videoFile;
    };

    const video = toVideo({
      videoPath: videoPath,
      thumbnailPath: 'bar',
      size: 1000000,
    });
    const file = video.play();

    expect(file).toBe(videoPath);
    expect(isPlayed).toBe(true);
  });
});
