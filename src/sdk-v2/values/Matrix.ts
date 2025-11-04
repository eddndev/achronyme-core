/**
 * Matrix.ts
 *
 * Matrix value with zero-copy TypedArray views
 * Represents a 2D array of floating-point numbers (row-major order)
 */

import { Value } from './Value';
import { Vector } from './Vector';
import type { Handle } from '../types';
import type { AchronymeSession } from '../core/Session';

/**
 * Matrix value with zero-copy access
 *
 * Data is stored in row-major order:
 * [[a, b], [c, d]] -> [a, b, c, d]
 *
 * Features:
 * - Zero-copy Float64Array views
 * - Row/column accessors
 * - 2D indexing
 * - Iterator support
 *
 * Usage:
 * ```typescript
 * const m = session.matrix([[1, 2], [3, 4]]);
 *
 * // Zero-copy view (instant)
 * const view = m.data; // Float64Array (flattened)
 *
 * // Get dimensions
 * const rows = m.rows;
 * const cols = m.cols;
 *
 * // Indexing
 * const x = m.get(0, 1); // Get element at row 0, col 1
 * m.set(0, 1, 42); // Set element at row 0, col 1
 *
 * // Row/column access
 * const row0 = m.row(0); // Vector
 * const col1 = m.col(1); // Vector
 * ```
 */
export class Matrix extends Value {
    private _rows?: number;
    private _cols?: number;

    constructor(session: AchronymeSession, handle: Handle, rows?: number, cols?: number) {
        super(session, handle, 'matrix');
        this._rows = rows;
        this._cols = cols;
    }

    /**
     * Get number of rows
     */
    get rows(): number {
        this.checkDisposed();
        if (this._rows === undefined) {
            // Extract from WASM if not cached
            // For now, we'll need to infer from data
            // This should be provided by Rust WASM ideally
            const data = this.data;
            // Assume square matrix if dimensions not provided
            this._rows = Math.sqrt(data.length);
        }
        return this._rows;
    }

    /**
     * Get number of columns
     */
    get cols(): number {
        this.checkDisposed();
        if (this._cols === undefined) {
            const data = this.data;
            this._cols = Math.sqrt(data.length);
        }
        return this._cols;
    }

    /**
     * Get total number of elements
     */
    get length(): number {
        return this.rows * this.cols;
    }

    /**
     * Get zero-copy view of matrix data (flattened, row-major)
     *
     * WARNING: This view is only valid while the handle exists.
     *
     * @returns Float64Array view (zero-copy, instant access)
     */
    get data(): Float64Array {
        this.checkDisposed();

        // Get data from WASM
        const arr = this.wasm.getVector(this.handle);
        return new Float64Array(arr);
    }

    /**
     * Get element at (row, col) with bounds checking
     *
     * @param row Row index (0-based)
     * @param col Column index (0-based)
     * @returns Value at (row, col)
     * @throws RangeError if indices out of bounds
     */
    get(row: number, col: number): number {
        if (row < 0 || row >= this.rows) {
            throw new RangeError(
                `Row index ${row} out of bounds [0, ${this.rows})`
            );
        }
        if (col < 0 || col >= this.cols) {
            throw new RangeError(
                `Column index ${col} out of bounds [0, ${this.cols})`
            );
        }

        const data = this.data;
        const index = row * this.cols + col;
        return data[index];
    }

    /**
     * Set element at (row, col) with bounds checking
     *
     * @param row Row index (0-based)
     * @param col Column index (0-based)
     * @param value New value
     * @throws RangeError if indices out of bounds
     */
    set(row: number, col: number, value: number): void {
        if (row < 0 || row >= this.rows) {
            throw new RangeError(
                `Row index ${row} out of bounds [0, ${this.rows})`
            );
        }
        if (col < 0 || col >= this.cols) {
            throw new RangeError(
                `Column index ${col} out of bounds [0, ${this.cols})`
            );
        }

        const data = this.data;
        const index = row * this.cols + col;
        data[index] = value;
    }

    /**
     * Get row as Vector
     *
     * @param row Row index
     * @returns Vector containing row data (copy)
     */
    row(row: number): Vector {
        if (row < 0 || row >= this.rows) {
            throw new RangeError(
                `Row index ${row} out of bounds [0, ${this.rows})`
            );
        }

        const data = this.data;
        const rowData: number[] = [];
        const start = row * this.cols;

        for (let j = 0; j < this.cols; j++) {
            rowData.push(data[start + j]);
        }

        const handle = this.wasm.createVector(rowData);
        return new Vector(this.session, handle);
    }

    /**
     * Get column as Vector
     *
     * @param col Column index
     * @returns Vector containing column data (copy)
     */
    col(col: number): Vector {
        if (col < 0 || col >= this.cols) {
            throw new RangeError(
                `Column index ${col} out of bounds [0, ${this.cols})`
            );
        }

        const data = this.data;
        const colData: number[] = [];

        for (let i = 0; i < this.rows; i++) {
            colData.push(data[i * this.cols + col]);
        }

        const handle = this.wasm.createVector(colData);
        return new Vector(this.session, handle);
    }

    /**
     * Convert to 2D JavaScript array (explicit copy)
     *
     * @returns number[][] (copy of data in 2D format)
     */
    toArray(): number[][] {
        const data = this.data;
        const result: number[][] = [];

        for (let i = 0; i < this.rows; i++) {
            const row: number[] = [];
            for (let j = 0; j < this.cols; j++) {
                row.push(data[i * this.cols + j]);
            }
            result.push(row);
        }

        return result;
    }

    /**
     * Convert to flat JavaScript array (row-major)
     *
     * @returns number[] (copy of flattened data)
     */
    toFlatArray(): number[] {
        return Array.from(this.data);
    }

    /**
     * Iterator support (iterates over rows)
     *
     * Allows: for (const row of matrix) { ... }
     */
    *[Symbol.iterator](): Iterator<Vector> {
        for (let i = 0; i < this.rows; i++) {
            yield this.row(i);
        }
    }

    /**
     * Map over matrix elements
     *
     * @param fn Mapping function (value, row, col)
     * @returns New matrix with mapped values
     */
    map(fn: (value: number, row: number, col: number) => number): Matrix {
        const data = this.data;
        const result = new Float64Array(data.length);

        for (let i = 0; i < this.rows; i++) {
            for (let j = 0; j < this.cols; j++) {
                const index = i * this.cols + j;
                result[index] = fn(data[index], i, j);
            }
        }

        // Create new matrix from result
        const handle = this.wasm.createVector(Array.from(result));
        return new Matrix(this.session, handle, this.rows, this.cols);
    }

    /**
     * String representation
     */
    toString(): string {
        if (this.isDisposed) {
            return '[Matrix (disposed)]';
        }

        const arr = this.toArray();
        const preview =
            this.rows <= 5
                ? arr.map((row) => `[${row.join(', ')}]`).join('\n  ')
                : arr
                      .slice(0, 3)
                      .map((row) => `[${row.join(', ')}]`)
                      .join('\n  ') + '\n  ...\n  ' +
                  arr
                      .slice(-1)
                      .map((row) => `[${row.join(', ')}]`)
                      .join('\n  ');

        return `[Matrix (${this.rows}x${this.cols}):\n  ${preview}\n]`;
    }
}
