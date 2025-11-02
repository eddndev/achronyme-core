/**
 * Achronyme - Main SDK class
 *
 * Provides a type-safe, idiomatic TypeScript API over the Achronyme Core WASM module.
 * Manages variables in the C++ Environment and provides fluent chainable operations.
 *
 * @example
 * const ach = new Achronyme();
 * await ach.init();
 *
 * const signal = ach.vector([1, 2, 3, 4, 5, 6, 7, 8]);
 * const spectrum = signal.fft_mag();
 * console.log(await spectrum.toVector());
 *
 * // Clean up
 * signal.dispose();
 * spectrum.dispose();
 */

import createAchronymeModule from '../achronyme-core.mjs';
import { AchronymeValue } from './AchronymeValue.js';
import {
  WasmModule,
  AchronymeOptions,
  MemoryStats,
  ComplexNumber,
  Handle,
  LUResult,
  QRResult,
  SVDResult,
  EigenResult,
  PowerIterationResult,
} from './types.js';
import {
  AchronymeNotInitializedError,
  AchronymeArgumentError,
  wrapCppError,
} from './errors.js';
import { formatValue, formatVector, formatMatrix, formatComplex, isValidVariableName } from './utils.js';

export class Achronyme {
  private module: WasmModule | null = null;
  private varCounter: number = 0;
  private variables: Set<string> = new Set();
  private options: AchronymeOptions;
  private initialized: boolean = false;

  // Handle tracking for fast path
  private handleToVar: Map<Handle, string> = new Map();
  private varToHandle: Map<string, Handle> = new Map();
  private fastPathOperationsCount: number = 0;
  private slowPathOperationsCount: number = 0;

  constructor(options: AchronymeOptions = {}) {
    this.options = {
      debug: false,
      maxVariables: 10000,
      fastPathThreshold: 8,
      alwaysUseFastPath: false,
      ...options,
    };
  }

  // ============================================================================
  // Initialization
  // ============================================================================

  /**
   * Initialize the WASM module
   * Must be called before any other operations
   */
  async init(): Promise<void> {
    if (this.initialized) {
      return;
    }

    try {
      const moduleInstance = await createAchronymeModule();
      this.module = moduleInstance as unknown as WasmModule;
      this.initialized = true;

      if (this.options.debug) {
        console.log('[Achronyme] Module initialized successfully');
      }
    } catch (e: any) {
      throw wrapCppError(`Failed to initialize WASM module: ${e.message || e}`);
    }
  }

  /**
   * Check if the module is initialized
   */
  isInitialized(): boolean {
    return this.initialized;
  }

  // ============================================================================
  // Internal Helpers
  // ============================================================================

  /**
   * @internal
   * Evaluate an expression and return the raw string result
   */
  _eval(expression: string): string {
    if (!this.module) {
      throw new AchronymeNotInitializedError();
    }

    try {
      const result = this.module.eval(expression);
      if (this.options.debug) {
        console.log(`[Achronyme] eval: ${expression} => ${result}`);
      }
      return result;
    } catch (e: any) {
      throw wrapCppError(e.message || String(e), expression);
    }
  }

  /**
   * @internal
   * Generate a unique variable name
   */
  private generateVarName(): string {
    return `__v${this.varCounter++}`;
  }

  /**
   * @internal
   * Create a new AchronymeValue from an expression
   */
  _createFromExpression(expression: string): AchronymeValue {
    const varName = this.generateVarName();
    this._eval(`let ${varName} = ${expression}`);
    this.variables.add(varName);
    this.checkMaxVariables();
    return new AchronymeValue(this, varName);
  }

  /**
   * @internal
   * Create a new AchronymeValue from a direct value
   */
  private createFromValue(value: any): AchronymeValue {
    const expr = formatValue(value);
    return this._createFromExpression(expr);
  }

  // ============================================================================
  // Fast Path Memory Management
  // ============================================================================

  /**
   * @internal
   * Allocate Float64 array in WASM heap
   */
  private _allocFloat64(data: ArrayLike<number>): number {
    if (!this.module) throw new AchronymeNotInitializedError();

    const byteLength = data.length * 8; // 8 bytes per double
    const ptr = this.module._malloc(byteLength);

    // Write data to heap
    // Access heap through HEAPF64 - Emscripten ensures this is always available after init
    const heap = this.module.HEAPF64.subarray(ptr / 8, ptr / 8 + data.length);

    // Copy data
    for (let i = 0; i < data.length; i++) {
      heap[i] = data[i];
    }

    return ptr;
  }

  /**
   * @internal
   * Read Float64 array from WASM heap
   */
  private _readFloat64FromHandle(handle: Handle): Float64Array {
    if (!this.module) throw new AchronymeNotInitializedError();

    // Allocate space for output length
    const lengthPtr = this.module._malloc(4); // size_t

    try {
      // Get pointer to data
      const dataPtr = this.module.getVectorData(handle, lengthPtr);

      // Read length
      const length = this.module.HEAPU32[lengthPtr / 4];

      // Create view (zero-copy)
      // Use subarray for Emscripten 4.0 compatibility
      return this.module.HEAPF64.subarray(dataPtr / 8, dataPtr / 8 + length);
    } finally {
      this.module._free(lengthPtr);
    }
  }

  /**
   * @internal
   * Create vector using fast path (handle-based, no parsing)
   */
  private _createVectorFast(data: ArrayLike<number>): AchronymeValue {
    if (!this.module) throw new AchronymeNotInitializedError();

    try {
      // Allocate and write data to WASM heap
      const ptr = this._allocFloat64(data);

      // Create handle from buffer
      const handle = this.module.createVectorFromBuffer(ptr, data.length);

      // Free temporary buffer (handle maintains its own copy)
      this.module._free(ptr);

      // Generate variable name and bind
      const varName = this.generateVarName();
      this.module.bindVariableToHandle(varName, handle);

      // Track
      this.variables.add(varName);
      this.handleToVar.set(handle, varName);
      this.varToHandle.set(varName, handle);
      this.fastPathOperationsCount++;

      if (this.options.debug) {
        console.log(`[Achronyme] Created vector via FAST path: ${varName} (${data.length} elements, handle=${handle})`);
      }

      return new AchronymeValue(this, varName, handle);
    } catch (e: any) {
      throw wrapCppError(`Failed to create vector (fast path): ${e.message || e}`);
    }
  }

