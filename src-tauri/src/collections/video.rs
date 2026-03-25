use crate::collections::collections::Style;
use crate::collections::events::VideoDataRetrieved;
use crate::event_bus::EventBusManager;
use crate::infra::files::file_manager::VideoData;
use std::path::PathBuf;

#[derive(serde::Serialize, Clone)]
pub struct ThumbnailItem {
    pub video_path: String,
    pub thumbnail: Option<String>,
    pub size_bytes: Option<u64>,
    pub duration_seconds: Option<u64>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct VideoCollectionToUpdate {
    pub collection_id: uuid::Uuid,
    pub video: VideoToUpdate,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct VideoToUpdate {
    pub path: PathBuf,
    pub name: String,
    pub artist: String,
    pub song: String,
    pub style: Vec<Style>,
    pub tags: Vec<String>,
    pub thumbnail: String,
    pub size_bytes: u64,
    pub duration_seconds: u64,
}

pub trait FileManager {
    fn retrieve_all_videos_data(
        &self,
        paths: Vec<String>,
        event_bus_manager: EventBusManager,
    ) -> Result<Vec<VideoData>, String> {
        if paths.is_empty() {
            return Err("Aucun fichier reçu".to_string());
        }

        let mut result = vec![];
        for (_index, p) in paths.iter().enumerate() {
            let video_data = self.retrieve_video_data(p)?;
            result.push(video_data.clone());
            event_bus_manager.publish(
                "video_data:retrieved",
                VideoDataRetrieved::from(&video_data),
            )
        }
        Ok(result)
    }

    fn retrieve_video_data(&self, path: &str) -> Result<VideoData, String>;
}

pub struct VideoFileManager {
    pub file_manager: Box<dyn FileManager>,
}

impl VideoFileManager {
    pub fn new(file_manager: Box<dyn FileManager>) -> Self {
        Self { file_manager }
    }
}

#[cfg(test)]
mod video_file_manager_tests {
    use crate::collections::events::VideoDataRetrieved;
    use crate::collections::video::{FileManager, VideoFileManager};
    use crate::event_bus::EventBusManager;
    use crate::infra::event_bus::memory_event_bus::MemoryEventBus;
    use crate::infra::files::file_manager::VideoData;
    use std::path::PathBuf;
    use std::sync::Arc;

    struct FileManagerMemory {}

    impl FileManager for FileManagerMemory {
        fn retrieve_video_data(&self, path: &str) -> Result<VideoData, String> {
            Ok(VideoData {
                path: path.parse().unwrap(),
                thumbnail: "".parse().unwrap(),
                size_bytes: 0,
                duration_seconds: 0,
            })
        }
    }

    #[test]
    fn emits_en_event_when_video_data_is_retrieved() {
        let file_manager = FileManagerMemory {};
        let video_file_manager = VideoFileManager::new(Box::new(file_manager));
        let event_bus = Arc::new(MemoryEventBus::new());
        let paths = vec!["path/to/video1".to_string(), "path/to/video2".to_string()];

        let result = video_file_manager
            .file_manager
            .retrieve_all_videos_data(paths, EventBusManager::new(event_bus.clone()));

        assert!(result.is_ok());
        let events = event_bus.events.lock();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].event_type, "video_data:retrieved");
        assert_eq!(
            events[0].data,
            serde_json::to_value(VideoDataRetrieved {
                path: PathBuf::from("path/to/video1"),
                thumbnail: "".to_string(),
                size_bytes: 0,
                duration_seconds: 0,
            })
            .unwrap()
        );
    }
}
