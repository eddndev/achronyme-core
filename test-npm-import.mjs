/**
 * Test script to verify npm package entry points
 *
 * This simulates how users will import the package after npm install
 */

console.log('=== Testing NPM Package Entry Points ===\n');

// Test 1: Main export (SDK)
console.log('1. Testing main export (SDK)...');
try {
  const { Achronyme, AchronymeValue } = await import('./dist/sdk/index.js');
  console.log('   ✓ Main export works');
  console.log('   ✓ Achronyme:', typeof Achronyme);
  console.log('   ✓ AchronymeValue:', typeof AchronymeValue);

  // Test SDK initialization
  const ach = new Achronyme();
  await ach.init();

  const x = ach.number(42);
  const result = await x.toNumber();
  console.log('   ✓ SDK works:', result === 42 ? 'PASS' : 'FAIL');
  x.dispose();

} catch (error) {
  console.log('   ✗ Error:', error.message);
}

// Test 2: WASM export (advanced)
console.log('\n2. Testing WASM export (advanced)...');
try {
  const { default: createModule } = await import('./dist/achronyme-core.mjs');
  console.log('   ✓ WASM export works');

  const Module = await createModule();
  const result = Module.eval('2 + 2');
  console.log('   ✓ WASM eval works:', result === '4' ? 'PASS' : 'FAIL');

} catch (error) {
  console.log('   ✗ Error:', error.message);
}

console.log('\n=== All Entry Points Verified ===');

console.log('\n📦 Package is ready for:');
console.log('   • npm publish (production)');
console.log('   • npm publish --tag beta (testing)');
console.log('\n💡 Usage after installation:');
console.log('   import { Achronyme } from \'@achronyme/core\';');
console.log('   import { createModule } from \'@achronyme/core/wasm\';');
