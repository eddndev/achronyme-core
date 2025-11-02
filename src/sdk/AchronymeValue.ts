/**
 * AchronymeValue - Proxy for values stored in the Achronyme Environment
 *
 * This class represents a value stored in the C++ Environment and provides
 * a fluent API for chaining operations. Each instance corresponds to a
 * variable in the C++ runtime (e.g., __v0, __v1, etc.).
 *
 * IMPORTANT: Call dispose() when done to clean up memory in the C++ Environment.
 */

import type { Achronyme } from './Achronyme.js';
import { ComplexNumber, ValueMetadata, AchronymeValueType, Handle } from './types.js';
import { AchronymeDisposedError, wrapCppError } from './errors.js';
import { parseResult, parseComplex, parseVector, parseMatrix, detectType } from './utils.js';

export class AchronymeValue {
  private _disposed: boolean = false;
  private readonly _metadata: ValueMetadata;
  private readonly _handle?: Handle; // Optional handle for fast path

  /**
   * @internal
   * Constructor should only be called by Achronyme class
   */
  constructor(
    private readonly ach: Achronyme,
    private readonly varName: string,
    handle?: Handle
  ) {
    this._metadata = {
      varName,
      type: 'unknown',
      disposed: false,
      createdAt: Date.now(),
      handle,
      usedFastPath: handle !== undefined,
    };
    this._handle = handle;
  }

  // ============================================================================
  // Memory Management
  // ============================================================================

  /**
   * Dispose this value and free memory in the C++ Environment.
   *
   * After calling dispose(), this value cannot be used anymore.
   * Any subsequent operations will throw AchronymeDisposedError.
   *
   * @example
   * const x = ach.vector([1, 2, 3]);
   * const y = x.add(5);
   * x.dispose(); // Clean up x
   * y.dispose(); // Clean up y
   */
  dispose(): void {
    if (this._disposed) return;

    try {
      // Tell Achronyme to remove this variable from tracking
      (this.ach as any)._disposeVariable(this.varName);
      this._disposed = true;
      this._metadata.disposed = true;
    } catch (e) {
      // Ignore errors during disposal
    }
  }

  /**
   * Check if this value has been disposed
   */
  isDisposed(): boolean {
    return this._disposed;
  }

  /**
   * Get metadata about this value
   */
  getMetadata(): Readonly<ValueMetadata> {
    return { ...this._metadata };
  }

  /**
   * @internal
   * Check if disposed and throw error if so
   */
  private checkDisposed(): void {
    if (this._disposed) {
      throw new AchronymeDisposedError(`Value ${this.varName} has been disposed`);
    }
  }

  /**
   * @internal
   * Get the variable name (for internal use by Achronyme)
   */
  get _varName(): string {
    return this.varName;
  }

  /**
   * @internal
   * Get the handle (for internal use by Achronyme and diagnostics)
   */
  get handle(): Handle | undefined {
    return this._handle;
  }

  // ============================================================================
  // Value Extraction
  // ============================================================================

  /**
   * Get the raw result as a string from C++
   */
  async raw(): Promise<string> {
    this.checkDisposed();
    try {
      return (this.ach as any)._eval(this.varName);
    } catch (e: any) {
      throw wrapCppError(e.message || String(e), this.varName);
    }
  }

  /**
   * Get the value parsed to appropriate JavaScript type
   */
  async value<T = any>(): Promise<T> {
    const raw = await this.raw();
    return parseResult(raw) as T;
  }

  /**
   * Get the value as a number
   * @throws AchronymeTypeError if value is not a number
   */
  async toNumber(): Promise<number> {
    const val = await this.value();
    if (typeof val !== 'number') {
      throw wrapCppError(`Expected number, got ${typeof val}`, this.varName);
    }
    return val;
  }

  /**
   * Get the value as a complex number
   * @throws AchronymeTypeError if value is not complex
   */
  async toComplex(): Promise<ComplexNumber> {
    const raw = await this.raw();
    return parseComplex(raw);
  }

