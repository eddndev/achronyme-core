/**
 * Test específico para exp(-abs(t)) de -5 a 5
 * Compara resultado con transformada analítica
 */

import { Achronyme } from '../dist/sdk/index.js';

console.log('='.repeat(70));
console.log('TEST: exp(-abs(t)) de -5 a 5');
console.log('='.repeat(70));
console.log();

const ach = new Achronyme({ debug: false });
await ach.init();

// Configuración: Reproducir tu caso
const N = 2000;  // Tus 2000 puntos
const tStart = -5;
const tEnd = 5;
const dt = (tEnd - tStart) / (N - 1);
const fs = 1 / dt;

console.log('Configuración:');
console.log(`  N = ${N} puntos`);
console.log(`  t = [${tStart}, ${tEnd}]`);
console.log(`  dt = ${dt.toFixed(6)} s`);
console.log(`  fs = ${fs.toFixed(6)} Hz`);
console.log();

// Generar señal: exp(-abs(t))
console.log('Generando señal exp(-abs(t))...');
const t = ach.linspace(tStart, tEnd, N);
const signal = ach.map('t => exp(-abs(t))', t);

const sigVals = await signal.toVector();
console.log(`  Valores en t=0: ${sigVals[Math.floor(N/2)].toFixed(6)} (esperado: 1.0)`);
console.log(`  Valores en t=-5: ${sigVals[0].toFixed(6)} (esperado: ~0.0067)`);
console.log(`  Valores en t=+5: ${sigVals[N-1].toFixed(6)} (esperado: ~0.0067)`);
console.log();

// Verificar simetría
const mid = Math.floor(N / 2);
console.log('Verificando simetría:');
for (let offset = 1; offset <= 5; offset++) {
  const left = sigVals[mid - offset];
  const right = sigVals[mid + offset];
  const diff = Math.abs(left - right);
  console.log(`  offset ${offset}: left=${left.toFixed(6)}, right=${right.toFixed(6)}, diff=${diff.toExponential(2)}`);
}
console.log();

// Calcular espectro
console.log('Calculando espectro con fft_spectrum()...');
const omegaRange = 20; // rad/s
const spectrum = ach.fft_spectrum(signal, fs, true, true, omegaRange);
const result = await spectrum.toMatrix();

console.log(`  Puntos del espectro: ${result.length}`);
console.log(`  Rango ω: [${result[0][0].toFixed(4)}, ${result[result.length-1][0].toFixed(4)}] rad/s`);
console.log();

// Función analítica: F(ω) = 2/(1 + ω²)
function analyticalSpectrum(omega) {
  return 2.0 / (1.0 + omega * omega);
}

// Buscar valores cerca del centro (donde sabemos el resultado exacto)
console.log('Comparación con transformada analítica:');
console.log('ω (rad/s) | FFT Mag | Analítica | Error | FFT Phase | Esperado');
console.log('-'.repeat(70));

// Buscar los puntos más cercanos a algunas frecuencias clave
const testFreqs = [0, 0.5, 1, 2, 5];

for (const targetOmega of testFreqs) {
  // Buscar el punto más cercano a targetOmega
  let closestIdx = 0;
  let minDist = Infinity;

  for (let i = 0; i < result.length; i++) {
    const dist = Math.abs(result[i][0] - targetOmega);
    if (dist < minDist) {
      minDist = dist;
      closestIdx = i;
    }
  }

  const omega = result[closestIdx][0];
  const fftMag = result[closestIdx][1];
  const fftPhase = result[closestIdx][2];

  // Calcular valor analítico
  const analytical = analyticalSpectrum(omega);

  // El FFT da el espectro discreto, necesita normalización por dt
  // Para comparar con la transformada continua, multiplicamos por dt
  const fftMagNormalized = fftMag * dt;

  const error = Math.abs(fftMagNormalized - analytical);
  const errorPercent = (error / analytical * 100).toFixed(2);

  const expectedPhase = 0; // Para señal par real, fase = 0

  console.log(
    `${omega.toFixed(4).padStart(10)} | ` +
    `${fftMagNormalized.toFixed(4).padStart(7)} | ` +
    `${analytical.toFixed(4).padStart(9)} | ` +
    `${errorPercent.padStart(5)}% | ` +
    `${fftPhase.toFixed(4).padStart(9)} | ` +
    `${expectedPhase.toFixed(4).padStart(8)}`
  );
}

console.log();

// Analizar el espectro completo
console.log('Análisis del espectro completo:');

// Verificar si hay partes reales negativas
let hasNegativeReal = false;
let negativeCount = 0;
let totalCount = result.length;

// Para obtener parte real, necesitamos calcularla de mag y phase
for (let i = 0; i < result.length; i++) {
  const omega = result[i][0];
  const mag = result[i][1];
  const phase = result[i][2];

  // real = mag * cos(phase)
  const real = mag * Math.cos(phase);

  if (real < -0.001) { // Pequeña tolerancia para errores numéricos
    hasNegativeReal = true;
    negativeCount++;

    if (negativeCount <= 5) { // Mostrar solo los primeros 5
      console.log(`  ⚠️  ω=${omega.toFixed(4)}: mag=${mag.toFixed(4)}, phase=${phase.toFixed(4)}, real=${real.toFixed(4)} (NEGATIVO!)`);
    }
  }
}

console.log();
if (hasNegativeReal) {
  console.log(`❌ ERROR: ${negativeCount}/${totalCount} puntos tienen parte real negativa`);
  console.log('   Para exp(-abs(t)), la transformada debería ser COMPLETAMENTE POSITIVA');
  console.log('   Esto indica un problema en el cálculo del espectro');
} else {
  console.log('✓ Todas las partes reales son positivas (correcto)');
}

console.log();

// Verificar simetría del espectro
console.log('Verificando simetría del espectro:');
const centerIdx = Math.floor(result.length / 2);

let isSymmetric = true;
for (let offset = 1; offset <= Math.min(5, centerIdx); offset++) {
  const leftIdx = centerIdx - offset;
  const rightIdx = centerIdx + offset;

  if (rightIdx >= result.length) break;

  const leftMag = result[leftIdx][1];
  const rightMag = result[rightIdx][1];
  const diffMag = Math.abs(leftMag - rightMag);

  const leftOmega = result[leftIdx][0];
  const rightOmega = result[rightIdx][0];

  console.log(`  ω=${leftOmega.toFixed(2)} vs ω=${rightOmega.toFixed(2)}: ` +
              `mag=${leftMag.toFixed(4)} vs ${rightMag.toFixed(4)}, ` +
              `diff=${diffMag.toExponential(2)}`);

  if (diffMag > 0.01) {
    isSymmetric = false;
  }
}

console.log();
if (isSymmetric) {
  console.log('✓ El espectro es simétrico (correcto para señal par)');
} else {
  console.log('❌ El espectro NO es simétrico (problema!)');
}

console.log();
console.log('='.repeat(70));
console.log('CONCLUSIÓN:');
console.log('Para exp(-abs(t)):');
console.log('  • Transformada analítica: F(ω) = 2/(1 + ω²)');
console.log('  • Debe ser REAL, PAR y POSITIVA');
console.log('  • Fase debe ser 0 en todas las frecuencias');
console.log('='.repeat(70));

// Cleanup
t.dispose();
signal.dispose();
spectrum.dispose();