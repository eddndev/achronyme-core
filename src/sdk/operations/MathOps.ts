/**
 * MathOps.ts
 *
 * Mathematical operations module
 * Provides trigonometric, exponential, and other math functions
 */

import type { AchronymeSession } from '../core/Session';
import { Vector } from '../values/Vector';
import { Scalar } from '../values/Scalar';
import type { Value } from '../values/Value';

/**
 * Mathematical operations
 *
 * All operations use Fast Path when possible
 * Scalar operations use JavaScript Math for efficiency
 */
export class MathOps {
    constructor(private session: AchronymeSession) {}

    // ========================================================================
    // Trigonometric Functions
    // ========================================================================

    /**
     * Sine function
     * @param x Input (scalar or vector)
     * @returns sin(x)
     */
    sin(x: number | Value): number | Value {
        if (typeof x === 'number') {
            return Math.sin(x);
        }

        const handle = this.session.wasm.sin(x.handle);
        return new Vector(this.session, handle);
    }

    /**
     * Cosine function
     * @param x Input (scalar or vector)
     * @returns cos(x)
     */
    cos(x: number | Value): number | Value {
        if (typeof x === 'number') {
            return Math.cos(x);
        }

        const handle = this.session.wasm.cos(x.handle);
        return new Vector(this.session, handle);
    }

    /**
     * Tangent function
     * @param x Input (scalar or vector)
     * @returns tan(x)
     */
    tan(x: number | Value): number | Value {
        if (typeof x === 'number') {
            return Math.tan(x);
        }

        const handle = this.session.wasm.tan(x.handle);
        return new Vector(this.session, handle);
    }

    /**
     * Arc sine function
     * @param x Input (scalar or vector)
     * @returns asin(x)
     */
    asin(x: number | Value): number | Value {
        if (typeof x === 'number') {
            return Math.asin(x);
        }

        // Map using Vector.map
        if (x instanceof Vector) {
            return x.map(Math.asin);
        }

        throw new Error('asin: unsupported value type');
    }

    /**
     * Arc cosine function
     * @param x Input (scalar or vector)
     * @returns acos(x)
     */
    acos(x: number | Value): number | Value {
        if (typeof x === 'number') {
            return Math.acos(x);
        }

        if (x instanceof Vector) {
            return x.map(Math.acos);
        }

        throw new Error('acos: unsupported value type');
    }

    /**
     * Arc tangent function
     * @param x Input (scalar or vector)
     * @returns atan(x)
     */
    atan(x: number | Value): number | Value {
        if (typeof x === 'number') {
            return Math.atan(x);
        }

        if (x instanceof Vector) {
            return x.map(Math.atan);
        }

        throw new Error('atan: unsupported value type');
    }

    /**
     * Arc tangent of y/x (two-argument form)
     * @param y Y coordinate
     * @param x X coordinate
     * @returns atan2(y, x)
     */
    atan2(y: number | Value, x: number | Value): number | Value {
        if (typeof y === 'number' && typeof x === 'number') {
            return Math.atan2(y, x);
        }

        // Both vectors
        if (y instanceof Vector && x instanceof Vector) {
            const yData = y.data;
            const xData = x.data;

            if (yData.length !== xData.length) {
                throw new Error('atan2: vectors must have same length');
            }

            const result = new Float64Array(yData.length);
            for (let i = 0; i < yData.length; i++) {
                result[i] = Math.atan2(yData[i], xData[i]);
            }

            const handle = this.session.wasm.createVector(Array.from(result));
            return new Vector(this.session, handle);
        }

        throw new Error('atan2: unsupported value types');
    }

    // ========================================================================
    // Hyperbolic Functions
    // ========================================================================

    /**
     * Hyperbolic sine
     * @param x Input (scalar or vector)
     * @returns sinh(x)
     */
    sinh(x: number | Value): number | Value {
        if (typeof x === 'number') {
            return Math.sinh(x);
        }

        if (x instanceof Vector) {
            return x.map(Math.sinh);
        }

        throw new Error('sinh: unsupported value type');
    }

    /**
     * Hyperbolic cosine
     * @param x Input (scalar or vector)
     * @returns cosh(x)
     */
    cosh(x: number | Value): number | Value {
        if (typeof x === 'number') {
            return Math.cosh(x);
        }

        if (x instanceof Vector) {
            return x.map(Math.cosh);
        }

        throw new Error('cosh: unsupported value type');
    }

    /**
     * Hyperbolic tangent
     * @param x Input (scalar or vector)
     * @returns tanh(x)
     */
    tanh(x: number | Value): number | Value {
        if (typeof x === 'number') {
            return Math.tanh(x);
        }

        if (x instanceof Vector) {
            return x.map(Math.tanh);
        }

        throw new Error('tanh: unsupported value type');
    }

    // ========================================================================
    // Exponential and Logarithmic Functions
    // ========================================================================

