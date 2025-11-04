/**
 * LinalgOps.ts
 *
 * Linear Algebra operations module
 * Provides matrix decompositions, eigenvalues, and matrix operations
 */

import type { AchronymeSession } from '../core/Session';
import { Vector } from '../values/Vector';
import { Matrix } from '../values/Matrix';
import type { LUResult, QRResult, SVDResult, CholeskyResult, EigenResult } from '../types';

/**
 * Linear Algebra operations
 *
 * Provides:
 * - Matrix decompositions (LU, QR, SVD, Cholesky)
 * - Eigenvalue computations
 * - Matrix utilities
 */
export class LinalgOps {
    constructor(private session: AchronymeSession) {}

    // ========================================================================
    // Matrix Decompositions
    // ========================================================================

    /**
     * LU Decomposition: PA = LU
     *
     * @param matrix Input matrix
     * @returns Object with L (lower), U (upper), P (permutation) matrices
     *
     * @example
     * ```typescript
     * const A = session.matrix([[4, 3], [6, 3]]);
     * const { L, U, P } = linalg.lu(A);
     * ```
     */
    lu(matrix: Matrix): LUResult {
        const result = this.session.wasm.lu_decomposition_js(matrix.handle);

        const rows = matrix.rows;
        const cols = matrix.cols;

        return {
            L: new Matrix(this.session, result.L, rows, cols),
            U: new Matrix(this.session, result.U, rows, cols),
            P: new Matrix(this.session, result.P, rows, cols),
        };
    }

    /**
     * QR Decomposition: A = QR
     *
     * @param matrix Input matrix
     * @returns Object with Q (orthogonal) and R (upper triangular) matrices
     */
    qr(matrix: Matrix): QRResult {
        const result = this.session.wasm.qr_decomposition_js(matrix.handle);

        const rows = matrix.rows;
        const cols = matrix.cols;

        return {
            Q: new Matrix(this.session, result.Q, rows, cols),
            R: new Matrix(this.session, result.R, rows, cols),
        };
    }

    /**
     * Singular Value Decomposition: A = U Σ V^T
     *
     * @param matrix Input matrix
     * @returns Object with U, S (singular values), V matrices
     */
    svd(matrix: Matrix): SVDResult {
        const result = this.session.wasm.svd_decomposition_js(matrix.handle);

        const rows = matrix.rows;
        const cols = matrix.cols;

        return {
            U: new Matrix(this.session, result.U, rows, rows),
            S: new Vector(this.session, result.S), // Singular values as vector
            V: new Matrix(this.session, result.V, cols, cols),
        };
    }

    /**
     * Cholesky Decomposition: A = L L^T
     * (Only for positive definite matrices)
     *
     * @param matrix Input matrix (must be positive definite)
     * @returns Object with L (lower triangular) matrix
     */
    cholesky(matrix: Matrix): CholeskyResult {
        // TODO: Add Rust WASM binding for Cholesky
        // For now, throw error
        throw new Error('Cholesky decomposition not yet implemented');
    }

    // ========================================================================
    // Eigenvalue Computations
    // ========================================================================

    /**
     * Power iteration method for dominant eigenvalue
     *
     * @param matrix Input matrix
     * @param maxIter Maximum iterations (default: 100)
     * @param tolerance Convergence tolerance (default: 1e-10)
     * @returns Object with dominant eigenvalue and eigenvector
     */
    powerIteration(
        matrix: Matrix,
        maxIter: number = 100,
        tolerance: number = 1e-10
    ): { value: number; vector: Vector } {
        const n = matrix.rows;
        if (n !== matrix.cols) {
            throw new Error('Power iteration requires square matrix');
        }

        // Initialize random vector
        let v = this.session.vector(Array.from({ length: n }, () => Math.random()));

        // Normalize
        v = this.normalize(v);

        let lambda = 0;

        for (let iter = 0; iter < maxIter; iter++) {
            // v_new = A * v
            const vNew = this.matVecMul(matrix, v);

            // lambda = v^T * v_new
            const lambdaNew = this.dot(v, vNew);

            // Check convergence
            if (Math.abs(lambdaNew - lambda) < tolerance) {
                lambda = lambdaNew;
                v = this.normalize(vNew);
                break;
            }

            lambda = lambdaNew;
            v = this.normalize(vNew);
        }

        return { value: lambda, vector: v };
    }

    /**
     * Compute all eigenvalues (using QR algorithm)
     *
     * @param matrix Input matrix (must be square)
     * @param maxIter Maximum iterations (default: 100)
     * @returns Vector of eigenvalues
     */
    eigenvalues(matrix: Matrix, maxIter: number = 100): Vector {
        // TODO: Implement QR algorithm for eigenvalues
        throw new Error('eigenvalues() not yet implemented');
    }

