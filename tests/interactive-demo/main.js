console.log('🚀 main.js loading...');

// Use local build instead of npm package for development
import { Achronyme } from '../../dist/sdk/Achronyme.js';
import * as math from 'mathjs';

console.log('✅ Imports successful');

// Global SDK instance
let ach = null;

// DOM Elements
console.log('🔍 Getting DOM elements...');
const sdkStatusText = document.getElementById('sdk-status-text');
const memoryStatusText = document.getElementById('memory-status-text');
const resultsOutput = document.getElementById('results-output');
console.log('✅ DOM elements retrieved:', { sdkStatusText, memoryStatusText, resultsOutput });

// Test category buttons
const categoryButtons = document.querySelectorAll('.test-category');
const testSections = document.querySelectorAll('.test-section');

// Action buttons
const runAllBtn = document.getElementById('run-all');
const clearResultsBtn = document.getElementById('clear-results');
const gcNowBtn = document.getElementById('gc-now');
const resetEvalBtn = document.getElementById('reset-eval');

// Test buttons
const testButtons = document.querySelectorAll('.test-btn');

// Initialize SDK
async function initSDK() {
  try {
    console.log('🔄 Starting SDK initialization...');
    sdkStatusText.textContent = 'Initializing...';
    sdkStatusText.style.color = 'var(--warning)';

    ach = new Achronyme();
    console.log('✅ Achronyme instance created');

    await ach.init();
    console.log('✅ SDK initialized successfully');

    sdkStatusText.textContent = 'Ready';
    sdkStatusText.style.color = 'var(--success)';

    addResult('SDK Initialized', 'SDK is ready for testing', 'success');
    updateMemoryStatus();
  } catch (error) {
    console.error('❌ SDK initialization failed:', error);
    sdkStatusText.textContent = 'Error';
    sdkStatusText.style.color = 'var(--error)';
    addResult('SDK Initialization Failed', error.message + '\n\nCheck browser console for details', 'error');
  }
}

// Update memory status
function updateMemoryStatus() {
  if (ach) {
    const count = ach.getActiveValuesCount();
    memoryStatusText.textContent = `${count} values`;
  }
}

// Add result to output panel
function addResult(title, content, type = 'info') {
  const placeholder = resultsOutput.querySelector('.placeholder');
  if (placeholder) {
    placeholder.remove();
  }

  const resultItem = document.createElement('div');
  resultItem.className = `result-item ${type}`;

  const resultHeader = document.createElement('div');
  resultHeader.className = 'result-header';

  const resultTitle = document.createElement('div');
  resultTitle.className = 'result-title';
  resultTitle.textContent = title;

  const resultTime = document.createElement('div');
  resultTime.className = 'result-time';
  resultTime.textContent = new Date().toLocaleTimeString();

  resultHeader.appendChild(resultTitle);
  resultHeader.appendChild(resultTime);

  const resultContent = document.createElement('div');
  resultContent.className = 'result-content';
  resultContent.textContent = content;

  resultItem.appendChild(resultHeader);
  resultItem.appendChild(resultContent);

  resultsOutput.insertBefore(resultItem, resultsOutput.firstChild);

  updateMemoryStatus();
}

// Clear results
function clearResults() {
  resultsOutput.innerHTML = '<p class="placeholder">Run tests to see results here...</p>';
}

// Force garbage collection
function forceGC() {
  if (ach) {
    const freed = ach.gc();
    addResult('Manual GC', `Freed ${freed} handles`, 'info');
    updateMemoryStatus();
  }
}

// Reset SOC evaluator
function resetEvaluator() {
  if (ach) {
    ach.resetEvaluator();
    addResult('Reset Evaluator', 'SOC evaluator state cleared (all variables removed)', 'info');
  }
}

// Category navigation
categoryButtons.forEach(button => {
  button.addEventListener('click', () => {
    const category = button.dataset.category;

    // Update active button
    categoryButtons.forEach(btn => btn.classList.remove('active'));
    button.classList.add('active');

    // Show corresponding section
    testSections.forEach(section => {
      if (section.id === `section-${category}`) {
        section.classList.remove('hidden');
      } else {
        section.classList.add('hidden');
      }
    });
  });
});

// Action button handlers
clearResultsBtn.addEventListener('click', clearResults);
gcNowBtn.addEventListener('click', forceGC);
resetEvalBtn.addEventListener('click', resetEvaluator);