    /**
     * Exponential function (e^x)
     * @param x Input (scalar or vector)
     * @returns exp(x)
     */
    exp(x: number | Value): number | Value {
        if (typeof x === 'number') {
            return Math.exp(x);
        }

        const handle = this.session.wasm.exp(x.handle);
        return new Vector(this.session, handle);
    }

    /**
     * Natural logarithm (base e)
     * @param x Input (scalar or vector)
     * @returns ln(x)
     */
    ln(x: number | Value): number | Value {
        if (typeof x === 'number') {
            return Math.log(x);
        }

        const handle = this.session.wasm.ln(x.handle);
        return new Vector(this.session, handle);
    }

    /**
     * Natural logarithm (alias for ln)
     * @param x Input (scalar or vector)
     * @returns log(x)
     */
    log(x: number | Value): number | Value {
        return this.ln(x);
    }

    /**
     * Base-10 logarithm
     * @param x Input (scalar or vector)
     * @returns log10(x)
     */
    log10(x: number | Value): number | Value {
        if (typeof x === 'number') {
            return Math.log10(x);
        }

        if (x instanceof Vector) {
            return x.map(Math.log10);
        }

        throw new Error('log10: unsupported value type');
    }

    /**
     * Base-2 logarithm
     * @param x Input (scalar or vector)
     * @returns log2(x)
     */
    log2(x: number | Value): number | Value {
        if (typeof x === 'number') {
            return Math.log2(x);
        }

        if (x instanceof Vector) {
            return x.map(Math.log2);
        }

        throw new Error('log2: unsupported value type');
    }

    /**
     * Power function (x^y)
     * @param x Base (scalar or vector)
     * @param y Exponent (scalar)
     * @returns x^y
     */
    pow(x: number | Value, y: number): number | Value {
        if (typeof x === 'number') {
            return Math.pow(x, y);
        }

        if (x instanceof Vector) {
            return x.map((val) => Math.pow(val, y));
        }

        throw new Error('pow: unsupported value type');
    }

    // ========================================================================
    // Rounding Functions
    // ========================================================================

    /**
     * Floor function (round down)
     * @param x Input (scalar or vector)
     * @returns floor(x)
     */
    floor(x: number | Value): number | Value {
        if (typeof x === 'number') {
            return Math.floor(x);
        }

        if (x instanceof Vector) {
            return x.map(Math.floor);
        }

        throw new Error('floor: unsupported value type');
    }

    /**
     * Ceiling function (round up)
     * @param x Input (scalar or vector)
     * @returns ceil(x)
     */
    ceil(x: number | Value): number | Value {
        if (typeof x === 'number') {
            return Math.ceil(x);
        }

        if (x instanceof Vector) {
            return x.map(Math.ceil);
        }

        throw new Error('ceil: unsupported value type');
    }

    /**
     * Round to nearest integer
     * @param x Input (scalar or vector)
     * @returns round(x)
     */
    round(x: number | Value): number | Value {
        if (typeof x === 'number') {
            return Math.round(x);
        }

        if (x instanceof Vector) {
            return x.map(Math.round);
        }

        throw new Error('round: unsupported value type');
    }

    /**
     * Truncate towards zero
     * @param x Input (scalar or vector)
     * @returns trunc(x)
     */
    trunc(x: number | Value): number | Value {
        if (typeof x === 'number') {
            return Math.trunc(x);
        }

        if (x instanceof Vector) {
            return x.map(Math.trunc);
        }

        throw new Error('trunc: unsupported value type');
    }

    // ========================================================================
    // Other Mathematical Functions
    // ========================================================================

    /**
     * Square root
     * @param x Input (scalar or vector)
     * @returns sqrt(x)
     */
    sqrt(x: number | Value): number | Value {
        if (typeof x === 'number') {
            return Math.sqrt(x);
        }

        const handle = this.session.wasm.sqrt(x.handle);
        return new Vector(this.session, handle);
    }

    /**
     * Cube root
     * @param x Input (scalar or vector)
     * @returns cbrt(x)
     */
    cbrt(x: number | Value): number | Value {
        if (typeof x === 'number') {
            return Math.cbrt(x);
        }

        if (x instanceof Vector) {
            return x.map(Math.cbrt);
        }

        throw new Error('cbrt: unsupported value type');
    }

    /**
     * Absolute value
     * @param x Input (scalar or vector)
     * @returns abs(x)
     */
    abs(x: number | Value): number | Value {
        if (typeof x === 'number') {
            return Math.abs(x);
        }

        const handle = this.session.wasm.abs(x.handle);
        return new Vector(this.session, handle);
    }

    /**
     * Sign function (-1, 0, or 1)
     * @param x Input (scalar or vector)
     * @returns sign(x)
     */
    sign(x: number | Value): number | Value {
        if (typeof x === 'number') {
            return Math.sign(x);
        }

        if (x instanceof Vector) {
            return x.map(Math.sign);
        }

        throw new Error('sign: unsupported value type');
    }
}
