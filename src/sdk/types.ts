/**
 * types.ts
 *
 * Shared type definitions for Achronyme SDK v2
 */

/**
 * Handle type (int32 reference to WASM memory)
 */
export type Handle = number;

/**
 * Handle statistics for memory leak detection
 */
export interface HandleStats {
    /** Total handles allocated */
    allocated: number;

    /** Total handles freed */
    freed: number;

    /** Currently active handles */
    active: number;

    /** Potentially leaked handles (allocated - freed - active) */
    leaked: number;
}

/**
 * Value metadata
 */
export interface ValueMetadata {
    /** Handle ID */
    handle: Handle;

    /** Whether Fast Path was used */
    usedFastPath: boolean;

    /** Creation timestamp */
    createdAt: number;

    /** Type of value */
    type: 'scalar' | 'vector' | 'matrix' | 'complex' | 'complex_vector';
}

/**
 * LU Decomposition result
 * PA = LU
 */
export interface LUResult {
    L: any; // Matrix (will be typed when Value types are created)
    U: any; // Matrix
    P: any; // Matrix (permutation)
}

/**
 * QR Decomposition result
 * A = QR
 */
export interface QRResult {
    Q: any; // Matrix (orthogonal)
    R: any; // Matrix (upper triangular)
}

/**
 * SVD Decomposition result
 * A = U * Σ * V^T
 */
export interface SVDResult {
    U: any; // Matrix (left singular vectors)
    S: any; // Vector (singular values)
    V: any; // Matrix (right singular vectors)
}

/**
 * Cholesky Decomposition result
 * A = L * L^T
 */
export interface CholeskyResult {
    L: any; // Matrix (lower triangular)
}

/**
 * Eigenvalue decomposition result
 * A * v = λ * v
 */
export interface EigenResult {
    values: any; // Vector (eigenvalues)
    vectors: any; // Matrix (eigenvectors as columns)
}
