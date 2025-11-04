/**
 * Achronyme.ts
 *
 * Main facade for Achronyme SDK v2.0
 * Provides unified API for all mathematical operations
 */

import { AchronymeSession } from './core/Session';
import { MathOps } from './operations/MathOps';
import { DSPOps } from './operations/DSPOps';
import { LinalgOps } from './operations/LinalgOps';
import { VectorOps } from './operations/VectorOps';
import { HOFOps } from './operations/HOFOps';
import { StatsOps } from './operations/StatsOps';
import { Vector } from './values/Vector';
import { Matrix } from './values/Matrix';
import { Scalar } from './values/Scalar';
import { Complex } from './values/Complex';
import type { Value } from './values/Value';
import type { LUResult, QRResult, SVDResult } from './types';

/**
 * Achronyme - Main facade for mathematical operations
 *
 * Features:
 * - Session-based resource management
 * - Modular operation modules (math, dsp, linalg, etc.)
 * - Convenience methods for common operations
 * - Zero-copy views for performance
 * - Automatic memory cleanup
 *
 * @example
 * ```typescript
 * const ach = new Achronyme();
 * await ach.init();
 *
 * // Session-based usage (recommended)
 * await ach.use(async () => {
 *     const signal = ach.vector([...Array(1000).keys()]);
 *     const spectrum = ach.dsp.fftMag(signal);
 *     // Auto-cleanup when scope exits
 * });
 *
 * // Direct usage (with auto-GC)
 * const v = ach.vector([1, 2, 3, 4]);
 * const result = ach.math.sin(v);
 * ```
 */
export class Achronyme {
    private session: AchronymeSession;

    // Operation modules (public API)
    readonly math: MathOps;
    readonly dsp: DSPOps;
    readonly linalg: LinalgOps;
    readonly vecOps: VectorOps;  // Renamed to avoid conflict with vector() method
    readonly hof: HOFOps;
    readonly stats: StatsOps;

    constructor() {
        this.session = new AchronymeSession();

        // Initialize operation modules
        this.math = new MathOps(this.session);
        this.dsp = new DSPOps(this.session);
        this.linalg = new LinalgOps(this.session);
        this.vecOps = new VectorOps(this.session);  // Renamed to avoid conflict
        this.hof = new HOFOps(this.session);
        this.stats = new StatsOps(this.session);
    }

    // ========================================================================
    // Initialization
    // ========================================================================

    /**
     * Initialize Achronyme (loads WASM module)
     *
     * Must be called before using any operations
     */
    async init(): Promise<void> {
        await this.session.init();
    }

    // ========================================================================
    // Session Management
    // ========================================================================

    /**
     * Use session with automatic cleanup (RAII style)
     *
     * All values created within the callback are automatically
     * cleaned up when the callback exits, even on error
     *
     * @example
     * ```typescript
     * await ach.use(async () => {
     *     const v = ach.vector([...10M elements]);
     *     const result = ach.math.sin(v);
     *     // Auto-cleanup!
     * });
     * ```
     */
    async use<T>(fn: () => Promise<T> | T): Promise<T> {
        return this.session.use(fn);
    }

    /**
     * Manual cleanup (if needed)
     *
     * Cleans up all values created by this instance
     */
    cleanup(): void {
        this.session.cleanup();
    }

    // ========================================================================
    // Value Constructors
    // ========================================================================

    /**
     * Create a vector from array
     *
     * @param data Array of numbers
     * @returns Vector value
     */
    vec(data: number[]): Vector {
        return this.session.vector(data);
    }

    /**
     * Create a vector from array (alias)
     */
    vector(data: number[]): Vector {
        return this.session.vector(data);
    }

    /**
     * Create a matrix from 2D array
     *
     * @param data 2D array of numbers (row-major)
     * @returns Matrix value
     */
    mat(data: number[][]): Matrix {
        return this.session.matrix(data);
    }

    /**
     * Create a matrix from 2D array (alias)
     */
    matrix(data: number[][]): Matrix {
        return this.session.matrix(data);
    }

    /**
     * Create a scalar
     *
     * @param value Number value
     * @returns Scalar value
     */
    scalar(value: number): Scalar {
        return this.session.scalar(value);
    }

    /**
     * Create a complex number
     *
     * @param re Real part
     * @param im Imaginary part
     * @returns Complex value
     */
    complex(re: number, im: number): Complex {
        return this.session.complex(re, im);
    }

    // ========================================================================
    // Utility Functions
    // ========================================================================

    /**
     * Create linearly spaced vector
     *
     * @param start Start value
     * @param stop Stop value
     * @param num Number of points
     * @returns Vector with linearly spaced values
     */
    linspace(start: number, stop: number, num: number): Vector {
        const handle = this.session.wasm.linspace(start, stop, num);
        return new Vector(this.session, handle);
    }

    /**
     * Create identity matrix
     *
     * @param n Size of identity matrix
     * @returns Identity matrix (n x n)
     */
    identity(n: number): Matrix {
        return this.linalg.identity(n);
    }

    /**
     * Create zeros vector
     *
     * @param n Length of vector
     * @returns Vector of zeros
     */
    zeros(n: number): Vector {
        return this.vector(new Array(n).fill(0));
    }

    /**
     * Create ones vector
     *
     * @param n Length of vector
     * @returns Vector of ones
     */
    ones(n: number): Vector {
        return this.vector(new Array(n).fill(1));
    }

    // ========================================================================
    // Convenience Methods (forward to modules)
    // ========================================================================

    // Math operations
    sin(x: number | Value): number | Value {
        return this.math.sin(x);
    }

