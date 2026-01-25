import { fileConverter } from './file-converter';

export type Video = {
  path: string;
  thumbnail: string;
  size: number;
  name: string;
};

export const toVideo = (video: {
  videoPath: string;
  thumbnailPath: string;
  size: number;
}): Video => {
  return {
    path: video.videoPath,
    name: video.videoPath.split('/').pop() || '',
    thumbnail: fileConverter.convertFile(video.thumbnailPath),
    size: video.size,
  };
};
