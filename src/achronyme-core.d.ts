declare module '*/achronyme-core.mjs' {
  type Handle = number;

  interface AchronymeCoreModule {
    // ========================================================================
    // Expression API (Original - Parsing-based)
    // ========================================================================
    eval(expression: string): string;
    reset(): string;
    listVariables(): string;

    // ========================================================================
    // Fast API (Handle-based, Zero-parsing)
    // ========================================================================

    // Vector/Matrix Creation
    createVectorFromBuffer(dataPtr: number, length: number): Handle;
    createMatrixFromBuffer(dataPtr: number, rows: number, cols: number): Handle;

    // Data Extraction
    getVectorData(handle: Handle, outLengthPtr: number): number;
    getMatrixData(handle: Handle, outRowsPtr: number, outColsPtr: number): number;
    copyVectorToBuffer(handle: Handle, destPtr: number, maxLength: number): number;

    // DSP Operations (Fast Path)
    fft_fast(inputHandle: Handle): Handle;
    fft_mag_fast(inputHandle: Handle): Handle;
    fft_phase_fast(inputHandle: Handle): Handle;
    ifft_fast(inputHandle: Handle): Handle;
    conv_fast(h1: Handle, h2: Handle): Handle;
    conv_fft_fast(h1: Handle, h2: Handle): Handle;

    // Vector Operations (Fast Path)
    vadd_fast(h1: Handle, h2: Handle): Handle;
    vsub_fast(h1: Handle, h2: Handle): Handle;
    vmul_fast(h1: Handle, h2: Handle): Handle;
    vdiv_fast(h1: Handle, h2: Handle): Handle;
    vscale_fast(h: Handle, scalar: number): Handle;
    dot_fast(h1: Handle, h2: Handle): Handle;
    norm_fast(h: Handle): Handle;

    // Mathematical Functions (Fast Path)
    sin_fast(h: Handle): Handle;
    cos_fast(h: Handle): Handle;
    tan_fast(h: Handle): Handle;
    exp_fast(h: Handle): Handle;
    ln_fast(h: Handle): Handle;
    abs_fast(h: Handle): Handle;
    sqrt_fast(h: Handle): Handle;

    // Optimization Functions (Fast Path)
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

    // Integration with Evaluator
    bindVariableToHandle(varName: string, handle: Handle): void;
    createHandleFromVariable(varName: string): Handle;

    // ========================================================================
    // Emscripten Memory Management
    // ========================================================================
    _malloc(size: number): number;
    _free(ptr: number): void;

    // Heap views
    HEAP8: Int8Array;
    HEAP16: Int16Array;
    HEAP32: Int32Array;
    HEAPU8: Uint8Array;
    HEAPU16: Uint16Array;
    HEAPU32: Uint32Array;
    HEAPF32: Float32Array;
    HEAPF64: Float64Array;
  }

  export default function createAchronymeModule(): Promise<AchronymeCoreModule>;
}
