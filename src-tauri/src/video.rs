use std::{
    path::{Path, PathBuf},
    process::Command,
};

use crate::collections::collections::CollectionService;
use tauri::{AppHandle, Emitter, Manager};

#[derive(serde::Serialize, Clone)]
pub struct ThumbnailItem {
    pub video_path: String,
    pub thumbnail_path: Option<String>,
    pub error: Option<String>,
    pub size_bytes: Option<u64>,
}

#[derive(serde::Serialize, Clone)]
pub struct ThumbnailProgress {
    pub index: usize,
    pub total: usize,
    pub video_path: String,
    pub thumbnail_path: Option<String>,
    pub error: Option<String>,
    pub size_bytes: Option<u64>,
}

fn safe_file_stem(p: &Path) -> String {
    p.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("video")
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect()
}

fn make_thumbnail_path(cache_dir: &Path, video_path: &Path) -> PathBuf {
    let stem = safe_file_stem(video_path);
    cache_dir.join("thumbnails").join(format!("{stem}.jpg"))
}

fn ensure_parent_dir(p: &Path) -> Result<(), String> {
    if let Some(parent) = p.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn generate_one_thumbnail_ffmpeg(video_path: &Path, out_path: &Path) -> Result<(), String> {
    ensure_parent_dir(out_path)?;

    let output = Command::new("ffmpeg")
        .args([
            "-hide_banner",
            "-loglevel",
            "error",
            "-y",
            "-ss",
            "1",
            "-i",
            video_path.to_string_lossy().as_ref(),
            "-frames:v",
            "1",
            "-vf",
            "scale=320:-2",
            "-q:v",
            "5",
            out_path.to_string_lossy().as_ref(),
        ])
        .output()
        .map_err(|e| format!("Impossible de lancer ffmpeg: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(format!("ffmpeg a échoué: {stderr}"));
    }

    Ok(())
}

#[tauri::command]
pub async fn process_video(
    app: AppHandle,
    paths: Vec<String>,
) -> Result<Vec<ThumbnailItem>, String> {
    let result = VideoFileManager::new(Box::new(FileManagerForHardDrive { app }))
        .file_manager
        .retrieve_files_in(paths);
    CollectionService::create_collection(result.clone().ok());
    result
}

trait FileManager {
    fn retrieve_files_in(&self, paths: Vec<String>) -> Result<Vec<ThumbnailItem>, String>;
}

struct FileManagerForHardDrive {
    app: AppHandle,
}

impl FileManager for FileManagerForHardDrive {
    fn retrieve_files_in(&self, paths: Vec<String>) -> Result<Vec<ThumbnailItem>, String> {
        if paths.is_empty() {
            return Err("Aucun fichier reçu".to_string());
        }

        let cache_dir = self.app.path().app_cache_dir().map_err(|e| e.to_string())?;

        let total = paths.len();
        let mut results = Vec::with_capacity(total);

        for (index, p) in paths.iter().enumerate() {
            let video_path = PathBuf::from(p);
            let thumb_path = make_thumbnail_path(&cache_dir, &video_path);
            let size_bytes = std::fs::metadata(&video_path).map(|m| m.len()).ok();

            let mut item = ThumbnailItem {
                video_path: p.clone(),
                thumbnail_path: None,
                error: None,
                size_bytes,
            };

            if thumb_path.exists() {
                item.thumbnail_path = Some(thumb_path.to_string_lossy().to_string());
            } else {
                match generate_one_thumbnail_ffmpeg(&video_path, &thumb_path) {
                    Ok(()) => item.thumbnail_path = Some(thumb_path.to_string_lossy().to_string()),
                    Err(e) => item.error = Some(e),
                }
            }

            let progress = ThumbnailProgress {
                index: index + 1,
                total,
                video_path: item.video_path.clone(),
                thumbnail_path: item.thumbnail_path.clone(),
                error: item.error.clone(),
                size_bytes: item.size_bytes,
            };
            let _ = self.app.emit("thumbnail:progress", progress);

            results.push(item);
        }
        Ok(results)
    }
}

struct VideoFileManager {
    file_manager: Box<dyn FileManager>,
}

impl VideoFileManager {
    pub fn new(file_manager: Box<dyn FileManager>) -> Self {
        Self { file_manager }
    }
}
