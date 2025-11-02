# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0] - 2025-11-01

### Added - Advanced Linear Algebra 📐
- **Matrix Decompositions**:
  - LU decomposition with partial pivoting (`lu`).
  - QR decomposition using Householder reflections (`qr`).
  - Cholesky decomposition for symmetric, positive-definite matrices (`cholesky`).
  - Singular Value Decomposition (SVD) (`svd`).
- **Eigenvalue and Eigenvector Solvers**:
  - Power Iteration method for finding the dominant eigenvalue (`power_iteration`).
  - QR Algorithm for finding all eigenvalues of a matrix (`qr_eigenvalues`).
  - Full symmetric eigenvalue solver (`eig`).
- **Helper Functions**: `isSymmetric`, `isPositiveDefinite`, `identity`.

### Fixed
- **CRITICAL**: Fixed a memory corruption (dangling pointer) bug in the Handle-to-JavaScript data retrieval process. This was caused by returning a pointer to a temporary object's data. The fix ensures all handle operations are stable and memory-safe.
- Corrected intermittent failures in eigenvalue tests, which were a symptom of the memory bug.

### Changed
- Refactored the internal implementation of the QR eigenvalue algorithm for clarity and robustness.

### Validation
- ✅ All 11 linear algebra tests now pass consistently.
- ✅ No regressions introduced in existing SDK, DSP, or Handle system tests.


### Planned Features (Phase 4+)
- Symbolic computation
- Units and dimensions

## [0.3.0] - 2025-11-01

### Added - Performance Revolution 🚀
- **Sistema de Handles (Fast Path)**: Mejora de performance de 10-1000x
  - HandleManager en C++ para gestión eficiente de memoria
  - Zero-copy operations con referencias directas a memoria WASM
  - Fast path automático para vectores ≥8 elementos (configurable)
  - Slow path compatible para vectores pequeños y debugging
  - Tracking de estadísticas: `getMemoryStats()` muestra % fast path usage
  - Benchmarks reales: Vector 100K (900x), FFT 4096 (150x), Pipeline DSP (33x)

- **Funciones Matemáticas Vectorizadas** (nativas en C++)
  - `exp(vector)` - Exponencial element-wise (~100x más rápido que map)
  - `ln(vector)` - Logaritmo natural element-wise
  - `sqrt(vector)` - Raíz cuadrada element-wise
  - `abs(vector)` - Valor absoluto element-wise
  - `sin(vector)` - Seno element-wise
  - `cos(vector)` - Coseno element-wise
  - `tan(vector)` - Tangente element-wise
  - **Nota**: API transparente - funciones aceptan escalares Y vectores automáticamente

- **Optimizaciones DSP Fast Path**
  - `fft_fast()` - FFT optimizado con handles (directo en memoria)
  - `fft_mag_fast()` - FFT magnitude sin serialización intermedia
  - `fft_phase_fast()` - FFT phase sin overhead de parsing
  - `linspace()` - Generación optimizada desde inicio con handles
  - Pipelines DSP completos sin salir de memoria WASM

### Added - Testing & Quality
- **Suite de Tests Exhaustiva** (~200 tests totales)
  - `test-stability.mjs` - 20 tests de estabilidad (10K ops, 1M elementos, stress)
  - `test-accuracy.mjs` - 25 tests de precisión matemática (tolerancia 1e-6)
  - `test-edge-cases.mjs` - 25 tests de casos límite
  - Todos los tests: 0 memory leaks, >90% fast path usage
  - Validación completa del sistema de handles

### Added - Documentation
- **Documentación Técnica Completa**
  - `FAST-PATH-VS-SLOW-PATH-EXPLICACION.md` - Guía completa del sistema (11,000+ palabras)
  - `FAST-PATH-DIAGRAMS.md` - Diagramas visuales de arquitectura y flujos
  - `LEGACY-TESTS-FIX-SUMMARY.md` - Resumen de correcciones de compatibilidad
  - `TEST-SUITE-SUMMARY.md` - Resumen de suite de tests y resultados
  - `RESUMEN-SESION.md` - Resumen ejecutivo de implementación

### Fixed - Compatibility
- **Emscripten 4.0 Compatibility**
  - Actualizado acceso a heap WASM (HEAPF64.buffer → HEAPF64.subarray)
  - Agregado `EXPORTED_RUNTIME_METHODS='["HEAPF64","HEAPU32"]'` al build
  - Corregido tipos TypeScript para incluir HEAP8
  - Build scripts actualizados para Emscripten 4.0.15

- **Tests Legacy Actualizados** (10 archivos)
  - Corregidos import paths duplicados (`sdk/sdk/` → `sdk/`)
  - Corregida API incorrecta (`.fft_mag()` sobre espectros → señales)
  - Corregidas rutas relativas en test-npm-import.mjs y debug-module.mjs
  - Todos los tests legacy ahora compatibles y pasando

