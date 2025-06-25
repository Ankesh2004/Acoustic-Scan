use std::fs;
use std::io;
use std::path::Path;

use base64;
use base64::Engine;
use byteorder::{LittleEndian, WriteBytesExt};
use chrono::prelude::*;
use slog::{error,info};

use crate::models;
use crate::wav;

/// Deletes the file or directory at `file_path` if it exists.
pub fn delete_file(file_path: &str) -> io::Result<()> {
    if Path::new(file_path).exists() {
        fs::remove_dir_all(file_path)?;
    }
    Ok(())
}

/// Creates a folder (and any necessary parent directories) at `folder_path`.
pub fn create_folder(folder_path: &str) -> io::Result<()> {
    fs::create_dir_all(folder_path)
}

/// Converts a slice of floating‑point samples to a vector of bytes according to the specified bits per sample.
/// Supported bitsPerSample values: 8, 16, 24, 32.
pub fn floats_to_bytes(data: &[f64], bits_per_sample: i32) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut byte_data = Vec::new();

    match bits_per_sample {
        8 => {
            for &sample in data {
                // Convert float in [-1, 1] to 8-bit unsigned integer.
                let val = ((sample + 1.0) * 127.5).round() as u8;
                byte_data.push(val);
            }
        }
        16 => {
            for &sample in data {
                // Convert float to 16-bit signed integer.
                let val = (sample * 32767.0).round() as i16;
                let mut buf = Vec::with_capacity(2);
                buf.write_i16::<LittleEndian>(val)?;
                byte_data.extend_from_slice(&buf);
            }
        }
        24 => {
            for &sample in data {
                // Convert float to 24-bit signed integer.
                let val = (sample * 8388607.0).round() as i32;
                let mut buf = [0u8; 4];
                (&mut buf[..]).write_i32::<LittleEndian>(val)?;
                // Only take the lower 3 bytes.
                byte_data.extend_from_slice(&buf[..3]);
            }
        }
        32 => {
            for &sample in data {
                // Convert float to 32-bit signed integer.
                let val = (sample * 2147483647.0).round() as i32;
                let mut buf = Vec::with_capacity(4);
                buf.write_i32::<LittleEndian>(val)?;
                byte_data.extend_from_slice(&buf);
            }
        }
        _ => return Err(format!("unsupported bitsPerSample: {}", bits_per_sample).into()),
    }
    Ok(byte_data)
}

/// Processes recording data by decoding, writing a temporary WAV file, reformatting it, reading samples,
/// and optionally moving the file to a recordings folder.
/// Temporary files are cleaned up afterward.
pub fn process_recording(rec_data: &models::RecordData, save_recording: bool) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    // Decode the Base64-encoded audio.
    let decoded_audio_data = base64::prelude::BASE64_STANDARD.decode(&rec_data.audio).expect("Failed to decode audio data.");

    // Generate a filename using the current time.
    let now = Local::now();
    let file_name = format!("{:04}_{:02}_{:02}_{:02}_{:02}_{:02}.wav",
        now.second(),
        now.minute(),
        now.hour(),
        now.day(),
        now.month(),
        now.year(),
    );
    let file_path = format!("tmp/{}", file_name);

    // Write the initial WAV file.
    wav::write_wav_file(
        &file_path,
        &decoded_audio_data,
        rec_data.sample_rate,
        rec_data.channels,
        rec_data.sample_size,
    )?;

    // Reformat the WAV file (forcing single channel).
    let reformatted_wav_file = wav::reformat_wav(&file_path, 1)?;

    // Read WAV info and extract samples.
    let wav_info = wav::read_wav_info(&reformatted_wav_file)?;
    let samples = wav::wav_bytes_to_samples(&wav_info.data)?;

    if save_recording {
        let logger = crate::utils::get_logger();
        // Create the recordings folder.
        if let Err(e) = create_folder("recordings") {
            // logger.error_context("", &e);
            error!(logger, "Failed to create folder: {}", e);
        }
        // Move the reformatted file into the recordings folder.
        let new_file_path = reformatted_wav_file.replacen("tmp/", "recordings/", 1);
        if let Err(e) = fs::rename(&reformatted_wav_file, &new_file_path) {
            // logger.error_context("Failed to move file.", &e);
            error!(logger, "Failed to move file.{}", e);
        }
    }

    // Clean up temporary files.
    let _ = delete_file(&file_path);
    let _ = delete_file(&reformatted_wav_file);

    Ok(samples)
}
