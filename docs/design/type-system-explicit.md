# Explicit Type System Design - FINAL SPECIFICATION

## Overview

This document specifies the **optional gradual type system** for Achronyme that:
- Maintains 100% backward compatibility with existing dynamic typing
- Allows explicit type annotations for function parameters, variables, and return types
- Provides compile-time (parse-time) type checking when annotations are present
- Enables better IDE support, documentation, and early error catching
- Focuses on scientific computing use cases with tensor shapes and numeric precision
- **Supports Union Types as a core feature** (not a future extension)
- **Uses structural typing for Records** (duck typing)
- **Allows partial type annotations** (gradual typing)

## Philosophy

**"Simplicity by default, specificity when needed"**

- **Dynamic by default**: Existing code continues to work without modification
- **Gradual typing**: Add types incrementally - mix typed and untyped parameters
- **Progressive enhancement**: Go from fully dynamic → partially typed → fully typed
- **Type inference**: Infer types where obvious, require annotations where ambiguous
- **Structural subtyping**: Records match by structure, not by name (duck typing)
- **Union types**: Core feature for expressing "one of these types"
- **Runtime safety**: Type errors fail fast with clear, helpful messages

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
// Type-safe, errors caught at parse time
let add = (a: Number, b: Number): Number => a + b

add(5, 10)        // 15 (OK)
add("hello", 42)  // Parse-time error: Expected Number, got String
add([1, 2], [3])  // Parse-time error: Expected Number, got Tensor
```

### With Gradual Typing (Mix Both)

```javascript
// Mix typed and untyped parameters
let flexible = (x: Number, y) => x + y

flexible(5, 10)      // OK: x type-checked, y dynamic
flexible(5, "text")  // OK at parse-time, runtime error if incompatible
flexible("x", 10)    // Parse-time error: x must be Number
```

## Type System Hierarchy

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
├── {field: Type}   // Record (structural, no wrapper)
├── Function<[A, B], R>  // Function type
├── Union (A | B)   // Union types (CORE FEATURE)
├── null            // Null type (for optional values)
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
    if(n <= 1) { 1 } else { n * rec(n - 1) }
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

### 7. Record Types (Structural Typing - NO `Record<...>` wrapper)

**IMPORTANT DESIGN DECISION**: Records use **direct structural syntax** `{field: Type}`, not `Record<{field: Type}>`.

#### Basic Record Types

```javascript
// Simple record type - NO Record<...> wrapper
let point: {x: Number, y: Number} = {x: 10, y: 20}

// Nested records
let person: {
    name: String,
    age: Number,
    address: {
        city: String,
        country: String
    }
} = {
    name: "Alice",
    age: 30,
    address: {
        city: "Madrid",
        country: "Spain"
    }
}
```

#### Record with Mutable Fields

```javascript
// Mutable field in type annotation
let counter: {mut value: Number} = {
    mut value: 0
}

// Function that requires mutable field
let increment = (c: {mut value: Number}): Number => {
    c.value = c.value + 1
    c.value
}

increment(counter)           // OK: field is mutable
increment({value: 10})       // Error: 'value' must be mutable
```

#### Structural Subtyping (Duck Typing)

**KEY FEATURE**: Records are structurally typed - a record matches if it has **at least** the required fields.

```javascript
// Function requires only 'name' field
let greet = (person: {name: String}): String => "Hello " + person.name

// All of these work (structural subtyping):
greet({name: "Alice"})                           // OK: exact match
greet({name: "Bob", age: 30})                    // OK: has extra 'age' field
greet({name: "Charlie", age: 30, city: "NY"})   // OK: has multiple extra fields

// This fails:
greet({age: 30})                                 // Error: missing 'name' field
```

#### Empty Record Type (Accepts Any Record)

```javascript
// Empty record type: {} accepts any record
let keys = (obj: {}): Vector => getKeys(obj)

keys({name: "Alice"})           // OK
keys({x: 1, y: 2, z: 3})       // OK
keys({})                        // OK: empty record
```

#### Access to Non-Typed Fields (Permissive with Warning)

**DESIGN DECISION**: Accessing fields not in the type annotation is **permitted but warns** and returns `Any`.

```javascript
let greet = (person: {name: String}) => {
    let base = "Hello " + person.name           // OK: field is typed

    // Warning: 'age' not in type {name: String}, accessing as Any
    if (person.age != null) {
        base + " (age " + str(person.age) + ")"
    } else {
        base
    }
}
```

**Rationale**: Allows exploratory/dynamic code while maintaining type safety for declared fields.

#### Record with Methods

```javascript
let Point: {
    x: Number,
    y: Number,
    distance: Function<[], Number>
} = {
    x: 10,
    y: 20,
    distance: () => sqrt(self.x^2 + self.y^2)
}
```

### 8. Union Types (CORE FEATURE)

**CRITICAL DESIGN DECISION**: Union types are **not** a future extension - they are a **core MVP feature**.

#### Basic Union Types

```javascript
// Variable can be Number OR Complex
let x: Number | Complex = 3.14
x = 2 + 3i  // OK: Complex is allowed

