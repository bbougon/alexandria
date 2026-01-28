use crate::repositories::repositories;
use crate::video::ThumbnailItem;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct VideoDTO {
    pub path: PathBuf,
}

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
    path: PathBuf,
    name: String,
    artist: String,
    song: String,
    style: Vec<Style>,
    tags: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Collection {
    pub id: Uuid,
    pub title: String,
    videos: Vec<Video>,
}

impl Collection {
    fn new(uuid: Uuid, title: &str, videos: Vec<VideoDTO>) -> Self {
        let videos = videos
            .into_iter()
            .map(|v| Video {
                path: PathBuf::from(v.path),
                name: "".to_string(),
                artist: "".to_string(),
                song: "".to_string(),
                style: vec![],
                tags: vec![],
            })
            .collect();
        Collection {
            id: uuid,
            title: title.to_string(),
            videos,
        }
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
    pub fn create_collection(videos: Option<Vec<ThumbnailItem>>) -> Collection {
        let vec = videos
            .as_ref()
            .map(|items| {
                items
                    .iter()
                    .map(|t| VideoDTO {
                        path: PathBuf::from(t.video_path.clone()),
                    })
                    .collect()
            })
            .unwrap_or_default();
        let collection = Collection::new(Uuid::new_v4(), "Ma collection", vec);
        repositories().collections().add(collection.clone());
        collection
    }
}

#[cfg(test)]
mod collection_service_tests {
    use crate::collections::collections::{
        Collection, CollectionRepositoryMemory, CollectionService, Video,
    };
    use crate::repositories::{repositories, set_repositories, Repositories};
    use crate::video::ThumbnailItem;
    use std::path::PathBuf;
    use std::sync::Arc;

    #[test]
    fn test_collection_service() {
        let mem = CollectionRepositoryMemory::new();
        set_repositories(Repositories::new(Arc::new(mem)));

        let mut videos = Vec::new();
        videos.push(ThumbnailItem {
            video_path: "foo".to_string(),
            thumbnail_path: None,
            error: None,
            size_bytes: None,
        });
        let collection = CollectionService::create_collection(Some(videos));

        assert_eq!(repositories().collections().list().len(), 1);
        assert_eq!(
            repositories().collections().get_by_id(&collection.id),
            Some(Collection {
                id: collection.id,
                title: "Ma collection".to_string(),
                videos: vec![Video {
                    path: PathBuf::from("foo"),
                    name: "".to_string(),
                    artist: "".to_string(),
                    song: "".to_string(),
                    style: vec![],
                    tags: vec![]
                }]
            })
        );
    }
}
