import { Achronyme } from './dist/sdk/index.js';

console.log('🔍 DIAGNÓSTICO: Verificando uso de handles en funciones matemáticas\n');

const ach = new Achronyme({ debug: false });
await ach.init();

// Test 1: Verificar que vector tiene handle
console.log('═══════════════════════════════════════════════════');
console.log('TEST 1: ¿Los vectores tienen handles?');
console.log('═══════════════════════════════════════════════════');

const v = ach.vector([1, 2, 3, 4, 5]);
console.log('Vector creado:', v._varName);
console.log('¿Tiene handle?', v.handle !== undefined ? '✅ SÍ' : '❌ NO');
console.log('Handle value:', v.handle);
console.log();

// Test 2: Verificar resultado de exp()
console.log('═══════════════════════════════════════════════════');
console.log('TEST 2: ¿ach.exp(vector) retorna handle?');
console.log('═══════════════════════════════════════════════════');

const expResult = ach.exp(v);
console.log('Resultado exp creado:', expResult._varName);
console.log('¿Tiene handle?', expResult.handle !== undefined ? '✅ SÍ' : '❌ NO');
console.log('Handle value:', expResult.handle);
console.log();

// Test 3: Benchmark mini para confirmar performance
console.log('═══════════════════════════════════════════════════');
console.log('TEST 3: Mini benchmark exp() - 10K elementos');
console.log('═══════════════════════════════════════════════════');

const data = Array.from({ length: 10000 }, (_, i) => (i % 100) / 10 + 0.1);
const vector = ach.vector(data);

// WASM
const wasmStart = performance.now();
const wasmResult = ach.exp(vector);
const wasmData = await wasmResult.toVector();
const wasmTime = performance.now() - wasmStart;

// JavaScript
const jsStart = performance.now();
const jsResult = data.map(x => Math.exp(x));
const jsTime = performance.now() - jsStart;

console.log('WASM (ach.exp):', wasmTime.toFixed(2), 'ms');
console.log('JavaScript (map):', jsTime.toFixed(2), 'ms');
console.log('Ratio:', (wasmTime / jsTime).toFixed(2) + 'x', wasmTime < jsTime ? '✅ WASM más rápido' : '❌ JS más rápido');
console.log();

// Test 4: Verificar stats de memoria
console.log('═══════════════════════════════════════════════════');
console.log('TEST 4: Estadísticas de memoria');
console.log('═══════════════════════════════════════════════════');

const stats = ach.getMemoryStats();
console.log('Fast path operations:', stats.fastPathOperationsCount);
console.log('Slow path operations:', stats.slowPathOperationsCount);
console.log('Fast path usage:', ((stats.fastPathOperationsCount / (stats.fastPathOperationsCount + stats.slowPathOperationsCount)) * 100).toFixed(1) + '%');
console.log();

// Test 5: Verificar múltiples operaciones en cadena
console.log('═══════════════════════════════════════════════════');
console.log('TEST 5: Cadena de operaciones matemáticas');
console.log('═══════════════════════════════════════════════════');

const v2 = ach.vector([1, 2, 3, 4, 5]);
console.log('v2 tiene handle?', v2.handle !== undefined ? '✅' : '❌');

const r1 = ach.exp(v2);
console.log('exp(v2) tiene handle?', r1.handle !== undefined ? '✅' : '❌');

const r2 = ach.sqrt(v2);
console.log('sqrt(v2) tiene handle?', r2.handle !== undefined ? '✅' : '❌');

const r3 = ach.sin(v2);
console.log('sin(v2) tiene handle?', r3.handle !== undefined ? '✅' : '❌');

const r4 = ach.abs(v2);
console.log('abs(v2) tiene handle?', r4.handle !== undefined ? '✅' : '❌');
console.log();

// Test 6: Benchmark más realista (como en el demo)
console.log('═══════════════════════════════════════════════════');
console.log('TEST 6: Benchmark realista - 50K × 10 iteraciones');
console.log('═══════════════════════════════════════════════════');

const testData = Array.from({ length: 50000 }, (_, i) => (i % 100) / 10 + 0.1);
const testVector = ach.vector(testData);

// WASM con múltiples iteraciones
const wasmMultiStart = performance.now();
for (let i = 0; i < 10; i++) {
  const result = ach.exp(testVector);
  await result.toVector();
  result.dispose();
}
const wasmMultiTime = performance.now() - wasmMultiStart;

// JS con múltiples iteraciones
const jsMultiStart = performance.now();
for (let i = 0; i < 10; i++) {
  const result = testData.map(x => Math.exp(x));
}
const jsMultiTime = performance.now() - jsMultiStart;

console.log('WASM (50K × 10):', wasmMultiTime.toFixed(2), 'ms');
console.log('JavaScript (50K × 10):', jsMultiTime.toFixed(2), 'ms');
console.log('Ratio:', (wasmMultiTime / jsMultiTime).toFixed(2) + 'x');

if (wasmMultiTime > jsMultiTime * 5) {
  console.log('');
  console.log('⚠️  PROBLEMA CONFIRMADO: WASM es >5x más lento');
  console.log('   Esto indica que NO está usando fast path');
} else if (wasmMultiTime > jsMultiTime * 2) {
  console.log('');
  console.log('⚠️  WASM es 2-5x más lento - Posible problema de overhead');
} else if (wasmMultiTime > jsMultiTime) {
  console.log('');
  console.log('ℹ️  WASM es un poco más lento - Competitivo pero JS gana');
} else {
  console.log('');
  console.log('✅ WASM es más rápido - Fast path funcionando!');
}

// Cleanup
v.dispose();
expResult.dispose();
vector.dispose();
wasmResult.dispose();
v2.dispose();
r1.dispose();
r2.dispose();
r3.dispose();
r4.dispose();
testVector.dispose();

console.log();
console.log('═══════════════════════════════════════════════════');
console.log('CONCLUSIÓN');
console.log('═══════════════════════════════════════════════════');
console.log('Revisa los resultados arriba para confirmar:');
console.log('1. Si las funciones matemáticas retornan handles');
console.log('2. El ratio WASM vs JS en benchmarks');
console.log('3. Las estadísticas de fast/slow path');
