# Explicit Type System Design

## Overview

This document proposes an **optional static type system** for Achronyme that:
- Maintains backward compatibility with existing dynamic typing
- Allows explicit type annotations for function parameters, variables, and return types
- Provides compile-time (parse-time) type checking when annotations are present
- Enables better IDE support, documentation, and error catching
- Focuses on scientific computing use cases with tensor shapes and numeric precision

## Philosophy

**"Types are optional, but when specified, they are enforced"**

- **Dynamic by default**: Existing code continues to work without modification
- **Gradual typing**: Add types incrementally where needed
- **Type inference**: Annotated functions infer types for local variables where possible
- **Runtime safety**: Type errors fail fast with clear messages
- **Documentation**: Types serve as machine-verified documentation

## Motivation

### Current System (Dynamic Only)

```javascript
// Works, but no guarantees
let add = (a, b) => a + b

add(5, 10)        // 15 (OK)
add("hello", 42)  // Runtime error: Incompatible types
add([1, 2], [3])  // Runtime error: Shape mismatch
```

### With Explicit Types

```javascript
// Type-safe, errors caught early
let add = (a: Number, b: Number): Number => a + b

add(5, 10)        // 15 (OK)
add("hello", 42)  // Parse-time error: Expected Number, got String
add([1, 2], [3])  // Parse-time error: Expected Number, got Tensor
```

## Type Syntax

### Basic Type Annotations

#### Variable Declarations

```javascript
// Immutable with type
let x: Number = 42
let name: String = "Alice"
let active: Boolean = true

// Mutable with type
mut counter: Number = 0
counter = counter + 1  // OK
counter = "text"       // Error: Expected Number, got String

// Type inference from value
let y = 3.14           // Inferred as Number
let z: Number = 3.14   // Explicit (same result)
```

#### Function Parameters and Return Types

```javascript
// Single parameter with return type
let square = (x: Number): Number => x^2

// Multiple parameters
let add = (a: Number, b: Number): Number => a + b

// No return type annotation (inferred)
let max = (a: Number, b: Number) => if(a > b, a, b)  // Inferred: Number

// Generic array parameter
let sum = (arr: Tensor<Number>): Number => reduce((a, b) => a + b, 0, arr)

// Heterogeneous vector
let process = (data: Vector): Vector => map(x => x, data)
```

### Type System Hierarchy

```
Type
├── Number          // f64
├── Boolean         // true/false
├── String          // "text"
├── Complex         // 2 + 3i
├── Tensor<T>       // N-dimensional array (homogeneous)
│   ├── Tensor<Number>        // RealTensor
│   ├── Tensor<Complex>       // ComplexTensor
│   └── Tensor<Number, [3,3]> // Shaped tensor (3x3 matrix)
├── Vector          // Heterogeneous array (any types)
├── Record<{...}>   // Structural type
├── Function<A, B>  // Function type (A -> B)
└── Any             // Dynamic type (opt-out of checking)
```

## Core Types

### 1. Number

```javascript
let pi: Number = 3.14159
let count: Number = 42

// Functions
let sqrt = (x: Number): Number => x^0.5
let factorial = (n: Number): Number =>
    if(n <= 1, 1, n * rec(n - 1))
```

### 2. Boolean

```javascript
let isActive: Boolean = true
let hasPermission: Boolean = false

// Functions
let not = (b: Boolean): Boolean => !b
let and = (a: Boolean, b: Boolean): Boolean => a && b
```

### 3. String

```javascript
let name: String = "Alice"
let greeting: String = "Hello, World!"

// Functions
let concat = (a: String, b: String): String => a + b
let len = (s: String): Number => length(s)
```

### 4. Complex

```javascript
let z1: Complex = 2 + 3i
let z2: Complex = 5i

// Functions
let magnitude = (z: Complex): Number => sqrt(real(z)^2 + imag(z)^2)
let conjugate = (z: Complex): Complex => conj(z)
```

### 5. Tensor Types

#### Unspecified Shape

```javascript
// Generic tensor (any shape, real numbers)
let vector: Tensor<Number> = [1, 2, 3]
let matrix: Tensor<Number> = [[1, 2], [3, 4]]

// Generic complex tensor
let complexVec: Tensor<Complex> = [1+2i, 3+4i]
```

