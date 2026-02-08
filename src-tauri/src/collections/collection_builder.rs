#[cfg(test)]
use crate::collections::collections::{Collection, Style, Video};
#[cfg(test)]
use fake::faker::filesystem::fr_fr::FilePath;
#[cfg(test)]
use fake::faker::job::fr_fr::Title;
#[cfg(test)]
use fake::faker::name::fr_fr::Name;
#[cfg(test)]
use fake::Fake;
#[cfg(test)]
use fake::Faker;
#[cfg(test)]
use std::path::PathBuf;

#[cfg(test)]
pub struct CollectionBuilder {
    title: String,
    videos: Vec<Video>,
}

#[cfg(test)]
impl CollectionBuilder {
    fn new() -> CollectionBuilder {
        CollectionBuilder {
            title: "".to_string(),
            videos: vec![],
        }
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.parse().unwrap();
        self
    }

    pub fn add_video(mut self, video: &Video) -> Self {
        self.videos.push(video.clone());
        self
    }

    pub fn build(self) -> Collection {
        Collection {
            id: uuid::Uuid::new_v4(),
            title: self.title.clone(),
            videos: self.videos.clone(),
        }
    }
}

#[cfg(test)]
pub struct VideoBuilder {
    path: String,
    name: String,
    artist: String,
    song: String,
    style: Vec<Style>,
    tags: Vec<String>,
    thumbnail: String,
    size_bytes: u64,
}

#[cfg(test)]
impl VideoBuilder {
    fn new() -> VideoBuilder {
        VideoBuilder {
            path: FilePath().fake(),
            name: Title().fake(),
            artist: Name().fake(),
            song: Title().fake(),
            style: vec![],
            tags: vec![],
            thumbnail: FilePath().fake(),
            size_bytes: Faker.fake::<u64>(),
        }
    }

    pub fn build(self) -> Video {
        Video {
            path: PathBuf::from(self.path),
            name: self.name.clone(),
            artist: self.artist.clone(),
            song: self.song.clone(),
            style: self.style.clone(),
            tags: self.tags.clone(),
            thumbnail: self.thumbnail.clone(),
            size_bytes: self.size_bytes,
        }
    }
}

#[cfg(test)]
pub fn a_collection() -> CollectionBuilder {
    CollectionBuilder::new()
}

#[cfg(test)]
pub fn a_video() -> VideoBuilder {
    VideoBuilder::new()
}
