import createAchronymeModule from './dist/achronyme-core.mjs';

console.log('Loading WASM module...');
const Module = await createAchronymeModule();

console.log('\n=== Achronyme Core - Phase 4A: Variable Declarations ===\n');

// Helper function to run tests
function test(description, expression, validator) {
    const result = Module.eval(expression);
    const passed = validator(result);

    if (passed) {
        console.log(`✓ ${description}`);
        console.log(`  Expression: ${expression}`);
        console.log(`  Result: ${result}\n`);
    } else {
        console.log(`✗ ${description}`);
        console.log(`  Expression: ${expression}`);
        console.log(`  Result: ${result}`);
        console.log(`  ⚠️ Warning: Result doesn't match expected value\n`);
    }
}

console.log('--- Basic Variable Declarations ---\n');

// Simple number variable
test('[Var] let x = 5', 'let x = 5', (r) => parseFloat(r) === 5);

// Expression as initializer
test('[Var] let result = 2 + 3', 'let result = 2 + 3', (r) => parseFloat(r) === 5);

// Complex expression
test('[Var] let calc = 2 * 3 + 4', 'let calc = 2 * 3 + 4', (r) => parseFloat(r) === 10);

// Using constants
test('[Var] let tau = 2 * PI', 'let tau = 2 * PI', (r) => Math.abs(parseFloat(r) - (2 * Math.PI)) < 0.0001);

// Using functions
test('[Var] let root = sqrt(16)', 'let root = sqrt(16)', (r) => parseFloat(r) === 4);

console.log('--- Comparison in Initializers ---\n');

// Comparison as initializer
test('[Var] let isGreater = 5 > 3', 'let isGreater = 5 > 3', (r) => parseFloat(r) === 1);

// Complex comparison
test('[Var] let check = (2 + 3) == 5', 'let check = (2 + 3) == 5', (r) => parseFloat(r) === 1);

console.log('--- Complex Types in Variables ---\n');

// Complex number
test('[Var] let z = 3i', 'let z = 3i', (r) => r.includes('i'));

// Complex from function
test('[Var] let c = complex(3, 4)', 'let c = complex(3, 4)', (r) => r.includes('i'));

// Vector
test('[Var] let vec = [1, 2, 3]', 'let vec = [1, 2, 3]', (r) => r.includes('['));

// Matrix
test('[Var] let mat = [[1, 2], [3, 4]]', 'let mat = [[1, 2], [3, 4]]', (r) => r.includes('[['));

console.log('\n=== All Variable Declaration Tests Complete ===\n');

console.log('Note: Variable persistence across expressions is not yet implemented.');
console.log('Each eval() call creates a new evaluator instance with fresh environment.\n');