  /**
   * @internal
   * Create matrix using fast path
   */
  private _createMatrixFast(data: number[][]): AchronymeValue {
    if (!this.module) throw new AchronymeNotInitializedError();

    try {
      const rows = data.length;
      const cols = data[0].length;

      // Flatten matrix (row-major)
      const flat = new Float64Array(rows * cols);
      let idx = 0;
      for (let i = 0; i < rows; i++) {
        for (let j = 0; j < cols; j++) {
          flat[idx++] = data[i][j];
        }
      }

      // Allocate and write
      const ptr = this._allocFloat64(flat);
      const handle = this.module.createMatrixFromBuffer(ptr, rows, cols);
      this.module._free(ptr);

      // Track
      const varName = this.generateVarName();
      this.module.bindVariableToHandle(varName, handle);
      this.variables.add(varName);
      this.handleToVar.set(handle, varName);
      this.varToHandle.set(varName, handle);
      this.fastPathOperationsCount++;

      if (this.options.debug) {
        console.log(`[Achronyme] Created matrix via FAST path: ${varName} (${rows}x${cols}, handle=${handle})`);
      }

      return new AchronymeValue(this, varName, handle);
    } catch (e: any) {
      throw wrapCppError(`Failed to create matrix (fast path): ${e.message || e}`);
    }
  }

  /**
   * @internal
   * Create value from handle and bind to variable
   */
  _createFromHandle(handle: Handle): AchronymeValue {
    if (!this.module) throw new AchronymeNotInitializedError();

    const varName = this.generateVarName();
    this.module.bindVariableToHandle(varName, handle);

    this.variables.add(varName);
    this.handleToVar.set(handle, varName);
    this.varToHandle.set(varName, handle);

    return new AchronymeValue(this, varName, handle);
  }

  /**
   * @internal
   * Get handle for a variable (if it exists)
   */
  _getHandle(varName: string): Handle | undefined {
    return this.varToHandle.get(varName);
  }

  /**
   * @internal
   * Dispose a variable (called by AchronymeValue.dispose())
   */
  _disposeVariable(varName: string): void {
    if (this.variables.has(varName)) {
      this.variables.delete(varName);

      // Release handle if exists
      const handle = this.varToHandle.get(varName);
      if (handle !== undefined && this.module) {
        try {
          this.module.releaseHandle(handle);
          this.handleToVar.delete(handle);
          this.varToHandle.delete(varName);

          if (this.options.debug) {
            console.log(`[Achronyme] Disposed variable: ${varName} (handle=${handle})`);
          }
        } catch (e) {
          // Ignore errors during disposal
          if (this.options.debug) {
            console.warn(`[Achronyme] Warning: Failed to release handle ${handle}`);
          }
        }
      } else if (this.options.debug) {
        console.log(`[Achronyme] Disposed variable: ${varName} (no handle)`);
      }
    }
  }

  /**
   * Check if we're approaching the max variables limit
   */
  private checkMaxVariables(): void {
    if (this.options.maxVariables && this.variables.size >= this.options.maxVariables) {
      console.warn(
        `[Achronyme] Warning: ${this.variables.size} variables in memory. ` +
        `Consider calling dispose() on unused values to free memory.`
      );
    }
  }

  // ============================================================================
  // Memory Management
  // ============================================================================

  /**
   * Get statistics about memory usage
   */
  getMemoryStats(): MemoryStats {
    const totalOps = this.fastPathOperationsCount + this.slowPathOperationsCount;
    const fastPathPercent = totalOps > 0
      ? (this.fastPathOperationsCount / totalOps) * 100
      : 0;

    return {
      totalVariables: this.varCounter,
      activeVariables: this.variables.size,
      disposedVariables: this.varCounter - this.variables.size,
      variableNames: Array.from(this.variables),
      activeHandles: this.handleToVar.size,
      fastPathUsagePercent: fastPathPercent,
    };
  }

  /**
   * Dispose all tracked variables
   * WARNING: This will invalidate all AchronymeValue instances
   */
  disposeAll(): void {
    this.variables.clear();
    if (this.options.debug) {
      console.log('[Achronyme] All variables disposed');
    }
  }

  /**
   * Reset the entire environment (clear all variables and state)
   * This also resets the WASM module's internal state
   */
  reset(): void {
    if (!this.module) {
      throw new AchronymeNotInitializedError();
    }

    // Reset WASM module
    try {
      (this.module as any).reset();
    } catch (e) {
      // reset() might not be available, ignore
    }

    // Reset SDK state
    this.disposeAll();
    this.varCounter = 0;
    if (this.options.debug) {
      console.log('[Achronyme] Environment reset');
    }
  }

  // ============================================================================
  // Type Constructors
  // ============================================================================

  /**
   * Create a number value
   * @example
   * const x = ach.number(42);
   * const y = ach.number(Math.PI);
   */
  number(value: number): AchronymeValue {
    return this.createFromValue(value);
  }

  /**
   * Create a vector value
   * OPTIMIZED: Automatically uses fast path for large arrays
   * @example
   * const v = ach.vector([1, 2, 3, 4]);
   * const large = ach.vector(new Float64Array(10000)); // Fast path!
   */
  vector(data: number[] | Float64Array): AchronymeValue {
    if (!Array.isArray(data) && !(data instanceof Float64Array)) {
      throw new AchronymeArgumentError('vector() requires an array of numbers or Float64Array');
    }

    const threshold = this.options.fastPathThreshold || 8;
    const useFastPath = this.options.alwaysUseFastPath || data.length >= threshold;

    if (useFastPath) {
      // FAST PATH: Handle-based, no parsing
      return this._createVectorFast(data);
    } else {
      // SLOW PATH: Expression-based with parsing
      this.slowPathOperationsCount++;
      if (this.options.debug) {
        console.log(`[Achronyme] Created vector via SLOW path (${data.length} elements < threshold ${threshold})`);
      }
      return this.createFromValue(formatVector(Array.from(data)));
    }
  }

  /**
   * Create a matrix value
   * OPTIMIZED: Automatically uses fast path for large matrices
   * @example
   * const m = ach.matrix([[1, 2], [3, 4]]);
   */
  matrix(data: number[][]): AchronymeValue {
    if (!Array.isArray(data) || !Array.isArray(data[0])) {
      throw new AchronymeArgumentError('matrix() requires a 2D array of numbers');
    }

    // Calculate total elements
    const totalElements = data.reduce((sum, row) => sum + row.length, 0);
    const threshold = (this.options.fastPathThreshold || 8) * 2; // Higher threshold for matrices
    const useFastPath = this.options.alwaysUseFastPath || totalElements >= threshold;

    if (useFastPath) {
      // FAST PATH
      return this._createMatrixFast(data);
    } else {
      // SLOW PATH
      this.slowPathOperationsCount++;
      if (this.options.debug) {
        console.log(`[Achronyme] Created matrix via SLOW path (${totalElements} elements < threshold ${threshold})`);
      }
      return this.createFromValue(formatMatrix(data));
    }
  }

  /**
   * Create a complex number value
   * @example
   * const c1 = ach.complex(2, 3);  // 2+3i
   * const c2 = ach.complex(0, 5);  // 5i
   */
  complex(re: number, im: number): AchronymeValue {
    return this.createFromValue(formatComplex({ re, im }));
  }

  // ============================================================================
  // Variables and Lambdas
  // ============================================================================

