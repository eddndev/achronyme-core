/**
 * Quick SDK Test - Achronyme
 *
 * Simple test to verify the SDK works correctly
 */

import { Achronyme } from './dist/sdk/sdk/index.js';

console.log('='.repeat(60));
console.log('ACHRONYME SDK - QUICK TEST');
console.log('='.repeat(60));
console.log();

let passed = 0;
let failed = 0;

function test(name, fn) {
  try {
    fn();
    console.log(`✓ ${name}`);
    passed++;
  } catch (e) {
    console.log(`✗ ${name}`);
    console.log(`  Error: ${e.message}`);
    failed++;
  }
}

async function asyncTest(name, fn) {
  try {
    await fn();
    console.log(`✓ ${name}`);
    passed++;
  } catch (e) {
    console.log(`✗ ${name}`);
    console.log(`  Error: ${e.message}`);
    failed++;
  }
}

// Initialize
const ach = new Achronyme({ debug: false });
await ach.init();
console.log('✓ Achronyme initialized\n');

console.log('Running tests...\n');

// ============================================================================
// Basic Tests
// ============================================================================
console.log('📐 Basic Operations');

await asyncTest('Number creation', async () => {
  const x = ach.number(42);
  const val = await x.toNumber();
  x.dispose();
  if (val !== 42) throw new Error(`Expected 42, got ${val}`);
});

await asyncTest('Addition', async () => {
  const a = ach.number(10);
  const b = ach.number(5);
  const c = a.add(b);
  const val = await c.toNumber();
  a.dispose();
  b.dispose();
  c.dispose();
  if (val !== 15) throw new Error(`Expected 15, got ${val}`);
});

await asyncTest('Multiplication', async () => {
  const a = ach.number(7);
  const b = a.mul(6);
  const val = await b.toNumber();
  a.dispose();
  b.dispose();
  if (val !== 42) throw new Error(`Expected 42, got ${val}`);
});

await asyncTest('Power', async () => {
  const a = ach.number(5);
  const b = a.pow(2);
  const val = await b.toNumber();
  a.dispose();
  b.dispose();
  if (val !== 25) throw new Error(`Expected 25, got ${val}`);
});

console.log();

// ============================================================================
// Vector Tests
// ============================================================================
console.log('📊 Vector Operations');

await asyncTest('Vector creation', async () => {
  const v = ach.vector([1, 2, 3, 4]);
  const val = await v.toVector();
  v.dispose();
  if (JSON.stringify(val) !== JSON.stringify([1, 2, 3, 4])) {
    throw new Error(`Vector mismatch`);
  }
});

await asyncTest('Vector addition', async () => {
  const v1 = ach.vector([1, 2, 3]);
  const v2 = ach.vector([4, 5, 6]);
  const v3 = v1.add(v2);
  const val = await v3.toVector();
  v1.dispose();
  v2.dispose();
  v3.dispose();
  if (JSON.stringify(val) !== JSON.stringify([5, 7, 9])) {
    throw new Error(`Expected [5,7,9], got ${JSON.stringify(val)}`);
  }
});

await asyncTest('Vector scalar multiplication', async () => {
  const v = ach.vector([1, 2, 3]);
  const v2 = v.mul(2);
  const val = await v2.toVector();
  v.dispose();
  v2.dispose();
  if (JSON.stringify(val) !== JSON.stringify([2, 4, 6])) {
    throw new Error(`Expected [2,4,6], got ${JSON.stringify(val)}`);
  }
});

console.log();

// ============================================================================
// Math Function Tests
// ============================================================================
console.log('🧮 Mathematical Functions');

await asyncTest('Sin function', async () => {
  const x = ach.number(0);
  const y = ach.sin(x);
  const val = await y.toNumber();
  x.dispose();
  y.dispose();
  if (Math.abs(val - 0) > 0.0001) {
    throw new Error(`Expected ~0, got ${val}`);
  }
});

