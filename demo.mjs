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
  // ========== PHASE 1: Arithmetic ==========
  { expr: '2 + 3', desc: '[Phase 1] Basic addition' },
  { expr: '2 * 3 * 4', desc: '[Phase 1] Precedence' },
  { expr: '2 ^ 3 ^ 2', desc: '[Phase 1] Right-associative power' },
  { expr: '((2 + 3) * 4) ^ 2', desc: '[Phase 1] Complex nested' },

  // ========== PHASE 2: Constants ==========
  { expr: 'PI', desc: '[Phase 2] Constant PI' },
  { expr: 'E', desc: '[Phase 2] Constant E' },
  { expr: '2 * PI', desc: '[Phase 2] Constant in expression' },
  { expr: 'PI * E', desc: '[Phase 2] Multiple constants' },

  // ========== PHASE 2: Trigonometric Functions ==========
  { expr: 'sin(0)', desc: '[Phase 2] sin(0)' },
  { expr: 'sin(PI/2)', desc: '[Phase 2] sin(90°) = 1' },
  { expr: 'cos(0)', desc: '[Phase 2] cos(0)' },
  { expr: 'cos(PI)', desc: '[Phase 2] cos(180°) = -1' },
  { expr: 'tan(PI/4)', desc: '[Phase 2] tan(45°) = 1' },

  // ========== PHASE 2: Exponential & Logarithmic ==========
  { expr: 'exp(0)', desc: '[Phase 2] e^0 = 1' },
  { expr: 'exp(1)', desc: '[Phase 2] e^1 = e' },
  { expr: 'log(E)', desc: '[Phase 2] ln(e) = 1' },
  { expr: 'log(1)', desc: '[Phase 2] ln(1) = 0' },
  { expr: 'sqrt(16)', desc: '[Phase 2] √16 = 4' },
  { expr: 'sqrt(2)', desc: '[Phase 2] √2' },

  // ========== PHASE 2: Rounding Functions ==========
  { expr: 'floor(3.7)', desc: '[Phase 2] floor(3.7) = 3' },
  { expr: 'ceil(3.2)', desc: '[Phase 2] ceil(3.2) = 4' },
  { expr: 'round(3.5)', desc: '[Phase 2] round(3.5) = 4' },
  { expr: 'abs(-5)', desc: '[Phase 2] abs(-5) = 5' },

  // ========== PHASE 2: Variadic Functions ==========
  { expr: 'min(5, 3, 8, 1)', desc: '[Phase 2] min(...) = 1' },
  { expr: 'max(5, 3, 8, 1)', desc: '[Phase 2] max(...) = 8' },
  { expr: 'min(2, max(1, 3))', desc: '[Phase 2] Nested min/max' },

  // ========== PHASE 2: Nested Functions ==========
  { expr: 'sqrt(abs(-16))', desc: '[Phase 2] Nested: √|−16|' },
  { expr: 'log(exp(5))', desc: '[Phase 2] Inverse functions' },
  { expr: 'abs(sin(PI/4))', desc: '[Phase 2] |sin(45°)|' },

  // ========== PHASE 2: Complex Expressions ==========
  { expr: 'sin(PI/6) + cos(PI/3)', desc: '[Phase 2] Trig combination' },
  { expr: 'abs(sin(PI/4)) ^ 2 + abs(cos(PI/4)) ^ 2', desc: '[Phase 2] Pythagorean identity' },
  { expr: '2 * PI * sqrt(9.8 / 0.5)', desc: '[Phase 2] Physics formula' },
  { expr: 'log(sqrt(E ^ 4))', desc: '[Phase 2] Nested logs/exp' },
  { expr: 'max(sin(0), cos(0), tan(0))', desc: '[Phase 2] Max of trig values' },
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
