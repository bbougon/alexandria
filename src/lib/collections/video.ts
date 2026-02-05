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
