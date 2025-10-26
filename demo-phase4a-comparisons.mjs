import createAchronymeModule from './dist/achronyme-core.mjs';

console.log('Loading WASM module...');
const Module = await createAchronymeModule();

console.log('\n=== Achronyme Core - Phase 4A: Comparison Operators ===\n');

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

console.log('--- Basic Comparison Operators ---\n');

// Greater than (>)
test('[GT] 5 > 3 should return 1.0 (true)', '5 > 3', (r) => parseFloat(r) === 1.0);
test('[GT] 3 > 5 should return 0.0 (false)', '3 > 5', (r) => parseFloat(r) === 0.0);
test('[GT] 5 > 5 should return 0.0 (false)', '5 > 5', (r) => parseFloat(r) === 0.0);

// Less than (<)
test('[LT] 3 < 5 should return 1.0 (true)', '3 < 5', (r) => parseFloat(r) === 1.0);
test('[LT] 5 < 3 should return 0.0 (false)', '5 < 3', (r) => parseFloat(r) === 0.0);
test('[LT] 5 < 5 should return 0.0 (false)', '5 < 5', (r) => parseFloat(r) === 0.0);

// Greater than or equal (>=)
test('[GTE] 5 >= 3 should return 1.0 (true)', '5 >= 3', (r) => parseFloat(r) === 1.0);
test('[GTE] 5 >= 5 should return 1.0 (true)', '5 >= 5', (r) => parseFloat(r) === 1.0);
test('[GTE] 3 >= 5 should return 0.0 (false)', '3 >= 5', (r) => parseFloat(r) === 0.0);

// Less than or equal (<=)
test('[LTE] 3 <= 5 should return 1.0 (true)', '3 <= 5', (r) => parseFloat(r) === 1.0);
test('[LTE] 5 <= 5 should return 1.0 (true)', '5 <= 5', (r) => parseFloat(r) === 1.0);
test('[LTE] 5 <= 3 should return 0.0 (false)', '5 <= 3', (r) => parseFloat(r) === 0.0);

// Equal (==)
test('[EQ] 5 == 5 should return 1.0 (true)', '5 == 5', (r) => parseFloat(r) === 1.0);
test('[EQ] 5 == 3 should return 0.0 (false)', '5 == 3', (r) => parseFloat(r) === 0.0);
test('[EQ] 3.14 == 3.14 should return 1.0 (true)', '3.14 == 3.14', (r) => parseFloat(r) === 1.0);

// Not equal (!=)
test('[NEQ] 5 != 3 should return 1.0 (true)', '5 != 3', (r) => parseFloat(r) === 1.0);
test('[NEQ] 5 != 5 should return 0.0 (false)', '5 != 5', (r) => parseFloat(r) === 0.0);

console.log('--- Comparison with Expressions ---\n');

// Comparisons with arithmetic expressions
test('[Expr] (2 + 3) > 4 should return 1.0 (true)', '(2 + 3) > 4', (r) => parseFloat(r) === 1.0);
test('[Expr] (2 * 3) < 10 should return 1.0 (true)', '(2 * 3) < 10', (r) => parseFloat(r) === 1.0);
test('[Expr] (10 / 2) == 5 should return 1.0 (true)', '(10 / 2) == 5', (r) => parseFloat(r) === 1.0);
test('[Expr] (2 ^ 3) >= 8 should return 1.0 (true)', '(2 ^ 3) >= 8', (r) => parseFloat(r) === 1.0);

// Comparisons with negative numbers
test('[Neg] -5 < 0 should return 1.0 (true)', '-5 < 0', (r) => parseFloat(r) === 1.0);
test('[Neg] -3 > -5 should return 1.0 (true)', '-3 > -5', (r) => parseFloat(r) === 1.0);

console.log('--- Precedence Tests ---\n');

// Comparison has lower precedence than arithmetic
test('[Precedence] 2 + 3 > 4 should parse as (2 + 3) > 4', '2 + 3 > 4', (r) => parseFloat(r) === 1.0);
test('[Precedence] 2 * 3 < 10 should parse as (2 * 3) < 10', '2 * 3 < 10', (r) => parseFloat(r) === 1.0);
test('[Precedence] 10 - 5 == 5 should parse as (10 - 5) == 5', '10 - 5 == 5', (r) => parseFloat(r) === 1.0);

console.log('--- Comparison with Functions ---\n');

// Comparisons with function calls
test('[Func] sqrt(16) == 4 should return 1.0 (true)', 'sqrt(16) == 4', (r) => parseFloat(r) === 1.0);
test('[Func] abs(-5) > 0 should return 1.0 (true)', 'abs(-5) > 0', (r) => parseFloat(r) === 1.0);
test('[Func] max(3, 7) >= 7 should return 1.0 (true)', 'max(3, 7) >= 7', (r) => parseFloat(r) === 1.0);

console.log('--- Comparison with Constants ---\n');

// Comparisons with constants
test('[Const] PI > 3 should return 1.0 (true)', 'PI > 3', (r) => parseFloat(r) === 1.0);
test('[Const] E < 3 should return 1.0 (true)', 'E < 3', (r) => parseFloat(r) === 1.0);

console.log('\n=== All Comparison Tests Complete ===\n');
