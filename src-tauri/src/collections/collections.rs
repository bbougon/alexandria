use crate::clock::clock;
use crate::collections::video::VideoFileManager;
use crate::repositories::repositories;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Style {
    #[serde(rename = "Rock")]
    Rock,
    #[serde(rename = "Hard Rock")]
    HardRock,
    #[serde(rename = "Metal")]
    Jazz,
    #[serde(rename = "Blues")]
    Metal,
    #[serde(rename = "Jazz")]
    Blues,
    #[serde(rename = "Funk")]
    Funk,
    #[serde(rename = "Pop")]
    Pop,
    #[serde(rename = "Country / Folk")]
    CountryFolk,
    #[serde(rename = "Reggae / Ska")]
    ReggaeSka,
    #[serde(rename = "Ambient / Post-Rock")]
    AmbientPostRock,
    #[serde(rename = "Neo-Classical")]
    NeoClassical,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Video {
    pub path: PathBuf,
    name: String,
    artist: String,
    song: String,
    style: Vec<Style>,
    tags: Vec<String>,
    pub thumbnail_path: String,
    pub size_bytes: u64,
}

impl Video {
    pub fn new(path: PathBuf, thumbnail_path: String, size_bytes: u64) -> Self {
        let name = path
            .file_name()
            .and_then(|os_str| os_str.to_str())
            .unwrap_or("unknown")
            .to_string();
        Self {
            path,
            name,
            artist: "".to_string(),
            song: "".to_string(),
            style: vec![],
            tags: vec![],
            thumbnail_path,
            size_bytes,
        }
    }
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Collection {
    pub id: Uuid,
    pub title: String,
    pub videos: Vec<Video>,
}

impl Collection {
    fn new(uuid: Uuid, title: &str) -> Self {
        Collection {
            id: uuid,
            title: title.to_string(),
            videos: vec![],
        }
    }

    pub fn add_video(&mut self, video: Video) {
        self.videos.push(video);
    }
}

pub trait CollectionRepository: Send + Sync {
    fn list(&self) -> Vec<Collection>;
    fn add(&self, c: Collection);
    fn get_by_id(&self, id: &Uuid) -> Option<Collection>;
}

#[derive(Default)]
pub struct CollectionRepositoryMemory {
    items: parking_lot::Mutex<Vec<Collection>>,
}

impl CollectionRepositoryMemory {
    pub fn new() -> Self {
        Self::default()
    }
}

impl CollectionRepository for CollectionRepositoryMemory {
    fn list(&self) -> Vec<Collection> {
        self.items.lock().clone()
    }
    fn add(&self, c: Collection) {
        self.items.lock().push(c);
    }

    fn get_by_id(&self, id: &Uuid) -> Option<Collection> {
        self.items.lock().iter().find(|c| c.id == *id).cloned()
    }
}

pub struct CollectionService {}

impl CollectionService {
    pub fn create_collection(
        videos_paths: Vec<String>,
        video_file_manager: VideoFileManager,
    ) -> Collection {
        let mut collection = Collection::new(
            Uuid::new_v4(),
            format!("Collection - {}", clock().now().format("%Y-%m-%d")).as_str(),
        );
        video_file_manager
            .file_manager
            .add_files_from_paths_to_collection(videos_paths, &mut collection)
            .unwrap();
        repositories().collections().add(collection.clone());
        collection
    }
}

#[cfg(test)]
mod collection_service_tests {
    use crate::collections::collections::{
        Collection, CollectionRepositoryMemory, CollectionService, Video,
    };
    use crate::collections::video::{FileManager, ThumbnailItem, VideoFileManager};
    use crate::repositories::{repositories, set_repositories, Repositories};
    use chrono::{TimeZone, Utc};
    use std::path::PathBuf;
    use std::sync::Arc;

    struct FileManagerMemory {
        items: Vec<ThumbnailItem>,
    }

    impl FileManager for FileManagerMemory {
        fn create_video(&self, path: &str) -> Result<Video, String> {
            Ok(Video::new(path.parse().unwrap(), "".parse().unwrap(), 0))
        }
    }

    #[test]
    fn test_collection_service() {
        let now = Utc.with_ymd_and_hms(2026, 1, 28, 12, 0, 0).unwrap();
        let _clock_guard = crate::clock::with_static_clock(now);
        let mem = CollectionRepositoryMemory::new();
        set_repositories(Repositories::new(Arc::new(mem)));

        let video_file_manager =
            VideoFileManager::new(Box::new(FileManagerMemory { items: vec![] }));
        let collection = CollectionService::create_collection(
            vec!["foo/video.mp4".parse().unwrap()],
            video_file_manager,
        );

        assert_eq!(repositories().collections().list().len(), 1);
        assert_eq!(
            repositories().collections().get_by_id(&collection.id),
            Some(Collection {
                id: collection.id,
                title: "Collection - 2026-01-28".to_string(),
                videos: vec![Video {
                    path: PathBuf::from("foo/video.mp4"),
                    name: "video.mp4".to_string(),
                    artist: "".to_string(),
                    song: "".to_string(),
                    style: vec![],
                    tags: vec![],
                    thumbnail_path: "".to_string(),
                    size_bytes: 0
                }]
            })
        );
    }
}