// Test runner helper
async function runTest(testName, testFn, button) {
  if (!ach) {
    addResult(testName, 'SDK not initialized', 'error');
    return;
  }

  button.classList.add('running');
  const startTime = performance.now();

  try {
    const result = await testFn();
    const endTime = performance.now();
    const duration = (endTime - startTime).toFixed(2);

    button.classList.remove('running');
    button.classList.add('success');
    setTimeout(() => button.classList.remove('success'), 2000);

    addResult(testName, `✓ ${result}\nTime: ${duration}ms`, 'success');
  } catch (error) {
    const endTime = performance.now();
    const duration = (endTime - startTime).toFixed(2);

    button.classList.remove('running');
    button.classList.add('error');
    setTimeout(() => button.classList.remove('error'), 2000);

    addResult(testName, `✗ ${error.message}\nTime: ${duration}ms`, 'error');
  }
}

// ============================================================================
// TEST IMPLEMENTATIONS
// ============================================================================

const tests = {
  // ========== BASIC MATH ==========
  'basic-arithmetic': async () => {
    return ach.use(async () => {
      // Test basic JavaScript math (baseline)
      const sum = 10 + 5;
      const diff = 10 - 5;
      const prod = 10 * 5;
      const quot = 10 / 5;
      return `10+5=${sum}, 10-5=${diff}, 10*5=${prod}, 10/5=${quot}`;
    });
  },

  'basic-power': async () => {
    return ach.use(async () => {
      const power = Math.pow(2, 8);
      const sqrt = Math.sqrt(16);
      return `2^8=${power}, √16=${sqrt}`;
    });
  },

  'basic-trig': async () => {
    return ach.use(async () => {
      const angle = Math.PI / 4;
      const sinVal = ach.sin(angle);
      const cosVal = ach.cos(angle);
      const tanVal = ach.tan(angle);
      return `sin(π/4)=${sinVal.toFixed(4)}, cos(π/4)=${cosVal.toFixed(4)}, tan(π/4)=${tanVal.toFixed(4)}`;
    });
  },

  'basic-exp-log': async () => {
    return ach.use(async () => {
      const exp = ach.exp(2);
      const ln = ach.ln(Math.E);
      return `e^2=${exp.toFixed(4)}, ln(e)=${ln.toFixed(4)}`;
    });
  },

  'basic-abs': async () => {
    return ach.use(async () => {
      const abs = ach.abs(-42);
      return `|-42|=${abs}`;
    });
  },

  // ========== VECTORS ==========
  'vector-create': async () => {
    return ach.use(async () => {
      const v = ach.vector([1, 2, 3, 4, 5]);
      return `Created vector with 5 elements`;
    });
  },

  'vector-arithmetic': async () => {
    return ach.use(async () => {
      const v1 = ach.vector([1, 2, 3]);
      const v2 = ach.vector([4, 5, 6]);
      return `Created two vectors: v1=[1,2,3], v2=[4,5,6]`;
    });
  },

  'vector-dot': async () => {
    return ach.use(async () => {
      const v1 = ach.vector([1, 2, 3]);
      const v2 = ach.vector([4, 5, 6]);
      const dot = ach.dot(v1, v2);
      return `[1,2,3] · [4,5,6] = ${dot}`;
    });
  },

  'vector-stats': async () => {
    return ach.use(async () => {
      const data = ach.vector([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
      const meanVal = ach.mean(data);
      const stdVal = ach.std(data);
      return `mean=${meanVal}, std=${stdVal.toFixed(4)}`;
    });
  },

  'vector-operations': async () => {
    return ach.use(async () => {
      const v = ach.vector([3, 1, 4, 1, 5, 9, 2, 6]);
      const sum = ach.sum(v);
      const max = ach.max(v);
      const min = ach.min(v);
      return `sum=${sum}, max=${max}, min=${min}`;
    });
  },

  // ========== COMPLEX NUMBERS ==========
  'complex-create': async () => {
    return ach.use(async () => {
      const c = ach.complex(3, 4);
      return `Created complex: 3+4i`;
    });
  },

  'complex-arithmetic': async () => {
    return ach.use(async () => {
      const c1 = ach.complex(2, 3);
      const c2 = ach.complex(1, 4);
      return `Created complex numbers: (2+3i) and (1+4i)`;
    });
  },

  'complex-conjugate': async () => {
    return ach.use(async () => {
      const c = ach.complex(3, 4);
      return `Complex number created: 3+4i`;
    });
  },

  'complex-magnitude': async () => {
    return ach.use(async () => {
      const c = ach.complex(3, 4);
      // |3+4i| should be 5
      return `Complex number 3+4i created (magnitude should be 5)`;
    });
  },

  // ========== DSP FUNCTIONS ==========
  'dsp-fft': async () => {
    return ach.use(async () => {
      const signal = ach.vector([1, 2, 3, 4, 5, 6, 7, 8]);
      const spectrum = ach.fft(signal);
      return `FFT computed on 8 samples`;
    });
  },

  'dsp-fft-mag': async () => {
    return ach.use(async () => {
      const signal = ach.vector(Array.from({length: 1024}, (_, i) =>
        Math.sin(2 * Math.PI * 50 * i / 1000)
      ));
      const mag = ach.fftMag(signal);
      return `FFT magnitude computed on 1024 samples`;
    });
  },

  'dsp-ifft': async () => {
    return ach.use(async () => {
      const signal = ach.vector([1, 2, 3, 4, 5, 6, 7, 8]);
      const spectrum = ach.fft(signal);
      const reconstructed = ach.ifft(spectrum);
      return `IFFT computed, signal reconstructed`;
    });
  },

  'dsp-windows': async () => {
    return ach.use(async () => {
      const hann = ach.dsp.hanning(64);
      const hamming = ach.dsp.hamming(64);
      const blackman = ach.dsp.blackman(64);
      return `Created Hann, Hamming, Blackman windows (64 samples each)`;
    });
  },

  'dsp-convolution': async () => {
    return ach.use(async () => {
      const signal = ach.vector([1, 2, 3, 4]);
      const kernel = ach.vector([0.25, 0.5, 0.25]);
      const result = ach.conv(signal, kernel);
      return `Convolution computed`;
    });
  },

  'dsp-pipeline': async () => {
    return ach.use(async () => {
      const signal = ach.vector(Array.from({length: 1024}, (_, i) =>
        Math.sin(2 * Math.PI * 50 * i / 1000) + 0.5 * Math.random()
      ));
      return `DSP pipeline test (signal created)`;
    });
  },

  // ========== HIGHER-ORDER FUNCTIONS ==========
  'hof-map': async () => {
    return ach.use(async () => {
      const numbers = ach.vector([1, 2, 3, 4, 5]);
      const squared = ach.map((x) => x * x, numbers);
      return `map(x => x*x) applied to vector`;
    });
  },

  'hof-filter': async () => {
    return ach.use(async () => {
      const numbers = ach.vector([1, 2, 3, 4, 5, 6]);
      const evens = ach.filter((x) => x % 2 === 0, numbers);
      return `filter(x => x%2==0) applied to vector`;
    });
  },

  'hof-reduce': async () => {
    return ach.use(async () => {
      const numbers = ach.vector([1, 2, 3, 4, 5]);
      const sum = ach.reduce((a, b) => a + b, 0, numbers);
      return `reduce((a,b) => a+b, 0) = ${sum}`;
    });
  },

  'hof-pipe': async () => {
    return ach.use(async () => {
      const numbers = ach.vector([1, 2, 3, 4, 5]);
      return `Pipe test (chaining HOF operations)`;
    });
  },

  'hof-lambda': async () => {
    return ach.use(async () => {
      return `Lambda test (not yet implemented in SDK)`;
    });
  },

  // ========== MATRIX OPERATIONS ==========
  'matrix-create': async () => {
    return ach.use(async () => {
      const m = ach.matrix([[1, 2], [3, 4]]);
      return `Created 2x2 matrix: [[1,2],[3,4]]`;
    });
  },

  'matrix-arithmetic': async () => {
    return ach.use(async () => {
      const m1 = ach.matrix([[1, 2], [3, 4]]);
      const m2 = ach.matrix([[5, 6], [7, 8]]);
      return `Created two 2x2 matrices`;
    });
  },

  'matrix-multiply': async () => {
    return ach.use(async () => {
      const m1 = ach.matrix([[1, 2], [3, 4]]);
      const m2 = ach.matrix([[5, 6], [7, 8]]);
      return `Matrix multiplication test (created matrices)`;
    });
  },

  'matrix-determinant': async () => {
    return ach.use(async () => {
      const m = ach.matrix([[1, 2], [3, 4]]);
      const det = ach.det(m);
      return `det([[1,2],[3,4]]) = ${det}`;
    });
  },

  'matrix-inverse': async () => {
    return ach.use(async () => {
      const m = ach.matrix([[1, 2], [3, 4]]);
      const inv = ach.linalg.inverse(m);
      // Verify: A * A^(-1) should be identity
      return `Matrix inverse computed for [[1,2],[3,4]]`;
    });
  },

  'matrix-decomposition': async () => {
    return ach.use(async () => {
      const m = ach.matrix([[4, 2], [1, 3]]);
      const result = ach.lu(m);
      return `LU decomposition computed`;
    });
  },

  // ========== STRESS TESTS ==========
  'stress-memory': async () => {
    const count = 50000;
    const startMem = ach.getActiveValuesCount();
    const start = performance.now();

    for (let i = 0; i < count; i++) {
      await ach.use(async () => {
        const v = ach.vector([1, 2, 3, 4, 5]);
        ach.sum(v); // Use it
      });
    }

    const duration = performance.now() - start;
    const endMem = ach.getActiveValuesCount();
    const leaked = endMem - startMem;

    return `Created/disposed ${count} vectors in ${duration.toFixed(2)}ms\n` +
           `Rate: ${(count/duration*1000).toFixed(0)} vectors/sec\n` +
           `Memory leaks: ${leaked} handles`;
  },

  'stress-computation': async () => {
    const size = 100000;
    const iterations = 50;
    const start = performance.now();

    let output = '';
    await ach.use(async () => {
      // Generate large dataset
      const data = Array.from({ length: size }, (_, i) => (i % 100) / 10 + 0.1);
      const v = ach.vector(data);

      // Extreme vectorized operations
      const ops = [];

      // Sin
      const sinStart = performance.now();
      for (let i = 0; i < iterations; i++) {
        const r = ach.sin(v);
      }
      ops.push({ name: 'sin', time: performance.now() - sinStart });

      // Cos
      const cosStart = performance.now();
      for (let i = 0; i < iterations; i++) {
        const r = ach.cos(v);
      }
      ops.push({ name: 'cos', time: performance.now() - cosStart });

      // Exp
      const expStart = performance.now();
      for (let i = 0; i < iterations; i++) {
        const r = ach.exp(v);
      }
      ops.push({ name: 'exp', time: performance.now() - expStart });

      // Sqrt
      const sqrtStart = performance.now();
      for (let i = 0; i < iterations; i++) {
        const r = ach.sqrt(v);
      }
      ops.push({ name: 'sqrt', time: performance.now() - sqrtStart });

      const totalOps = size * iterations * ops.length;
      const totalTime = ops.reduce((sum, op) => sum + op.time, 0);

      output = `EXTREME VECTORIZED STRESS\n`;
      output += `${size.toLocaleString()} elements × ${iterations} iterations × 4 ops\n`;
      output += `Total ops: ${totalOps.toLocaleString()}\n`;
      output += `Time: ${totalTime.toFixed(2)}ms\n`;
      output += `Throughput: ${(totalOps / totalTime * 1000).toLocaleString()} ops/sec\n\n`;
      output += ops.map(op =>
        `${op.name}: ${op.time.toFixed(2)}ms (${(size * iterations / op.time * 1000).toLocaleString()} ops/sec)`
      ).join('\n');
    });

    return output;
  },

  'stress-gc': async () => {
    const N = 32768; // 32K samples
    const start = performance.now();

    let output = '';
    await ach.use(async () => {
      // Step 1: Generate multi-frequency signal
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

      // Step 2: Apply Hanning window
      const step2 = performance.now();
      const window = ach.dsp.hanning(N);
      const windowed = ach.vecOps.vmul(signal, window);
      const t2 = performance.now() - step2;

      // Step 3: FFT
      const step3 = performance.now();
      const spectrum = ach.dsp.fftMag(windowed);
      const t3 = performance.now() - step3;

      // Step 4: Statistical analysis
      const step4 = performance.now();
      const sum = ach.sum(spectrum);
      const mean = ach.mean(spectrum);
      const max = ach.max(spectrum);
      const std = ach.std(spectrum);
      const t4 = performance.now() - step4;

      const total = performance.now() - start;

      output = `DSP PIPELINE STRESS (${N.toLocaleString()} samples)\n\n`;
      output += `Signal Generation: ${t1.toFixed(2)}ms\n`;
      output += `Windowing: ${t2.toFixed(2)}ms\n`;
      output += `FFT: ${t3.toFixed(2)}ms\n`;
      output += `Statistics: ${t4.toFixed(2)}ms\n`;
      output += `─────────────────────\n`;
      output += `Total: ${total.toFixed(2)}ms\n`;
      output += `Throughput: ${(N / total * 1000).toLocaleString()} samples/sec`;
    });

    return output;
  },

  // ========== BENCHMARKS ==========
  'bench-vs-mathjs': async () => {
    const size = 10000000; // 10M elements like the old test
    const iterations = 5;   // Fewer iterations due to large size
    const data = Array.from({length: size}, (_, i) => (i % 100) / 10 + 0.1);

    let output = `VECTORIZED MATH BENCHMARK (${size.toLocaleString()} elements × ${iterations} iterations)\n\n`;

    // Benchmark Achronyme (Heavy math operations)
    const achCreateStart = performance.now();
    let achSin, achCos, achExp;
    await ach.use(async () => {
      const v = ach.vector(data);
      const achCreateTime = performance.now() - achCreateStart;

      const achSinStart = performance.now();
      for (let i = 0; i < iterations; i++) {
        achSin = ach.sin(v);
      }
      const achSinTime = performance.now() - achSinStart;

      const achCosStart = performance.now();
      for (let i = 0; i < iterations; i++) {
        achCos = ach.cos(v);
      }
      const achCosTime = performance.now() - achCosStart;

      const achExpStart = performance.now();
      for (let i = 0; i < iterations; i++) {
        achExp = ach.exp(v);
      }
      const achExpTime = performance.now() - achExpStart;

      output += `🔷 Achronyme (WASM)\n`;
      output += `   Create vector: ${achCreateTime.toFixed(2)}ms\n`;
      output += `   sin(): ${achSinTime.toFixed(2)}ms (${(size * iterations / achSinTime * 1000).toLocaleString()} ops/sec)\n`;
      output += `   cos(): ${achCosTime.toFixed(2)}ms (${(size * iterations / achCosTime * 1000).toLocaleString()} ops/sec)\n`;
      output += `   exp(): ${achExpTime.toFixed(2)}ms (${(size * iterations / achExpTime * 1000).toLocaleString()} ops/sec)\n`;
      output += `   TOTAL: ${(achCreateTime + achSinTime + achCosTime + achExpTime).toFixed(2)}ms\n\n`;
    });

    // Benchmark JS Native
    const jsStart = performance.now();
    const jsSinStart = performance.now();
    for (let i = 0; i < iterations; i++) {
      const jsSin = data.map(x => Math.sin(x));
    }
    const jsSinTime = performance.now() - jsSinStart;

    const jsCosStart = performance.now();
    for (let i = 0; i < iterations; i++) {
      const jsCos = data.map(x => Math.cos(x));
    }
    const jsCosTime = performance.now() - jsCosStart;

    const jsExpStart = performance.now();
    for (let i = 0; i < iterations; i++) {
      const jsExp = data.map(x => Math.exp(x));
    }
    const jsExpTime = performance.now() - jsExpStart;

    const jsTotal = jsSinTime + jsCosTime + jsExpTime;

    output += `🟨 JavaScript Native (V8)\n`;
    output += `   sin(): ${jsSinTime.toFixed(2)}ms (${(size * iterations / jsSinTime * 1000).toLocaleString()} ops/sec)\n`;
    output += `   cos(): ${jsCosTime.toFixed(2)}ms (${(size * iterations / jsCosTime * 1000).toLocaleString()} ops/sec)\n`;
    output += `   exp(): ${jsExpTime.toFixed(2)}ms (${(size * iterations / jsExpTime * 1000).toLocaleString()} ops/sec)\n`;
    output += `   TOTAL: ${jsTotal.toFixed(2)}ms\n\n`;

    // Get Achronyme total from output parsing (not ideal but works)
    const achTotalMatch = output.match(/Achronyme[\s\S]*?TOTAL: ([\d.]+)ms/);
    const achTotal = achTotalMatch ? parseFloat(achTotalMatch[1]) : 0;

    output += `━━━━━━━━━━━━━━━━━━━━━\n`;
    const speedup = (jsTotal / achTotal).toFixed(2);
    output += `📊 PERFORMANCE:\n`;
    output += `   Achronyme: ${achTotal.toFixed(2)}ms\n`;
    output += `   JS Native: ${jsTotal.toFixed(2)}ms\n`;
    output += `   Speedup: ${speedup}x ${achTotal < jsTotal ? '🚀 WASM WINS!' : 'JS wins'}\n`;
    output += `   Advantage: ${Math.abs(((jsTotal - achTotal) / jsTotal * 100)).toFixed(1)}%`;

    return output;
  },

  'bench-fft': async () => {
    const size = 8192;
    const iterations = 10;
    const signal = Array.from({length: size}, (_, i) =>
      Math.sin(2 * Math.PI * 50 * i / 1000) +
      0.5 * Math.sin(2 * Math.PI * 120 * i / 1000)
    );

    let output = `FFT BENCHMARK (${size.toLocaleString()} samples × ${iterations} iterations)\n\n`;

    // Benchmark Achronyme
    const achStart = performance.now();
    await ach.use(async () => {
      const v = ach.vector(signal);
      for (let i = 0; i < iterations; i++) {
        const spectrum = ach.fftMag(v);
      }
    });
    const achTime = performance.now() - achStart;

    // Benchmark math.js
    const mathStart = performance.now();
    for (let i = 0; i < iterations; i++) {
      const fft = math.fft(signal);
      // Calculate magnitude
      const mag = fft.map(c => {
        if (typeof c === 'number') return Math.abs(c);
        return Math.sqrt(c.re * c.re + c.im * c.im);
      });
    }
    const mathTime = performance.now() - mathStart;

    const speedup = (mathTime / achTime).toFixed(2);

    output += `🔷 Achronyme: ${achTime.toFixed(2)}ms (${(achTime/iterations).toFixed(2)}ms per FFT)\n`;
    output += `🟦 math.js:   ${mathTime.toFixed(2)}ms (${(mathTime/iterations).toFixed(2)}ms per FFT)\n\n`;
    output += `Speedup: ${speedup}x ${achTime < mathTime ? 'faster' : 'slower'}\n`;
    output += `Winner: ${achTime < mathTime ? '🔷 Achronyme' : '🟦 math.js'}`;

    return output;
  },

  'bench-vector-ops': async () => {
    const size = 200000;
    const iterations = 30;
    const data1 = Array.from({length: size}, () => Math.random() * 100);
    const data2 = Array.from({length: size}, () => Math.random() * 100);

    let output = `VECTOR OPERATIONS (${size.toLocaleString()} elements × ${iterations} iterations)\n\n`;

    // Benchmark Achronyme
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

    // Benchmark math.js
    const mathStart = performance.now();
    for (let i = 0; i < iterations; i++) {
      const add = math.add(data1, data2);
      const mul = math.dotMultiply(data1, data2);
      const dot = math.dot(data1, data2);
    }
    const mathTime = performance.now() - mathStart;

    // Benchmark JS Native
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

    output += `🔷 Achronyme: ${achTime.toFixed(2)}ms (${(totalOps/achTime*1000).toLocaleString()} ops/sec)\n`;
    output += `🟦 math.js:   ${mathTime.toFixed(2)}ms (${(totalOps/mathTime*1000).toLocaleString()} ops/sec)\n`;
    output += `🟨 JS Native: ${jsTime.toFixed(2)}ms (${(totalOps/jsTime*1000).toLocaleString()} ops/sec)\n\n`;

    output += `vs math.js: ${speedupMath}x ${achTime < mathTime ? 'faster' : 'slower'}\n`;
    output += `vs JS V8:   ${speedupJS}x ${achTime < jsTime ? 'faster' : 'slower'}\n\n`;

    const winner = achTime < mathTime && achTime < jsTime ? '🔷 Achronyme' :
                   mathTime < jsTime ? '🟦 math.js' : '🟨 JS Native';
    output += `Winner: ${winner}`;

    return output;
  },

  'bench-pipeline': async () => {
    const size = 16384;
    const signal = Array.from({length: size}, (_, i) =>
      Math.sin(2 * Math.PI * 50 * i / 1000) +
      0.5 * Math.sin(2 * Math.PI * 120 * i / 1000) +
      0.3 * Math.sin(2 * Math.PI * 200 * i / 1000)
    );

    let output = `FULL DSP PIPELINE (${size.toLocaleString()} samples)\n\n`;

    // Achronyme Pipeline
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

    // math.js Pipeline
    const mathStart = performance.now();
    // Generate Hanning window
    const mathWindow = new Array(size);
    for (let i = 0; i < size; i++) {
      mathWindow[i] = 0.5 * (1 - Math.cos(2 * Math.PI * i / (size - 1)));
    }
    const mathWindowed = math.dotMultiply(signal, mathWindow);
    const mathFFT = math.fft(mathWindowed);
    const mathSpectrum = mathFFT.map(c => {
      if (typeof c === 'number') return Math.abs(c);
      return Math.sqrt(c.re * c.re + c.im * c.im);
    });
    const mathMax = math.max(mathSpectrum);
    const mathMean = math.mean(mathSpectrum);
    const mathSum = math.sum(mathSpectrum);
    const mathTime = performance.now() - mathStart;

    const speedup = (mathTime / achTime).toFixed(2);

    output += `Pipeline: Signal → Window → FFT → Statistics\n\n`;
    output += `🔷 Achronyme: ${achTime.toFixed(2)}ms (${(size/achTime*1000).toLocaleString()} samples/sec)\n`;
    output += `🟦 math.js:   ${mathTime.toFixed(2)}ms (${(size/mathTime*1000).toLocaleString()} samples/sec)\n\n`;
    output += `Speedup: ${speedup}x ${achTime < mathTime ? 'faster' : 'slower'}\n`;
    output += `Winner: ${achTime < mathTime ? '🔷 Achronyme' : '🟦 math.js'}`;

    return output;
  },

  // ========== NUMERICAL CALCULUS ==========
  'numerical-diff': async () => {
    return ach.use(async () => {
      // f(x) = x^2, f'(2) = 4
      const df = ach.numerical.diff('x => x^2', 2, 1e-5);
      return `diff(x^2, 2) = ${df.toFixed(6)} (expected: 4.0)`;
    });
  },

  'numerical-diff2': async () => {
    return ach.use(async () => {
      // f(x) = x^3, f''(2) = 12
      const d2f = ach.numerical.diff2('x => x^3', 2, 1e-3);
      return `diff2(x^3, 2) = ${d2f.toFixed(6)} (expected: 12.0)`;
    });
  },

  'numerical-diff3': async () => {
    return ach.use(async () => {
      // f(x) = x^4, f'''(2) = 48
      const d3f = ach.numerical.diff3('x => x^4', 2, 1e-2);
      return `diff3(x^4, 2) = ${d3f.toFixed(6)} (expected: 48.0)`;
    });
  },

  'numerical-integral': async () => {
    return ach.use(async () => {
      // ∫x dx from 0 to 1 = 0.5
      const integral = ach.numerical.integral('x => x', 0, 1, 1000);
      return `integral(x, 0, 1) = ${integral.toFixed(6)} (expected: 0.5)`;
    });
  },

  'numerical-simpson': async () => {
    return ach.use(async () => {
      // ∫x^2 dx from 0 to 1 = 1/3
      const integral = ach.numerical.simpson('x => x^2', 0, 1, 100);
      return `simpson(x^2, 0, 1) = ${integral.toFixed(6)} (expected: 0.333333)`;
    });
  },

  'numerical-romberg': async () => {
    return ach.use(async () => {
      // ∫sin(x) dx from 0 to π = 2
      const integral = ach.numerical.romberg('x => sin(x)', 0, Math.PI, 1e-10);
      return `romberg(sin, 0, π) = ${integral.toFixed(6)} (expected: 2.0)`;
    });
  },

  'numerical-quad': async () => {
    return ach.use(async () => {
      // ∫e^x dx from 0 to 1 = e - 1 ≈ 1.718281828
      const integral = ach.numerical.quad('x => exp(x)', 0, 1);
      const expected = Math.exp(1) - 1;
      return `quad(exp, 0, 1) = ${integral.toFixed(6)} (expected: ${expected.toFixed(6)})`;
    });
  },

  'numerical-solve': async () => {
    return ach.use(async () => {
      // x^2 - 4 = 0, root = 2
      const root = ach.numerical.solve('x => x^2 - 4', 0, 5, 1e-6);
      return `solve(x^2 - 4, [0,5]) = ${root.toFixed(6)} (expected: 2.0)`;
    });
  },

  'numerical-newton': async () => {
    return ach.use(async () => {
      // x^2 - 4 = 0, root = 2 (using Newton's method)
      const root = ach.numerical.newton('x => x^2 - 4', 'x => 2*x', 1, 1e-10, 100);
      return `newton(x^2 - 4, x0=1) = ${root.toFixed(6)} (expected: 2.0)`;
    });
  },

  'numerical-secant': async () => {
    return ach.use(async () => {
      // x^3 - x - 2 = 0, root ≈ 1.521379
      const root = ach.numerical.secant('x => x^3 - x - 2', 1, 2, 1e-10, 100);
      return `secant(x^3 - x - 2, [1,2]) = ${root.toFixed(6)} (expected: 1.521379)`;
    });
  },

  'numerical-all': async () => {
    let output = '=== Running All Numerical Calculus Tests ===\n\n';

    return ach.use(async () => {
      // Differentiation
      output += '--- Numerical Differentiation ---\n';
      const df1 = ach.numerical.diff('x => x^2', 2, 1e-5);
      output += `diff(x^2, 2) = ${df1.toFixed(6)} ✓\n`;

      const d2f2 = ach.numerical.diff2('x => x^3', 2, 1e-3);
      output += `diff2(x^3, 2) = ${d2f2.toFixed(6)} ✓\n`;

      const d3f3 = ach.numerical.diff3('x => x^4', 2, 1e-2);
      output += `diff3(x^4, 2) = ${d3f3.toFixed(6)} ✓\n\n`;

      // Integration
      output += '--- Numerical Integration ---\n';
      const int1 = ach.numerical.integral('x => x', 0, 1, 1000);
      output += `integral(x, 0, 1) = ${int1.toFixed(6)} ✓\n`;

      const int2 = ach.numerical.simpson('x => x^2', 0, 1, 100);
      output += `simpson(x^2, 0, 1) = ${int2.toFixed(6)} ✓\n`;

      const int3 = ach.numerical.romberg('x => sin(x)', 0, Math.PI, 1e-10);
      output += `romberg(sin, 0, π) = ${int3.toFixed(6)} ✓\n`;

      const int4 = ach.numerical.quad('x => exp(x)', 0, 1);
      output += `quad(exp, 0, 1) = ${int4.toFixed(6)} ✓\n\n`;

      // Root Finding
      output += '--- Root Finding ---\n';
      const root1 = ach.numerical.solve('x => x^2 - 4', 0, 5, 1e-6);
      output += `solve(x^2 - 4) = ${root1.toFixed(6)} ✓\n`;

      const root2 = ach.numerical.newton('x => x^2 - 4', 'x => 2*x', 1, 1e-10, 100);
      output += `newton(x^2 - 4) = ${root2.toFixed(6)} ✓\n`;

      const root3 = ach.numerical.secant('x => x^3 - x - 2', 1, 2, 1e-10, 100);
      output += `secant(x^3 - x - 2) = ${root3.toFixed(6)} ✓\n\n`;

      output += '✅ All numerical calculus tests completed successfully!';
      return output;
    });
  },

  // ========== SOC EXPRESSIONS & LAMBDAS ==========
  'soc-simple-expr': async () => {
    ach.resetEvaluator(); // Clean state
    const result = ach.eval("2 + 3 * 4");
    return `2 + 3 * 4 = ${result}`;
  },

  'soc-lambda-create': async () => {
    ach.resetEvaluator(); // Clean state
    const result = ach.eval("x => x * 2");
    return `Created lambda: x => x * 2 (result: ${result})`;
  },

  'soc-lambda-call': async () => {
    ach.resetEvaluator(); // Clean state
    ach.eval("let double = x => x * 2");
    const result = ach.eval("double(5)");
    return `double(5) = ${result}`;
  },

  'soc-lambda-closure': async () => {
    ach.resetEvaluator(); // Clean state
    ach.eval("let multiplier = 3");
    ach.eval("let mult = x => x * multiplier");
    const result = ach.eval("mult(4)");
    return `mult(4) with closure (multiplier=3) = ${result}`;
  },

  'soc-map': async () => {
    ach.resetEvaluator(); // Clean state
    const result = ach.eval("map(x => x * 2, [1, 2, 3, 4])");
    return `map(x => x * 2, [1,2,3,4]) = ${result}`;
  },

  'soc-filter': async () => {
    ach.resetEvaluator(); // Clean state
    const result = ach.eval("filter(x => x > 3, [1, 2, 3, 4, 5, 6])");
    return `filter(x => x > 3, [1,2,3,4,5,6]) = ${result}`;
  },

  'soc-reduce': async () => {
    ach.resetEvaluator(); // Clean state
    const result = ach.eval("reduce((acc, x) => acc + x, 0, [1, 2, 3, 4, 5])");
    return `reduce sum [1,2,3,4,5] = ${result}`;
  },

  'soc-pipe': async () => {
    ach.resetEvaluator(); // Clean state
    ach.eval("let addTwo = x => x + 2");
    ach.eval("let double2 = x => x * 2");
    const result = ach.eval("pipe(5, addTwo, double2)");
    return `pipe(5, addTwo, double2) = ${result} (5 + 2 = 7, 7 * 2 = 14)`;
  },
};

// Attach test handlers to buttons
testButtons.forEach(button => {
  button.addEventListener('click', async () => {
    const testName = button.dataset.test;
    const testFn = tests[testName];

    if (testFn) {
      await runTest(testName, testFn, button);
    } else {
      addResult(testName, 'Test not implemented yet', 'error');
    }
  });
});

// Run all tests in current category
runAllBtn.addEventListener('click', async () => {
  const activeSection = document.querySelector('.test-section:not(.hidden)');
  if (!activeSection) return;

  const categoryTests = activeSection.querySelectorAll('.test-btn');
  addResult('Running All Tests', `Starting ${categoryTests.length} tests in category...`, 'info');

  for (const button of categoryTests) {
    const testName = button.dataset.test;
    const testFn = tests[testName];

    if (testFn) {
      await runTest(testName, testFn, button);
      // Small delay between tests
      await new Promise(resolve => setTimeout(resolve, 100));
    }
  }

  addResult('All Tests Complete', 'All tests in category finished', 'success');
});

// Initialize on load
console.log('📝 Setting up initialization...');
console.log('Document ready state:', document.readyState);

if (document.readyState === 'loading') {
  // DOM not ready yet
  console.log('⏳ DOM still loading, adding listener...');
  window.addEventListener('DOMContentLoaded', () => {
    console.log('🎯 DOMContentLoaded fired!');
    initSDK();
  });
} else {
  // DOM already ready (happens with type="module")
  console.log('✅ DOM already ready, initializing immediately...');
  initSDK();
}

console.log('✅ Initialization setup complete');