await asyncTest('Sqrt function', async () => {
  const x = ach.number(16);
  const y = ach.sqrt(x);
  const val = await y.toNumber();
  x.dispose();
  y.dispose();
  if (val !== 4) throw new Error(`Expected 4, got ${val}`);
});

await asyncTest('Exp function', async () => {
  const x = ach.number(0);
  const y = ach.exp(x);
  const val = await y.toNumber();
  x.dispose();
  y.dispose();
  if (Math.abs(val - 1) > 0.0001) {
    throw new Error(`Expected ~1, got ${val}`);
  }
});

console.log();

// ============================================================================
// DSP Tests
// ============================================================================
console.log('📡 DSP Functions');

await asyncTest('FFT magnitude', async () => {
  const sig = ach.vector([1, 2, 3, 4, 5, 6, 7, 8]);
  const spec = ach.fft_mag(sig);
  const val = await spec.toVector();
  sig.dispose();
  spec.dispose();
  if (val.length !== 8) {
    throw new Error(`Expected 8 samples, got ${val.length}`);
  }
});

await asyncTest('Convolution', async () => {
  const s1 = ach.vector([1, 2, 3]);
  const s2 = ach.vector([1, 1]);
  const conv = ach.conv(s1, s2);
  const val = await conv.toVector();
  s1.dispose();
  s2.dispose();
  conv.dispose();
  if (val.length !== 4) {
    throw new Error(`Expected length 4, got ${val.length}`);
  }
});

await asyncTest('Hanning window', async () => {
  const w = ach.hanning(8);
  const val = await w.toVector();
  w.dispose();
  if (val.length !== 8) {
    throw new Error(`Expected 8 samples, got ${val.length}`);
  }
  if (Math.abs(val[0] - 0) > 0.0001) {
    throw new Error(`Expected first sample ~0, got ${val[0]}`);
  }
});

console.log();

// ============================================================================
// Functional Programming Tests
// ============================================================================
console.log('λ Functional Programming');

// Note: Reset would clear all state, so we avoid it here
// Variables are managed through dispose() instead

await asyncTest('Lambda creation', async () => {
  const fn = ach.lambda(['n'], 'n ^ 2');
  fn.dispose();
});

await asyncTest('Map function', async () => {
  const v = ach.vector([1, 2, 3, 4]);
  const mapped = ach.map('n => n ^ 2', v);
  const val = await mapped.toVector();
  v.dispose();
  mapped.dispose();
  if (JSON.stringify(val) !== JSON.stringify([1, 4, 9, 16])) {
    throw new Error(`Expected [1,4,9,16], got ${JSON.stringify(val)}`);
  }
});

// Filter function now works correctly with modulo operator (fixed 2025-10-26)
await asyncTest('Filter function', async () => {
  const v = ach.vector([1, 2, 3, 4, 5, 6]);
  const filtered = ach.filter('val => val % 2 == 0', v);
  const val = await filtered.toVector();
  v.dispose();
  filtered.dispose();
  if (JSON.stringify(val) !== JSON.stringify([2, 4, 6])) {
    throw new Error(`Expected [2,4,6], got ${JSON.stringify(val)}`);
  }
});

console.log();

// ============================================================================
// Memory Management Tests
// ============================================================================
console.log('💾 Memory Management');

test('Memory statistics', () => {
  const stats = ach.getMemoryStats();
  if (typeof stats.totalVariables !== 'number') {
    throw new Error('Invalid stats');
  }
  if (typeof stats.activeVariables !== 'number') {
    throw new Error('Invalid stats');
  }
});

test('Dispose functionality', () => {
  const x = ach.number(10);
  if (x.isDisposed()) throw new Error('Should not be disposed');
  x.dispose();
  if (!x.isDisposed()) throw new Error('Should be disposed');
});

console.log();

// ============================================================================
// Matrix Operations Tests
// ============================================================================
console.log('🔢 Matrix Operations');

asyncTest('Matrix creation', async () => {
  const m = ach.matrix([[1, 2], [3, 4]]);
  const val = await m.toMatrix();
  m.dispose();
  if (!Array.isArray(val) || val.length !== 2) {
    throw new Error('Invalid matrix');
  }
});

