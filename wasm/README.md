# Achronyme Core - WASM Module

This directory contains the C++ implementation of the SOC (Superior Order Calculator).

## Structure

```
wasm/
├── src/
│   ├── core/           # Core types (Value, Complex, Vector, etc.)
│   ├── parser/         # Lexer, Parser, Evaluator
│   ├── dsp/            # Digital Signal Processing (Phase 5+)
│   ├── linalg/         # Linear Algebra (Phase 5+)
│   ├── numerical/      # Numerical Methods (Phase 5+)
│   ├── optimization/   # Optimization algorithms (Phase 5+)
│   └── bindings/       # Emscripten bindings
├── tests/              # C++ tests (Google Test)
└── CMakeLists.txt
```

## Building

### With Emscripten (for WebAssembly)

```bash
# From the project root
npm run build:wasm
```

### Native build (for testing)

```bash
mkdir -p build
cd build
cmake ..
make
ctest
```

## Current Phase: Phase 1 - Arithmetic Evaluator

Implemented:
- ✅ Lexer (tokenizer)
- ✅ Parser (recursive descent with precedence)
- ✅ Evaluator (AST walker)
- ✅ Emscripten bindings

Supported expressions:
- Basic arithmetic: `2 + 3 * 4` → `14`
- Parentheses: `(2 + 3) * 4` → `20`
- Exponentiation: `2 ^ 3 ^ 2` → `512` (right-associative)
- Unary minus: `-5 + 3` → `-2`
- Decimals: `3.14 * 2` → `6.28`
- Scientific notation: `1e-3 + 2e10` → `20000000000.001`

## Next Phases

- **Phase 2**: Add mathematical functions (sin, cos, exp, log, etc.) and constants (PI, E)
- **Phase 3**: Add Complex numbers and Vectors
- **Phase 4**: Add higher-order functions (map, reduce, compose)
- **Phase 5+**: Add specialized modules (DSP, LinAlg, etc.)
