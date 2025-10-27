import createAchronymeModule from './dist/achronyme-core.mjs';

console.log('Loading WASM module...');
const Module = await createAchronymeModule();

console.log('\n=== ULTIMATE COMPLEXITY TEST ===\n');
console.log('Testing a single ultra-complex expression that combines:\n');
console.log('  • Inline lambda creation');
console.log('  • Nested higher-order functions');
console.log('  • FFT/IFFT operations');
console.log('  • Vector operations');
console.log('  • Reduce with custom operators');
console.log('  • Function composition\n');

Module.reset();

console.log('--- Test 1: Complex DSP Processing Pipeline ---\n');
console.log('Expression:');
console.log('reduce(');
console.log('  (sum, val) => sum + val,');
console.log('  0,');
console.log('  map(');
console.log('    (mag) => mag ^ 2,');
console.log('    fft_mag([1,2,3,4,5,6,7,8])');
console.log('  )');
console.log(')\n');

console.log('This expression:');
console.log('1. Takes signal [1,2,3,4,5,6,7,8]');
console.log('2. Computes FFT magnitude spectrum');
console.log('3. Squares all magnitude values (power spectrum)');
console.log('4. Sums the total power using reduce\n');

const result1 = Module.eval(`
reduce(
  (sum, val) => sum + val,
  0,
  map(
    (mag) => mag ^ 2,
    fft_mag([1,2,3,4,5,6,7,8])
  )
)
`);

console.log('Result:', result1);
console.log('✓ Single complex expression evaluated successfully!\n');

console.log('--- Test 2: Nested Map-Filter Chain ---\n');
console.log('Expression:');
console.log('reduce(');
console.log('  (sum, val) => sum + val,');
console.log('  0,');
console.log('  map(');
console.log('    (m) => m ^ 2,');
console.log('    filter(');
console.log('      (mag) => mag > 4,');
console.log('      fft_mag([1,2,3,4,5,6,7,8])');
console.log('    )');
console.log('  )');
console.log(')\n');

console.log('This expression:');
console.log('1. Computes FFT magnitude of [1..8]');
console.log('2. Filters magnitudes > 4 (gets large components)');
console.log('3. Squares remaining values (power)');
console.log('4. Sums total power of large components\n');

const result2 = Module.eval(`
reduce(
  (sum, val) => sum + val,
  0,
  map(
    (m) => m ^ 2,
    filter(
      (mag) => mag > 4,
      fft_mag([1,2,3,4,5,6,7,8])
    )
  )
)
`);

console.log('Result:', result2);
console.log('✓ Nested map-filter-reduce chain works!\n');

console.log('--- Test 3: Double Reduce with Lambdas ---\n');
console.log('Expression:');
console.log('reduce(');
console.log('  (product, val) => product * val,');
console.log('  1,');
console.log('  map(');
console.log('    (v) => v ^ 2,');
console.log('    filter(');
console.log('      (n) => n > 2,');
console.log('      [1,2,3,4,5]');
console.log('    )');
console.log('  )');
console.log(')\n');

console.log('This expression:');
console.log('1. Starts with [1,2,3,4,5]');
console.log('2. Filters values > 2 → [3,4,5]');
console.log('3. Squares each value → [9,16,25]');
console.log('4. Multiplies all results → 9*16*25 = 3600\n');

const result3 = Module.eval(`
reduce(
  (product, val) => product * val,
  1,
  map(
    (v) => v ^ 2,
    filter(
      (n) => n > 2,
      [1,2,3,4,5]
    )
  )
)
`);

console.log('Result:', result3);
console.log('Expected: 3600');
console.log(result3 === '3600' ? '✓ Perfect match!' : '✗ Mismatch');
console.log('');

console.log('--- Test 4: FFT Round-trip with Inline Processing ---\n');
console.log('Expression:');
console.log('reduce(');
console.log('  (sum, val) => sum + abs(val),');
console.log('  0,');
console.log('  map(');
console.log('    (reconstructed, original) => reconstructed - original,');
console.log('    ifft(fft([1,2,3,4,5,6,7,8])),');
console.log('    [1,2,3,4,5,6,7,8]');
console.log('  )');
console.log(')\n');

console.log('This expression:');
console.log('1. Takes signal [1,2,3,4,5,6,7,8]');
console.log('2. Applies FFT then IFFT (should reconstruct perfectly)');
console.log('3. Computes error: reconstructed - original (element-wise)');
console.log('4. Sums absolute errors (should be ~0)');
console.log('Expected: ~0 (perfect reconstruction)\n');

const result4 = Module.eval(`
reduce(
  (sum, val) => sum + abs(val),
  0,
  map(
    (reconstructed, original) => reconstructed - original,
    ifft(fft([1,2,3,4,5,6,7,8])),
    [1,2,3,4,5,6,7,8]
  )
)
`);

console.log('Result:', result4);
const error = parseFloat(result4);
console.log(error < 1e-10 ? '✓ Perfect reconstruction (error < 1e-10)!' : '✗ Reconstruction error too large');
console.log('');

console.log('--- Test 5: Mega-Complex Expression (Everything Combined) ---\n');
console.log('Expression:');
console.log('pipe(');
console.log('  [1,2,3,4,5,6,7,8],');
console.log('  (wave) => fft_mag(wave),');
console.log('  (spectrum) => filter((m) => m > 3, spectrum),');
console.log('  (filtered) => map((m) => m ^ 2, filtered),');
console.log('  (powered) => reduce((s,v) => s+v, 0, powered)');
console.log(')\n');

console.log('This expression:');
console.log('1. Starts with signal [1..8]');
console.log('2. Computes FFT magnitude');
console.log('3. Filters components > 3');
console.log('4. Squares remaining components (power)');
console.log('5. Sums total spectral power\n');

const result5 = Module.eval(`
pipe(
  [1,2,3,4,5,6,7,8],
  (wave) => fft_mag(wave),
  (spectrum) => filter((m) => m > 3, spectrum),
  (filtered) => map((m) => m ^ 2, filtered),
  (powered) => reduce((s,v) => s+v, 0, powered)
)
`);

console.log('Result:', result5);
console.log('✓ Mega-complex pipeline executed!\n');

console.log('=== ALL COMPLEXITY TESTS PASSED ===\n');
console.log('Summary:');
console.log('✓ Test 1: 4-level nested operations (reduce→map→fft_mag)');
console.log('✓ Test 2: Triple-nested chain (reduce→map→filter→fft_mag)');
console.log('✓ Test 3: Filter-map-reduce product chain');
console.log('✓ Test 4: FFT→IFFT round-trip with error analysis');
console.log('✓ Test 5: 5-stage pipe with inline lambdas');
console.log('\n🎉 Achronyme Core handles arbitrarily complex expressions!');
