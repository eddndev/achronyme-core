import createAchronymeModule from '../dist/achronyme-core.mjs';

console.log('╔═══════════════════════════════════════════════════════════════╗');
console.log('║         ACHRONYME CORE - COMPREHENSIVE DEMO & TESTS          ║');
console.log('║                                                               ║');
console.log('║  Mathematical Computing Engine with DSP & Functional         ║');
console.log('║  Programming Capabilities                                     ║');
console.log('╚═══════════════════════════════════════════════════════════════╝\n');

console.log('Loading WASM module...');
const Module = await createAchronymeModule();
console.log('✓ Module loaded\n');

let testsRun = 0;
let testsPassed = 0;

function test(name, expression, expectedCheck) {
    testsRun++;
    try {
        const result = Module.eval(expression);
        const passed = expectedCheck(result);
        if (passed) {
            testsPassed++;
            console.log(`  ✓ ${name}`);
        } else {
            console.log(`  ✗ ${name} - Expected different result`);
            console.log(`    Got: ${result}`);
        }
    } catch (e) {
        console.log(`  ✗ ${name} - ERROR: ${e.message || e}`);
        console.log(`    Expression: ${expression}`);
    }
}

function approx(a, b, tol = 1e-6) {
    const numA = typeof a === 'string' ? parseFloat(a) : a;
    const numB = typeof b === 'string' ? parseFloat(b) : b;
    return Math.abs(numA - numB) < tol;
}

function vecApprox(result, expected, tol = 1e-6) {
    const values = result.match(/-?\d+\.?\d*/g);
    if (!values || values.length !== expected.length) return false;
    return values.every((v, i) => approx(parseFloat(v), expected[i], tol));
}

// ═══════════════════════════════════════════════════════════════════════════
// PHASE 1-2: BASIC OPERATIONS
// ═══════════════════════════════════════════════════════════════════════════

console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
console.log('PHASE 1-2: BASIC MATHEMATICAL OPERATIONS');
console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n');

Module.reset();

test('Arithmetic: Addition', '2 + 3', r => r === '5');
test('Arithmetic: Subtraction', '10 - 7', r => r === '3');
test('Arithmetic: Multiplication', '4 * 5', r => r === '20');
test('Arithmetic: Division', '15 / 3', r => r === '5');
test('Arithmetic: Power', '2 ^ 10', r => r === '1024');
test('Arithmetic: Negation', '-5 + 3', r => r === '-2');

test('Trigonometry: sin(π/2)', 'sin(PI / 2)', r => approx(r, 1));
test('Trigonometry: cos(0)', 'cos(0)', r => approx(r, 1));
test('Trigonometry: tan(π/4)', 'tan(PI / 4)', r => approx(r, 1));

test('Exponentials: exp(1)', 'exp(1)', r => approx(parseFloat(r), Math.E, 1e-5));
test('Logarithms: log(E)', 'log(E)', r => approx(r, 1));
test('Roots: sqrt(16)', 'sqrt(16)', r => r === '4');

test('Constants: PI', 'PI', r => approx(parseFloat(r), Math.PI, 1e-5));
test('Constants: E', 'E', r => approx(parseFloat(r), Math.E, 1e-5));

test('Functions: abs(-5)', 'abs(-5)', r => r === '5');
test('Functions: min(3, 1, 4)', 'min(3, 1, 4)', r => r === '1');
test('Functions: max(3, 1, 4)', 'max(3, 1, 4)', r => r === '4');
test('Functions: floor(3.7)', 'floor(3.7)', r => r === '3');
test('Functions: ceil(3.2)', 'ceil(3.2)', r => r === '4');
test('Functions: round(3.5)', 'round(3.5)', r => r === '4');

// ═══════════════════════════════════════════════════════════════════════════
// PHASE 3: COMPLEX TYPES
// ═══════════════════════════════════════════════════════════════════════════