asyncTest('Matrix addition', async () => {
  const m1 = ach.matrix([[1, 2], [3, 4]]);
  const m2 = ach.matrix([[5, 6], [7, 8]]);
  const result = m1.add(m2);
  const val = await result.toMatrix();
  m1.dispose();
  m2.dispose();
  result.dispose();
  // Expected: [[6, 8], [10, 12]]
  if (val[0][0] !== 6 || val[1][1] !== 12) {
    throw new Error(`Expected [[6,8],[10,12]], got ${JSON.stringify(val)}`);
  }
});

asyncTest('Matrix scalar multiplication', async () => {
  const m = ach.matrix([[1, 2], [3, 4]]);
  const result = m.mul(2);
  const val = await result.toMatrix();
  m.dispose();
  result.dispose();
  // Expected: [[2, 4], [6, 8]]
  if (val[0][0] !== 2 || val[1][1] !== 8) {
    throw new Error(`Expected [[2,4],[6,8]], got ${JSON.stringify(val)}`);
  }
});

asyncTest('Matrix transpose', async () => {
  const m = ach.matrix([[1, 2, 3], [4, 5, 6]]);
  const t = m.transpose();
  const val = await t.toMatrix();
  m.dispose();
  t.dispose();
  // Expected: [[1, 4], [2, 5], [3, 6]]
  if (val.length !== 3 || val[0].length !== 2) {
    throw new Error('Invalid transpose dimensions');
  }
  if (val[0][0] !== 1 || val[0][1] !== 4 || val[2][1] !== 6) {
    throw new Error(`Expected [[1,4],[2,5],[3,6]], got ${JSON.stringify(val)}`);
  }
});

asyncTest('Matrix determinant', async () => {
  const m = ach.matrix([[1, 2], [3, 4]]);
  const det = m.det();
  const val = await det.toNumber();
  m.dispose();
  det.dispose();
  // det([[1,2],[3,4]]) = 1*4 - 2*3 = -2
  if (Math.abs(val - (-2)) > 0.0001) {
    throw new Error(`Expected -2, got ${val}`);
  }
});

asyncTest('Matrix multiplication', async () => {
  const m1 = ach.matrix([[1, 2], [3, 4]]);
  const m2 = ach.matrix([[5, 6], [7, 8]]);
  const result = m1.mul(m2);
  const val = await result.toMatrix();
  m1.dispose();
  m2.dispose();
  result.dispose();
  // [[1,2],[3,4]] * [[5,6],[7,8]] = [[19,22],[43,50]]
  if (val[0][0] !== 19 || val[1][1] !== 50) {
    throw new Error(`Expected [[19,22],[43,50]], got ${JSON.stringify(val)}`);
  }
});

console.log();

// ============================================================================
// Advanced Example: Matrix Operations Showcase
// ============================================================================
console.log('🔢 Advanced Matrix Operations Example');
console.log();

console.log('Matrix transformations pipeline:');

// Create a 3x3 rotation-like matrix
const matrix = ach.matrix([
  [1, 0, 0],
  [0, 0.707, -0.707],
  [0, 0.707, 0.707]
]);
const matrixVals = await matrix.toMatrix();
console.log('  1. Original matrix (3×3):');
console.log(`     [${matrixVals[0].map(v => v.toFixed(3)).join(', ')}]`);
console.log(`     [${matrixVals[1].map(v => v.toFixed(3)).join(', ')}]`);
console.log(`     [${matrixVals[2].map(v => v.toFixed(3)).join(', ')}]`);

// Transpose
const transposed = matrix.transpose();
const transpVals = await transposed.toMatrix();
console.log('  2. Transposed:');
console.log(`     [${transpVals[0].map(v => v.toFixed(3)).join(', ')}]`);
console.log(`     [${transpVals[1].map(v => v.toFixed(3)).join(', ')}]`);
console.log(`     [${transpVals[2].map(v => v.toFixed(3)).join(', ')}]`);

