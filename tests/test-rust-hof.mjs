/**
 * Test Higher-Order Functions (HOF) in Achronyme Rust WASM
 */

import { readFileSync } from 'fs';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';
import init, { initSync, _eval as wasmEval } from '../dist-rust/achronyme-core.mjs';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

console.log('Loading Rust WASM module...');
const wasmPath = join(__dirname, '../dist-rust/achronyme-core.wasm');
const wasmBuffer = readFileSync(wasmPath);
initSync(wasmBuffer);
console.log('✓ Rust WASM loaded\n');

let passed = 0, failed = 0;

function test(name, expr, expected) {
    try {
        const result = wasmEval(expr);
        const numResult = typeof result === 'string' ? parseFloat(result) : result;
        if (Math.abs(numResult - expected) < 1e-10) {
            console.log(`✅ ${name}: ${result}`);
            passed++;
        } else {
            console.log(`❌ ${name}: expected ${expected}, got ${result}`);
            failed++;
        }
    } catch (error) {
        console.log(`❌ ${name}: ${error.message}`);
        failed++;
    }
}

console.log('Testing HOF:\n');

test('reduce() sum', 'reduce((acc,x) => acc + x,0,[1,2,3,4])', 10);
test('pipe() simple', 'pipe(5,x => x * 2,x => x + 1)', 11);
test('map() double', 'map(x => x * 2,[1,2,3])', 2); // First element  
test('filter() even', 'filter(x => x % 2,[1,2,3,4])', 2); // First even

console.log(`\n✅ Passed: ${passed}, ❌ Failed: ${failed}`);
if (failed === 0) console.log('\n🎉 All Rust HOF tests passed! 🦀');
process.exit(failed === 0 ? 0 : 1);
