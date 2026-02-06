use crate::collections::collections::{Collection, CollectionService};
use crate::collections::video::{ThumbnailItem, VideoCollectionToUpdate, VideoFileManager};
use crate::event_bus::{EventBusManager, TauriEventBus};
use crate::infra::files::file_manager::FileManagerForHardDrive;
use crate::repositories::repositories;
use tauri::{AppHandle, Manager, Runtime};

#[tauri::command]
pub async fn create_collection(
    app: AppHandle,
    paths: Vec<String>,
) -> Result<Vec<ThumbnailItem>, String> {
    for path in &paths {
        allow_path(&app, path)?;
    }
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
    allow_path(&app, video.video.path.to_str().unwrap())?;
    CollectionService::update_video(
        video,
        EventBusManager::new(Box::new(TauriEventBus::new(app))),
    );
    Ok(())
}

#[tauri::command]
pub async fn get_collections(app: AppHandle) -> Result<Vec<Collection>, String> {
    let collections = repositories().collections().list();
    for c in &collections {
        for v in &c.videos {
            allow_path(&app, v.path.to_str().unwrap())?;
        }
    }
    Ok(collections)
}

fn allow_path<R: Runtime>(app: &AppHandle<R>, path: &str) -> Result<(), String> {
    app.asset_protocol_scope()
        .allow_file(path)
        .map_err(|e: tauri::Error| e.to_string())
}