// Scale by 2
const scaled = matrix.mul(2);
const scaledVals = await scaled.toMatrix();
console.log('  3. Scaled by 2:');
console.log(`     [${scaledVals[0].map(v => v.toFixed(3)).join(', ')}]`);
console.log(`     [${scaledVals[1].map(v => v.toFixed(3)).join(', ')}]`);
console.log(`     [${scaledVals[2].map(v => v.toFixed(3)).join(', ')}]`);

// Determinant of 2x2 submatrix
const subMatrix = ach.matrix([[1, 2], [3, 4]]);
const det = subMatrix.det();
const detVal = await det.toNumber();
console.log(`  4. Determinant of [[1,2],[3,4]]: ${detVal.toFixed(2)}`);

// Matrix multiplication example
const m1 = ach.matrix([[1, 2], [3, 4]]);
const m2 = ach.matrix([[5, 6], [7, 8]]);
const product = m1.mul(m2);
const productVals = await product.toMatrix();
console.log('  5. Matrix multiplication:');
console.log(`     [[1,2],[3,4]] × [[5,6],[7,8]] =`);
console.log(`     [${productVals[0].map(v => v.toFixed(0)).join(', ')}]`);
console.log(`     [${productVals[1].map(v => v.toFixed(0)).join(', ')}]`);

// Cleanup
matrix.dispose();
transposed.dispose();
scaled.dispose();
subMatrix.dispose();
det.dispose();
m1.dispose();
m2.dispose();
product.dispose();
console.log();

console.log('✓ Advanced matrix operations complete!');
console.log('  This demonstrates:');
console.log('  • Matrix creation with ach.matrix()');
console.log('  • Transposition with .transpose()');
console.log('  • Scalar operations with .mul()');
console.log('  • Determinant with .det()');
console.log('  • Matrix multiplication');
console.log();

// ============================================================================
// Advanced Example: DSP Workflow with TypeScript SDK
// ============================================================================
console.log('🔬 Advanced DSP Workflow Example (TypeScript SDK)');
console.log();

// Demonstrate complete DSP pipeline using SDK
console.log('Complete DSP Pipeline:');
const dspSignal = ach.vector([1, 2, 3, 4, 5, 6, 7, 8]);
console.log('  1. Input signal:', await dspSignal.toVector());

// Apply Hanning window
const dspWindow = ach.hanning(8);
const windowVals = await dspWindow.toVector();
console.log(`  2. Hanning window: [${windowVals.slice(0, 4).map(v => v.toFixed(3)).join(', ')}, ...]`);

// Get spectrum
const dspSpectrum = dspSignal.fft_mag();
const specValues = await dspSpectrum.toVector();
console.log(`  3. FFT spectrum: [${specValues.slice(0, 4).map(v => v.toFixed(2)).join(', ')}, ...]`);

// Find peak using JavaScript (simplest approach for SDK example)
const peakMag = Math.max(...specValues);
console.log(`  4. Peak magnitude: ${peakMag.toFixed(2)}`);

dspSignal.dispose();
dspWindow.dispose();
dspSpectrum.dispose();
console.log();

console.log('✓ Advanced DSP workflow complete!');
console.log('  This demonstrates a typical pattern:');
console.log('  • Create signal with ach.vector()');
console.log('  • Generate window with ach.hanning()');
console.log('  • Compute FFT with .fft_mag()');
console.log('  • Extract values with .toVector()');
console.log('  • Process in JavaScript or use ach.reduce()');
console.log();

// ============================================================================
// Results
// ============================================================================
console.log('='.repeat(60));
console.log(`RESULTS: ${passed} passed, ${failed} failed`);

const stats = ach.getMemoryStats();
console.log(`\nMemory: ${stats.activeVariables} active, ${stats.disposedVariables} disposed`);

if (failed === 0) {
  console.log('\n✓ All tests passed!');
  console.log('='.repeat(60));
  process.exit(0);
} else {
  console.log('\n✗ Some tests failed');
  console.log('='.repeat(60));
  process.exit(1);
}
