use crate::collections::video::FileManager;
use base64::Engine;
use std::path::{Path, PathBuf};
use std::process::Command;

fn get_ffmpeg_command() -> Command {
    get_command("ffmpeg")
}

fn get_ffprobe_command() -> Command {
    get_command("ffprobe")
}

fn get_command(command: &str) -> Command {
    #[cfg(target_os = "windows")]
    let paths = [command, &format!("{}.exe", command)];

    #[cfg(target_os = "macos")]
    let paths = [
        command,
        &format!("/opt/homebrew/bin/{}", command),
        &format!("/usr/local/bin/{}", command),
    ];

    #[cfg(target_os = "linux")]
    let paths = [
        command,
        &format!("/usr/bin/{}", command),
        &format!("/usr/local/bin/{}", command),
        &format!("/snap/bin/{}", command),
    ];

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    let paths = [command];

    for path in paths {
        if Command::new(path).arg("-version").output().is_ok() {
            return Command::new(path);
        }
    }

    Command::new(command)
}

pub struct FfmpegMetadata {
    pub thumbnail: String,
    pub duration_seconds: u64,
}

fn get_video_duration_ffprobe(video_path: &Path) -> Result<u64, String> {
    let output = get_ffprobe_command()
        .args([
            "-v",
            "error",
            "-show_entries",
            "format=duration",
            "-of",
            "default=noprint_wrappers=1:nokey=1",
            video_path.to_string_lossy().as_ref(),
        ])
        .output()
        .map_err(|e| format!("Impossible de lancer ffprobe: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(format!("ffprobe a échoué: {stderr}"));
    }

    let duration_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let duration = duration_str
        .parse::<f64>()
        .map_err(|e| format!("Impossible de parser la durée {}: {}", duration_str, e))?;

    Ok(duration as u64)
}

fn generate_one_thumbnail_ffmpeg(video_path: &Path) -> Result<FfmpegMetadata, String> {
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
    let thumbnail = format!("data:image/jpeg;base64,{}", b64);

    let duration_seconds = get_video_duration_ffprobe(video_path)?;

    Ok(FfmpegMetadata {
        thumbnail,
        duration_seconds,
    })
}

pub struct FileManagerForHardDrive {}

impl FileManagerForHardDrive {
    pub fn new() -> Self {
        Self {}
    }

    fn create_thumbnail(video_path: &PathBuf) -> Result<Option<FfmpegMetadata>, String> {
        match generate_one_thumbnail_ffmpeg(&video_path) {
            Ok(metadata) => Ok(Some(metadata)),
            Err(e) => {
                log::error!("Failed to generate thumbnail {video_path:?}: {e}");
                Ok(None)
            }
        }
    }
}

#[derive(Clone)]
pub struct VideoData {
    pub path: PathBuf,
    pub thumbnail: String,
    pub size_bytes: u64,
    pub duration_seconds: u64,
}

impl FileManager for FileManagerForHardDrive {
    fn retrieve_video_data(&self, path: &str) -> Result<VideoData, String> {
        let video_path = PathBuf::from(path);
        let size_bytes = std::fs::metadata(&video_path).map(|m| m.len()).unwrap_or(0);
        let metadata = Self::create_thumbnail(&video_path)?;
        let thumbnail = metadata
            .as_ref()
            .map(|m| m.thumbnail.clone())
            .unwrap_or_default();
        let duration_seconds = metadata.as_ref().map(|m| m.duration_seconds).unwrap_or(0);
        let video_data: VideoData = VideoData {
            path: video_path.clone(),
            thumbnail: thumbnail.clone(),
            size_bytes,
            duration_seconds,
        };
        Ok(video_data)
    }
}
