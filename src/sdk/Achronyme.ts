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
    const heap = new Float64Array(
      this.module.HEAPF64.buffer,
      ptr,
      data.length
    );

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
      return new Float64Array(
        this.module.HEAPF64.buffer,
        dataPtr,
        length
      );
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
    const arg = typeof x === 'number' ? x.toString() : x._varName;
    return this._createFromExpression(`sin(${arg})`);
  }

  /**
   * Cosine function
   */
  cos(x: AchronymeValue | number): AchronymeValue {
    const arg = typeof x === 'number' ? x.toString() : x._varName;
    return this._createFromExpression(`cos(${arg})`);
  }

  /**
   * Tangent function
   */
  tan(x: AchronymeValue | number): AchronymeValue {
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
    const arg = typeof x === 'number' ? x.toString() : x._varName;
    return this._createFromExpression(`exp(${arg})`);
  }

  /**
   * Natural logarithm
   */
  ln(x: AchronymeValue | number): AchronymeValue {
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
