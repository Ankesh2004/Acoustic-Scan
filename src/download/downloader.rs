use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use std::time::Duration;

use num_cpus;
use tokio::sync::Semaphore;

use crate::db;
use crate::shazam;
use crate::utils;
use crate::wav;
use crate::models::Track; // Assume Track is defined in your models module

const DELETE_SONG_FILE: bool = false;

pub fn dl_single_track(url: &str, save_path: &str) -> Result<i32, Box<dyn Error>> {
    let track_info = track_info(url)?;
    println!("Getting track info...");
    thread::sleep(Duration::from_millis(500));
    let tracks = vec![track_info];
    println!("Now, downloading track...");
    let total = dl_track(&tracks, save_path)?;
    Ok(total)
}

pub fn dl_playlist(url: &str, save_path: &str) -> Result<i32, Box<dyn Error>> {
    let tracks = playlist_info(url)?;
    thread::sleep(Duration::from_secs(1));
    println!("Now, downloading playlist...");
    let total = dl_track(&tracks, save_path)?;
    Ok(total)
}

pub fn dl_album(url: &str, save_path: &str) -> Result<i32, Box<dyn Error>> {
    let tracks = album_info(url)?;
    thread::sleep(Duration::from_secs(1));
    println!("Now, downloading album...");
    let total = dl_track(&tracks, save_path)?;
    Ok(total)
}

fn dl_track(tracks: &[Track], path: &str) -> Result<i32, Box<dyn Error>> {
    // Use a semaphore to limit concurrency to number of CPUs.
    let num_cpus = num_cpus::get();
    let semaphore = Arc::new(Semaphore::new(num_cpus));
    let (tx, rx) = mpsc::channel();

    // Get a DB client and wrap it in an Arc<Mutex<...>> so it can be shared.
    let db_client = db::new_db_client();
    let db_client = Arc::new(Mutex::new(db_client));

    let logger = utils::get_logger();
    let mut handles = Vec::new();

    for track in tracks.to_owned() {
        let sem = semaphore.clone();
        let tx = tx.clone();
        let db_client = db_client.clone();
        let path = path.to_string();
        let logger = logger.clone();
        let track_clone = track.clone();

        let handle = thread::spawn(move || {
            // Acquire a semaphore permit.
            let _permit = sem.acquire();

            // Create a copy of the track.
            let mut track_copy = Track {
                album: track_clone.album.clone(),
                artist: track_clone.artist.clone(),
                artists: track_clone.artists.clone(),
                duration: track_clone.duration,
                title: track_clone.title.clone(),
            };

            // Check if the song already exists.
            let song_key = utils::generate_song_key(&track_copy.title, &track_copy.artist);
            match song_key_exists(&song_key) {
                Ok(true) => {
                    let log_message = format!("'{}' by '{}' already exists.", track_copy.title, track_copy.artist);
                    slog::info!(logger, "{}", log_message);
                    return;
                }
                Err(e) => {
                    slog::error!(logger, "error checking song existence: {}", e);
                    // logger.error_context("error checking song existence", &e);
                    return;
                }
                _ => {} // Continue if not exists.
            }

            // Retrieve YouTube ID.
            let yt_id = match get_ytid(&track_copy) {
                Ok(id) => id,
                Err(e) => {
                    let log_message = format!("'{}' by '{}' could not be downloaded", track_copy.title, track_copy.artist);
                    slog::error!(logger, "{} error :{}", log_message,e);
                    // logger.error_context(&log_message, &e);
                    return;
                }
            };

            // Correct filename.
            let (corrected_title, corrected_artist) = correct_filename(&track_copy.title, &track_copy.artist);
            track_copy.title = corrected_title.clone();
            track_copy.artist = corrected_artist.clone();
            let file_name = format!("{} - {}", corrected_title, corrected_artist);
            let file_path = Path::new(&path).join(format!("{}.m4a", file_name));

            if let Err(e) = download_yt_audio(&yt_id, &path, file_path.to_str().unwrap()) {
                let log_message = format!("'{}' by '{}' could not be downloaded", track_copy.title, track_copy.artist);
                slog::error!(logger, "{} error :{}", log_message,e);
                // logger.error_context(&log_message, &e);
                return;
            }

            if let Err(e) = process_and_save_song(file_path.to_str().unwrap(), &track_copy.title, &track_copy.artist, &yt_id) {
                let log_message = format!("Failed to process song ('{}' by '{}')", track_copy.title, track_copy.artist);
                slog::error!(logger, "{} error :{}", log_message,e);
                // logger.error_context(&log_message, &e);
                return;
            }

            // Delete the downloaded m4a file.
            let m4a_path = Path::new(&path).join(format!("{}.m4a", file_name));
            let _ = utils::delete_file(m4a_path.to_str().unwrap());

            let wav_file_path = Path::new(&path).join(format!("{}.wav", file_name));
            if let Err(e) = add_tags(wav_file_path.to_str().unwrap(), &track_copy) {
                let log_message = format!("Error adding tags: {}.wav", file_name);
                slog::error!(logger, "{} error :{}", log_message,e);
                // logger.error_context(&log_message, &e);
                return;
            }

            if DELETE_SONG_FILE {
                let _ = utils::delete_file(wav_file_path.to_str().unwrap());
            }

            println!("'{}' by '{}' was downloaded", track_copy.title, track_copy.artist);
            tx.send(1).expect("Failed to send result");
        });
        handles.push(handle);
    }

    // Wait for all threads to finish.
    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    // Sum up results.
    let total_tracks: i32 = rx.iter().sum();
    println!("Total tracks downloaded: {}", total_tracks);
    Ok(total_tracks)
}

