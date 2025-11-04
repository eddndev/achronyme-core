import { readFileSync } from 'fs';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';
import { initSync, _eval } from '../dist-rust/achronyme-core.mjs';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const wasmPath = join(__dirname, '../dist-rust/achronyme-core.wasm');
const wasmBuffer = readFileSync(wasmPath);
initSync({ module: new WebAssembly.Module(wasmBuffer) });

let passed = 0, failed = 0;

function test(name, expr, expected) {
    try {
        const result = _eval(expr);
        const num = typeof result === 'string' ? parseFloat(result) : result;
        if (Math.abs(num - expected) < 1e-10) {
            console.log(`✅ ${name}`);
            passed++;
        } else {
            console.log(`❌ ${name}: expected ${expected}, got ${num}`);
            failed++;
        }
    } catch (error) {
        console.log(`❌ ${name}: ${error.message}`);
        failed++;
    }
}

console.log('🦀 Testing Rust WASM HOF:\n');

test('reduce() sum', 'reduce((acc,x)=>acc+x,0,[1,2,3,4])', 10);
test('pipe() simple', 'pipe(5,x=>x*2,x=>x+1)', 11);
test('map() double', 'map(x=>x*2,[1,2,3])', 2);
test('filter() even', 'filter(x=>x%2,[1,2,3,4])', 2);

console.log(`\n✅ Passed: ${passed}, ❌ Failed: ${failed}`);
if (failed === 0) console.log('\n🎉 All Rust WASM HOF tests passed! 🦀\n');
else console.log(`\n❌ ${failed} test(s) failed\n`);

process.exit(failed === 0 ? 0 : 1);
