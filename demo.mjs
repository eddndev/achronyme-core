/**
 * Live demo of Achronyme Core
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

console.log('🚀 Achronyme Core - Live Demo\n');
console.log('═'.repeat(60));

// Test expressions
const tests = [
  // Basic arithmetic
  { expr: '2 + 3', desc: 'Basic addition' },
  { expr: '10 - 5', desc: 'Basic subtraction' },
  { expr: '6 * 7', desc: 'Basic multiplication' },
  { expr: '20 / 4', desc: 'Basic division' },

  // Precedence
  { expr: '2 + 3 * 4', desc: 'Precedence (mult before add)' },
  { expr: '(2 + 3) * 4', desc: 'Parentheses override' },
  { expr: '10 - 6 / 2', desc: 'Division before subtraction' },

  // Exponentiation
  { expr: '2 ^ 8', desc: 'Power operator' },
  { expr: '2 ^ 3 ^ 2', desc: 'Right-associative power' },
  { expr: '(2 + 3) ^ 2', desc: 'Power with parentheses' },

  // Unary minus
  { expr: '-5', desc: 'Unary minus' },
  { expr: '--5', desc: 'Double negation' },
  { expr: '-5 + 3', desc: 'Unary in expression' },
  { expr: '2 * -3', desc: 'Multiplication with negative' },

  // Decimals
  { expr: '3.14 * 2', desc: 'Decimal numbers' },
  { expr: '0.1 + 0.2', desc: 'Floating point addition' },

  // Scientific notation
  { expr: '1e3', desc: 'Scientific notation (1000)' },
  { expr: '1e-3', desc: 'Scientific notation (0.001)' },
  { expr: '2.5e2', desc: 'Scientific notation (250)' },

  // Complex expressions
  { expr: '2 + 3 * 4 - 5', desc: 'Complex expression 1' },
  { expr: '10 / 2 + 3 * 4', desc: 'Complex expression 2' },
  { expr: '((2 + 3) * 4) ^ 2', desc: 'Deeply nested expression' },
];

for (const test of tests) {
  const result = module.eval(test.expr);
  console.log(`\n${test.desc}`);
  console.log(`  Expression: ${test.expr}`);
  console.log(`  Result:     ${result}`);
}

console.log('\n' + '═'.repeat(60));
console.log('✅ All expressions evaluated successfully!');
console.log('\n📊 Performance Stats:');
console.log(`  WASM size: ${(wasmBinary.length / 1024).toFixed(2)} KB`);
console.log(`  Tests run: ${tests.length}`);
console.log('\n🎉 Achronyme Core Phase 1 Complete!');
