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

type HumanReadable = {
  toHumanReadable(): string;
};

type Size = HumanReadable & {
  bytes: number;
};

type Duration = HumanReadable & {
  seconds: number;
};

export type Video = {
  path: string;
  thumbnail: string;
  size: Size;
  duration: Duration;
  name: string;
  artist: string;
  song: string;
  style: Style[];
  tags: string[];
  play(): string;
};