  /**
   * Create a named variable
   * @example
   * const x = ach.let('myVar', 42);
   * const y = ach.let('signal', [1, 2, 3, 4]);
   */
  let(name: string, value: AchronymeValue | number | number[] | ComplexNumber): AchronymeValue {
    if (!isValidVariableName(name)) {
      throw new AchronymeArgumentError(`Invalid variable name: ${name}`);
    }

    let expr: string;
    if (value instanceof AchronymeValue) {
      expr = value._varName;
    } else if (typeof value === 'number') {
      expr = value.toString();
    } else if (Array.isArray(value)) {
      expr = formatVector(value);
    } else {
      expr = formatValue(value);
    }

    this._eval(`let ${name} = ${expr}`);
    this.variables.add(name);
    return new AchronymeValue(this, name);
  }

  /**
   * Get a reference to an existing variable
   * @example
   * ach.let('x', 10);
   * const xRef = ach.get('x');
   * console.log(await xRef.toNumber()); // 10
   */
  get(name: string): AchronymeValue {
    if (!this.variables.has(name)) {
      // Variable might exist but not tracked - try to access it anyway
      if (this.options.debug) {
        console.warn(`[Achronyme] Accessing untracked variable: ${name}`);
      }
    }
    return new AchronymeValue(this, name);
  }

  /**
   * Create a lambda function
   * @example
   * const square = ach.lambda(['x'], 'x ^ 2');
   * const add = ach.lambda(['a', 'b'], 'a + b');
   */
  lambda(params: string[], body: string): AchronymeValue {
    if (!Array.isArray(params) || params.length === 0) {
      throw new AchronymeArgumentError('lambda() requires at least one parameter');
    }

    const paramList = params.join(', ');
    const lambdaExpr = `${paramList} => ${body}`;
    return this._createFromExpression(lambdaExpr);
  }

  // ============================================================================
  // Mathematical Functions (Basic)
  // ============================================================================

  /**
   * Sine function
   */
  sin(x: AchronymeValue | number): AchronymeValue {
    // Fast path: si x tiene handle, usar sin_fast
    if (typeof x !== 'number' && (x as any)._handle !== undefined) {
      const handle = (x as any)._handle;
      const resultHandle = this.module!.sin_fast(handle);
      return new AchronymeValue(this, `__sin_${handle}`, resultHandle);
    }

    // Slow path: usar parser
    const arg = typeof x === 'number' ? x.toString() : x._varName;
    return this._createFromExpression(`sin(${arg})`);
  }

  /**
   * Cosine function
   */
  cos(x: AchronymeValue | number): AchronymeValue {
    // Fast path: si x tiene handle, usar cos_fast
    if (typeof x !== 'number' && (x as any)._handle !== undefined) {
      const handle = (x as any)._handle;
      const resultHandle = this.module!.cos_fast(handle);
      return new AchronymeValue(this, `__cos_${handle}`, resultHandle);
    }

    // Slow path: usar parser
    const arg = typeof x === 'number' ? x.toString() : x._varName;
    return this._createFromExpression(`cos(${arg})`);
  }

  /**
   * Tangent function
   */
  tan(x: AchronymeValue | number): AchronymeValue {
    // Fast path: si x tiene handle, usar tan_fast
    if (typeof x !== 'number' && (x as any)._handle !== undefined) {
      const handle = (x as any)._handle;
      const resultHandle = this.module!.tan_fast(handle);
      return new AchronymeValue(this, `__tan_${handle}`, resultHandle);
    }

    // Slow path: usar parser
    const arg = typeof x === 'number' ? x.toString() : x._varName;
    return this._createFromExpression(`tan(${arg})`);
  }

  /**
   * Arcsine function
   */
  asin(x: AchronymeValue | number): AchronymeValue {
    const arg = typeof x === 'number' ? x.toString() : x._varName;
    return this._createFromExpression(`asin(${arg})`);
  }

  /**
   * Arccosine function
   */
  acos(x: AchronymeValue | number): AchronymeValue {
    const arg = typeof x === 'number' ? x.toString() : x._varName;
    return this._createFromExpression(`acos(${arg})`);
  }

  /**
   * Arctangent function
   */
  atan(x: AchronymeValue | number): AchronymeValue {
    const arg = typeof x === 'number' ? x.toString() : x._varName;
    return this._createFromExpression(`atan(${arg})`);
  }

  /**
   * Two-argument arctangent
   */
  atan2(y: AchronymeValue | number, x: AchronymeValue | number): AchronymeValue {
    const yArg = typeof y === 'number' ? y.toString() : y._varName;
    const xArg = typeof x === 'number' ? x.toString() : x._varName;
    return this._createFromExpression(`atan2(${yArg}, ${xArg})`);
  }

  /**
   * Hyperbolic sine
   */
  sinh(x: AchronymeValue | number): AchronymeValue {
    const arg = typeof x === 'number' ? x.toString() : x._varName;
    return this._createFromExpression(`sinh(${arg})`);
  }

  /**
   * Hyperbolic cosine
   */
  cosh(x: AchronymeValue | number): AchronymeValue {
    const arg = typeof x === 'number' ? x.toString() : x._varName;
    return this._createFromExpression(`cosh(${arg})`);
  }

  /**
   * Hyperbolic tangent
   */
  tanh(x: AchronymeValue | number): AchronymeValue {
    const arg = typeof x === 'number' ? x.toString() : x._varName;
    return this._createFromExpression(`tanh(${arg})`);
  }

  /**
   * Square root
   */
  sqrt(x: AchronymeValue | number): AchronymeValue {
    // Fast path: si x tiene handle, usar sqrt_fast
    if (typeof x !== 'number' && (x as any)._handle !== undefined) {
      const handle = (x as any)._handle;
      const resultHandle = this.module!.sqrt_fast(handle);
      return new AchronymeValue(this, `__sqrt_${handle}`, resultHandle);
    }

    // Slow path: usar parser
    const arg = typeof x === 'number' ? x.toString() : x._varName;
    return this._createFromExpression(`sqrt(${arg})`);
  }

  /**
   * Cube root
   */
  cbrt(x: AchronymeValue | number): AchronymeValue {
    const arg = typeof x === 'number' ? x.toString() : x._varName;
    return this._createFromExpression(`cbrt(${arg})`);
  }

  /**
   * Exponential (e^x)
   */
  exp(x: AchronymeValue | number): AchronymeValue {
    // Fast path: si x tiene handle, usar exp_fast
    if (typeof x !== 'number' && (x as any)._handle !== undefined) {
      const handle = (x as any)._handle;
      const resultHandle = this.module!.exp_fast(handle);
      return new AchronymeValue(this, `__exp_${handle}`, resultHandle);
    }

    // Slow path: usar parser
    const arg = typeof x === 'number' ? x.toString() : x._varName;
    return this._createFromExpression(`exp(${arg})`);
  }

