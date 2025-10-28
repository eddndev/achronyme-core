#!/usr/bin/env node

/**
 * Cross-platform clean script for Achronyme Core
 * Works on Windows, Linux, and macOS
 */

import { rmSync, existsSync } from 'fs';
import { join } from 'path';
import { fileURLToPath } from 'url';
import { dirname } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const rootDir = join(__dirname, '..');

console.log('🧹 Cleaning build artifacts...\n');

const pathsToClean = [
  join(rootDir, 'dist'),
  join(rootDir, 'wasm', 'build'),
  join(rootDir, 'node_modules', '.cache')
];

let cleanedCount = 0;

for (const path of pathsToClean) {
  if (existsSync(path)) {
    try {
      console.log(`   Removing: ${path}`);
      rmSync(path, { recursive: true, force: true });
      cleanedCount++;
    } catch (error) {
      console.error(`   ⚠️  Failed to remove ${path}:`, error.message);
    }
  }
}

if (cleanedCount === 0) {
  console.log('   Nothing to clean (already clean)');
} else {
  console.log(`\n✅ Cleaned ${cleanedCount} director${cleanedCount === 1 ? 'y' : 'ies'}`);
}
