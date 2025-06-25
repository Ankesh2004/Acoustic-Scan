use std::env;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

use urlencoding::encode;

use crate::db; // Assumes your db module is available.
// use crate::models::Song; // Assumes your models module defines Song-related types.

/// Encodes a parameter for URL usage.
pub fn encode_param(s: &str) -> String {
    encode(s).into_owned()
}

/// Converts a string to lowercase.
/// This implementation uses Rust's built-in functionality.
pub fn to_lower_case(s: &str) -> String {
    s.to_lowercase()
}

/// Returns the file size in bytes for the given file path.
pub fn get_file_size(file: &str) -> io::Result<u64> {
    let metadata = fs::metadata(file)?;
    Ok(metadata.len())
}

/// Checks if a song with the given key already exists in the database.
pub async fn song_key_exists(key: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let mut db_client = db::new_db_client().await?;
    let (_song, exists) = db_client.get_song_by_key(key)?;
    db_client.close();
    Ok(exists)
}

/// Checks if a song with the given YouTube ID already exists in the database.
pub async fn yt_id_exists(yt_id: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let mut db_client = db::new_db_client().await?;
    let (_song, exists) = db_client.get_song_by_ytid(yt_id)?;
    db_client.close();
    Ok(exists)
}

/// Fixes some invalid file names (particularly for Windows).
/// On Windows, removes characters that are not allowed in filenames.
/// On other systems, replaces forward slashes with backslashes.
pub fn correct_filename(title: &str, artist: &str) -> (String, String) {
    #[cfg(target_os = "windows")]
    {
        // List of invalid characters for Windows.
        let invalid_chars = ['<', '>', ':', '"', '\\', '/', '|', '?', '*'];
        let mut fixed_title = title.to_string();
        let mut fixed_artist = artist.to_string();
        for ch in invalid_chars.iter() {
            fixed_title = fixed_title.replace(*ch, "");
            fixed_artist = fixed_artist.replace(*ch, "");
        }
        (fixed_title, fixed_artist)
    }
    #[cfg(not(target_os = "windows"))]
    {
        (
            title.replace("/", "\\"),
            artist.replace("/", "\\"),
        )
    }
}

/// Converts a stereo audio file to mono by using ffprobe to check the number of channels
/// and, if necessary, invoking ffmpeg to perform the conversion. Returns the audio bytes.
pub fn convert_stereo_to_mono(stereo_file_path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let stereo_path = Path::new(stereo_file_path);
    let file_ext = stereo_path.extension().and_then(|s| s.to_str()).unwrap_or("");
    let mut mono_file_path = stereo_path.with_file_name(
        format!("{}{}_mono.{}", 
            stereo_path.file_stem().and_then(|s| s.to_str()).unwrap_or(""),
            "",
            file_ext
        )
    );
    // Ensure temporary file removal at the end.
    let cleanup = || {
        let _ = fs::remove_file(&mono_file_path);
    };

    // Check number of channels using ffprobe.
    let ffprobe_output = Command::new("ffprobe")
        .args(&[
            "-v", "error",
            "-show_entries", "stream=channels",
            "-of", "default=noprint_wrappers=1:nokey=1",
            stereo_file_path,
        ])
        .output()?;
    if !ffprobe_output.status.success() {
        cleanup();
        return Err(format!(
            "error getting number of channels: {}",
            String::from_utf8_lossy(&ffprobe_output.stdout)
        ).into());
    }
    let channels = String::from_utf8_lossy(&ffprobe_output.stdout).trim().to_string();

    // Read the original audio bytes.
    let mut audio_bytes = fs::read(stereo_file_path)
        .map_err(|e| format!("error reading stereo file: {}", e))?;

    if channels != "1" {
        // Convert stereo to mono using ffmpeg.
        let ffmpeg_status = Command::new("ffmpeg")
            .args(&[
                "-i", stereo_file_path,
                "-af", "pan=mono|c0=c0",
                mono_file_path.to_str().unwrap(),
            ])
            .status()?;
        if !ffmpeg_status.success() {
            cleanup();
            return Err(format!("error converting stereo to mono: {}", ffmpeg_status).into());
        }
        // Read the mono file.
        audio_bytes = fs::read(&mono_file_path)
            .map_err(|e| format!("error reading mono file: {}", e))?;
    }
    cleanup();
    Ok(audio_bytes)
}
