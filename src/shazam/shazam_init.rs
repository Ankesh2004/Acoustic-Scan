use std::collections::HashMap;
use std::error::Error;
use std::time::{Duration, Instant};

use crate::db;
use crate::models;
use crate::db::Song;
use crate::models::Couple;
use crate::shazam::{extract_peaks, fingerprint, spectrogram};
use crate::utils;
use crate::shazam::fingerprint::Peak;
use slog::info;

#[derive(Debug, Clone)]
pub struct Match1 {
    pub song_id: u32,
    pub song_title: String,
    pub song_artist: String,
    pub youtube_id: String,
    pub timestamp: u32,
    pub coherency: f64,
}

/// Processes the audio samples and searches for matching songs.
/// Returns a list of matches sorted in descending order of coherency.
pub async fn search(
    audio_samples: &[f64],
    audio_duration: f64,
    sample_rate: i32,
) -> Result<Vec<Match1>, Box<dyn Error>> {
    // Compute spectrogram.
    let spectrogram = spectrogram(audio_samples, sample_rate)
        .map_err(|e| format!("failed to get spectrogram of samples: {}", e))?;
    
    // Extract peaks from the spectrogram.
    let peaks = extract_peaks(&spectrogram, audio_duration);
    // Generate fingerprints using a unique song ID.
    let fingerprints = fingerprint(&peaks, utils::generate_unique_id());
    
    // Collect fingerprint addresses.
    let addresses: Vec<u32> = fingerprints.keys().cloned().collect();

    let mut db_client = db::new_db_client().await?;
    // Get couples from the database.
    let couples_map = db_client.get_couples(&addresses)?;
    db_client.close();

    // Build maps for relative timing analysis.
    let mut matches_map: HashMap<u32, Vec<[u32; 2]>> = HashMap::new(); // song_id -> list of [sample_time, db_time]
    let mut timestamps: HashMap<u32, Vec<u32>> = HashMap::new();

    for (_address, couples) in couples_map.iter() {
        for couple in couples {
            // Use the anchor time from our fingerprint (for the given address).
            // If not found, default to 0.
            let sample_time = fingerprints.get(&_address).map(|c| c.anchor_time_ms).unwrap_or(0);
            matches_map
                .entry(couple.song_id)
                .or_insert_with(Vec::new)
                .push([sample_time, couple.anchor_time_ms]);
            timestamps
                .entry(couple.song_id)
                .or_insert_with(Vec::new)
                .push(couple.anchor_time_ms);
        }
    }

    // Compute a score for each song based on relative timing.
    let scores = analyze_relative_timing(&matches_map);

    // Prepare the final match list.
    let mut match_list = Vec::new();
    let mut db_client = db::new_db_client().await?;
    for (&song_id, &coherency) in scores.iter() {
        let (song, song_exists) = db_client.get_song_by_id(song_id)?;
        if !song_exists {
            // utils::get_logger().info(&format!("song with ID ({}) doesn't exist", song_id));
            let logger = utils::get_logger();
info!(logger, "song with ID ({}) doesn't exist", song_id);

            continue;
        }
        // Sort timestamps for the song.
        if let Some(ts) = timestamps.get_mut(&song_id) {
            ts.sort_unstable();
        }
        let timestamp = timestamps.get(&song_id).and_then(|ts| ts.first().cloned()).unwrap_or(0);
        let m = Match1 {
            song_id,
            song_title: song.title,
            song_artist: song.artist,
            youtube_id: song.youtube_id,
            timestamp,
            coherency: coherency as f64,
        };
        match_list.push(m);
    }
    db_client.close();

    // Sort match list in descending order by coherency.
    match_list.sort_by(|a, b| b.coherency.partial_cmp(&a.coherency).unwrap_or(std::cmp::Ordering::Equal));

    Ok(match_list)
}

/// Analyzes relative timing of fingerprint pairs and returns a score per song.
/// For each song, counts pairs of (sample time, db time) whose difference is within tolerance.
fn analyze_relative_timing(matches: &HashMap<u32, Vec<[u32; 2]>>) -> HashMap<u32, i32> {
    let mut scores = HashMap::new();
    for (&song_id, times) in matches.iter() {
        let mut count = 0;
        for i in 0..times.len() {
            for j in i + 1..times.len() {
                let sample_diff = (times[i][0] as f64 - times[j][0] as f64).abs();
                let db_diff = (times[i][1] as f64 - times[j][1] as f64).abs();
                if (sample_diff - db_diff).abs() < 100.0 {
                    count += 1;
                }
            }
        }
        scores.insert(song_id, count);
    }
    scores
}

/// Builds target zones from the couples in the database.
/// Keeps only anchor times with at least 5 occurrences.
fn target_zones(couples_map: &HashMap<u32, Vec<models::Couple>>) -> HashMap<u32, Vec<u32>> {
    let mut songs: HashMap<u32, HashMap<u32, i32>> = HashMap::new();

    for couples in couples_map.values() {
        for couple in couples {
            songs.entry(couple.song_id)
                .or_insert_with(HashMap::new)
                .entry(couple.anchor_time_ms)
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
    }

    // Remove anchor times with counts less than 5.
    for (_song_id, anchor_times) in songs.iter_mut() {
        anchor_times.retain(|_, &mut count| count >= 5);
    }

    // Build target zones: for each song, list remaining anchor times.
    let mut target_zones = HashMap::new();
    for (song_id, anchor_times) in songs {
        let zones: Vec<u32> = anchor_times.into_iter().map(|(ms, _)| ms).collect();
        target_zones.insert(song_id, zones);
    }

    target_zones
}

/// Computes time coherency between the fingerprint record and target zones.
/// For each song, compares each target anchor time with every record anchor time and counts matches.
fn time_coherency(record: &HashMap<u32, models::Couple>, songs: &HashMap<u32, Vec<u32>>) -> HashMap<u32, i32> {
    let mut matches = HashMap::new();

    for (&song_id, song_anchor_times) in songs.iter() {
        // Use quantization to handle floating point keys
        let mut deltas = HashMap::new();
        for &song_anchor_time in song_anchor_times.iter() {
            for couple in record.values() {
                let record_anchor_time = couple.anchor_time_ms as f64;
                let delta = record_anchor_time - song_anchor_time as f64;
                // Convert to integer by quantizing (round to nearest integer)
                let quantized_delta = (delta * 10.0).round() as i32;
                *deltas.entry(quantized_delta).or_insert(0) += 1;
            }
        }
        let max_occurrences = deltas.values().cloned().max().unwrap_or(0);
        matches.insert(song_id, max_occurrences);
    }

    matches
}
