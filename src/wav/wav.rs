use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, Cursor, Read, Write};
use std::process::Command;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};

/// Represents the header of a WAV file.
pub struct WavHeader {
    pub chunk_id: [u8; 4],
    pub chunk_size: u32,
    pub format: [u8; 4],
    pub subchunk1_id: [u8; 4],
    pub subchunk1_size: u32,
    pub audio_format: u16,
    pub num_channels: u16,
    pub sample_rate: u32,
    pub bytes_per_sec: u32,
    pub block_align: u16,
    pub bits_per_sample: u16,
    pub subchunk2_id: [u8; 4],
    pub subchunk2_size: u32,
}

impl WavHeader {
    fn write_to<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(&self.chunk_id)?;
        writer.write_u32::<LittleEndian>(self.chunk_size)?;
        writer.write_all(&self.format)?;
        writer.write_all(&self.subchunk1_id)?;
        writer.write_u32::<LittleEndian>(self.subchunk1_size)?;
        writer.write_u16::<LittleEndian>(self.audio_format)?;
        writer.write_u16::<LittleEndian>(self.num_channels)?;
        writer.write_u32::<LittleEndian>(self.sample_rate)?;
        writer.write_u32::<LittleEndian>(self.bytes_per_sec)?;
        writer.write_u16::<LittleEndian>(self.block_align)?;
        writer.write_u16::<LittleEndian>(self.bits_per_sample)?;
        writer.write_all(&self.subchunk2_id)?;
        writer.write_u32::<LittleEndian>(self.subchunk2_size)?;
        Ok(())
    }
}

/// Writes a WAV header to the given writer.
pub fn write_wav_header<W: Write>(
    writer: &mut W,
    data: &[u8],
    sample_rate: i32,
    channels: i32,
    bits_per_sample: i32,
) -> Result<(), Box<dyn Error>> {
    if data.len() % channels as usize != 0 {
        return Err("data size not divisible by channels".into());
    }

    let subchunk1_size = 16u32; // PCM format
    let bytes_per_sample = (bits_per_sample / 8) as u32;
    let block_align = channels as u16 * bytes_per_sample as u16;
    let subchunk2_size = data.len() as u32;

    let header = WavHeader {
        chunk_id: *b"RIFF",
        chunk_size: 36 + subchunk2_size,
        format: *b"WAVE",
        subchunk1_id: *b"fmt ",
        subchunk1_size,
        audio_format: 1, // PCM format
        num_channels: channels as u16,
        sample_rate: sample_rate as u32,
        bytes_per_sec: sample_rate as u32 * channels as u32 * bytes_per_sample,
        block_align,
        bits_per_sample: bits_per_sample as u16,
        subchunk2_id: *b"data",
        subchunk2_size,
    };

    header.write_to(writer)?;
    Ok(())
}

/// Creates a WAV file with the given filename, header values and data.
pub fn write_wav_file(
    filename: &str,
    data: &[u8],
    sample_rate: i32,
    channels: i32,
    bits_per_sample: i32,
) -> Result<(), Box<dyn Error>> {
    if sample_rate <= 0 || channels <= 0 || bits_per_sample <= 0 {
        return Err(format!(
            "values must be greater than zero (sampleRate: {}, channels: {}, bitsPerSample: {})",
            sample_rate, channels, bits_per_sample
        )
        .into());
    }

    let mut file = File::create(filename)?;
    write_wav_header(&mut file, data, sample_rate, channels, bits_per_sample)?;
    file.write_all(data)?;
    Ok(())
}

/// Contains information extracted from a WAV file.
pub struct WavInfo {
    pub channels: i32,
    pub sample_rate: i32,
    pub data: Vec<u8>,
    pub duration: f64,
}