// Function accepting multiple numeric types
let abs = (x: Number | Complex): Number => {
    if (type(x) == "Number") {
        if (x < 0) { -x } else { x }
    } else {
        magnitude(x)
    }
}

abs(5)       // OK: Number
abs(-3.14)   // OK: Number
abs(3 + 4i)  // OK: Complex
abs("text")  // Error: Expected Number | Complex, got String
```

#### Union for Optional Values (Result Pattern)

```javascript
// Function that may fail
let invert = (m: Tensor): Tensor | String => {
    if (det(m) == 0) {
        "Matrix is singular"  // Return error message
    } else {
        inv(m)                // Return result
    }
}

let result = invert([[1, 2], [2, 4]])
if (type(result) == "String") {
    print("Error: " + result)
} else {
    print("Inverted: " + str(result))
}
```

#### Union with Null (Optional Values)

**NEW TYPE**: `null` type represents absence of value.

```javascript
// Optional value (may be null)
let find = (arr: Vector, pred: Function): Any | null => {
    for (i in range(len(arr))) {
        if (pred(arr[i])) {
            return arr[i]
        }
    }
    return null
}

let result = find([1, 2, 3], x => x > 5)
if (result != null) {
    print("Found: " + str(result))
} else {
    print("Not found")
}
```

#### Union in Record Fields

```javascript
// Record with union-typed field
let response: {
    status: Number,
    data: Tensor | String | null
} = {
    status: 200,
    data: [[1, 2], [3, 4]]
}

// Later...
response.data = "Error occurred"  // OK: String is in union
response.data = null              // OK: null is in union
response.data = true              // Error: Boolean not in union
```

#### Multiple Types in Union

```javascript
// Parse function returns different types
let parse = (input: String): Number | Boolean | String | null => {
    if (input == "true") { true }
    else if (input == "false") { false }
    else if (isNumeric(input)) { toNumber(input) }
    else if (input == "") { null }
    else { input }
}
```

#### Type Narrowing with Type Guards

```javascript
let process = (x: Number | String): String => {
    // Type guard with type() function
    if (type(x) == "Number") {
        // Here x is narrowed to Number
        return str(x * 2)
    } else {
        // Here x is narrowed to String
        return x + x
    }
}
```

### 9. Function Types

```javascript
// Function type annotation
let apply: Function<[Number], Number> = x => x^2

// Higher-order function
let twice: Function<[Function<[Number], Number>, Number], Number> =
    (f, x) => f(f(x))

// Simplified notation (future)
let twice = (f: Number => Number, x: Number): Number => f(f(x))
```

### 10. Any Type (Opt-out)

```javascript
// Explicitly dynamic
let dynamic: Any = 42
dynamic = "hello"  // OK
dynamic = [1, 2, 3]  // OK

// Function accepting any type
let describe = (value: Any): String => str(value)
```

### 11. Null Type

```javascript
// Null literal
let nothing: null = null

// Optional pattern (T | null)
let maybeNumber: Number | null = null
maybeNumber = 42  // OK

// Null check
if (maybeNumber != null) {
    print(maybeNumber + 10)
}
```

## Gradual Typing (Partial Type Annotations)

**CORE PHILOSOPHY**: You can mix typed and untyped parameters in the same function.

### Partially Typed Parameters

```javascript
// Only 'a' is type-checked, 'b' is dynamic
let add = (a: Number, b) => a + b

add(5, 10)       // OK: a=Number (checked), b=Any (dynamic)
add(5, "text")   // OK at parse-time, may fail at runtime
add("x", 10)     // Parse-time error: 'a' must be Number

// Mix typed and untyped for flexibility
let compute = (x: Number, y: Number, z) => x + y + z

compute(1, 2, 3)       // OK: z dynamic
compute(1, 2, "text")  // OK at parse-time, runtime error
compute("a", 2, 3)     // Parse-time error: x must be Number
```

### Return Type Inference

```javascript
// Fully typed params → return type inferred
let multiply = (a: Number, b: Number) => a * b  // Returns Number (inferred)

