import { convertFileSrc } from '@tauri-apps/api/core';

export const fileConverter = {
  convertFile: (path: string): string => convertFileSrc(path),
};
