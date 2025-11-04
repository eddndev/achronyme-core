/**
 * Vector.ts
 *
 * Vector value with zero-copy TypedArray views
 * Represents a 1D array of floating-point numbers
 */

import { Value } from './Value';
import type { Handle } from '../types';
import type { AchronymeSession } from '../core/Session';

/**
 * Vector value with zero-copy access
 *
 * Features:
 * - Zero-copy Float64Array views
 * - Bounds-checked indexing
 * - Iterator support
 * - Explicit copy when needed
 *
 * Usage:
 * ```typescript
 * const v = session.vector([1, 2, 3, 4]);
 *
 * // Zero-copy view (instant)
 * const view = v.data; // Float64Array
 *
 * // Explicit copy (slower)
 * const arr = v.toArray(); // number[]
 *
 * // Iteration (zero-copy)
 * for (const x of v) {
 *     console.log(x);
 * }
 *
 * // Indexing (bounds-checked)
 * const x = v.get(0);
 * v.set(0, 42);
 * ```
 */
export class Vector extends Value {
    private _cachedData?: Float64Array;

    constructor(session: AchronymeSession, handle: Handle) {
        super(session, handle, 'vector');
    }

    /**
     * Get vector length
     */
    get length(): number {
        this.checkDisposed();
        return this.wasm.getVectorLength(this.handle);
    }

    /**
     * Get zero-copy view of vector data
     *
     * WARNING: This view is only valid while the handle exists.
     * Do not store this reference beyond the lifetime of the vector.
     *
     * For long-term storage, use toArray() instead.
     *
     * @returns Float64Array view (zero-copy, instant access)
     */
    get data(): Float64Array {
        this.checkDisposed();

        // Get data from WASM as a regular array
        // Note: Rust wasm-bindgen returns a JavaScript array, not a view
        // This is different from Emscripten's HEAPF64 approach
        const arr = this.wasm.getVector(this.handle);
        return new Float64Array(arr);
    }

    /**
     * Get element at index (bounds-checked)
     *
     * @param index Index to access
     * @returns Value at index
     * @throws RangeError if index out of bounds
     */
    get(index: number): number {
        const data = this.data;

        if (index < 0 || index >= data.length) {
            throw new RangeError(
                `Index ${index} out of bounds [0, ${data.length})`
            );
        }

        return data[index];
    }

    /**
     * Set element at index (bounds-checked)
     *
     * @param index Index to modify
     * @param value New value
     * @throws RangeError if index out of bounds
     */
    set(index: number, value: number): void {
        const data = this.data;

        if (index < 0 || index >= data.length) {
            throw new RangeError(
                `Index ${index} out of bounds [0, ${data.length})`
            );
        }

        data[index] = value;
    }

    /**
     * Convert to JavaScript array (explicit copy)
     *
     * Use this when you need a JavaScript array for compatibility
     * or when you need to store the data beyond the vector's lifetime
     *
     * @returns number[] (copy of data)
     */
    toArray(): number[] {
        return Array.from(this.data);
    }

    /**
     * Iterator support (zero-copy)
     *
     * Allows: for (const x of vector) { ... }
     */
    *[Symbol.iterator](): Iterator<number> {
        const data = this.data;
        for (let i = 0; i < data.length; i++) {
            yield data[i];
        }
    }

    /**
     * Map over vector elements
     *
     * @param fn Mapping function
     * @returns New vector with mapped values
     */
    map(fn: (value: number, index: number) => number): Vector {
        const data = this.data;
        const result = new Float64Array(data.length);

        for (let i = 0; i < data.length; i++) {
            result[i] = fn(data[i], i);
        }

        // Create new vector from result
        const handle = this.wasm.createVector(Array.from(result));
        return new Vector(this.session, handle);
    }

    /**
     * Filter vector elements
     *
     * @param fn Filter predicate
     * @returns New vector with filtered values
     */
    filter(fn: (value: number, index: number) => boolean): Vector {
        const data = this.data;
        const result: number[] = [];

        for (let i = 0; i < data.length; i++) {
            if (fn(data[i], i)) {
                result.push(data[i]);
            }
        }

        // Create new vector from result
        const handle = this.wasm.createVector(result);
        return new Vector(this.session, handle);
    }

    /**
     * Reduce vector to single value
     *
     * @param fn Reducer function
     * @param initialValue Initial accumulator value
     * @returns Reduced value
     */
    reduce<T>(
        fn: (acc: T, value: number, index: number) => T,
        initialValue: T
    ): T {
        const data = this.data;
        let acc = initialValue;

        for (let i = 0; i < data.length; i++) {
            acc = fn(acc, data[i], i);
        }

        return acc;
    }

    /**
     * String representation
     */
    toString(): string {
        if (this.isDisposed) {
            return '[Vector (disposed)]';
        }

        const data = this.data;
        const preview =
            data.length <= 10
                ? Array.from(data).join(', ')
                : `${Array.from(data.slice(0, 5)).join(', ')}, ..., ${Array.from(data.slice(-2)).join(', ')}`;

        return `[Vector (${data.length}): ${preview}]`;
    }

    /**
     * Dispose and invalidate cached data
     */
    dispose(): void {
        this._cachedData = undefined;
        super.dispose();
    }
}
