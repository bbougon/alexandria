use crate::collections::collections::CollectionService;
use crate::infra::files::file_manager::FileManagerForHardDrive;
use tauri::AppHandle;

#[derive(serde::Serialize, Clone)]
pub struct ThumbnailItem {
    pub video_path: String,
    pub thumbnail_path: Option<String>,
    pub size_bytes: Option<u64>,
}

pub trait FileManager {
    fn retrieve_files_in(&self, paths: Vec<String>) -> Result<Vec<ThumbnailItem>, String>;
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
    let result = VideoFileManager::new(Box::new(FileManagerForHardDrive::new(app)))
        .file_manager
        .retrieve_files_in(paths);
    CollectionService::create_collection(result.clone().ok());
    result
}
