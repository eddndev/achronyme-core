/**
 * DSPOps.ts
 *
 * Digital Signal Processing operations module
 * Provides FFT, convolution, windowing functions, etc.
 */

import type { AchronymeSession } from '../core/Session';
import { Vector } from '../values/Vector';
import { Matrix } from '../values/Matrix';
import type { Value } from '../values/Value';

/**
 * Digital Signal Processing operations
 *
 * Provides:
 * - FFT and IFFT
 * - Convolution
 * - Window functions
 * - Spectral analysis utilities
 */
export class DSPOps {
    constructor(private session: AchronymeSession) {}

    // ========================================================================
    // FFT Operations
    // ========================================================================

    /**
     * Fast Fourier Transform
     *
     * @param signal Input signal (Vector)
     * @returns Complex spectrum as Matrix [N x 2] (real, imaginary)
     *
     * @example
     * ```typescript
     * const signal = session.vector([...1024]);
     * const spectrum = dsp.fft(signal); // Matrix [1024 x 2]
     * ```
     */
    fft(signal: Vector): Matrix {
        const handle = this.session.wasm.fft(signal.handle);
        // FFT returns a matrix with [real, imag] columns
        const len = signal.length;
        return new Matrix(this.session, handle, len, 2);
    }

    /**
     * FFT Magnitude spectrum
     *
     * @param signal Input signal (Vector) or FFT result (Matrix)
     * @returns Magnitude spectrum (Vector)
     *
     * @example
     * ```typescript
     * const signal = session.vector([...1024]);
     * const magnitudes = dsp.fftMag(signal);
     * ```
     */
    fftMag(signal: Vector | Matrix): Vector {
        const handle = this.session.wasm.fft_mag(signal.handle);
        return new Vector(this.session, handle);
    }

    /**
     * FFT Phase spectrum
     *
     * @param signal Input signal (Vector) or FFT result (Matrix)
     * @returns Phase spectrum in radians (Vector)
     */
    fftPhase(signal: Vector | Matrix): Vector {
        // If vector, compute FFT first
        let fftResult: Matrix;
        if (signal instanceof Vector) {
            fftResult = this.fft(signal);
        } else {
            fftResult = signal;
        }

        // Extract real and imaginary parts
        const data = fftResult.data;
        const n = fftResult.rows;
        const phases = new Float64Array(n);

        for (let i = 0; i < n; i++) {
            const re = data[i * 2];
            const im = data[i * 2 + 1];
            phases[i] = Math.atan2(im, re);
        }

        const handle = this.session.wasm.createVector(Array.from(phases));
        return new Vector(this.session, handle);
    }

    /**
     * Inverse Fast Fourier Transform
     *
     * @param spectrum Complex spectrum as Matrix [N x 2]
     * @returns Real signal (Vector)
     */
    ifft(spectrum: Matrix): Vector {
        const handle = this.session.wasm.ifft(spectrum.handle);
        return new Vector(this.session, handle);
    }

    // ========================================================================
    // DFT Operations (Discrete Fourier Transform)
    // ========================================================================

    /**
     * Discrete Fourier Transform (DFT)
     * Slower than FFT but works for any size
     *
     * @param signal Input signal
     * @returns Complex spectrum as Matrix [N x 2]
     */
    dft(signal: Vector): Matrix {
        const data = signal.data;
        const n = data.length;
        const result = new Float64Array(n * 2);

        for (let k = 0; k < n; k++) {
            let re = 0;
            let im = 0;

            for (let t = 0; t < n; t++) {
                const angle = (-2 * Math.PI * k * t) / n;
                re += data[t] * Math.cos(angle);
                im += data[t] * Math.sin(angle);
            }

            result[k * 2] = re;
            result[k * 2 + 1] = im;
        }

        const handle = this.session.wasm.createVector(Array.from(result));
        return new Matrix(this.session, handle, n, 2);
    }

    /**
     * DFT Magnitude spectrum
     */
    dftMag(signal: Vector): Vector {
        const spectrum = this.dft(signal);
        return this.fftMag(spectrum);
    }

    /**
     * DFT Phase spectrum
     */
    dftPhase(signal: Vector): Vector {
        const spectrum = this.dft(signal);
        return this.fftPhase(spectrum);
    }

