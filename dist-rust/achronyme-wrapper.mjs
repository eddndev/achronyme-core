/**
 * Node.js wrapper for Achronyme WASM (bundler target)
 *
 * This wrapper manually loads the WASM module using Node.js APIs
 * because Node's ESM loader doesn't support importing .wasm files directly.
 */

import { readFileSync } from 'fs';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

let wasmModule = null;
let isInitialized = false;

/**
 * Initialize the WASM module
 * @returns {Promise<Object>} The initialized module with all exports
 */
async function init() {
    if (isInitialized && wasmModule) {
        return wasmModule;
    }

    // Read WASM file
    const wasmPath = join(__dirname, 'achronyme-core_bg.wasm');
    const wasmBytes = readFileSync(wasmPath);

    // We need to set up imports BEFORE importing bg.js
    // because the imports will be called during WASM instantiation
    let wasm;

    // Helper functions needed by WASM (inlined to avoid circular dependency)
    let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
    cachedTextDecoder.decode();

    let cachedUint8ArrayMemory0 = null;
    function getUint8ArrayMemory0() {
        if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
            cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
        }
        return cachedUint8ArrayMemory0;
    }

    function getStringFromWasm0(ptr, len) {
        ptr = ptr >>> 0;
        return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
    }

    // Create WASM imports object
    const imports = {
        './achronyme_wasm_bg.js': {
            __wbindgen_throw: function(arg0, arg1) {
                throw new Error(getStringFromWasm0(arg0, arg1));
            },
            __wbindgen_init_externref_table: function() {
                const table = wasm.__wbindgen_externrefs;
                const offset = table.grow(4);
                table.set(0, undefined);
                table.set(offset + 0, undefined);
                table.set(offset + 1, null);
                table.set(offset + 2, true);
                table.set(offset + 3, false);
            },
            __wbindgen_cast: function(arg0, arg1) {
                return getStringFromWasm0(arg0, arg1);
            },
        }
    };

    // Instantiate WASM module
    const { instance } = await WebAssembly.instantiate(wasmBytes, imports);
    wasm = instance.exports;

    // Now import the background JS with all the glue code
    const bg = await import('./achronyme-core_bg.js');

    // Set the WASM instance in the background JS
    bg.__wbg_set_wasm(wasm);

    // Call initialization function
    wasm.__wbindgen_start();

    // Create module object with all exports from bg.js
    wasmModule = {
        // Core evaluation functions
        _eval: bg._eval,
        reset: bg.reset,

        // Memory management
        releaseHandle: bg.releaseHandle,

        // All other exports from bg.js are available through bg.*
        ...bg
    };

    isInitialized = true;
    return wasmModule;
}

/**
 * Create Achronyme module (lazy initialization)
 */
async function createAchronymeModule() {
    return await init();
}

export default createAchronymeModule;
export { init };
