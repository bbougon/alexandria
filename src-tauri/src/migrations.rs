use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub trait Migration: Send + Sync {
    fn id(&self) -> &str;
    fn run(&self, base_dir: &Path) -> Result<(), String>;
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct MigrationStatus {
    pub id: String,
    pub played_at: String,
}

pub trait MigrationRepository: Send + Sync {
    fn list(&self) -> Vec<MigrationStatus>;
    fn add(&self, status: MigrationStatus);
    fn exists(&self, id: &str) -> bool {
        self.list().iter().any(|m| m.id == id)
    }
}

pub struct MigrationRepositoryFile {
    path: PathBuf,
}

impl MigrationRepositoryFile {
    pub fn new(base_dir: PathBuf) -> Self {
        Self {
            path: base_dir.join("migrations.json"),
        }
    }
}

impl MigrationRepository for MigrationRepositoryFile {
    fn list(&self) -> Vec<MigrationStatus> {
        if !self.path.exists() {
            return Vec::new();
        }
        match fs::read_to_string(&self.path) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
            Err(e) => {
                log::error!("Failed to read migrations file {:?}: {}", self.path, e);
                Vec::new()
            }
        }
    }

    fn add(&self, status: MigrationStatus) {
        let mut list = self.list();
        if !list.iter().any(|m| m.id == status.id) {
            list.push(status);
            if let Ok(content) = serde_json::to_string_pretty(&list) {
                if let Err(e) = fs::write(&self.path, content) {
                    log::error!("Failed to write migrations file {:?}: {}", self.path, e);
                }
            }
        }
    }
}

pub struct MigrationManager {
    repository: Box<dyn MigrationRepository>,
    base_dir: PathBuf,
}

impl MigrationManager {
    pub fn new(repository: Box<dyn MigrationRepository>, base_dir: PathBuf) -> Self {
        Self {
            repository,
            base_dir,
        }
    }

    pub fn play(&self, migrations: Vec<Box<dyn Migration>>) -> Result<(), String> {
        for migration in migrations {
            if !self.repository.exists(migration.id()) {
                migration.run(&self.base_dir)?;
                self.repository.add(MigrationStatus {
                    id: migration.id().to_string(),
                    played_at: chrono::Utc::now().to_rfc3339(),
                });
            }
        }
        Ok(())
    }
}

pub struct VideoDurationMigration;

impl Migration for VideoDurationMigration {
    fn id(&self) -> &str {
        "video_duration_migration_001"
    }

    fn run(&self, base_dir: &Path) -> Result<(), String> {
        if !base_dir.exists() {
            return Ok(());
        }

        let entries = fs::read_dir(&base_dir)
            .map_err(|e| format!("Failed to read directory {:?}: {}", base_dir, e))?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;

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
                            fs::write(&path, updated_content).map_err(|e| {
                                format!("Failed to write updated collection to {:?}: {}", path, e)
                            })?;
                        }
                    }
                }
            }
        }
        Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::Mutex;

    struct MockMigration {
        id: String,
        run_count: Arc<Mutex<usize>>,
    }

    impl Migration for MockMigration {
        fn id(&self) -> &str {
            &self.id
        }
        fn run(&self, _base_dir: &Path) -> Result<(), String> {
            let mut count = self.run_count.lock().unwrap();
            *count += 1;
            Ok(())
        }
    }

    struct MemoryMigrationRepository {
        statuses: Mutex<Vec<MigrationStatus>>,
    }

    impl MigrationRepository for MemoryMigrationRepository {
        fn list(&self) -> Vec<MigrationStatus> {
            self.statuses.lock().unwrap().clone()
        }
        fn add(&self, status: MigrationStatus) {
            self.statuses.lock().unwrap().push(status);
        }
    }

    #[test]
    fn plays_one_migration() {
        let repo = Box::new(MemoryMigrationRepository {
            statuses: Mutex::new(Vec::new()),
        });
        let manager = MigrationManager::new(repo, PathBuf::from("/tmp"));
        let run_count = Arc::new(Mutex::new(0));
        let migration = Box::new(MockMigration {
            id: "m1".to_string(),
            run_count: run_count.clone(),
        });

        manager.play(vec![migration]).unwrap();

        assert_eq!(*run_count.lock().unwrap(), 1);
        assert_eq!(manager.repository.list().len(), 1);
        assert_eq!(manager.repository.list()[0].id, "m1");
    }

    #[test]
    fn plays_all_existing_migrations() {
        let repo = Box::new(MemoryMigrationRepository {
            statuses: Mutex::new(Vec::new()),
        });
        let manager = MigrationManager::new(repo, PathBuf::from("/tmp"));
        let run_count1 = Arc::new(Mutex::new(0));
        let run_count2 = Arc::new(Mutex::new(0));
        let m1 = Box::new(MockMigration {
            id: "m1".to_string(),
            run_count: run_count1.clone(),
        });
        let m2 = Box::new(MockMigration {
            id: "m2".to_string(),
            run_count: run_count2.clone(),
        });

        manager.play(vec![m1, m2]).unwrap();

        assert_eq!(*run_count1.lock().unwrap(), 1);
        assert_eq!(*run_count2.lock().unwrap(), 1);
        assert_eq!(manager.repository.list().len(), 2);
    }

    #[test]
    fn plays_only_migrations_not_yet_played() {
        let repo = Box::new(MemoryMigrationRepository {
            statuses: Mutex::new(vec![MigrationStatus {
                id: "m1".to_string(),
                played_at: chrono::Utc::now().to_rfc3339(),
            }]),
        });
        let manager = MigrationManager::new(repo, PathBuf::from("/tmp"));
        let run_count1 = Arc::new(Mutex::new(0));
        let run_count2 = Arc::new(Mutex::new(0));
        let m1 = Box::new(MockMigration {
            id: "m1".to_string(),
            run_count: run_count1.clone(),
        });
        let m2 = Box::new(MockMigration {
            id: "m2".to_string(),
            run_count: run_count2.clone(),
        });

        manager.play(vec![m1, m2]).unwrap();

        assert_eq!(
            *run_count1.lock().unwrap(),
            0,
            "m1 should not have been played"
        );
        assert_eq!(*run_count2.lock().unwrap(), 1, "m2 should have been played");
        assert_eq!(manager.repository.list().len(), 2);
    }
}
