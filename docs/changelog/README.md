# Achronyme Core - Changelog Index

This directory contains the detailed changelog for Achronyme Core, organized by major version series.

## Quick Links

- **[Current Development (Unreleased)](./v0.6.x.md)** - Latest features in development
- **[Version 0.5.x](./v0.5.x.md)** - Rust WASM Integration & Advanced Features
- **[Version 0.4.x](./v0.4.x.md)** - Advanced Linear Algebra
- **[Archive](./archive/)** - Older versions (0.1.x - 0.3.x)

## Version Overview

### [Unreleased / v0.6.x](./v0.6.x.md)
Major features in development:
- **Phase 1 Iterators Complete** - Generators with `yield`, for-in loops, iterator protocol
- **Generator as Static Type** - Opaque type in the type system
- **Tier 3 Array Transformations** - zip, flatten, take, drop, slice, unique, chunk
- **Advanced Type System** - Function types, edge types, type aliases
- **Control Flow Enhancements** - if-else statements, early returns
- **Module System** - Import/export, mutability, do blocks
- **Environment I/O** - Save/restore REPL sessions
- **Graph Theory & PERT/CPM** - Comprehensive graph algorithms

### [Version 0.5.x](./v0.5.x.md)
Released: November 2025
- **0.5.3** - Conditional expressions, piecewise functions, parser migration to Pest
- **0.5.2** - Comprehensive built-in function reference
- **0.5.1** - Numerical calculus module (differentiation, integration, root finding)
- **0.5.0** - Rust WASM Integration & SDK v2.0, 5.25x performance improvement

### [Version 0.4.x](./v0.4.x.md)
Released: November 2025
- **0.4.0** - Advanced linear algebra (LU, QR, Cholesky, SVD decompositions, eigensolvers)

### [Archive - Older Versions](./archive/)
- **[v0.3.x](./archive/v0.3.x.md)** - Performance revolution (10-1000x improvement), complex numbers, DSP
- **[v0.2.x](./archive/v0.2.x.md)** - Mathematical functions and constants
- **[v0.1.x](./archive/v0.1.x.md)** - Initial release with arithmetic evaluator

## Changelog Format

This project follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) format and adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Each changelog entry includes:
- **Added** - New features
- **Changed** - Changes in existing functionality
- **Fixed** - Bug fixes
- **Breaking Changes** - Changes that require user action
- **Performance** - Performance improvements with benchmarks
- **Technical Details** - Implementation details for developers

## Contributing

When adding to the changelog:
1. Add new entries to [v0.6.x.md](./v0.6.x.md) under the `[Unreleased]` section
2. Use clear, descriptive headings with relevant information
3. Include code examples where helpful
4. Document any breaking changes prominently
5. Include performance metrics when applicable

## Repository Links

- [Unreleased](https://github.com/eddndev/achronyme-core/compare/v0.5.3...HEAD)
- [All Releases](https://github.com/eddndev/achronyme-core/releases)
