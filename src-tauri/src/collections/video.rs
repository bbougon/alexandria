use crate::collections::collections::{Collection, CollectionService, Style, Video};
use crate::event_bus::{Event, EventBusManager, TauriEventBus};
use crate::infra::files::file_manager::FileManagerForHardDrive;
use std::path::PathBuf;
use tauri::AppHandle;

#[derive(serde::Serialize, Clone)]
pub struct ThumbnailItem {
    pub video_path: String,
    pub thumbnail: Option<String>,
    pub size_bytes: Option<u64>,
}

#[derive(serde::Serialize, Clone)]
pub struct VideoAddedToCollection {
    pub collection_id: uuid::Uuid,
    pub path: PathBuf,
    pub name: String,
    pub artist: String,
    pub song: String,
    pub style: Vec<Style>,
    pub tags: Vec<String>,
    pub thumbnail: String,
    pub size_bytes: u64,
}

pub trait FileManager {
    fn add_files_from_paths_to_collection(
        &self,
        paths: Vec<String>,
        collection: &mut Collection,
        bus_manager: EventBusManager,
    ) -> Result<(), String> {
        if paths.is_empty() {
            return Err("Aucun fichier reçu".to_string());
        }

        for (_index, p) in paths.iter().enumerate() {
            let video = self.create_video(p)?;
            collection.add_video(video.clone());
            let event = Event {
                event_type: "video:added".parse().unwrap(),
                data: {
                    serde_json::to_value(VideoAddedToCollection {
                        collection_id: collection.id,
                        path: video.path.clone(),
                        name: video.name.clone(),
                        artist: video.artist.clone(),
                        song: video.song.clone(),
                        style: video.style.clone(),
                        tags: video.tags.clone(),
                        thumbnail: video.thumbnail.clone(),
                        size_bytes: video.size_bytes,
                    })
                    .unwrap()
                },
            };
            bus_manager.event_bus.publish(event)
        }
        Ok(())
    }

    fn create_video(&self, path: &str) -> Result<Video, String>;
}

pub struct VideoFileManager {
    pub file_manager: Box<dyn FileManager>,
}

impl VideoFileManager {
    pub fn new(file_manager: Box<dyn FileManager>) -> Self {
        Self { file_manager }
    }
}

#[tauri::command]
pub async fn process_video(
    app: AppHandle,
    paths: Vec<String>,
) -> Result<Vec<ThumbnailItem>, String> {
    let video_file_manager = VideoFileManager::new(Box::new(FileManagerForHardDrive::new()));
    let collection = CollectionService::create_collection(
        paths,
        video_file_manager,
        EventBusManager::new(Box::new(TauriEventBus::new(app.clone()))),
    );
    Ok(collection
        .videos
        .iter()
        .map(|v| ThumbnailItem {
            video_path: v.path.to_string_lossy().to_string(),
            thumbnail: v.thumbnail.parse().ok(),
            size_bytes: Option::from(v.size_bytes),
        })
        .collect())
}
