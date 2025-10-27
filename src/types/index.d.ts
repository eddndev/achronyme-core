/**
 * Type declarations for Achronyme Core
 */

declare module '@achronyme/core' {
  // Main SDK classes
  export class Achronyme {
    constructor(options?: AchronymeOptions);
    init(): Promise<void>;
    isInitialized(): boolean;

    // Memory management
    getMemoryStats(): MemoryStats;
    disposeAll(): void;
    reset(): void;

    // Type constructors
    number(value: number): AchronymeValue;
    vector(data: number[]): AchronymeValue;
    matrix(data: number[][]): AchronymeValue;
    complex(re: number, im: number): AchronymeValue;

    // Variables and lambdas
    let(name: string, value: AchronymeValue | number | number[] | ComplexNumber): AchronymeValue;
    get(name: string): AchronymeValue;
    lambda(params: string[], body: string): AchronymeValue;

    // Mathematical functions
    sin(x: AchronymeValue | number): AchronymeValue;
    cos(x: AchronymeValue | number): AchronymeValue;
    tan(x: AchronymeValue | number): AchronymeValue;
    sqrt(x: AchronymeValue | number): AchronymeValue;
    exp(x: AchronymeValue | number): AchronymeValue;
    ln(x: AchronymeValue | number): AchronymeValue;
    log(x: AchronymeValue | number): AchronymeValue;
    abs(x: AchronymeValue | number): AchronymeValue;
    // ... (complete list in implementation)

    // DSP functions
    fft(signal: AchronymeValue): AchronymeValue;
    fft_mag(signal: AchronymeValue): AchronymeValue;
    ifft(spectrum: AchronymeValue): AchronymeValue;
    dft(signal: AchronymeValue): AchronymeValue;
    conv(signal1: AchronymeValue, signal2: AchronymeValue): AchronymeValue;
    conv_fft(signal1: AchronymeValue, signal2: AchronymeValue): AchronymeValue;
    hanning(n: number): AchronymeValue;
    hamming(n: number): AchronymeValue;
    blackman(n: number): AchronymeValue;

    // Higher-order functions
    map(fn: string | AchronymeValue, arr: AchronymeValue): AchronymeValue;
    filter(predicate: string | AchronymeValue, arr: AchronymeValue): AchronymeValue;
    reduce(fn: string | AchronymeValue, arr: AchronymeValue, initial: number): AchronymeValue;
    pipe(...fnsAndValue: AchronymeValue[]): AchronymeValue;
    compose(...fns: AchronymeValue[]): AchronymeValue;

    // Constants
    readonly PI: AchronymeValue;
    readonly E: AchronymeValue;
    readonly PHI: AchronymeValue;
    readonly TAU: AchronymeValue;

    // Raw eval
    eval(expression: string): string;
    evalValue(expression: string): AchronymeValue;
  }

  export class AchronymeValue {
    // Memory management
    dispose(): void;
    isDisposed(): boolean;
    getMetadata(): Readonly<ValueMetadata>;

    // Value extraction
    raw(): Promise<string>;
    value<T = any>(): Promise<T>;
    toNumber(): Promise<number>;
    toComplex(): Promise<ComplexNumber>;
    toVector(): Promise<number[]>;
    toMatrix(): Promise<number[][]>;
    getType(): Promise<AchronymeValueType>;

    // Arithmetic
    add(other: AchronymeValue | number): AchronymeValue;
    sub(other: AchronymeValue | number): AchronymeValue;
    mul(other: AchronymeValue | number): AchronymeValue;
    div(other: AchronymeValue | number): AchronymeValue;
    pow(other: AchronymeValue | number): AchronymeValue;
    neg(): AchronymeValue;

    // Comparisons
    gt(other: AchronymeValue | number): AchronymeValue;
    gte(other: AchronymeValue | number): AchronymeValue;
    lt(other: AchronymeValue | number): AchronymeValue;
    lte(other: AchronymeValue | number): AchronymeValue;
    eq(other: AchronymeValue | number): AchronymeValue;
    neq(other: AchronymeValue | number): AchronymeValue;

    // Mathematical functions
    sin(): AchronymeValue;
    cos(): AchronymeValue;
    tan(): AchronymeValue;
    sqrt(): AchronymeValue;
    abs(): AchronymeValue;
    ln(): AchronymeValue;
    exp(): AchronymeValue;
    floor(): AchronymeValue;
    ceil(): AchronymeValue;
    round(): AchronymeValue;

    // DSP functions
    fft(): AchronymeValue;
    fft_mag(): AchronymeValue;
    ifft(): AchronymeValue;
    dft(): AchronymeValue;
    dft_mag(): AchronymeValue;
    dft_phase(): AchronymeValue;

    // Vector/Matrix operations
    dot(other: AchronymeValue): AchronymeValue;
    cross(other: AchronymeValue): AchronymeValue;
    norm(): AchronymeValue;
    transpose(): AchronymeValue;
    det(): AchronymeValue;
    inverse(): AchronymeValue;
  }

  // Error classes
  export class AchronymeError extends Error {
    readonly code?: string;
  }

  export class AchronymeSyntaxError extends AchronymeError {
    readonly expression?: string;
  }

  export class AchronymeRuntimeError extends AchronymeError {
    readonly expression?: string;
  }

  export class AchronymeTypeError extends AchronymeError {
    readonly expected?: string;
    readonly received?: string;
  }

  export class AchronymeDisposedError extends AchronymeError {}

  export class AchronymeNotInitializedError extends AchronymeError {}

  export class AchronymeArgumentError extends AchronymeError {
    readonly functionName?: string;
    readonly expectedArity?: number;
    readonly receivedArity?: number;
  }

  // Types and interfaces
  export interface AchronymeOptions {
    wasmPath?: string;
    debug?: boolean;
    maxVariables?: number;
  }

  export interface ComplexNumber {
    re: number;
    im: number;
  }

  export type AchronymeValueType =
    | 'number'
    | 'complex'
    | 'vector'
    | 'matrix'
    | 'function'
    | 'unknown';

  export interface ValueMetadata {
    varName: string;
    type: AchronymeValueType;
    disposed: boolean;
    createdAt: number;
  }

  export interface MemoryStats {
    totalVariables: number;
    activeVariables: number;
    disposedVariables: number;
    variableNames: string[];
  }

  export interface LambdaDefinition {
    params: string[];
    body: string;
  }

  export type WindowFunction = 'hanning' | 'hamming' | 'blackman';

  export interface FFTResult {
    real: number[];
    imag: number[];
    magnitude: number[];
    phase: number[];
  }

  export type ConvolutionMode = 'direct' | 'fft';

  // Utility functions
  export function parseResult(result: string): any;
  export function parseComplex(str: string): ComplexNumber;
  export function parseVector(str: string): number[];
  export function parseMatrix(str: string): number[][];
  export function formatValue(value: any): string;
  export function formatVector(vec: number[]): string;
  export function formatMatrix(mat: number[][]): string;
  export function formatComplex(c: ComplexNumber): string;
  export function detectType(result: string): AchronymeValueType;

  // WASM module
  export function createModule(): Promise<any>;
}
