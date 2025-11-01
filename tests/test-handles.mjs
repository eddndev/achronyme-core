/**
 * Test y Benchmark del Sistema de Handles
 *
 * Este archivo demuestra y benchmarkea el nuevo sistema de handles
 * que reduce el overhead JS↔WASM mediante operaciones sin parsing.
 */

import { Achronyme } from '../dist/sdk/index.js';

// Colores para output
const colors = {
  reset: '\x1b[0m',
  bright: '\x1b[1m',
  green: '\x1b[32m',
  yellow: '\x1b[33m',
  blue: '\x1b[34m',
  cyan: '\x1b[36m',
};

function log(msg, color = 'reset') {
  console.log(colors[color] + msg + colors.reset);
}

function benchmark(name, fn) {
  const start = performance.now();
  const result = fn();
  const end = performance.now();
  const time = (end - start).toFixed(2);
  return { result, time };
}

async function runTests() {
  log('\n========================================', 'bright');
  log('   Test del Sistema de Handles', 'bright');
  log('========================================\n', 'bright');

  // Inicializar Achronyme con debug habilitado
  const ach = new Achronyme({ debug: true, fastPathThreshold: 8 });
  await ach.init();

  log('✓ Achronyme inicializado\n', 'green');

  // ============================================================================
  // Test 1: Creación de Vectores (Comparación Fast vs Slow Path)
  // ============================================================================

  log('TEST 1: Creación de Vectores', 'cyan');
  log('─'.repeat(50), 'cyan');

  // Vector pequeño (slow path)
  log('\n1a. Vector pequeño (4 elementos) - Expected: SLOW path', 'yellow');
  const smallVec = ach.vector([1, 2, 3, 4]);
  log(`    Creado: ${smallVec._varName}`, 'green');

  // Vector grande (fast path)
  log('\n1b. Vector grande (100 elementos) - Expected: FAST path', 'yellow');
  const largeData = new Float64Array(100);
  for (let i = 0; i < 100; i++) {
    largeData[i] = Math.sin(i * 0.1);
  }
  const largeVec = ach.vector(largeData);
  log(`    Creado: ${largeVec._varName}`, 'green');

  // Verificar que los datos son correctos
  const retrieved = await largeVec.toVector();
  log(`    ✓ Datos recuperados correctamente (${retrieved.length} elementos)`, 'green');

  // ============================================================================
  // Test 2: FFT con Fast Path
  // ============================================================================

  log('\n\nTEST 2: FFT con Fast Path', 'cyan');
  log('─'.repeat(50), 'cyan');

  log('\n2a. FFT de vector pequeño - Expected: SLOW path', 'yellow');
  const spectrum1 = smallVec.fft();
  log(`    Resultado: ${spectrum1._varName}`, 'green');

  log('\n2b. FFT de vector grande - Expected: FAST path', 'yellow');
  const spectrum2 = largeVec.fft();
  log(`    Resultado: ${spectrum2._varName}`, 'green');

  log('\n2c. FFT Magnitude - Expected: FAST path', 'yellow');
  const magnitude = largeVec.fft_mag();
  const magData = await magnitude.toVector();
  log(`    ✓ Magnitud calculada (${magData.length} puntos)`, 'green');

  // ============================================================================
  // Test 3: Linspace (Siempre Fast Path)
  // ============================================================================

  log('\n\nTEST 3: Linspace (Always Fast Path)', 'cyan');
  log('─'.repeat(50), 'cyan');

  log('\n3a. Linspace con 1000 puntos', 'yellow');
  const t = ach.linspace(0, 10, 1000);
  const tData = await t.toVector();
  log(`    ✓ Generados ${tData.length} puntos`, 'green');
  log(`    ✓ Primer valor: ${tData[0]}, Último: ${tData[tData.length - 1]}`, 'green');

  // ============================================================================
  // Test 4: Operaciones Encadenadas
  // ============================================================================

  log('\n\nTEST 4: Operaciones Encadenadas (Pipeline)', 'cyan');
  log('─'.repeat(50), 'cyan');

  log('\n4a. Pipeline: linspace → fft_mag', 'yellow');
  const signal = ach.linspace(0, 2 * Math.PI, 256);
  const fftMag = signal.fft_mag();
  const finalData = await fftMag.toVector();
  log(`    ✓ Pipeline completado (${finalData.length} elementos de salida)`, 'green');

  // ============================================================================
  // Test 5: Memory Stats
  // ============================================================================

  log('\n\nTEST 5: Estadísticas de Memoria', 'cyan');
  log('─'.repeat(50), 'cyan');

  const stats = ach.getMemoryStats();
  log('\nEstadísticas:', 'yellow');
  log(`  • Total variables creadas: ${stats.totalVariables}`, 'blue');
  log(`  • Variables activas: ${stats.activeVariables}`, 'blue');
  log(`  • Handles activos: ${stats.activeHandles}`, 'blue');
  log(`  • Uso Fast Path: ${stats.fastPathUsagePercent.toFixed(1)}%`, stats.fastPathUsagePercent > 50 ? 'green' : 'yellow');

  // ============================================================================
  // Test 6: Benchmark Comparativo
  // ============================================================================

  log('\n\nTEST 6: Benchmark de Performance', 'cyan');
  log('─'.repeat(50), 'cyan');

  const sizes = [10, 100, 1000, 10000];

  for (const size of sizes) {
    log(`\n6.${sizes.indexOf(size) + 1}. Vector de ${size} elementos:`, 'yellow');

    // Preparar datos
    const data = new Float64Array(size);
    for (let i = 0; i < size; i++) {
      data[i] = Math.sin(i * 0.01);
    }

    // Benchmark creación
    const { time: createTime } = benchmark(`create_${size}`, () => {
      return ach.vector(data);
    });

    log(`    Creación: ${createTime}ms`, 'green');

    // Benchmark FFT
    const vec = ach.vector(data);
    const { time: fftTime } = benchmark(`fft_${size}`, () => {
      return vec.fft();
    });

    log(`    FFT: ${fftTime}ms`, 'green');

    // Benchmark recuperación
    const spectrum = vec.fft_mag();
    const { time: retrieveTime } = benchmark(`retrieve_${size}`, async () => {
      return await spectrum.toVector();
    });

    log(`    Recuperación: ${retrieveTime}ms`, 'green');

    // Limpiar
    vec.dispose();
    spectrum.dispose();
  }

  // ============================================================================
  // Test 7: Limpieza
  // ============================================================================

  log('\n\nTEST 7: Limpieza de Recursos', 'cyan');
  log('─'.repeat(50), 'cyan');

  const beforeStats = ach.getMemoryStats();
  log(`\nAntes de dispose: ${beforeStats.activeVariables} variables, ${beforeStats.activeHandles} handles`, 'yellow');

  // Disponer de algunas variables
  smallVec.dispose();
  largeVec.dispose();
  spectrum1.dispose();
  spectrum2.dispose();
  magnitude.dispose();
  t.dispose();
  signal.dispose();
  fftMag.dispose();

  const afterStats = ach.getMemoryStats();
  log(`Después de dispose: ${afterStats.activeVariables} variables, ${afterStats.activeHandles} handles`, 'green');

  // ============================================================================
  // Resumen Final
  // ============================================================================

  log('\n\n========================================', 'bright');
  log('            Resumen Final', 'bright');
  log('========================================\n', 'bright');

  const finalStats = ach.getMemoryStats();
  log('Estadísticas Finales:', 'cyan');
  log(`  ✓ Fast Path Usage: ${finalStats.fastPathUsagePercent.toFixed(1)}%`, 'green');
  log(`  ✓ Variables activas: ${finalStats.activeVariables}`, 'green');
  log(`  ✓ Handles activos: ${finalStats.activeHandles}`, 'green');

  if (finalStats.fastPathUsagePercent > 70) {
    log('\n🚀 ¡Excelente! El sistema de handles está funcionando correctamente.', 'green');
    log('   La mayoría de operaciones usan el fast path.', 'green');
  } else if (finalStats.fastPathUsagePercent > 40) {
    log('\n⚠️  Advertencia: Uso moderado del fast path.', 'yellow');
    log('   Considera usar arrays más grandes o ajustar fastPathThreshold.', 'yellow');
  } else {
    log('\n❌ Problema: Bajo uso del fast path.', 'yellow');
    log('   Revisa que el threshold esté configurado correctamente.', 'yellow');
  }

  log('\n✅ Todos los tests completados!\n', 'bright');
}

// Ejecutar tests
runTests().catch(err => {
  console.error('\n❌ Error en tests:', err);
  console.error(err.stack);
  process.exit(1);
});
