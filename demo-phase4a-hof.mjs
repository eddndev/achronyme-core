import createAchronymeModule from './dist/achronyme-core.mjs';

console.log('Loading WASM module...');
const Module = await createAchronymeModule();

console.log('\n=== Achronyme Core - Phase 4A: Higher-Order Functions ===\n');

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

console.log('--- map() with Single Collection ---\n');

test('[map-1] Double all elements',
    'map(x => x * 2, [1, 2, 3])',
    /\[2\.0.*4\.0.*6\.0/);

test('[map-1] Square all elements',
    'map(n => n ^ 2, [1, 2, 3, 4])',
    /\[1\.0.*4\.0.*9\.0.*16\.0/);

test('[map-1] Add 10 to each',
    'map(x => x + 10, [5, 10, 15])',
    /\[15\.0.*20\.0.*25\.0/);

console.log('--- map() with Multiple Collections ---\n');

test('[map-2] Add two vectors',
    'map((a, b) => a + b, [1, 2, 3], [10, 20, 30])',
    /\[11\.0.*22\.0.*33\.0/);

test('[map-2] Multiply two vectors',
    'map((x, y) => x * y, [2, 3, 4], [5, 6, 7])',
    /\[10\.0.*18\.0.*28\.0/);

test('[map-2] Truncate to shortest',
    'map((a, b) => a + b, [1, 2, 3, 4], [10, 20])',
    /\[11\.0.*22\.0/);

console.log('--- map() with Three Collections ---\n');

test('[map-3] Sum three vectors',
    'map((a, b, c) => a + b + c, [1, 2], [10, 20], [100, 200])',
    /\[111\.0.*222\.0/);

console.log('--- filter() ---\n');

test('[filter] Positive numbers',
    'filter(x => x > 0, [-2, -1, 0, 1, 2])',
    /\[1\.0.*2\.0/);

test('[filter] Even numbers',
    'filter(n => n - 2 * floor(n / 2) == 0, [1, 2, 3, 4, 5, 6])',
    /\[2\.0.*4\.0.*6\.0/);

test('[filter] Greater than 5',
    'filter(x => x > 5, [1, 3, 5, 7, 9])',
    /\[7\.0.*9\.0/);

test('[filter] All pass',
    'filter(x => 1, [1, 2, 3])',
    /\[1\.0.*2\.0.*3\.0/);

test('[filter] None pass',
    'filter(x => 0, [1, 2, 3])',
    /\[\]/);

console.log('--- reduce() ---\n');

test('[reduce] Sum',
    'reduce((acc, x) => acc + x, 0, [1, 2, 3, 4])',
    /10/);

test('[reduce] Product',
    'reduce((acc, x) => acc * x, 1, [2, 3, 4])',
    /24/);

test('[reduce] Max',
    'reduce((a, b) => max(a, b), 0, [3, 7, 2, 9, 1])',
    /9/);

test('[reduce] Count (constant 1)',
    'reduce((acc, x) => acc + 1, 0, [10, 20, 30])',
    /3/);

console.log('--- Composition of HOFs ---\n');

test('[compose] Map then reduce',
    'reduce((acc, x) => acc + x, 0, map(n => n ^ 2, [1, 2, 3]))',
    /14/);  // 1 + 4 + 9 = 14

console.log('--- DSP Example: Interference Pattern ---\n');

test('[DSP] Wave interference',
    'map((w1, w2) => w1 + w2, [1, 0, -1], [0.5, 0, -0.5])',
    /\[1\.5.*0\.0.*-1\.5/);

console.log('\n=== Higher-Order Functions Tests Complete ===\n');
