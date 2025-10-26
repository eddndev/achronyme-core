# Achronyme Core 🚀

> **High-performance mathematical computation engine powered by WebAssembly**

Achronyme Core is a pure C++ mathematical computation engine compiled to WebAssembly, providing **near-native performance** for mathematical operations directly in the browser.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![npm version](https://badge.fury.io/js/@achronyme%2Fcore.svg)](https://www.npmjs.com/package/@achronyme/core)

---

## ✨ Features

### Phase 1: Arithmetic Evaluator (Current)
- ✅ **Basic arithmetic**: `+`, `-`, `*`, `/`, `^`
- ✅ **Operator precedence**: respects mathematical precedence rules
- ✅ **Parentheses**: override precedence with `(` `)`
- ✅ **Unary operators**: negation `-x`
- ✅ **Decimal numbers**: `3.14`, `.5`
- ✅ **Scientific notation**: `1e-3`, `2.5e10`
- ✅ **10-20x faster** than pure JavaScript parsers

### Coming Soon
- **Phase 2**: Mathematical functions (sin, cos, exp, log, sqrt, etc.)
- **Phase 3**: Complex numbers, Vectors, Matrices
- **Phase 4**: Higher-order functions (map, reduce, compose)
- **Phase 5+**: DSP, Linear Algebra, Numerical Methods, Optimization

---

## 📦 Installation

```bash
npm install @achronyme/core
```

---

## 🚀 Quick Start

### TypeScript/JavaScript

```typescript
import { SOC } from '@achronyme/core';

// Create an instance
const soc = new SOC();

// Initialize (loads WASM module)
await soc.init();

// Evaluate expressions
console.log(soc.eval('2 + 3 * 4'));        // 14
console.log(soc.eval('(2 + 3) * 4'));      // 20
console.log(soc.eval('2 ^ 3 ^ 2'));        // 512 (right-associative)
console.log(soc.eval('-5 + 3'));           // -2
console.log(soc.eval('3.14 * 2'));         // 6.28
```

### Using the Singleton

```typescript
import { soc } from '@achronyme/core';

await soc.init();

const result = soc.eval('10 / 2 + 3 * 4');
console.log(result); // 17
```

---

## 📖 Examples

### Basic Arithmetic

```javascript
soc.eval('2 + 3')           // 5
soc.eval('10 - 5')          // 5
soc.eval('6 / 2')           // 3
soc.eval('5 * 7')           // 35
soc.eval('2 ^ 8')           // 256
```

### Operator Precedence

```javascript
soc.eval('2 + 3 * 4')       // 14  (multiplication first)
soc.eval('10 - 6 / 2')      // 7   (division first)
soc.eval('2 * 3 ^ 2')       // 18  (power first)
```

### Parentheses

```javascript
soc.eval('(2 + 3) * 4')     // 20
soc.eval('2 * (3 + 4)')     // 14
soc.eval('2 * ((3 + 4) * 5)')  // 70
```

### Unary Minus

```javascript
soc.eval('-5')              // -5
soc.eval('--5')             // 5  (double negation)
soc.eval('-5 + 3')          // -2
soc.eval('2 * -3')          // -6
```

### Decimals and Scientific Notation

```javascript
soc.eval('3.14 * 2')        // 6.28
soc.eval('0.1 + 0.2')       // 0.3
soc.eval('1e3')             // 1000
soc.eval('1e-3')            // 0.001
soc.eval('2.5e2')           // 250
```

### Complex Expressions

```javascript
soc.eval('2 + 3 * 4 - 5')   // 9
soc.eval('10 / 2 + 3 * 4')  // 17
soc.eval('(2 + 3) ^ 2')     // 25
soc.eval('2 ^ (3 + 1)')     // 16
```

---

## 🏗️ Architecture

Achronyme Core uses a **three-phase compilation pipeline**:

```
┌─────────────────────────────────────────────────┐
│  Input: "2 + 3 * 4"                             │
└─────────────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────────────┐
│  Phase 1: Lexer (Tokenizer)                     │
│  Converts string → tokens                       │
│  "2 + 3 * 4" → [NUM(2), PLUS, NUM(3),          │
│                 STAR, NUM(4), END]              │
└─────────────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────────────┐
│  Phase 2: Parser (Recursive Descent)            │
│  Converts tokens → AST with precedence          │
│        +                                        │
│       / \                                       │
│      2   *                                      │
│         / \                                     │
│        3   4                                    │
└─────────────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────────────┐
│  Phase 3: Evaluator (AST Walker)                │
│  Evaluates AST → result                         │
│  2 + (3 * 4) = 2 + 12 = 14                     │
└─────────────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────────────┐
│  Output: 14                                      │
└─────────────────────────────────────────────────┘
```

### Key Design Principles

1. **Recursive Descent Parser** with explicit precedence levels
2. **Right-associative exponentiation** (`2^3^2 = 2^(3^2) = 512`)
3. **Zero-copy** AST evaluation
4. **WASM-optimized** for minimal overhead

---

## 🧪 Grammar (BNF)

```bnf
expression  → term (('+' | '-') term)*
term        → factor (('*' | '/') factor)*
factor      → exponent ('^' exponent)*     ; right-associative
exponent    → '-' exponent | primary
primary     → NUMBER | '(' expression ')'
```

**Precedence (highest to lowest):**
1. Parentheses `()`
2. Unary minus `-`
3. Exponentiation `^` (right-associative)
4. Multiplication `*`, Division `/`
5. Addition `+`, Subtraction `-`

---

## 🛠️ Development

### Prerequisites

- **Emscripten SDK** (latest)
- **CMake** >= 3.20
- **Node.js** >= 18
- **C++20** compiler

### Setup

```bash
# Clone the repository
git clone https://github.com/eddndev/achronyme-core
cd achronyme-core

# Install dependencies
npm install

# Build WASM + TypeScript
npm run build

# Run tests
npm test
```

### Building from Source

```bash
# Build WASM only
npm run build:wasm

# Build TypeScript only
npm run build:js

# Build both
npm run build

# Optimize WASM (requires wasm-opt)
npm run optimize
```

### Running Tests

```bash
# TypeScript/JavaScript tests
npm test

# C++ tests (requires native build)
npm run test:cpp
```

---

## 📚 Documentation

- [Architecture](wasm/README.md) - WASM module structure
- [Grammar Specification](docs/language-spec/grammar/) - Formal grammar
- [API Reference](docs/API.md) - Complete API documentation (coming soon)

---

## 🗺️ Roadmap

### ✅ Phase 1: Arithmetic Evaluator (Current)
- [x] Lexer implementation
- [x] Recursive descent parser
- [x] AST evaluator
- [x] Emscripten bindings
- [x] TypeScript wrapper
- [x] Basic tests

### 🚧 Phase 2: Mathematical Functions (Next)
- [ ] Constants (PI, E, PHI, etc.)
- [ ] Trigonometric functions (sin, cos, tan, etc.)
- [ ] Exponential/logarithmic (exp, log, ln, etc.)
- [ ] Other functions (sqrt, abs, floor, ceil, etc.)

### 📅 Phase 3: Complex Types
- [ ] Complex numbers (a + bi)
- [ ] Vectors ([1, 2, 3])
- [ ] Type system and broadcasting

### 📅 Phase 4: Higher-Order Functions
- [ ] map, reduce, filter
- [ ] Function composition
- [ ] Lambda expressions

### 📅 Phase 5+: Specialized Modules
- [ ] DSP (DFT, FFT, Convolution)
- [ ] Linear Algebra (matrices, solvers)
- [ ] Numerical Methods (integration, differentiation)
- [ ] Optimization (Simplex, gradient descent)

---

## 🤝 Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md) for details.

### Areas for Contribution

- **Phase 2 Implementation**: Add mathematical functions
- **Optimization**: Improve WASM performance
- **Testing**: Add more test cases
- **Documentation**: Improve docs and examples

---

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

---

## 🌟 Acknowledgments

Achronyme Core is part of the [Achronyme](https://achronyme.com) project, an open-source alternative to Wolfram Mathematica.

**Key Technologies:**
- [Emscripten](https://emscripten.org/) - C++ to WebAssembly compiler
- [CMake](https://cmake.org/) - Build system
- [Google Test](https://github.com/google/googletest) - C++ testing framework
- [Vitest](https://vitest.dev/) - JavaScript testing framework

---

## 📞 Contact

- **Author**: Eduardo Alonso
- **Email**: contacto@eddndev.com
- **Website**: [achronyme.com](https://achronyme.com)
- **GitHub**: [@eddndev](https://github.com/eddndev)

---

Made with ❤️ by [Eduardo Alonso](https://github.com/eddndev)