/// Reads a WAV file and extracts header information along with the PCM data.
pub fn read_wav_info(filename: &str) -> Result<WavInfo, Box<dyn Error>> {
    let data = std::fs::read(filename)?;
    if data.len() < 44 {
        return Err("invalid WAV file size (too small)".into());
    }

    let mut rdr = Cursor::new(&data[..44]);

    let mut chunk_id = [0u8; 4];
    rdr.read_exact(&mut chunk_id)?;
    let _chunk_size = rdr.read_u32::<LittleEndian>()?;
    let mut format = [0u8; 4];
    rdr.read_exact(&mut format)?;
    let mut subchunk1_id = [0u8; 4];
    rdr.read_exact(&mut subchunk1_id)?;
    let _subchunk1_size = rdr.read_u32::<LittleEndian>()?;
    let audio_format = rdr.read_u16::<LittleEndian>()?;
    let num_channels = rdr.read_u16::<LittleEndian>()?;
    let sample_rate = rdr.read_u32::<LittleEndian>()?;
    let _bytes_per_sec = rdr.read_u32::<LittleEndian>()?;
    let _block_align = rdr.read_u16::<LittleEndian>()?;
    let bits_per_sample = rdr.read_u16::<LittleEndian>()?;
    let mut subchunk2_id = [0u8; 4];
    rdr.read_exact(&mut subchunk2_id)?;
    let _subchunk2_size = rdr.read_u32::<LittleEndian>()?;

    if &chunk_id != b"RIFF" || &format != b"WAVE" || audio_format != 1 {
        return Err("invalid WAV header format".into());
    }

    let mut info = WavInfo {
        channels: num_channels as i32,
        sample_rate: sample_rate as i32,
        data: data[44..].to_vec(),
        duration: 0.0,
    };

    if bits_per_sample == 16 {
        info.duration = info.data.len() as f64 / (num_channels as f64 * 2.0 * sample_rate as f64);
    } else {
        return Err("unsupported bits per sample format".into());
    }
    Ok(info)
}

/// Converts a slice of 16-bit PCM bytes to a vector of f64 samples scaled in the range [-1, 1].
pub fn wav_bytes_to_samples(input: &[u8]) -> Result<Vec<f64>, Box<dyn Error>> {
    if input.len() % 2 != 0 {
        return Err("invalid input length".into());
    }
    let num_samples = input.len() / 2;
    let mut output = Vec::with_capacity(num_samples);
    for i in 0..num_samples {
        let sample = i16::from_le_bytes([input[i * 2], input[i * 2 + 1]]);
        output.push(sample as f64 / 32768.0);
    }
    Ok(output)
}
fn default_start_time() -> String {
    "0".to_string()
}
/// Represents the metadata structure returned by ffprobe.
#[derive(Serialize, Deserialize, Debug)]
pub struct FFmpegMetadata {
    pub streams: Vec<Stream>,
    pub format: Format,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stream {
    pub index: i32,
    pub codec_name: String,
    pub codec_long_name: String,
    pub codec_type: String,
    pub sample_fmt: Option<String>,
    pub sample_rate: Option<String>,
    pub channels: Option<i32>,
    pub channel_layout: Option<String>,
    pub bits_per_sample: Option<i32>,
    pub duration: Option<String>,
    pub bit_rate: Option<String>,
    pub disposition: Option<HashMap<String, i32>>,
    pub tags: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Format {
    #[serde(rename = "nb_streams")]
    pub nb_streams: i32,
    #[serde(rename = "filename")]
    pub filename: String,
    #[serde(rename = "format_name")]
    pub format_name: String,
    #[serde(rename = "format_long_name")]
    pub format_long_name: String,
    #[serde(rename = "start_time",default = "default_start_time")]
    pub start_time: String,
    #[serde(rename = "duration")]
    pub duration: String,
    #[serde(rename = "size")]
    pub size: String,
    #[serde(rename = "bit_rate")]
    pub bit_rate: String,
    pub tags: Option<HashMap<String, String>>,
}

/// Retrieves metadata from a file using ffprobe.
pub fn get_metadata(file_path: &str) -> Result<FFmpegMetadata, Box<dyn Error>> {
    let output = Command::new("ffprobe")
        .args(&[
            "-v", "quiet",
            "-print_format", "json",
            "-show_format",
            "-show_streams",
            file_path,
        ])
        .output()?;
    let metadata: FFmpegMetadata = serde_json::from_slice(&output.stdout)?;
    Ok(metadata)
}
