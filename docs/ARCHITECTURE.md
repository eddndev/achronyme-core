# Achronyme Core - Architecture

## Overview

Achronyme Core is a mathematical computation engine built with **C++** and compiled to **WebAssembly** for near-native performance in browsers.

---

## 🏗️ System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    JavaScript/TypeScript Layer              │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐  │
│  │  SOC (Superior Order Calculator) Class              │  │
│  │  - init(): Initialize WASM module                   │  │
│  │  - eval(expression: string): number                 │  │
│  └─────────────────────────────────────────────────────┘  │
│                          ↓ calls ↓                          │
│  ┌─────────────────────────────────────────────────────┐  │
│  │  WASM Loader                                        │  │
│  │  - loadWASM(): Load and initialize WASM module      │  │
│  └─────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                           ↓ FFI ↓
┌─────────────────────────────────────────────────────────────┐
│              Emscripten Bindings Layer (C++)                │
│                                                             │
│  EMSCRIPTEN_BINDINGS(achronyme_core) {                     │
│    function("eval", &eval);                                │
│  }                                                          │
└─────────────────────────────────────────────────────────────┘
                           ↓ calls ↓
┌─────────────────────────────────────────────────────────────┐
│                   Core Computation Engine (C++)             │
│                                                             │
│  ┌──────────────┐   ┌──────────────┐   ┌──────────────┐  │
│  │   Lexer      │ → │   Parser     │ → │  Evaluator   │  │
│  │  (Tokenizer) │   │ (AST Builder)│   │ (AST Walker) │  │
│  └──────────────┘   └──────────────┘   └──────────────┘  │
│                                                             │
│  Input: "2+3*4" → Tokens → AST → Result: 14               │
└─────────────────────────────────────────────────────────────┘
```

---

## 📦 Module Structure

### 1. **Core Module** (`wasm/src/core/`)

**Purpose**: Fundamental data types and utilities

**Components**:
- `value.hpp/cpp`: Base `Value` type (currently `double`, will expand to Complex, Vector, etc.)

**Future**:
- Complex numbers
- Vectors
- Matrices
- Type system

---

### 2. **Parser Module** (`wasm/src/parser/`)

**Purpose**: Convert string expressions to executable AST

#### 2.1 Lexer (`lexer.hpp/cpp`)

**Input**: String expression
**Output**: Token stream

```cpp
"2 + 3 * 4"
    ↓
[NUMBER(2), PLUS, NUMBER(3), STAR, NUMBER(4), END]
```

**Supported Tokens**:
- `NUMBER`: Decimal numbers, scientific notation
- `PLUS`, `MINUS`, `STAR`, `SLASH`, `CARET`: Operators
- `LPAREN`, `RPAREN`: Parentheses
- `END`: End of input

**Algorithm**: Single-pass linear scan with lookahead

---

#### 2.2 AST (`ast.hpp`)

**Purpose**: Abstract Syntax Tree representation

**Node Types**:
```cpp
enum class ASTNodeType {
    NUMBER,      // Literal: 42, 3.14
    BINARY_OP,   // Operations: +, -, *, /, ^
    UNARY_OP     // Unary minus: -x
};
```

**Example AST for `2 + 3 * 4`**:
```
      +
     / \
    2   *
       / \
      3   4
```

**Design Pattern**: Composite pattern with smart pointers (`std::unique_ptr`)

---

#### 2.3 Parser (`parser.hpp/cpp`)

**Input**: Token stream
**Output**: AST

**Algorithm**: **Recursive Descent Parser** with explicit precedence

**Grammar**:
```bnf
expression  → term (('+' | '-') term)*
term        → factor (('*' | '/') factor)*
factor      → exponent ('^' exponent)*
exponent    → '-' exponent | primary
primary     → NUMBER | '(' expression ')'
```

**Precedence Levels** (highest to lowest):
1. Parentheses `()`
2. Unary minus `-`
3. Exponentiation `^` (right-associative)
4. Multiplication `*`, Division `/`
5. Addition `+`, Subtraction `-`

**Key Features**:
- **Right-associative power**: `2^3^2 = 2^(3^2) = 512`
- **Parentheses override**: `(2+3)*4 = 20`
- **Error detection**: Syntax errors throw exceptions

---

#### 2.4 Evaluator (`evaluator.hpp/cpp`)

**Input**: AST
**Output**: Result (`Value`)

**Algorithm**: **Post-order traversal** of AST

```cpp
eval(+)
  ├─ eval(2) → 2
  └─ eval(*)
      ├─ eval(3) → 3
      └─ eval(4) → 4
      Result: 12
  Result: 2 + 12 = 14
```

**Operations**:
```cpp
switch (node->op()) {
    case BinaryOp::ADD:      return left + right;
    case BinaryOp::SUBTRACT: return left - right;
    case BinaryOp::MULTIPLY: return left * right;
    case BinaryOp::DIVIDE:   return left / right;
    case BinaryOp::POWER:    return left.pow(right);
}
```

---

### 3. **Bindings Module** (`wasm/src/bindings/`)

**Purpose**: Expose C++ functions to JavaScript via Emscripten

**Example**:
```cpp
#include <emscripten/bind.h>

double eval(const std::string& expression) {
    Lexer lexer(expression);
    auto tokens = lexer.tokenize();

    Parser parser(tokens);
    auto ast = parser.parse();

    Evaluator evaluator;
    auto result = evaluator.evaluate(ast.get());

    return result.asNumber();
}