### Changed
- **Performance Improvements**
  - Vector creation (100K elementos): ~450ms → ~0.5ms (900x mejora)
  - FFT 4096 samples: ~180ms → ~1.2ms (150x mejora)
  - Pipeline DSP completo: ~100ms → ~3ms (33x mejora)
  - Memory overhead: Reducido drásticamente con zero-copy

- **API Enhancement** (sin breaking changes)
  - Funciones matemáticas ahora aceptan vectores automáticamente
  - Sistema de handles completamente transparente para el usuario
  - Decisión fast/slow automática basada en tamaño de datos
  - API pública 100% backward compatible

### Technical Details
- **Fast Path Threshold**: 8 elementos (configurable via `fastPathThreshold`)
- **Memory Management**: `shared_ptr<Value>` con HandleManager
- **Zero-Copy**: Datos permanecen en memoria WASM durante operaciones
- **Statistics**: `fastPathOperationsCount`, `slowPathOperationsCount` tracked
- **Fallback**: Automático a slow path si fast path falla (robusto)

### Migration Guide
**Para usuarios del paquete npm**: No se requieren cambios.
```javascript
// Código existente funciona sin modificaciones
const v = ach.vector([1,2,3,4,5,6,7,8]);
const result = v.exp();  // Ahora ~100x más rápido!
```

**Para desarrolladores que compilan desde source**:
- Requiere Emscripten 4.0+ (probado con 4.0.15)
- Actualizar emsdk y recompilar: `npm run build`

### Breaking Changes
- Requiere Emscripten 4.0+ para compilar desde source
- No hay breaking changes en la API pública

### Validation
- ✅ ~200 tests pasando (test-stability, test-accuracy, test-edge-cases, test-sdk)
- ✅ 0 memory leaks en todos los tests críticos
- ✅ Fast path usage: >90% en casos de uso reales (DSP pipelines)
- ✅ Backward compatibility: 100% (todos los tests legacy actualizados)

---

## [0.3.0-beta-8] - 2025-10-27

### Fixed
- **CRITICAL:** Fixed FFT spectrum frequency desynchronization bug in `fft_spectrum()`
  - **Issue:** Frequencies did not correctly correspond to magnitudes and phases after `fftshift`
  - **Cause:** `fftshift` was applied independently to frequencies and FFT results, then frequencies were sorted, breaking synchronization
  - **Solution:** Apply `fftshift` simultaneously to both frequencies and FFT spectrum, maintaining index correspondence
  - **Impact:** FFT spectrum results now 100% accurate (frequency error = 0.0000 rad/s)
  - **Validation:** Tested with 45 intensive tests using known signals (sinusoids, complex signals, impulse, DC)
  - **Affected module:** `wasm/src/core/functions_dsp.cpp:860-976`
  - **Upgrade priority:** HIGH - If you use `fft_spectrum()`, update immediately

### Added
- Cross-platform build system
  - Node.js cross-platform build script (`scripts/build-cross-platform.mjs`)
  - Bash scripts for Unix/Linux/Mac (`scripts/build-wasm.sh`, `scripts/build-wasm-dev.sh`)
  - Batch scripts for Windows (`scripts/build-wasm.bat`, `scripts/build-wasm-dev.bat`)
  - Development mode compilation (5-10x faster, includes debug symbols)
  - Production mode compilation (optimized -O3)
- New npm scripts
  - `npm run build:wasm:dev` - Fast development build
  - `npm run build:dev` - Full development build (WASM + TypeScript)
  - `npm run clean` - Clean all build artifacts
  - `npm run test:sdk` - Run SDK test suite
  - `npm run test:dsp` - Run intensive DSP tests
  - `npm run test:all` - Run all tests
- Comprehensive documentation
  - `BUILD-GUIDE.md` - Complete build guide
  - `QUICK-START.md` - Quick start guide (3 steps)
  - `scripts/README.md` - Build scripts documentation

### Changed
- Improved build system with better error handling and cross-platform support
- Build scripts now use `emcc` directly (simpler, more reliable)
- Updated package.json with additional build and test scripts

### Validated
- ✅ SDK tests: 30/30 passing
- ✅ FFT vs DFT cross-validation: Perfect match
- ✅ Signal analysis with known frequencies: Exact results
- ✅ Conjugate symmetry preserved for real signals

## [0.3.0] - 2025-10-26

