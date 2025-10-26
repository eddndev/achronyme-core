import createAchronymeModule from './dist/achronyme-core.mjs';

console.log('Loading WASM module...');
const Module = await createAchronymeModule();

console.log('\n=== Achronyme Core - Phase 4A: Lambda Functions ===\n');

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

console.log('--- Basic Lambda Creation ---\n');

// Simple lambda
test('[Lambda] x => x * 2', 'x => x * 2', /function/);

// Lambda with arithmetic
test('[Lambda] n => n ^ 2', 'n => n ^ 2', /function/);

// Lambda with comparison
test('[Lambda] x => x > 0', 'x => x > 0', /function/);

// Lambda in variable
test('[Lambda] let double = x => x * 2', 'let double = x => x * 2', /function/);

// Lambda with complex expression
test('[Lambda] x => 2 * x + 1', 'x => 2 * x + 1', /function/);

console.log('--- Lambdas with Constants ---\n');

// Lambda using PI
test('[Lambda] r => PI * r ^ 2', 'r => PI * r ^ 2', /function/);

// Lambda using functions
test('[Lambda] x => sqrt(x)', 'x => sqrt(x)', /function/);

console.log('\n=== Lambda Creation Tests Complete ===\n');

console.log('Note: Function application (calling lambdas) is not yet implemented.');
console.log('That will be implemented next along with map(), filter(), and reduce().\n');
