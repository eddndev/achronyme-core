import createAchronymeModule from './dist/achronyme-core.mjs';

console.log('Loading WASM module...');
const Module = await createAchronymeModule();

console.log('\n=== Achronyme Core - Phase 4B: DFT (Discrete Fourier Transform) ===\n');

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

console.log('--- DFT of Simple Signals ---\n');

test('[dft-1] DFT of impulse [1, 0, 0, 0]',
    'dft_mag([1, 0, 0, 0])',
    /1\.0/);  // All frequencies should have magnitude 1

test('[dft-2] DFT of constant [1, 1, 1, 1]',
    'dft_mag([1, 1, 1, 1])',
    /4\.0.*0\.0.*0\.0.*0\.0/);  // DC component = 4, rest = 0

test('[dft-3] DFT magnitude spectrum',
    'dft_mag([1, 2, 3, 4])',
    null);  // Just verify it runs

console.log('--- DFT Full Output (Matrix) ---\n');

test('[dft-full-1] DFT returns matrix [N x 2]',
    'dft([1, 0, 0, 0])',
    /\[/);  // Should return a matrix

test('[dft-full-2] DFT of [1, 1]',
    'dft([1, 1])',
    null);  // Real: [2, 0], Imag: [0, 0]

console.log('--- DFT Phase Spectrum ---\n');

test('[dft-phase-1] Phase of real signal',
    'dft_phase([1, 1, 1, 1])',
    /0\.0/);  // Real signals have zero phase (or π)

test('[dft-phase-2] Phase spectrum',
    'dft_phase([1, 2, 3, 4])',
    null);  // Just verify it runs

console.log('--- DFT Properties ---\n');

test('[dft-prop-1] DFT size matches input',
    'dft_mag([1, 2, 3, 4, 5])',
    /15\.000000/);  // Should have 5 elements, first is DC = sum

test('[dft-prop-2] DC component of constant',
    'let signal = [2, 2, 2, 2]',
    /2/);  // Set up signal

test('[dft-prop-3] DC magnitude is N * value',
    'dft_mag([2, 2, 2, 2])',
    /8\.0/);  // DC = 4 * 2 = 8

console.log('--- DFT with Sinusoidal Signals ---\n');

test('[dft-sin-1] Single frequency - DC',
    'dft_mag([1, 1, 1, 1, 1, 1, 1, 1])',
    /8\.0.*0\.0/);  // DC component = 8, others ~0

test('[dft-sin-2] Nyquist frequency',
    'dft_mag([1, -1, 1, -1])',
    /0\.0.*0\.0.*4\.0.*0\.0/);  // Energy at Nyquist (bin 2)

console.log('--- DFT Use Cases ---\n');

test('[dft-use-1] Frequency analysis',
    'pipe([1, 2, 3, 4, 3, 2, 1, 0], v => dft_mag(v), v => map(x => round(x), v))',
    null);  // Analyze and round magnitudes

test('[dft-use-2] Peak detection',
    'pipe([1, 1, 1, 1], v => dft_mag(v), v => reduce((acc, x) => max(acc, x), 0, v))',
    /4/);  // Max magnitude should be DC = 4

console.log('\n=== DFT Tests Complete ===\n');