    /**
     * Eigenvalue decomposition: A v = λ v
     *
     * @param matrix Input matrix (must be square)
     * @returns Object with eigenvalues and eigenvectors
     */
    eig(matrix: Matrix): EigenResult {
        // TODO: Implement full eigenvalue decomposition
        throw new Error('eig() not yet implemented');
    }

    // ========================================================================
    // Matrix Utilities
    // ========================================================================

    /**
     * Check if matrix is symmetric
     *
     * @param matrix Input matrix
     * @param tolerance Tolerance for comparison (default: 1e-10)
     * @returns True if symmetric
     */
    isSymmetric(matrix: Matrix, tolerance: number = 1e-10): boolean {
        if (matrix.rows !== matrix.cols) {
            return false;
        }

        const n = matrix.rows;
        for (let i = 0; i < n; i++) {
            for (let j = i + 1; j < n; j++) {
                if (Math.abs(matrix.get(i, j) - matrix.get(j, i)) > tolerance) {
                    return false;
                }
            }
        }

        return true;
    }

    /**
     * Check if matrix is positive definite
     *
     * @param matrix Input matrix
     * @returns True if positive definite
     */
    isPositiveDefinite(matrix: Matrix): boolean {
        // TODO: Implement using Cholesky or eigenvalues
        throw new Error('isPositiveDefinite() not yet implemented');
    }

    /**
     * Create identity matrix
     *
     * @param n Size of identity matrix
     * @returns Identity matrix (n x n)
     */
    identity(n: number): Matrix {
        const data: number[][] = [];
        for (let i = 0; i < n; i++) {
            const row: number[] = [];
            for (let j = 0; j < n; j++) {
                row.push(i === j ? 1 : 0);
            }
            data.push(row);
        }

        return this.session.matrix(data);
    }

    /**
     * Matrix determinant
     *
     * @param matrix Input matrix (must be square)
     * @returns Determinant
     */
    det(matrix: Matrix): number {
        // Use LU decomposition: det(A) = det(P) * det(L) * det(U)
        const { L, U, P } = this.lu(matrix);

        // det(L) = product of diagonal (all 1s for unit lower triangular)
        // det(U) = product of diagonal
        let detU = 1;
        for (let i = 0; i < U.rows; i++) {
            detU *= U.get(i, i);
        }

        // det(P) = ±1 (count permutations)
        // For simplicity, assume det(P) = 1 (TODO: calculate properly)
        const detP = 1;

        return detP * detU;
    }

    /**
     * Matrix inverse
     *
     * @param matrix Input matrix (must be square and invertible)
     * @returns Inverse matrix
     */
    inverse(matrix: Matrix): Matrix {
        const handle = this.session.wasm.matrixInverse(matrix.handle);
        return new Matrix(this.session, handle, matrix.rows, matrix.cols);
    }

    /**
     * Matrix transpose
     *
     * @param matrix Input matrix
     * @returns Transposed matrix
     */
    transpose(matrix: Matrix): Matrix {
        const rows = matrix.rows;
        const cols = matrix.cols;
        const data: number[][] = [];

        for (let j = 0; j < cols; j++) {
            const row: number[] = [];
            for (let i = 0; i < rows; i++) {
                row.push(matrix.get(i, j));
            }
            data.push(row);
        }

        return this.session.matrix(data);
    }

    // ========================================================================
    // Helper Methods
    // ========================================================================

    /**
     * Matrix-vector multiplication
     */
    private matVecMul(matrix: Matrix, vector: Vector): Vector {
        const m = matrix.rows;
        const n = matrix.cols;

        if (n !== vector.length) {
            throw new Error('Matrix-vector dimensions mismatch');
        }

        const result = new Float64Array(m);
        const matData = matrix.data;
        const vecData = vector.data;

        for (let i = 0; i < m; i++) {
            let sum = 0;
            for (let j = 0; j < n; j++) {
                sum += matData[i * n + j] * vecData[j];
            }
            result[i] = sum;
        }

        const handle = this.session.wasm.createVector(Array.from(result));
        return new Vector(this.session, handle);
    }

    /**
     * Dot product
     */
    private dot(v1: Vector, v2: Vector): number {
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
     * Normalize vector (L2 norm)
     */
    private normalize(vector: Vector): Vector {
        const data = vector.data;
        let norm = 0;

        for (let i = 0; i < data.length; i++) {
            norm += data[i] * data[i];
        }

        norm = Math.sqrt(norm);

        const normalized = new Float64Array(data.length);
        for (let i = 0; i < data.length; i++) {
            normalized[i] = data[i] / norm;
        }

        const handle = this.session.wasm.createVector(Array.from(normalized));
        return new Vector(this.session, handle);
    }
}
