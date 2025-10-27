/**
 * Achronyme SDK - Type-safe TypeScript wrapper for Achronyme Core
 *
 * @packageDocumentation
 */

export { Achronyme } from './Achronyme.js';
export { AchronymeValue } from './AchronymeValue.js';

export {
  AchronymeError,
  AchronymeSyntaxError,
  AchronymeRuntimeError,
  AchronymeTypeError,
  AchronymeDisposedError,
  AchronymeNotInitializedError,
  AchronymeArgumentError,
} from './errors.js';

export type {
  WasmModule,
  ComplexNumber,
  AchronymeValueType,
  PrimitiveValue,
  AchronymeOptions,
  ValueMetadata,
  MemoryStats,
  LambdaDefinition,
  WindowFunction,
  FFTResult,
  ConvolutionMode,
} from './types.js';

export {
  parseResult,
  parseComplex,
  parseVector,
  parseMatrix,
  formatValue,
  formatVector,
  formatMatrix,
  formatComplex,
  detectType,
} from './utils.js';