#### Specified Shape (Advanced)

```javascript
// 1D tensor (vector) with known length
let vec3: Tensor<Number, [3]> = [1, 2, 3]
let vec3_bad: Tensor<Number, [3]> = [1, 2]  // Error: Expected length 3

// 2D tensor (matrix) with known dimensions
let mat2x3: Tensor<Number, [2,3]> = [[1, 2, 3], [4, 5, 6]]
let mat2x3_bad: Tensor<Number, [2,3]> = [[1, 2]]  // Error: Wrong shape

// 3D tensor
let cube: Tensor<Number, [2,2,2]> = [
    [[1, 2], [3, 4]],
    [[5, 6], [7, 8]]
]
```

#### Dynamic Dimensions

```javascript
// Known rank, dynamic dimensions (use _ for unknown)
let matrix: Tensor<Number, [_, _]> = [[1, 2, 3], [4, 5, 6]]  // OK: 2x3
let matrix2: Tensor<Number, [_, _]> = [[1, 2]]               // OK: 1x2

// Rank 2 (matrix), any dimensions
let anyMatrix: Tensor<Number, [_, _]> = loadMatrix("data.csv")
```

### 6. Vector (Heterogeneous)

```javascript
// Generic vector (can hold any types)
let mixed: Vector = [1, "hello", true, {x: 10}]

// Type error if trying to use as Tensor
let add = (a: Tensor<Number>, b: Tensor<Number>) => a + b
add(mixed, [1, 2, 3])  // Error: Expected Tensor<Number>, got Vector
```

### 7. Record Types

#### Anonymous Records (Structural)

```javascript
// Simple record type
let point: Record<{x: Number, y: Number}> = {x: 10, y: 20}

// Nested records
let person: Record<{
    name: String,
    age: Number,
    address: Record<{
        city: String,
        country: String
    }>
}> = {
    name: "Alice",
    age: 30,
    address: {
        city: "Madrid",
        country: "Spain"
    }
}
```

#### Record with Methods

```javascript
// Record type with methods
let Point: Record<{
    x: Number,
    y: Number,
    distance: Function<[], Number>
}> = {
    x: 10,
    y: 20,
    distance: () => sqrt(self.x^2 + self.y^2)
}
```

#### Simplified Syntax (Type Aliases - Future)

```javascript
// Type alias (proposed for future)
type Point = Record<{x: Number, y: Number}>

let p1: Point = {x: 10, y: 20}
let p2: Point = {x: 5, y: 15}

let distance = (a: Point, b: Point): Number =>
    sqrt((a.x - b.x)^2 + (a.y - b.y)^2)
```

### 8. Function Types

```javascript
// Function type annotation
let apply: Function<[Number], Number> = x => x^2

// Higher-order function
let twice: Function<[Function<[Number], Number>, Number], Number> =
    (f, x) => f(f(x))

// Simplified notation (future)
let twice = (f: Number => Number, x: Number): Number => f(f(x))
```

### 9. Any Type (Opt-out)

```javascript
// Explicitly dynamic
let dynamic: Any = 42
dynamic = "hello"  // OK
dynamic = [1, 2, 3]  // OK

// Function accepting any type
let describe = (value: Any): String => str(value)
```

## Type Checking Rules

### 1. Assignment Compatibility

```javascript
let x: Number = 42        // OK
let y: Number = "text"    // Error: Incompatible types

let z: Any = 42           // OK
z = "text"                // OK (Any accepts anything)
```

### 2. Function Call Type Checking

```javascript
let add = (a: Number, b: Number): Number => a + b

add(5, 10)       // OK
add(5, "text")   // Error: Expected Number for parameter 'b', got String
add(5)           // Error: Expected 2 arguments, got 1
```

### 3. Tensor Shape Checking

```javascript
// Shape-aware operations
let matmul = (a: Tensor<Number, [_, K]>, b: Tensor<Number, [K, _]]): Tensor<Number, [_, _]> =>
    // Matrix multiplication with compatible inner dimension K
    dot(a, b)

let A: Tensor<Number, [2, 3]> = [[1, 2, 3], [4, 5, 6]]
let B: Tensor<Number, [3, 2]> = [[1, 2], [3, 4], [5, 6]]
let C: Tensor<Number, [2, 2]> = [[1, 2], [3, 4]]

matmul(A, B)  // OK: [2,3] × [3,2] = [2,2]
matmul(A, C)  // Error: Incompatible shapes [2,3] × [2,2]
```

