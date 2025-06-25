use std::error::Error;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Converts an input audio file to WAV format with the specified number of channels.
/// It uses FFmpeg to perform the conversion and writes the result to a temporary file
/// before renaming it to the final output.
pub fn convert_to_wav(input_file_path: &str, mut channels: i32) -> Result<String, Box<dyn Error>> {
    // Check if the input file exists.
    if !Path::new(input_file_path).exists() {
        return Err(format!("input file does not exist: {}", input_file_path).into());
    }
    
    println!("Converting file: {}", input_file_path);
    
    // Force channels to be either 1 or 2, defaulting to 1 if outside the allowed range.
    if channels < 1 || channels > 2 {
        channels = 1;
    }

    let input_path = Path::new(input_file_path);
    let file_ext = input_path.extension().and_then(|s| s.to_str()).unwrap_or("");
    let mut output_file = input_file_path.trim_end_matches(&format!(".{}", file_ext)).to_string();
    output_file.push_str(".wav");
    
    println!("Output will be: {}", output_file);
    
    // Create a temporary file path in the same directory.
    let tmp_file = {
        let mut tmp = input_path.with_file_name(format!("tmp_{}", input_path.file_name().unwrap().to_string_lossy()));
        tmp.set_extension("wav");  // Make sure temp file has .wav extension
        tmp
    };

    // Run FFmpeg with more verbose output
    let ffmpeg_output = Command::new("ffmpeg")
        .arg("-y")
        .arg("-i")
        .arg(input_file_path)
        .arg("-c")
        .arg("pcm_s16le")
        .arg("-ar")
        .arg("44100")
        .arg("-ac")
        .arg(channels.to_string())
        .arg(tmp_file.to_str().unwrap())
        .output()?;

    if !ffmpeg_output.status.success() {
        // Remove tmp file if it exists.
        let _ = fs::remove_file(&tmp_file);
        println!("FFmpeg STDERR: {}", String::from_utf8_lossy(&ffmpeg_output.stderr));
        return Err(format!(
            "failed to convert to WAV: {}. output: {}, error: {}",
            ffmpeg_output.status,
            String::from_utf8_lossy(&ffmpeg_output.stdout),
            String::from_utf8_lossy(&ffmpeg_output.stderr)
        ).into());
    }

    // Verify the file was created and has content
    if !tmp_file.exists() || fs::metadata(&tmp_file)?.len() == 0 {
        return Err(format!("FFmpeg did not produce a valid output file").into());
    }

    // Rename the temporary file to the output file.
    fs::rename(&tmp_file, &output_file)
        .map_err(|e| format!("failed to rename temporary file to output file: {}", e))?;

    Ok(output_file)
}

/// Reformats a WAV file with the specified number of channels. The reformatted file will have
/// "rfm.wav" appended to its original base name.
pub fn reformat_wav(input_file_path: &str, mut channels: i32) -> Result<String, Box<dyn Error>> {
    if channels < 1 || channels > 2 {
        channels = 1;
    }

    let input_path = Path::new(input_file_path);
    let file_ext = input_path.extension().and_then(|s| s.to_str()).unwrap_or("");
    let mut output_file = input_file_path.trim_end_matches(&format!(".{}", file_ext)).to_string();
    output_file.push_str("rfm.wav");

    let ffmpeg_status = Command::new("ffmpeg")
        .arg("-y")
        .arg("-i")
        .arg(input_file_path)
        .arg("-c")
        .arg("pcm_s16le")
        .arg("-ar")
        .arg("44100")
        .arg("-ac")
        .arg(channels.to_string())
        .arg(&output_file)
        .output()?;

    if !ffmpeg_status.status.success() {
        return Err(format!(
            "failed to convert to WAV: {}. output: {}",
            ffmpeg_status.status,
            String::from_utf8_lossy(&ffmpeg_status.stdout)
        )
        .into());
    }

    Ok(output_file)
}