  /**
   * Natural logarithm
   */
  ln(x: AchronymeValue | number): AchronymeValue {
    // Fast path: si x tiene handle, usar ln_fast
    if (typeof x !== 'number' && (x as any)._handle !== undefined) {
      const handle = (x as any)._handle;
      const resultHandle = this.module!.ln_fast(handle);
      return new AchronymeValue(this, `__ln_${handle}`, resultHandle);
    }

    // Slow path: usar parser
    const arg = typeof x === 'number' ? x.toString() : x._varName;
    return this._createFromExpression(`ln(${arg})`);
  }

  /**
   * Common logarithm (base 10)
   */
  log(x: AchronymeValue | number): AchronymeValue {
    const arg = typeof x === 'number' ? x.toString() : x._varName;
    return this._createFromExpression(`log(${arg})`);
  }

  /**
   * Base-10 logarithm
   */
  log10(x: AchronymeValue | number): AchronymeValue {
    return this.log(x);
  }

  /**
   * Base-2 logarithm
   */
  log2(x: AchronymeValue | number): AchronymeValue {
    const arg = typeof x === 'number' ? x.toString() : x._varName;
    return this._createFromExpression(`log2(${arg})`);
  }

  /**
   * Power function
   */
  pow(base: AchronymeValue | number, exponent: AchronymeValue | number): AchronymeValue {
    const baseArg = typeof base === 'number' ? base.toString() : base._varName;
    const expArg = typeof exponent === 'number' ? exponent.toString() : exponent._varName;
    return this._createFromExpression(`pow(${baseArg}, ${expArg})`);
  }

  /**
   * Absolute value
   */
  abs(x: AchronymeValue | number): AchronymeValue {
    // Fast path: si x tiene handle, usar abs_fast
    if (typeof x !== 'number' && (x as any)._handle !== undefined) {
      const handle = (x as any)._handle;
      const resultHandle = this.module!.abs_fast(handle);
      return new AchronymeValue(this, `__abs_${handle}`, resultHandle);
    }

    // Slow path: usar parser
    const arg = typeof x === 'number' ? x.toString() : x._varName;
    return this._createFromExpression(`abs(${arg})`);
  }

  /**
   * Sign function (-1, 0, or 1)
   */
  sign(x: AchronymeValue | number): AchronymeValue {
    const arg = typeof x === 'number' ? x.toString() : x._varName;
    return this._createFromExpression(`sign(${arg})`);
  }

  /**
   * Floor function
   */
  floor(x: AchronymeValue | number): AchronymeValue {
    const arg = typeof x === 'number' ? x.toString() : x._varName;
    return this._createFromExpression(`floor(${arg})`);
  }

  /**
   * Ceiling function
   */
  ceil(x: AchronymeValue | number): AchronymeValue {
    const arg = typeof x === 'number' ? x.toString() : x._varName;
    return this._createFromExpression(`ceil(${arg})`);
  }

  /**
   * Round function
   */
  round(x: AchronymeValue | number): AchronymeValue {
    const arg = typeof x === 'number' ? x.toString() : x._varName;
    return this._createFromExpression(`round(${arg})`);
  }

  /**
   * Truncate function
   */
  trunc(x: AchronymeValue | number): AchronymeValue {
    const arg = typeof x === 'number' ? x.toString() : x._varName;
    return this._createFromExpression(`trunc(${arg})`);
  }

  /**
   * Minimum value - works with both variadic scalars and vectors
   * @example
   * ach.min(1, 5, 3) // 1 (variadic scalars)
   * const v = ach.vector([1, 5, 3]);
   * ach.min(v) // 1 (vector)
   */
  min(...values: (AchronymeValue | number)[]): AchronymeValue {
    if (values.length === 0) {
      throw new AchronymeArgumentError('min() requires at least one argument');
    }
    const args = values.map(v => typeof v === 'number' ? v.toString() : v._varName).join(', ');
    return this._createFromExpression(`min(${args})`);
  }

  /**
   * Maximum value - works with both variadic scalars and vectors
   * @example
   * ach.max(1, 5, 3) // 5 (variadic scalars)
   * const v = ach.vector([1, 5, 3]);
   * ach.max(v) // 5 (vector)
   */
  max(...values: (AchronymeValue | number)[]): AchronymeValue {
    if (values.length === 0) {
      throw new AchronymeArgumentError('max() requires at least one argument');
    }
    const args = values.map(v => typeof v === 'number' ? v.toString() : v._varName).join(', ');
    return this._createFromExpression(`max(${args})`);
  }

  // ============================================================================
  // DSP Functions
  // ============================================================================

  /**
   * Fast Fourier Transform
   * OPTIMIZED: Uses fast path for handle-based values
   * @example
   * const signal = ach.vector([1, 2, 3, 4, 5, 6, 7, 8]);
   * const spectrum = ach.fft(signal);
   */
  fft(signal: AchronymeValue): AchronymeValue {
    if (!this.module) throw new AchronymeNotInitializedError();

    // Check if signal has a handle (was created via fast path)
    const inputHandle = this.varToHandle.get(signal._varName);

    if (inputHandle !== undefined) {
      // FAST PATH: Operate directly on handles
      try {
        const resultHandle = this.module.fft_fast(inputHandle);
        this.fastPathOperationsCount++;

        if (this.options.debug) {
          console.log(`[Achronyme] FFT via FAST path (handle ${inputHandle} -> ${resultHandle})`);
        }

        return this._createFromHandle(resultHandle);
      } catch (e: any) {
        // Fallback to slow path on error
        if (this.options.debug) {
          console.warn(`[Achronyme] FFT fast path failed, falling back to slow path: ${e.message}`);
        }
      }
    }

    // SLOW PATH: Expression-based
    this.slowPathOperationsCount++;
    if (this.options.debug) {
      console.log(`[Achronyme] FFT via SLOW path (no handle for ${signal._varName})`);
    }
    return this._createFromExpression(`fft(${signal._varName})`);
  }

  /**
   * FFT Magnitude (absolute value of complex FFT result)
   * OPTIMIZED: Uses fast path for handle-based values
   */
  fft_mag(signal: AchronymeValue): AchronymeValue {
    if (!this.module) throw new AchronymeNotInitializedError();

    const inputHandle = this.varToHandle.get(signal._varName);

    if (inputHandle !== undefined) {
      try {
        const resultHandle = this.module.fft_mag_fast(inputHandle);
        this.fastPathOperationsCount++;

        if (this.options.debug) {
          console.log(`[Achronyme] FFT_MAG via FAST path`);
        }

        return this._createFromHandle(resultHandle);
      } catch (e: any) {
        if (this.options.debug) {
          console.warn(`[Achronyme] FFT_MAG fast path failed: ${e.message}`);
        }
      }
    }

    this.slowPathOperationsCount++;
    return this._createFromExpression(`fft_mag(${signal._varName})`);
  }

  /**
   * FFT Phase spectrum
   * Returns vector of phases: arg(X[k])
   * Optimized for performance - avoids JS overhead of atan2 mapping
   */
  fft_phase(signal: AchronymeValue): AchronymeValue {
    return this._createFromExpression(`fft_phase(${signal._varName})`);
  }