### 4. Return Type Checking

```javascript
let square = (x: Number): Number => x^2        // OK
let bad = (x: Number): Number => "not a number"  // Error: Expected Number, got String

// Early return type checking
let sign = (x: Number): Number => if (x < 0) {
    return -1;  // OK: returns Number
    "unreachable"
} else if (x > 0) {
    return 1;   // OK: returns Number
} else {
    return 0;   // OK: returns Number
}
```

### 5. Array Type Inference

```javascript
// Homogeneous array → Tensor
let nums: Tensor<Number> = [1, 2, 3]        // Inferred from elements
let nums2 = [1, 2, 3]                       // Auto-inferred as Tensor<Number>

// Heterogeneous array → Vector
let mixed = [1, "hello", true]              // Auto-inferred as Vector
let mixed2: Vector = [1, "hello", true]     // Explicit

// Complex promotion
let complex: Tensor<Complex> = [1, 2+3i, 4] // All promoted to Complex
```

## Type Annotations in Context

### Lambda Functions

```javascript
// Without types (dynamic)
let add1 = (a, b) => a + b

// With types
let add2 = (a: Number, b: Number): Number => a + b

// Partial annotation (parameters only)
let add3 = (a: Number, b: Number) => a + b  // Return type inferred

// With do block
let processValue = (x: Number): Number => do {
    let doubled: Number = x * 2;
    let result: Number = doubled + 10;
    result
}
```

### If-Else Statements

```javascript
// Return type must be consistent across branches
let abs = (x: Number): Number => if (x < 0) {
    -x          // Type: Number (OK)
} else {
    x           // Type: Number (OK)
}

let bad = (x: Number): Number => if (x < 0) {
    -x          // Type: Number
} else {
    "positive"  // Type: String - Error: Expected Number
}
```

### Higher-Order Functions

```javascript
// map with typed callback
let double: Tensor<Number> = map(
    (x: Number): Number => x * 2,
    [1, 2, 3]
)

// filter with typed predicate
let positives: Tensor<Number> = filter(
    (x: Number): Boolean => x > 0,
    [-5, 3, -2, 8, 0]
)

// reduce with typed accumulator
let sum: Number = reduce(
    (acc: Number, x: Number): Number => acc + x,
    0,
    [1, 2, 3, 4, 5]
)
```

### Record Fields with Types

```javascript
// Record with typed fields
let config: Record<{
    host: String,
    port: Number,
    ssl: Boolean,
    timeout: Number
}> = {
    host: "localhost",
    port: 8080,
    ssl: true,
    timeout: 30000
}

// Mutable typed field
let counter: Record<{
    mut value: Number,
    increment: Function<[], Number>
}> = {
    mut value: 0,
    increment: () => do {
        self.value = self.value + 1;
        self.value
    }
}
```

## Scientific Computing Use Cases

### Linear Algebra

```javascript
// Type-safe vector operations
let dot = (a: Tensor<Number, [N]>, b: Tensor<Number, [N]>): Number =>
    sum(a * b)

// Type-safe matrix operations
let matmul = (a: Tensor<Number, [M, K]>, b: Tensor<Number, [K, N]>): Tensor<Number, [M, N]> =>
    // Implementation with shape guarantees
    ...

// Eigenvalues (matrix → vector)
let eigenvalues = (m: Tensor<Number, [N, N]>): Tensor<Number, [N]> =>
    // Square matrix required by type
    ...
```

### Signal Processing

```javascript
// FFT with length constraint
let fft = (signal: Tensor<Complex, [N]>): Tensor<Complex, [N]> =>
    // Input and output have same length
    ...

// Convolution with shape inference
let convolve = (
    signal: Tensor<Number, [N]>,
    kernel: Tensor<Number, [K]>
): Tensor<Number, [N+K-1]> =>
    // Output length is sum - 1
    ...
```

### Statistical Analysis