/// Downloads the YouTube audio stream for the given video ID.
/// This function uses an external library (or command) to download the audio.
/// It repeatedly attempts the download until the downloaded file size is non-zero.
fn download_yt_audio(id: &str, path: &str, file_path: &str) -> Result<(), Box<dyn Error>> {
    // Verify that `path` is a directory.
    if !Path::new(path).is_dir() {
        return Err("the path is not valid (not a dir)".into());
    }

    // For demonstration purposes, we use a placeholder implementation.
    // Replace this block with an actual YouTube audio download using your preferred crate.
    let mut file_size = 0;
    while file_size == 0 {
        // Simulate download by writing dummy data.
        let output = Command::new("echo")
            .arg("Simulated download")
            .output()?;
        // Create the file with some dummy content.
        fs::write(file_path, b"dummy audio data")?;
        file_size = fs::metadata(file_path)?.len();
    }
    Ok(())
}

/// Executes an FFmpeg command to add metadata tags to the given file.
/// It creates a temporary file and renames it to the original file.
fn add_tags(file: &str, track: &Track) -> Result<(), Box<dyn Error>> {
    // Create temporary file name by inserting "2" before the ".wav" extension.
    let temp_file = if let Some(index) = file.rfind(".wav") {
        format!("{}2.wav", &file[..index])
    } else {
        return Err("Invalid file name".into());
    };

    let output = Command::new("ffmpeg")
        .args(&[
            "-i", file,
            "-c", "copy",
            "-metadata", &format!("album_artist={}", track.artist),
            "-metadata", &format!("title={}", track.title),
            "-metadata", &format!("artist={}", track.artist),
            "-metadata", &format!("album={}", track.album),
            &temp_file,
        ])
        .output()?;

    if !output.status.success() {
        return Err(format!(
            "failed to add tags: {}",
            String::from_utf8_lossy(&output.stdout)
        )
        .into());
    }

    fs::rename(&temp_file, file)?;
    Ok(())
}

/// Processes and saves a song by converting it to WAV, creating its spectrogram,
/// extracting peaks and fingerprints, and then storing the fingerprints in the database.
pub fn process_and_save_song(song_file_path: &str, song_title: &str, song_artist: &str, yt_id: &str) -> Result<(), Box<dyn Error>> {
    // Create a runtime to run async code in sync context
    let rt = tokio::runtime::Runtime::new()?;
    
    // Use the runtime to block on the async db client creation
    let mut db_client = rt.block_on(db::new_db_client())?;
    
    let wav_file_path = wav::convert_to_wav(song_file_path, 1)?;
    let wav_info = wav::read_wav_info(&wav_file_path)?;
    let samples = wav::wav_bytes_to_samples(&wav_info.data)?;
    let spectro = shazam::spectrogram(&samples, wav_info.sample_rate)?;
    let song_id = db_client.register_song(song_title, song_artist, yt_id)?;
    let peaks = shazam::extract_peaks(&spectro, wav_info.duration);
    let fingerprints = shazam::fingerprint(&peaks, song_id);

    db_client.store_fingerprints(&fingerprints).map_err(|e| {
        let _ = db_client.delete_song_by_id(song_id);
        format!("error storing fingerprint: {}", e)
    })?;

    println!("Fingerprint for {} by {} saved in DB successfully", song_title, song_artist);
    Ok(())
}

/// Retrieves a YouTube ID for the given track.
/// If the obtained ID already exists, it will try again.
fn get_ytid(track: &Track) -> Result<String, Box<dyn Error>> {
    let mut yt_id = get_youtube_id(track)?;
    if yt_id.is_empty() {
        return Err("YouTube ID is empty".into());
    }
    if ytid_exists(&yt_id)? {
        println!("WARN: YouTube ID ({}) exists. Trying again...", yt_id);
        yt_id = get_youtube_id(track)?;
        if yt_id.is_empty() || ytid_exists(&yt_id)? {
            return Err(format!("youTube ID ({}) exists", yt_id).into());
        }
    }
    Ok(yt_id)
}

// --- Stub implementations below ---
// These functions must be implemented according to your project logic.

fn track_info(url: &str) -> Result<Track, Box<dyn Error>> {
    // Placeholder: return a dummy track.
    Ok(Track {
        album: "Album".to_string(),
        artist: "Artist".to_string(),
        artists: vec!["Artist".to_string()],
        duration: 180 as f64,
        title: "Title".to_string(),
    })
}

fn playlist_info(url: &str) -> Result<Vec<Track>, Box<dyn Error>> {
    // Placeholder implementation.
    Ok(vec![track_info(url)?])
}

fn album_info(url: &str) -> Result<Vec<Track>, Box<dyn Error>> {
    // Placeholder implementation.
    Ok(vec![track_info(url)?])
}

fn song_key_exists(_key: &str) -> Result<bool, Box<dyn Error>> {
    // Placeholder: always return false.
    Ok(false)
}

fn get_youtube_id(track: &Track) -> Result<String, Box<dyn Error>> {
    // Placeholder: return a dummy YouTube ID.
    Ok("dummy_yt_id".to_string())
}

fn ytid_exists(_yt_id: &str) -> Result<bool, Box<dyn Error>> {
    // Placeholder: always return false.
    Ok(false)
}

fn correct_filename(title: &str, artist: &str) -> (String, String) {
    // Placeholder: simply trim and return.
    (title.trim().to_string(), artist.trim().to_string())
}
