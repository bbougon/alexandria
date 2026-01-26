import { fileConverter } from './file-converter';

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

export const toVideo = (video: {
  videoPath: string;
  thumbnailPath: string;
  size: number;
}): Video => {
  return {
    path: video.videoPath,
    name: video.videoPath.split('/').pop() || '',
    artist: 'Artist name',
    song: 'Song name',
    style: ['Rock', 'Hard Rock'],
    tags: [],
    thumbnail: fileConverter.convertFile(video.thumbnailPath),
    size: video.size,
    toHumanReadable: () =>
      `${(video.size / 1_000_000).toLocaleString('fr-FR', { minimumFractionDigits: 2, maximumFractionDigits: 2 })} MB`,
    play: () => fileConverter.convertFile(video.videoPath),
  };
};