```javascript
// Mean requires non-empty tensor
let mean = (data: Tensor<Number, [N]>): Number =>
    sum(data) / len(data)

// Covariance matrix (NxM data → MxM covariance)
let cov = (data: Tensor<Number, [N, M]>): Tensor<Number, [M, M]> =>
    // Returns square covariance matrix
    ...

// Regression (typed inputs/outputs)
let linearRegression = (
    x: Tensor<Number, [N]>,
    y: Tensor<Number, [N]>
): Record<{slope: Number, intercept: Number, r2: Number}> =>
    // Returns structured result
    ...
```

## Implementation Strategy

### Phase 1: Grammar Extensions

Add type annotation syntax to `grammar.pest`:

```pest
// Type annotations
type_annotation = {
    simple_type
  | tensor_type
  | vector_type
  | record_type
  | function_type
  | any_type
}

simple_type = {
    "Number" | "Boolean" | "String" | "Complex"
}

tensor_type = {
    "Tensor" ~ "<" ~ type_annotation ~ ("," ~ shape_spec)? ~ ">"
}

shape_spec = {
    "[" ~ (dimension ~ ("," ~ dimension)*)? ~ "]"
}

dimension = { number | "_" }  // _ for unknown dimension

vector_type = { "Vector" }

record_type = {
    "Record" ~ "<" ~ "{" ~ record_type_field ~ ("," ~ record_type_field)* ~ "}" ~ ">"
}

record_type_field = {
    ("mut")? ~ identifier ~ ":" ~ type_annotation
}

function_type = {
    "Function" ~ "<" ~ "[" ~ (type_annotation ~ ("," ~ type_annotation)*)? ~ "]" ~ "," ~ type_annotation ~ ">"
}

any_type = { "Any" }

// Modified declarations with optional types
typed_let_statement = {
    "let" ~ identifier ~ (":" ~ type_annotation)? ~ "=" ~ expr
}

typed_mut_statement = {
    "mut" ~ identifier ~ (":" ~ type_annotation)? ~ "=" ~ expr
}

// Modified lambda with typed parameters and return type
typed_lambda_params = {
    typed_param
  | ("(" ~ (typed_param ~ ("," ~ typed_param)*)? ~ ")")
}

typed_param = {
    identifier ~ (":" ~ type_annotation)?
}

typed_lambda = {
    typed_lambda_params ~ (":" ~ type_annotation)? ~ "=>" ~ lambda_body
}
```

### Phase 2: Type Representation

Extend `Value` enum or create separate `Type` enum:

```rust
// New file: crates/achronyme-types/src/type_annotation.rs

#[derive(Debug, Clone, PartialEq)]
pub enum TypeAnnotation {
    Number,
    Boolean,
    String,
    Complex,
    Tensor {
        element_type: Box<TypeAnnotation>,
        shape: Option<Vec<Option<usize>>>,  // None = unknown rank, Some([None, None]) = known rank
    },
    Vector,
    Record(HashMap<String, (bool, TypeAnnotation)>),  // (is_mutable, type)
    Function {
        params: Vec<TypeAnnotation>,
        return_type: Box<TypeAnnotation>,
    },
    Any,
}

impl TypeAnnotation {
    /// Check if a value matches this type annotation
    pub fn matches(&self, value: &Value) -> bool {
        // Implementation
    }

    /// Check if two types are compatible for assignment
    pub fn is_assignable_from(&self, other: &TypeAnnotation) -> bool {
        // Implementation
    }
}
```

### Phase 3: Type Checker

```rust
// New file: crates/achronyme-eval/src/type_checker.rs

pub struct TypeChecker {
    // Symbol table for type information
    type_env: HashMap<String, TypeAnnotation>,
}

impl TypeChecker {
    pub fn check_expr(&mut self, expr: &Expr) -> Result<TypeAnnotation, TypeError> {
        // Type check an expression and return its type
    }

    pub fn check_function_call(
        &mut self,
        func_type: &TypeAnnotation,
        args: &[Expr],
    ) -> Result<TypeAnnotation, TypeError> {
        // Verify argument types match parameter types
    }

    pub fn check_assignment(
        &mut self,
        target: &Expr,
        value: &Expr,
    ) -> Result<(), TypeError> {
        // Verify value type matches target type
    }
}
```

