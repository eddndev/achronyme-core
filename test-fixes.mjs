/**
 * Test script for verifying modulo operator and vector-vector multiplication fixes
 */

import Module from './dist/achronyme-core.mjs';

const achronyme = await Module();

console.log('=== Testing Modulo Operator (%) ===\n');

// Test 1: Basic modulo operations
console.log('Test 1: Basic modulo operations');
try {
    const result1 = achronyme.eval('5 % 2');
    console.log('  5 % 2 =', result1, result1 === '1' ? '✓' : '✗');

    const result2 = achronyme.eval('10 % 3');
    console.log('  10 % 3 =', result2, result2 === '1' ? '✓' : '✗');

    const result3 = achronyme.eval('7.5 % 2.5');
    console.log('  7.5 % 2.5 =', result3, result3 === '0' ? '✓' : '✗');

    const result4 = achronyme.eval('15 % 4');
    console.log('  15 % 4 =', result4, result4 === '3' ? '✓' : '✗');
} catch (error) {
    console.log('  ✗ Error:', error.message);
}

// Test 2: Modulo in expressions
console.log('\nTest 2: Modulo in expressions');
try {
    const result1 = achronyme.eval('(10 + 5) % 4');
    console.log('  (10 + 5) % 4 =', result1, result1 === '3' ? '✓' : '✗');

    const result2 = achronyme.eval('20 % (3 + 2)');
    console.log('  20 % (3 + 2) =', result2, result2 === '0' ? '✓' : '✗');
} catch (error) {
    console.log('  ✗ Error:', error.message);
}

// Test 3: Modulo with negative numbers
console.log('\nTest 3: Modulo with negative numbers');
try {
    const result1 = achronyme.eval('-5 % 2');
    console.log('  -5 % 2 =', result1);

    const result2 = achronyme.eval('5 % -2');
    console.log('  5 % -2 =', result2);
} catch (error) {
    console.log('  ✗ Error:', error.message);
}

// Test 4: Modulo in lambdas and HOF functions (from known issue)
console.log('\nTest 4: Modulo in lambdas and HOF functions');
try {
    const result1 = achronyme.eval('filter(x => x % 2 == 0, [1, 2, 3, 4, 5, 6])');
    console.log('  filter(x => x % 2 == 0, [1,2,3,4,5,6]) =', result1);
    console.log('  Expected: [2, 4, 6]', result1 === '[2, 4, 6]' ? '✓' : '✗');

    const result2 = achronyme.eval('map(x => x % 3, [10, 11, 12, 13, 14, 15])');
    console.log('  map(x => x % 3, [10,11,12,13,14,15]) =', result2);
    console.log('  Expected: [1, 2, 0, 1, 2, 0]', result2 === '[1, 2, 0, 1, 2, 0]' ? '✓' : '✗');
} catch (error) {
    console.log('  ✗ Error:', error.message);
}

// Test 5: Modulo by zero (should error)
console.log('\nTest 5: Modulo by zero (should error)');
try {
    const result = achronyme.eval('5 % 0');
    console.log('  ✗ Should have thrown error, got:', result);
} catch (error) {
    console.log('  ✓ Correctly threw error:', error.message);
}

console.log('\n=== Testing Vector-Vector Multiplication ===\n');

// Test 6: Basic vector-vector multiplication
console.log('Test 6: Basic vector-vector element-wise multiplication');
try {
    const result1 = achronyme.eval('[1, 2, 3] * [4, 5, 6]');
    console.log('  [1,2,3] * [4,5,6] =', result1);
    console.log('  Expected: [4, 10, 18]', result1 === '[4, 10, 18]' ? '✓' : '✗');

    const result2 = achronyme.eval('[2, 3, 4] * [5, 6, 7]');
    console.log('  [2,3,4] * [5,6,7] =', result2);
    console.log('  Expected: [10, 18, 28]', result2 === '[10, 18, 28]' ? '✓' : '✗');
} catch (error) {
    console.log('  ✗ Error:', error.message);
}

// Test 7: Vector-vector with different sizes (should error)
console.log('\nTest 7: Vector dimension mismatch (should error)');
try {
    const result = achronyme.eval('[1, 2, 3] * [4, 5]');
    console.log('  ✗ Should have thrown error, got:', result);
} catch (error) {
    console.log('  ✓ Correctly threw error:', error.message);
}

// Test 8: Vector-vector in expressions
console.log('\nTest 8: Vector-vector multiplication in expressions');
try {
    const result1 = achronyme.eval('[1, 2, 3] * [2, 2, 2] + [1, 1, 1]');
    console.log('  [1,2,3] * [2,2,2] + [1,1,1] =', result1);
    console.log('  Expected: [3, 5, 7]', result1 === '[3, 5, 7]' ? '✓' : '✗');
} catch (error) {
    console.log('  ✗ Error:', error.message);
}

// Test 9: Vector-vector with scalar still works
console.log('\nTest 9: Vector-scalar multiplication (should still work)');
try {
    const result1 = achronyme.eval('[1, 2, 3] * 2');
    console.log('  [1,2,3] * 2 =', result1, result1 === '[2, 4, 6]' ? '✓' : '✗');

    const result2 = achronyme.eval('3 * [2, 3, 4]');
    console.log('  3 * [2,3,4] =', result2, result2 === '[6, 9, 12]' ? '✓' : '✗');
} catch (error) {
    console.log('  ✗ Error:', error.message);
}

console.log('\n=== All Tests Complete ===');
