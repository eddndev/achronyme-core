import createAchronymeModule from './dist/achronyme-core.mjs';

console.log('Loading WASM module...');
const Module = await createAchronymeModule();

console.log('\n=== Simple Variable Persistence Test ===\n');

// Test 1: Simple number variable
console.log('Test 1: let x = 5');
try {
    const r1 = Module.eval('let x = 5');
    console.log('Result:', r1);
} catch (e) {
    console.log('Error:', e);
}

// Test 2: Use variable
console.log('\nTest 2: x + 10');
try {
    const r2 = Module.eval('x + 10');
    console.log('Result:', r2);
} catch (e) {
    console.log('Error:', e);
}

// Test 3: Vector variable
console.log('\nTest 3: let v = [1, 2, 3]');
try {
    const r3 = Module.eval('let v = [1, 2, 3]');
    console.log('Result:', r3);
} catch (e) {
    console.log('Error:', e);
}

// Test 4: Use vector
console.log('\nTest 4: v');
try {
    const r4 = Module.eval('v');
    console.log('Result:', r4);
} catch (e) {
    console.log('Error:', e);
}

// Test 5: Reset
console.log('\nTest 5: Module.reset()');
try {
    const r5 = Module.reset();
    console.log('Result:', r5);
} catch (e) {
    console.log('Error:', e);
}

// Test 6: x after reset
console.log('\nTest 6: x (after reset)');
try {
    const r6 = Module.eval('x');
    console.log('Result:', r6);
} catch (e) {
    console.log('Error:', e);
}

console.log('\n=== End ===\n');
