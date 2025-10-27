import createAchronymeModule from './dist/achronyme-core.mjs';

console.log('Loading WASM module...');
const Module = await createAchronymeModule();

console.log('\n=== Achronyme Core - Phase 4A: Pipe Function ===\n');

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

console.log('--- pipe() with Simple Functions ---\n');

test('[pipe-1] Simple pipeline: 3 + 1 = 4',
    'pipe(3, x => x + 1)',
    /4/);

test('[pipe-2] Two functions: (3 + 1) * 2 = 8',
    'pipe(3, x => x + 1, x => x * 2)',
    /8/);

test('[pipe-3] Three functions: ((3 + 1) * 2) ^ 2 = 64',
    'pipe(3, x => x + 1, x => x * 2, x => x ^ 2)',
    /64/);

console.log('--- pipe() with Mathematical Functions ---\n');

test('[pipe-4] Pipeline with sqrt and abs',
    'pipe(-16, x => abs(x), x => sqrt(x))',
    /4/);

test('[pipe-5] Trigonometric pipeline',
    'pipe(PI / 4, x => sin(x), x => abs(x), x => sqrt(x))',
    /0\.840/);  // sqrt(abs(sin(π/4))) ≈ 0.840

test('[pipe-6] Exponential pipeline',
    'pipe(2, x => x ^ 2, x => log(x), x => exp(x))',
    /4/);  // exp(log(4)) = 4

console.log('--- pipe() with Complex Chains ---\n');

test('[pipe-7] Long pipeline',
    'pipe(1, x => x + 1, x => x * 2, x => x + 3, x => x * 4, x => x - 5)',
    /23/);  // ((((1+1)*2)+3)*4)-5 = 23

test('[pipe-8] Data transformation',
    'pipe(10, x => x * 0.5, x => floor(x), x => x ^ 2)',
    /25/);  // floor(10*0.5)^2 = 5^2 = 25

console.log('--- pipe() Equivalence to Nested Calls ---\n');

test('[pipe-eq-1] pipe(x, f, g) == g(f(x))',
    'pipe(5, y => y * 2, y => y + 1) == ((5 * 2) + 1)',
    /1/);  // Returns 1 (true) - both equal 11

test('[pipe-eq-2] Verify nested equivalence',
    'pipe(7, x => x ^ 2, x => x + 10) == ((7 ^ 2) + 10)',
    /1/);  // Returns 1 (true) - both equal 59

console.log('--- pipe() with HOF Combinations ---\n');

test('[pipe-hof-1] Pipe with map result',
    'pipe([1, 2, 3], v => map(x => x ^ 2, v), v => reduce((acc, x) => acc + x, 0, v))',
    /14/);  // map [1,2,3] to [1,4,9], then sum = 14

test('[pipe-hof-2] Pipe with filter result',
    'pipe([1, 2, 3, 4, 5], v => filter(x => x > 2, v), v => map(x => x * 2, v), v => reduce((acc, x) => acc + x, 0, v))',
    /24/);  // filter [3,4,5], map to [6,8,10], sum = 24

console.log('\n=== Pipe Function Tests Complete ===\n');