  /**
   * Get the value as a vector (number array)
   * OPTIMIZED: Uses direct memory read if handle exists
   * @throws AchronymeTypeError if value is not a vector
   */
  async toVector(): Promise<number[]> {
    this.checkDisposed();

    // Try fast path if handle exists
    if (this._handle !== undefined) {
      try {
        const module = (this.ach as any).module;
        if (!module) throw new Error('Module not initialized');

        // Use new pointer-free API for Emscripten 4.0 compatibility
        const length = module.getVectorLength(this._handle);
        const dataPtr = module.getVectorDataPtr(this._handle);

        // Create TypedArray view (zero-copy)
        const view = module.HEAPF64.subarray(dataPtr / 8, dataPtr / 8 + length);

        // Copy to regular array (necessary to preserve data)
        return Array.from(view);
      } catch (e: any) {
        // Fallback to slow path on error
        console.warn(`[AchronymeValue] Fast toVector failed, using slow path: ${e.message}`);
      }
    }

    // Slow path: parse from string
    const raw = await this.raw();
    return parseVector(raw);
  }

  /**
   * Get the value as a matrix (2D number array)
   * @throws AchronymeTypeError if value is not a matrix
   */
  async toMatrix(): Promise<number[][]> {
    const raw = await this.raw();
    return parseMatrix(raw);
  }

  /**
   * Get the detected type of this value
   */
  async getType(): Promise<AchronymeValueType> {
    if (this._metadata.type !== 'unknown') {
      return this._metadata.type;
    }
    const raw = await this.raw();
    const type = detectType(raw);
    this._metadata.type = type;
    return type;
  }

  // ============================================================================
  // Arithmetic Operations
  // ============================================================================

  /**
   * Add another value or number
   * @example
   * const x = ach.number(5);
   * const y = x.add(3); // 8
   * const z = x.add(y); // 13
   */
  add(other: AchronymeValue | number): AchronymeValue {
    this.checkDisposed();

    // Fast path: Si ambos tienen handles, usar vadd_fast
    if (typeof other !== 'number' && this._handle !== undefined && other._handle !== undefined) {
      const module = (this.ach as any).module;
      const resultHandle = module.vadd_fast(this._handle, other._handle);
      return new AchronymeValue(this.ach, `__vadd_${this._handle}_${other._handle}`, resultHandle);
    }

    // Slow path: fallback al parser
    const otherExpr = typeof other === 'number' ? other.toString() : other._varName;
    return (this.ach as any)._createFromExpression(`${this.varName} + ${otherExpr}`);
  }

  /**
   * Subtract another value or number
   */
  sub(other: AchronymeValue | number): AchronymeValue {
    this.checkDisposed();

    // Fast path: Si ambos tienen handles, usar vsub_fast
    if (typeof other !== 'number' && this._handle !== undefined && other._handle !== undefined) {
      const module = (this.ach as any).module;
      const resultHandle = module.vsub_fast(this._handle, other._handle);
      return new AchronymeValue(this.ach, `__vsub_${this._handle}_${other._handle}`, resultHandle);
    }

    // Slow path: fallback al parser
    const otherExpr = typeof other === 'number' ? other.toString() : other._varName;
    return (this.ach as any)._createFromExpression(`${this.varName} - ${otherExpr}`);
  }

  /**
   * Multiply by another value or number
   */
  mul(other: AchronymeValue | number): AchronymeValue {
    this.checkDisposed();

    // Fast path: Si ambos tienen handles, usar vmul_fast (element-wise para vectores)
    if (typeof other !== 'number' && this._handle !== undefined && other._handle !== undefined) {
      const module = (this.ach as any).module;
      const resultHandle = module.vmul_fast(this._handle, other._handle);
      return new AchronymeValue(this.ach, `__vmul_${this._handle}_${other._handle}`, resultHandle);
    }

    // Fast path: Si es escalar, usar vscale_fast
    if (typeof other === 'number' && this._handle !== undefined) {
      const module = (this.ach as any).module;
      const resultHandle = module.vscale_fast(this._handle, other);
      return new AchronymeValue(this.ach, `__vscale_${this._handle}_${other}`, resultHandle);
    }

    // Slow path: fallback al parser
    const otherExpr = typeof other === 'number' ? other.toString() : other._varName;
    return (this.ach as any)._createFromExpression(`${this.varName} * ${otherExpr}`);
  }

