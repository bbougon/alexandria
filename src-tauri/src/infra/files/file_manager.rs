use crate::collections::collections::Video;
use crate::collections::video::FileManager;
use base64::Engine;
use std::path::{Path, PathBuf};
use std::process::Command;
use tauri::{AppHandle, Emitter};

fn generate_one_thumbnail_ffmpeg(video_path: &Path) -> Result<String, String> {
    let output = Command::new("ffmpeg")
        .args([
            "-hide_banner",
            "-loglevel",
            "error",
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
            "-f",
            "image2pipe",
            "-vcodec",
            "mjpeg",
            "pipe:1",
        ])
        .output()
        .map_err(|e| format!("Impossible de lancer ffmpeg: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(format!("ffmpeg a échoué: {stderr}"));
    }

    let b64 = base64::engine::general_purpose::STANDARD.encode(output.stdout);

    Ok(format!("data:image/jpeg;base64,{}", b64))
}

pub struct FileManagerForHardDrive {
    app: AppHandle,
}

impl FileManagerForHardDrive {
    pub fn new(app: AppHandle) -> Self {
        Self { app: app.clone() }
    }

    fn create_thumbnail(video_path: &PathBuf) -> Result<Option<String>, String> {
        match generate_one_thumbnail_ffmpeg(&video_path) {
            Ok(base_64_image) => Ok(Some(base_64_image)),
            Err(e) => {
                log::error!("Failed to generate thumbnail {video_path:?}: {e}");
                Err("Failed".to_string())
            }
        }
    }
}

impl FileManager for FileManagerForHardDrive {
    fn create_video(&self, path: &str) -> Result<Video, String> {
        let video_path = PathBuf::from(path);
        let size_bytes = std::fs::metadata(&video_path).map(|m| m.len()).ok();
        let thumbnail = Self::create_thumbnail(&video_path)?;
        let video = Video::new(video_path, thumbnail.unwrap(), size_bytes.unwrap());
        let _ = self.app.emit("thumbnail:progress", video.clone());
        Ok(video)
    }
}
