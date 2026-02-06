use crate::collections::collections::{Collection, CollectionRepository};
use crate::repositories::{set_repositories, Repositories};
use chrono::Local;
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;
use std::sync::Arc;
use uuid::Uuid;

pub struct CollectionRepositoryFile {
    base_dir: PathBuf,
}

impl CollectionRepositoryFile {
    pub fn new(base_dir: PathBuf) -> Self {
        Self { base_dir }
    }

    fn find_collection_file_path(&self, id: &Uuid) -> Option<PathBuf> {
        let collections_dir = self.base_dir.clone();
        if !collections_dir.exists() {
            return None;
        }

        let entries = match fs::read_dir(&collections_dir) {
            Ok(entries) => entries,
            Err(e) => {
                log::error!("Failed to read directory {collections_dir:?}: {e}");
                return None;
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
                if let Ok(collection) = serde_json::from_str::<Collection>(&content) {
                    if collection.id == *id {
                        return Some(path);
                    }
                }
            }
        }

        None
    }
}

impl CollectionRepository for CollectionRepositoryFile {
    fn list(&self) -> Vec<Collection> {
        let collections_dir = self.base_dir.clone();
        if !collections_dir.exists() {
            return Vec::new();
        }

        let entries = match fs::read_dir(&collections_dir) {
            Ok(entries) => entries,
            Err(e) => {
                log::error!("Failed to read directory {collections_dir:?}: {e}");
                return Vec::new();
            }
        };

        let mut collections = Vec::new();
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

            match fs::read_to_string(&path) {
                Ok(content) => match serde_json::from_str::<Collection>(&content) {
                    Ok(collection) => collections.push(collection),
                    Err(e) => log::error!("Failed to parse JSON from {path:?}: {e}"),
                },
                Err(e) => log::error!("Failed to read file {path:?}: {e}"),
            }
        }

        collections
    }

    fn add(&self, _c: Collection) {
        let existing_path = if let Some(_existing) = self.get_by_id(&_c.id) {
            self.find_collection_file_path(&_c.id)
        } else {
            None
        };

        let path = if let Some(existing_path) = existing_path {
            existing_path
        } else {
            let now = Local::now();
            let file_name = format!("collection-{}.json", now.format("%Y-%m-%d_%H-%M-%S"));
            self.base_dir.join(file_name)
        };

        if let Some(parent) = path.parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                log::error!("Impossible de créer le dossier {parent:?}: {e}");
                return;
            }
        }

        let file = match File::create(&path) {
            Ok(f) => f,
            Err(e) => {
                log::error!("Impossible de créer {path:?}: {e}");
                return;
            }
        };
        let writer = BufWriter::new(file);

        if let Err(e) = serde_json::to_writer_pretty(writer, &_c) {
            log::error!("Impossible d'écrire le JSON: {e}");
        }
    }

    fn get_by_id(&self, id: &Uuid) -> Option<Collection> {
        self.list().into_iter().find(|c| c.id == *id)
    }
}

pub fn init_prod(base_dir: PathBuf) {
    let file_repo = CollectionRepositoryFile::new(base_dir);
    set_repositories(Repositories::new(Arc::new(file_repo)));
}
