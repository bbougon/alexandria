use crate::collections::collections::Video;
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
    cache_dir: PathBuf,
}

impl FileManagerForHardDrive {
    pub fn new(app: AppHandle) -> Self {
        let result = app
            .path()
            .app_cache_dir()
            .map_err(|e| e.to_string())
            .unwrap();
        Self {
            app: app.clone(),
            cache_dir: result,
        }
    }

    fn create_thumbnail(
        video_path: &PathBuf,
        thumb_path: &PathBuf,
    ) -> Result<Option<String>, String> {
        if thumb_path.exists() {
            Ok(Some(thumb_path.to_string_lossy().to_string()))
        } else {
            match generate_one_thumbnail_ffmpeg(&video_path, &thumb_path) {
                Ok(()) => Ok(Some(thumb_path.to_string_lossy().to_string())),
                Err(e) => {
                    log::error!("Failed to generate thumbnail {video_path:?}: {e}");
                    Err("Failed".to_string())
                }
            }
        }
    }
}

impl FileManager for FileManagerForHardDrive {
    fn create_video(&self, path: &str) -> Result<Video, String> {
        let video_path = PathBuf::from(path);
        let thumb_path = make_thumbnail_path(&self.cache_dir, &video_path);
        let size_bytes = std::fs::metadata(&video_path).map(|m| m.len()).ok();
        let thumbnail_path = Self::create_thumbnail(&video_path, &thumb_path)?;
        let video = Video::new(video_path, thumbnail_path.unwrap(), size_bytes.unwrap());
        let _ = self.app.emit("thumbnail:progress", video.clone());
        Ok(video)
    }
}