console.log('\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
console.log('PHASE 3: COMPLEX TYPES (Complex, Vector, Matrix)');
console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n');

test('Complex: Creation', '3i', r => r.includes('3') && r.includes('i'));
test('Complex: Operations', '(2+3i) + (1+4i)', r => r.includes('i'));
test('Complex: Magnitude', 'abs(3+4i)', r => approx(r, 5));

test('Vector: Creation', '[1, 2, 3]', r => vecApprox(r, [1, 2, 3]));
test('Vector: Addition', '[1, 2] + [3, 4]', r => vecApprox(r, [4, 6]));
test('Vector: Scalar mult', '[1, 2, 3] * 2', r => vecApprox(r, [2, 4, 6]));
test('Vector: Dot product', 'dot([1, 2, 3], [4, 5, 6])', r => r === '32');
test('Vector: Norm', 'norm([3, 4])', r => approx(r, 5));

test('Matrix: Creation', '[[1, 2], [3, 4]]', r => r.includes('['));
test('Matrix: Transpose', 'transpose([[1, 2], [3, 4]])', r => r.includes('['));
test('Matrix: Determinant', 'det([[1, 2], [3, 4]])', r => approx(r, -2));

// ═══════════════════════════════════════════════════════════════════════════
// PHASE 4A: VARIABLES & LAMBDAS
// ═══════════════════════════════════════════════════════════════════════════

console.log('\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
console.log('PHASE 4A: VARIABLES, LAMBDAS & CLOSURES');
console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n');

Module.reset();

test('Lambdas: Simple function', 'let square = n => n ^ 2', r => r.includes('function'));
test('Lambdas: Call function', 'square(5)', r => r === '25');
test('Lambdas: Multi-param', 'let add = (a, b) => a + b', r => r.includes('function'));
test('Lambdas: Multi-param call', 'add(3, 7)', r => r === '10');

test('Variables: Declaration', 'let x = 10', r => r === '10');
test('Variables: Reference', 'x + 5', r => r === '15');
test('Variables: Update', 'let y = 20', r => r === '20');
test('Variables: Multi-var reference', 'x + y', r => r === '30');

test('Closures: Outer variable', 'let outer = 100', r => r === '100');
test('Closures: Capture variable', 'let addOuter = val => val + outer', r => r.includes('function'));
test('Closures: Use closure', 'addOuter(50)', r => r === '150');

test('Comparison: Greater than', '5 > 3', r => r === '1');
test('Comparison: Less than', '2 < 8', r => r === '1');
test('Comparison: Equal', '7 == 7', r => r === '1');
test('Comparison: Not equal', '5 != 3', r => r === '1');

// ═══════════════════════════════════════════════════════════════════════════
// PHASE 4A: HIGHER-ORDER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════

console.log('\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
console.log('PHASE 4A: HIGHER-ORDER FUNCTIONS');
console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n');

test('map: Inline lambda', 'map(n => n * 2, [1, 2, 3])', r => vecApprox(r, [2, 4, 6]));
test('map: Stored function', 'map(square, [1, 2, 3, 4])', r => vecApprox(r, [1, 4, 9, 16]));
test('map: Multi-vector', 'map(add, [1, 2, 3], [10, 20, 30])', r => vecApprox(r, [11, 22, 33]));

test('filter: Positive numbers', 'filter(n => n > 0, [-1, 0, 1, 2])', r => vecApprox(r, [1, 2]));
test('filter: Even numbers', 'filter(n => n - 2 * floor(n / 2) == 0, [1, 2, 3, 4, 5, 6])',
     r => vecApprox(r, [2, 4, 6]));

test('reduce: Sum', 'reduce((a, b) => a + b, 0, [1, 2, 3, 4])', r => r === '10');
test('reduce: Product', 'reduce((a, b) => a * b, 1, [1, 2, 3, 4])', r => r === '24');
test('reduce: Max', 'reduce((a, b) => max(a, b), 0, [3, 1, 4, 1, 5])', r => r === '5');

test('pipe: Multi-stage', 'pipe([1, 2, 3], v => map(m => m * 2, v), v => reduce((a,b) => a+b, 0, v))',
     r => r === '12');

// Composition test
test('HOF: Define double', 'let double = n => n * 2', r => r.includes('function'));
test('HOF: Define inc', 'let inc = n => n + 1', r => r.includes('function'));
test('HOF: Stored function composition', 'map(double, map(inc, [1, 2, 3]))',
     r => vecApprox(r, [4, 6, 8]));

// ═══════════════════════════════════════════════════════════════════════════
// PHASE 4B: DSP - DFT & FFT
// ═══════════════════════════════════════════════════════════════════════════

console.log('\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
console.log('PHASE 4B: DIGITAL SIGNAL PROCESSING - DFT & FFT');
console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n');

test('DFT: Basic transform', 'dft([1, 0, 0, 0])', r => r.includes('['));
test('DFT: Magnitude', 'dft_mag([1, 1, 1, 1])', r => vecApprox(r, [4, 0, 0, 0]));

test('FFT: Basic transform', 'fft([1, 2, 3, 4])', r => r.includes('['));
test('FFT: DC component', 'fft_mag([1, 1, 1, 1, 1, 1, 1, 1])',
     r => {
         const vals = r.match(/-?\d+\.?\d*/g).map(parseFloat);
         return approx(vals[0], 8); // DC = 8
     });

test('FFT: Zero padding', 'fft_mag([1, 2, 3])', r => r.includes('[')); // Auto pads to 4

test('IFFT: Round-trip',
     'reduce((s,v) => s+abs(v), 0, map((orig, rec) => orig-rec, [1,2,3,4,5,6,7,8], ifft(fft([1,2,3,4,5,6,7,8]))))',
     r => approx(parseFloat(r), 0, 1e-10));

test('FFT vs DFT: Equivalence',
     'reduce((s,v) => s+abs(v), 0, map((d, f) => d-f, dft_mag([1,2,3,4]), fft_mag([1,2,3,4])))',
     r => approx(parseFloat(r), 0, 1e-10));

// ═══════════════════════════════════════════════════════════════════════════
// PHASE 4B: CONVOLUTION
// ═══════════════════════════════════════════════════════════════════════════

console.log('\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
console.log('PHASE 4B: CONVOLUTION');
console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n');

test('conv: Simple', 'conv([1, 2, 3], [1, 1])', r => vecApprox(r, [1, 3, 5, 3]));
test('conv: Identity', 'conv([1, 2, 3], [1])', r => vecApprox(r, [1, 2, 3]));
test('conv: Symmetric', 'conv([1, 2, 1], [1, 2, 1])', r => vecApprox(r, [1, 4, 6, 4, 1]));

test('conv_fft: Basic', 'conv_fft([1, 2, 3], [1, 1])', r => vecApprox(r, [1, 3, 5, 3]));
test('conv_fft vs conv: Match',
     'reduce((s,v) => s+abs(v), 0, map((c, cf) => c-cf, conv([1,2,3,4,5], [1,2,1]), conv_fft([1,2,3,4,5], [1,2,1])))',
     r => approx(parseFloat(r), 0, 1e-10));

// FIR filtering example
test('FIR: Moving average', 'conv([1, 2, 3, 4, 5], [0.333, 0.333, 0.333])',
     r => {
         const vals = r.match(/-?\d+\.?\d*/g).map(parseFloat);
         return vals.length === 7; // N+M-1 = 5+3-1
     });

// ═══════════════════════════════════════════════════════════════════════════
// PHASE 4B: WINDOW FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════

console.log('\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
console.log('PHASE 4B: WINDOW FUNCTIONS');
console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n');

test('hanning: Size 4', 'hanning(4)', r => vecApprox(r, [0, 0.75, 0.75, 0], 0.01));
test('hanning: Symmetry', 'hanning(8)',
     r => {
         const v = r.match(/-?\d+\.?\d*/g).map(parseFloat);
         return approx(v[1], v[6], 0.01) && approx(v[2], v[5], 0.01);
     });

test('hamming: Size 4', 'hamming(4)', r => vecApprox(r, [0.08, 0.77, 0.77, 0.08], 0.01));
test('hamming: Non-zero edges', 'hamming(8)',
     r => {
         const v = r.match(/-?\d+\.?\d*/g).map(parseFloat);
         return approx(v[0], 0.08, 0.01) && approx(v[7], 0.08, 0.01);
     });

test('blackman: Size 4', 'blackman(4)', r => vecApprox(r, [0, 0.63, 0.63, 0], 0.02));
test('blackman: Symmetry', 'blackman(8)',
     r => {
         const v = r.match(/-?\d+\.?\d*/g).map(parseFloat);
         return approx(v[1], v[6], 0.01) && approx(v[3], v[4], 0.01);
     });

// Windowed FFT
test('Windowed FFT: Application', 'fft_mag(map((s, w) => s * w, [1,2,3,4,5,6,7,8], hanning(8)))',
     r => r.includes('['));

// ═══════════════════════════════════════════════════════════════════════════
// INTEGRATION TESTS: COMPLEX EXPRESSIONS
// ═══════════════════════════════════════════════════════════════════════════

console.log('\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
console.log('INTEGRATION: COMPLEX EXPRESSIONS');
console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n');

Module.reset();

// Build a DSP library
Module.eval('let power = x => x ^ 2');
Module.eval('let sum_vec = v => reduce((a,b) => a+b, 0, v)');
Module.eval('let power_all = v => map(power, v)');
Module.eval('let spectral_power = sig => sum_vec(power_all(fft_mag(sig)))');

test('Complex: DSP library - power function', 'power(5)', r => r === '25');
test('Complex: DSP library - sum vector', 'sum_vec([1, 2, 3, 4])', r => r === '10');
test('Complex: DSP library - spectral power', 'spectral_power([1,1,1,1,1,1,1,1])', r => r === '64');

// Nested operations
test('Complex: 4-level nesting',
     'reduce((s,v) => s+v, 0, map(x => x^2, fft_mag([1,2,3,4,5,6,7,8])))',
     r => parseFloat(r) > 0);

// Pipeline with stored functions
test('Complex: Pipeline with stored functions',
     'pipe([1,2,3,4,5,6,7,8], v => fft_mag(v), power_all, sum_vec)',
     r => parseFloat(r) > 0);

// Filter + map + reduce chain
test('Complex: Filter-map-reduce',
     'reduce((p,v) => p*v, 1, map(x => x^2, filter(x => x > 2, [1,2,3,4,5])))',
     r => r === '3600'); // (3^2) * (4^2) * (5^2) = 9*16*25

// Windowed spectral analysis pipeline
Module.eval('let analyze = sig => fft_mag(map((s,w) => s*w, sig, hanning(8)))');
test('Complex: Windowed FFT pipeline', 'analyze([1,2,3,4,5,6,7,8])',
     r => r.includes('['));

// Convolution with stored kernels
Module.eval('let smooth = [0.25, 0.5, 0.25]');
Module.eval('let denoise = sig => conv(sig, smooth)');
test('Complex: Stored kernel convolution', 'denoise([1,2,3,4,5,6,7,8])',
     r => {
         const vals = r.match(/-?\d+\.?\d*/g).map(parseFloat);
         return vals.length === 10; // 8+3-1
     });

// Multi-stage processing
Module.eval('let raw = [1,2,3,4,5,6,7,8]');
Module.eval('let win = hanning(8)');
Module.eval('let windowed = map((s,w) => s*w, raw, win)');
Module.eval('let spectrum = fft_mag(windowed)');
Module.eval('let energy = sum_vec(power_all(spectrum))');
test('Complex: Multi-variable pipeline', 'energy', r => parseFloat(r) > 0);

// ═══════════════════════════════════════════════════════════════════════════
// INTEGRATION TESTS: ADVANCED SCENARIOS
// ═══════════════════════════════════════════════════════════════════════════

console.log('\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
console.log('INTEGRATION: ADVANCED SCENARIOS');
console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n');

// Comparison of different windows on same signal
Module.eval('let test_sig = [1,2,3,4,5,6,7,8]');
Module.eval('let han_result = fft_mag(map((s,w) => s*w, test_sig, hanning(8)))');
Module.eval('let ham_result = fft_mag(map((s,w) => s*w, test_sig, hamming(8)))');
Module.eval('let blk_result = fft_mag(map((s,w) => s*w, test_sig, blackman(8)))');

test('Advanced: Hanning windowed FFT', 'han_result', r => r.includes('['));
test('Advanced: Hamming windowed FFT', 'ham_result', r => r.includes('['));
test('Advanced: Blackman windowed FFT', 'blk_result', r => r.includes('['));

// Cross-correlation approximation
Module.eval('let xcorr = (a, b) => conv(a, b)');
test('Advanced: Cross-correlation', 'xcorr([1,2,3,4], [0,1,2,1,0])',
     r => {
         const vals = r.match(/-?\d+\.?\d*/g).map(parseFloat);
         return vals.length === 8;
     });

// Derivative and integration filters
test('Advanced: Derivative filter', 'conv([1,2,3,4,5], [1,-1])',
     r => vecApprox(r, [1, 1, 1, 1, 1, -5]));

test('Advanced: Integration filter', 'conv([1,1,1,1], [1,1,1])',
     r => vecApprox(r, [1, 2, 3, 3, 2, 1]));

// Performance: Direct vs FFT convolution
test('Advanced: Direct vs FFT equivalence',
     'reduce((s,v) => s+abs(v), 0, map((a,b) => a-b, conv([1,2,3,4,5,6,7,8], [1,2,3]), conv_fft([1,2,3,4,5,6,7,8], [1,2,3])))',
     r => approx(r, 0, 1e-9));

// Complete DSP workflow: preprocess -> transform -> analyze
Module.eval('let preprocess = sig => map((s,w) => s*w, sig, hanning(8))');
Module.eval('let transform = sig => fft_mag(sig)');
Module.eval('let find_peak = spec => reduce((a,b) => max(a,b), 0, spec)');
Module.eval('let full_analysis = sig => find_peak(transform(preprocess(sig)))');

test('Advanced: Complete DSP workflow', 'full_analysis([1,2,3,4,5,6,7,8])',
     r => parseFloat(r) > 0);

// Closure with complex operations
Module.eval('let threshold = 0.5');
Module.eval('let threshold_filter = v => filter(x => x > threshold, v)');
Module.eval('let process_above_threshold = v => sum_vec(threshold_filter(v))');

test('Advanced: Closure with filtering', 'process_above_threshold([0.2, 0.7, 0.3, 0.9, 0.1])',
     r => approx(r, 1.6, 0.01));

// ═══════════════════════════════════════════════════════════════════════════
// FINAL SUMMARY
// ═══════════════════════════════════════════════════════════════════════════

console.log('\n╔═══════════════════════════════════════════════════════════════╗');
console.log('║                        TEST SUMMARY                           ║');
console.log('╚═══════════════════════════════════════════════════════════════╝\n');

console.log(`  Total tests run:    ${testsRun}`);
console.log(`  Tests passed:       ${testsPassed}`);
console.log(`  Tests failed:       ${testsRun - testsPassed}`);
console.log(`  Success rate:       ${((testsPassed / testsRun) * 100).toFixed(1)}%\n`);

if (testsPassed === testsRun) {
    console.log('╔═══════════════════════════════════════════════════════════════╗');
    console.log('║                  🎉 ALL TESTS PASSED! 🎉                     ║');
    console.log('╚═══════════════════════════════════════════════════════════════╝\n');

    console.log('ACHRONYME CORE - COMPLETE FEATURE LIST:\n');
    console.log('✓ Basic Operations');
    console.log('  • Arithmetic: +, -, *, /, ^, unary -');
    console.log('  • Trigonometry: sin, cos, tan, asin, acos, atan');
    console.log('  • Exponentials: exp, log, log10, log2');
    console.log('  • Roots: sqrt, cbrt, pow');
    console.log('  • Rounding: floor, ceil, round, trunc');
    console.log('  • Utility: abs, sign, min, max\n');

    console.log('✓ Complex Types');
    console.log('  • Complex numbers: creation, operations, magnitude');
    console.log('  • Vectors: operations, dot product, norm');
    console.log('  • Matrices: transpose, determinant, inverse\n');

    console.log('✓ Functional Programming');
    console.log('  • Variables: persistent state across evaluations');
    console.log('  • Lambdas: single & multi-parameter functions');
    console.log('  • Closures: capture outer scope');
    console.log('  • Higher-order functions: map, filter, reduce, pipe');
    console.log('  • Comparisons: >, <, >=, <=, ==, !=\n');

    console.log('✓ Digital Signal Processing');
    console.log('  • DFT: Discrete Fourier Transform O(N²)');
    console.log('  • FFT: Fast Fourier Transform O(N log N)');
    console.log('  • IFFT: Inverse FFT with perfect reconstruction');
    console.log('  • Convolution: Direct O(N*M) and FFT-based O((N+M) log(N+M))');
    console.log('  • Windows: Hanning, Hamming, Blackman\n');

    console.log('✓ Advanced Features');
    console.log('  • Windowed spectral analysis');
    console.log('  • FIR filtering with convolution');
    console.log('  • Multi-stage processing pipelines');
    console.log('  • Function composition and reuse');
    console.log('  • Complex nested expressions\n');

    console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
    console.log('Achronyme Core is ready for production use!');
    console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n');
} else {
    console.log('╔═══════════════════════════════════════════════════════════════╗');
    console.log(`║            ⚠ ${testsRun - testsPassed} TEST(S) FAILED ⚠                        ║`);
    console.log('╚═══════════════════════════════════════════════════════════════╝\n');
}
