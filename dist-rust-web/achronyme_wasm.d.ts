/* tslint:disable */
/* eslint-disable */
export function is_positive_definite_js(handle: number): boolean;
export function is_symmetric_js(handle: number, tolerance: number): boolean;
export function createMatrixFromBuffer(data_ptr: number, rows: number, cols: number): number;
export function _free(ptr: number): void;
export function qr_eigenvalues_js(handle: number, max_iterations: number, tolerance: number): number;
export function fft_mag_fast(handle: number): number;
export function sin_fast(handle: number): number;
/**
 * Resetea el evaluador y libera todos los handles
 */
export function reset(): void;
export function cholesky_decomposition_js(handle: number): number;
export function fft_fast(handle: number): number;
export function bindVariableToHandle(name: string, handle: number): void;
export function createVectorFromBuffer(data_ptr: number, len: number): number;
export function _malloc(size: number): number;
export function releaseHandle(handle: number): void;
export function svd_decomposition_js(handle: number): SVDResult;
export function sqrt_fast(handle: number): number;
export function abs_fast(handle: number): number;
export function lu_decomposition_js(handle: number): LUResult;
export function cos_fast(handle: number): number;
export function identity_js(n: number): number;
export function tan_fast(handle: number): number;
export function qr_decomposition_js(handle: number): QRResult;
/**
 * Create vector from JavaScript array (easier for testing)
 */
export function createVector(data: Float64Array): number;
export function getVectorData(handle: number, length_ptr: number): number;
/**
 * Evalua una expresión y retorna el resultado como string
 */
export function _eval(expression: string): string;
export function exp_fast(handle: number): number;
export function eigen_symmetric_js(handle: number, max_iterations: number, tolerance: number): EigenResult;
/**
 * Get vector data from handle (for verification/extraction)
 */
export function getVector(handle: number): Float64Array;
export function power_iteration_js(handle: number, max_iterations: number, tolerance: number): PowerIterationResult;
export function ln_fast(handle: number): number;
export function linspace_fast(start: number, end: number, n: number): number;
export class EigenResult {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  readonly eigenvalues: number;
  readonly eigenvectors: number;
}
export class LUResult {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  readonly L: number;
  readonly U: number;
  readonly P: number;
}
export class PowerIterationResult {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  readonly eigenvalue: number;
  readonly eigenvector: number;
}
export class QRResult {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  readonly Q: number;
  readonly R: number;
}
export class SVDResult {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  readonly U: number;
  readonly S: number;
  readonly V: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_eigenresult_free: (a: number, b: number) => void;
  readonly __wbg_get_eigenresult_eigenvalues: (a: number) => number;
  readonly __wbg_get_eigenresult_eigenvectors: (a: number) => number;
  readonly __wbg_get_luresult_P: (a: number) => number;
  readonly __wbg_get_poweriterationresult_eigenvalue: (a: number) => number;
  readonly __wbg_get_poweriterationresult_eigenvector: (a: number) => number;
  readonly __wbg_luresult_free: (a: number, b: number) => void;
  readonly __wbg_poweriterationresult_free: (a: number, b: number) => void;
  readonly __wbg_qrresult_free: (a: number, b: number) => void;
  readonly __wbg_svdresult_free: (a: number, b: number) => void;
  readonly _eval: (a: number, b: number) => [number, number, number, number];
  readonly _free: (a: number) => void;
  readonly _malloc: (a: number) => number;
  readonly abs_fast: (a: number) => [number, number, number];
  readonly bindVariableToHandle: (a: number, b: number, c: number) => [number, number];
  readonly cholesky_decomposition_js: (a: number) => [number, number, number];
  readonly cos_fast: (a: number) => [number, number, number];
  readonly createMatrixFromBuffer: (a: number, b: number, c: number) => [number, number, number];
  readonly createVector: (a: number, b: number) => number;
  readonly createVectorFromBuffer: (a: number, b: number) => number;
  readonly eigen_symmetric_js: (a: number, b: number, c: number) => [number, number, number];
  readonly exp_fast: (a: number) => [number, number, number];
  readonly fft_fast: (a: number) => [number, number, number];
  readonly fft_mag_fast: (a: number) => [number, number, number];
  readonly getVector: (a: number) => [number, number, number, number];
  readonly getVectorData: (a: number, b: number) => number;
  readonly identity_js: (a: number) => [number, number, number];
  readonly is_positive_definite_js: (a: number) => [number, number, number];
  readonly is_symmetric_js: (a: number, b: number) => [number, number, number];
  readonly linspace_fast: (a: number, b: number, c: number) => [number, number, number];
  readonly ln_fast: (a: number) => [number, number, number];
  readonly lu_decomposition_js: (a: number) => [number, number, number];
  readonly power_iteration_js: (a: number, b: number, c: number) => [number, number, number];
  readonly qr_decomposition_js: (a: number) => [number, number, number];
  readonly qr_eigenvalues_js: (a: number, b: number, c: number) => [number, number, number];
  readonly releaseHandle: (a: number) => void;
  readonly reset: () => void;
  readonly sin_fast: (a: number) => [number, number, number];
  readonly sqrt_fast: (a: number) => [number, number, number];
  readonly svd_decomposition_js: (a: number) => [number, number, number];
  readonly tan_fast: (a: number) => [number, number, number];
  readonly __wbg_get_luresult_L: (a: number) => number;
  readonly __wbg_get_luresult_U: (a: number) => number;
  readonly __wbg_get_qrresult_Q: (a: number) => number;
  readonly __wbg_get_qrresult_R: (a: number) => number;
  readonly __wbg_get_svdresult_S: (a: number) => number;
  readonly __wbg_get_svdresult_U: (a: number) => number;
  readonly __wbg_get_svdresult_V: (a: number) => number;
  readonly __wbindgen_externrefs: WebAssembly.Table;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
