/**
 * Phase 3 Demo: Complex Numbers, Vectors, and Matrices
 */

import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));

// Load WASM module
const AchronymeCore = (await import('./dist/achronyme-core.js')).default;
const wasmPath = path.join(__dirname, 'dist', 'achronyme-core.wasm');
const wasmBinary = fs.readFileSync(wasmPath);
const module = await AchronymeCore({ wasmBinary });

console.log('🚀 Achronyme Core - Phase 3 Demo\n');
console.log('═'.repeat(70));

// Test helper
function test(description, expression, expectedCheck = null) {
    try {
        const result = module.eval(expression);
        console.log(`\n✅ ${description}`);
        console.log(`   Expression: ${expression}`);
        console.log(`   Result:     ${result}`);

        if (expectedCheck && !expectedCheck(result)) {
            console.log(`   ⚠️  Warning: Result doesn't match expected value`);
        }
    } catch (error) {
        console.log(`\n❌ ${description}`);
        console.log(`   Expression: ${expression}`);
        console.log(`   Error:      ${error.message}`);
    }
}

// ============================================================================
// COMPLEX NUMBERS
// ============================================================================
console.log('\n' + '─'.repeat(70));
console.log('📐 COMPLEX NUMBERS');
console.log('─'.repeat(70));

test('[Complex] Imaginary unit i', 'i');
test('[Complex] Pure imaginary 3i', '3i');
test('[Complex] Complex from parts', 'complex(3, 4)');
test('[Complex] Addition 2i + 3i', '2i + 3i');
test('[Complex] Real + imaginary', '2 + 3i');
test('[Complex] Multiplication i * i', 'i * i');
test('[Complex] (2+3i) * (1-2i)', '(2 + 3i) * (1 - 2i)');
test('[Complex] Division (3+4i) / (1+2i)', '(3 + 4i) / (1 + 2i)');
test('[Complex] Power (1+i) ^ 2', '(1 + i) ^ 2');
test('[Complex] Magnitude |3+4i|', 'abs(3 + 4i)', (r) => Math.abs(parseFloat(r) - 5) < 0.01);
test('[Complex] Real part of 3+4i', 'real(3 + 4i)', (r) => Math.abs(parseFloat(r) - 3) < 0.01);
test('[Complex] Imaginary part of 3+4i', 'imag(3 + 4i)', (r) => Math.abs(parseFloat(r) - 4) < 0.01);
test('[Complex] Conjugate of 3+4i', 'conj(3 + 4i)');
test('[Complex] Argument of 1+i', 'arg(1 + i)', (r) => Math.abs(parseFloat(r) - Math.PI/4) < 0.01);

// ============================================================================
// VECTORS
// ============================================================================
console.log('\n' + '─'.repeat(70));
console.log('📊 VECTORS');
console.log('─'.repeat(70));

test('[Vector] Simple vector [1, 2, 3]', '[1, 2, 3]');
test('[Vector] Empty vector []', '[]');
test('[Vector] Vector addition [1,2] + [3,4]', '[1, 2] + [3, 4]');
test('[Vector] Vector subtraction [5,7] - [2,3]', '[5, 7] - [2, 3]');
test('[Vector] Scalar multiplication 2 * [1,2,3]', '2 * [1, 2, 3]');
test('[Vector] Scalar multiplication [1,2,3] * 3', '[1, 2, 3] * 3');
test('[Vector] Scalar division [6,9,12] / 3', '[6, 9, 12] / 3');
test('[Vector] Dot product dot([1,2], [3,4])', 'dot([1, 2], [3, 4])', (r) => parseFloat(r) === 11);
test('[Vector] Dot product dot([1,2,3], [4,5,6])', 'dot([1, 2, 3], [4, 5, 6])', (r) => parseFloat(r) === 32);
test('[Vector] Cross product cross([1,0,0], [0,1,0])', 'cross([1, 0, 0], [0, 1, 0])');
test('[Vector] Cross product cross([1,2,3], [4,5,6])', 'cross([1, 2, 3], [4, 5, 6])');
test('[Vector] Norm of [3,4]', 'norm([3, 4])', (r) => Math.abs(parseFloat(r) - 5) < 0.01);
test('[Vector] Norm of [1,2,2]', 'norm([1, 2, 2])', (r) => Math.abs(parseFloat(r) - 3) < 0.01);
test('[Vector] Normalize [3,4]', 'normalize([3, 4])');
test('[Vector] Unary minus -[1,2,3]', '-[1, 2, 3]');
test('[Vector] Mixed operations [1,2] + [3,4] * 2', '[1, 2] + [3, 4] * 2');

// ============================================================================
// MATRICES
// ============================================================================
console.log('\n' + '─'.repeat(70));
console.log('📈 MATRICES');
console.log('─'.repeat(70));

