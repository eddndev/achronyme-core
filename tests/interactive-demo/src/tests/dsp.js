// tests/dsp.js
export const tests = {
  'dsp-fft': async (ach) => {
    return ach.use(async () => {
      const signal = ach.vector([1, 2, 3, 4, 5, 6, 7, 8]);
      const spectrum = ach.fft(signal);
      return `FFT computed on 8 samples`;
    });
  },
  'dsp-fft-mag': async (ach) => {
    return ach.use(async () => {
      const signal = ach.vector(Array.from({length: 1024}, (_, i) =>
        Math.sin(2 * Math.PI * 50 * i / 1000)
      ));
      const mag = ach.fftMag(signal);
      return `FFT magnitude computed on 1024 samples`;
    });
  },
  'dsp-ifft': async (ach) => {
    return ach.use(async () => {
      const signal = ach.vector([1, 2, 3, 4, 5, 6, 7, 8]);
      const spectrum = ach.fft(signal);
      const reconstructed = ach.ifft(spectrum);
      return `IFFT computed, signal reconstructed`;
    });
  },
  'dsp-windows': async (ach) => {
    return ach.use(async () => {
      const hann = ach.dsp.hanning(64);
      const hamming = ach.dsp.hamming(64);
      const blackman = ach.dsp.blackman(64);
      return `Created Hann, Hamming, Blackman windows (64 samples each)`;
    });
  },
  'dsp-convolution': async (ach) => {
    return ach.use(async () => {
      const signal = ach.vector([1, 2, 3, 4]);
      const kernel = ach.vector([0.25, 0.5, 0.25]);
      const result = ach.conv(signal, kernel);
      return `Convolution computed`;
    });
  },
  'dsp-pipeline': async (ach) => {
    return ach.use(async () => {
      const signal = ach.vector(Array.from({length: 1024}, (_, i) =>
        Math.sin(2 * Math.PI * 50 * i / 1000) + 0.5 * Math.random()
      ));
      return `DSP pipeline test (signal created)`;
    });
  },
};
