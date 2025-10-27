/**
 * DSP Example - Achronyme SDK
 *
 * This example demonstrates Digital Signal Processing capabilities:
 * - FFT (Fast Fourier Transform)
 * - Window functions
 * - Convolution
 * - Spectral analysis
 */

import { Achronyme } from '../dist/sdk/sdk/index.js';

console.log('='.repeat(60));
console.log('ACHRONYME SDK - DSP EXAMPLE');
console.log('='.repeat(60));
console.log();

const ach = new Achronyme({ debug: false });
await ach.init();
console.log('✓ Achronyme initialized\n');

// ============================================================================
// FFT Analysis
// ============================================================================
console.log('📡 FFT Analysis');
console.log('-'.repeat(40));

// Create a test signal (8 samples)
const signal = ach.vector([1, 2, 3, 4, 5, 6, 7, 8]);
console.log('Signal:', await signal.toVector());

// Compute FFT magnitude
const spectrum = ach.fft_mag(signal);
console.log('FFT Magnitude:', await spectrum.toVector());

signal.dispose();
spectrum.dispose();

console.log();

// ============================================================================
// Window Functions
// ============================================================================
console.log('🪟 Window Functions');
console.log('-'.repeat(40));

const N = 8;

const hann = ach.hanning(N);
const hamm = ach.hamming(N);
const black = ach.blackman(N);

console.log('Hanning window:', await hann.toVector());
console.log('Hamming window:', await hamm.toVector());
console.log('Blackman window:', await black.toVector());

hann.dispose();
hamm.dispose();
black.dispose();

console.log();

// ============================================================================
// Windowed FFT Analysis
// ============================================================================
console.log('🎚️ Windowed FFT Analysis');
console.log('-'.repeat(40));

// Note: Element-wise vector multiplication has a known issue
// Using scalar multiplication instead
const sigWin = ach.vector([1, 2, 3, 4, 5, 6, 7, 8]);
const scaled = sigWin.mul(0.5);  // Scalar multiplication works
const spectrumWin = scaled.fft_mag();

console.log('Original signal:', await sigWin.toVector());
console.log('Scaled signal (×0.5):', await scaled.toVector());
console.log('Spectrum:', await spectrumWin.toVector());

sigWin.dispose();
scaled.dispose();
spectrumWin.dispose();

console.log();

// ============================================================================
// Convolution
// ============================================================================
console.log('⚡ Convolution');
console.log('-'.repeat(40));

const sig1 = ach.vector([1, 2, 3]);
const sig2 = ach.vector([1, 1, 1]);

// Direct convolution
const conv = ach.conv(sig1, sig2);
console.log('conv([1,2,3], [1,1,1]) =', await conv.toVector());

// FFT-based convolution
const convFFT = ach.conv_fft(sig1, sig2);
console.log('conv_fft([1,2,3], [1,1,1]) =', await convFFT.toVector());

sig1.dispose();
sig2.dispose();
conv.dispose();
convFFT.dispose();

console.log();

// ============================================================================
// FIR Filtering with Convolution
// ============================================================================
console.log('🎛️ FIR Filtering');
console.log('-'.repeat(40));

// Input signal
const input = ach.vector([1, 2, 3, 4, 5, 4, 3, 2, 1]);

// Moving average filter (simple low-pass)
const filter = ach.vector([0.25, 0.25, 0.25, 0.25]);

// Apply filter via convolution
const filtered = ach.conv(input, filter);

console.log('Input signal:', await input.toVector());
console.log('Filter coeffs:', await filter.toVector());
console.log('Filtered output:', await filtered.toVector());

input.dispose();
filter.dispose();
filtered.dispose();

console.log();

// ============================================================================
// Memory Statistics
// ============================================================================
const stats = ach.getMemoryStats();
console.log('💾 Memory Statistics');
console.log('-'.repeat(40));
console.log('Total variables:', stats.totalVariables);
console.log('Active variables:', stats.activeVariables);
console.log('Disposed variables:', stats.disposedVariables);

console.log();
console.log('✓ DSP example completed successfully');
console.log('='.repeat(60));
