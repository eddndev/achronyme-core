/**
 * Test Higher-Order Functions (HOF) in Achronyme WASM
 *
 * Tests: map(), filter(), reduce(), pipe()
 */

import createAchronymeModule from '../dist-rust/achronyme-core.mjs';

console.log('Loading WASM module...');
const Module = await createAchronymeModule();
console.log('✓ Module loaded\n');

console.log('🧪 Testing Higher-Order Functions (HOF)\n');

let passed = 0;
let failed = 0;

function test(name, expression, expected) {
    try {
        const result = Module.eval(expression);
        const numResult = typeof result === 'string' ? parseFloat(result) : result;
        const match = Math.abs(numResult - expected) < 1e-10;

        if (match) {
            console.log(`✅ ${name}`);
            console.log(`   Expression: ${expression}`);
            console.log(`   Result: ${result}`);
            passed++;
        } else {
            console.log(`❌ ${name}`);
            console.log(`   Expression: ${expression}`);
            console.log(`   Expected: ${expected}, Got: ${result}`);
            failed++;
        }
    } catch (error) {
        console.log(`❌ ${name} - ERROR`);
        console.log(`   Expression: ${expression}`);
        console.log(`   Error: ${error.message}`);
        failed++;
    }
    console.log();
}

function testVector(name, expression, expectedLength) {
    try {
        const result = Module.eval(expression);
        const vectorMatch = result.match(/\[([^\]]+)\]/);

        if (vectorMatch) {
            const values = vectorMatch[1].split(',').map(v => parseFloat(v.trim()));
            if (values.length === expectedLength) {
                console.log(`✅ ${name}`);
                console.log(`   Expression: ${expression}`);
                console.log(`   Result: [${values.join(', ')}]`);
                passed++;
            } else {
                console.log(`❌ ${name}`);
                console.log(`   Expression: ${expression}`);
                console.log(`   Expected length: ${expectedLength}, Got: ${values.length}`);
                failed++;
            }
        } else {
            console.log(`❌ ${name} - Not a vector`);
            console.log(`   Expression: ${expression}`);
            console.log(`   Got: ${result}`);
            failed++;
        }
    } catch (error) {
        console.log(`❌ ${name} - ERROR`);
        console.log(`   Expression: ${expression}`);
        console.log(`   Error: ${error.message}`);
        failed++;
    }
    console.log();
}

// ============================================================================
// MAP TESTS
// ============================================================================

console.log('📦 Testing map()...\n');

// Test 1: map with lambda inline
testVector(
    'map() - square each element',
    'map(x => x ^ 2,[2,3,4])',
    3  // [4,9,16]
);

// Test 2: map with multiple collections
testVector(
    'map() - add pairs',
    'map((x,y) => x + y,[1,2,3],[4,5,6])',
    3  // [5,7,9]
);

// Test 3: map truncates to shortest
testVector(
    'map() - truncates to shortest collection',
    'map((x,y) => x * y,[1,2],[10,20,30])',
    2  // [10,40]
);

// ============================================================================
// FILTER TESTS
// ============================================================================

console.log('📦 Testing filter()...\n');

// Test 5: filter even numbers
testVector(
    'filter() - even numbers',
    'filter(x => (x % 2) == 0,[1,2,3,4,5,6])',
    3  // [2,4,6]
);

// Test 6: filter with complex predicate
testVector(
    'filter() - values >= 3',
    'filter(x => x >= 3,[1,2,3,4])',
    2  // [3,4]
);

// Test 7: filter values > 2
testVector(
    'filter() - values > 2',
    'filter(x => x > 2,[1,2,3,4,5])',
    3  // [3,4,5]
);

// ============================================================================
// REDUCE TESTS
// ============================================================================

console.log('📦 Testing reduce()...\n');

// Test 8: reduce - sum
test(
    'reduce() - sum all elements',
    'reduce((acc,x) => acc + x,0,[1,2,3,4])',
    10
);

// Test 9: reduce - product
test(
    'reduce() - product of elements',
    'reduce((acc,x) => acc * x,1,[2,3,4])',
    24
);

// Test 10: reduce - max
test(
    'reduce() - find maximum',
    'reduce((acc,x) => max(acc,x),0,[3,1,4,1,5,9])',
    9
);

// Test 11: reduce - count
test(
    'reduce() - count elements',
    'reduce((acc,x) => acc + 1,0,[10,20,30,40,50])',
    5
);

// ============================================================================
// PIPE TESTS
// ============================================================================

console.log('📦 Testing pipe()...\n');

// Test 12: pipe with 2 functions
test(
    'pipe() - simple pipeline',
    'pipe(5,x => x * 2,x => x + 1)',
    11  // 5 * 2 = 10, 10 + 1 = 11
);

// Test 13: pipe with 3 functions
test(
    'pipe() - multi-step pipeline',
    'pipe(2,x => x + 1,x => x * 2,x => x ^ 2)',
    36  // 2 + 1 = 3, 3 * 2 = 6, 6^2 = 36
);

// Test 14: pipe with lambdas wrapping math functions
test(
    'pipe() - with sqrt wrapper',
    'pipe(16,x => sqrt(x),x => x / 2)',
    2  // sqrt(16) = 4, 4 / 2 = 2
);

// ============================================================================
// SUMMARY
// ============================================================================

console.log('═'.repeat(60));
console.log(`\n📊 Test Summary:`);
console.log(`   ✅ Passed: ${passed}`);
console.log(`   ❌ Failed: ${failed}`);
console.log(`   📈 Total:  ${passed + failed}`);
console.log(`   🎯 Success Rate: ${((passed / (passed + failed)) * 100).toFixed(1)}%\n`);

if (failed === 0) {
    console.log('🎉 All HOF tests passed! C++ WASM has full HOF support.\n');
    process.exit(0);
} else {
    console.log('⚠️  Some tests failed. Review implementation.\n');
    process.exit(1);
}
