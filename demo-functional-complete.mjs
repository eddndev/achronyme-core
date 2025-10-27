import createAchronymeModule from './dist/achronyme-core.mjs';

console.log('Loading WASM module...');
const Module = await createAchronymeModule();

console.log('\n=== Achronyme Core - Complete Functional Programming Demo ===\n');

// Reset environment
Module.reset();

console.log('--- 1. Lambda Functions in Variables ---\n');
Module.eval('let square = x => x ^ 2');
console.log('square = x => x ^ 2');
console.log('square(5) =', Module.eval('square(5)'), '✓\n');

Module.eval('let cube = x => x ^ 3');
console.log('cube = x => x ^ 3');
console.log('cube(3) =', Module.eval('cube(3)'), '✓\n');

console.log('--- 2. Higher-Order Functions with Stored Lambdas ---\n');
Module.eval('let double = x => x * 2');
console.log('double = x => x * 2');
console.log('map(double, [1, 2, 3]) =', Module.eval('map(double, [1, 2, 3])'), '✓\n');

Module.eval('let positive = x => x > 0');
console.log('positive = x => x > 0');
console.log('filter(positive, [-1, 0, 1, 2]) =', Module.eval('filter(positive, [-1, 0, 1, 2])'), '✓\n');

console.log('--- 3. Closures ---\n');
Module.eval('let x = 10');
console.log('x = 10');
Module.eval('let addX = y => x + y');
console.log('addX = y => x + y');
console.log('addX(5) =', Module.eval('addX(5)'), '✓');
console.log('addX(20) =', Module.eval('addX(20)'), '✓\n');

console.log('--- 4. DSP Function Library ---\n');
console.log('Building DSP library...');
Module.eval('let scale = (v, factor) => map(val => val * factor, v)');
console.log('scale = (v, factor) => map(val => val * factor, v)');

Module.eval('let signal = [1, 2, 3, 4]');
console.log('\nsignal = [1, 2, 3, 4]');
console.log('scale(signal, 0.5) =', Module.eval('scale(signal, 0.5)'), '✓\n');

console.log('--- 5. Function Composition with Pipe ---\n');
Module.eval('let abs_all = v => map(n => abs(n), v)');
Module.eval('let sum_all = v => reduce((a,b) => a+b, 0, v)');
console.log('abs_all = v => map(n => abs(n), v)');
console.log('sum_all = v => reduce((a,b) => a+b, 0, v)');

console.log('\ndata = [-1, -2, -3]');
Module.eval('let data = [-1, -2, -3]');
console.log('pipe(data, abs_all, sum_all) =', Module.eval('pipe(data, abs_all, sum_all)'), '✓\n');

console.log('--- 6. Multi-Parameter Lambdas ---\n');
Module.eval('let add = (a, b) => a + b');
console.log('add = (a, b) => a + b');
console.log('map(add, [1,2,3], [10,20,30]) =', Module.eval('map(add, [1,2,3], [10,20,30])'), '✓\n');

console.log('--- 7. Complex DSP Pipeline ---\n');
console.log('Creating FFT analysis pipeline...');
Module.eval('let analyze = s => fft_mag(s)');
Module.eval('let sum = v => reduce((a,b) => a+b, 0, v)');
console.log('analyze = s => fft_mag(s)');
console.log('sum = v => reduce((a,b) => a+b, 0, v)');

Module.eval('let waveform = [1, 1, 1, 1, 1, 1, 1, 1]');
console.log('\nwaveform = [1, 1, 1, 1, 1, 1, 1, 1]');
console.log('spectrum = analyze(waveform):', Module.eval('analyze(waveform)'));
console.log('sum(spectrum) =', Module.eval('sum(analyze(waveform))'), '✓\n');

console.log('--- 8. Variable Persistence Across Calls ---\n');
console.log('All variables are still available:');
console.log('square(10) =', Module.eval('square(10)'));
console.log('cube(2) =', Module.eval('cube(2)'));
console.log('addX(100) =', Module.eval('addX(100)'));
console.log('✓ All functions persist across calls!\n');

console.log('--- 9. Reset Environment ---\n');
Module.reset();
console.log('Module.reset() called');
try {
    Module.eval('square(5)');
} catch (e) {
    console.log('square(5) → Error (as expected)');
    console.log('✓ Reset works correctly!\n');
}

console.log('=== Functional Programming Complete! ===\n');
console.log('Summary:');
console.log('✓ Lambda functions in variables');
console.log('✓ Stored lambdas with map/filter/reduce');
console.log('✓ Closures over outer variables');
console.log('✓ DSP function libraries');
console.log('✓ Function composition with pipe');
console.log('✓ Multi-parameter lambdas');
console.log('✓ Complex DSP pipelines');
console.log('✓ Variable persistence');
console.log('✓ Environment reset\n');
