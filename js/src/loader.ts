/**
 * WASM Module Loader
 *
 * Loads the Achronyme Core WebAssembly module.
 */

export interface AchronymeModule {
  eval(expression: string): number;
}

let wasmModule: AchronymeModule | null = null;

/**
 * Load the WASM module
 *
 * @returns Promise that resolves to the loaded module
 */
export async function loadWASM(): Promise<AchronymeModule> {
  if (wasmModule) {
    return wasmModule;
  }

  try {
    // Import the Emscripten-generated module
    // In production, this path should be configurable
    // @ts-ignore - WASM module doesn't have TypeScript declarations
    const AchronymeCore = await import('../../dist/achronyme-core.js');

    // Initialize the module
    const module = await AchronymeCore.default();

    wasmModule = module;
    return module;
  } catch (error) {
    throw new Error(`Failed to load WASM module: ${error}`);
  }
}

/**
 * Check if WASM is loaded
 */
export function isLoaded(): boolean {
  return wasmModule !== null;
}

/**
 * Get the loaded module (throws if not loaded)
 */
export function getModule(): AchronymeModule {
  if (!wasmModule) {
    throw new Error('WASM module not loaded. Call loadWASM() first.');
  }
  return wasmModule;
}