  /**
   * Inverse Fast Fourier Transform
   */
  ifft(spectrum: AchronymeValue): AchronymeValue {
    return this._createFromExpression(`ifft(${spectrum._varName})`);
  }

  /**
   * Discrete Fourier Transform
   */
  dft(signal: AchronymeValue): AchronymeValue {
    return this._createFromExpression(`dft(${signal._varName})`);
  }

  /**
   * DFT Magnitude
   */
  dft_mag(signal: AchronymeValue): AchronymeValue {
    return this._createFromExpression(`dft_mag(${signal._varName})`);
  }

  /**
   * DFT Phase
   */
  dft_phase(signal: AchronymeValue): AchronymeValue {
    return this._createFromExpression(`dft_phase(${signal._varName})`);
  }

  /**
   * Convolution (direct method)
   * @example
   * const sig1 = ach.vector([1, 2, 3]);
   * const sig2 = ach.vector([1, 1]);
   * const result = ach.conv(sig1, sig2);
   */
  conv(signal1: AchronymeValue, signal2: AchronymeValue): AchronymeValue {
    return this._createFromExpression(`conv(${signal1._varName}, ${signal2._varName})`);
  }

  /**
   * Convolution using FFT (faster for large signals)
   */
  conv_fft(signal1: AchronymeValue, signal2: AchronymeValue): AchronymeValue {
    return this._createFromExpression(`conv_fft(${signal1._varName}, ${signal2._varName})`);
  }

  /**
   * Hanning window function
   * @param n - Window size
   */
  hanning(n: number): AchronymeValue {
    return this._createFromExpression(`hanning(${n})`);
  }

  /**
   * Hamming window function
   * @param n - Window size
   */
  hamming(n: number): AchronymeValue {
    return this._createFromExpression(`hamming(${n})`);
  }

  /**
   * Blackman window function
   * @param n - Window size
   */
  blackman(n: number): AchronymeValue {
    return this._createFromExpression(`blackman(${n})`);
  }

  // ============================================================================
  // Optimization Functions (Reduce JS-WASM Overhead)
  // ============================================================================

  /**
   * Generate linearly spaced vector
   *
   * Generates N evenly spaced samples from start to end (inclusive).
   * This is MUCH faster than generating the array in JS with a loop!
   * ALWAYS uses fast path (no parsing).
   *
   * @param start - Starting value
   * @param end - Ending value
   * @param n - Number of samples
   * @returns Vector with N evenly spaced values
   *
   * @example
   * const t = ach.linspace(0, 10, 100); // 100 samples from 0 to 10
   * const samples = await t.toVector();
   */
  linspace(start: number, end: number, n: number): AchronymeValue {
    if (!this.module) throw new AchronymeNotInitializedError();

    try {
      // ALWAYS use fast path for linspace (it's meant to be fast!)
      const resultHandle = this.module.linspace_fast(start, end, n);
      this.fastPathOperationsCount++;

      if (this.options.debug) {
        console.log(`[Achronyme] Linspace via FAST path (${n} points)`);
      }

      return this._createFromHandle(resultHandle);
    } catch (e: any) {
      // Fallback to slow path
      if (this.options.debug) {
        console.warn(`[Achronyme] Linspace fast path failed: ${e.message}`);
      }
      this.slowPathOperationsCount++;
      return this._createFromExpression(`linspace(${start}, ${end}, ${n})`);
    }
  }

  /**
   * Reorder FFT output to center zero frequency
   *
   * Shifts zero-frequency component to center of spectrum.
   * Common in spectral analysis for visualization.
   *
   * @param vector - FFT output vector
   * @returns Shifted vector with zero frequency at center
   *
   * @example
   * const spectrum = ach.fft_mag(signal);
   * const centered = ach.fftshift(spectrum);
   */
  fftshift(vector: AchronymeValue): AchronymeValue {
    return this._createFromExpression(`fftshift(${vector._varName})`);
  }

  /**
   * Inverse of fftshift - undoes the shift operation
   *
   * @param vector - fftshift'd vector
   * @returns Original ordering
   */
  ifftshift(vector: AchronymeValue): AchronymeValue {
    return this._createFromExpression(`ifftshift(${vector._varName})`);
  }

  /**
   * All-in-one FFT spectrum analysis (HIGH PERFORMANCE)
   *
   * Computes omega, magnitude, and phase in a SINGLE PASS!
   * This eliminates multiple JS-WASM crossings and achieves ~90% overhead reduction.
   *
   * @param signal - Input signal vector
   * @param fs - Sampling frequency (Hz)
   * @param shift - Apply fftshift to center spectrum (default: true)
   * @param angular - Convert Hz to rad/s (default: true)
   * @param omegaRange - Filter frequencies to [-range, range] (default: no filter)
   * @returns Matrix [N x 3] where each row is [omega, magnitude, phase]
   *
   * @example
   * const signal = ach.vector([...]);
   * const spectrum = ach.fft_spectrum(signal, 1000, true, true, 20);
   * const result = await spectrum.toMatrix();
   * // result[i][0] = omega (rad/s)
   * // result[i][1] = magnitude
   * // result[i][2] = phase
   *
   * // Extract individual components:
   * const omega = result.map(row => row[0]);
   * const magnitude = result.map(row => row[1]);
   * const phase = result.map(row => row[2]);
   */
  fft_spectrum(
    signal: AchronymeValue,
    fs: number,
    shift: boolean = true,
    angular: boolean = true,
    omegaRange?: number
  ): AchronymeValue {
    const shiftVal = shift ? 1 : 0;
    const angularVal = angular ? 1 : 0;
    const rangeVal = omegaRange !== undefined ? omegaRange : -1;
    return this._createFromExpression(
      `fft_spectrum(${signal._varName}, ${fs}, ${shiftVal}, ${angularVal}, ${rangeVal})`
    );
  }

  // ============================================================================
  // Native Statistical Functions (Optimized)
  // ============================================================================

  /**
   * Sum of all elements in a vector (native C++ implementation, much faster than reduce)
   * @example
   * const v = ach.vector([1, 2, 3, 4, 5]);
   * const total = await ach.sum(v).toNumber(); // 15
   */
  sum(arr: AchronymeValue): AchronymeValue {
    return this._createFromExpression(`sum(${arr._varName})`);
  }

  /**
   * Mean (average) of all elements in a vector (native C++ implementation)
   * @example
   * const v = ach.vector([1, 2, 3, 4, 5]);
   * const avg = await ach.mean(v).toNumber(); // 3
   */
  mean(arr: AchronymeValue): AchronymeValue {
    return this._createFromExpression(`mean(${arr._varName})`);
  }

  /**
   * Standard deviation of a vector (native C++ implementation)
   * @example
   * const v = ach.vector([2, 4, 4, 4, 5, 5, 7, 9]);
   * const stdDev = await ach.std(v).toNumber();
   */
  std(arr: AchronymeValue): AchronymeValue {
    return this._createFromExpression(`std(${arr._varName})`);
  }

