import { Achronyme } from '../../src/sdk/index.js';

async function main() {
  console.log('--- SDK Example: DSP Pipeline ---');

  const ach = new Achronyme();
  await ach.init();

  await ach.use(async () => {
    const numSamples = 1024;
    const sampleRate = 1024;
    const signalFreq = 50; // 50 Hz

    // 1. Create a time vector using the efficient `linspace` function
    const t = ach.linspace(0, 1, numSamples);

    // 2. Create a 50 Hz sine wave signal
    const signal = ach.map(x => Math.sin(2 * Math.PI * signalFreq * x), t);
    console.log(`Generated a ${signalFreq} Hz signal with ${signal.length} samples.`);

    // 3. Apply a Hanning window to reduce spectral leakage
    const window = ach.dsp.hanning(numSamples);
    const windowedSignal = ach.vecOps.vmul(signal, window); // Efficient element-wise multiplication
    console.log('Applied Hanning window to the signal.');

    // 4. Compute the FFT magnitude spectrum
    const spectrum = ach.dsp.fftMag(windowedSignal);
    console.log('Computed FFT magnitude spectrum.');

    // 5. Find the peak frequency from the spectrum data
    const spectrumData = spectrum.data; // Zero-copy view
    let maxMagnitude = -1;
    let peakFrequency = -1;

    for (let i = 0; i < numSamples / 2; i++) {
      if (spectrumData[i] > maxMagnitude) {
        maxMagnitude = spectrumData[i];
        peakFrequency = i * (sampleRate / numSamples);
      }
    }

    console.log(`
Analysis complete:`);
    console.log(`  - Found peak frequency at: ${peakFrequency.toFixed(2)} Hz`);
    console.log(`  - Peak magnitude: ${maxMagnitude.toFixed(2)}`);

    // All values (t, signal, window, windowedSignal, spectrum) will be auto-disposed.
  });

  console.log('--- End of DSP Pipeline Example ---
');
}

main().catch(console.error);