EMSCRIPTEN_BINDINGS(achronyme_core) {
    function("eval", &eval);
}
```

**JavaScript Usage**:
```javascript
const result = Module.eval("2 + 3 * 4");
console.log(result); // 14
```

---

## 🔄 Data Flow

### Complete Evaluation Pipeline

```
┌──────────────────────────────────────────────────────────────┐
│ 1. Input                                                     │
│    "2 + 3 * 4"                                              │
└──────────────────────────────────────────────────────────────┘
                        ↓
┌──────────────────────────────────────────────────────────────┐
│ 2. Lexer (Tokenization)                                      │
│    [NUMBER(2), PLUS, NUMBER(3), STAR, NUMBER(4), END]       │
└──────────────────────────────────────────────────────────────┘
                        ↓
┌──────────────────────────────────────────────────────────────┐
│ 3. Parser (AST Construction)                                 │
│          +                                                   │
│         / \                                                  │
│        2   *                                                 │
│           / \                                                │
│          3   4                                               │
└──────────────────────────────────────────────────────────────┘
                        ↓
┌──────────────────────────────────────────────────────────────┐
│ 4. Evaluator (Post-order Traversal)                          │
│    eval(+)                                                   │
│      eval(2) = 2                                            │
│      eval(*)                                                │
│        eval(3) = 3                                          │
│        eval(4) = 4                                          │
│        3 * 4 = 12                                           │
│      2 + 12 = 14                                            │
└──────────────────────────────────────────────────────────────┘
                        ↓
┌──────────────────────────────────────────────────────────────┐
│ 5. Output                                                    │
│    14                                                        │
└──────────────────────────────────────────────────────────────┘
```

---

## 🚀 Build Process

### C++ → WebAssembly Compilation

```bash
# 1. Configure with Emscripten
emcmake cmake -DCMAKE_BUILD_TYPE=Release

# 2. Compile C++ to WASM
emmake make

# 3. Output files
#    - achronyme-core.wasm (WebAssembly binary)
#    - achronyme-core.js   (Glue code)
```

### Emscripten Flags

```cmake
-s WASM=1                    # Output WebAssembly
-s ALLOW_MEMORY_GROWTH=1     # Dynamic memory allocation
-s MODULARIZE=1              # ES6 module export
-s EXPORT_ES6=1              # ES6 syntax
-s EXPORT_NAME='AchronymeCore'
--bind                       # Enable Embind (C++/JS bindings)
-O3                          # Maximum optimization
```

---

## 🧪 Testing Strategy

### C++ Tests (Google Test)

```cpp
TEST(EvaluatorTest, BasicArithmetic) {
    EXPECT_DOUBLE_EQ(eval("2 + 3"), 5.0);
    EXPECT_DOUBLE_EQ(eval("2 * 3 * 4"), 24.0);
}
```

### TypeScript Tests (Vitest)

```typescript
it('should evaluate expressions', () => {
    expect(soc.eval('2 + 3')).toBe(5);
    expect(soc.eval('2 * 3 * 4')).toBe(24);
});
```

---

## 📈 Performance Characteristics

### Time Complexity

| Operation | Complexity | Notes |
|-----------|-----------|-------|
| Lexer | O(n) | Single-pass scan |
| Parser | O(n) | Recursive descent |
| Evaluator | O(n) | Post-order traversal |
| **Total** | **O(n)** | Linear in expression length |

### Space Complexity

| Component | Complexity | Notes |
|-----------|-----------|-------|
| Tokens | O(n) | Token array |
| AST | O(n) | Tree nodes |
| **Total** | **O(n)** | Linear memory usage |

### Optimizations

- **Zero-copy evaluation**: AST nodes use smart pointers, no deep copying
- **WASM compilation**: Near-native C++ performance
- **Future**: Constant folding, common subexpression elimination (CSE)

---

## 🗺️ Future Architecture

### Phase 2: Functions & Constants

```cpp
namespace functions {
    double sin(double x);
    double cos(double x);
    double exp(double x);
    double log(double x);
}

namespace constants {
    constexpr double PI = 3.141592653589793;
    constexpr double E = 2.718281828459045;
}
```

### Phase 3: Complex Type System

```cpp
class Value {
    std::variant<double, Complex, Vector, Matrix> data_;

    Value operator+(const Value& other) const;  // Type-aware
};
```

### Phase 5+: Specialized Modules

```
achronyme::dsp       // Digital Signal Processing
achronyme::linalg    // Linear Algebra
achronyme::numerical // Numerical Methods
achronyme::optimize  // Optimization algorithms
```

---

## 🔧 Design Decisions

### Why Recursive Descent Parser?

**Pros**:
- ✅ Simple to implement and understand
- ✅ Easy to extend (add new operators/functions)
- ✅ Natural precedence handling
- ✅ Good error messages

**Cons**:
- ❌ Not as efficient as table-driven parsers
- ❌ Left-recursion must be eliminated

**Verdict**: For mathematical expressions, simplicity and maintainability outweigh minor performance costs.

### Why WebAssembly?

**Pros**:
- ✅ Near-native performance (10-20x faster than JS)
- ✅ Language agnostic (can write in C++, Rust, etc.)
- ✅ Secure sandbox execution
- ✅ Portable across browsers

**Cons**:
- ❌ Requires compilation step
- ❌ Larger bundle size than pure JS

**Verdict**: Performance is critical for mathematical computations. WASM enables us to compete with native tools like Mathematica.

---

## 📚 References

- [Crafting Interpreters](https://craftinginterpreters.com/) - Parsing techniques
- [Emscripten Documentation](https://emscripten.org/docs/)
- [WebAssembly Specification](https://webassembly.github.io/spec/)
- [C++ Core Guidelines](https://isocpp.github.io/CppCoreGuidelines/)

---

**Last Updated**: 2025-10-26
**Version**: 0.1.0 (Phase 1)
