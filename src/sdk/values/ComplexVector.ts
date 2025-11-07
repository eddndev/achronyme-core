/**
 * ComplexVector.ts
 *
 * Complex vector value with interleaved real/imaginary components
 * Represents a 1D array of complex numbers
 */

import { Value } from './Value';
import type { Handle } from '../types';
import type { AchronymeSession } from '../core/Session';

/**
 * Complex number interface
 */
export interface Complex {
    re: number;
    im: number;
}

/**
 * Complex vector value
 *
 * Features:
 * - Interleaved storage: [re0, im0, re1, im1, ...]
 * - Complex number indexing
 * - Iterator support
 *
 * Usage:
 * ```typescript
 * const cv = session.eval('[i, 2+3i, 4]');
 *
 * // Get complex number at index
 * const z = cv.get(0); // { re: 0, im: 1 }
 *
 * // Get length
 * const len = cv.length; // 3
 *
 * // Iterate
 * for (const z of cv) {
 *     console.log(`${z.re} + ${z.im}i`);
 * }
 * ```
 */
export class ComplexVector extends Value {
    constructor(session: AchronymeSession, handle: Handle) {
        super(session, handle, 'complex_vector');
    }

    /**
     * Get vector length (number of complex numbers)
     */
    get length(): number {
        this.checkDisposed();
        const data = this.wasm.getComplexVector(this.handle);
        return data.length / 2;
    }

    /**
     * Get interleaved data [re0, im0, re1, im1, ...]
     */
    get data(): Float64Array {
        this.checkDisposed();
        const arr = this.wasm.getComplexVector(this.handle);
        return new Float64Array(arr);
    }

    /**
     * Get complex number at index (bounds-checked)
     *
     * @param index Index to access
     * @returns Complex number { re, im }
     * @throws RangeError if index out of bounds
     */
    get(index: number): Complex {
        const data = this.data;
        const len = data.length / 2;

        if (index < 0 || index >= len) {
            throw new RangeError(
                `Index ${index} out of bounds [0, ${len})`
            );
        }

        const i = index * 2;
        return {
            re: data[i],
            im: data[i + 1]
        };
    }

    /**
     * Convert to array of complex numbers
     *
     * @returns Array of { re, im } objects
     */
    toComplexArray(): Complex[] {
        const data = this.data;
        const result: Complex[] = [];

        for (let i = 0; i < data.length; i += 2) {
            result.push({
                re: data[i],
                im: data[i + 1]
            });
        }

        return result;
    }

    /**
     * Convert to interleaved array (for Value compatibility)
     *
     * @returns Array in format [re0, im0, re1, im1, ...]
     */
    toArray(): number[] {
        return Array.from(this.data);
    }

    /**
     * Iterator support (for...of loops)
     */
    *[Symbol.iterator](): Iterator<Complex> {
        const data = this.data;

        for (let i = 0; i < data.length; i += 2) {
            yield {
                re: data[i],
                im: data[i + 1]
            };
        }
    }

    /**
     * Format complex number for display
     */
    private formatComplex(z: Complex): string {
        if (z.im >= 0) {
            return `${z.re}+${z.im}i`;
        } else {
            return `${z.re}${z.im}i`;
        }
    }

    /**
     * Convert to string representation
     */
    toString(): string {
        const elements = this.toComplexArray().map(z => this.formatComplex(z));
        return `[${elements.join(', ')}]`;
    }

    /**
     * Debug representation
     */
    [Symbol.for('nodejs.util.inspect.custom')](): string {
        return `ComplexVector(${this.length}) ${this.toString()}`;
    }
}
