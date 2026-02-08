use crate::clock::clock;
use crate::collections::video::{VideoCollectionToUpdate, VideoFileManager};
use crate::event_bus::{Event, EventBusManager};
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
    Metal,
    #[serde(rename = "Blues")]
    Blues,
    #[serde(rename = "Jazz")]
    Jazz,
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
    pub name: String,
    pub artist: String,
    pub song: String,
    pub style: Vec<Style>,
    pub tags: Vec<String>,
    pub thumbnail: String,
    pub size_bytes: u64,
}

impl Video {
    pub fn new(path: PathBuf, thumbnail: String, size_bytes: u64) -> Self {
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
            thumbnail,
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

#[derive(serde::Serialize, Clone)]
pub struct CollectionCreated {
    pub collection_id: Uuid,
    pub title: String,
    pub videos: Vec<Video>,
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
        if let Some(item) = self.items.lock().iter_mut().find(|x| x.id == c.id) {
            *item = c;
            return;
        }
        self.items.lock().push(c);
    }

    fn get_by_id(&self, id: &Uuid) -> Option<Collection> {
        self.items.lock().iter().find(|c| c.id == *id).cloned()
    }
}

pub struct CollectionService {}

impl CollectionService {
    pub(crate) fn update_video(
        video_to_update: VideoCollectionToUpdate,
        _bus_manager: EventBusManager,
    ) {
        let mut collection = repositories()
            .collections()
            .get_by_id(&video_to_update.collection_id)
            .unwrap();

        if let Some(video) = collection
            .videos
            .iter_mut()
            .find(|p| p.path == video_to_update.video.path)
        {
            video.style = video_to_update.video.style.clone();
            video.name = video_to_update.video.name.clone();
            video.artist = video_to_update.video.artist.clone();
            video.song = video_to_update.video.song.clone();
            video.tags = video_to_update.video.tags.clone();
            video.thumbnail = video_to_update.video.thumbnail.clone();
            video.size_bytes = video_to_update.video.size_bytes;
        }
        repositories().collections().add(collection);
    }
}

impl CollectionService {
    pub fn create_collection(
        videos_paths: Vec<String>,
        video_file_manager: VideoFileManager,
        bus_manager: EventBusManager,
    ) -> Collection {
        let mut collection = Collection::new(
            Uuid::new_v4(),
            format!("Collection - {}", clock().now().format("%Y-%m-%d")).as_str(),
        );
        bus_manager.event_bus.publish(Event {
            event_type: "collection:created".parse().unwrap(),
            data: {
                serde_json::to_value(CollectionCreated {
                    collection_id: collection.id,
                    title: collection.title.clone(),
                    videos: vec![],
                })
                .unwrap()
            },
        });
        video_file_manager
            .file_manager
            .add_files_from_paths_to_collection(videos_paths, &mut collection, bus_manager)
            .unwrap();
        repositories().collections().add(collection.clone());
        collection
    }
}

#[cfg(test)]
mod collection_service_setup {
    use crate::clock::ClockGuard;
    use crate::collections::collections::{CollectionRepositoryMemory, Video};
    use crate::collections::video::{FileManager, ThumbnailItem, VideoFileManager};
    use crate::infra::event_bus::memory_event_bus::MemoryEventBus;
    use crate::repositories::{with_test_repositories, Repositories, RepositoriesGuard};
    use chrono::{DateTime, MappedLocalTime, Utc};
    use std::sync::Arc;

    struct FileManagerMemory {
        items: Vec<ThumbnailItem>,
    }

    impl FileManager for FileManagerMemory {
        fn create_video(&self, path: &str) -> Result<Video, String> {
            Ok(Video::new(path.parse().unwrap(), "".parse().unwrap(), 0))
        }
    }

    pub fn setup(
        current_date_time: MappedLocalTime<DateTime<Utc>>,
    ) -> (
        VideoFileManager,
        Arc<MemoryEventBus>,
        ClockGuard,
        RepositoriesGuard,
    ) {
        let now = current_date_time.unwrap();
        let _clock_guard = crate::clock::with_static_clock(now);
        let mem = CollectionRepositoryMemory::new();
        let _repos_guard = with_test_repositories(Repositories::new(Arc::new(mem)));
        let video_file_manager =
            VideoFileManager::new(Box::new(FileManagerMemory { items: vec![] }));
        let event_bus = Arc::new(MemoryEventBus::new());

        (video_file_manager, event_bus, _clock_guard, _repos_guard)
    }
}

#[cfg(test)]
mod collection_service_create_collection_tests {
    use crate::collections::collections::collection_service_setup::setup;
    use crate::collections::collections::{
        Collection, CollectionCreated, CollectionService, Video,
    };
    use crate::collections::video::VideoAddedToCollection;
    use crate::event_bus::EventBusManager;
    use crate::repositories::repositories;
    use chrono::{TimeZone, Utc};
    use std::path::PathBuf;

