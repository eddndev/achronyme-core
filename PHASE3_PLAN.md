# Phase 3 Implementation Plan: Complex Types

**Date**: 2025-10-26
**Status**: Planning
**Objective**: Implement Complex numbers, Vectors, and Matrices with full arithmetic support

---

## Overview

Phase 3 introduces a **type system** to Achronyme Core, expanding from scalar numbers to support:
1. **Complex numbers** (a + bi)
2. **Vectors** ([x, y, z, ...])
3. **Matrices** ([[a, b], [c, d], ...])

This requires a fundamental architectural change: the `Value` class must become a **variant type** that can hold different data types.

---

## Architecture Changes

### 1. Value Type System

**Current** (Phase 2):
```cpp
class Value {
private:
    double data_;
};
```

**New** (Phase 3):
```cpp
class Value {
public:
    enum class Type { NUMBER, COMPLEX, VECTOR, MATRIX };

    Type type() const;

    // Type checking
    bool isNumber() const;
    bool isComplex() const;
    bool isVector() const;
    bool isMatrix() const;

    // Type access
    double asNumber() const;
    Complex asComplex() const;
    Vector asVector() const;
    Matrix asMatrix() const;

private:
    std::variant<double, Complex, Vector, Matrix> data_;
};
```

### 2. New Data Types

#### Complex
```cpp
class Complex {
public:
    Complex(double real, double imag);

    double real() const;
    double imag() const;
    double magnitude() const;  // |z| = sqrt(a² + b²)
    double argument() const;    // arg(z) = atan2(b, a)
    Complex conjugate() const;  // conj(z) = a - bi

    Complex operator+(const Complex& other) const;
    Complex operator-(const Complex& other) const;
    Complex operator*(const Complex& other) const;
    Complex operator/(const Complex& other) const;
    Complex pow(const Complex& exponent) const;

private:
    double real_;
    double imag_;
};
```

#### Vector
```cpp
class Vector {
public:
    Vector(std::vector<double> elements);

    size_t size() const;
    double operator[](size_t index) const;

    double norm() const;           // ||v|| = sqrt(sum(xi²))
    Vector normalize() const;      // v / ||v||
    double dot(const Vector& other) const;
    Vector cross(const Vector& other) const;  // Only for 3D

    Vector operator+(const Vector& other) const;
    Vector operator-(const Vector& other) const;
    Vector operator*(double scalar) const;

private:
    std::vector<double> elements_;
};
```

#### Matrix
```cpp
class Matrix {
public:
    Matrix(size_t rows, size_t cols, std::vector<double> data);

    size_t rows() const;
    size_t cols() const;
    double at(size_t row, size_t col) const;

    Matrix transpose() const;
    double determinant() const;
    Matrix inverse() const;

    Matrix operator+(const Matrix& other) const;
    Matrix operator-(const Matrix& other) const;
    Matrix operator*(const Matrix& other) const;
    Matrix operator*(double scalar) const;

private:
    size_t rows_;
    size_t cols_;
    std::vector<double> data_;  // Row-major order
};
```

---

## Syntax Extensions

### Complex Numbers

**Syntax options**:
1. `3 + 4i` (natural syntax)
2. `complex(3, 4)` (function call)
3. `i` as imaginary unit constant

**Examples**:
```
3 + 4i                    → Complex(3, 4)
(2 + 3i) * (1 - 2i)       → Complex(8, -1)
abs(3 + 4i)               → 5
arg(1 + i)                → π/4 = 0.785...
conj(3 + 4i)              → 3 - 4i
```

### Vectors

**Syntax**: `[element1, element2, ...]`

**Examples**:
```
[1, 2, 3]                 → Vector([1, 2, 3])
[1, 2] + [3, 4]           → [4, 6]
[1, 2, 3] * 2             → [2, 4, 6]
dot([1, 2], [3, 4])       → 11
cross([1, 0, 0], [0, 1, 0]) → [0, 0, 1]
norm([3, 4])              → 5
```

### Matrices

**Syntax**: `[[row1], [row2], ...]`

