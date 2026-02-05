import { fileConverter } from './file-converter';
import type { Style, Video } from './video';
import type { VideoFromCollection } from './collection.tauri';

export const toVideo = (
  video: VideoAddedToCollection | VideoFromCollection
): Video => {
  const size = video.size_bytes || 0;
  return {
    path: video.path,
    name: video.name,
    artist: video.artist,
    song: video.song,
    style: video.style,
    tags: video.tags,
    thumbnail: video.thumbnail || '',
    size,
    toHumanReadable: () =>
      `${(size / 1_000_000).toLocaleString('fr-FR', { minimumFractionDigits: 2, maximumFractionDigits: 2 })} MB`,
    play: () => fileConverter.convertFile(video.path),
  };
};
export type VideoAddedToCollection = {
  collection_id: string;
  path: string;
  name: string;
  artist: string;
  song: string;
  style: Style[];
  tags: string[];
  thumbnail?: string;
  size_bytes?: number;
  error?: string | null;
};
