use num_complex::Complex;
use std::error::Error;
use std::f64::consts::PI;

use crate::shazam::filter::LowPassFilter; // Assumes a LowPassFilter struct with a `filter(&[f64]) -> Vec<f64>` method.
use crate::shazam::fft::fft;      // Assumes an FFT function: `fn fft(input: &[f64]) -> Vec<Complex<f64>>`
use crate::shazam::fingerprint::Peak;
// Constants
const DSP_RATIO: i32 = 4;
const FREQ_BIN_SIZE: usize = 1024;
const MAX_FREQ: f64 = 5000.0; // 5kHz
const HOP_SIZE: usize = FREQ_BIN_SIZE / 32;

/// Computes the spectrogram (STFT) of the input audio samples.
/// Returns a two-dimensional vector where each row is the FFT of a windowed segment.
pub fn spectrogram(samples: &[f64], sample_rate: i32) -> Result<Vec<Vec<Complex<f64>>>, Box<dyn Error>> {
    // Apply a low-pass filter
    let mut lpf = LowPassFilter::new(MAX_FREQ, sample_rate as f64);
    let filtered_samples = lpf.filter(samples);

    // Downsample the filtered samples.
    let target_sample_rate = sample_rate / DSP_RATIO;
    let downsampled_samples = downsample(&filtered_samples, sample_rate, target_sample_rate)
        .map_err(|e| format!("couldn't downsample audio samples: {}", e))?;

    // Compute number of windows for the spectrogram.
    let window_length = FREQ_BIN_SIZE;
    let hop = HOP_SIZE;
    let num_of_windows = downsampled_samples.len() / (window_length - hop);
    let mut spectrogram = Vec::with_capacity(num_of_windows);

    // Create a Hamming window.
    let window: Vec<f64> = (0..window_length)
        .map(|i| 0.54 - 0.46 * ((2.0 * PI * i as f64) / ((window_length - 1) as f64)).cos())
        .collect();

    // Perform STFT.
    for i in 0..num_of_windows {
        let start = i * hop;
        let mut end = start + window_length;
        if end > downsampled_samples.len() {
            end = downsampled_samples.len();
        }
        let mut bin = vec![0.0; window_length];
        // Copy available samples into bin.
        bin[..(end - start)].copy_from_slice(&downsampled_samples[start..end]);

        // Apply the Hamming window.
        for j in 0..window_length {
            bin[j] *= window[j];
        }

        // Compute the FFT for this bin.
        let fft_result = fft(&bin);
        spectrogram.push(fft_result);
    }

    Ok(spectrogram)
}

/// Downsamples the input audio from the original sample rate to the target sample rate.
pub fn downsample(input: &[f64], original_sample_rate: i32, target_sample_rate: i32) -> Result<Vec<f64>, Box<dyn Error>> {
    if target_sample_rate <= 0 || original_sample_rate <= 0 {
        return Err("sample rates must be positive".into());
    }
    if target_sample_rate > original_sample_rate {
        return Err("target sample rate must be less than or equal to original sample rate".into());
    }

    let ratio = original_sample_rate / target_sample_rate;
    if ratio <= 0 {
        return Err("invalid ratio calculated from sample rates".into());
    }

    let mut resampled = Vec::new();
    let len = input.len();
    let ratio = ratio as usize;
    let mut i = 0;
    while i < len {
        let end = if i + ratio > len { len } else { i + ratio };
        let sum: f64 = input[i..end].iter().sum();
        let avg = sum / (end - i) as f64;
        resampled.push(avg);
        i += ratio;
    }

    Ok(resampled)
}

/// A Peak in the spectrogram with its time (in seconds) and frequency (as a complex number).
// #[derive(Debug, Clone)]
// pub struct Peak {
//     pub time: f64,
//     pub freq: Complex<f64>,
// }

/// Analyzes a spectrogram and extracts significant peaks in the frequency domain over time.
pub fn extract_peaks(spectrogram: &[Vec<Complex<f64>>], audio_duration: f64) -> Vec<Peak> {
    if spectrogram.is_empty() {
        return vec![];
    }

    // Local struct for tracking maximum magnitude within a band.
    struct Maxies {
        max_mag: f64,
        max_freq: Complex<f64>,
        freq_idx: usize,
    }

    // Define frequency bands (indices).
    let bands = vec![(0, 10), (10, 20), (20, 40), (40, 80), (80, 160), (160, 512)];

    let mut peaks = Vec::new();
    let bin_duration = audio_duration / spectrogram.len() as f64;

    // Iterate over each time window (bin) in the spectrogram.
    for (bin_idx, bin) in spectrogram.iter().enumerate() {
        let mut bin_band_maxies = Vec::new();
        // For each defined band, find the frequency bin with maximum magnitude.
        for &(min, max) in bands.iter() {
            let mut max_val = 0.0;
            let mut max_entry = Maxies { max_mag: 0.0, max_freq: Complex::new(0.0, 0.0), freq_idx: min };
            for (idx, freq) in bin[min..max].iter().enumerate() {
                let magnitude = freq.norm();
                if magnitude > max_val {
                    max_val = magnitude;
                    max_entry = Maxies {
                        max_mag: magnitude,
                        max_freq: *freq,
                        freq_idx: min + idx,
                    };
                }
            }
            bin_band_maxies.push(max_entry);
        }

        // Extract arrays of maximum magnitudes, frequencies, and their indices.
        let max_mags: Vec<f64> = bin_band_maxies.iter().map(|m| m.max_mag).collect();
        let max_freqs: Vec<Complex<f64>> = bin_band_maxies.iter().map(|m| m.max_freq).collect();
        let freq_indices: Vec<f64> = bin_band_maxies.iter().map(|m| m.freq_idx as f64).collect();

        // Calculate average magnitude across bands.
        let sum: f64 = max_mags.iter().sum();
        let avg = sum / max_mags.len() as f64;

        // For each band, if the maximum magnitude exceeds the average, record a peak.
        for i in 0..max_mags.len() {
            if max_mags[i] > avg {
                // Calculate a time offset within the bin.
                let peak_time_in_bin = freq_indices[i] * bin_duration / bin.len() as f64;
                let peak_time = bin_idx as f64 * bin_duration + peak_time_in_bin;
                peaks.push(Peak { time: peak_time, freq: max_freqs[i] });
            }
        }
    }

    peaks
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_complex::Complex;

    #[test]
    fn test_downsample() {
        // Test with a simple sequence.
        let input: Vec<f64> = (0..100).map(|x| x as f64).collect();
        let original_rate = 100;
        let target_rate = 25;
        let resampled = downsample(&input, original_rate, target_rate).unwrap();
        // Expect length approx 100/4 = 25.
        assert!((resampled.len() as i32 - 25).abs() <= 1);
    }

    #[test]
    fn test_extract_peaks_empty() {
        let spec: Vec<Vec<Complex<f64>>> = vec![];
        let peaks = extract_peaks(&spec, 1.0);
        assert!(peaks.is_empty());
    }
}