  // ============================================================================
  // Higher-Order Functions
  // ============================================================================

  /**
   * Map function over a vector
   * @example
   * const v = ach.vector([1, 2, 3, 4]);
   * const squared = ach.map('x => x ^ 2', v);
   * // Or with a lambda:
   * const fn = ach.lambda(['x'], 'x ^ 2');
   * const squared2 = ach.map(fn, v);
   */
  map(fn: string | AchronymeValue, arr: AchronymeValue): AchronymeValue {
    const fnExpr = typeof fn === 'string' ? fn : fn._varName;
    return this._createFromExpression(`map(${fnExpr}, ${arr._varName})`);
  }

  /**
   * Filter function over a vector
   * @example
   * const v = ach.vector([1, 2, 3, 4, 5]);
   * const evens = ach.filter('x => x % 2 == 0', v);
   */
  filter(predicate: string | AchronymeValue, arr: AchronymeValue): AchronymeValue {
    const predExpr = typeof predicate === 'string' ? predicate : predicate._varName;
    return this._createFromExpression(`filter(${predExpr}, ${arr._varName})`);
  }

  /**
   * Reduce function over a vector
   * @example
   * const v = ach.vector([1, 2, 3, 4]);
   * const sum = ach.reduce('a, b => a + b', v, 0);
   */
  reduce(fn: string | AchronymeValue, arr: AchronymeValue, initial: number): AchronymeValue {
    const fnExpr = typeof fn === 'string' ? fn : fn._varName;
    // Core expects: reduce(f, init, collection)
    return this._createFromExpression(`reduce(${fnExpr}, ${initial}, ${arr._varName})`);
  }

  /**
   * Pipe - compose functions left to right
   * @example
   * const double = ach.lambda(['x'], 'x * 2');
   * const addTen = ach.lambda(['x'], 'x + 10');
   * const result = ach.pipe(double, addTen, ach.number(5));
   * // result = (5 * 2) + 10 = 20
   */
  pipe(...fnsAndValue: AchronymeValue[]): AchronymeValue {
    if (fnsAndValue.length < 2) {
      throw new AchronymeArgumentError('pipe() requires at least two arguments');
    }
    const args = fnsAndValue.map(v => v._varName).join(', ');
    return this._createFromExpression(`pipe(${args})`);
  }

  /**
   * Compose - compose functions right to left
   */
  compose(...fns: AchronymeValue[]): AchronymeValue {
    if (fns.length === 0) {
      throw new AchronymeArgumentError('compose() requires at least one argument');
    }
    const args = fns.map(v => v._varName).join(', ');
    return this._createFromExpression(`compose(${args})`);
  }

  // ============================================================================
  // Vector/Matrix Operations
  // ============================================================================

  /**
   * Dot product of two vectors
   */
  dot(v1: AchronymeValue, v2: AchronymeValue): AchronymeValue {
    return this._createFromExpression(`dot(${v1._varName}, ${v2._varName})`);
  }

  /**
   * Cross product of two vectors
   */
  cross(v1: AchronymeValue, v2: AchronymeValue): AchronymeValue {
    return this._createFromExpression(`cross(${v1._varName}, ${v2._varName})`);
  }

  /**
   * Norm (magnitude) of a vector
   */
  norm(v: AchronymeValue): AchronymeValue {
    return this._createFromExpression(`norm(${v._varName})`);
  }

  /**
   * Native vector addition (element-wise) - much faster than v1 + v2
   * @example
   * const v1 = ach.vector([1, 2, 3]);
   * const v2 = ach.vector([4, 5, 6]);
   * const result = ach.vadd(v1, v2); // [5, 7, 9]
   */
  vadd(v1: AchronymeValue, v2: AchronymeValue): AchronymeValue {
    return this._createFromExpression(`vadd(${v1._varName}, ${v2._varName})`);
  }

  /**
   * Native vector subtraction (element-wise) - much faster than v1 - v2
   */
  vsub(v1: AchronymeValue, v2: AchronymeValue): AchronymeValue {
    return this._createFromExpression(`vsub(${v1._varName}, ${v2._varName})`);
  }

  /**
   * Native vector multiplication (element-wise) - much faster than v1 * v2
   */
  vmul(v1: AchronymeValue, v2: AchronymeValue): AchronymeValue {
    return this._createFromExpression(`vmul(${v1._varName}, ${v2._varName})`);
  }

  /**
   * Native vector division (element-wise) - much faster than v1 / v2
   */
  vdiv(v1: AchronymeValue, v2: AchronymeValue): AchronymeValue {
    return this._createFromExpression(`vdiv(${v1._varName}, ${v2._varName})`);
  }

  /**
   * Native vector scaling - multiply vector by scalar
   */
  vscale(v: AchronymeValue, scalar: number): AchronymeValue {
    return this._createFromExpression(`vscale(${v._varName}, ${scalar})`);
  }

  /**
   * Transpose a matrix
   */
  transpose(m: AchronymeValue): AchronymeValue {
    return this._createFromExpression(`transpose(${m._varName})`);
  }

  /**
   * Determinant of a matrix
   */
  det(m: AchronymeValue): AchronymeValue {
    return this._createFromExpression(`det(${m._varName})`);
  }

  /**
   * Inverse of a matrix
   */
  inverse(m: AchronymeValue): AchronymeValue {
    return this._createFromExpression(`inverse(${m._varName})`);
  }

  // ============================================================================
  // Linear Algebra - Advanced Matrix Decompositions (v0.4.0)
  // ============================================================================

  /**
   * LU Decomposition with partial pivoting: PA = LU
   *
   * Factorizes matrix A into:
   *   P × A = L × U
   * where:
   *   - P is a permutation matrix
   *   - L is lower triangular with 1s on diagonal
   *   - U is upper triangular
   *
   * Algorithm: Gaussian elimination with partial pivoting
   * Complexity: O(n³)
   *
   * @param matrix Square matrix to decompose
   * @returns Object with L, U, P matrices
   * @throws Error if matrix is singular or not square
   *
   * @example
   * const A = ach.matrix([[4, 3], [6, 3]]);
   * const { L, U, P } = ach.lu(A);
   * // Verify: P * A == L * U
   */
  lu(matrix: AchronymeValue): LUResult {
    if (!this.module) throw new AchronymeNotInitializedError();

    // Check if matrix has a handle (fast path)
    const matrixHandle = this.varToHandle.get(matrix._varName);

    if (matrixHandle !== undefined) {
      try {
        const result = this.module.lu_decomposition_js(matrixHandle);

        const L = this._createFromHandle(result.L);
        const U = this._createFromHandle(result.U);
        const P = this._createFromHandle(result.P);

        if (this.options.debug) {
          console.log(`[Achronyme] LU decomposition via FAST path`);
        }

        return { L, U, P };
      } catch (e: any) {
        throw wrapCppError(`LU decomposition failed: ${e.message || e}`);
      }
    }

    // Slow path: not supported for decompositions (require matrix handles)
    throw new AchronymeArgumentError('LU decomposition requires a matrix created via ach.matrix()');
  }

