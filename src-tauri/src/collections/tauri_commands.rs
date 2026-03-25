use crate::collections::collections::{Collection, CollectionService};
use crate::collections::tauri_dtos::VideoDataDTO;
use crate::collections::video::{ThumbnailItem, VideoCollectionToUpdate, VideoFileManager};
use crate::event_bus::EventBusManager;
use crate::infra::event_bus::tauri_event_bus::TauriEventBus;
use crate::infra::files::file_manager::FileManagerForHardDrive;
use crate::infra::tauri::path::allow_path;
use crate::repositories::repositories;
use crate::search::search_service::{ApplyPathRights, Indexer, SearchService, TantivyIndexer};
use once_cell::sync::Lazy;
use std::sync::Arc;
use tantivy::schema::Field;
use tantivy::IndexWriter;
use tauri::AppHandle;

static SEARCH_SERVICE: Lazy<SearchService<IndexWriter, Field>> =
    Lazy::new(|| SearchService::new(TantivyIndexer::initialize()));

#[tauri::command]
pub async fn retrieve_videos_data(
    app: AppHandle,
    paths: Vec<String>,
) -> Result<Vec<VideoDataDTO>, String> {
    let video_file_manager = VideoFileManager::new(Box::new(FileManagerForHardDrive::new()));
    let video_data = video_file_manager.file_manager.retrieve_all_videos_data(
        paths,
        EventBusManager::new(Arc::new(TauriEventBus::new(app.clone()))),
    );
    Ok(video_data?.iter().map(|v| VideoDataDTO::from(v)).collect())
}

// TODO: Refactor create_collection to return a collection instead of a vector of thumbnails
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
        EventBusManager::new(Arc::new(TauriEventBus::new(app.clone()))),
    );
    Ok(collection
        .videos
        .iter()
        .map(|v| ThumbnailItem {
            video_path: v.path.to_string_lossy().to_string(),
            thumbnail: v.thumbnail.parse().ok(),
            size_bytes: Option::from(v.size_bytes),
            duration_seconds: Option::from(v.duration_seconds),
        })
        .collect())
}

#[tauri::command]
pub async fn update_video(app: AppHandle, video: VideoCollectionToUpdate) -> Result<(), String> {
    allow_path(&app, video.video.path.to_str().unwrap())?;
    CollectionService::update_video(
        video,
        EventBusManager::new(Arc::new(TauriEventBus::new(app))),
    );
    Ok(())
}

// TODO: Refactor get_collections to return a CollectionDTO instead of a domain Collection
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

#[tauri::command]
pub async fn search_videos(app: AppHandle, query: String) -> Result<(), String> {
    SEARCH_SERVICE.initialize(EventBusManager::new(Arc::new(TauriEventBus::new(
        app.clone(),
    ))));
    if !query.is_empty() {
        SEARCH_SERVICE.index_all_videos()?;
    }
    let app_clone = app.clone();
    let callback: ApplyPathRights = Box::new(move |path| allow_path(&app_clone, path));
    SEARCH_SERVICE.search(&query, Some(&callback))?;
    Ok(())
}
