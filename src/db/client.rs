use std::collections::HashMap;
use std::error::Error;

use crate::models;
use crate::utils;
use serde::Serialize;

/// The DBClient trait defines the interface for database operations.
pub trait DBClient {
    fn close(&mut self) -> Result<(), Box<dyn Error>>;
    fn store_fingerprints(&mut self, fingerprints: &HashMap<u32, models::Couple>) -> Result<(), Box<dyn Error>>;
    fn get_couples(&self, addresses: &[u32]) -> Result<HashMap<u32, Vec<models::Couple>>, Box<dyn Error>>;
    fn total_songs(&self) -> Result<i32, Box<dyn Error>>;
    fn register_song(&mut self, song_title: &str, song_artist: &str, yt_id: &str) -> Result<u32, Box<dyn Error>>;
    fn get_song(&self, filter_key: &str, value: &str) -> Result<(Song, bool), Box<dyn Error>>;
    fn get_song_by_id(&self, song_id: u32) -> Result<(Song, bool), Box<dyn Error>>;
    fn get_song_by_ytid(&self, yt_id: &str) -> Result<(Song, bool), Box<dyn Error>>;
    fn get_song_by_key(&self, key: &str) -> Result<(Song, bool), Box<dyn Error>>;
    fn delete_song_by_id(&mut self, song_id: u32) -> Result<(), Box<dyn Error>>;
    fn delete_collection(&mut self, collection_name: &str) -> Result<(), Box<dyn Error>>;
    fn get_all_songs(&self) -> Result<Vec<LibrarySong>, Box<dyn Error>>;
}
#[derive(Debug, Clone, Serialize)]
pub struct LibrarySong {
    pub id: u32,
    pub name: String,
    pub artist: String,
    pub youtube_id: String,
    pub thumbnail_url: Option<String>,
}
/// A simple Song struct with title, artist, and YouTubeID.
#[derive(Debug, Clone)]
pub struct Song {
    pub title: String,
    pub artist: String,
    pub youtube_id: String,
}
// impl Default for Song {
//     fn default() -> Self {
//         Song {
//             title: String::new(),
//             artist: String::new(),
//             youtube_id: String::new(),
//         }
//     }
// }


/// Creates a new database client based on the environment variable "DB_TYPE".
/// Supported types are "mongo" and "sqlite". If not set, defaults to "sqlite".
pub fn new_db_client() -> Result<Box<dyn DBClient>, Box<dyn Error>> {
    let db_type = utils::get_env("DB_TYPE", Some("sqlite"));
    match db_type.as_str() {
        "mongo" => {
            let db_username = utils::get_env("DB_USER", None);
            let db_password = utils::get_env("DB_PASS", None);
            let db_name = utils::get_env("DB_NAME", None);
            let db_host = utils::get_env("DB_HOST", None);
            let db_port = utils::get_env("DB_PORT", None);
            let db_uri = if db_username.is_empty() || db_password.is_empty() {
                "mongodb://localhost:27017".to_string()
            } else {
                format!("mongodb://{}:{}@{}:{}/{}", db_username, db_password, db_host, db_port, db_name)
            };
            new_mongo_client(&db_uri)
        },
        "sqlite" => {
            new_sqlite_client("db.sqlite3")
        },
        other => Err(format!("unsupported database type: {}", other).into()),
    }
}

/// Placeholder function for creating a MongoDB client.
/// Replace with an actual implementation as needed.
fn new_mongo_client(db_uri: &str) -> Result<Box<dyn DBClient>, Box<dyn Error>> {
    Err(format!("MongoDB client not implemented. db_uri: {}", db_uri).into())
}

/// Placeholder function for creating a SQLite client.
/// Replace with an actual implementation as needed.
fn new_sqlite_client(db_file: &str) -> Result<Box<dyn DBClient>, Box<dyn Error>> {
    Err(format!("SQLite client not implemented. db_file: {}", db_file).into())
}
