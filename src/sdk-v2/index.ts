/**
 * Achronyme SDK v2.0
 *
 * Modern TypeScript SDK for Achronyme mathematical engine
 * Built on Rust WASM with zero-copy views and automatic memory management
 *
 * @module @achronyme/core
 * @version 2.0.0
 */

// Core infrastructure
export {
    RustWASM,
    rustWASM,
    HandleManager,
    AchronymeSession,
    MemoryPool,
} from './core';

// Value types
export {
    Value,
    Vector,
    Matrix,
    Scalar,
    Complex,
} from './values';

// Type definitions
export type {
    Handle,
    HandleStats,
    ValueMetadata,
    LUResult,
    QRResult,
    SVDResult,
    CholeskyResult,
    EigenResult,
} from './types';
