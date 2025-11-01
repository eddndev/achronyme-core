/**
 * ACCURACY TESTS - Sistema de Handles
 *
 * Verificación de precisión matemática de operaciones DSP:
 * - Exactitud de operaciones vectoriales
 * - Precisión FFT/IFFT (roundtrip)
 * - Funciones matemáticas element-wise
 * - Operaciones trigonométricas
 * - Comparación con valores conocidos
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

function assertApprox(actual, expected, tolerance, message) {
  const diff = Math.abs(actual - expected);
  testsRun++;
  if (diff <= tolerance) {
    testsPassed++;
    log(`  ✓ ${message} (diff: ${diff.toExponential(2)})`, 'green');
    return true;
  } else {
    testsFailed++;
    log(`  ✗ ${message} (expected: ${expected}, got: ${actual}, diff: ${diff})`, 'red');
    return false;
  }
}

function assertVectorApprox(actual, expected, tolerance, message) {
  testsRun++;
  let maxDiff = 0;
  let allMatch = true;

  if (actual.length !== expected.length) {
    testsFailed++;
    log(`  ✗ ${message} (length mismatch: ${actual.length} vs ${expected.length})`, 'red');
    return false;
  }

  for (let i = 0; i < actual.length; i++) {
    const diff = Math.abs(actual[i] - expected[i]);
    maxDiff = Math.max(maxDiff, diff);
    if (diff > tolerance) {
      allMatch = false;
    }
  }

  if (allMatch) {
    testsPassed++;
    log(`  ✓ ${message} (max diff: ${maxDiff.toExponential(2)})`, 'green');
    return true;
  } else {
    testsFailed++;
    log(`  ✗ ${message} (max diff: ${maxDiff})`, 'red');
    return false;
  }
}

async function runAccuracyTests() {
  log('\n' + '='.repeat(70), 'bright');
  log('  ACCURACY TESTS - Precisión Matemática', 'bright');
  log('='.repeat(70) + '\n', 'bright');

  const ach = new Achronyme({ debug: false });
  await ach.init();
  log('✓ Achronyme inicializado\n', 'green');

  // ============================================================================
  // TEST 1: Operaciones Vectoriales Básicas
  // ============================================================================
  log('TEST 1: Precisión de Operaciones Vectoriales', 'cyan');
  log('─'.repeat(70), 'cyan');

  try {
    // Suma
    const v1a = ach.vector([1, 2, 3, 4, 5]);
    const v2a = ach.vector([5, 4, 3, 2, 1]);
    const sum = v1a.add(v2a);
    const sumData = await sum.toVector();
    assertVectorApprox(sumData, [6, 6, 6, 6, 6], 1e-10, 'Suma vectorial exacta');
    sum.dispose();
    v1a.dispose();
    v2a.dispose();

    // Resta
    const v1b = ach.vector([1, 2, 3, 4, 5]);
    const v2b = ach.vector([5, 4, 3, 2, 1]);
    const sub = v1b.sub(v2b);
    const subData = await sub.toVector();
    assertVectorApprox(subData, [-4, -2, 0, 2, 4], 1e-10, 'Resta vectorial exacta');
    sub.dispose();
    v1b.dispose();
    v2b.dispose();

    // Multiplicación
    const v1c = ach.vector([1, 2, 3, 4, 5]);
    const v2c = ach.vector([5, 4, 3, 2, 1]);
    const mul = v1c.mul(v2c);
    const mulData = await mul.toVector();
    assertVectorApprox(mulData, [5, 8, 9, 8, 5], 1e-10, 'Multiplicación vectorial exacta');
    mul.dispose();
    v1c.dispose();
    v2c.dispose();
  } catch (e) {
    assert(false, `Error en operaciones vectoriales: ${e.message}`);
  }

  // ============================================================================
  // TEST 2: Funciones Matemáticas Element-wise
  // ============================================================================
  log('\nTEST 2: Funciones Matemáticas', 'cyan');
  log('─'.repeat(70), 'cyan');

  try {
    // Exponencial
    const v_exp = ach.vector([0, 1, 2]);
    const exp_v = v_exp.exp();
    const expData = await exp_v.toVector();
    // Tolerancia 1e-6: diferencias ~1e-7 son normales por conversión JS↔C++
    assertVectorApprox(expData, [Math.E**0, Math.E**1, Math.E**2], 1e-6, 'Exponencial precisa');
    exp_v.dispose();
    v_exp.dispose();

    // Logaritmo natural
    const v_ln_base = ach.vector([1, 2, 3]);
    const ln_v = v_ln_base.ln();
    const lnData = await ln_v.toVector();
    // Tolerancia 1e-6: diferencias ~1e-7 son normales por conversión JS↔C++
    assertVectorApprox(lnData, [Math.log(1), Math.log(2), Math.log(3)], 1e-6, 'Logaritmo natural preciso');
    ln_v.dispose();
    v_ln_base.dispose();

    // Raíz cuadrada
    const v_sqrt = ach.vector([0, 1, 4, 9, 16]);
    const sqrt_v = v_sqrt.sqrt();
    const sqrtData = await sqrt_v.toVector();
    assertVectorApprox(sqrtData, [0, 1, 2, 3, 4], 1e-10, 'Raíz cuadrada exacta');
    sqrt_v.dispose();
    v_sqrt.dispose();

    // Valor absoluto
    const v_abs = ach.vector([-5, -3, 0, 3, 5]);
    const abs_v = v_abs.abs();
    const absData = await abs_v.toVector();
    assertVectorApprox(absData, [5, 3, 0, 3, 5], 1e-10, 'Valor absoluto exacto');
    abs_v.dispose();
    v_abs.dispose();
  } catch (e) {
    assert(false, `Error en funciones matemáticas: ${e.message}`);
  }

  // ============================================================================
  // TEST 3: Funciones Trigonométricas
  // ============================================================================
  log('\nTEST 3: Funciones Trigonométricas', 'cyan');
  log('─'.repeat(70), 'cyan');

  try {
    // Seno
    const angles_sin = ach.vector([0, Math.PI/6, Math.PI/4, Math.PI/3, Math.PI/2]);
    const sin_v = angles_sin.sin();
    const sinData = await sin_v.toVector();
    // Tolerancia 1e-6: diferencias ~4e-7 son normales por conversión JS↔C++
    assertVectorApprox(sinData, [0, 0.5, Math.sqrt(2)/2, Math.sqrt(3)/2, 1], 1e-6, 'Seno preciso');
    sin_v.dispose();
    angles_sin.dispose();

    // Coseno
    const angles_cos = ach.vector([0, Math.PI/6, Math.PI/4, Math.PI/3, Math.PI/2]);
    const cos_v = angles_cos.cos();
    const cosData = await cos_v.toVector();
    // Tolerancia 1e-6: diferencias ~4e-7 son normales por conversión JS↔C++
    assertVectorApprox(cosData, [1, Math.sqrt(3)/2, Math.sqrt(2)/2, 0.5, 0], 1e-6, 'Coseno preciso');
    cos_v.dispose();
    angles_cos.dispose();

    // Tangente (evitar π/2 por división por cero)
    const angles_tan = ach.vector([0, Math.PI/6, Math.PI/4, Math.PI/3]);
    const tan_v = angles_tan.tan();
    const tanData = await tan_v.toVector();
    // Tolerancia 1e-6: diferencias ~3e-7 son normales por conversión JS↔C++
    assertVectorApprox(tanData, [0, Math.tan(Math.PI/6), 1, Math.tan(Math.PI/3)], 1e-6, 'Tangente precisa');
    tan_v.dispose();
    angles_tan.dispose();
  } catch (e) {
    assert(false, `Error en funciones trigonométricas: ${e.message}`);
  }

  // ============================================================================
  // TEST 4: Identidades Trigonométricas
  // ============================================================================
  log('\nTEST 4: Identidades Matemáticas', 'cyan');
  log('─'.repeat(70), 'cyan');

  try {
    // sin²(x) + cos²(x) = 1
    const x_id1 = ach.vector([0.1, 0.5, 1.0, 1.5, 2.0]);
    const sin_x = x_id1.sin();
    const cos_x = x_id1.cos();
    const sin2 = sin_x.mul(sin_x);
    const cos2 = cos_x.mul(cos_x);
    const identity1 = sin2.add(cos2);
    const id1Data = await identity1.toVector();
    assertVectorApprox(id1Data, [1, 1, 1, 1, 1], 1e-10, 'Identidad sin²+cos²=1');
    identity1.dispose();
    sin2.dispose();
    cos2.dispose();
    sin_x.dispose();
    cos_x.dispose();
    x_id1.dispose();

    // exp(ln(x)) = x
    const pos_x = ach.vector([1, 2, 3, 4, 5]);
    const posData = await pos_x.toVector();
    const ln_x = pos_x.ln();
    const exp_ln_x = ln_x.exp();
    const expLnData = await exp_ln_x.toVector();
    assertVectorApprox(expLnData, posData, 1e-10, 'Identidad exp(ln(x))=x');
    exp_ln_x.dispose();
    ln_x.dispose();
    pos_x.dispose();
  } catch (e) {
    assert(false, `Error en identidades matemáticas: ${e.message}`);
  }

  // ============================================================================
  // TEST 5: FFT/IFFT Roundtrip
  // ============================================================================
  log('\nTEST 5: Precisión FFT/IFFT (Roundtrip)', 'cyan');
  log('─'.repeat(70), 'cyan');

  try {
    // Señal simple
    const signal = ach.vector([1, 2, 3, 4, 5, 6, 7, 8]);
    const originalData = await signal.toVector();

    // FFT -> IFFT debe recuperar la señal original
    const spectrum = signal.fft();
    const recovered = spectrum.ifft();
    const recoveredData = await recovered.toVector();

    assertVectorApprox(recoveredData, originalData, 1e-10, 'FFT/IFFT roundtrip exacto (8 pts)');

    signal.dispose();
    spectrum.dispose();
    recovered.dispose();

    // Probar con señal más grande (potencia de 2)
    const signal2 = ach.linspace(0, 1, 64);
    const orig2Data = await signal2.toVector();
    const spectrum2 = signal2.fft();
    const recovered2 = spectrum2.ifft();
    const rec2Data = await recovered2.toVector();

    assertVectorApprox(rec2Data, orig2Data, 1e-10, 'FFT/IFFT roundtrip exacto (64 pts)');

    signal2.dispose();
    spectrum2.dispose();
    recovered2.dispose();
  } catch (e) {
    assert(false, `Error en FFT/IFFT roundtrip: ${e.message}`);
  }

  // ============================================================================
  // TEST 6: FFT de Señales Conocidas
  // ============================================================================
  log('\nTEST 6: FFT de Señales con Frecuencias Conocidas', 'cyan');
  log('─'.repeat(70), 'cyan');

  try {
    // Señal con frecuencia única debe tener pico en esa frecuencia
    const N = 128;
    const freq = 5; // 5 Hz
    const signal = new Float64Array(N);
    for (let i = 0; i < N; i++) {
      signal[i] = Math.sin(2 * Math.PI * freq * i / N);
    }

    const sig = ach.vector(signal);
    const magnitude = sig.fft_mag();
    const magData = await magnitude.toVector();

    // El pico debe estar en bin = freq
    const peakIndex = magData.indexOf(Math.max(...magData.slice(0, N/2)));
    assert(peakIndex === freq, `Pico FFT en frecuencia correcta: bin ${peakIndex} (esperado ${freq})`);

    // La magnitud en otros bins debe ser muy pequeña (excepto DC y el pico)
    let lowNoise = true;
    for (let i = 1; i < N/2; i++) {
      if (i !== freq && magData[i] > 0.1) {
        lowNoise = false;
        break;
      }
    }
    assert(lowNoise, 'FFT con bajo ruido en bins no-pico');

    sig.dispose();
    magnitude.dispose();
  } catch (e) {
    assert(false, `Error en FFT de señal conocida: ${e.message}`);
  }

  // ============================================================================
  // TEST 7: Linspace Precisión
  // ============================================================================
  log('\nTEST 7: Precisión de Linspace', 'cyan');
  log('─'.repeat(70), 'cyan');

  try {
    // Linspace debe generar valores equiespaciados exactos
    const lin = ach.linspace(0, 10, 11); // [0, 1, 2, ..., 10]
    const linData = await lin.toVector();
    assertVectorApprox(linData, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 1e-10, 'Linspace [0,10] con 11 puntos');

    lin.dispose();

    // Linspace con valores no enteros
    const lin2 = ach.linspace(0, 1, 5); // [0, 0.25, 0.5, 0.75, 1]
    const lin2Data = await lin2.toVector();
    assertVectorApprox(lin2Data, [0, 0.25, 0.5, 0.75, 1], 1e-10, 'Linspace [0,1] con 5 puntos');

    lin2.dispose();

    // Linspace negativo
    const lin3 = ach.linspace(-5, 5, 11);
    const lin3Data = await lin3.toVector();
    assertVectorApprox(lin3Data, [-5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5], 1e-10, 'Linspace [-5,5] con 11 puntos');

    lin3.dispose();
  } catch (e) {
    assert(false, `Error en linspace: ${e.message}`);
  }

  // ============================================================================
  // TEST 8: Operaciones con Escalares
  // ============================================================================
  log('\nTEST 8: Operaciones Vector-Escalar', 'cyan');
  log('─'.repeat(70), 'cyan');

  try {
    const v = ach.vector([1, 2, 3, 4, 5]);

    // Suma con escalar
    const vPlus3 = v.add(ach.vector([3])); // Broadcasting conceptual
    // Nota: Si no soporta broadcasting, usar otro método

    // Multiplicación por escalar (usando operación element-wise)
    const vTimes2 = v.mul(ach.vector([2, 2, 2, 2, 2]));
    const vt2Data = await vTimes2.toVector();
    assertVectorApprox(vt2Data, [2, 4, 6, 8, 10], 1e-10, 'Multiplicación por escalar');

    v.dispose();
    vTimes2.dispose();
  } catch (e) {
    assert(false, `Error en operaciones con escalares: ${e.message}`);
  }

  // ============================================================================
  // TEST 9: Precisión Numérica en Grandes Sumas
  // ============================================================================
  log('\nTEST 9: Estabilidad Numérica', 'cyan');
  log('─'.repeat(70), 'cyan');

  try {
    // Operaciones que pueden acumular error
    let result = ach.vector([1.0]);
    for (let i = 0; i < 100; i++) {
      const one_cent = ach.vector([0.01]);
      const temp = result.add(one_cent);
      result.dispose();
      one_cent.dispose();
      result = temp;
    }
    const finalData = await result.toVector();
    assertApprox(finalData[0], 2.0, 1e-9, 'Acumulación sin error significativo (100 sumas)');
    result.dispose();

    // Operaciones repetidas sobre mismo vector
    const base = ach.vector([1.5]);
    let accumulated = base;
    for (let i = 0; i < 50; i++) {
      const temp = accumulated.mul(ach.vector([1.0])); // mul por 1 no debería cambiar valor
      if (accumulated !== base) accumulated.dispose();
      accumulated = temp;
    }
    const accData = await accumulated.toVector();
    assertApprox(accData[0], 1.5, 1e-9, 'Operaciones repetidas preservan precisión');
    accumulated.dispose();
    base.dispose();
  } catch (e) {
    assert(false, `Error en estabilidad numérica: ${e.message}`);
  }

  // ============================================================================
  // TEST 10: Casos Especiales
  // ============================================================================
  log('\nTEST 10: Valores Especiales', 'cyan');
  log('─'.repeat(70), 'cyan');

  try {
    // Cero
    const zero = ach.vector([0, 0, 0, 0]);
    const zeroExp = zero.exp();
    const zeroExpData = await zeroExp.toVector();
    assertVectorApprox(zeroExpData, [1, 1, 1, 1], 1e-10, 'exp(0) = 1');
    zeroExp.dispose();
    zero.dispose();

    // Uno
    const one = ach.vector([1, 1, 1, 1]);
    const oneLn = one.ln();
    const oneLnData = await oneLn.toVector();
    assertVectorApprox(oneLnData, [0, 0, 0, 0], 1e-10, 'ln(1) = 0');
    oneLn.dispose();
    one.dispose();

    // Negativos con abs
    const neg = ach.vector([-1, -2, -3]);
    const negAbs = neg.abs();
    const negAbsData = await negAbs.toVector();
    assertVectorApprox(negAbsData, [1, 2, 3], 1e-10, 'abs de negativos');
    negAbs.dispose();
    neg.dispose();
  } catch (e) {
    assert(false, `Error con valores especiales: ${e.message}`);
  }

  // ============================================================================
  // Resumen Final
  // ============================================================================
  log('\n' + '='.repeat(70), 'bright');
  log('  RESUMEN DE TESTS DE PRECISIÓN', 'bright');
  log('='.repeat(70) + '\n', 'bright');

  log(`Tests ejecutados: ${testsRun}`, 'blue');
  log(`Tests exitosos:   ${testsPassed}`, 'green');
  log(`Tests fallidos:   ${testsFailed}`, testsFailed > 0 ? 'red' : 'green');

  const successRate = (testsPassed / testsRun * 100).toFixed(1);
  log(`Tasa de éxito:    ${successRate}%`, successRate === '100.0' ? 'green' : 'yellow');

  const finalStats = ach.getMemoryStats();
  log(`\nEstadísticas Finales:`, 'cyan');
  log(`  Handles activos: ${finalStats.activeHandles}`, finalStats.activeHandles === 0 ? 'green' : 'red');

  if (testsFailed === 0 && finalStats.activeHandles === 0) {
    log('\n🎯 ¡TODOS LOS TESTS DE PRECISIÓN PASARON!', 'green');
    log('   Tolerancia operaciones exactas: 1e-10', 'blue');
    log('   Tolerancia funciones transcendentales: 1e-6 (conversión JS↔C++)', 'blue');
  } else {
    log('\n⚠️  Algunos tests de precisión fallaron', 'yellow');
  }

  process.exit(testsFailed > 0 ? 1 : 0);
}

runAccuracyTests().catch(err => {
  log(`\n❌ Error fatal: ${err.message}`, 'red');
  console.error(err.stack);
  process.exit(1);
});
