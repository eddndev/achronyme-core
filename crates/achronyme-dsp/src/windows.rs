use achronyme_types::vector::Vector;
use std::f64::consts::PI;

/// Generate a Hanning window
///
/// Also known as Hann window, commonly used in spectral analysis.
/// w(n) = 0.5 * (1 - cos(2πn/(N-1)))
///
/// # Arguments
/// * `n` - Window length
///
/// # Returns
/// Hanning window vector
///
/// # Example
/// ```
/// use achronyme_dsp::hanning_window;
///
/// let window = hanning_window(128);
/// ```
pub fn hanning_window(n: usize) -> Vector {
    if n == 0 {
        return Vector::new(vec![]);
    }
    if n == 1 {
        return Vector::new(vec![1.0]);
    }
    
    let mut window = Vec::with_capacity(n);
    let n_minus_1 = (n - 1) as f64;
    
    for i in 0..n {
        let value = 0.5 * (1.0 - (2.0 * PI * i as f64 / n_minus_1).cos());
        window.push(value);
    }
    
    Vector::new(window)
}

/// Generate a Hamming window
///
/// Similar to Hanning but with different coefficients.
/// w(n) = 0.54 - 0.46 * cos(2πn/(N-1))
///
/// # Arguments
/// * `n` - Window length
///
/// # Returns
/// Hamming window vector
///
/// # Example
/// ```
/// use achronyme_dsp::hamming_window;
///
/// let window = hamming_window(256);
/// ```
pub fn hamming_window(n: usize) -> Vector {
    if n == 0 {
        return Vector::new(vec![]);
    }
    if n == 1 {
        return Vector::new(vec![1.0]);
    }
    
    let mut window = Vec::with_capacity(n);
    let n_minus_1 = (n - 1) as f64;
    
    for i in 0..n {
        let value = 0.54 - 0.46 * (2.0 * PI * i as f64 / n_minus_1).cos();
        window.push(value);
    }
    
    Vector::new(window)
}

/// Generate a Blackman window
///
/// Provides better sidelobe suppression than Hanning/Hamming.
/// w(n) = 0.42 - 0.5*cos(2πn/(N-1)) + 0.08*cos(4πn/(N-1))
///
/// # Arguments
/// * `n` - Window length
///
/// # Returns
/// Blackman window vector
///
/// # Example
/// ```
/// use achronyme_dsp::blackman_window;
///
/// let window = blackman_window(512);
/// ```
pub fn blackman_window(n: usize) -> Vector {
    if n == 0 {
        return Vector::new(vec![]);
    }
    if n == 1 {
        return Vector::new(vec![1.0]);
    }
    
    let mut window = Vec::with_capacity(n);
    let n_minus_1 = (n - 1) as f64;
    
    for i in 0..n {
        let t = 2.0 * PI * i as f64 / n_minus_1;
        let value = 0.42 - 0.5 * t.cos() + 0.08 * (2.0 * t).cos();
        window.push(value);
    }
    
    Vector::new(window)
}

/// Generate a rectangular (boxcar) window
///
/// All values are 1.0 (no windowing).
///
/// # Arguments
/// * `n` - Window length
///
/// # Returns
/// Rectangular window vector
///
/// # Example
/// ```
/// use achronyme_dsp::rectangular_window;
///
/// let window = rectangular_window(64);
/// ```
pub fn rectangular_window(n: usize) -> Vector {
    Vector::new(vec![1.0; n])
}

/// Apply a window to a signal (element-wise multiplication)
///
/// # Arguments
/// * `signal` - Input signal
/// * `window` - Window function (must have same length as signal)
///
/// # Returns
/// Windowed signal
///
/// # Example
/// ```
/// use achronyme_dsp::{apply_window, hanning_window};
/// use achronyme_types::vector::Vector;
///
/// let signal = Vector::new(vec![1.0, 2.0, 3.0, 4.0]);
/// let window = hanning_window(4);
/// let windowed = apply_window(&signal, &window).unwrap();
/// ```
pub fn apply_window(signal: &Vector, window: &Vector) -> Result<Vector, String> {
    if signal.len() != window.len() {
        return Err(format!(
            "Signal length ({}) must match window length ({})",
            signal.len(),
            window.len()
        ));
    }

    let result: Vec<f64> = signal
        .data()
        .iter()
        .zip(window.data().iter())
        .map(|(s, w)| s * w)
        .collect();

    Ok(Vector::new(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_hanning_window() {
        let window = hanning_window(5);
        
        assert_eq!(window.len(), 5);
        
        // Hanning window should start and end at 0
        assert_relative_eq!(window.data()[0], 0.0, epsilon = 1e-10);
        assert_relative_eq!(window.data()[4], 0.0, epsilon = 1e-10);
        
        // Middle value should be 1.0
        assert_relative_eq!(window.data()[2], 1.0, epsilon = 1e-10);
    }

    #[test]
    fn test_hamming_window() {
        let window = hamming_window(5);
        
        assert_eq!(window.len(), 5);
        
        // Hamming window should have non-zero endpoints
        assert!(window.data()[0] > 0.0);
        assert!(window.data()[4] > 0.0);
    }

    #[test]
    fn test_blackman_window() {
        let window = blackman_window(5);
        
        assert_eq!(window.len(), 5);
        
        // Blackman window should start and end near 0
        assert!(window.data()[0] < 0.01);
        assert!(window.data()[4] < 0.01);
    }

    #[test]
    fn test_rectangular_window() {
        let window = rectangular_window(10);
        
        assert_eq!(window.len(), 10);
        
        for &val in window.data() {
            assert_relative_eq!(val, 1.0, epsilon = 1e-10);
        }
    }

    #[test]
    fn test_apply_window() {
        let signal = Vector::new(vec![1.0, 2.0, 3.0, 4.0]);
        let window = Vector::new(vec![1.0, 0.5, 0.5, 1.0]);
        
        let result = apply_window(&signal, &window).unwrap();
        
        assert_relative_eq!(result.data()[0], 1.0, epsilon = 1e-10);
        assert_relative_eq!(result.data()[1], 1.0, epsilon = 1e-10);
        assert_relative_eq!(result.data()[2], 1.5, epsilon = 1e-10);
        assert_relative_eq!(result.data()[3], 4.0, epsilon = 1e-10);
    }

    #[test]
    fn test_apply_window_length_mismatch() {
        let signal = Vector::new(vec![1.0, 2.0, 3.0]);
        let window = Vector::new(vec![1.0, 1.0]);
        
        let result = apply_window(&signal, &window);
        assert!(result.is_err());
    }

    #[test]
    fn test_window_edge_cases() {
        // Zero-length window
        let w0 = hanning_window(0);
        assert_eq!(w0.len(), 0);
        
        // Single-element window
        let w1 = hanning_window(1);
        assert_eq!(w1.len(), 1);
        assert_relative_eq!(w1.data()[0], 1.0, epsilon = 1e-10);
    }
}
