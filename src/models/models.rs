use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Couple {
    pub anchor_time_ms: u32,
    pub song_id: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordData {
    pub audio: String,
    pub duration: f64,
    pub channels: i32,
    pub sample_rate: i32,
    pub sample_size: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track{
    pub title: String,
    pub artist: String,
    pub album: String,
    pub artists: Vec<String>,
    pub duration: f64,
}