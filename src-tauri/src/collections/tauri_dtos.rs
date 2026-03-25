use crate::infra::files::file_manager::VideoData;
use std::path::PathBuf;

#[derive(serde::Serialize, Clone)]
pub struct VideoDataDTO {
    pub path: PathBuf,
    pub thumbnail: String,
    pub size_bytes: u64,
    pub duration_seconds: u64,
}

impl From<&VideoData> for VideoDataDTO {
    fn from(video_data: &VideoData) -> Self {
        Self {
            path: video_data.path.clone(),
            thumbnail: video_data.thumbnail.clone(),
            size_bytes: video_data.size_bytes,
            duration_seconds: video_data.duration_seconds,
        }
    }
}
