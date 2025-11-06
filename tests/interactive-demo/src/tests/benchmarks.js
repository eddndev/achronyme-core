// tests/benchmarks.js
import * as math from 'mathjs';

export const tests = {
  'bench-vs-mathjs': async (ach) => {
    const size = 10000000;
    const iterations = 5;
    const data = Array.from({length: size}, (_, i) => (i % 100) / 10 + 0.1);
    let output = `VECTORIZED MATH BENCHMARK (${size.toLocaleString()} elements × ${iterations} iterations)

`;

    const achCreateStart = performance.now();
    let achSin, achCos, achExp;
    await ach.use(async () => {
      const v = ach.vector(data);
      const achCreateTime = performance.now() - achCreateStart;
      const achSinStart = performance.now();
      for (let i = 0; i < iterations; i++) { achSin = ach.sin(v); }
      const achSinTime = performance.now() - achSinStart;
      const achCosStart = performance.now();
      for (let i = 0; i < iterations; i++) { achCos = ach.cos(v); }
      const achCosTime = performance.now() - achCosStart;
      const achExpStart = performance.now();
      for (let i = 0; i < iterations; i++) { achExp = ach.exp(v); }
      const achExpTime = performance.now() - achExpStart;
      output += `🔷 Achronyme (WASM)
`;
      output += `   Create vector: ${achCreateTime.toFixed(2)}ms
`;
      output += `   sin(): ${achSinTime.toFixed(2)}ms (${(size * iterations / achSinTime * 1000).toLocaleString()} ops/sec)
`;
      output += `   cos(): ${achCosTime.toFixed(2)}ms (${(size * iterations / achCosTime * 1000).toLocaleString()} ops/sec)
`;
      output += `   exp(): ${achExpTime.toFixed(2)}ms (${(size * iterations / achExpTime * 1000).toLocaleString()} ops/sec)
`;
      output += `   TOTAL: ${(achCreateTime + achSinTime + achCosTime + achExpTime).toFixed(2)}ms

`;
    });

    const jsStart = performance.now();
    const jsSinStart = performance.now();
    for (let i = 0; i < iterations; i++) { const jsSin = data.map(x => Math.sin(x)); }
    const jsSinTime = performance.now() - jsSinStart;
    const jsCosStart = performance.now();
    for (let i = 0; i < iterations; i++) { const jsCos = data.map(x => Math.cos(x)); }
    const jsCosTime = performance.now() - jsCosStart;
    const jsExpStart = performance.now();
    for (let i = 0; i < iterations; i++) { const jsExp = data.map(x => Math.exp(x)); }
    const jsExpTime = performance.now() - jsExpStart;
    const jsTotal = jsSinTime + jsCosTime + jsExpTime;
    output += `🟨 JavaScript Native (V8)
`;
    output += `   sin(): ${jsSinTime.toFixed(2)}ms (${(size * iterations / jsSinTime * 1000).toLocaleString()} ops/sec)
`;
    output += `   cos(): ${jsCosTime.toFixed(2)}ms (${(size * iterations / jsCosTime * 1000).toLocaleString()} ops/sec)
`;
    output += `   exp(): ${jsExpTime.toFixed(2)}ms (${(size * iterations / jsExpTime * 1000).toLocaleString()} ops/sec)
`;
    output += `   TOTAL: ${jsTotal.toFixed(2)}ms

`;

    const achTotalMatch = output.match(/Achronyme[\s\S]*?TOTAL: ([\d.]+)ms/);
    const achTotal = achTotalMatch ? parseFloat(achTotalMatch[1]) : 0;
    output += `━━━━━━━━━━━━━━━━━━━━━
`;
    const speedup = (jsTotal / achTotal).toFixed(2);
    output += `📊 PERFORMANCE:
`;
    output += `   Achronyme: ${achTotal.toFixed(2)}ms
`;
    output += `   JS Native: ${jsTotal.toFixed(2)}ms
`;
    output += `   Speedup: ${speedup}x ${achTotal < jsTotal ? '🚀 WASM WINS!' : 'JS wins'}
`;
    output += `   Advantage: ${Math.abs(((jsTotal - achTotal) / jsTotal * 100)).toFixed(1)}%`;
    return output;
  },
  'bench-fft': async (ach) => {
    const size = 8192;
    const iterations = 10;
    const signal = Array.from({length: size}, (_, i) => Math.sin(2 * Math.PI * 50 * i / 1000) + 0.5 * Math.sin(2 * Math.PI * 120 * i / 1000));
    let output = `FFT BENCHMARK (${size.toLocaleString()} samples × ${iterations} iterations)

`;
    const achStart = performance.now();
    await ach.use(async () => {
      const v = ach.vector(signal);
      for (let i = 0; i < iterations; i++) { const spectrum = ach.fftMag(v); }
    });
    const achTime = performance.now() - achStart;
    const mathStart = performance.now();
    for (let i = 0; i < iterations; i++) {
      const fft = math.fft(signal);
      const mag = fft.map(c => (typeof c === 'number') ? Math.abs(c) : Math.sqrt(c.re * c.re + c.im * c.im));
    }
    const mathTime = performance.now() - mathStart;
    const speedup = (mathTime / achTime).toFixed(2);
    output += `🔷 Achronyme: ${achTime.toFixed(2)}ms (${(achTime/iterations).toFixed(2)}ms per FFT)
`;
    output += `🟦 math.js:   ${mathTime.toFixed(2)}ms (${(mathTime/iterations).toFixed(2)}ms per FFT)

`;
    output += `Speedup: ${speedup}x ${achTime < mathTime ? 'faster' : 'slower'}
`;
    output += `Winner: ${achTime < mathTime ? '🔷 Achronyme' : '🟦 math.js'}`;
    return output;
  },
  'bench-vector-ops': async (ach) => {
    const size = 200000;
    const iterations = 30;
    const data1 = Array.from({length: size}, () => Math.random() * 100);
    const data2 = Array.from({length: size}, () => Math.random() * 100);
    let output = `VECTOR OPERATIONS (${size.toLocaleString()} elements × ${iterations} iterations)

`;
    const achStart = performance.now();
    await ach.use(async () => {
      const v1 = ach.vector(data1);
      const v2 = ach.vector(data2);
      for (let i = 0; i < iterations; i++) {
        const add = ach.vecOps.vadd(v1, v2);
        const mul = ach.vecOps.vmul(v1, v2);
        const dot = ach.dot(v1, v2);
      }
    });
    const achTime = performance.now() - achStart;
    const mathStart = performance.now();
    for (let i = 0; i < iterations; i++) {
      const add = math.add(data1, data2);
      const mul = math.dotMultiply(data1, data2);
      const dot = math.dot(data1, data2);
    }
    const mathTime = performance.now() - mathStart;
    const jsStart = performance.now();
    for (let i = 0; i < iterations; i++) {
      const add = data1.map((v, i) => v + data2[i]);
      const mul = data1.map((v, i) => v * data2[i]);
      const dot = data1.reduce((sum, v, i) => sum + v * data2[i], 0);
    }
    const jsTime = performance.now() - jsStart;
    const totalOps = size * iterations * 3;
    const speedupMath = (mathTime / achTime).toFixed(2);
    const speedupJS = (jsTime / achTime).toFixed(2);
    output += `🔷 Achronyme: ${achTime.toFixed(2)}ms (${(totalOps/achTime*1000).toLocaleString()} ops/sec)
`;
    output += `🟦 math.js:   ${mathTime.toFixed(2)}ms (${(totalOps/mathTime*1000).toLocaleString()} ops/sec)
`;
    output += `🟨 JS Native: ${jsTime.toFixed(2)}ms (${(totalOps/jsTime*1000).toLocaleString()} ops/sec)

`;
    output += `vs math.js: ${speedupMath}x ${achTime < mathTime ? 'faster' : 'slower'}
`;
    output += `vs JS V8:   ${speedupJS}x ${achTime < jsTime ? 'faster' : 'slower'}

`;
    const winner = achTime < mathTime && achTime < jsTime ? '🔷 Achronyme' : mathTime < jsTime ? '🟦 math.js' : '🟨 JS Native';
    output += `Winner: ${winner}`;
    return output;
  },
  'bench-pipeline': async (ach) => {
    const size = 16384;
    const signal = Array.from({length: size}, (_, i) => Math.sin(2 * Math.PI * 50 * i / 1000) + 0.5 * Math.sin(2 * Math.PI * 120 * i / 1000) + 0.3 * Math.sin(2 * Math.PI * 200 * i / 1000));
    let output = `FULL DSP PIPELINE (${size.toLocaleString()} samples)

`;
    const achStart = performance.now();
    await ach.use(async () => {
      const v = ach.vector(signal);
      const window = ach.dsp.hanning(size);
      const windowed = ach.vecOps.vmul(v, window);
      const spectrum = ach.fftMag(windowed);
      const max = ach.max(spectrum);
      const mean = ach.mean(spectrum);
      const sum = ach.sum(spectrum);
    });
    const achTime = performance.now() - achStart;
    const mathStart = performance.now();
    const mathWindow = new Array(size);
    for (let i = 0; i < size; i++) { mathWindow[i] = 0.5 * (1 - Math.cos(2 * Math.PI * i / (size - 1))); }
    const mathWindowed = math.dotMultiply(signal, mathWindow);
    const mathFFT = math.fft(mathWindowed);
    const mathSpectrum = mathFFT.map(c => (typeof c === 'number') ? Math.abs(c) : Math.sqrt(c.re * c.re + c.im * c.im));
    const mathMax = math.max(mathSpectrum);
    const mathMean = math.mean(mathSpectrum);
    const mathSum = math.sum(mathSpectrum);
    const mathTime = performance.now() - mathStart;
    const speedup = (mathTime / achTime).toFixed(2);
    output += `Pipeline: Signal → Window → FFT → Statistics

`;
    output += `🔷 Achronyme: ${achTime.toFixed(2)}ms (${(size/achTime*1000).toLocaleString()} samples/sec)
`;
    output += `🟦 math.js:   ${mathTime.toFixed(2)}ms (${(size/mathTime*1000).toLocaleString()} samples/sec)

`;
    output += `Speedup: ${speedup}x ${achTime < mathTime ? 'faster' : 'slower'}
`;
    output += `Winner: ${achTime < mathTime ? '🔷 Achronyme' : '🟦 math.js'}`;
    return output;
  },
};
