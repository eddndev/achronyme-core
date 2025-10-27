/**
 * Functional Programming Example - Achronyme SDK
 *
 * This example demonstrates functional programming features:
 * - Variables
 * - Lambda functions
 * - Higher-order functions (map, filter, reduce, pipe)
 * - Closures
 */

import { Achronyme } from '../dist/sdk/sdk/index.js';

console.log('='.repeat(60));
console.log('ACHRONYME SDK - FUNCTIONAL PROGRAMMING EXAMPLE');
console.log('='.repeat(60));
console.log();

const ach = new Achronyme({ debug: false });
await ach.init();
console.log('✓ Achronyme initialized\n');

// ============================================================================
// Variables
// ============================================================================
console.log('📦 Variables');
console.log('-'.repeat(40));

const x = ach.let('x', 10);
const y = ach.let('y', 20);
const sum = ach.get('x').add(ach.get('y'));

console.log('x =', await x.toNumber());
console.log('y =', await y.toNumber());
console.log('x + y =', await sum.toNumber());

sum.dispose();

console.log();

// ============================================================================
// Lambda Functions
// ============================================================================
console.log('λ Lambda Functions');
console.log('-'.repeat(40));

// Single parameter lambda
const square = ach.lambda(['n'], 'n ^ 2');
console.log('square = n => n ^ 2');

// Call lambda directly (using raw eval)
ach.let('square', square);
const result5 = ach.evalValue('square(5)');
console.log('square(5) =', await result5.toNumber());
result5.dispose();

// Multi-parameter lambda
const add = ach.lambda(['a', 'b'], 'a + b');
ach.let('add', add);
const result3_7 = ach.evalValue('add(3, 7)');
console.log('add(3, 7) =', await result3_7.toNumber());
result3_7.dispose();

console.log();

// ============================================================================
// Higher-Order Functions - MAP
// ============================================================================
console.log('🗺️ Map Function');
console.log('-'.repeat(40));

const numbers = ach.vector([1, 2, 3, 4, 5]);
console.log('numbers =', await numbers.toVector());

// Map with inline lambda
const squared = ach.map('n => n ^ 2', numbers);
console.log('map(n => n^2) =', await squared.toVector());

const doubled = ach.map('n => n * 2', numbers);
console.log('map(n => n*2) =', await doubled.toVector());

squared.dispose();
doubled.dispose();

console.log();

// ============================================================================
// Higher-Order Functions - FILTER
// ============================================================================
console.log('🔍 Filter Function');
console.log('-'.repeat(40));

const nums = ach.vector([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
console.log('nums =', await nums.toVector());

const evens = ach.filter('n => n % 2 == 0', nums);
console.log('filter(n => n%2==0) =', await evens.toVector());

const gt5 = ach.filter('n => n > 5', nums);
console.log('filter(n => n>5) =', await gt5.toVector());

evens.dispose();
gt5.dispose();

console.log();

// ============================================================================
// Higher-Order Functions - REDUCE
// ============================================================================
console.log('📉 Reduce Function');
console.log('-'.repeat(40));

const values = ach.vector([1, 2, 3, 4, 5]);
console.log('values =', await values.toVector());

const sum_reduce = ach.reduce('a, b => a + b', values, 0);
console.log('reduce(a,b => a+b, 0) =', await sum_reduce.toNumber());

const product = ach.reduce('a, b => a * b', values, 1);
console.log('reduce(a,b => a*b, 1) =', await product.toNumber());

sum_reduce.dispose();
product.dispose();

console.log();

// ============================================================================
// Complex Functional Pipeline
// ============================================================================
console.log('🔄 Functional Pipeline');
console.log('-'.repeat(40));

// Create a complex pipeline:
// 1. Start with [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
// 2. Filter even numbers
// 3. Square each number
// 4. Sum them all

const data = ach.vector([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
console.log('data =', await data.toVector());

const step1 = ach.filter('n => n % 2 == 0', data);
console.log('1. Filter evens =', await step1.toVector());

const step2 = ach.map('n => n ^ 2', step1);
console.log('2. Square each =', await step2.toVector());

const step3 = ach.reduce('a, b => a + b', step2, 0);
console.log('3. Sum =', await step3.toNumber());

data.dispose();
step1.dispose();
step2.dispose();
step3.dispose();

console.log();

// ============================================================================
// Comparisons
// ============================================================================
console.log('⚖️ Comparison Operations');
console.log('-'.repeat(40));

const a = ach.number(10);
const b = ach.number(5);

const gt = a.gt(b);
const lt = a.lt(b);
const eq = a.eq(b);

console.log('a = 10, b = 5');
console.log('a > b =', await gt.toNumber());
console.log('a < b =', await lt.toNumber());
console.log('a == b =', await eq.toNumber());

a.dispose();
b.dispose();
gt.dispose();
lt.dispose();
eq.dispose();

console.log();

// ============================================================================
// Memory Statistics
// ============================================================================
const stats = ach.getMemoryStats();
console.log('💾 Memory Statistics');
console.log('-'.repeat(40));
console.log('Total variables:', stats.totalVariables);
console.log('Active variables:', stats.activeVariables);
console.log('Disposed variables:', stats.disposedVariables);
console.log('Variable names:', stats.variableNames.slice(0, 10).join(', '), '...');

console.log();
console.log('✓ Functional programming example completed successfully');
console.log('='.repeat(60));
