import { readFileSync } from 'fs';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';
import { initSync, _eval } from '../dist-rust/achronyme-core.mjs';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

console.log('Loading Rust WASM...');
const wasmPath = join(__dirname, '../dist-rust/achronyme-core.wasm');
const wasmBuffer = readFileSync(wasmPath);
initSync({ module: new WebAssembly.Module(wasmBuffer) });
console.log('✓ Loaded\n');

console.log('Testing eval:');
try {
    const result = _eval('2 + 2');
    console.log('Result type:', typeof result);
    console.log('Result value:', result);
} catch (error) {
    console.error('Error:', error);
}
