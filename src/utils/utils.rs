use rand::Rng;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

pub const SONGS_DIR: &str = "songs";

/// Generates a unique ID based on a random u32 value.
pub fn generate_unique_id() -> u32 {
    // Using thread_rng ensures a properly seeded RNG.
    let mut rng = rand::rng();
    rng.random::<u32>()
}

/// Generates a song key by concatenating the song title and song artist with "---" as separator.
pub fn generate_song_key(song_title: &str, song_artist: &str) -> String {
    format!("{}---{}", song_title, song_artist)
}

/// Returns the value of the environment variable `key`.
/// If the variable is not set, returns the provided fallback value or an empty string if no fallback is provided.
pub fn get_env(key: &str, fallback: Option<&str>) -> String {
    env::var(key).unwrap_or_else(|_| fallback.unwrap_or("").to_string())
}

use std::error::Error;
use std::fs;
use std::fmt;

#[derive(Debug)]
pub struct WrappedError {
    message: String,
    source: Box<dyn Error + Send + Sync>,
}

impl fmt::Display for WrappedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.message, self.source)
    }
}

impl Error for WrappedError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&*self.source)
    }
}

// pub fn wrap_error<E: Error + 'static>(error: E) -> Box<dyn Error> {
//     Box::new(WrappedError {
//         message: "Operation failed".to_string(),
//         source: Box::new(error),
//     })
// }
pub fn wrap_error<E: Error + Send + Sync + 'static>(error: E) -> Box<dyn Error + Send + Sync> {
    Box::new(WrappedError {
        message: "Operation failed".to_string(),
        source: Box::new(error),
    })
}