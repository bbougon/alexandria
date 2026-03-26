import { fileConverter } from './file-converter';
import type { Style, Video } from './video';
import type { VideoFromCollection } from './collection.tauri';
import { intervalToDuration } from 'date-fns';
import type { VideoData } from './collectionCreation.store';

export const toVideo = (
  video: VideoAddedToCollection | VideoFromCollection
): Video => {
  const size = video.size_bytes || 0;
  const durationInSeconds = video.duration_seconds || 0;
  return {
    path: video.path,
    name: video.name,
    artist: video.artist,
    song: video.song,
    style: video.style,
    tags: video.tags,
    thumbnail: video.thumbnail || '',
    size: {
      bytes: size,
      toHumanReadable: () => bytesToHumanReadable(size),
    },
    duration: {
      seconds: durationInSeconds,
      toHumanReadable: () => secondsToHumanReadable(durationInSeconds),
    },
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
  duration_seconds?: number;
  error?: string | null;
};

export type VideoDataRetrievedDTO = {
  path: string;
  thumbnail: string;
  size_bytes: number;
  duration_seconds: number;
};

const secondsToHumanReadable = (durationInSeconds: number) => {
  const duration = intervalToDuration({
    start: 0,
    end: durationInSeconds * 1000,
  });
  const format = (numberToFormat: number | undefined) =>
    new Intl.NumberFormat('en-EN', { minimumIntegerDigits: 2 }).format(
      numberToFormat || 0
    );

  const minutesSeconds = `${format(duration.minutes)}:${format(duration.seconds)}`;
  return duration.hours && (duration.hours || 0) > 0
    ? `${format(duration.hours)}:${minutesSeconds}`
    : minutesSeconds;
};

const bytesToHumanReadable = (size: number) => {
  const units = ['byte', 'kilobyte', 'megabyte'];
  const unitIndex = size < 1_000 ? 0 : size < 1_000_000 ? 1 : 2;
  const value = size / Math.pow(1000, unitIndex);
  return new Intl.NumberFormat('en-EN', {
    style: 'unit',
    unit: units[unitIndex],
    minimumFractionDigits: unitIndex === 2 ? 2 : 0,
    maximumFractionDigits: unitIndex === 2 ? 2 : 0,
  }).format(value);
};

export const toVideoData = (video: VideoDataRetrievedDTO): VideoData => {
  const size = video.size_bytes;
  const durationInSeconds = video.duration_seconds;
  return {
    duration: {
      seconds: video.duration_seconds,
      toHumanReadable: () => {
        return secondsToHumanReadable(durationInSeconds);
      },
    },
    path: video.path,
    size: {
      bytes: video.size_bytes,
      toHumanReadable: () => {
        return bytesToHumanReadable(size);
      },
    },
    thumbnail: video.thumbnail,
    name: video.path.split('/').pop()!,
  };
};
