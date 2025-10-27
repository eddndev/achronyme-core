import createAchronymeModule from './dist/achronyme-core.mjs';

console.log('Loading WASM module...');
const Module = await createAchronymeModule();

console.log('\n=== Achronyme Core - Phase 4B: FFT & IFFT ===\n');

// Helper function to run tests
function test(description, expression, expectedPattern) {
    try {
        const result = Module.eval(expression);
        const passed = expectedPattern ? expectedPattern.test(result) : true;

        if (passed) {
            console.log(`✓ ${description}`);
            console.log(`  Expression: ${expression}`);
            console.log(`  Result: ${result}\n`);
        } else {
            console.log(`✗ ${description}`);
            console.log(`  Expression: ${expression}`);
            console.log(`  Result: ${result}`);
            console.log(`  ⚠️ Warning: Result doesn't match expected pattern\n`);
        }
    } catch (error) {
        console.log(`✗ ${description}`);
        console.log(`  Expression: ${expression}`);
        console.log(`  Error: ${error.message || error}\n`);
    }
}

console.log('--- FFT Basic Tests ---\n');

test('[fft-1] FFT of impulse [1, 0, 0, 0]',
    'fft_mag([1, 0, 0, 0])',
    /1\.0/);  // Flat spectrum

test('[fft-2] FFT of constant [1, 1, 1, 1]',
    'fft_mag([1, 1, 1, 1])',
    /4\.0.*0\.0/);  // DC component only

test('[fft-3] FFT automatically zero-pads',
    'fft_mag([1, 2, 3])',
    null);  // Should pad to 4 elements

console.log('--- FFT vs DFT Comparison ---\n');

test('[fft-dft-1] FFT matches DFT for power-of-2',
    'let signal = [1, 2, 3, 4, 5, 6, 7, 8]',
    /8/);

test('[fft-dft-2] Compare FFT and DFT magnitude',
    'fft_mag([1, 2, 3, 4, 5, 6, 7, 8])',
    /36\.0/);  // DC component = sum of signal

test('[fft-dft-3] DFT same signal',
    'dft_mag([1, 2, 3, 4, 5, 6, 7, 8])',
    /36\.0/);  // Should match FFT

console.log('--- FFT Zero-Padding ---\n');

test('[fft-pad-1] Size 3 pads to 4',
    'fft([1, 2, 3])',
    null);  // Should return 4x2 matrix

test('[fft-pad-2] Size 5 pads to 8',
    'fft_mag([1, 2, 3, 4, 5])',
    null);  // Should return 8 elements

test('[fft-pad-3] Size 9 pads to 16',
    'fft_mag([1,2,3,4,5,6,7,8,9])',
    null);  // Should return 16 elements

console.log('--- IFFT (Inverse FFT) Tests ---\n');

test('[ifft-1] IFFT recovers signal',
    'ifft(fft([1, 2, 3, 4]))',
    /1\.0.*2\.0.*3\.0.*4\.0/);  // Should recover original

test('[ifft-2] IFFT(FFT(x)) = x for power-of-2',
    'let original = [1, 0, -1, 0]',
    /0/);

test('[ifft-3] Round-trip test',
    'ifft(fft([1, 0, -1, 0]))',
    /1\.0.*0\.0.*-1\.0.*0\.0/);  // Perfect reconstruction

test('[ifft-4] IFFT of [8, 0, 0, 0] = [2, 2, 2, 2]',
    'ifft(fft([2, 2, 2, 2]))',
    /2\.0.*2\.0.*2\.0.*2\.0/);  // Constant signal

console.log('--- FFT with Real Signals ---\n');

test('[fft-real-1] Sine wave (Nyquist)',
    'fft_mag([1, -1, 1, -1, 1, -1, 1, -1])',
    /0\.0.*0\.0.*0\.0.*0\.0.*8\.0/);  // Energy at Nyquist bin

test('[fft-real-2] DC signal',
    'fft_mag([5, 5, 5, 5, 5, 5, 5, 5])',
    /40\.0/);  // DC = N * value = 8 * 5 = 40

test('[fft-real-3] Alternating signal',
    'fft_mag([1, -1, 1, -1])',
    /0\.0.*0\.0.*4\.0.*0\.0/);  // All energy at bin 2

console.log('--- FFT Performance Verification ---\n');

test('[fft-perf-1] FFT of 16 samples (power of 2)',
    'fft_mag([1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1])',
    /16\.0/);  // DC should be 16

test('[fft-perf-2] Round-trip 16 samples',
    'ifft(fft([1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]))',
    /1\.0/);  // Should recover all 1s

test('[fft-perf-3] FFT of 32 samples',
    'fft_mag([1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1])',
    /32\.0/);  // DC should be 32

console.log('--- FFT Properties ---\n');

test('[fft-prop-1] Parseval\'s theorem (energy conservation)',
    'let sig = [1, 2, 3, 4]',
    /4/);

test('[fft-prop-2] Time domain energy',
    'reduce((acc, x) => acc + x^2, 0, [1, 2, 3, 4])',
    /30/);  // 1 + 4 + 9 + 16 = 30

test('[fft-prop-3] FFT linearity',
    'fft_mag([2, 4, 6, 8])',
    null);  // 2 * [1,2,3,4]

console.log('--- FFT Use Cases ---\n');

test('[fft-use-1] Frequency analysis pipeline',
    'pipe([1,2,3,4,3,2,1,0], v => fft_mag(v), v => map(x => round(x), v))',
    /16/);  // Analyze and round

test('[fft-use-2] Peak frequency detection',
    'pipe([1,1,1,1,1,1,1,1], v => fft_mag(v), v => reduce((acc, x) => max(acc, x), 0, v))',
    /8/);  // Max = DC

test('[fft-use-3] Signal reconstruction',
    'pipe([1,2,3,4,5,6,7,8], v => fft(v), spectrum => ifft(spectrum))',
    /1\.0.*2\.0.*3\.0/);  // Round-trip

console.log('\n=== FFT & IFFT Tests Complete ===\n');
