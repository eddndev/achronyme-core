/**
 * VectorOps.ts
 *
 * Vector operations module
 * Provides element-wise operations, dot product, norms, etc.
 */

import type { AchronymeSession } from '../core/Session';
import { Vector } from '../values/Vector';

/**
 * Vector operations
 *
 * Provides:
 * - Element-wise arithmetic (add, sub, mul, div)
 * - Dot product and cross product
 * - Vector norms
 * - Scaling operations
 */
export class VectorOps {
    constructor(private session: AchronymeSession) {}

    // ========================================================================
    // Element-wise Arithmetic
    // ========================================================================

    /**
     * Vector addition (element-wise)
     *
     * @param v1 First vector
     * @param v2 Second vector
     * @returns v1 + v2
     */
    vadd(v1: Vector, v2: Vector): Vector {
        // Fast path: use WASM directly
        const handle = this.session.wasm.vadd(v1.handle, v2.handle);
        return new Vector(this.session, handle);
    }

    /**
     * Vector subtraction (element-wise)
     *
     * @param v1 First vector
     * @param v2 Second vector
     * @returns v1 - v2
     */
    vsub(v1: Vector, v2: Vector): Vector {
        // Fast path: use WASM directly
        const handle = this.session.wasm.vsub(v1.handle, v2.handle);
        return new Vector(this.session, handle);
    }

    /**
     * Vector multiplication (element-wise, Hadamard product)
     *
     * @param v1 First vector
     * @param v2 Second vector
     * @returns v1 ⊙ v2
     */
    vmul(v1: Vector, v2: Vector): Vector {
        // Fast path: use WASM directly
        const handle = this.session.wasm.vmul(v1.handle, v2.handle);
        return new Vector(this.session, handle);
    }

    /**
     * Vector division (element-wise)
     *
     * @param v1 First vector
     * @param v2 Second vector
     * @returns v1 / v2
     */
    vdiv(v1: Vector, v2: Vector): Vector {
        // Fast path: use WASM directly
        const handle = this.session.wasm.vdiv(v1.handle, v2.handle);
        return new Vector(this.session, handle);
    }

    /**
     * Scalar multiplication
     *
     * @param vector Input vector
     * @param scalar Scalar value
     * @returns scalar * vector
     */
    vscale(vector: Vector, scalar: number): Vector {
        const data = vector.data;
        const result = new Float64Array(data.length);

        for (let i = 0; i < data.length; i++) {
            result[i] = data[i] * scalar;
        }

        const handle = this.session.wasm.createVector(Array.from(result));
        return new Vector(this.session, handle);
    }

    // ========================================================================
    // Vector Products
    // ========================================================================

    /**
     * Dot product (inner product)
     *
     * @param v1 First vector
     * @param v2 Second vector
     * @returns v1 · v2
     */
    dot(v1: Vector, v2: Vector): number {
        const d1 = v1.data;
        const d2 = v2.data;

        if (d1.length !== d2.length) {
            throw new Error('Vectors must have same length');
        }

        let sum = 0;
        for (let i = 0; i < d1.length; i++) {
            sum += d1[i] * d2[i];
        }

        return sum;
    }

    /**
     * Cross product (only for 3D vectors)
     *
     * @param v1 First 3D vector
     * @param v2 Second 3D vector
     * @returns v1 × v2
     */
    cross(v1: Vector, v2: Vector): Vector {
        const d1 = v1.data;
        const d2 = v2.data;

        if (d1.length !== 3 || d2.length !== 3) {
            throw new Error('Cross product only defined for 3D vectors');
        }

        const result = new Float64Array(3);
        result[0] = d1[1] * d2[2] - d1[2] * d2[1];
        result[1] = d1[2] * d2[0] - d1[0] * d2[2];
        result[2] = d1[0] * d2[1] - d1[1] * d2[0];

        const handle = this.session.wasm.createVector(Array.from(result));
        return new Vector(this.session, handle);
    }

    // ========================================================================
    // Vector Norms
    // ========================================================================

    /**
     * L2 norm (Euclidean norm)
     *
     * @param vector Input vector
     * @returns ||v||₂
     */
    norm(vector: Vector): number {
        const data = vector.data;
        let sum = 0;

        for (let i = 0; i < data.length; i++) {
            sum += data[i] * data[i];
        }

        return Math.sqrt(sum);
    }

    /**
     * L1 norm (Manhattan norm)
     *
     * @param vector Input vector
     * @returns ||v||₁
     */
    normL1(vector: Vector): number {
        const data = vector.data;
        let sum = 0;

        for (let i = 0; i < data.length; i++) {
            sum += Math.abs(data[i]);
        }

        return sum;
    }

    /**
     * L-infinity norm (maximum norm)
     *
     * @param vector Input vector
     * @returns ||v||∞
     */
    normInf(vector: Vector): number {
        const data = vector.data;
        let max = 0;

        for (let i = 0; i < data.length; i++) {
            const abs = Math.abs(data[i]);
            if (abs > max) {
                max = abs;
            }
        }

        return max;
    }

    /**
     * Normalize vector (L2 norm)
     *
     * @param vector Input vector
     * @returns Normalized vector
     */
    normalize(vector: Vector): Vector {
        const norm = this.norm(vector);

        if (norm < 1e-10) {
            throw new Error('Cannot normalize zero vector');
        }

        return this.vscale(vector, 1 / norm);
    }
}