**Examples**:
```
[[1, 2], [3, 4]]          → Matrix 2x2
[[1, 2], [3, 4]] + [[5, 6], [7, 8]]  → [[6, 8], [10, 12]]
[[1, 2], [3, 4]] * [[5, 6], [7, 8]]  → [[19, 22], [43, 50]]
transpose([[1, 2], [3, 4]])          → [[1, 3], [2, 4]]
det([[1, 2], [3, 4]])                → -2
```

---

## Implementation Steps

### Step 1: Core Types (C++)

**Files to create**:
- `wasm/src/core/complex.hpp`
- `wasm/src/core/complex.cpp`
- `wasm/src/core/vector.hpp`
- `wasm/src/core/vector.cpp`
- `wasm/src/core/matrix.hpp`
- `wasm/src/core/matrix.cpp`

**Files to modify**:
- `wasm/src/core/value.hpp` (change to variant)
- `wasm/src/core/value.cpp` (update all operations)

### Step 2: Lexer Extensions

**New tokens**:
- `LBRACKET` (`[`)
- `RBRACKET` (`]`)
- `IMAGINARY_UNIT` (`i`) - special case

**Modified**:
- Handle `i` as both identifier and imaginary unit
- Distinguish `2i` from `2 * i`

### Step 3: Parser Extensions

**New parsing methods**:
```cpp
std::unique_ptr<ASTNode> parseVector();
std::unique_ptr<ASTNode> parseMatrix();
std::unique_ptr<ASTNode> parseComplexLiteral();
```

**Grammar updates**:
```bnf
primary → NUMBER
        | NUMBER 'i'              // 3i
        | IDENTIFIER ('(' args ')')?
        | '(' expression ')' 'i'?  // (2 + 3)i
        | '[' vectorElements ']'
        | '[[' matrixRows ']]'

vectorElements → expression (',' expression)*
matrixRows     → '[' vectorElements ']' (',' '[' vectorElements ']')*
```

### Step 4: AST Extensions

**New node types**:
```cpp
class ComplexLiteralNode : public ASTNode {
    double real_;
    double imag_;
};

class VectorLiteralNode : public ASTNode {
    std::vector<std::unique_ptr<ASTNode>> elements_;
};

class MatrixLiteralNode : public ASTNode {
    std::vector<std::vector<std::unique_ptr<ASTNode>>> rows_;
};
```

### Step 5: Evaluator Extensions

**Type checking and coercion**:
- Number + Number → Number
- Complex + Complex → Complex
- Number + Complex → Complex (coerce number to complex)
- Vector + Vector → Vector (same dimensions)
- Matrix + Matrix → Matrix (same dimensions)

**Operator overloading by type**:
```cpp
Value Evaluator::evaluateBinaryOp(
    const BinaryOpNode* node,
    const Value& left,
    const Value& right
) {
    // Dispatch based on types
    if (left.isNumber() && right.isNumber()) {
        return evaluateNumberOp(node->op(), left, right);
    }
    else if (left.isComplex() || right.isComplex()) {
        return evaluateComplexOp(node->op(), left, right);
    }
    else if (left.isVector() && right.isVector()) {
        return evaluateVectorOp(node->op(), left, right);
    }
    // ... etc
}
```

### Step 6: Function Extensions

**Complex functions**:
- `complex(real, imag)` - create complex
- `real(z)` - extract real part
- `imag(z)` - extract imaginary part
- `abs(z)` - magnitude
- `arg(z)` - argument/phase
- `conj(z)` - conjugate

**Vector functions**:
- `vector(...)` - create vector
- `dot(v1, v2)` - dot product
- `cross(v1, v2)` - cross product (3D only)
- `norm(v)` - magnitude
- `normalize(v)` - unit vector

**Matrix functions**:
- `matrix(...)` - create matrix
- `transpose(M)` - transpose
- `det(M)` - determinant
- `inverse(M)` - inverse (if exists)
- `trace(M)` - trace
- `rank(M)` - rank (future)
- `eigenvalues(M)` - eigenvalues (future)

### Step 7: Bindings and TypeScript