### Phase 4: Integration with Evaluator

```rust
// Modified evaluator to use type information

impl Evaluator {
    pub fn eval_with_type_check(&mut self, expr: &Expr) -> Result<Value, String> {
        // 1. Type check the expression (if annotations present)
        if self.strict_mode {
            let type_checker = TypeChecker::new();
            type_checker.check_expr(expr)?;
        }

        // 2. Evaluate normally
        self.eval(expr)
    }
}
```

## Backward Compatibility

All existing code continues to work without modification:

```javascript
// Existing dynamic code (no changes needed)
let add = (a, b) => a + b
let data = [1, 2, 3]
let point = {x: 10, y: 20}

// New typed code
let addTyped = (a: Number, b: Number): Number => a + b

// Can call dynamic from typed (runtime check)
let result = addTyped(add(1, 2), 3)  // OK if add returns Number

// Can call typed from dynamic (always OK)
let result2 = add(addTyped(1, 2), 3)  // OK
```

## Error Messages

### Type Mismatch

```
Error: Type mismatch in function call
  --> example.soc:5:10
   |
5  | add("hello", 42)
   |     ^^^^^^^ Expected Number, got String
   |
Note: Function 'add' expects (Number, Number) -> Number
```

### Shape Mismatch

```
Error: Tensor shape mismatch
  --> example.soc:8:15
   |
8  | matmul(A, C)
   |           ^ Expected Tensor<Number, [3, _]>, got Tensor<Number, [2, 2]>
   |
Note: Matrix multiplication requires compatible inner dimensions
      Left matrix:  [2, 3]
      Right matrix: [2, 2]  <- dimension mismatch
```

### Return Type Mismatch

```
Error: Return type mismatch
  --> example.soc:12:5
   |
10 | let abs = (x: Number): Number => if (x < 0) {
11 |     -x
12 | } else {
13 |     "positive"
   |     ^^^^^^^^^^ Expected Number, got String
   |
Note: Function declared to return Number
```

## Future Extensions

### 1. Type Aliases

```javascript
type Point2D = Record<{x: Number, y: Number}>
type Matrix = Tensor<Number, [_, _]>
type Transform = Function<[Point2D], Point2D>
```

### 2. Generic Types

```javascript
// Generic function
let identity = <T>(x: T): T => x

// Generic array operations
let map = <A, B>(f: Function<[A], B>, arr: Tensor<A>): Tensor<B> => ...
```

### 3. Union Types

```javascript
type NumericValue = Number | Complex
type Result = Record<{success: Boolean, value: Number}> | Record<{error: String}>
```

### 4. Type Constraints

```javascript
// Constrain to numeric types only
let sum = <T: Number | Complex>(arr: Tensor<T>): T => reduce((a, b) => a + b, 0, arr)
```

## Benefits

### For Scientific Computing

1. **Shape Safety**: Catch dimension mismatches at compile-time
2. **Numeric Precision**: Ensure real vs. complex consistency
3. **API Documentation**: Types document expected inputs/outputs
4. **Tooling**: Enable autocomplete, inline docs, refactoring
5. **Performance**: Potential for optimization with known types

### For Application Development

1. **Type Safety**: Catch errors before runtime
2. **Refactoring**: Safe renames and restructuring
3. **Team Collaboration**: Self-documenting interfaces
4. **IDE Support**: Better autocomplete and error highlighting
5. **Gradual Adoption**: Add types incrementally

## Summary

This proposal adds **optional, gradual typing** to Achronyme:

✅ **Backward Compatible**: All existing code works unchanged
✅ **Opt-in**: Add types where they provide value
✅ **Comprehensive**: Covers all value types including tensor shapes
✅ **Scientific Focus**: Designed for mathematical computing use cases
✅ **Clear Errors**: Helpful type error messages
✅ **Future-Proof**: Foundation for generics and advanced features

The type system strikes a balance between:
- **Flexibility**: Dynamic typing for rapid prototyping
- **Safety**: Static checking for production code
- **Expressiveness**: Rich types for scientific computing

This makes Achronyme suitable for both:
- Quick exploratory data analysis (dynamic)
- Production scientific applications (typed)
