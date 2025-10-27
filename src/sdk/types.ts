/**
 * Type definitions and interfaces for Achronyme SDK
 */

/**
 * WebAssembly Module interface
 */
export interface WasmModule {
  eval(expression: string): string;
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
}

/**
 * Metadata about a value
 */
export interface ValueMetadata {
  varName: string;
  type: AchronymeValueType;
  disposed: boolean;
  createdAt: number;
}

/**
 * Statistics about memory usage
 */
export interface MemoryStats {
  totalVariables: number;
  activeVariables: number;
  disposedVariables: number;
  variableNames: string[];
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