    // ========================================================================
    // Convolution
    // ========================================================================

    /**
     * Linear convolution
     *
     * @param signal1 First signal
     * @param signal2 Second signal
     * @returns Convolved signal (length: n1 + n2 - 1)
     */
    conv(signal1: Vector, signal2: Vector): Vector {
        const x = signal1.data;
        const h = signal2.data;
        const n1 = x.length;
        const n2 = h.length;
        const resultLen = n1 + n2 - 1;
        const result = new Float64Array(resultLen);

        for (let i = 0; i < resultLen; i++) {
            let sum = 0;
            for (let j = 0; j < n2; j++) {
                const k = i - j;
                if (k >= 0 && k < n1) {
                    sum += x[k] * h[j];
                }
            }
            result[i] = sum;
        }

        const handle = this.session.wasm.createVector(Array.from(result));
        return new Vector(this.session, handle);
    }

    /**
     * Fast convolution using FFT
     * More efficient for large signals
     *
     * @param signal1 First signal
     * @param signal2 Second signal
     * @returns Convolved signal
     */
    convFFT(signal1: Vector, signal2: Vector): Vector {
        // TODO: Implement FFT-based convolution
        // For now, use regular convolution
        return this.conv(signal1, signal2);
    }

    // ========================================================================
    // Window Functions
    // ========================================================================

    /**
     * Hanning window
     *
     * @param n Window length
     * @returns Hanning window (Vector)
     */
    hanning(n: number): Vector {
        const window = new Float64Array(n);

        for (let i = 0; i < n; i++) {
            window[i] = 0.5 * (1 - Math.cos((2 * Math.PI * i) / (n - 1)));
        }

        const handle = this.session.wasm.createVector(Array.from(window));
        return new Vector(this.session, handle);
    }

    /**
     * Hamming window
     *
     * @param n Window length
     * @returns Hamming window (Vector)
     */
    hamming(n: number): Vector {
        const window = new Float64Array(n);

        for (let i = 0; i < n; i++) {
            window[i] = 0.54 - 0.46 * Math.cos((2 * Math.PI * i) / (n - 1));
        }

        const handle = this.session.wasm.createVector(Array.from(window));
        return new Vector(this.session, handle);
    }

    /**
     * Blackman window
     *
     * @param n Window length
     * @returns Blackman window (Vector)
     */
    blackman(n: number): Vector {
        const window = new Float64Array(n);
        const a0 = 0.42;
        const a1 = 0.5;
        const a2 = 0.08;

        for (let i = 0; i < n; i++) {
            const factor = (2 * Math.PI * i) / (n - 1);
            window[i] = a0 - a1 * Math.cos(factor) + a2 * Math.cos(2 * factor);
        }

        const handle = this.session.wasm.createVector(Array.from(window));
        return new Vector(this.session, handle);
    }

    // ========================================================================
    // Utility Functions
    // ========================================================================

    /**
     * FFT shift (move zero frequency to center)
     *
     * @param spectrum FFT spectrum
     * @returns Shifted spectrum
     */
    fftshift(spectrum: Vector): Vector {
        const data = spectrum.data;
        const n = data.length;
        const mid = Math.floor(n / 2);
        const result = new Float64Array(n);

        // Swap halves
        for (let i = 0; i < mid; i++) {
            result[i] = data[i + mid];
            result[i + mid] = data[i];
        }

        // Handle odd length
        if (n % 2 !== 0) {
            result[n - 1] = data[mid];
        }

        const handle = this.session.wasm.createVector(Array.from(result));
        return new Vector(this.session, handle);
    }

    /**
     * Inverse FFT shift
     *
     * @param spectrum Shifted spectrum
     * @returns Original spectrum
     */
    ifftshift(spectrum: Vector): Vector {
        // For symmetric shift, ifftshift is the same as fftshift
        return this.fftshift(spectrum);
    }

    /**
     * Compute full spectrum (magnitude and phase)
     *
     * @param signal Input signal
     * @returns Object with magnitude and phase vectors
     */
    fftSpectrum(signal: Vector): { magnitude: Vector; phase: Vector } {
        const fftResult = this.fft(signal);
        return {
            magnitude: this.fftMag(fftResult),
            phase: this.fftPhase(fftResult),
        };
    }
}
