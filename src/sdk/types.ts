/**
 * Type definitions and interfaces for Achronyme SDK
 */

/**
 * Handle type for fast path operations
 */
export type Handle = number;

/**
 * WebAssembly Module interface
 */
export interface WasmModule {
  // Expression API (Parsing-based)
  eval(expression: string): string;
  reset(): string;
  listVariables(): string;

  // Fast API (Handle-based)
  createVectorFromBuffer(dataPtr: number, length: number): Handle;
  createMatrixFromBuffer(dataPtr: number, rows: number, cols: number): Handle;
  getVectorData(handle: Handle, outLengthPtr: number): number;
  getMatrixData(handle: Handle, outRowsPtr: number, outColsPtr: number): number;
  copyVectorToBuffer(handle: Handle, destPtr: number, maxLength: number): number;

  // DSP Operations (Fast)
  fft_fast(inputHandle: Handle): Handle;
  fft_mag_fast(inputHandle: Handle): Handle;
  fft_phase_fast(inputHandle: Handle): Handle;
  ifft_fast(inputHandle: Handle): Handle;
  conv_fast(h1: Handle, h2: Handle): Handle;
  conv_fft_fast(h1: Handle, h2: Handle): Handle;

  // Vector Operations (Fast)
  vadd_fast(h1: Handle, h2: Handle): Handle;
  vsub_fast(h1: Handle, h2: Handle): Handle;
  vmul_fast(h1: Handle, h2: Handle): Handle;
  vdiv_fast(h1: Handle, h2: Handle): Handle;
  vscale_fast(h: Handle, scalar: number): Handle;
  dot_fast(h1: Handle, h2: Handle): Handle;
  norm_fast(h: Handle): Handle;

  // Math Functions (Fast)
  sin_fast(h: Handle): Handle;
  cos_fast(h: Handle): Handle;
  tan_fast(h: Handle): Handle;
  exp_fast(h: Handle): Handle;
  ln_fast(h: Handle): Handle;
  abs_fast(h: Handle): Handle;
  sqrt_fast(h: Handle): Handle;

  // Optimization Functions (Fast)
  linspace_fast(start: number, end: number, n: number): Handle;
  fftshift_fast(h: Handle): Handle;
  ifftshift_fast(h: Handle): Handle;
  fft_spectrum_fast(
    signalHandle: Handle,
    fs: number,
    shift: boolean,
    angular: boolean,
    omegaRange: number
  ): Handle;

  // Handle Management
  releaseHandle(handle: Handle): void;
  isValidHandle(handle: Handle): boolean;
  getHandleType(handle: Handle): number;
  cloneHandle(handle: Handle): Handle;
  bindVariableToHandle(varName: string, handle: Handle): void;
  createHandleFromVariable(varName: string): Handle;

  // Emscripten Memory
  _malloc(size: number): number;
  _free(ptr: number): void;
  HEAP8: Int8Array;
  HEAPF64: Float64Array;
  HEAPU32: Uint32Array;
}

/**
 * Complex number representation
 */
export interface ComplexNumber {
  re: number;
  im: number;
}

/**
 * Value types that can be returned from Achronyme
 */
export type AchronymeValueType =
  | 'number'
  | 'complex'
  | 'vector'
  | 'matrix'
  | 'function'
  | 'unknown';

/**
 * Raw JavaScript types that can be used as input
 */
export type PrimitiveValue = number | string | boolean;

/**
 * Options for initializing Achronyme
 */
export interface AchronymeOptions {
  /**
   * Path to the WASM module (if not using default)
   */
  wasmPath?: string;

  /**
   * Whether to enable debug logging
   */
  debug?: boolean;

  /**
   * Maximum number of variables to keep in memory before warning
   */
  maxVariables?: number;

  /**
   * Threshold size for automatic fast path detection (default: 8)
   * Arrays/vectors with length >= threshold will use fast path
   */
  fastPathThreshold?: number;

  /**
   * Force all operations to use fast path when possible (default: false)
   */
  alwaysUseFastPath?: boolean;
}

/**
 * Metadata about a value
 */
export interface ValueMetadata {
  varName: string;
  type: AchronymeValueType;
  disposed: boolean;
  createdAt: number;
  handle?: Handle; // Optional handle for fast path values
  usedFastPath?: boolean; // Whether this value was created via fast path
}

/**
 * Statistics about memory usage
 */
export interface MemoryStats {
  totalVariables: number;
  activeVariables: number;
  disposedVariables: number;
  variableNames: string[];
  activeHandles: number; // Number of active handles in C++
  fastPathUsagePercent: number; // Percentage of operations using fast path
}

/**
 * Lambda function definition
 */
export interface LambdaDefinition {
  params: string[];
  body: string;
}

/**
 * Window function types for DSP
 */
export type WindowFunction = 'hanning' | 'hamming' | 'blackman';

/**
 * FFT result structure
 */
export interface FFTResult {
  real: number[];
  imag: number[];
  magnitude: number[];
  phase: number[];
}

/**
 * Convolution mode
 */
export type ConvolutionMode = 'direct' | 'fft';
