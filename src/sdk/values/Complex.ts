/**
 * Complex.ts
 *
 * Complex number value
 * Represents a complex number with real and imaginary parts
 */

import { Value } from './Value';
import type { Handle } from '../types';
import type { AchronymeSession } from '../core/Session';

/**
 * Complex number value
 *
 * Represents: z = a + bi
 *
 * Features:
 * - Real and imaginary parts
 * - Magnitude and phase
 * - String formatting
 *
 * Usage:
 * ```typescript
 * const z = session.complex(3, 4);
 * const re = z.re; // 3
 * const im = z.im; // 4
 * const mag = z.magnitude; // 5
 * const phase = z.phase; // 0.927... radians
 * ```
 */
export class Complex extends Value {
    private _re?: number;
    private _im?: number;

    constructor(session: AchronymeSession, handle: Handle, re?: number, im?: number) {
        super(session, handle, 'complex');
        this._re = re;
        this._im = im;
    }

    /**
     * Get real part
     */
    get re(): number {
        this.checkDisposed();

        if (this._re === undefined) {
            // Extract from WASM if not cached
            const data = this.wasm.getVector(this.handle);
            this._re = data[0];
            this._im = data[1];
        }

        return this._re;
    }

    /**
     * Get imaginary part
     */
    get im(): number {
        this.checkDisposed();

        if (this._im === undefined) {
            // Extract from WASM if not cached
            const data = this.wasm.getVector(this.handle);
            this._re = data[0];
            this._im = data[1];
        }

        return this._im;
    }

    /**
     * Get magnitude (|z| = sqrt(re² + im²))
     */
    get magnitude(): number {
        const re = this.re;
        const im = this.im;
        return Math.sqrt(re * re + im * im);
    }

    /**
     * Get phase/argument (angle in radians)
     */
    get phase(): number {
        return Math.atan2(this.im, this.re);
    }

    /**
     * Get conjugate (z* = re - i*im)
     */
    conjugate(): Complex {
        const handle = this.wasm.createVector([this.re, -this.im]);
        return new Complex(this.session, handle, this.re, -this.im);
    }

    /**
     * Convert to array [re, im]
     */
    toArray(): number[] {
        return [this.re, this.im];
    }

    /**
     * Convert to polar form [magnitude, phase]
     */
    toPolar(): [number, number] {
        return [this.magnitude, this.phase];
    }

    /**
     * String representation
     *
     * Formats as:
     * - "a + bi" for positive imaginary
     * - "a - bi" for negative imaginary
     * - "a" for zero imaginary
     * - "bi" for zero real
     */
    toString(): string {
        if (this.isDisposed) {
            return '[Complex (disposed)]';
        }

        const re = this.re;
        const im = this.im;

        // Pure real
        if (Math.abs(im) < 1e-10) {
            return `${re}`;
        }

        // Pure imaginary
        if (Math.abs(re) < 1e-10) {
            return `${im}i`;
        }

        // Complex
        const sign = im >= 0 ? '+' : '-';
        return `${re} ${sign} ${Math.abs(im)}i`;
    }
}
