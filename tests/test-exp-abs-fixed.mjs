/**
 * Test exp(-abs(t)) con N IMPAR para simetría perfecta
 */

import { Achronyme } from '../dist/sdk/index.js';

console.log('='.repeat(70));
console.log('TEST: exp(-abs(t)) con N IMPAR (simétrica)');
console.log('='.repeat(70));
console.log();

const ach = new Achronyme({ debug: false });
await ach.init();

// Usar N IMPAR para incluir t=0 exactamente
const N = 2001;  // ← IMPAR!
const tStart = -5;
const tEnd = 5;
const dt = (tEnd - tStart) / (N - 1);
const fs = 1 / dt;

console.log('Configuración:');
console.log(`  N = ${N} puntos (IMPAR)`);
console.log(`  t = [${tStart}, ${tEnd}]`);
console.log(`  dt = ${dt.toFixed(6)} s`);
console.log(`  fs = ${fs.toFixed(6)} Hz`);
console.log();

// Generar señal
const t = ach.linspace(tStart, tEnd, N);
const tVals = await t.toVector();
const signal = ach.map('t => exp(-abs(t))', t);
const sigVals = await signal.toVector();

// Verificar que t=0 está en el centro
const mid = Math.floor(N / 2);
console.log(`Punto central (t=0):`);
console.log(`  índice: ${mid}`);
console.log(`  t[${mid}] = ${tVals[mid].toFixed(6)} (esperado: 0.0)`);
console.log(`  signal[${mid}] = ${sigVals[mid].toFixed(6)} (esperado: 1.0)`);
console.log();

// Verificar simetría
console.log('Verificando simetría (debe ser perfecta):');
let maxDiff = 0;
for (let offset = 1; offset <= 5; offset++) {
  const left = sigVals[mid - offset];
  const right = sigVals[mid + offset];
  const diff = Math.abs(left - right);
  maxDiff = Math.max(maxDiff, diff);
  console.log(`  offset ${offset}: left=${left.toFixed(6)}, right=${right.toFixed(6)}, diff=${diff.toExponential(2)}`);
}
console.log();

if (maxDiff < 1e-10) {
  console.log('✓ Señal PERFECTAMENTE simétrica!\n');
} else {
  console.log(`⚠️  Señal tiene asimetría máxima de ${maxDiff.toExponential(2)}\n`);
}

// Calcular espectro
const omegaRange = 20;
const spectrum = ach.fft_spectrum(signal, fs, true, true, omegaRange);
const result = await spectrum.toMatrix();

console.log(`Espectro calculado: ${result.length} puntos`);
console.log();

// Analizar fases
console.log('Análisis de fases (deberían ser ~0):');
console.log('ω (rad/s) | Magnitud | Fase | |Fase| < 0.1?');
console.log('-'.repeat(60));

let correctPhaseCount = 0;
const centralPoints = result.filter(r => Math.abs(r[0]) < 5); // Solo ω cercanas a 0

for (const row of centralPoints.slice(0, 10)) {
  const omega = row[0];
  const mag = row[1];
  const phase = row[2];
  const absPhase = Math.abs(phase);
  const isCorrect = absPhase < 0.1 || absPhase > (Math.PI - 0.1);

  console.log(
    `${omega.toFixed(4).padStart(10)} | ` +
    `${mag.toFixed(4).padStart(8)} | ` +
    `${phase.toFixed(4).padStart(8)} | ` +
    `${isCorrect ? '✓' : '✗'}`
  );

  if (isCorrect) correctPhaseCount++;
}

console.log();
console.log(`Fases correctas: ${correctPhaseCount}/${centralPoints.length}`);
console.log();

// Verificar partes reales
let negativeCount = 0;
for (const row of result) {
  const mag = row[1];
  const phase = row[2];
  const real = mag * Math.cos(phase);

  if (real < -0.001 && mag > 0.01) { // Ignorar magnitudes muy pequeñas
    negativeCount++;
  }
}

if (negativeCount === 0) {
  console.log('✓ TODAS las partes reales son positivas (correcto!)');
} else {
  console.log(`✗ ${negativeCount}/${result.length} puntos con parte real negativa`);
}

console.log();
console.log('='.repeat(70));
console.log('RESULTADO:');
if (maxDiff < 1e-10 && negativeCount === 0 && correctPhaseCount >= centralPoints.length * 0.8) {
  console.log('✅ TODO CORRECTO: Usar N IMPAR resuelve el problema!');
} else {
  console.log('⚠️  Aún hay problemas. Investigar más...');
}
console.log('='.repeat(70));

// Cleanup
t.dispose();
signal.dispose();
spectrum.dispose();