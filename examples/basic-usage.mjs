/**
 * Basic Usage Example - Achronyme SDK
 *
 * This example demonstrates basic mathematical operations
 * using the Achronyme SDK.
 */

import { Achronyme } from '../dist/sdk/sdk/index.js';

console.log('='.repeat(60));
console.log('ACHRONYME SDK - BASIC USAGE EXAMPLE');
console.log('='.repeat(60));
console.log();

// Initialize Achronyme
const ach = new Achronyme({ debug: false });
await ach.init();
console.log('✓ Achronyme initialized\n');

// ============================================================================
// Basic Arithmetic
// ============================================================================
console.log('📐 Basic Arithmetic');
console.log('-'.repeat(40));

const a = ach.number(10);
const b = ach.number(5);

const sum = a.add(b);
const diff = a.sub(b);
const prod = a.mul(b);
const quot = a.div(b);
const power = a.pow(2);

console.log(`a = ${await a.toNumber()}`);
console.log(`b = ${await b.toNumber()}`);
console.log(`a + b = ${await sum.toNumber()}`);
console.log(`a - b = ${await diff.toNumber()}`);
console.log(`a * b = ${await prod.toNumber()}`);
console.log(`a / b = ${await quot.toNumber()}`);
console.log(`a ^ 2 = ${await power.toNumber()}`);

// Clean up
a.dispose();
b.dispose();
sum.dispose();
diff.dispose();
prod.dispose();
quot.dispose();
power.dispose();

console.log();

// ============================================================================
// Mathematical Functions
// ============================================================================
console.log('🧮 Mathematical Functions');
console.log('-'.repeat(40));

const x = ach.number(Math.PI / 4);
const sinX = ach.sin(x);
const cosX = ach.cos(x);
const sqrtX = ach.sqrt(x);

console.log(`x = π/4 = ${await x.toNumber()}`);
console.log(`sin(x) = ${await sinX.toNumber()}`);
console.log(`cos(x) = ${await cosX.toNumber()}`);
console.log(`sqrt(x) = ${await sqrtX.toNumber()}`);

x.dispose();
sinX.dispose();
cosX.dispose();
sqrtX.dispose();

console.log();

// ============================================================================
// Fluent API (Chaining)
// ============================================================================
console.log('🔗 Fluent API (Chaining)');
console.log('-'.repeat(40));

const result = ach.number(5)
  .mul(2)
  .add(10)
  .div(4)
  .pow(2);

console.log('(((5 * 2) + 10) / 4) ^ 2 =', await result.toNumber());
result.dispose();

console.log();

// ============================================================================
// Vectors
// ============================================================================
console.log('📊 Vector Operations');
console.log('-'.repeat(40));

const v1 = ach.vector([1, 2, 3, 4]);
const v2 = ach.vector([2, 2, 2, 2]);

const vSum = v1.add(v2);
const vMul = v1.mul(3);
const vNorm = ach.norm(v1);

console.log('v1 =', await v1.toVector());
console.log('v2 =', await v2.toVector());
console.log('v1 + v2 =', await vSum.toVector());
console.log('v1 * 3 =', await vMul.toVector());
console.log('norm(v1) =', await vNorm.toNumber());

v1.dispose();
v2.dispose();
vSum.dispose();
vMul.dispose();
vNorm.dispose();

console.log();

// ============================================================================
// Complex Numbers
// ============================================================================
console.log('🔢 Complex Numbers');
console.log('-'.repeat(40));

const c1 = ach.complex(2, 3);
const c2 = ach.complex(1, -1);
const cSum = c1.add(c2);
const cProd = c1.mul(c2);

console.log('c1 = 2+3i =', await c1.raw());
console.log('c2 = 1-1i =', await c2.raw());
console.log('c1 + c2 =', await cSum.raw());
console.log('c1 * c2 =', await cProd.raw());

c1.dispose();
c2.dispose();
cSum.dispose();
cProd.dispose();

console.log();

// ============================================================================
// Constants
// ============================================================================
console.log('🌟 Mathematical Constants');
console.log('-'.repeat(40));

const pi = ach.PI;
const e = ach.E;
const phi = ach.PHI;

console.log('PI =', await pi.toNumber());
console.log('E =', await e.toNumber());
console.log('PHI =', await phi.toNumber());

pi.dispose();
e.dispose();
phi.dispose();

console.log();

// ============================================================================
// Memory Statistics
// ============================================================================
console.log('💾 Memory Statistics');
console.log('-'.repeat(40));

const stats = ach.getMemoryStats();
console.log('Total variables created:', stats.totalVariables);
console.log('Active variables:', stats.activeVariables);
console.log('Disposed variables:', stats.disposedVariables);

console.log();
console.log('✓ Example completed successfully');
console.log('='.repeat(60));