test('[Matrix] 2x2 matrix [[1,2],[3,4]]', '[[1, 2], [3, 4]]');
test('[Matrix] 3x3 identity-like [[1,0,0],[0,1,0],[0,0,1]]', '[[1, 0, 0], [0, 1, 0], [0, 0, 1]]');
test('[Matrix] Matrix addition [[1,2],[3,4]] + [[5,6],[7,8]]', '[[1, 2], [3, 4]] + [[5, 6], [7, 8]]');
test('[Matrix] Matrix subtraction [[5,6],[7,8]] - [[1,2],[3,4]]', '[[5, 6], [7, 8]] - [[1, 2], [3, 4]]');
test('[Matrix] Scalar multiplication 2 * [[1,2],[3,4]]', '2 * [[1, 2], [3, 4]]');
test('[Matrix] Scalar division [[2,4],[6,8]] / 2', '[[2, 4], [6, 8]] / 2');
test('[Matrix] Matrix multiplication [[1,2],[3,4]] * [[5,6],[7,8]]', '[[1, 2], [3, 4]] * [[5, 6], [7, 8]]');
test('[Matrix] Identity multiplication [[1,2],[3,4]] * [[1,0],[0,1]]', '[[1, 2], [3, 4]] * [[1, 0], [0, 1]]');
test('[Matrix] Transpose of [[1,2],[3,4]]', 'transpose([[1, 2], [3, 4]])');
test('[Matrix] Transpose of [[1,2,3],[4,5,6]]', 'transpose([[1, 2, 3], [4, 5, 6]])');
test('[Matrix] Determinant of [[1,2],[3,4]]', 'det([[1, 2], [3, 4]])', (r) => Math.abs(parseFloat(r) - (-2)) < 0.01);
test('[Matrix] Determinant of [[3,8],[4,6]]', 'det([[3, 8], [4, 6]])', (r) => Math.abs(parseFloat(r) - (-14)) < 0.01);
test('[Matrix] Determinant of [[1,0],[0,1]]', 'det([[1, 0], [0, 1]])', (r) => Math.abs(parseFloat(r) - 1) < 0.01);
test('[Matrix] Trace of [[1,2],[3,4]]', 'trace([[1, 2], [3, 4]])', (r) => Math.abs(parseFloat(r) - 5) < 0.01);
test('[Matrix] Trace of [[5,0,0],[0,6,0],[0,0,7]]', 'trace([[5, 0, 0], [0, 6, 0], [0, 0, 7]])', (r) => Math.abs(parseFloat(r) - 18) < 0.01);
test('[Matrix] Inverse of [[1,2],[3,4]]', 'inverse([[1, 2], [3, 4]])');
test('[Matrix] Inverse of [[4,7],[2,6]]', 'inverse([[4, 7], [2, 6]])');
test('[Matrix] Unary minus -[[1,2],[3,4]]', '-[[1, 2], [3, 4]]');

// ============================================================================
// MIXED OPERATIONS
// ============================================================================
console.log('\n' + '─'.repeat(70));
console.log('🔀 MIXED TYPE OPERATIONS');
console.log('─'.repeat(70));

test('[Mixed] Number + Complex: 2 + 3i', '2 + 3i');
test('[Mixed] Complex + Number: 3i + 4', '3i + 4');
test('[Mixed] Number * Complex: 5 * (2+i)', '5 * (2 + i)');
test('[Mixed] Vector + Scalar broadcast: [1,2,3] + 10', '[1, 2, 3] + 10');
test('[Mixed] Scalar + Vector broadcast: 10 + [1,2,3]', '10 + [1, 2, 3]');
test('[Mixed] Expressions in vector: [sin(0), cos(0), 1+1]', '[sin(0), cos(0), 1 + 1]');
test('[Mixed] Expressions in matrix: [[PI, E], [sqrt(2), sqrt(3)]]', '[[PI, E], [sqrt(2), sqrt(3)]]');
test('[Mixed] Complex arithmetic: (1+2i) + (3+4i)', '(1 + 2i) + (3 + 4i)');
test('[Mixed] Vector with complex dot: dot([1,2], [3,4]) + 5i', 'dot([1, 2], [3, 4]) + 5i');

// ============================================================================
// REAL-WORLD EXAMPLES
// ============================================================================
console.log('\n' + '─'.repeat(70));
console.log('🌍 REAL-WORLD APPLICATIONS');
console.log('─'.repeat(70));

test('[Physics] 2D force vector sum', '[3, 4] + [5, 12]');
test('[Physics] Unit vector direction', 'normalize([3, 4])');
test('[Physics] Work (force dot displacement)', 'dot([10, 5], [3, 4])');
test('[Linear Algebra] Matrix transformation', '[[2, 0], [0, 2]] * [[1, 2], [3, 4]]');
test('[Signal Processing] Complex frequency', '2 * PI * 60i');
test('[Quantum] Pauli X matrix', '[[0, 1], [1, 0]]');
test('[Graphics] Rotation matrix det', 'det([[cos(PI/4), -sin(PI/4)], [sin(PI/4), cos(PI/4)]])');
test('[Engineering] Impedance addition', '(50 + 10i) + (30 - 5i)');

// ============================================================================
// SUMMARY
// ============================================================================
console.log('\n' + '═'.repeat(70));
console.log('✅ Phase 3 Demo Complete!\n');
console.log('📊 Statistics:');
console.log(`   WASM size: ${(wasmBinary.length / 1024).toFixed(2)} KB`);
console.log(`   Tests run: 65+`);
console.log('\n📦 New Features:');
console.log('   • Complex numbers (a + bi)');
console.log('   • Vectors ([x, y, z])');
console.log('   • Matrices ([[a, b], [c, d]])');
console.log('   • 13+ new functions');
console.log('   • Full type system with automatic coercion');
console.log('\n🎉 Phase 3 Complete! Ready for Phase 4 (Higher-order functions & DSP)');
console.log('═'.repeat(70));
