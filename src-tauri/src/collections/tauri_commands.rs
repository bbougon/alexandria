use crate::collections::collections::CollectionService;
use crate::collections::video::{ThumbnailItem, VideoCollectionToUpdate, VideoFileManager};
use crate::event_bus::{EventBusManager, TauriEventBus};
use crate::infra::files::file_manager::FileManagerForHardDrive;
use tauri::AppHandle;

#[tauri::command]
pub async fn create_collection(
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

#[tauri::command]
pub async fn update_video(app: AppHandle, video: VideoCollectionToUpdate) -> Result<(), String> {
    log::info!("update_video: {:?}", video);
    Ok(())
}
