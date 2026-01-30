use crate::collections::collections::{Collection, CollectionService, Video};
use crate::infra::files::file_manager::FileManagerForHardDrive;
use tauri::AppHandle;

#[derive(serde::Serialize, Clone)]
pub struct ThumbnailItem {
    pub video_path: String,
    pub thumbnail: Option<String>,
    pub size_bytes: Option<u64>,
}

pub trait FileManager {
    fn add_files_from_paths_to_collection(
        &self,
        paths: Vec<String>,
        collection: &mut Collection,
    ) -> Result<(), String> {
        if paths.is_empty() {
            return Err("Aucun fichier reçu".to_string());
        }

        for (_index, p) in paths.iter().enumerate() {
            let video = self.create_video(p)?;
            collection.add_video(video.clone());
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
    let video_file_manager = VideoFileManager::new(Box::new(FileManagerForHardDrive::new(app)));
    let collection = CollectionService::create_collection(paths, video_file_manager);
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