  /**
   * QR Decomposition: A = QR
   *
   * Factorizes matrix A into:
   *   A = Q × R
   * where:
   *   - Q is orthogonal (Q^T × Q = I)
   *   - R is upper triangular
   *
   * Algorithm: Householder reflections
   * Complexity: O(mn²) for m×n matrix
   *
   * @param matrix Matrix to decompose (m×n, m >= n)
   * @returns Object with Q and R matrices
   *
   * @example
   * const A = ach.matrix([[12, -51, 4], [6, 167, -68], [-4, 24, -41]]);
   * const { Q, R } = ach.qr(A);
   * // Verify: A == Q * R
   */
  qr(matrix: AchronymeValue): QRResult {
    if (!this.module) throw new AchronymeNotInitializedError();

    const matrixHandle = this.varToHandle.get(matrix._varName);

    if (matrixHandle !== undefined) {
      try {
        const result = this.module.qr_decomposition_js(matrixHandle);

        const Q = this._createFromHandle(result.Q);
        const R = this._createFromHandle(result.R);

        if (this.options.debug) {
          console.log(`[Achronyme] QR decomposition via FAST path`);
        }

        return { Q, R };
      } catch (e: any) {
        throw wrapCppError(`QR decomposition failed: ${e.message || e}`);
      }
    }

    throw new AchronymeArgumentError('QR decomposition requires a matrix created via ach.matrix()');
  }

  /**
   * Cholesky Decomposition: A = L×L^T
   *
   * Factorizes symmetric positive definite matrix A into:
   *   A = L × L^T
   * where:
   *   - L is lower triangular
   *
   * Algorithm: Cholesky-Banachiewicz
   * Complexity: O(n³/3) - Faster than LU
   * Requirements: A must be symmetric and positive definite
   *
   * @param matrix Symmetric positive definite matrix
   * @returns Lower triangular matrix L
   * @throws Error if matrix is not positive definite
   *
   * @example
   * const A = ach.matrix([[4, 12, -16], [12, 37, -43], [-16, -43, 98]]);
   * const L = ach.cholesky(A);
   * // Verify: A == L * L^T
   */
  cholesky(matrix: AchronymeValue): AchronymeValue {
    if (!this.module) throw new AchronymeNotInitializedError();

    const matrixHandle = this.varToHandle.get(matrix._varName);

    if (matrixHandle !== undefined) {
      try {
        const resultHandle = this.module.cholesky_decomposition_js(matrixHandle);

        if (this.options.debug) {
          console.log(`[Achronyme] Cholesky decomposition via FAST path`);
        }

        return this._createFromHandle(resultHandle);
      } catch (e: any) {
        throw wrapCppError(`Cholesky decomposition failed: ${e.message || e}`);
      }
    }

    throw new AchronymeArgumentError('Cholesky decomposition requires a matrix created via ach.matrix()');
  }

  /**
   * Singular Value Decomposition: A = UΣV^T
   *
   * Factorizes matrix A into:
   *   A = U × Σ × V^T
   * where:
   *   - U: m×m orthogonal (left singular vectors)
   *   - Σ: m×n diagonal (singular values, non-negative)
   *   - V: n×n orthogonal (right singular vectors)
   *
   * Algorithm: Golub-Reinsch with bidiagonalization
   * Complexity: O(min(m,n)² × max(m,n))
   *
   * Applications:
   *   - Principal Component Analysis (PCA)
   *   - Data compression
   *   - Pseudoinverse computation
   *   - Low-rank approximation
   *
   * @param matrix Matrix to decompose (m×n)
   * @returns Object with U, S (singular values as vector), V
   *
   * @example
   * const A = ach.matrix([[1, 2], [3, 4], [5, 6]]);
   * const { U, S, V } = ach.svd(A);
   */
  svd(matrix: AchronymeValue): SVDResult {
    if (!this.module) throw new AchronymeNotInitializedError();

    const matrixHandle = this.varToHandle.get(matrix._varName);

    if (matrixHandle !== undefined) {
      try {
        const result = this.module.svd_decomposition_js(matrixHandle);

        const U = this._createFromHandle(result.U);
        const S = this._createFromHandle(result.S);
        const V = this._createFromHandle(result.V);

        if (this.options.debug) {
          console.log(`[Achronyme] SVD via FAST path`);
        }

        return { U, S, V };
      } catch (e: any) {
        throw wrapCppError(`SVD failed: ${e.message || e}`);
      }
    }

    throw new AchronymeArgumentError('SVD requires a matrix created via ach.matrix()');
  }

  /**
   * Check if a matrix is symmetric
   *
   * @param matrix Matrix to check
   * @param tol Tolerance for symmetry check (default: 1e-12)
   * @returns true if A[i,j] == A[j,i] within tolerance
   *
   * @example
   * const A = ach.matrix([[1, 2], [2, 1]]);
   * const symmetric = ach.isSymmetric(A); // true
   */
  isSymmetric(matrix: AchronymeValue, tol: number = 1e-12): boolean {
    if (!this.module) throw new AchronymeNotInitializedError();

    const matrixHandle = this.varToHandle.get(matrix._varName);

    if (matrixHandle !== undefined) {
      try {
        return this.module.is_symmetric_js(matrixHandle, tol);
      } catch (e: any) {
        throw wrapCppError(`isSymmetric failed: ${e.message || e}`);
      }
    }

    throw new AchronymeArgumentError('isSymmetric requires a matrix created via ach.matrix()');
  }

  /**
   * Check if a matrix is positive definite
   *
   * Uses Sylvester's criterion (all leading principal minors > 0)
   *
   * @param matrix Matrix to check
   * @returns true if matrix is positive definite
   *
   * @example
   * const A = ach.matrix([[2, -1], [-1, 2]]);
   * const pd = ach.isPositiveDefinite(A); // true
   */
  isPositiveDefinite(matrix: AchronymeValue): boolean {
    if (!this.module) throw new AchronymeNotInitializedError();

    const matrixHandle = this.varToHandle.get(matrix._varName);

    if (matrixHandle !== undefined) {
      try {
        return this.module.is_positive_definite_js(matrixHandle);
      } catch (e: any) {
        throw wrapCppError(`isPositiveDefinite failed: ${e.message || e}`);
      }
    }

    throw new AchronymeArgumentError('isPositiveDefinite requires a matrix created via ach.matrix()');
  }

  /**
   * Create identity matrix of size n×n
   *
   * @param n Size of identity matrix
   * @returns n×n identity matrix (1s on diagonal, 0s elsewhere)
   *
   * @example
   * const I = ach.identity(3);
   * // [[1, 0, 0],
   * //  [0, 1, 0],
   * //  [0, 0, 1]]
   */
  identity(n: number): AchronymeValue {
    if (!this.module) throw new AchronymeNotInitializedError();

    try {
      const resultHandle = this.module.identity_js(n);

      if (this.options.debug) {
        console.log(`[Achronyme] Created identity matrix ${n}×${n}`);
      }

      return this._createFromHandle(resultHandle);
    } catch (e: any) {
      throw wrapCppError(`identity failed: ${e.message || e}`);
    }
  }

