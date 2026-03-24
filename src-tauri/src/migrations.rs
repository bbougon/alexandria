use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct VideoDurationMigration;

impl VideoDurationMigration {
    pub fn run(base_dir: PathBuf) {
        if !base_dir.exists() {
            return;
        }

        let entries = match fs::read_dir(&base_dir) {
            Ok(entries) => entries,
            Err(e) => {
                log::error!("Failed to read directory {base_dir:?}: {e}");
                return;
            }
        };

        for entry in entries {
            let entry = match entry {
                Ok(entry) => entry,
                Err(e) => {
                    log::error!("Failed to read directory entry: {e}");
                    continue;
                }
            };

            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
                continue;
            }

            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(mut json) = serde_json::from_str::<serde_json::Value>(&content) {
                    let mut updated = false;
                    if let Some(videos) = json.get_mut("videos").and_then(|v| v.as_array_mut()) {
                        for video in videos {
                            let duration = video
                                .get("duration_seconds")
                                .and_then(|d| d.as_u64())
                                .unwrap_or(0);
                            if duration == 0 {
                                if let Some(video_path_str) =
                                    video.get("path").and_then(|p| p.as_str())
                                {
                                    let video_path = Path::new(video_path_str);
                                    match get_video_duration_ffprobe(video_path) {
                                        Ok(new_duration) => {
                                            video["duration_seconds"] =
                                                serde_json::Value::from(new_duration);
                                            updated = true;
                                        }
                                        Err(e) => {
                                            log::error!(
                                                "Failed to migrate duration for video {:?}: {}",
                                                video_path,
                                                e
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }

                    if updated {
                        if let Ok(updated_content) = serde_json::to_string_pretty(&json) {
                            if let Err(e) = fs::write(&path, updated_content) {
                                log::error!("Failed to write updated collection to {path:?}: {e}");
                            }
                        }
                    }
                }
            }
        }
    }
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

fn get_ffprobe_command() -> Command {
    let command = "ffprobe";
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
