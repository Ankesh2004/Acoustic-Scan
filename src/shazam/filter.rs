/// A first-order low-pass filter defined by H(p) = 1 / (1 + pRC)
pub struct LowPassFilter {
    alpha: f64, // Filter coefficient
    y_prev: f64, // Previous output value
}

impl LowPassFilter {
    /// Creates a new low-pass filter given a cutoff frequency and sample rate.
    pub fn new(cutoff_frequency: f64, sample_rate: f64) -> Self {
        let rc = 1.0 / (2.0 * std::f64::consts::PI * cutoff_frequency);
        let dt = 1.0 / sample_rate;
        let alpha = dt / (rc + dt);
        LowPassFilter { alpha, y_prev: 0.0 }
    }

    /// Processes the input signal through the low-pass filter.
    /// Returns a new vector containing the filtered signal.
    pub fn filter(&mut self, input: &[f64]) -> Vec<f64> {
        let mut filtered = Vec::with_capacity(input.len());
        for (i, &x) in input.iter().enumerate() {
            let y = if i == 0 {
                x * self.alpha
            } else {
                self.alpha * x + (1.0 - self.alpha) * self.y_prev
            };
            self.y_prev = y;
            filtered.push(y);
        }
        filtered
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_low_pass_filter() {
        // Create a low-pass filter with cutoff frequency of 1000 Hz and sample rate 44100 Hz.
        let mut filter = LowPassFilter::new(1000.0, 44100.0);
        // A simple impulse response: first sample is 1.0, rest are 0.
        let input = vec![1.0, 0.0, 0.0, 0.0, 0.0];
        let output = filter.filter(&input);
        // Verify that the filter produces a decaying output.
        assert!(output[0] > output[1]);
        assert!(output[1] > output[2]);
    }
}