  // ============================================================================
  // Eigenvalue and Eigenvector Solvers (v0.4.0)
  // ============================================================================

  /**
   * Power Iteration - Find dominant eigenvalue and eigenvector
   *
   * Iteratively computes the largest eigenvalue (by magnitude) and its
   * corresponding eigenvector using the power iteration method.
   *
   * Algorithm: Power iteration with normalization
   * Complexity: O(n² × iterations)
   * Convergence: Linear (depends on eigenvalue separation)
   *
   * @param matrix Square matrix
   * @param maxIterations Maximum number of iterations (default: 1000)
   * @param tolerance Convergence tolerance (default: 1e-10)
   * @returns Object with dominant eigenvalue and eigenvector
   *
   * @example
   * const A = ach.matrix([[4, 1], [2, 3]]);
   * const { eigenvalue, eigenvector } = ach.powerIteration(A);
   * console.log(eigenvalue); // ~5.372
   * console.log(await eigenvector.toVector());
   */
  powerIteration(
    matrix: AchronymeValue,
    maxIterations: number = 1000,
    tolerance: number = 1e-10
  ): PowerIterationResult {
    if (!this.module) throw new AchronymeNotInitializedError();

    const matrixHandle = this.varToHandle.get(matrix._varName);

    if (matrixHandle !== undefined) {
      try {
        const result = this.module.power_iteration_js(matrixHandle, maxIterations, tolerance);

        const eigenvalue = result.eigenvalue as number;
        const eigenvector = this._createFromHandle(result.eigenvector as Handle);

        if (this.options.debug) {
          console.log(`[Achronyme] Power iteration: eigenvalue = ${eigenvalue}`);
        }

        return { eigenvalue, eigenvector };
      } catch (e: any) {
        throw wrapCppError(`Power iteration failed: ${e.message || e}`);
      }
    }

    throw new AchronymeArgumentError('powerIteration requires a matrix created via ach.matrix()');
  }

  /**
   * Compute all eigenvalues using QR algorithm
   *
   * Uses iterative QR decomposition to find all eigenvalues of a matrix.
   * Works best for symmetric or nearly symmetric matrices.
   *
   * Algorithm: QR iteration
   * Complexity: O(n³ × iterations)
   *
   * @param matrix Square matrix
   * @param maxIterations Maximum iterations (default: 1000)
   * @param tolerance Convergence tolerance (default: 1e-10)
   * @returns Vector of eigenvalues
   *
   * Note: For non-symmetric matrices, may return real parts of complex eigenvalues
   *
   * @example
   * const A = ach.matrix([[4, 1], [2, 3]]);
   * const eigenvalues = ach.eigenvalues(A);
   * console.log(await eigenvalues.toVector()); // [~5.372, ~1.628]
   */
  eigenvalues(
    matrix: AchronymeValue,
    maxIterations: number = 1000,
    tolerance: number = 1e-10
  ): AchronymeValue {
    if (!this.module) throw new AchronymeNotInitializedError();

    const matrixHandle = this.varToHandle.get(matrix._varName);

    if (matrixHandle !== undefined) {
      try {
        const resultHandle = this.module.qr_eigenvalues_js(matrixHandle, maxIterations, tolerance);

        if (this.options.debug) {
          console.log(`[Achronyme] Computed eigenvalues via QR algorithm`);
        }

        return this._createFromHandle(resultHandle);
      } catch (e: any) {
        throw wrapCppError(`Eigenvalues computation failed: ${e.message || e}`);
      }
    }

    throw new AchronymeArgumentError('eigenvalues requires a matrix created via ach.matrix()');
  }

  /**
   * Compute eigenvalues and eigenvectors for symmetric matrices
   *
   * Uses QR algorithm with accumulation to compute both eigenvalues and
   * eigenvectors. Only works reliably for symmetric matrices.
   *
   * Algorithm: QR algorithm with eigenvector accumulation
   * Complexity: O(n³ × iterations)
   * Requirements: Matrix must be symmetric
   *
   * @param matrix Symmetric square matrix
   * @param maxIterations Maximum iterations (default: 1000)
   * @param tolerance Convergence tolerance (default: 1e-10)
   * @returns Object with eigenvalues (vector) and eigenvectors (matrix columns)
   *
   * @example
   * const A = ach.matrix([[2, 1], [1, 2]]); // Symmetric
   * const { eigenvalues, eigenvectors } = ach.eig(A);
   * console.log(await eigenvalues.toVector()); // [3, 1]
   * console.log(await eigenvectors.toMatrix());
   * // Each column is an eigenvector
   */
  eig(
    matrix: AchronymeValue,
    maxIterations: number = 1000,
    tolerance: number = 1e-10
  ): EigenResult {
    if (!this.module) throw new AchronymeNotInitializedError();

    const matrixHandle = this.varToHandle.get(matrix._varName);

    if (matrixHandle !== undefined) {
      try {
        const result = this.module.eigen_symmetric_js(matrixHandle, maxIterations, tolerance);

        const eigenvalues = this._createFromHandle(result.eigenvalues as Handle);
        const eigenvectors = this._createFromHandle(result.eigenvectors as Handle);

        if (this.options.debug) {
          console.log(`[Achronyme] Computed eigenvalues and eigenvectors`);
        }

        return { eigenvalues, eigenvectors };
      } catch (e: any) {
        throw wrapCppError(`Eigen decomposition failed: ${e.message || e}`);
      }
    }

    throw new AchronymeArgumentError('eig requires a symmetric matrix created via ach.matrix()');
  }

  // ============================================================================
  // Constants
  // ============================================================================

  /**
   * Get PI constant
   */
  get PI(): AchronymeValue {
    return this._createFromExpression('PI');
  }

  /**
   * Get E constant (Euler's number)
   */
  get E(): AchronymeValue {
    return this._createFromExpression('E');
  }

  /**
   * Get PHI constant (Golden ratio)
   */
  get PHI(): AchronymeValue {
    return this._createFromExpression('PHI');
  }

  /**
   * Get TAU constant (2*PI)
   */
  get TAU(): AchronymeValue {
    return this._createFromExpression('TAU');
  }

  // ============================================================================
  // Raw Eval (Advanced Usage)
  // ============================================================================

  /**
   * Direct evaluation of an expression (advanced usage)
   * Returns a raw string result
   * @example
   * const result = ach.eval('sin(PI / 4) * 2');
   */
  eval(expression: string): string {
    return this._eval(expression);
  }

  /**
   * Evaluate an expression and wrap result in AchronymeValue
   * @example
   * const result = ach.evalValue('sin(PI / 4) * 2');
   * console.log(await result.toNumber());
   */
  evalValue(expression: string): AchronymeValue {
    return this._createFromExpression(expression);
  }
}
