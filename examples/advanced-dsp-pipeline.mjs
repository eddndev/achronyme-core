/**
 * Advanced DSP Pipeline Example - Achronyme SDK
 *
 * This example demonstrates a realistic DSP pipeline:
 * - Generate a test signal with multiple frequencies
 * - Apply windowing
 * - Perform FFT analysis
 * - Find dominant frequencies
 * - Apply filtering
 *
 * This showcases the power of the SDK API for real-world signal processing.
 */

import { Achronyme } from '../dist/sdk/sdk/index.js';

console.log('='.repeat(60));
console.log('ACHRONYME SDK - ADVANCED DSP PIPELINE');
console.log('='.repeat(60));
console.log();

const ach = new Achronyme({ debug: false });
await ach.init();
console.log('✓ Achronyme initialized\n');

// ============================================================================
// Step 1: Generate Test Signal
// ============================================================================
console.log('🎵 Step 1: Generate Test Signal');
console.log('-'.repeat(40));

// We'll simulate a signal with 50Hz and 120Hz components
// For simplicity, we'll use pre-computed values
// In reality, you'd generate: sin(2*pi*50*t) + 0.5*sin(2*pi*120*t)

const testSignal = ach.vector([
  0.0, 0.588, 0.951, 0.951, 0.588, 0.0, -0.588, -0.951,
  -0.951, -0.588, 0.0, 0.588, 0.951, 0.951, 0.588, 0.0
]);

console.log('Test signal (16 samples):');
const sigValues = await testSignal.toVector();
console.log(sigValues.map(v => v.toFixed(3)).join(', '));
console.log();

// ============================================================================
// Step 2: Apply Window Function
// ============================================================================
console.log('🪟 Step 2: Apply Hanning Window');
console.log('-'.repeat(40));

const window = ach.hanning(16);
const windowedSignal = testSignal.mul(window);

console.log('Window function:');
const windowValues = await window.toVector();
console.log(windowValues.map(v => v.toFixed(3)).join(', '));
console.log();

console.log('Windowed signal:');
const windowedValues = await windowedSignal.toVector();
console.log(windowedValues.map(v => v.toFixed(3)).join(', '));
console.log();

// ============================================================================
// Step 3: Perform FFT Analysis
// ============================================================================
console.log('📊 Step 3: FFT Analysis');
console.log('-'.repeat(40));

const spectrum = windowedSignal.fft_mag();
const spectrumValues = await spectrum.toVector();

console.log('FFT Magnitude (frequency bins):');
console.log(spectrumValues.map(v => v.toFixed(3)).join(', '));
console.log();

// Find peak frequency bin
const maxMagnitude = Math.max(...spectrumValues);
const peakBin = spectrumValues.indexOf(maxMagnitude);
console.log(`Peak frequency bin: ${peakBin}`);
console.log(`Peak magnitude: ${maxMagnitude.toFixed(3)}`);
console.log();

// ============================================================================
// Step 4: Design and Apply Filter
// ============================================================================
console.log('🎛️ Step 4: Design and Apply Low-Pass Filter');
console.log('-'.repeat(40));

// Simple 5-tap moving average filter (low-pass)
const filterCoeffs = ach.vector([0.2, 0.2, 0.2, 0.2, 0.2]);

// Apply filter to original signal
const filtered = ach.conv(testSignal, filterCoeffs);

console.log('Filter coefficients:');
console.log(await filterCoeffs.toVector());
console.log();

console.log('Filtered signal:');
const filteredValues = await filtered.toVector();
console.log(filteredValues.map(v => v.toFixed(3)).join(', '));
console.log(`Filtered signal length: ${filteredValues.length} (original + filter - 1)`);
console.log();

// ============================================================================
// Step 5: Compare Original vs Filtered Spectrum
// ============================================================================
console.log('🔬 Step 5: Compare Spectra');
console.log('-'.repeat(40));

// Need to pad filtered signal back to power of 2 for fair comparison
// For simplicity, we'll just analyze the first 16 samples
const filteredTrimmed = ach.vector(filteredValues.slice(0, 16));
const filteredSpectrum = filteredTrimmed.fft_mag();
const filteredSpecValues = await filteredSpectrum.toVector();

console.log('Original spectrum (first 8 bins):');
console.log(spectrumValues.slice(0, 8).map(v => v.toFixed(3)).join(', '));
console.log();

console.log('Filtered spectrum (first 8 bins):');
console.log(filteredSpecValues.slice(0, 8).map(v => v.toFixed(3)).join(', '));
console.log();

console.log('✓ Notice how the filtered spectrum has attenuated high frequencies');
console.log();

// ============================================================================
// Step 6: Advanced - FFT-based Convolution for Large Signals
// ============================================================================
console.log('⚡ Step 6: FFT-based Convolution (Faster)');
console.log('-'.repeat(40));

const convFFT = ach.conv_fft(testSignal, filterCoeffs);
const convFFTValues = await convFFT.toVector();

console.log('FFT-based convolution result:');
console.log(convFFTValues.slice(0, 16).map(v => v.toFixed(3)).join(', '));
console.log();

// Compare with direct convolution
const maxDiff = filteredValues.slice(0, 16).reduce((max, val, i) => {
  return Math.max(max, Math.abs(val - convFFTValues[i]));
}, 0);

console.log(`Maximum difference between direct and FFT convolution: ${maxDiff.toFixed(6)}`);
console.log('(Small differences are expected due to numerical precision)');
console.log();

// ============================================================================
// Clean Up Memory
// ============================================================================
console.log('🧹 Cleanup');
console.log('-'.repeat(40));

testSignal.dispose();
window.dispose();
windowedSignal.dispose();
spectrum.dispose();
filterCoeffs.dispose();
filtered.dispose();
filteredTrimmed.dispose();
filteredSpectrum.dispose();
convFFT.dispose();

const stats = ach.getMemoryStats();
console.log('Memory after cleanup:');
console.log(`- Active variables: ${stats.activeVariables}`);
console.log(`- Disposed variables: ${stats.disposedVariables}`);
console.log();

console.log('✓ Advanced DSP pipeline completed successfully');
console.log('='.repeat(60));
console.log();
console.log('📝 Key Takeaways:');
console.log('  • Window functions reduce spectral leakage');
console.log('  • FFT provides frequency-domain analysis');
console.log('  • Convolution can be used for filtering');
console.log('  • FFT-based convolution is faster for large signals');
console.log('  • Always dispose() values to manage memory');
console.log('='.repeat(60));
