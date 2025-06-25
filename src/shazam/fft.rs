use num_complex::Complex;
use std::f64::consts::PI;

/// Performs the Fast Fourier Transform on the input signal.
pub fn fft(input: &[f64]) -> Vec<Complex<f64>> {
    // Convert input to complex numbers.
    let complex_array: Vec<Complex<f64>> = input.iter().map(|&v| Complex::new(v, 0.0)).collect();
    recursive_fft(&complex_array)
}

/// Recursively computes the FFT of the given slice of complex numbers.
fn recursive_fft(data: &[Complex<f64>]) -> Vec<Complex<f64>> {
    let n = data.len();
    if n <= 1 {
        return data.to_vec();
    }

    // Split the input into even and odd elements.
    let even: Vec<Complex<f64>> = data.iter().step_by(2).cloned().collect();
    let odd: Vec<Complex<f64>> = data.iter().skip(1).step_by(2).cloned().collect();

    let fft_even = recursive_fft(&even);
    let fft_odd = recursive_fft(&odd);

    let mut result = vec![Complex::new(0.0, 0.0); n];
    for k in 0..n / 2 {
        let t = Complex::from_polar(1.0, -2.0 * PI * k as f64 / n as f64) * fft_odd[k];
        result[k] = fft_even[k] + t;
        result[k + n / 2] = fft_even[k] - t;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_complex::Complex;

    #[test]
    fn test_fft() {
        // Test with a simple input.
        let input = [1.0, 2.0, 3.0, 4.0];
        let result = fft(&input);

        // Compare with expected values (computed externally or via another library)
        // This is a basic sanity check for the length and type.
        assert_eq!(result.len(), 4);
        // You may add more rigorous tests with known FFT outputs.
    }
}
