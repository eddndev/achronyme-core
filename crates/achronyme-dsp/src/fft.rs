use achronyme_types::complex::Complex;
use rustfft::{FftPlanner, num_complex::Complex64};

/// Convert Achronyme Complex vector to rustfft Complex64 vector
fn to_complex64(input: &[Complex]) -> Vec<Complex64> {
    input
        .iter()
        .map(|c| Complex64::new(c.re, c.im))
        .collect()
}

/// Convert rustfft Complex64 vector to Achronyme Complex vector
fn from_complex64(input: &[Complex64]) -> Vec<Complex> {
    input
        .iter()
        .map(|c| Complex::new(c.re, c.im))
        .collect()
}

/// Fast Fourier Transform
///
/// Computes the FFT of a complex-valued signal.
///
/// # Arguments
/// * `input` - Input complex signal (length should be power of 2 for best performance)
///
/// # Returns
/// FFT of the input signal (same length)
///
/// # Example
/// ```
/// use achronyme_dsp::fft_transform;
/// use achronyme_types::complex::Complex;
///
/// let signal = vec![
///     Complex::new(1.0, 0.0),
///     Complex::new(0.0, 0.0),
///     Complex::new(-1.0, 0.0),
///     Complex::new(0.0, 0.0),
/// ];
///
/// let spectrum = fft_transform(&signal);
/// ```
pub fn fft_transform(input: &[Complex]) -> Vec<Complex> {
    let mut buffer = to_complex64(input);
    
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(buffer.len());
    
    fft.process(&mut buffer);
    
    from_complex64(&buffer)
}

/// Inverse Fast Fourier Transform
///
/// Computes the inverse FFT of a complex-valued spectrum.
///
/// # Arguments
/// * `input` - Input complex spectrum (length should be power of 2 for best performance)
///
/// # Returns
/// IFFT of the input spectrum (same length), scaled by 1/N
///
/// # Example
/// ```
/// use achronyme_dsp::{fft_transform, ifft_transform};
/// use achronyme_types::complex::Complex;
///
/// let signal = vec![
///     Complex::new(1.0, 0.0),
///     Complex::new(2.0, 0.0),
///     Complex::new(3.0, 0.0),
///     Complex::new(4.0, 0.0),
/// ];
///
/// let spectrum = fft_transform(&signal);
/// let reconstructed = ifft_transform(&spectrum);
/// // reconstructed should be close to signal
/// ```
pub fn ifft_transform(input: &[Complex]) -> Vec<Complex> {
    let mut buffer = to_complex64(input);
    let n = buffer.len();
    
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_inverse(n);
    
    fft.process(&mut buffer);
    
    // Scale by 1/N
    let scale = 1.0 / (n as f64);
    for x in &mut buffer {
        *x *= scale;
    }
    
    from_complex64(&buffer)
}

/// FFT for real-valued signals
///
/// Convenience function that converts real vector to complex and computes FFT.
///
/// # Arguments
/// * `input` - Real-valued signal (slice)
///
/// # Returns
/// Complex FFT of the input
///
/// # Example
/// ```
/// use achronyme_dsp::fft_real;
///
/// let signal = vec![1.0, 0.0, -1.0, 0.0];
/// let spectrum = fft_real(&signal);
/// ```
pub fn fft_real(input: &[f64]) -> Vec<Complex> {
    let complex_input: Vec<Complex> = input
        .iter()
        .map(|&x| Complex::new(x, 0.0))
        .collect();

    fft_transform(&complex_input)
}

/// Inverse FFT returning real part only
///
/// Computes IFFT and returns only the real parts (assumes input is conjugate symmetric).
///
/// # Arguments
/// * `input` - Complex spectrum (should be conjugate symmetric)
///
/// # Returns
/// Real-valued signal (real parts of IFFT)
///
/// # Example
/// ```
/// use achronyme_dsp::{fft_real, ifft_real};
///
/// let signal = vec![1.0, 2.0, 3.0, 4.0];
/// let spectrum = fft_real(&signal);
/// let reconstructed = ifft_real(&spectrum);
/// ```
pub fn ifft_real(input: &[Complex]) -> Vec<f64> {
    let result = ifft_transform(input);
    result.iter().map(|c| c.re).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_fft_ifft_roundtrip() {
        let signal = vec![
            Complex::new(1.0, 0.0),
            Complex::new(2.0, 0.0),
            Complex::new(3.0, 0.0),
            Complex::new(4.0, 0.0),
        ];

        let spectrum = fft_transform(&signal);
        let reconstructed = ifft_transform(&spectrum);

        assert_eq!(reconstructed.len(), signal.len());
        
        for (orig, recon) in signal.iter().zip(reconstructed.iter()) {
            assert_relative_eq!(orig.re, recon.re, epsilon = 1e-10);
            assert_relative_eq!(orig.im, recon.im, epsilon = 1e-10);
        }
    }

    #[test]
    fn test_fft_real() {
        let signal = vec![1.0, 0.0, -1.0, 0.0];
        let spectrum = fft_real(&signal);

        assert_eq!(spectrum.len(), 4);
    }

    #[test]
    fn test_fft_dc_component() {
        // DC signal (constant)
        let signal = vec![
            Complex::new(1.0, 0.0),
            Complex::new(1.0, 0.0),
            Complex::new(1.0, 0.0),
            Complex::new(1.0, 0.0),
        ];

        let spectrum = fft_transform(&signal);
        
        // DC component should be in first bin
        assert_relative_eq!(spectrum[0].re, 4.0, epsilon = 1e-10);
        
        // Other components should be near zero
        for i in 1..spectrum.len() {
            assert_relative_eq!(spectrum[i].magnitude(), 0.0, epsilon = 1e-10);
        }
    }

    #[test]
    fn test_fft_sine_wave() {
        use std::f64::consts::PI;
        
        // Single frequency sine wave
        let n = 16;
        let freq = 2.0; // 2 cycles in N samples
        let signal: Vec<Complex> = (0..n)
            .map(|i| {
                let t = i as f64;
                let val = (2.0 * PI * freq * t / (n as f64)).sin();
                Complex::new(val, 0.0)
            })
            .collect();

        let spectrum = fft_transform(&signal);

        // Peak should be at bin corresponding to frequency
        // Search only in positive frequencies (first half of spectrum)
        let peak_idx = spectrum
            .iter()
            .take(n / 2)  // Only consider positive frequencies
            .enumerate()
            .max_by(|(_, a), (_, b)| {
                a.magnitude().partial_cmp(&b.magnitude()).unwrap()
            })
            .map(|(idx, _)| idx)
            .unwrap();

        // For freq=2, peak should be at bin 2
        assert_eq!(peak_idx, 2);
    }
}
