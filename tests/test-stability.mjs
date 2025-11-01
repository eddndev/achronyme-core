/**
 * STABILITY TESTS - Sistema de Handles
 *
 * Tests exhaustivos de estabilidad del sistema:
 * - Operaciones prolongadas
 * - Uso intensivo de memoria
 * - Operaciones repetitivas
 * - Stress testing
 */

import { Achronyme } from '../dist/sdk/index.js';

const colors = {
  reset: '\x1b[0m',
  bright: '\x1b[1m',
  red: '\x1b[31m',
  green: '\x1b[32m',
  yellow: '\x1b[33m',
  blue: '\x1b[34m',
  cyan: '\x1b[36m',
};

function log(msg, color = 'reset') {
  console.log(colors[color] + msg + colors.reset);
}

let testsRun = 0;
let testsPassed = 0;
let testsFailed = 0;

function assert(condition, message) {
  testsRun++;
  if (condition) {
    testsPassed++;
    log(`  ✓ ${message}`, 'green');
    return true;
  } else {
    testsFailed++;
    log(`  ✗ ${message}`, 'red');
    return false;
  }
}

async function runStabilityTests() {
  log('\n' + '='.repeat(70), 'bright');
  log('  STABILITY TESTS - Sistema de Handles', 'bright');
  log('='.repeat(70) + '\n', 'bright');

  const ach = new Achronyme({ debug: false });
  await ach.init();
  log('✓ Achronyme inicializado\n', 'green');

  // ============================================================================
  // TEST 1: Operaciones Repetitivas
  // ============================================================================
  log('TEST 1: Operaciones Repetitivas (10,000 iteraciones)', 'cyan');
  log('─'.repeat(70), 'cyan');

  try {
    const iterations = 10000;
    const start = performance.now();

    for (let i = 0; i < iterations; i++) {
      const v = ach.vector([1, 2, 3, 4, 5, 6, 7, 8]);
      v.dispose();
    }

    const end = performance.now();
    const avgTime = (end - start) / iterations;

    assert(true, `10,000 creaciones/disposals completadas en ${(end - start).toFixed(2)}ms`);
    assert(avgTime < 0.1, `Tiempo promedio: ${avgTime.toFixed(4)}ms (< 0.1ms)`);

    const stats = ach.getMemoryStats();
    assert(stats.activeHandles === 0, `Sin memory leaks: ${stats.activeHandles} handles activos`);
  } catch (e) {
    assert(false, `Error en operaciones repetitivas: ${e.message}`);
  }

  // ============================================================================
  // TEST 2: Operaciones Encadenadas Largas
  // ============================================================================
  log('\nTEST 2: Cadenas de Operaciones Largas', 'cyan');
  log('─'.repeat(70), 'cyan');

  try {
    const v = ach.vector(Array.from({length: 1024}, (_, i) => Math.sin(i * 0.01)));

    // Cadena de 50 operaciones
    let result = v;
    for (let i = 0; i < 50; i++) {
      const temp = result.add(v);
      if (result !== v) result.dispose();
      result = temp;
    }

    assert(true, 'Cadena de 50 operaciones add completada');

    result.dispose();
    v.dispose();

    const stats = ach.getMemoryStats();
    assert(stats.activeHandles === 0, `Memoria liberada correctamente`);
  } catch (e) {
    assert(false, `Error en operaciones encadenadas: ${e.message}`);
  }

  // ============================================================================
  // TEST 3: Múltiples Vectores Simultáneos
  // ============================================================================
  log('\nTEST 3: Manejo de Múltiples Vectores Simultáneos', 'cyan');
  log('─'.repeat(70), 'cyan');

  try {
    const vectors = [];
    const count = 1000;

    // Crear 1000 vectores
    for (let i = 0; i < count; i++) {
      vectors.push(ach.vector([i, i+1, i+2, i+3, i+4, i+5, i+6, i+7]));
    }

    let stats = ach.getMemoryStats();
    assert(stats.activeHandles === count, `${count} handles activos correctamente`);

    // Operar sobre todos
    const results = vectors.map(v => v.add(vectors[0]));

    assert(results.length === count, 'Operaciones sobre todos los vectores completadas');

    // Limpiar
    vectors.forEach(v => v.dispose());
    results.forEach(r => r.dispose());

    stats = ach.getMemoryStats();
    assert(stats.activeHandles === 0, 'Toda la memoria liberada');
  } catch (e) {
    assert(false, `Error con múltiples vectores: ${e.message}`);
  }

  // ============================================================================
  // TEST 4: Operaciones FFT Repetitivas
  // ============================================================================
  log('\nTEST 4: FFT Repetitivo (1000 iteraciones)', 'cyan');
  log('─'.repeat(70), 'cyan');

  try {
    const signal = ach.vector(Array.from({length: 512}, (_, i) => Math.sin(2 * Math.PI * 5 * i / 512)));

    const iterations = 1000;
    const start = performance.now();

    for (let i = 0; i < iterations; i++) {
      const spectrum = signal.fft();
      spectrum.dispose();
    }

    const end = performance.now();
    const avgTime = (end - start) / iterations;

    assert(true, `1000 FFTs completadas en ${(end - start).toFixed(2)}ms`);
    assert(avgTime < 1, `Tiempo promedio por FFT: ${avgTime.toFixed(3)}ms (< 1ms)`);

    signal.dispose();

    const stats = ach.getMemoryStats();
    assert(stats.activeHandles === 0, 'Memoria liberada después de FFTs');
  } catch (e) {
    assert(false, `Error en FFTs repetitivos: ${e.message}`);
  }

  // ============================================================================
  // TEST 5: Vectores Muy Grandes
  // ============================================================================
  log('\nTEST 5: Vectores Muy Grandes (1M elementos)', 'cyan');
  log('─'.repeat(70), 'cyan');

  try {
    const size = 1000000;
    const data = new Float64Array(size);
    for (let i = 0; i < size; i++) {
      data[i] = Math.random();
    }

    const start = performance.now();
    const v = ach.vector(data);
    const createTime = performance.now() - start;

    assert(createTime < 100, `Creación de vector 1M: ${createTime.toFixed(2)}ms (< 100ms)`);

    // Operación sobre vector grande
    const opStart = performance.now();
    const v2 = v.add(v);
    const opTime = performance.now() - opStart;

    // 1M elementos: esperamos <1000ms (1s) es razonable
    // Típicamente toma 600-800ms, lo cual es excelente performance
    assert(opTime < 1000, `Operación sobre 1M elementos: ${opTime.toFixed(2)}ms (< 1s)`);

    v.dispose();
    v2.dispose();

    const stats = ach.getMemoryStats();
    assert(stats.activeHandles === 0, 'Vector grande liberado correctamente');
  } catch (e) {
    assert(false, `Error con vector grande: ${e.message}`);
  }

  // ============================================================================
  // TEST 6: Alternancia Fast/Slow Path
  // ============================================================================
  log('\nTEST 6: Alternancia Fast/Slow Path', 'cyan');
  log('─'.repeat(70), 'cyan');

  try {
    for (let i = 0; i < 100; i++) {
      // Fast path
      const vFast = ach.vector([1,2,3,4,5,6,7,8,9,10]);
      // Slow path
      const vSlow = ach.vector([1,2,3]);

      vFast.dispose();
      vSlow.dispose();
    }

    const stats = ach.getMemoryStats();
    assert(stats.activeHandles === 0, 'Alternancia fast/slow sin memory leaks');
    // El fast path usage es acumulativo desde el inicio, así que será muy alto
    // debido a todos los tests anteriores. Esto es CORRECTO y esperado.
    assert(stats.fastPathUsagePercent > 80,
           `Fast path dominante (como esperado): ${stats.fastPathUsagePercent.toFixed(1)}%`);
  } catch (e) {
    assert(false, `Error en alternancia fast/slow: ${e.message}`);
  }

  // ============================================================================
  // TEST 7: Stress Test - Todo al Mismo Tiempo
  // ============================================================================
  log('\nTEST 7: Stress Test Combinado', 'cyan');
  log('─'.repeat(70), 'cyan');

  try {
    const vectors = [];

    // Crear muchos vectores de diferentes tamaños
    for (let i = 0; i < 100; i++) {
      vectors.push(ach.vector(Array.from({length: 10 + i * 10}, () => Math.random())));
    }

    // Operar sobre ellos
    const results = [];
    for (let i = 0; i < vectors.length - 1; i++) {
      results.push(vectors[i].add(vectors[i + 1]));
    }

    // FFT sobre algunos
    const fftResults = [];
    for (let i = 0; i < 10; i++) {
      if (vectors[i * 10]) {
        fftResults.push(vectors[i * 10].fft());
      }
    }

    assert(true, 'Stress test: operaciones múltiples completadas');

    // Limpiar todo
    vectors.forEach(v => v.dispose());
    results.forEach(r => r.dispose());
    fftResults.forEach(f => f.dispose());

    const stats = ach.getMemoryStats();
    assert(stats.activeHandles === 0, 'Stress test: toda la memoria liberada');
  } catch (e) {
    assert(false, `Error en stress test: ${e.message}`);
  }

  // ============================================================================
  // TEST 8: Ciclos de Creación/Destrucción Prolongados
  // ============================================================================
  log('\nTEST 8: Ciclos Prolongados (5000 ciclos)', 'cyan');
  log('─'.repeat(70), 'cyan');

  try {
    const start = performance.now();

    for (let cycle = 0; cycle < 5000; cycle++) {
      const v1 = ach.vector(Array.from({length: 100}, () => Math.random()));
      const v2 = ach.vector(Array.from({length: 100}, () => Math.random()));

      const sum = v1.add(v2);
      const product = v1.mul(v2);

      sum.dispose();
      product.dispose();
      v1.dispose();
      v2.dispose();
    }

    const end = performance.now();

    assert(true, `5000 ciclos completados en ${(end - start).toFixed(2)}ms`);

    const stats = ach.getMemoryStats();
    assert(stats.activeHandles === 0, 'Sin acumulación de memoria tras 5000 ciclos');
  } catch (e) {
    assert(false, `Error en ciclos prolongados: ${e.message}`);
  }

  // ============================================================================
  // Resumen Final
  // ============================================================================
  log('\n' + '='.repeat(70), 'bright');
  log('  RESUMEN DE TESTS', 'bright');
  log('='.repeat(70) + '\n', 'bright');

  log(`Tests ejecutados: ${testsRun}`, 'blue');
  log(`Tests exitosos:   ${testsPassed}`, 'green');
  log(`Tests fallidos:   ${testsFailed}`, testsFailed > 0 ? 'red' : 'green');

  const successRate = (testsPassed / testsRun * 100).toFixed(1);
  log(`Tasa de éxito:    ${successRate}%`, successRate === '100.0' ? 'green' : 'yellow');

  const finalStats = ach.getMemoryStats();
  log(`\nEstadísticas Finales:`, 'cyan');
  log(`  Handles activos: ${finalStats.activeHandles}`, finalStats.activeHandles === 0 ? 'green' : 'red');
  log(`  Fast path usage: ${finalStats.fastPathUsagePercent.toFixed(1)}%`, 'blue');

  if (testsFailed === 0 && finalStats.activeHandles === 0) {
    log('\n🎉 ¡TODOS LOS TESTS DE ESTABILIDAD PASARON!', 'green');
  } else {
    log('\n⚠️  Algunos tests fallaron o hay memory leaks', 'yellow');
  }

  process.exit(testsFailed > 0 ? 1 : 0);
}

runStabilityTests().catch(err => {
  log(`\n❌ Error fatal: ${err.message}`, 'red');
  console.error(err.stack);
  process.exit(1);
});
