/**
 * Achronyme SDK v2.0
 *
 * Modern TypeScript SDK for Achronyme mathematical engine
 * Built on Rust WASM with zero-copy views and automatic memory management
 *
 * @module @achronyme/core
 * @version 2.0.0
 */

// Main facade (primary export)
export { Achronyme } from './Achronyme';

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

// Operations modules
export {
    MathOps,
    DSPOps,
    LinalgOps,
    VectorOps,
    HOFOps,
    StatsOps,
} from './operations';

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

// Default export
export { Achronyme as default } from './Achronyme';
