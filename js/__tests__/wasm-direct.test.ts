/**
 * Direct WASM module test (Node.js compatible)
 */

import { describe, it, expect } from 'vitest';
import fs from 'fs';
import path from 'path';

describe('WASM Module Direct Test', () => {
  it('should load and evaluate expressions', async () => {
    // Import the Emscripten module factory
    const AchronymeCore = (await import('../../dist/achronyme-core.js')).default;

    // Load WASM file manually
    const wasmPath = path.join(process.cwd(), 'dist', 'achronyme-core.wasm');
    const wasmBinary = fs.readFileSync(wasmPath);

    // Initialize module with WASM binary
    const module = await AchronymeCore({ wasmBinary });

    // Test basic arithmetic
    expect(module.eval('2 + 3')).toBe(5);
    expect(module.eval('2 * 3')).toBe(6);
    expect(module.eval('10 / 2')).toBe(5);
  });

  it('should respect operator precedence', async () => {
    const AchronymeCore = (await import('../../dist/achronyme-core.js')).default;
    const wasmPath = path.join(process.cwd(), 'dist', 'achronyme-core.wasm');
    const wasmBinary = fs.readFileSync(wasmPath);
    const module = await AchronymeCore({ wasmBinary });

    expect(module.eval('2 + 3 * 4')).toBe(14);
    expect(module.eval('(2 + 3) * 4')).toBe(20);
  });

  it('should handle exponentiation', async () => {
    const AchronymeCore = (await import('../../dist/achronyme-core.js')).default;
    const wasmPath = path.join(process.cwd(), 'dist', 'achronyme-core.wasm');
    const wasmBinary = fs.readFileSync(wasmPath);
    const module = await AchronymeCore({ wasmBinary });

    expect(module.eval('2 ^ 3')).toBe(8);
    expect(module.eval('2 ^ 3 ^ 2')).toBe(512); // Right-associative
  });

  it('should handle unary minus', async () => {
    const AchronymeCore = (await import('../../dist/achronyme-core.js')).default;
    const wasmPath = path.join(process.cwd(), 'dist', 'achronyme-core.wasm');
    const wasmBinary = fs.readFileSync(wasmPath);
    const module = await AchronymeCore({ wasmBinary });

    expect(module.eval('-5')).toBe(-5);
    expect(module.eval('--5')).toBe(5);
    expect(module.eval('-5 + 3')).toBe(-2);
  });

  it('should handle decimal numbers', async () => {
    const AchronymeCore = (await import('../../dist/achronyme-core.js')).default;
    const wasmPath = path.join(process.cwd(), 'dist', 'achronyme-core.wasm');
    const wasmBinary = fs.readFileSync(wasmPath);
    const module = await AchronymeCore({ wasmBinary });

    expect(module.eval('3.14 * 2')).toBeCloseTo(6.28);
    expect(module.eval('0.1 + 0.2')).toBeCloseTo(0.3);
  });

  it('should handle scientific notation', async () => {
    const AchronymeCore = (await import('../../dist/achronyme-core.js')).default;
    const wasmPath = path.join(process.cwd(), 'dist', 'achronyme-core.wasm');
    const wasmBinary = fs.readFileSync(wasmPath);
    const module = await AchronymeCore({ wasmBinary });

    expect(module.eval('1e3')).toBe(1000);
    expect(module.eval('1e-3')).toBe(0.001);
    expect(module.eval('2.5e2')).toBe(250);
  });

  it('should handle complex expressions', async () => {
    const AchronymeCore = (await import('../../dist/achronyme-core.js')).default;
    const wasmPath = path.join(process.cwd(), 'dist', 'achronyme-core.wasm');
    const wasmBinary = fs.readFileSync(wasmPath);
    const module = await AchronymeCore({ wasmBinary });

    expect(module.eval('2 + 3 * 4 - 5')).toBe(9);
    expect(module.eval('10 / 2 + 3 * 4')).toBe(17);
    expect(module.eval('(2 + 3) ^ 2')).toBe(25);
    expect(module.eval('2 ^ (3 + 1)')).toBe(16);
  });
});