// Partially typed → return type is Any (conservative)
let mixed = (a: Number, b) => a + b  // Returns Any (b unknown)

// Explicit return type overrides inference
let explicit = (a: Number, b): Number => a + b  // Returns Number (enforced)
```

### Backward Compatibility

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

### 3. Union Type Matching

```javascript
let x: Number | String = 42     // OK: Number in union
x = "hello"                     // OK: String in union
x = true                        // Error: Boolean not in union

// Function with union parameter
let show = (x: Number | String): String => str(x)
show(42)       // OK
show("text")   // OK
show(true)     // Error
```

### 4. Structural Record Matching

```javascript
let greet = (p: {name: String}): String => "Hello " + p.name

greet({name: "Alice"})                    // OK: exact match
greet({name: "Bob", age: 30})            // OK: has extra 'age' (structural subtyping)
greet({age: 30})                          // Error: missing 'name'

// Type compatibility
let person: {name: String, age: Number} = {name: "Alice", age: 30}
let nameOnly: {name: String} = person  // OK: structural subtyping (wider type)
```

### 5. Tensor Shape Checking

```javascript
// Shape-aware operations
let matmul = (a: Tensor<Number, [_, K]>, b: Tensor<Number, [K, _]]): Tensor<Number, [_, _]> =>
    dot(a, b)

let A: Tensor<Number, [2, 3]> = [[1, 2, 3], [4, 5, 6]]
let B: Tensor<Number, [3, 2]> = [[1, 2], [3, 4], [5, 6]]
let C: Tensor<Number, [2, 2]> = [[1, 2], [3, 4]]

matmul(A, B)  // OK: [2,3] × [3,2] = [2,2]
matmul(A, C)  // Error: Incompatible shapes [2,3] × [2,2]
```

### 6. Return Type Checking

```javascript
let square = (x: Number): Number => x^2        // OK
let bad = (x: Number): Number => "not a number"  // Error: Expected Number, got String

