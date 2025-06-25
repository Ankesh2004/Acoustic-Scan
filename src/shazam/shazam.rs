use std::collections::HashMap;
use std::error::Error;
use std::time::{Duration, Instant};
use serde::Serialize;
use crate::wav;

use crate::db;
use crate::models::Couple;
use crate::db::Song; 
use slog::info;

// Assumes Song has fields: title, artist, youtube_id, etc.
use crate::shazam::{extract_peaks, fingerprint, spectrogram, Peak};
use crate::utils;

// Represents a matching song from the database.


#[derive(Serialize)]
pub struct Match {
    pub song_id: u32,
    pub song_title: String,
    pub song_artist: String,
    pub youtube_id: String,
    pub timestamp: u32,
    pub score: f64,
}

pub async fn find_matches_for_api(file_path: &str) -> Result<Vec<Match>, Box<dyn Error>> {
    let wav_info = wav::read_wav_info(file_path)?;
    let samples = wav::wav_bytes_to_samples(&wav_info.data)?;
    
    let (matches, _) = find_matches(&samples, wav_info.duration, wav_info.sample_rate).await?;
    Ok(matches)
}

/// Processes the audio samples and finds matching songs from the database.
/// Returns a list of matches sorted in descending order by score along with the duration
/// of the search.
pub async fn find_matches(
    audio_samples: &[f64],
    audio_duration: f64,
    sample_rate: i32,
) -> Result<(Vec<Match>, Duration), Box<dyn Error>> {
    let start_time = Instant::now();
    let logger = utils::get_logger();

    // Get the spectrogram of the audio samples.
    let spectro = spectrogram(audio_samples, sample_rate)
        .map_err(|e| format!("failed to get spectrogram of samples: {}", e))?;
    // Extract peaks from the spectrogram.
    let peaks = extract_peaks(&spectro, audio_duration);
    // Generate fingerprints using a unique song ID.
    let fingerprints = fingerprint(&peaks, utils::generate_unique_id());

    // Collect all fingerprint addresses.
    let addresses: Vec<u32> = fingerprints.keys().cloned().collect();

    let mut db_client = db::new_db_client().await?;
    // Query the database to get couples (fingerprint matches) for the addresses.
    let couples_map = db_client.get_couples(&addresses)?;
    // Close the DB client once we're done.
    db_client.close();

    // Build maps for relative timing analysis.
    let mut matches_map: HashMap<u32, Vec<[u32; 2]>> = HashMap::new(); // song_id -> list of [sample_time, db_time]
    let mut timestamps: HashMap<u32, Vec<u32>> = HashMap::new();

    // Iterate over each fingerprint address found in the database.
    for (&address, couples) in couples_map.iter() {
        // For each couple (from the database) corresponding to this fingerprint:
        for couple in couples {
            // Here, fingerprints[address] must have an anchor time in ms.
            // We assume the Couple struct in models has a field `anchor_time` of type u32.
            let anchor_time_ms = fingerprints.get(&address)
                .map(|couple| couple.anchor_time_ms)
                .unwrap_or(0);
            // Add the pair [sample_time, db_time] into the matches_map for this song.
            matches_map.entry(couple.song_id)
                .or_insert_with(Vec::new)
                .push([anchor_time_ms, couple.anchor_time_ms]);
            timestamps.entry(couple.song_id)
                .or_insert_with(Vec::new)
                .push(couple.anchor_time_ms);
        }
    }

    // Analyze relative timing to produce a score for each song.
    let scores = analyze_relative_timing(&matches_map);

    let mut match_list = Vec::new();

    // For each song with a score, fetch its metadata from the database.
    let mut db_client = db::new_db_client().await?;
    for (&song_id, &points) in scores.iter() {
        let (song, song_exists) = db_client.get_song_by_id(song_id)?;
        if !song_exists {
            let logger = utils::get_logger();
            info!(logger, "song with ID ({}) doesn't exist", song_id);

            continue;
        }
        // Sort the timestamps for the song in ascending order.
        if let Some(ts) = timestamps.get_mut(&song_id) {
            ts.sort_unstable();
        }
        let timestamp = timestamps.get(&song_id)
            .and_then(|ts| ts.first().cloned())
            .unwrap_or(0);
        let m = Match {
            song_id,
            song_title: song.title,
            song_artist: song.artist,
            youtube_id: song.youtube_id,
            timestamp,
            score: points,
        };
        match_list.push(m);
    }
    db_client.close();

    // Sort match_list in descending order by score.
    match_list.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

    Ok((match_list, start_time.elapsed()))
}

/// Analyzes the relative timing between matched fingerprint pairs and returns a score for each song.
/// The score is computed as the number of pairs whose relative timing differences are within a tolerance.
fn analyze_relative_timing(matches: &HashMap<u32, Vec<[u32; 2]>>) -> HashMap<u32, f64> {
    let mut scores = HashMap::new();
    for (&song_id, times) in matches.iter() {
        let mut count = 0;
        for i in 0..times.len() {
            for j in i + 1..times.len() {
                let sample_diff = (times[i][0] as f64 - times[j][0] as f64).abs();
                let db_diff = (times[i][1] as f64 - times[j][1] as f64).abs();
                if (sample_diff - db_diff).abs() < 100.0 { // Allow some tolerance
                    count += 1;
                }
            }
        }
        scores.insert(song_id, count as f64);
    }
    scores
}
