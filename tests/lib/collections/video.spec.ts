import { toVideo, Video } from '../../../src/lib/collections/video.ts';
import { expect } from 'vitest';
import { fileConverter } from '../../../src/lib/collections/file-converter.ts';

describe('Video', () => {
  it('should be mapped', function () {
    fileConverter.convertFile = () => 'fooBar';
    const video = toVideo({ videoPath: 'foo', thumbnailPath: 'bar', size: 1000 });

    expect(video).toStrictEqual<Video>({
      path: 'foo',
      thumbnail: 'fooBar',
      size: 1000,
      name: 'foo',
    });
  });

  it('should be mapped with a name', function () {
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
    });
  });
});