    cos(x: number | Value): number | Value {
        return this.math.cos(x);
    }

    tan(x: number | Value): number | Value {
        return this.math.tan(x);
    }

    exp(x: number | Value): number | Value {
        return this.math.exp(x);
    }

    ln(x: number | Value): number | Value {
        return this.math.ln(x);
    }

    log(x: number | Value): number | Value {
        return this.math.log(x);
    }

    sqrt(x: number | Value): number | Value {
        return this.math.sqrt(x);
    }

    abs(x: number | Value): number | Value {
        return this.math.abs(x);
    }

    pow(x: number | Value, y: number): number | Value {
        return this.math.pow(x, y);
    }

    // DSP operations
    fft(signal: Vector): Matrix {
        return this.dsp.fft(signal);
    }

    fftMag(signal: Vector | Matrix): Vector {
        return this.dsp.fftMag(signal);
    }

    ifft(spectrum: Matrix): Vector {
        return this.dsp.ifft(spectrum);
    }

    conv(s1: Vector, s2: Vector): Vector {
        return this.dsp.conv(s1, s2);
    }

    // Linalg operations
    lu(matrix: Matrix): LUResult {
        return this.linalg.lu(matrix);
    }

    qr(matrix: Matrix): QRResult {
        return this.linalg.qr(matrix);
    }

    svd(matrix: Matrix): SVDResult {
        return this.linalg.svd(matrix);
    }

    det(matrix: Matrix): number {
        return this.linalg.det(matrix);
    }

    transpose(matrix: Matrix): Matrix {
        return this.linalg.transpose(matrix);
    }

    // Vector operations
    dot(v1: Vector, v2: Vector): number {
        return this.vecOps.dot(v1, v2);
    }

    cross(v1: Vector, v2: Vector): Vector {
        return this.vecOps.cross(v1, v2);
    }

    norm(vector: Vector): number {
        return this.vecOps.norm(vector);
    }

    // Stats operations
    sum(vector: Vector): number {
        return this.stats.sum(vector);
    }

    mean(vector: Vector): number {
        return this.stats.mean(vector);
    }

    std(vector: Vector, ddof?: number): number {
        return this.stats.std(vector, ddof);
    }

    min(vector: Vector): number {
        return this.stats.min(vector);
    }

    max(vector: Vector): number {
        return this.stats.max(vector);
    }

    // HOF operations
    map(fn: (value: number, index: number) => number, vector: Vector): Vector {
        return this.hof.map(fn, vector);
    }

    filter(fn: (value: number, index: number) => boolean, vector: Vector): Vector {
        return this.hof.filter(fn, vector);
    }

    reduce<T>(
        fn: (acc: T, value: number, index: number) => T,
        initialValue: T,
        vector: Vector
    ): T {
        return this.hof.reduce(fn, initialValue, vector);
    }

    // ========================================================================
    // Expression Evaluation (SOC Language)
    // ========================================================================

    /**
     * Evaluate SOC expression string
     *
     * Supports:
     * - Lambda expressions: `x => x * 2`
     * - Variable declarations: `let f = x => x + 1`
     * - Function calls: `f(5)`
     * - All mathematical operations
     * - Higher-order functions (map, filter, reduce, pipe)
     *
     * @param expr SOC expression string
     * @returns Result as string
     *
     * @example
     * ```typescript
     * // Simple expression
     * ach.eval("2 + 3 * 4"); // "14"
     *
     * // Lambda expression
     * ach.eval("x => x * 2"); // "function"
     *
     * // Multi-statement
     * ach.eval("let f = x => x * 2");
     * ach.eval("f(5)"); // "10"
     *
     * // With vectors
     * ach.eval("map(x => x * 2, [1, 2, 3])"); // "[2, 4, 6]"
     *
     * // Higher-order functions
     * ach.eval("filter(x => x > 3, [1, 2, 3, 4, 5])"); // "[4, 5]"
     * ach.eval("reduce((acc, x) => acc + x, 0, [1, 2, 3])"); // "6"
     * ach.eval("pipe(x => x + 1, x => x * 2)(5)"); // "12"
     * ```
     */
    eval(expr: string): string {
        try {
            return this.session.wasm._eval(expr);
        } catch (error) {
            throw new Error(`Eval failed: ${error}`);
        }
    }

    /**
     * Reset the SOC evaluator state
     *
     * Clears all variables and functions declared with `let`
     * Useful when you need a clean evaluator state
     *
     * @example
     * ```typescript
     * ach.eval("let x = 5");
     * ach.eval("x"); // "5"
     * ach.resetEvaluator();
     * ach.eval("x"); // Error: Unknown variable 'x'
     * ```
     */
    resetEvaluator(): void {
        this.session.wasm.reset();
    }

    // ========================================================================
    // Mathematical Constants
    // ========================================================================

    get PI(): number {
        return Math.PI;
    }

    get E(): number {
        return Math.E;
    }

    get SQRT2(): number {
        return Math.SQRT2;
    }

    get LN2(): number {
        return Math.LN2;
    }

    get LN10(): number {
        return Math.LN10;
    }

    // ========================================================================
    // Memory Statistics (for debugging)
    // ========================================================================

    /**
     * Get memory statistics
     *
     * Useful for debugging memory leaks
     */
    getMemoryStats() {
        return this.session.handleManager.getStats();
    }

    /**
     * Force garbage collection of dead handles
     *
     * Returns number of handles cleaned up
     */
    gc(): number {
        return this.session.handleManager.gc();
    }

    /**
     * Get number of active values
     */
    getActiveValuesCount(): number {
        return this.session.getActiveValuesCount();
    }
}
