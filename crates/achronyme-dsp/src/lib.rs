//! Digital Signal Processing module for Achronyme
//!
//! Provides DSP operations using rustfft:
//! - Fast Fourier Transform (FFT)
//! - Inverse FFT
//! - Convolution
//! - Windowing functions
//!
//! Phase 3 of the Rust refactor

pub mod fft;
pub mod convolution;
pub mod windows;

// Re-exports for convenience
pub use fft::{fft_transform, ifft_transform, fft_real, ifft_real};
pub use convolution::{convolve, convolve_fft};
pub use windows::{hanning_window, hamming_window, blackman_window, rectangular_window, apply_window};
