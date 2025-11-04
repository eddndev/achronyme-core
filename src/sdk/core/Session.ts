/**
 * Session.ts
 *
 * Session-based management for MANUAL resource cleanup
 * Implements RAII-style lifetime management for WASM resources
 */

import type { Handle } from '../types';
import { RustWASM } from './RustBindings';
import { HandleManager } from './HandleManager';
import { Vector } from '../values/Vector';
import { Matrix } from '../values/Matrix';
import { Scalar } from '../values/Scalar';
import { Complex } from '../values/Complex';
import type { Value } from '../values/Value';

/**
 * Session for managing WASM resources with MANUAL cleanup
 *
 * Features:
 * - Automatic cleanup with use() method (RAII style)
 * - Direct value tracking (no WeakRef)
 * - Manual cleanup only
 *
 * Usage:
 * ```typescript
 * const session = new AchronymeSession();
 * await session.use(async () => {
 *     const v = session.vector([1, 2, 3]);
 *     // v auto-cleaned up when scope exits
 * });
 * ```
 */
export class AchronymeSession {
    readonly wasm: RustWASM;
    readonly handleManager: HandleManager;

    private values = new Set<Value>();
    private isActive = false;

    constructor(wasmInstance?: RustWASM) {
        this.wasm = wasmInstance || new RustWASM();
        this.handleManager = new HandleManager(this.wasm);
    }

    /**
     * Initialize the session
     * Must be called before using the session
     */
    async init(): Promise<void> {
        await this.wasm.init();
        this.isActive = true;
    }

    /**
     * Use session with automatic cleanup (RAII style)
     *
     * All values created within the callback are automatically
     * cleaned up when the callback exits, even on error
     *
     * @example
     * ```typescript
     * await session.use(async () => {
     *     const v = session.vector([...10M elements]);
     *     const result = v.sin();
     *     // Auto-cleanup!
     * });
     * ```
     */
    async use<T>(fn: () => Promise<T> | T): Promise<T> {
        if (!this.isActive) {
            throw new Error('Session not initialized. Call init() first.');
        }

        try {
            return await fn();
        } finally {
            this.cleanup();
        }
    }

    /**
     * Manual cleanup
     *
     * Releases all values tracked by this session
     * Called automatically by use(), but can be called manually
     */
    cleanup(): void {
        // Dispose all tracked values
        for (const value of this.values) {
            try {
                value.dispose();
            } catch (error) {
                console.warn('Error disposing value:', error);
            }
        }

        // Clear tracking set
        this.values.clear();
    }

    /**
     * Track a value for manual cleanup
     *
     * This is called automatically by value constructors
     * Users should not call this directly
     */
    track(value: Value): void {
        this.values.add(value);
        this.handleManager.register(value.handle, value);
    }

    /**
     * Untrack a value (when manually disposed)
     *
     * This is called automatically by value.dispose()
     * Users should not call this directly
     */
    untrack(value: Value): void {
        this.values.delete(value);
    }

    /**
     * Get active values count
     *
     * For debugging and testing
     */
    getActiveValuesCount(): number {
        return this.values.size;
    }

    /**
     * Check if session is active
     */
    isSessionActive(): boolean {
        return this.isActive;
    }

    /**
     * Destroy session and release all resources
     *
     * After calling this, the session cannot be used anymore
     */
    destroy(): void {
        this.cleanup();
        this.isActive = false;
    }

    // ========================================================================
    // Value Constructors
    // ========================================================================

    /**
     * Create a vector from JavaScript array
     *
     * @param data Array of numbers
     * @returns Vector value
     *
     * @example
     * ```typescript
     * const v = session.vector([1, 2, 3, 4, 5]);
     * ```
     */
    vector(data: number[]): Vector {
        if (!this.isActive) {
            throw new Error('Session not initialized. Call init() first.');
        }

        const handle = this.wasm.createVector(data);
        return new Vector(this, handle);
    }

    /**
     * Create a matrix from 2D JavaScript array
     *
     * @param data 2D array of numbers (row-major)
     * @returns Matrix value
     *
     * @example
     * ```typescript
     * const m = session.matrix([[1, 2], [3, 4]]);
     * ```
     */
    matrix(data: number[][]): Matrix {
        if (!this.isActive) {
            throw new Error('Session not initialized. Call init() first.');
        }

        // Flatten to row-major order
        const rows = data.length;
        const cols = data[0]?.length || 0;
        const flat: number[] = [];

        for (let i = 0; i < rows; i++) {
            if (data[i].length !== cols) {
                throw new Error('Matrix rows must have equal length');
            }
            for (let j = 0; j < cols; j++) {
                flat.push(data[i][j]);
            }
        }

        const handle = this.wasm.createMatrix(flat, rows, cols);
        return new Matrix(this, handle, rows, cols);
    }

    /**
     * Create a scalar from number
     *
     * @param value Number value
     * @returns Scalar value
     *
     * @example
     * ```typescript
     * const s = session.scalar(42);
     * ```
     */
    scalar(value: number): Scalar {
        if (!this.isActive) {
            throw new Error('Session not initialized. Call init() first.');
        }

        const handle = this.wasm.createVector([value]);
        return new Scalar(this, handle, value);
    }

    /**
     * Create a complex number
     *
     * @param re Real part
     * @param im Imaginary part
     * @returns Complex value
     *
     * @example
     * ```typescript
     * const z = session.complex(3, 4); // 3 + 4i
     * ```
     */
    complex(re: number, im: number): Complex {
        if (!this.isActive) {
            throw new Error('Session not initialized. Call init() first.');
        }

        const handle = this.wasm.createVector([re, im]);
        return new Complex(this, handle, re, im);
    }
}
