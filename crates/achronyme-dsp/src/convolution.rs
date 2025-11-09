use crate::fft::{fft_real, ifft_real};
use achronyme_types::complex::Complex;

/// Convolve two signals using direct method
///
/// Computes the discrete convolution of two signals using the definition:
/// (f * g)[n] = Σ f[m] * g[n - m]
///
/// # Arguments
/// * `signal` - Input signal (slice)
/// * `kernel` - Convolution kernel (slice)
///
/// # Returns
/// Convolved signal (length = signal.len() + kernel.len() - 1)
///
/// # Example
/// ```
/// use achronyme_dsp::convolve;
///
/// let signal = vec![1.0, 2.0, 3.0];
/// let kernel = vec![0.5, 0.5];
///
/// let result = convolve(&signal, &kernel);
/// ```
pub fn convolve(signal: &[f64], kernel: &[f64]) -> Vec<f64> {
    let n = signal.len();
    let m = kernel.len();
    let output_len = n + m - 1;

    let mut result = vec![0.0; output_len];

    for i in 0..output_len {
        let mut sum = 0.0;
        for j in 0..m {
            if i >= j && i - j < n {
                sum += signal[i - j] * kernel[j];
            }
        }
        result[i] = sum;
    }

    result
}

/// Convolve two signals using FFT (fast convolution)
///
/// Uses FFT-based convolution which is faster for large signals.
/// Uses the convolution theorem: conv(f, g) = IFFT(FFT(f) * FFT(g))
///
/// # Arguments
/// * `signal` - Input signal (slice)
/// * `kernel` - Convolution kernel (slice)
///
/// # Returns
/// Convolved signal
///
/// # Example
/// ```
/// use achronyme_dsp::convolve_fft;
///
/// let signal = vec![1.0, 2.0, 3.0, 4.0];
/// let kernel = vec![0.25, 0.5, 0.25];
///
/// let result = convolve_fft(&signal, &kernel);
/// ```
pub fn convolve_fft(signal: &[f64], kernel: &[f64]) -> Vec<f64> {
    let n = signal.len();
    let m = kernel.len();
    let output_len = n + m - 1;

    // Pad to next power of 2 for FFT efficiency
    let fft_len = output_len.next_power_of_two();

    // Pad signals
    let mut signal_padded = signal.to_vec();
    signal_padded.resize(fft_len, 0.0);

    let mut kernel_padded = kernel.to_vec();
    kernel_padded.resize(fft_len, 0.0);

    // FFT of both signals
    let signal_fft = fft_real(&signal_padded);
    let kernel_fft = fft_real(&kernel_padded);

    // Multiply in frequency domain
    let product: Vec<Complex> = signal_fft
        .iter()
        .zip(kernel_fft.iter())
        .map(|(a, b)| *a * *b)
        .collect();

    // IFFT to get convolution result
    let result_full = ifft_real(&product);

    // Trim to actual output length
    result_full.into_iter().take(output_len).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_convolve_simple() {
        let signal = vec![1.0, 2.0, 3.0];
        let kernel = vec![1.0, 1.0];

        let result = convolve(&signal, &kernel);

        // Expected: [1, 3, 5, 3]
        assert_eq!(result.len(), 4);
        assert_relative_eq!(result[0], 1.0, epsilon = 1e-10);
        assert_relative_eq!(result[1], 3.0, epsilon = 1e-10);
        assert_relative_eq!(result[2], 5.0, epsilon = 1e-10);
        assert_relative_eq!(result[3], 3.0, epsilon = 1e-10);
    }

    #[test]
    fn test_convolve_identity() {
        let signal = vec![1.0, 2.0, 3.0, 4.0];
        let kernel = vec![1.0]; // Identity kernel

        let result = convolve(&signal, &kernel);

        assert_eq!(result.len(), 4);
        for i in 0..4 {
            assert_relative_eq!(result[i], signal[i], epsilon = 1e-10);
        }
    }

    #[test]
    fn test_convolve_averaging() {
        let signal = vec![1.0, 2.0, 3.0, 4.0];
        let kernel = vec![0.5, 0.5]; // Moving average

        let result = convolve(&signal, &kernel);

        // Expected: [0.5, 1.5, 2.5, 3.5, 2.0]
        assert_eq!(result.len(), 5);
        assert_relative_eq!(result[0], 0.5, epsilon = 1e-10);
        assert_relative_eq!(result[1], 1.5, epsilon = 1e-10);
        assert_relative_eq!(result[2], 2.5, epsilon = 1e-10);
        assert_relative_eq!(result[3], 3.5, epsilon = 1e-10);
        assert_relative_eq!(result[4], 2.0, epsilon = 1e-10);
    }

    #[test]
    fn test_convolve_fft_matches_direct() {
        let signal = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let kernel = vec![0.25, 0.5, 0.25];

        let result_direct = convolve(&signal, &kernel);
        let result_fft = convolve_fft(&signal, &kernel);

        assert_eq!(result_direct.len(), result_fft.len());

        for i in 0..result_direct.len() {
            assert_relative_eq!(
                result_direct[i],
                result_fft[i],
                epsilon = 1e-8
            );
        }
    }
}
