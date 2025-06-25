use std::collections::HashMap;
use num_complex::Complex;
use crate::models::Couple;

const MAX_FREQ_BITS: u32 = 9;
const MAX_DELTA_BITS: u32 = 14;
const TARGET_ZONE_SIZE: usize = 5;

/// Generates fingerprints from a list of peaks and associates each fingerprint (address)
/// with a couple (anchor time in ms and song ID).
pub fn fingerprint(peaks: &[Peak], song_id: u32) -> HashMap<u32, Couple> {
    let mut fingerprints = HashMap::new();

    for (i, anchor) in peaks.iter().enumerate() {
        for target in peaks.iter().skip(i + 1).take(TARGET_ZONE_SIZE) {
            let address = create_address(anchor, target);
            let anchor_time_ms = (anchor.time * 1000.0) as u32;
            fingerprints.insert(address, Couple { anchor_time_ms, song_id });
        }
    }

    fingerprints
}

/// Generates a unique address for a pair of anchor and target peaks.
/// The address is a 32-bit integer combining the integer parts of the anchor frequency,
/// target frequency, and the delta time (in ms) between them.
pub fn create_address(anchor: &Peak, target: &Peak) -> u32 {
    let anchor_freq = anchor.freq.re as u32;
    let target_freq = target.freq.re as u32;
    let delta_ms = ((target.time - anchor.time) * 1000.0) as u32;
    
    // Combine the values into a single 32-bit address.
    (anchor_freq << 23) | (target_freq << 14) | delta_ms
}

/// Dummy Peak struct for illustration purposes.
/// In your project, ensure this is defined in the proper module.
#[derive(Debug)]
pub struct Peak {
    pub time: f64,
    pub freq: Complex<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_complex::Complex;

    #[test]
    fn test_create_address() {
        // Create two dummy peaks.
        let anchor = Peak { time: 1.0, freq: Complex::new(100.0, 0.0) };
        let target = Peak { time: 1.05, freq: Complex::new(200.0, 0.0) };
        let address = create_address(&anchor, &target);
        // Verify that the address is computed as expected.
        let expected_delta = ((1.05 - 1.0) * 1000.0) as u32;
        let expected = (100 << 23) | (200 << 14) | expected_delta;
        assert_eq!(address, expected);
    }

    #[test]
    fn test_fingerprint() {
        // Create a few dummy peaks.
        let peaks = vec![
            Peak { time: 0.0, freq: Complex::new(50.0, 0.0) },
            Peak { time: 0.1, freq: Complex::new(60.0, 0.0) },
            Peak { time: 0.2, freq: Complex::new(70.0, 0.0) },
            Peak { time: 0.3, freq: Complex::new(80.0, 0.0) },
        ];
        let song_id = 42;
        let fingerprints = fingerprint(&peaks, song_id);
        // We expect some fingerprints to be generated.
        assert!(!fingerprints.is_empty());
    }
}
