// tests/stress.js
export const tests = {
  'stress-memory': async (ach) => {
    const count = 50000;
    const startMem = ach.getActiveValuesCount();
    const start = performance.now();
    for (let i = 0; i < count; i++) {
      await ach.use(async () => {
        const v = ach.vector([1, 2, 3, 4, 5]);
        ach.sum(v);
      });
    }
    const duration = performance.now() - start;
    const endMem = ach.getActiveValuesCount();
    const leaked = endMem - startMem;
    return `Created/disposed ${count} vectors in ${duration.toFixed(2)}ms
` +
           `Rate: ${(count/duration*1000).toFixed(0)} vectors/sec
` +
           `Memory leaks: ${leaked} handles`;
  },
  'stress-computation': async (ach) => {
    const size = 100000;
    const iterations = 50;
    const start = performance.now();
    let output = '';
    await ach.use(async () => {
      const data = Array.from({ length: size }, (_, i) => (i % 100) / 10 + 0.1);
      const v = ach.vector(data);
      const ops = [];
      const sinStart = performance.now();
      for (let i = 0; i < iterations; i++) { const r = ach.sin(v); }
      ops.push({ name: 'sin', time: performance.now() - sinStart });
      const cosStart = performance.now();
      for (let i = 0; i < iterations; i++) { const r = ach.cos(v); }
      ops.push({ name: 'cos', time: performance.now() - cosStart });
      const expStart = performance.now();
      for (let i = 0; i < iterations; i++) { const r = ach.exp(v); }
      ops.push({ name: 'exp', time: performance.now() - expStart });
      const sqrtStart = performance.now();
      for (let i = 0; i < iterations; i++) { const r = ach.sqrt(v); }
      ops.push({ name: 'sqrt', time: performance.now() - sqrtStart });
      const totalOps = size * iterations * ops.length;
      const totalTime = ops.reduce((sum, op) => sum + op.time, 0);
      output = `EXTREME VECTORIZED STRESS
`;
      output += `${size.toLocaleString()} elements × ${iterations} iterations × 4 ops
`;
      output += `Total ops: ${totalOps.toLocaleString()}
`;
      output += `Time: ${totalTime.toFixed(2)}ms
`;
      output += `Throughput: ${(totalOps / totalTime * 1000).toLocaleString()} ops/sec

`;
      output += ops.map(op =>
        `${op.name}: ${op.time.toFixed(2)}ms (${(size * iterations / op.time * 1000).toLocaleString()} ops/sec)`
      ).join('\n');
    });
    return output;
  },
  'stress-gc': async (ach) => {
    const N = 32768;
    const start = performance.now();
    let output = '';
    await ach.use(async () => {
      const freq1 = 50, freq2 = 120, freq3 = 200;
      const sampleRate = 1000;
      const omega1 = 2 * Math.PI * freq1 / sampleRate;
      const omega2 = 2 * Math.PI * freq2 / sampleRate;
      const omega3 = 2 * Math.PI * freq3 / sampleRate;
      const signalData = new Array(N);
      for (let i = 0; i < N; i++) {
        signalData[i] = Math.sin(omega1 * i) +
                        0.5 * Math.sin(omega2 * i) +
                        0.3 * Math.sin(omega3 * i);
      }
      const step1 = performance.now();
      const signal = ach.vector(signalData);
      const t1 = performance.now() - step1;
      const step2 = performance.now();
      const window = ach.dsp.hanning(N);
      const windowed = ach.vecOps.vmul(signal, window);
      const t2 = performance.now() - step2;
      const step3 = performance.now();
      const spectrum = ach.dsp.fftMag(windowed);
      const t3 = performance.now() - step3;
      const step4 = performance.now();
      const sum = ach.sum(spectrum);
      const mean = ach.mean(spectrum);
      const max = ach.max(spectrum);
      const std = ach.std(spectrum);
      const t4 = performance.now() - step4;
      const total = performance.now() - start;
      output = `DSP PIPELINE STRESS (${N.toLocaleString()} samples)

`;
      output += `Signal Generation: ${t1.toFixed(2)}ms
`;
      output += `Windowing: ${t2.toFixed(2)}ms
`;
      output += `FFT: ${t3.toFixed(2)}ms
`;
      output += `Statistics: ${t4.toFixed(2)}ms
`;
      output += `─────────────────────
`;
      output += `Total: ${total.toFixed(2)}ms
`;
      output += `Throughput: ${(N / total * 1000).toLocaleString()} samples/sec`;
    });
    return output;
  },
};