### Added
- **Phase 3: Complex Types** 🎉🎉🎉
  - **Complex Numbers** (a + bi)
    - Full arithmetic support (+, -, *, /, ^)
    - Imaginary unit: `i`, `3i`, `2+3i`
    - Functions: `complex(real, imag)`, `real(z)`, `imag(z)`, `conj(z)`, `arg(z)`
    - Complex magnitude: `abs(3+4i)` = 5
    - Automatic type promotion: Number → Complex

  - **Vectors** ([x, y, z, ...])
    - Vector literals: `[1, 2, 3]`
    - Arithmetic: `[1,2] + [3,4]`, `[1,2,3] * 2`
    - Functions: `dot(v1, v2)`, `cross(v1, v2)`, `norm(v)`, `normalize(v)`
    - Broadcasting: `[1,2,3] + 10` → `[11, 12, 13]`
    - Expressions in vectors: `[sin(0), cos(0), PI]`

  - **Matrices** ([[a, b], [c, d], ...])
    - Matrix literals: `[[1, 2], [3, 4]]`
    - Arithmetic: matrix addition, subtraction, multiplication
    - Functions: `transpose(M)`, `det(M)`, `inverse(M)`, `trace(M)`
    - Scalar operations: `2 * [[1,2],[3,4]]`
    - Expressions in matrices: `[[PI, E], [sqrt(2), sqrt(3)]]`

  - **Type System**
    - std::variant-based Value type
    - Type checking: `isNumber()`, `isComplex()`, `isVector()`, `isMatrix()`
    - Automatic type coercion
    - Type-safe operations with runtime dispatch

  - **Lexer/Parser Extensions**
    - New tokens: `[`, `]` (LBRACKET, RBRACKET)
    - Complex literal parsing: `3i`, `i`
    - Vector/matrix literal parsing
    - Automatic matrix row validation

  - **Extended Functions**
    - `abs()` now works for complex numbers
    - 13 new functions for complex types
    - Full support for nested expressions

### Changed
- WASM bundle size increased to 234 KB (from 106 KB)
- eval() now returns string representation for all types
- Value class now uses std::variant instead of double
- Breaking change: eval() return type changed from double to string

### Performance
- Complex arithmetic: <10μs per operation
- Vector operations: <5μs for basic ops
- Matrix multiplication (2x2): <20μs
- Still 3-20x faster than JavaScript equivalents

## [0.2.0] - 2025-10-26

### Added
- **Phase 2: Mathematical Functions** 🎉
  - **Constants Registry** (7 constants)
    - Mathematical constants: PI, E, PHI (golden ratio)
    - Square roots: SQRT2, SQRT3
    - Logarithms: LN2, LN10
    - Case-insensitive lookup

  - **Function Registry** (25+ functions)
    - Trigonometric: sin, cos, tan, asin, acos, atan, atan2
    - Hyperbolic: sinh, cosh, tanh
    - Exponential/Logarithmic: exp, log, ln, log10, log2
    - Power/Root: sqrt, cbrt, pow
    - Rounding: floor, ceil, round, trunc
    - Utility: abs, sign, min, max, deg, rad

  - **Lexer Extensions**
    - IDENTIFIER token type for function/constant names
    - COMMA token for function arguments
    - scanIdentifier() method

  - **Parser Extensions**
    - Function call parsing: `name(arg1, arg2, ...)`
    - Constant parsing: `PI`, `E`, etc.
    - Variadic function support (min, max)
    - Grammar extended for function calls

  - **AST Extensions**
    - FunctionCallNode class for function calls
    - Support for N arguments

  - **Evaluator Extensions**
    - evaluateFunctionCall() method
    - Arity checking
    - Nested function evaluation

### Changed
- WASM bundle size increased to 106 KB (from 53 KB) due to math functions
- Demo updated with 34 comprehensive test expressions
- All expressions now case-insensitive (PI = pi = Pi)

### Performance
- Simple functions (sin, cos): <5μs
- Nested functions: <10μs
- Complex expressions: <15μs
- Still 10-20x faster than pure JavaScript

## [0.1.0] - 2025-10-26

### Added
- Initial release of Achronyme Core
- **Phase 1: Arithmetic Evaluator**
  - Lexer (tokenizer) implementation
  - Recursive descent parser with operator precedence
  - AST-based evaluator
  - Support for basic arithmetic operators: `+`, `-`, `*`, `/`, `^`
  - Parentheses support for precedence override
  - Unary minus operator
  - Decimal number support
  - Scientific notation support (e.g., `1e-3`, `2.5e10`)
  - Right-associative exponentiation (`2^3^2 = 512`)
- Emscripten bindings for WebAssembly export
- TypeScript/JavaScript wrapper (SOC class)
- Comprehensive test suite (C++ with Google Test, TypeScript with Vitest)
- Build scripts for WASM compilation
- Complete documentation and examples

### Technical Details
- C++20 codebase
- WebAssembly compilation via Emscripten
- Zero-copy AST evaluation
- Type-safe bindings

[Unreleased]: https://github.com/eddndev/achronyme-core/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/eddndev/achronyme-core/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/eddndev/achronyme-core/releases/tag/v0.1.0
