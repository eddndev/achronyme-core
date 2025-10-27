import createAchronymeModule from './dist/achronyme-core.mjs';

console.log('Loading WASM module...');
const Module = await createAchronymeModule();

console.log('\n=== Lambda Persistence Test ===\n');

// Test 1: Define lambda
console.log('Test 1: let square = x => x ^ 2');
try {
    const r1 = Module.eval('let square = x => x ^ 2');
    console.log('Result:', r1);
    console.log('Type:', typeof r1);
} catch (e) {
    console.log('Error:', e);
}

// Test 2: Check if variable exists
console.log('\nTest 2: square (check variable)');
try {
    const r2 = Module.eval('square');
    console.log('Result:', r2);
} catch (e) {
    console.log('Error:', e);
}

// Test 3: Call lambda
console.log('\nTest 3: square(5)');
try {
    const r3 = Module.eval('square(5)');
    console.log('Result:', r3);
    console.log('✓ SUCCESS - Lambda stored in variable works!');
} catch (e) {
    console.log('❌ ERROR:', e);
}

// Test 4: Inline lambda (for comparison)
console.log('\nTest 4: (x => x ^ 2)(5) - inline');
try {
    const r4 = Module.eval('(x => x ^ 2)(5)');
    console.log('Result:', r4);
} catch (e) {
    console.log('Error:', e);
}

console.log('\n=== End ===\n');
