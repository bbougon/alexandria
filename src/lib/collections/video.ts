import { fileConverter } from './file-converter';
import type { VideoFromCollection } from './collection.store';

export const STYLES = [
  'Rock',
  'Hard Rock',
  'Metal',
  'Blues',
  'Jazz',
  'Funk',
  'Pop',
  'Country / Folk',
  'Reggae / Ska',
  'Ambient / Post-Rock',
  'Neo-Classical',
] as const;

export type Style = (typeof STYLES)[number];

// type Approach =
//   | 'Rhythm'
//   | 'Lead'
//   | 'Riffs'
//   | 'Solos'
//   | 'Improvisation'
//   | 'Accompaniment'
//   | 'Groove'
//   | 'Instrumental';
// type Tone = 'Clean' | 'Crunch' | 'High Gain' | 'Heavy';
// type Technique =
//   | 'Shred'
//   | 'Djent'
//   | 'Tapping'
//   | 'Sweep Picking'
//   | 'Palm Mute'
//   | 'Fingerstyle';
//
// type Styles = {
//   styles: Style[];
//   tags: {
//     approach: Approach[];
//     tone: Tone[];
//     technique: Technique[];
//   };
// };

export type Video = {
  path: string;
  thumbnail: string;
  size: number;
  name: string;
  artist: string;
  song: string;
  style: Style[];
  tags: string[];
  toHumanReadable(): string;
  play(): string;
};

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