**Emscripten bindings**:
```cpp
// Need to expose Complex, Vector, Matrix to JS
class_<Complex>("Complex")
    .constructor<double, double>()
    .property("real", &Complex::real)
    .property("imag", &Complex::imag)
    .function("magnitude", &Complex::magnitude);

class_<Vector>("Vector")
    .constructor<std::vector<double>>()
    .function("size", &Vector::size)
    .function("norm", &Vector::norm);

class_<Matrix>("Matrix")
    .constructor<size_t, size_t, std::vector<double>>()
    .function("rows", &Matrix::rows)
    .function("cols", &Matrix::cols);
```

**TypeScript wrapper**:
```typescript
export interface ComplexNumber {
  real: number;
  imag: number;
  magnitude: number;
}

export interface VectorValue {
  size: number;
  elements: number[];
}

export interface MatrixValue {
  rows: number;
  cols: number;
  data: number[];
}

export type SOCValue = number | ComplexNumber | VectorValue | MatrixValue;

// Update eval signature
eval(expression: string): SOCValue;
```

---

## Testing Strategy

### Complex Number Tests
```javascript
'3 + 4i'                           → Complex(3, 4)
'(2 + 3i) + (1 - 2i)'             → Complex(3, 1)
'(2 + 3i) * (1 - 2i)'             → Complex(8, -1)
'abs(3 + 4i)'                     → 5
'arg(1 + i)'                      → 0.785... (π/4)
'conj(3 + 4i)'                    → Complex(3, -4)
'(3 + 4i) ^ 2'                    → Complex(-7, 24)
'real(3 + 4i)'                    → 3
'imag(3 + 4i)'                    → 4
```

### Vector Tests
```javascript
'[1, 2, 3]'                       → Vector([1, 2, 3])
'[1, 2] + [3, 4]'                 → Vector([4, 6])
'[1, 2, 3] * 2'                   → Vector([2, 4, 6])
'dot([1, 2, 3], [4, 5, 6])'       → 32
'cross([1, 0, 0], [0, 1, 0])'     → Vector([0, 0, 1])
'norm([3, 4])'                    → 5
'normalize([3, 4])'               → Vector([0.6, 0.8])
```

### Matrix Tests
```javascript
'[[1, 2], [3, 4]]'                → Matrix 2x2
'[[1, 2], [3, 4]] + [[5, 6], [7, 8]]'  → Matrix([[6,8],[10,12]])
'[[1, 2], [3, 4]] * [[1, 0], [0, 1]]'  → Matrix([[1,2],[3,4]])
'transpose([[1, 2], [3, 4]])'          → Matrix([[1,3],[2,4]])
'det([[1, 2], [3, 4]])'                → -2
'det([[3, 8], [4, 6]])'                → -14
```

### Mixed Type Tests
```javascript
'2 + (3 + 4i)'                    → Complex(5, 4)  // Number promoted
'[1, 2, 3] + 5'                   → Vector([6, 7, 8])  // Broadcast
'[[1, 2], [3, 4]] * 2'            → Matrix([[2,4],[6,8]])
```

---

## Success Criteria

- [ ] Complex class with +, -, *, /, ^ operations
- [ ] Vector class with +, -, *, dot, cross operations
- [ ] Matrix class with +, -, *, transpose, det operations
- [ ] Value type supports all three new types
- [ ] Lexer recognizes `i`, `[`, `]` tokens
- [ ] Parser can parse complex literals, vectors, matrices
- [ ] Evaluator handles type checking and coercion
- [ ] All 30+ test expressions pass
- [ ] WASM compiles successfully
- [ ] TypeScript bindings work correctly
- [ ] Performance remains excellent (<50μs for complex operations)

---

## Estimated Size Impact

- **Current WASM**: 106 KB
- **Estimated after Phase 3**: ~180-220 KB
- **Reason**: Complex arithmetic, vector/matrix operations, linear algebra

---

## Development Order

1. **Complex numbers** (simplest, most useful)
2. **Vectors** (moderate complexity)
3. **Matrices** (most complex, builds on vectors)

This allows incremental testing and validation at each stage.

---

## Future Phase 4+ Features

- Matrix decompositions (LU, QR, SVD)
- Eigenvalue/eigenvector computation
- Linear system solving (Ax = b)
- Sparse matrix support
- GPU acceleration for large matrices

---

**Ready to implement!** 🚀