  /**
   * Divide by another value or number
   */
  div(other: AchronymeValue | number): AchronymeValue {
    this.checkDisposed();

    // Fast path: Si ambos tienen handles, usar vdiv_fast
    if (typeof other !== 'number' && this._handle !== undefined && other._handle !== undefined) {
      const module = (this.ach as any).module;
      const resultHandle = module.vdiv_fast(this._handle, other._handle);
      return new AchronymeValue(this.ach, `__vdiv_${this._handle}_${other._handle}`, resultHandle);
    }

    // Slow path: fallback al parser
    const otherExpr = typeof other === 'number' ? other.toString() : other._varName;
    return (this.ach as any)._createFromExpression(`${this.varName} / ${otherExpr}`);
  }

  /**
   * Raise to a power
   */
  pow(other: AchronymeValue | number): AchronymeValue {
    this.checkDisposed();
    const otherExpr = typeof other === 'number' ? other.toString() : other._varName;
    return (this.ach as any)._createFromExpression(`${this.varName} ^ ${otherExpr}`);
  }

  /**
   * Modulo operation
   */
  mod(other: AchronymeValue | number): AchronymeValue {
    this.checkDisposed();
    const otherExpr = typeof other === 'number' ? other.toString() : other._varName;
    return (this.ach as any)._createFromExpression(`${this.varName} % ${otherExpr}`);
  }

  /**
   * Negate (unary minus)
   */
  neg(): AchronymeValue {
    this.checkDisposed();
    return (this.ach as any)._createFromExpression(`-${this.varName}`);
  }

  // ============================================================================
  // Comparison Operations
  // ============================================================================

  /**
   * Greater than
   */
  gt(other: AchronymeValue | number): AchronymeValue {
    this.checkDisposed();
    const otherExpr = typeof other === 'number' ? other.toString() : other._varName;
    return (this.ach as any)._createFromExpression(`${this.varName} > ${otherExpr}`);
  }

  /**
   * Greater than or equal
   */
  gte(other: AchronymeValue | number): AchronymeValue {
    this.checkDisposed();
    const otherExpr = typeof other === 'number' ? other.toString() : other._varName;
    return (this.ach as any)._createFromExpression(`${this.varName} >= ${otherExpr}`);
  }

  /**
   * Less than
   */
  lt(other: AchronymeValue | number): AchronymeValue {
    this.checkDisposed();
    const otherExpr = typeof other === 'number' ? other.toString() : other._varName;
    return (this.ach as any)._createFromExpression(`${this.varName} < ${otherExpr}`);
  }

  /**
   * Less than or equal
   */
  lte(other: AchronymeValue | number): AchronymeValue {
    this.checkDisposed();
    const otherExpr = typeof other === 'number' ? other.toString() : other._varName;
    return (this.ach as any)._createFromExpression(`${this.varName} <= ${otherExpr}`);
  }

  /**
   * Equal to
   */
  eq(other: AchronymeValue | number): AchronymeValue {
    this.checkDisposed();
    const otherExpr = typeof other === 'number' ? other.toString() : other._varName;
    return (this.ach as any)._createFromExpression(`${this.varName} == ${otherExpr}`);
  }

  /**
   * Not equal to
   */
  neq(other: AchronymeValue | number): AchronymeValue {
    this.checkDisposed();
    const otherExpr = typeof other === 'number' ? other.toString() : other._varName;
    return (this.ach as any)._createFromExpression(`${this.varName} != ${otherExpr}`);
  }

  // ============================================================================
  // Mathematical Functions (Unary)
  // ============================================================================

  /**
   * Sine
   */
  sin(): AchronymeValue {
    this.checkDisposed();
    return (this.ach as any)._createFromExpression(`sin(${this.varName})`);
  }

  /**
   * Cosine
   */
  cos(): AchronymeValue {
    this.checkDisposed();
    return (this.ach as any)._createFromExpression(`cos(${this.varName})`);
  }

  /**
   * Tangent
   */
  tan(): AchronymeValue {
    this.checkDisposed();
    return (this.ach as any)._createFromExpression(`tan(${this.varName})`);
  }

