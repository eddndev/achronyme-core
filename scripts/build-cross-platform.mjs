#!/usr/bin/env node

/**
 * Cross-platform build script for Achronyme Core WASM
 * Works on Windows, Linux, and macOS
 */

import { spawn } from 'child_process';
import { platform } from 'os';
import { existsSync, mkdirSync } from 'fs';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const rootDir = join(__dirname, '..');

// Get build mode from command line arguments
const buildMode = process.argv[2] || 'wasm';
const isDev = buildMode === 'wasm-dev';

console.log(`🔨 Building Achronyme Core (WASM${isDev ? ' - Development Mode' : ''})`);

// Check if emcc is available
function checkEmscripten() {
  return new Promise((resolve) => {
    const emccCheck = spawn(platform() === 'win32' ? 'where' : 'which', ['emcc']);

    emccCheck.on('close', (code) => {
      if (code !== 0) {
        console.error('\n⚠️  Emscripten not found. Please install and activate emsdk:');
        console.error('   git clone https://github.com/emscripten-core/emsdk.git');
        console.error('   cd emsdk');
        if (platform() === 'win32') {
          console.error('   emsdk install latest');
          console.error('   emsdk activate latest');
          console.error('   emsdk_env.bat');
        } else {
          console.error('   ./emsdk install latest');
          console.error('   ./emsdk activate latest');
          console.error('   source ./emsdk_env.sh');
        }
        process.exit(1);
      }
      resolve();
    });
  });
}

// Show emcc version
function showVersion() {
  return new Promise((resolve, reject) => {
    console.log('\n📦 Emscripten version:');
    const emccVersion = spawn('emcc', ['--version'], {
      stdio: 'inherit'
    });

    emccVersion.on('close', () => {
      resolve();
    });

    emccVersion.on('error', (err) => {
      console.error('\n⚠️  Failed to run emcc. Is Emscripten activated?');
      reject(err);
    });
  });
}

// Build WASM with emcc
function buildWasm() {
  return new Promise((resolve, reject) => {
    // Ensure dist directory exists
    const distDir = join(rootDir, 'dist');
    if (!existsSync(distDir)) {
      mkdirSync(distDir, { recursive: true });
    }

    console.log('\n🔧 Compiling C++ → WASM...');

    // Build arguments
    const args = [
      'wasm/src/core/constants.cpp',
      'wasm/src/core/complex.cpp',
      'wasm/src/core/vector.cpp',
      'wasm/src/core/matrix.cpp',
      'wasm/src/core/function.cpp',
      'wasm/src/core/functions.cpp',
      'wasm/src/core/functions_dsp.cpp',
      'wasm/src/core/functions_hof.cpp',
      'wasm/src/core/value.cpp',
      'wasm/src/parser/lexer.cpp',
      'wasm/src/parser/parser.cpp',
      'wasm/src/parser/evaluator.cpp',
      'wasm/src/bindings/main.cpp',
      '-I', 'wasm/src',
      '-o', 'dist/achronyme-core.mjs',
      '-s', 'WASM=1',
      '-s', 'ALLOW_MEMORY_GROWTH=1',
      '-s', 'MODULARIZE=1',
      '-s', 'EXPORT_ES6=1',
      '-s', 'EXPORT_NAME=AchronymeCore',
      '-s', 'ENVIRONMENT=web,worker,node',
      '--bind',
      '-fexceptions',
      '-std=c++17'
    ];

    // Add optimization flags
    if (isDev) {
      args.push('-O0', '-g');
    } else {
      args.push('-O3');
    }

    const emcc = spawn('emcc', args, {
      cwd: rootDir,
      stdio: 'inherit'
    });

    emcc.on('close', (code) => {
      if (code !== 0) {
        console.error('\n❌ Build failed!');
        reject(new Error(`emcc exited with code ${code}`));
        return;
      }

      console.log('\n✅ Build complete!');

      if (isDev) {
        console.log('⚠️  This is a development build (not optimized)');
        console.log('   Use \'npm run build:wasm\' for production builds');
      }

      resolve();
    });

    emcc.on('error', (err) => {
      console.error('\n❌ Failed to start emcc:', err.message);
      reject(err);
    });
  });
}

// Main execution
async function main() {
  try {
    await checkEmscripten();
    await showVersion();
    await buildWasm();

    console.log('\n🎉 Ready to use!');
    console.log('   Import in JS: import AchronymeCore from \'./dist/achronyme-core.mjs\'');
  } catch (error) {
    console.error('Build failed:', error.message);
    process.exit(1);
  }
}

main();
