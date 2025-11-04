/**
 * HOFOps.ts
 *
 * Higher-Order Functions operations module
 * Provides map, filter, reduce, pipe, compose, etc.
 */

import type { AchronymeSession } from '../core/Session';
import { Vector } from '../values/Vector';
import type { Value } from '../values/Value';

/**
 * Higher-Order Functions operations
 *
 * Provides functional programming utilities:
 * - map, filter, reduce
 * - pipe, compose
 */
export class HOFOps {
    constructor(private session: AchronymeSession) {}

    // ========================================================================
    // Basic HOF Operations
    // ========================================================================

    /**
     * Map function over vector elements
     *
     * @param fn Mapping function (value, index) => newValue
     * @param vector Input vector
     * @returns New vector with mapped values
     *
     * @example
     * ```typescript
     * const v = session.vector([1, 2, 3, 4]);
     * const squared = hof.map(x => x * x, v); // [1, 4, 9, 16]
     * ```
     */
    map(fn: (value: number, index: number) => number, vector: Vector): Vector {
        return vector.map(fn);
    }

    /**
     * Filter vector elements
     *
     * @param fn Filter predicate (value, index) => boolean
     * @param vector Input vector
     * @returns New vector with filtered values
     *
     * @example
     * ```typescript
     * const v = session.vector([1, 2, 3, 4, 5, 6]);
     * const evens = hof.filter(x => x % 2 === 0, v); // [2, 4, 6]
     * ```
     */
    filter(fn: (value: number, index: number) => boolean, vector: Vector): Vector {
        return vector.filter(fn);
    }

    /**
     * Reduce vector to single value
     *
     * @param fn Reducer function (acc, value, index) => newAcc
     * @param initialValue Initial accumulator value
     * @param vector Input vector
     * @returns Reduced value
     *
     * @example
     * ```typescript
     * const v = session.vector([1, 2, 3, 4]);
     * const sum = hof.reduce((acc, x) => acc + x, 0, v); // 10
     * ```
     */
    reduce<T>(
        fn: (acc: T, value: number, index: number) => T,
        initialValue: T,
        vector: Vector
    ): T {
        return vector.reduce(fn, initialValue);
    }

    // ========================================================================
    // Function Composition
    // ========================================================================

    /**
     * Pipe functions (left to right composition)
     *
     * @param fns Array of functions to apply in sequence
     * @returns Function that applies all functions in order
     *
     * @example
     * ```typescript
     * const v = session.vector([1, 2, 3, 4]);
     * const pipeline = hof.pipe([
     *     (x) => math.sin(x),
     *     (x) => math.abs(x),
     *     (x) => dsp.fftMag(x)
     * ]);
     * const result = pipeline(v);
     * ```
     */
    pipe<T extends Value>(fns: Array<(x: T) => T>): (x: T) => T {
        return (x: T) => {
            let result = x;
            for (const fn of fns) {
                result = fn(result);
            }
            return result;
        };
    }

    /**
     * Compose functions (right to left composition)
     *
     * @param fns Array of functions to compose
     * @returns Function that applies all functions in reverse order
     *
     * @example
     * ```typescript
     * const f = hof.compose([
     *     (x) => math.sqrt(x),  // Applied last
     *     (x) => math.abs(x),   // Applied second
     *     (x) => math.sin(x)    // Applied first
     * ]);
     * ```
     */
    compose<T extends Value>(fns: Array<(x: T) => T>): (x: T) => T {
        return (x: T) => {
            let result = x;
            for (let i = fns.length - 1; i >= 0; i--) {
                result = fns[i](result);
            }
            return result;
        };
    }

    // ========================================================================
    // Specialized HOF Operations
    // ========================================================================

    /**
     * Scan (cumulative reduce)
     *
     * @param fn Accumulator function
     * @param initialValue Initial value
     * @param vector Input vector
     * @returns Vector of cumulative results
     *
     * @example
     * ```typescript
     * const v = session.vector([1, 2, 3, 4]);
     * const cumsum = hof.scan((acc, x) => acc + x, 0, v); // [1, 3, 6, 10]
     * ```
     */
    scan(
        fn: (acc: number, value: number) => number,
        initialValue: number,
        vector: Vector
    ): Vector {
        const data = vector.data;
        const result = new Float64Array(data.length);
        let acc = initialValue;

        for (let i = 0; i < data.length; i++) {
            acc = fn(acc, data[i]);
            result[i] = acc;
        }

        const handle = this.session.wasm.createVector(Array.from(result));
        return new Vector(this.session, handle);
    }

    /**
     * Zip two vectors together
     *
     * @param fn Combining function
     * @param v1 First vector
     * @param v2 Second vector
     * @returns New vector with combined values
     *
     * @example
     * ```typescript
     * const v1 = session.vector([1, 2, 3]);
     * const v2 = session.vector([4, 5, 6]);
     * const zipped = hof.zip((a, b) => a + b, v1, v2); // [5, 7, 9]
     * ```
     */
    zip(
        fn: (a: number, b: number) => number,
        v1: Vector,
        v2: Vector
    ): Vector {
        const d1 = v1.data;
        const d2 = v2.data;

        const len = Math.min(d1.length, d2.length);
        const result = new Float64Array(len);

        for (let i = 0; i < len; i++) {
            result[i] = fn(d1[i], d2[i]);
        }

        const handle = this.session.wasm.createVector(Array.from(result));
        return new Vector(this.session, handle);
    }

    /**
     * For each element (side effects)
     *
     * @param fn Function to apply to each element
     * @param vector Input vector
     */
    forEach(fn: (value: number, index: number) => void, vector: Vector): void {
        const data = vector.data;
        for (let i = 0; i < data.length; i++) {
            fn(data[i], i);
        }
    }

    /**
     * Check if any element satisfies predicate
     *
     * @param fn Predicate function
     * @param vector Input vector
     * @returns True if any element satisfies predicate
     */
    some(fn: (value: number, index: number) => boolean, vector: Vector): boolean {
        const data = vector.data;
        for (let i = 0; i < data.length; i++) {
            if (fn(data[i], i)) {
                return true;
            }
        }
        return false;
    }

    /**
     * Check if all elements satisfy predicate
     *
     * @param fn Predicate function
     * @param vector Input vector
     * @returns True if all elements satisfy predicate
     */
    every(fn: (value: number, index: number) => boolean, vector: Vector): boolean {
        const data = vector.data;
        for (let i = 0; i < data.length; i++) {
            if (!fn(data[i], i)) {
                return false;
            }
        }
        return true;
    }
}
