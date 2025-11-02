import { Achronyme } from './dist/sdk/index.js';

console.log('🔍 DIAGNÓSTICO CORREGIDO: Verificando handles con FAST PATH forzado\n');

// IMPORTANTE: Activar alwaysUseFastPath para forzar fast path incluso con vectores pequeños
const ach = new Achronyme({ debug: true, alwaysUseFastPath: true });
await ach.init();

// Test 1: Verificar que vector tiene handle
console.log('═══════════════════════════════════════════════════');
console.log('TEST 1: ¿Los vectores tienen handles? (fast path forzado)');
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

// Test 3: Verificar que exp() con handle es rápido
console.log('═══════════════════════════════════════════════════');
console.log('TEST 3: Benchmark exp() - 10K elementos');
console.log('═══════════════════════════════════════════════════');

const data = Array.from({ length: 10000 }, (_, i) => (i % 100) / 10 + 0.1);
const vector = ach.vector(data);
console.log('Vector 10K tiene handle?', vector.handle !== undefined ? '✅' : '❌');

// WASM
const wasmStart = performance.now();
const wasmResult = ach.exp(vector);
console.log('Resultado exp tiene handle?', wasmResult.handle !== undefined ? '✅' : '❌');
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

// Test 5: Verificar múltiples operaciones matemáticas
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

console.log('═══════════════════════════════════════════════════');
console.log('CONCLUSIÓN');
console.log('═══════════════════════════════════════════════════');
console.log('Si los vectores tienen handles pero exp/sqrt/sin/abs no,');
console.log('entonces confirma que las funciones matemáticas NO están');
console.log('usando fast path y necesitan implementación.');
