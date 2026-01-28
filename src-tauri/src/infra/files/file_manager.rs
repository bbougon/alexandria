use crate::collections::video::{FileManager, ThumbnailItem};
use std::path::{Path, PathBuf};
use std::process::Command;
use tauri::{AppHandle, Emitter, Manager};

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

pub struct FileManagerForHardDrive {
    app: AppHandle,
}

impl FileManagerForHardDrive {
    pub fn new(app: AppHandle) -> Self {
        Self { app }
    }
}

impl FileManager for FileManagerForHardDrive {
    fn retrieve_files_in(&self, paths: Vec<String>) -> Result<Vec<ThumbnailItem>, String> {
        if paths.is_empty() {
            return Err("Aucun fichier reçu".to_string());
        }

        let cache_dir = self.app.path().app_cache_dir().map_err(|e| e.to_string())?;

        let total = paths.len();
        let mut results = Vec::with_capacity(total);

        for (_index, p) in paths.iter().enumerate() {
            let video_path = PathBuf::from(p);
            let thumb_path = make_thumbnail_path(&cache_dir, &video_path);
            let size_bytes = std::fs::metadata(&video_path).map(|m| m.len()).ok();

            let mut item = ThumbnailItem {
                video_path: p.clone(),
                thumbnail_path: None,
                size_bytes,
            };

            if thumb_path.exists() {
                item.thumbnail_path = Some(thumb_path.to_string_lossy().to_string());
            } else {
                match generate_one_thumbnail_ffmpeg(&video_path, &thumb_path) {
                    Ok(()) => item.thumbnail_path = Some(thumb_path.to_string_lossy().to_string()),
                    Err(e) => log::error!("Failed to generate thumbnail {video_path:?}: {e}"),
                }
            }

            let _ = self.app.emit("thumbnail:progress", item.clone());

            results.push(item);
        }
        Ok(results)
    }
}
