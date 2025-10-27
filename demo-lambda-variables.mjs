import createAchronymeModule from './dist/achronyme-core.mjs';

console.log('Loading WASM module...');
const Module = await createAchronymeModule();

console.log('\n=== Test: Lambda Variables (Current Behavior) ===\n');

// Test 1: Define lambda in variable
console.log('Test 1: Define lambda function');
console.log('Expression: let square = x => x ^ 2');
try {
    const result1 = Module.eval('let square = x => x ^ 2');
    console.log('Result:', result1, '✓\n');
} catch (e) {
    console.log('Error:', e, '\n');
}

// Test 2: Use lambda (SHOULD work but FAILS)
console.log('Test 2: Use lambda function');
console.log('Expression: square(5)');
try {
    const result2 = Module.eval('square(5)');
    console.log('Result:', result2, '✓\n');
} catch (e) {
    console.log('❌ FAILS - Variable "square" not found\n');
    console.log('Error:', e, '\n');
}

// Test 3: Higher-order function usage
console.log('Test 3: Lambda with map (inline works)');
console.log('Expression: map(x => x ^ 2, [1, 2, 3])');
try {
    const result3 = Module.eval('map(x => x ^ 2, [1, 2, 3])');
    console.log('Result:', result3, '✓ Works (inline)\n');
} catch (e) {
    console.log('Error:', e, '\n');
}

// Test 4: Lambda stored then used with map (SHOULD work but FAILS)
console.log('Test 4: Stored lambda with map');
console.log('Step 1: let double = x => x * 2');
try {
    Module.eval('let double = x => x * 2');
    console.log('Defined ✓');
} catch (e) {
    console.log('Error:', e);
}

console.log('Step 2: map(double, [1, 2, 3])');
try {
    const result4 = Module.eval('map(double, [1, 2, 3])');
    console.log('Result:', result4, '✓\n');
} catch (e) {
    console.log('❌ FAILS - Variable "double" not found\n');
    console.log('Error:', e, '\n');
}

// Test 5: Closures (SHOULD work but FAILS)
console.log('Test 5: Closures over variables');
console.log('Step 1: let x = 10');
try {
    Module.eval('let x = 10');
    console.log('Defined x ✓');
} catch (e) {
    console.log('Error:', e);
}

console.log('Step 2: let adder = y => x + y');
try {
    Module.eval('let adder = y => x + y');
    console.log('Defined adder ✓');
} catch (e) {
    console.log('❌ FAILS - Variable "x" not accessible\n');
    console.log('Error:', e);
}

console.log('Step 3: adder(5)');
try {
    const result5 = Module.eval('adder(5)');
    console.log('Result:', result5, '✓\n');
} catch (e) {
    console.log('❌ FAILS - Variable "adder" not found\n');
    console.log('Error:', e, '\n');
}

console.log('=== Summary ===');
console.log('✓ Inline lambdas work: map(x => x^2, [1,2,3])');
console.log('❌ Stored lambdas fail: let f = x => x^2; f(5)');
console.log('❌ Closures fail: let x=5; let f = y => x+y; f(3)');
console.log('\nConclusion: Need persistent evaluator for functional programming!\n');
