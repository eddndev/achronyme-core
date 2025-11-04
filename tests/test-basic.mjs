import { readFileSync } from 'fs';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';
import { initSync, _eval } from '../dist-rust/achronyme-core.mjs';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const wasmPath = join(__dirname, '../dist-rust/achronyme-core.wasm');
const wasmBuffer = readFileSync(wasmPath);
initSync({ module: new WebAssembly.Module(wasmBuffer) });

console.log('Testing basic expressions:\n');

try {
    console.log('2+2 =', _eval('2+2'));
    console.log('3*4 =', _eval('3*4'));
    console.log('sin(0) =', _eval('sin(0)'));
} catch (e) {
    console.error('Error:', e.message);
}