// Early return type checking
let sign = (x: Number): Number => {
    if (x < 0) {
        return -1  // OK: returns Number
    } else if (x > 0) {
        return 1   // OK: returns Number
    } else {
        return 0   // OK: returns Number
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
let matmul = (a: Tensor<Number, [M, K]>, b: Tensor<Number, [K, N]]): Tensor<Number, [M, N]> =>
    // Implementation with shape guarantees
    ...

// Eigenvalues (matrix → vector)
let eigenvalues = (m: Tensor<Number, [N, N]>): Tensor<Complex, [N]> =>
    // Square matrix required by type, returns complex eigenvalues
    ...
```

### Signal Processing

```javascript
// FFT with length constraint
let fft = (signal: Tensor<Number> | Tensor<Complex>): Tensor<Complex> =>
    // Accepts real or complex signal, always returns complex
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
let mean = (data: Tensor<Number>): Number =>
    sum(data) / len(data)

// Regression with typed result
let linearRegression = (
    x: Tensor<Number, [N]>,
    y: Tensor<Number, [N]>
): {slope: Number, intercept: Number, r2: Number} =>
    // Returns structured result
    ...

// Result pattern with union
let fit = (data: Tensor): {params: Tensor, error: Number} | String => {
    if (len(data) < 2) {
        "Insufficient data"
    } else {
        {params: computeParams(data), error: computeError(data)}
    }
}
```

## Grammar Specification

### Type Annotations (Pest Grammar)

```pest
// ============================================================================
// Type Annotations
// ============================================================================

type_annotation = {
    union_type | simple_type_annotation
}

// Union types (core feature): Number | String | null
union_type = {
    simple_type_annotation ~ ("|" ~ simple_type_annotation)+
}

simple_type_annotation = {
    simple_type
  | tensor_type
  | vector_type
  | record_type
  | function_type
  | any_type
  | null_type
}

simple_type = {
    "Number" | "Boolean" | "String" | "Complex"
}

// Tensor with optional shape: Tensor<Number> or Tensor<Complex, [2,3]>
tensor_type = {
    "Tensor" ~ "<" ~ type_annotation ~ ("," ~ shape_spec)? ~ ">"
}

shape_spec = {
    "[" ~ (dimension ~ ("," ~ dimension)*)? ~ "]"
}

dimension = { number | "_" }  // _ for unknown dimension

vector_type = { "Vector" }

// Record type: direct structural syntax (NO Record<...> wrapper)
// Syntax: {field1: Type1, mut field2: Type2}
record_type = {
    "{" ~ NEWLINE* ~
    (record_type_field ~ (NEWLINE* ~ "," ~ NEWLINE* ~ record_type_field)*)? ~
    NEWLINE* ~ "}"
}

record_type_field = {
    (mut_keyword ~ identifier ~ ":" ~ type_annotation)  // mut value: Number
  | (identifier ~ ":" ~ type_annotation)                // name: String
}

// Function type: Function<[param types], return type>
function_type = {
    "Function" ~ "<" ~
    "[" ~ (type_annotation ~ ("," ~ type_annotation)*)? ~ "]" ~
    "," ~ type_annotation ~
    ">"
}

any_type = { "Any" }
null_type = { "null" }

// ============================================================================
// Modified Statements with Type Annotations
// ============================================================================

// Let statement with optional type annotation
// Syntax: let x: Type = value  OR  let x = value
let_statement = {
    "let" ~ identifier ~ (":" ~ type_annotation)? ~ "=" ~ expr
}

// Mut statement with optional type annotation
// Syntax: mut x: Type = value  OR  mut x = value
mut_statement = {
    "mut" ~ identifier ~ (":" ~ type_annotation)? ~ "=" ~ expr
}

// ============================================================================
// Modified Lambda with Type Annotations (Gradual Typing)
// ============================================================================

// Typed parameter: x (untyped) OR x: Type (typed)
typed_param = {
    identifier ~ (":" ~ type_annotation)?
}

// Lambda parameters with optional types (gradual typing)
typed_lambda_params = {
    typed_param                                          // Single: x or x: Type
  | ("(" ~ (typed_param ~ ("," ~ typed_param)*)? ~ ")") // Multi/no params
}

// Lambda with optional return type annotation
// Syntax: (x: Number, y) => x + y              (partial typing)
// Syntax: (x: Number, y: Number): Number => x + y  (full typing)
lambda = {
    typed_lambda_params ~ (":" ~ type_annotation)? ~ "=>" ~ lambda_body
}
```

## Type Representation (Rust Implementation)

```rust
// File: crates/achronyme-types/src/type_annotation.rs

#[derive(Debug, Clone, PartialEq)]
pub enum TypeAnnotation {
    Number,
    Boolean,
    String,
    Complex,
    Tensor {
        element_type: Box<TypeAnnotation>,
        shape: Option<Vec<Option<usize>>>,  // None = unknown rank, Some([None]) = known rank
    },
    Vector,
    Record {
        fields: HashMap<String, (bool, TypeAnnotation)>,  // (is_mutable, type)
    },
    Function {
        params: Vec<TypeAnnotation>,
        return_type: Box<TypeAnnotation>,
    },
    Union(Vec<TypeAnnotation>),  // CORE FEATURE
    Null,                        // For optional values
    Any,
}

impl TypeAnnotation {
    /// Check if a value matches this type annotation
    pub fn matches(&self, value: &Value) -> bool {
        match (self, value) {
            (TypeAnnotation::Number, Value::Number(_)) => true,
            (TypeAnnotation::Boolean, Value::Boolean(_)) => true,
            (TypeAnnotation::String, Value::String(_)) => true,
            (TypeAnnotation::Complex, Value::Complex(_)) => true,
            (TypeAnnotation::Null, Value::Null) => true,

            // Union: matches if value matches ANY of the types
            (TypeAnnotation::Union(types), val) => {
                types.iter().any(|t| t.matches(val))
            }

            // Record: structural subtyping (must have all required fields)
            (TypeAnnotation::Record(required_fields), Value::Record(actual_fields)) => {
                required_fields.iter().all(|(field_name, (_is_mut, field_type))| {
                    actual_fields.get(field_name)
                        .map(|actual_value| field_type.matches(actual_value))
                        .unwrap_or(false)  // Missing field = no match
                })
            }

            // Tensor: check element types and optionally shape
            (TypeAnnotation::Tensor { element_type, shape }, Value::Tensor(tensor)) => {
                // Check element type for all elements
                let elements_match = tensor.data().iter()
                    .all(|elem| element_type.matches(&Value::Number(*elem)));

                // Check shape if specified
                let shape_match = match shape {
                    None => true,  // No shape constraint
                    Some(expected_shape) => {
                        let actual_shape = tensor.shape();
                        expected_shape.len() == actual_shape.len() &&
                        expected_shape.iter().zip(actual_shape).all(|(exp, act)| {
                            exp.map_or(true, |e| e == *act)  // None (_) matches any
                        })
                    }
                };

                elements_match && shape_match
            }

            // Any: matches everything
            (TypeAnnotation::Any, _) => true,

            _ => false,
        }
    }

    /// Check if this type can be assigned from another type
    pub fn is_assignable_from(&self, other: &TypeAnnotation) -> bool {
        match (self, other) {
            // Same types are assignable
            (a, b) if a == b => true,

            // Any accepts anything
            (TypeAnnotation::Any, _) => true,

            // Anything can be assigned to Any
            (_, TypeAnnotation::Any) => true,

            // Union assignability
            (TypeAnnotation::Union(self_types), TypeAnnotation::Union(other_types)) => {
                // All types in 'other' must be in 'self'
                other_types.iter().all(|ot|
                    self_types.iter().any(|st| st.is_assignable_from(ot))
                )
            }
            (TypeAnnotation::Union(types), other) => {
                // Single type assignable to union if it's in the list
                types.iter().any(|t| t.is_assignable_from(other))
            }
            (self_type, TypeAnnotation::Union(other_types)) => {
                // Union assignable to single type if ALL union members are assignable
                other_types.iter().all(|ot| self_type.is_assignable_from(ot))
            }

            // Record structural subtyping
            (TypeAnnotation::Record(self_fields), TypeAnnotation::Record(other_fields)) => {
                // 'other' must have all fields of 'self' (can have extras)
                self_fields.iter().all(|(field_name, (self_mut, self_type))| {
                    other_fields.get(field_name).map_or(false, |(other_mut, other_type)| {
                        // Mutability must match
                        self_mut == other_mut &&
                        // Type must be assignable
                        self_type.is_assignable_from(other_type)
                    })
                })
            }

            _ => false,
        }
    }
}
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

### Union Type Mismatch

```
Error: Type not in union
  --> example.soc:3:5
   |
3  | x = true
   |     ^^^^ Expected Number | String, got Boolean
   |
Note: Variable 'x' has type Number | String
      Allowed types: Number, String
```

### Record Field Missing

```
Error: Missing required field in record
  --> example.soc:10:8
   |
10 | greet({age: 30})
   |        ^^^^^^^^ Missing field 'name: String'
   |
Note: Function 'greet' expects record with at least:
      {name: String}
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
      Right matrix: [2, 2]  <- dimension mismatch (expected 3)
```

### Return Type Mismatch

```
Error: Return type mismatch
  --> example.soc:12:9
   |
10 | let abs = (x: Number): Number => {
11 |     if (x < 0) { -x }
12 |     else { "positive" }
   |            ^^^^^^^^^^ Expected Number, got String
   |
Note: Function declared to return Number
```

## Benefits

### For Scientific Computing

1. **Shape Safety**: Catch dimension mismatches at compile-time
2. **Numeric Precision**: Ensure real vs. complex consistency
3. **Union Types**: Express "accepts multiple numeric types" naturally
4. **API Documentation**: Types document expected inputs/outputs
5. **Tooling**: Enable autocomplete, inline docs, refactoring
6. **Performance**: Potential for optimization with known types

### For Application Development

1. **Type Safety**: Catch errors before runtime
2. **Gradual Adoption**: Add types incrementally (mix typed/untyped)
3. **Refactoring**: Safe renames and restructuring
4. **Team Collaboration**: Self-documenting interfaces
5. **IDE Support**: Better autocomplete and error highlighting
6. **Flexibility**: Dynamic when prototyping, typed when shipping

## Summary

This specification defines **optional, gradual typing** for Achronyme with:

✅ **Backward Compatible**: All existing code works unchanged
✅ **Gradual**: Mix typed and untyped parameters in same function
✅ **Union Types**: Core feature, not future extension
✅ **Structural Records**: Duck typing with `{field: Type}` syntax (no wrapper)
✅ **Null Safety**: `T | null` pattern for optional values
✅ **Comprehensive**: Covers all value types including tensor shapes
✅ **Scientific Focus**: Designed for mathematical computing use cases
✅ **Clear Errors**: Helpful type error messages
✅ **Future-Proof**: Foundation for generics and advanced features

The type system enables:
- **Quick exploratory analysis** (dynamic, no types)
- **Hybrid development** (types on critical parameters only)
- **Production scientific applications** (fully typed for safety)

**Philosophy**: "Simplicity by default, specificity when needed" - start dynamic, add types progressively as your code matures.
