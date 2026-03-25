use crate::collections::collections::{Collection, Style, Video};
use crate::infra::files::file_manager::VideoData;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(serde::Serialize, Clone)]
pub struct VideoAddedToCollection {
    pub collection_id: Uuid,
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

impl From<(&Video, Uuid)> for VideoAddedToCollection {
    fn from((video, collection_id): (&Video, Uuid)) -> Self {
        Self {
            collection_id,
            path: video.path.clone(),
            name: video.name.clone(),
            artist: video.artist.clone(),
            song: video.song.clone(),
            style: video.style.clone(),
            tags: video.tags.clone(),
            thumbnail: video.thumbnail.clone(),
            size_bytes: video.size_bytes,
            duration_seconds: video.duration_seconds,
        }
    }
}

#[derive(serde::Serialize, Clone)]
pub struct VideoDataRetrieved {
    pub path: PathBuf,
    pub thumbnail: String,
    pub size_bytes: u64,
    pub duration_seconds: u64,
}

impl From<&VideoData> for VideoDataRetrieved {
    fn from(video_data: &VideoData) -> Self {
        Self {
            path: video_data.path.clone(),
            thumbnail: video_data.thumbnail.clone(),
            size_bytes: video_data.size_bytes,
            duration_seconds: video_data.duration_seconds,
        }
    }
}

#[derive(serde::Serialize, Clone)]
pub struct CollectionCreated {
    pub collection_id: Uuid,
    pub title: String,
    pub videos: Vec<Video>,
}

impl From<&Collection> for CollectionCreated {
    fn from(collection: &Collection) -> Self {
        Self {
            collection_id: collection.id,
            title: collection.title.clone(),
            videos: vec![],
        }
    }
}
