# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned Features
- Mathematical functions (sin, cos, exp, log, etc.)
- Mathematical constants (PI, E, PHI, etc.)
- Complex number support
- Vector operations
- Higher-order functions (map, reduce, compose)

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

[Unreleased]: https://github.com/eddndev/achronyme-core/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/eddndev/achronyme-core/releases/tag/v0.1.0
