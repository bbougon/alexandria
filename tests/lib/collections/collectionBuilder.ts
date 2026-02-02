import { type Style, STYLES } from '../../../src/lib/collections/video';
import { fakerFR } from '@faker-js/faker';
import type { VideoFromCollection } from '../../../src/lib/collections/collection.store.ts';

interface Builder<T> {
  build: () => T;
}

class VideoFromCollectionBuilder implements Builder<VideoFromCollection> {
  private path: string = fakerFR.system.filePath();
  private thumbnail: string = fakerFR.string.alpha();
  private size: number = fakerFR.number.int();
  private name: string = fakerFR.string.alpha();
  private artist: string = fakerFR.music.artist();
  private song: string = fakerFR.music.songName();
  private style: Style[] = [];
  private tags: string[] = [];

  constructor() {
    const range = fakerFR.number.int({ min: 0, max: STYLES.length - 1 });
    for (let i = 0; i < range; i++) {
      this.style.push(
        STYLES[fakerFR.number.int({ min: 0, max: STYLES.length - 1 })]
      );
      this.tags.push(fakerFR.lorem.word());
    }
  }

  withName(name: string): VideoFromCollectionBuilder {
    this.name = name;
    return this;
  }

  withArtist(artist: string) {
    this.artist = artist;
    return this;
  }

  withSong(song: string) {
    this.song = song;
    return this;
  }

  withTags(tags: string[]) {
    this.tags = tags;
    return this;
  }

  withStyles(styles: Style[]) {
    this.style = styles;
    return this;
  }

  build(): VideoFromCollection {
    return {
      path: this.path,
      thumbnail: this.thumbnail,
      size_bytes: this.size,
      name: this.name,
      artist: this.artist,
      song: this.song,
      style: this.style,
      tags: this.tags,
    };
  }
}

export const aVideoFromCollection = () => {
  return new VideoFromCollectionBuilder();
};