  /**
   * Square root
   */
  sqrt(): AchronymeValue {
    this.checkDisposed();
    return (this.ach as any)._createFromExpression(`sqrt(${this.varName})`);
  }

  /**
   * Absolute value
   */
  abs(): AchronymeValue {
    this.checkDisposed();
    return (this.ach as any)._createFromExpression(`abs(${this.varName})`);
  }

  /**
   * Natural logarithm
   */
  ln(): AchronymeValue {
    this.checkDisposed();
    return (this.ach as any)._createFromExpression(`ln(${this.varName})`);
  }

  /**
   * Exponential (e^x)
   */
  exp(): AchronymeValue {
    this.checkDisposed();
    return (this.ach as any)._createFromExpression(`exp(${this.varName})`);
  }

  /**
   * Floor
   */
  floor(): AchronymeValue {
    this.checkDisposed();
    return (this.ach as any)._createFromExpression(`floor(${this.varName})`);
  }

  /**
   * Ceiling
   */
  ceil(): AchronymeValue {
    this.checkDisposed();
    return (this.ach as any)._createFromExpression(`ceil(${this.varName})`);
  }

  /**
   * Round
   */
  round(): AchronymeValue {
    this.checkDisposed();
    return (this.ach as any)._createFromExpression(`round(${this.varName})`);
  }

  // ============================================================================
  // DSP Functions
  // ============================================================================

  /**
   * Fast Fourier Transform
   * OPTIMIZED: Uses fast path if this value has a handle
   */
  fft(): AchronymeValue {
    this.checkDisposed();
    // Delegate to Achronyme method which handles fast path detection
    return (this.ach as any).fft(this);
  }

  /**
   * FFT Magnitude
   * OPTIMIZED: Uses fast path if this value has a handle
   */
  fft_mag(): AchronymeValue {
    this.checkDisposed();
    return (this.ach as any).fft_mag(this);
  }

  /**
   * Inverse FFT
   */
  ifft(): AchronymeValue {
    this.checkDisposed();
    return (this.ach as any)._createFromExpression(`ifft(${this.varName})`);
  }

  /**
   * Discrete Fourier Transform
   */
  dft(): AchronymeValue {
    this.checkDisposed();
    return (this.ach as any)._createFromExpression(`dft(${this.varName})`);
  }

  /**
   * DFT Magnitude
   */
  dft_mag(): AchronymeValue {
    this.checkDisposed();
    return (this.ach as any)._createFromExpression(`dft_mag(${this.varName})`);
  }

  /**
   * DFT Phase
   */
  dft_phase(): AchronymeValue {
    this.checkDisposed();
    return (this.ach as any)._createFromExpression(`dft_phase(${this.varName})`);
  }

  // ============================================================================
  // Vector/Matrix Operations
  // ============================================================================

  /**
   * Dot product (for vectors)
   */
  dot(other: AchronymeValue): AchronymeValue {
    this.checkDisposed();
    return (this.ach as any)._createFromExpression(`dot(${this.varName}, ${other._varName})`);
  }

  /**
   * Cross product (for vectors)
   */
  cross(other: AchronymeValue): AchronymeValue {
    this.checkDisposed();
    return (this.ach as any)._createFromExpression(`cross(${this.varName}, ${other._varName})`);
  }

  /**
   * Norm (magnitude) of a vector
   */
  norm(): AchronymeValue {
    this.checkDisposed();
    return (this.ach as any)._createFromExpression(`norm(${this.varName})`);
  }

  /**
   * Transpose (for matrices)
   */
  transpose(): AchronymeValue {
    this.checkDisposed();
    return (this.ach as any)._createFromExpression(`transpose(${this.varName})`);
  }

  /**
   * Determinant (for matrices)
   */
  det(): AchronymeValue {
    this.checkDisposed();
    return (this.ach as any)._createFromExpression(`det(${this.varName})`);
  }

  /**
   * Inverse (for matrices)
   */
  inverse(): AchronymeValue {
    this.checkDisposed();
    return (this.ach as any)._createFromExpression(`inverse(${this.varName})`);
  }
}
