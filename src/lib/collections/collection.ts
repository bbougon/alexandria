import { type Style, toVideo, type Video } from './video';

export type Collection = {
  id: string;
  title: string;
  videos: Video[];
};

export type VideoFromCollection = {
  path: string;
  name: string;
  artist: string;
  song: string;
  style: Style[];
  tags: string[];
  thumbnail?: string;
  size_bytes?: number;
};

export type CollectionCreated = {
  collection_id: string;
  title: string;
  videos: VideoFromCollection[];
};

export const toCollection = (collectionCreated: CollectionCreated): Collection => ({
  id: collectionCreated.collection_id,
  title: collectionCreated.title,
  videos: collectionCreated.videos.map(toVideo),
});