    #[test]
    fn create_collection() {
        let (video_file_manager, event_bus, _clock_guard, _repositories_guard) =
            setup(Utc.with_ymd_and_hms(2026, 1, 28, 12, 0, 0));

        let collection = CollectionService::create_collection(
            vec!["foo/video.mp4".parse().unwrap()],
            video_file_manager,
            EventBusManager::new(Box::new(event_bus.clone())),
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
                    thumbnail: "".to_string(),
                    size_bytes: 0
                }]
            })
        );
    }

    #[test]
    fn collection_service_publish_collection_created() {
        let (video_file_manager, event_bus, _clock_guard, _repositories_guard) =
            setup(Utc.with_ymd_and_hms(2026, 1, 28, 12, 0, 0));

        let collection = CollectionService::create_collection(
            vec!["foo/video.mp4".parse().unwrap()],
            video_file_manager,
            EventBusManager::new(Box::new(event_bus.clone())),
        );

        let events = event_bus.events.lock();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].event_type, "collection:created");
        assert_eq!(
            events[0].data,
            serde_json::to_value(CollectionCreated {
                collection_id: collection.id,
                title: "Collection - 2026-01-28".to_string(),
                videos: vec![],
            })
            .unwrap()
        );
    }

    #[test]
    fn collection_service_publish_video_added_to_collection() {
        let (video_file_manager, event_bus, _clock_guard, _repositories_guard) =
            setup(Utc.with_ymd_and_hms(2026, 1, 28, 12, 0, 0));

        let collection = CollectionService::create_collection(
            vec!["foo/video.mp4".parse().unwrap()],
            video_file_manager,
            EventBusManager::new(Box::new(event_bus.clone())),
        );

        let events = event_bus.events.lock();
        assert_eq!(events.len(), 2);
        assert_eq!(events[1].event_type, "video:added");
        assert_eq!(
            events[1].data,
            serde_json::to_value(VideoAddedToCollection {
                collection_id: collection.id,
                path: "foo/video.mp4".parse().unwrap(),
                name: "video.mp4".to_string(),
                artist: "".to_string(),
                song: "".to_string(),
                style: vec![],
                tags: vec![],
                thumbnail: "".to_string(),
                size_bytes: 0,
            })
            .unwrap()
        );
    }
}

#[cfg(test)]
mod collection_service_update_video_tests {
    use crate::collections::collections::collection_service_setup::setup;
    use crate::collections::collections::Style::Rock;
    use crate::collections::collections::{CollectionService, Video};
    use crate::collections::video::{VideoCollectionToUpdate, VideoToUpdate};
    use crate::event_bus::EventBusManager;
    use crate::repositories::repositories;
    use chrono::{TimeZone, Utc};

    #[test]
    fn update_video() {
        let (video_file_manager, event_bus, _clock_guard, _repositories_guard) =
            setup(Utc.with_ymd_and_hms(2026, 1, 28, 12, 0, 0));
        let collection = CollectionService::create_collection(
            vec!["foo/video.mp4".parse().unwrap()],
            video_file_manager,
            EventBusManager::new(Box::new(event_bus.clone())),
        );
        let video = collection.videos[0].clone();

        CollectionService::update_video(
            VideoCollectionToUpdate {
                collection_id: collection.id,
                video: VideoToUpdate {
                    path: video.path.clone(),
                    name: "Rest my chemistry - Intro".to_string(),
                    artist: "Interpol".to_string(),
                    song: "Rest my chemistry".to_string(),
                    style: vec![Rock],
                    tags: vec!["alternative".to_string(), "rhythmic".to_string()],
                    thumbnail: video.thumbnail.clone(),
                    size_bytes: video.size_bytes,
                },
            },
            EventBusManager::new(Box::new(event_bus.clone())),
        );

        let retrieved_collection = repositories()
            .collections()
            .get_by_id(&collection.id)
            .unwrap();
        assert_eq!(
            retrieved_collection.videos[0],
            Video {
                path: video.path,
                name: "Rest my chemistry - Intro".to_string(),
                artist: "Interpol".to_string(),
                song: "Rest my chemistry".to_string(),
                style: vec![Rock],
                tags: vec!["alternative".to_string(), "rhythmic".to_string()],
                thumbnail: video.thumbnail,
                size_bytes: video.size_bytes,
            }
        )
    }
}
