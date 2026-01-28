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
}

impl CollectionRepository for CollectionRepositoryFile {
    fn list(&self) -> Vec<Collection> {
        let collections_dir = self.base_dir.join("collections");
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
        let now = Local::now();
        let file_name = format!("collection-{}.json", now.format("%Y-%m-%d_%H-%M-%S"));
        let path = self.base_dir.join(file_name);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Impossible de créer le dossier {parent:?}: {e}"))
                .unwrap();
        }
        let file = match File::create(&path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Impossible de créer {path:?}: {e}");
                return;
            }
        };
        let writer = BufWriter::new(file);

        if let Err(e) = serde_json::to_writer_pretty(writer, &_c) {
            eprintln!("Impossible d'écrire le JSON: {e}");
        }
    }

    fn get_by_id(&self, id: &Uuid) -> Option<Collection> {
        self.list().iter().find(|c| c.id == *id).cloned()
    }
}

pub fn init_prod() {
    let file_repo = CollectionRepositoryFile::new(PathBuf::from("./data/collections/"));
    set_repositories(Repositories::new(Arc::new(file_repo)));
}
