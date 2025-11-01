/**
 * Test exp(-abs(t)) con solución correcta
 * Aplica fftshift a la señal ANTES de calcular FFT
 */

import { Achronyme } from '../dist/sdk/index.js';

console.log('='.repeat(70));
console.log('SOLUCIÓN: exp(-abs(t)) con fftshift en la señal');
console.log('='.repeat(70));
console.log();

const ach = new Achronyme({ debug: false });
await ach.init();

// Usar N IMPAR para simetría perfecta
const N = 2001;
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

// Generar señal original
const t = ach.linspace(tStart, tEnd, N);
const signal = ach.map('t => exp(-abs(t))', t);

console.log('Método 1: SIN aplicar fftshift a la señal (INCORRECTO)');
console.log('-'.repeat(70));

// Calcular espectro directamente (como lo haces actualmente)
const spectrum1 = ach.fft_spectrum(signal, fs, true, true, 20);
const result1 = await spectrum1.toMatrix();

// Contar fases incorrectas
let badPhases1 = 0;
let negativeReal1 = 0;

for (const row of result1) {
  const omega = row[0];
  const mag = row[1];
  const phase = row[2];
  const absPhase = Math.abs(phase);

  // Para señal par real, fase debe ser 0 o ±π (muy cercano)
  if (absPhase > 0.2 && absPhase < (Math.PI - 0.2)) {
    badPhases1++;
  }

  // Verificar parte real
  const real = mag * Math.cos(phase);
  if (real < -0.001 && mag > 0.01) {
    negativeReal1++;
  }
}

console.log(`  Fases incorrectas: ${badPhases1}/${result1.length}`);
console.log(`  Partes reales negativas: ${negativeReal1}/${result1.length}`);
console.log();

// MÉTODO 2: Aplicar ifftshift a la señal ANTES de calcular FFT
console.log('Método 2: CON ifftshift en la señal (CORRECTO)');
console.log('-'.repeat(70));

// Obtener valores de la señal
const sigVals = await signal.toVector();

// Aplicar ifftshift manualmente: mover el centro al inicio
const mid = Math.floor(N / 2);
const shiftedSigVals = [
  ...sigVals.slice(mid),
  ...sigVals.slice(0, mid)
];

// Crear nueva señal shifteada
const signalShifted = ach.vector(shiftedSigVals);

// Ahora calcular FFT
// IMPORTANTE: NO aplicar fftshift al resultado (shift=false)
const fftResult = ach.fft(signalShifted);
const fftMat = await fftResult.toMatrix();

// Calcular magnitudes y fases manualmente
const N_fft = fftMat.length;
const omegaRange = 20;

// Generar frecuencias y aplicar fftshift
const frequencies = [];
for (let k = 0; k < N_fft; k++) {
  frequencies.push(k * fs / N_fft);
}

// Aplicar fftshift a frecuencias y FFT
const midFFT = Math.floor((N_fft + 1) / 2);
const freqShifted = [...frequencies.slice(midFFT), ...frequencies.slice(0, midFFT)];
const fftShifted = [...fftMat.slice(midFFT), ...fftMat.slice(0, midFFT)];

// Ajustar frecuencias a [-fs/2, fs/2] y convertir a rad/s
const result2 = [];
for (let i = 0; i < N_fft; i++) {
  let freq = freqShifted[i];
  if (freq > fs / 2) freq -= fs;

  const omega = freq * 2 * Math.PI;

  // Aplicar filtro de rango
  if (Math.abs(omega) > omegaRange) continue;

  const real = fftShifted[i][0];
  const imag = fftShifted[i][1];
  const mag = Math.sqrt(real * real + imag * imag);
  const phase = Math.atan2(imag, real);

  result2.push([omega, mag * dt, phase]); // Normalizar por dt
}

// Analizar resultado
let badPhases2 = 0;
let negativeReal2 = 0;

for (const row of result2) {
  const omega = row[0];
  const mag = row[1];
  const phase = row[2];
  const absPhase = Math.abs(phase);

  if (absPhase > 0.2 && absPhase < (Math.PI - 0.2)) {
    badPhases2++;
  }

  const real = mag * Math.cos(phase);
  if (real < -0.001 && mag > 0.01) {
    negativeReal2++;
  }
}

console.log(`  Fases incorrectas: ${badPhases2}/${result2.length}`);
console.log(`  Partes reales negativas: ${negativeReal2}/${result2.length}`);
console.log();

// Mostrar algunas fases del método 2
console.log('Muestra de fases (Método 2):');
console.log('ω (rad/s) | Magnitud | Fase');
console.log('-'.repeat(50));
for (let i = 0; i < Math.min(10, result2.length); i++) {
  const row = result2[i];
  console.log(`${row[0].toFixed(4).padStart(10)} | ${row[1].toFixed(4).padStart(8)} | ${row[2].toFixed(4).padStart(8)}`);
}

console.log();
console.log('='.repeat(70));
console.log('CONCLUSIÓN:');
if (badPhases2 < badPhases1 && negativeReal2 < negativeReal1) {
  console.log('✅ Aplicar ifftshift a la señal MEJORA los resultados!');
  console.log(`   Reducción de fases incorrectas: ${badPhases1} → ${badPhases2}`);
  console.log(`   Reducción de reales negativas: ${negativeReal1} → ${negativeReal2}`);
} else if (badPhases2 === 0 && negativeReal2 === 0) {
  console.log('✅ PERFECTO! Todos los valores son correctos.');
} else {
  console.log('⚠️  Aún hay problemas. El issue puede ser más profundo...');
}
console.log('='.repeat(70));

// Cleanup
t.dispose();
signal.dispose();
spectrum1.dispose();
signalShifted.dispose();
fftResult.dispose();
