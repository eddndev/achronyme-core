import { Achronyme } from './dist/sdk/index.js';

console.log('🔬 BENCHMARK: Operaciones matemáticas vectorizadas\n');
console.log('Comparando WASM vs JavaScript en operaciones element-wise\n');

const ach = new Achronyme({ debug: false, alwaysUseFastPath: true });
await ach.init();

const sizes = [1000, 10000, 50000, 100000];
const iterations = 10;

for (const size of sizes) {
  console.log(`═══════════════════════════════════════════════════`);
  console.log(`Tamaño: ${size.toLocaleString()} elementos × ${iterations} iteraciones`);
  console.log(`═══════════════════════════════════════════════════`);

  // Generar datos de prueba
  const data = Array.from({ length: size }, (_, i) => (i % 100) / 10 + 0.1);
  const vector = ach.vector(data);

  // Test exp()
  let wasmExpTime = 0;
  for (let i = 0; i < iterations; i++) {
    const start = performance.now();
    const result = ach.exp(vector);
    await result.toVector();
    wasmExpTime += performance.now() - start;
    result.dispose();
  }
  wasmExpTime /= iterations;

  let jsExpTime = 0;
  for (let i = 0; i < iterations; i++) {
    const start = performance.now();
    const result = data.map(x => Math.exp(x));
    jsExpTime += performance.now() - start;
  }
  jsExpTime /= iterations;

  // Test sin()
  let wasmSinTime = 0;
  for (let i = 0; i < iterations; i++) {
    const start = performance.now();
    const result = ach.sin(vector);
    await result.toVector();
    wasmSinTime += performance.now() - start;
    result.dispose();
  }
  wasmSinTime /= iterations;

  let jsSinTime = 0;
  for (let i = 0; i < iterations; i++) {
    const start = performance.now();
    const result = data.map(x => Math.sin(x));
    jsSinTime += performance.now() - start;
  }
  jsSinTime /= iterations;

  // Test sqrt()
  let wasmSqrtTime = 0;
  for (let i = 0; i < iterations; i++) {
    const start = performance.now();
    const result = ach.sqrt(vector);
    await result.toVector();
    wasmSqrtTime += performance.now() - start;
    result.dispose();
  }
  wasmSqrtTime /= iterations;

  let jsSqrtTime = 0;
  for (let i = 0; i < iterations; i++) {
    const start = performance.now();
    const result = data.map(x => Math.sqrt(x));
    jsSqrtTime += performance.now() - start;
  }
  jsSqrtTime /= iterations;

  // Resultados
  console.log('\nexp():');
  console.log(`  WASM:       ${wasmExpTime.toFixed(3)} ms`);
  console.log(`  JavaScript: ${jsExpTime.toFixed(3)} ms`);
  console.log(`  Ratio:      ${(wasmExpTime / jsExpTime).toFixed(2)}x ${wasmExpTime < jsExpTime ? '✅ WASM gana' : '❌ JS gana'}`);

  console.log('\nsin():');
  console.log(`  WASM:       ${wasmSinTime.toFixed(3)} ms`);
  console.log(`  JavaScript: ${jsSinTime.toFixed(3)} ms`);
  console.log(`  Ratio:      ${(wasmSinTime / jsSinTime).toFixed(2)}x ${wasmSinTime < jsSinTime ? '✅ WASM gana' : '❌ JS gana'}`);

  console.log('\nsqrt():');
  console.log(`  WASM:       ${wasmSqrtTime.toFixed(3)} ms`);
  console.log(`  JavaScript: ${jsSqrtTime.toFixed(3)} ms`);
  console.log(`  Ratio:      ${(wasmSqrtTime / jsSqrtTime).toFixed(2)}x ${wasmSqrtTime < jsSqrtTime ? '✅ WASM gana' : '❌ JS gana'}`);

  console.log();
  vector.dispose();
}

console.log('═══════════════════════════════════════════════════');
console.log('ANÁLISIS');
console.log('═══════════════════════════════════════════════════');
console.log('Si WASM sigue perdiendo, el problema es que el overhead');
console.log('de handles + creación de vectores + toVector() es mayor');
console.log('que el ahorro en la operación matemática misma.');
console.log('\nEl V8 de JavaScript está extremadamente optimizado para');
console.log('operaciones map() en arrays, y es difícil de vencer.');
