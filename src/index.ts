/**
 * Achronyme Core - Main entry point
 *
 * This is the primary entry point for the Achronyme SDK.
 * It exports both the high-level SDK and direct WASM module access.
 *
 * @example
 * // Using the SDK (recommended)
 * import { Achronyme } from '@achronyme/core';
 *
 * const ach = new Achronyme();
 * await ach.init();
 *
 * const signal = ach.vector([1, 2, 3, 4, 5, 6, 7, 8]);
 * const spectrum = signal.fft_mag();
 * console.log(await spectrum.toVector());
 *
 * signal.dispose();
 * spectrum.dispose();
 *
 * @example
 * // Direct WASM module access (advanced)
 * import { createModule } from '@achronyme/core';
 *
 * const module = await createModule();
 * const result = module.eval('sin(PI / 4)');
 * console.log(result);
 */

// Export SDK
export * from './sdk/index.js';

// Export WASM module creator for advanced usage
import createAchronymeModule from '../achronyme-core.mjs';
export { createAchronymeModule as createModule };

// Re-export main classes for convenience
export { Achronyme } from './sdk/Achronyme.js';
export { AchronymeValue } from './sdk/AchronymeValue.js';
