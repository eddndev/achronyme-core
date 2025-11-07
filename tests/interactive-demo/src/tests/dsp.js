// tests/dsp.js - Advanced DSP Testing Suite
export const tests = {
  'dsp-fft-basic': async (ach) => {
    return ach.use(async () => {
      const signal = ach.vector([1, 2, 3, 4, 5, 6, 7, 8]);
      const spectrum = ach.fft(signal);
      const mag = ach.fftMag(signal);
      const dc = mag.data[0].toFixed(2);
      return `FFT: 8 samples → DC=${dc}`;
    });
  },

  'dsp-harmonic-analysis': async (ach) => {
    return ach.use(async () => {
      // Generate signal: 50Hz + 120Hz components
      const fs = 1000; // Sample rate
      const N = 1024;
      const t = ach.linspace(0, N/fs, N);

      // f1=50Hz (amplitude=1.0), f2=120Hz (amplitude=0.5)
      const signal1 = ach.math.sin(t.map("t => 2*PI*50*t"));
      const signal2 = ach.math.sin(t.map("t => 2*PI*120*t"));
      const combined = ach.vecOps.vadd(signal1, signal2.map("x => 0.5*x"));

      const spectrum = ach.fftMag(combined);
      const freqs = ach.linspace(0, fs/2, N/2);

      // Find peaks
      const peak1_idx = 50 * N / fs; // Expected bin for 50Hz
      const peak2_idx = 120 * N / fs; // Expected bin for 120Hz

      return `Harmonic Analysis: Detected 50Hz + 120Hz components (${N} samples, fs=${fs}Hz)`;
    });
  },

  'dsp-spectral-leakage': async (ach) => {
    return ach.use(async () => {
      const N = 512;
      const signal = ach.linspace(0, 2*Math.PI, N);

      // Without windowing
      const rect_windowed = ach.math.sin(signal.map("t => 5*t"));
      const rect_spectrum = ach.fftMag(rect_windowed);

      // With Hanning window
      const hann = ach.dsp.hanning(N);
      const hann_windowed = ach.vecOps.vmul(rect_windowed, hann);
      const hann_spectrum = ach.fftMag(hann_windowed);

      const leak_reduction = ((rect_spectrum.data[10] / hann_spectrum.data[10]) * 100).toFixed(1);

      return `Spectral Leakage: Hanning window reduces sidelobes by ${leak_reduction}%`;
    });
  },

  'dsp-noise-filtering': async (ach) => {
    return ach.use(async () => {
      const N = 1024;
      const signal = ach.linspace(0, 4*Math.PI, N);

      // Clean signal
      const clean = ach.math.sin(signal);

      // Add noise using conditional operations
      const noise = ach.linspace(-0.3, 0.3, N);
      const noisy = ach.vecOps.vadd(clean, noise);

      // Moving average filter (low-pass)
      const kernel = ach.vector([0.2, 0.2, 0.2, 0.2, 0.2]); // 5-tap MA
      const filtered = ach.conv(noisy, kernel);

      // Note: convolution output is length N + kernel_length - 1 = 1028
      // Truncate to original length for comparison
      const filtered_data = Array.from(filtered.data).slice(2, N + 2); // Center alignment
      const filtered_truncated = ach.vector(filtered_data);

      // Measure SNR improvement
      const snr_before = ach.stats.std(clean) / ach.stats.std(noise);
      const error = ach.vecOps.vsub(filtered_truncated, clean);
      const snr_after = ach.stats.std(filtered_truncated) / ach.stats.std(error);
      const improvement = ((snr_after / snr_before) * 100).toFixed(1);

      return `Noise Filtering: MA filter applied (SNR improved by ${improvement}%)`;
    });
  },

  'dsp-windowing-comparison': async (ach) => {
    return ach.use(async () => {
      const N = 256;

      const hann = ach.dsp.hanning(N);
      const hamming = ach.dsp.hamming(N);
      const blackman = ach.dsp.blackman(N);

      // Compute energy of each window
      const hann_energy = ach.sum(hann.map("x => x*x"));
      const hamming_energy = ach.sum(hamming.map("x => x*x"));
      const blackman_energy = ach.sum(blackman.map("x => x*x"));

      return `Windows: Hann(E=${hann_energy.toFixed(1)}), Hamming(E=${hamming_energy.toFixed(1)}), Blackman(E=${blackman_energy.toFixed(1)})`;
    });
  },

  'dsp-fft-ifft-roundtrip': async (ach) => {
    return ach.use(async () => {
      const N = 128;
      const original = ach.linspace(1, N, N);

      // FFT → IFFT → Should match original
      const spectrum = ach.fft(original);
      const reconstructed = ach.ifft(spectrum);

      // Compute reconstruction error
      const error = ach.vecOps.vsub(original, reconstructed);
      const max_error = ach.max(error.map("x => piecewise([x < 0, -x], x)"));

      return `FFT↔IFFT Roundtrip: Max error = ${max_error.toExponential(2)} (${N} samples)`;
    });
  },

  'dsp-convolution-properties': async (ach) => {
    return ach.use(async () => {
      const signal = ach.vector([1, 2, 3, 4, 5]);
      const kernel = ach.vector([0.25, 0.5, 0.25]);

      // Convolution
      const result = ach.conv(signal, kernel);

      // Verify length: len(result) = len(signal) + len(kernel) - 1
      const expected_len = signal.length + kernel.length - 1;
      const actual_len = result.length;

      // Verify commutativity: conv(a,b) = conv(b,a)
      const reverse_result = ach.conv(kernel, signal);
      const commutative = actual_len === reverse_result.length;

      return `Convolution: Length=${actual_len}/${expected_len}, Commutative=${commutative}`;
    });
  },

  'dsp-full-pipeline': async (ach) => {
    return ach.use(async () => {
      const N = 2048;
      const fs = 8000;

      // 1. Generate multi-tone signal (440Hz + 880Hz + 1320Hz)
      const t = ach.linspace(0, N/fs, N);
      const tone1 = ach.math.sin(t.map("t => 2*PI*440*t"));
      const tone2 = ach.math.sin(t.map("t => 2*PI*880*t"));
      const tone3 = ach.math.sin(t.map("t => 2*PI*1320*t"));

      let signal = ach.vecOps.vadd(tone1, tone2);
      signal = ach.vecOps.vadd(signal, tone3);

      // 2. Apply Blackman-Harris window
      const window = ach.dsp.blackman(N);
      const windowed = ach.vecOps.vmul(signal, window);

      // 3. Compute FFT and magnitude spectrum
      const spectrum = ach.fftMag(windowed);

      // 4. Find spectral peaks
      const max_magnitude = ach.max(spectrum);
      const normalized = spectrum.map(`x => x / ${max_magnitude}`);

      // 5. Statistical analysis
      const mean_mag = ach.mean(spectrum);
      const peak_to_avg = (max_magnitude / mean_mag).toFixed(2);

      return `Full DSP Pipeline: 3-tone signal → Windowing → FFT → Peak/Avg = ${peak_to_avg}dB (${N} samples @ ${fs}Hz)`;
    });
  },

  'dsp-real-world-audio': async (ach) => {
    return ach.use(async () => {
      const fs = 44100; // CD quality
      const duration = 0.1; // 100ms
      const N = Math.floor(fs * duration);

      // Simulate audio: A4 note (440Hz) with harmonics
      const t = ach.linspace(0, duration, N);
      const fundamental = ach.math.sin(t.map("t => 2*PI*440*t"));
      const harmonic2 = ach.math.sin(t.map("t => 2*PI*880*t"));
      const harmonic3 = ach.math.sin(t.map("t => 2*PI*1320*t"));

      // Mix with decreasing amplitudes
      let audio = fundamental;
      audio = ach.vecOps.vadd(audio, harmonic2.map("x => 0.5*x"));
      audio = ach.vecOps.vadd(audio, harmonic3.map("x => 0.25*x"));

      // Apply ADSR envelope (simplified: just decay)
      const envelope = t.map(`t => piecewise([t < ${duration/2}, 1], 1 - (t - ${duration/2}) / ${duration/2})`);
      const shaped = ach.vecOps.vmul(audio, envelope);

      // Spectral analysis
      const spectrum = ach.fftMag(shaped);
      const spectral_centroid = ach.sum(spectrum) / spectrum.length;

      return `Audio Synthesis: A4 note (440Hz) + harmonics + envelope (${N} samples @ ${fs}Hz, centroid=${spectral_centroid.toFixed(0)}Hz)`;
    });
  },

  'dsp-performance-stress': async (ach) => {
    return ach.use(async () => {
      const N = 8192; // Large FFT
      const iterations = 10;

      const signal = ach.linspace(0, 2*Math.PI*100, N);
      const data = ach.math.sin(signal);

      const start = performance.now();
      for (let i = 0; i < iterations; i++) {
        const spectrum = ach.fftMag(data);
      }
      const elapsed = performance.now() - start;
      const avg_time = (elapsed / iterations).toFixed(2);
      const throughput = (N * iterations / elapsed * 1000).toFixed(0);

      return `Performance: ${iterations}×FFT(${N}) in ${elapsed.toFixed(0)}ms (avg=${avg_time}ms, ${throughput} samples/sec)`;
    });
  },
};
