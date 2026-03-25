use crate::collections::collections::Style;
use crate::infra::files::file_manager::VideoData;
use std::path::PathBuf;

#[derive(serde::Serialize, Clone)]
pub struct ThumbnailItem {
    pub video_path: String,
    pub thumbnail: Option<String>,
    pub size_bytes: Option<u64>,
    pub duration_seconds: Option<u64>,
}

#[derive(serde::Serialize, Clone)]
pub struct VideoAddedToCollection {
    pub collection_id: uuid::Uuid,
    pub path: PathBuf,
    pub name: String,
    pub artist: String,
    pub song: String,
    pub style: Vec<Style>,
    pub tags: Vec<String>,
    pub thumbnail: String,
    pub size_bytes: u64,
    pub duration_seconds: u64,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct VideoCollectionToUpdate {
    pub collection_id: uuid::Uuid,
    pub video: VideoToUpdate,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct VideoToUpdate {
    pub path: PathBuf,
    pub name: String,
    pub artist: String,
    pub song: String,
    pub style: Vec<Style>,
    pub tags: Vec<String>,
    pub thumbnail: String,
    pub size_bytes: u64,
    pub duration_seconds: u64,
}

pub trait FileManager {
    fn retrieve_all_videos_data(&self, paths: Vec<String>) -> Result<Vec<VideoData>, String> {
        if paths.is_empty() {
            return Err("Aucun fichier reçu".to_string());
        }

        let mut result = vec![];
        for (_index, p) in paths.iter().enumerate() {
            // TODO: Emit an event when video data is retrieved
            let video_data = self.retrieve_video_data(p)?;
            result.push(video_data.clone());
        }
        Ok(result)
    }

    fn retrieve_video_data(&self, path: &str) -> Result<VideoData, String>;
}

pub struct VideoFileManager {
    pub file_manager: Box<dyn FileManager>,
}

impl VideoFileManager {
    pub fn new(file_manager: Box<dyn FileManager>) -> Self {
        Self { file_manager }
    }
}
