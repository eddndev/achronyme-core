import createAchronymeModule from './dist/achronyme-core.mjs';

console.log('Loading WASM module...');
const Module = await createAchronymeModule();

console.log('\n=== Achronyme Core - Phase 4A: Multi-Parameter Lambdas ===\n');

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

console.log('--- Single Parameter Lambdas ---\n');

test('[1-param] x => x * 2', 'x => x * 2', /x => <function>/);
test('[1-param] n => n ^ 2', 'n => n ^ 2', /n => <function>/);

console.log('--- Multi-Parameter Lambdas ---\n');

test('[2-param] (x, y) => x + y', '(x, y) => x + y', /\(x, y\) => <function>/);
test('[2-param] (a, b) => a * b', '(a, b) => a * b', /\(a, b\) => <function>/);
test('[2-param] (w1, w2) => w1 + w2', '(w1, w2) => w1 + w2', /\(w1, w2\) => <function>/);

console.log('--- Three Parameter Lambdas ---\n');

test('[3-param] (x, y, z) => x + y + z', '(x, y, z) => x + y + z', /\(x, y, z\) => <function>/);
test('[3-param] (a, b, c) => a * b * c', '(a, b, c) => a * b * c', /\(a, b, c\) => <function>/);

console.log('--- Lambdas in Variables ---\n');

test('[var] let add = (x, y) => x + y', 'let add = (x, y) => x + y', /\(x, y\) => <function>/);
test('[var] let mul3 = (a, b, c) => a * b * c', 'let mul3 = (a, b, c) => a * b * c', /\(a, b, c\) => <function>/);

console.log('--- Parenthesized Expressions (not lambdas) ---\n');

test('[expr] (2 + 3)', '(2 + 3)', /5/);
test('[expr] (2 * 3 + 4)', '(2 * 3 + 4)', /10/);

console.log('\n=== Multi-Parameter Lambda Tests Complete ===\n');
