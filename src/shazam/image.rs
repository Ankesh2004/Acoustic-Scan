use image::{GrayImage, Luma, ExtendedColorType, imageops::FilterType, ImageEncoder};
use image::codecs::png::PngEncoder;
use num_complex::Complex;
use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::f64;

/// Converts a spectrogram to a grayscale heat map image and saves it as a PNG file.
///
/// # Arguments
///
/// * `spectrogram` - A two-dimensional slice of complex numbers representing the spectrogram.
/// * `output_path` - The file path where the PNG image will be saved.
///
/// # Errors
///
/// Returns an error if the image cannot be saved.
pub fn spectrogram_to_image(
    spectrogram: &[Vec<Complex<f64>>],
    output_path: &str,
) -> Result<(), Box<dyn Error>> {
    // Determine dimensions of the spectrogram.
    let num_windows = spectrogram.len();
    if num_windows == 0 {
        return Err("Spectrogram has no windows".into());
    }
    let num_freq_bins = spectrogram[0].len();
    if num_freq_bins == 0 {
        return Err("Spectrogram has no frequency bins".into());
    }

    // Create a new grayscale image with dimensions (width, height) = (num_freq_bins, num_windows).
    let mut img = GrayImage::new(num_freq_bins as u32, num_windows as u32);

    // Determine the maximum magnitude in the spectrogram.
    let mut max_magnitude = 0.0;
    for window in spectrogram.iter() {
        for &value in window.iter() {
            let magnitude = value.norm();
            if magnitude > max_magnitude {
                max_magnitude = magnitude;
            }
        }
    }

    // Convert spectrogram values to pixel intensities in the range [0, 255].
    for (i, window) in spectrogram.iter().enumerate() {
        for (j, &value) in window.iter().enumerate() {
            let magnitude = value.norm();
            let intensity = if max_magnitude > 0.0 {
                (255.0 * (magnitude / max_magnitude)).floor() as u8
            } else {
                0
            };
            // Set the pixel at (j, i). Note: j corresponds to x (columns), i to y (rows).
            img.put_pixel(j as u32, i as u32, Luma([intensity]));
        }
    }

    // Save the image as a PNG file.
    let file = File::create(output_path)?;
    let w = BufWriter::new(file);
    let encoder = PngEncoder::new(w);
    encoder.write_image(
        &img.as_raw(),
        img.width(),
        img.height(),
        ExtendedColorType::L8,
    )?;

    Ok(())
}
