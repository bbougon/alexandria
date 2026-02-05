import { type Style, STYLES, Video } from '../../../src/lib/collections/video';
import { fakerFR } from '@faker-js/faker';
import { Collection } from '../../../src/lib/collections/collection.ts';

interface Builder<T> {
  build: () => T;
}

class CollectionBuilder implements Builder<Collection> {
  private id: string = fakerFR.string.uuid();
  private title: string = fakerFR.lorem.sentence();
  private videos: Video[] = [];
  build(): Collection {
    return { id: this.id, title: this.title, videos: this.videos };
  }

  withId(id: string) {
    this.id = id;
    return this;
  }

  withTitle(title: string) {
    this.title = title;
    return this;
  }

  addVideo(video: Video) {
    this.videos.push(video);
    return this;
  }
}

class VideoBuilder implements Builder<Video> {
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

  withName(name: string) {
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

  withPath(path: string) {
    this.path = path;
    return this;
  }

  withSize(size: number) {
    this.size = size;
    return this;
  }

  withThumbnail(thumbnail: string) {
    this.thumbnail = thumbnail;
    return this;
  }

  build(): Video {
    return {
      play(): string {
        return '';
      },
      toHumanReadable(): string {
        return '';
      },
      path: this.path,
      thumbnail: this.thumbnail,
      size: this.size,
      name: this.name,
      artist: this.artist,
      song: this.song,
      style: this.style,
      tags: this.tags,
    };
  }
}
export const aVideo = () => new VideoBuilder();

export const aCollection = () => new CollectionBuilder();
