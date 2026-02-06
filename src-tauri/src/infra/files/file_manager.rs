use crate::collections::collections::Video;
use crate::collections::video::FileManager;
use base64::Engine;
use std::path::{Path, PathBuf};
use std::process::Command;

fn get_ffmpeg_command() -> Command {
    #[cfg(target_os = "windows")]
    let paths = ["ffmpeg", "ffmpeg.exe"];

    #[cfg(target_os = "macos")]
    let paths = [
        "ffmpeg",
        "/opt/homebrew/bin/ffmpeg",
        "/usr/local/bin/ffmpeg",
    ];

    #[cfg(target_os = "linux")]
    let paths = [
        "ffmpeg",
        "/usr/bin/ffmpeg",
        "/usr/local/bin/ffmpeg",
        "/snap/bin/ffmpeg",
    ];

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    let paths = ["ffmpeg"];

    for path in paths {
        if Command::new(path).arg("-version").output().is_ok() {
            return Command::new(path);
        }
    }

    Command::new("ffmpeg")
}

fn generate_one_thumbnail_ffmpeg(video_path: &Path) -> Result<String, String> {
    let output = get_ffmpeg_command()
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

pub struct FileManagerForHardDrive {}

impl FileManagerForHardDrive {
    pub fn new() -> Self {
        Self {}
    }

    fn create_thumbnail(video_path: &PathBuf) -> Result<Option<String>, String> {
        match generate_one_thumbnail_ffmpeg(&video_path) {
            Ok(base_64_image) => Ok(Some(base_64_image)),
            Err(e) => {
                log::error!("Failed to generate thumbnail {video_path:?}: {e}");
                Ok(None)
            }
        }
    }
}

impl FileManager for FileManagerForHardDrive {
    fn create_video(&self, path: &str) -> Result<Video, String> {
        let video_path = PathBuf::from(path);
        let size_bytes = std::fs::metadata(&video_path).map(|m| m.len()).unwrap_or(0);
        let thumbnail = Self::create_thumbnail(&video_path)?.unwrap_or_default();
        let video = Video::new(video_path, thumbnail, size_bytes);
        Ok(video)
    }
}
