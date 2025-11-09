# Tensor Architecture

## Overview

Achronyme now includes a powerful **unified tensor system** that provides efficient N-dimensional arrays for both real and complex numbers. This architecture replaces the previous fragmented approach (separate `Vector`, `Matrix`, `ComplexVector`) with a single, generic `Tensor<T>` structure.

## Core Structure

### Generic Tensor

```rust
pub struct Tensor<T> {
    pub data: Vec<T>,           // Flat storage in row-major order
    pub shape: Vec<usize>,      // Dimensions [d0, d1, d2, ...]
    pub strides: Vec<usize>,    // Strides for efficient indexing
}
```

### Type Aliases

```rust
pub type RealTensor = Tensor<f64>;
pub type ComplexTensor = Tensor<Complex>;
```

## Value Types

The `Value` enum now includes two new variants:

```rust
pub enum Value {
    Number(f64),
    Boolean(bool),
    Complex(Complex),
    Vector(Vec<Value>),           // Generic - for strings, records, heterogeneous data
    Tensor(RealTensor),           // Optimized N-dimensional array of real numbers
    ComplexTensor(ComplexTensor), // Optimized N-dimensional array of complex numbers
    Matrix(Matrix),               // Legacy - to be phased out
    Function(Function),
    String(String),
    Record(HashMap<String, Value>),
    Edge { ... },
}
```

## Tensor Ranks

Tensors are classified by their **rank** (number of dimensions):

| Rank | Type      | Shape Example | Description |
|------|-----------|---------------|-------------|
| 0    | Scalar    | `[]`          | Single value |
| 1    | Vector    | `[5]`         | 1D array of 5 elements |
| 2    | Matrix    | `[3, 4]`      | 3×4 matrix |
| 3+   | Tensor    | `[2, 3, 4]`   | Higher-order tensors |

## Creating Tensors

### RealTensor Constructors

```rust
// Scalar (rank 0)
let scalar = RealTensor::scalar(5.0);

// Vector (rank 1)
let vec = RealTensor::vector(vec![1.0, 2.0, 3.0]);

// Matrix (rank 2)
let mat = RealTensor::matrix(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0])?;

// Higher-order tensor
let tensor = RealTensor::new(data, vec![2, 3, 4])?;

// Utility constructors
let zeros = RealTensor::zeros(vec![3, 3]);
let ones = RealTensor::ones(vec![2, 4]);
let filled = RealTensor::filled(vec![5, 5], 7.0);
let identity = RealTensor::eye(4);
```

### ComplexTensor Constructors

```rust
// Complex scalar
let c_scalar = ComplexTensor::scalar(Complex::new(1.0, 2.0));

// Complex vector
let c_vec = ComplexTensor::vector(vec![
    Complex::new(1.0, 0.0),
    Complex::new(0.0, 1.0),
]);

// Complex matrix
let c_zeros = ComplexTensor::zeros(vec![3, 3]);
let c_ones = ComplexTensor::ones(vec![2, 2]);
let c_eye = ComplexTensor::eye(3);
```

## Indexing and Access

```rust
// Create a 2x3 matrix
let mat = RealTensor::matrix(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0])?;

// Get element at [row, col]
let value = mat.get(&[0, 1])?;  // 2.0

// Set element
let mut mat = mat.clone();
mat.set(&[1, 2], 10.0)?;
```

## Reshaping

```rust
// Create a vector
let vec = RealTensor::vector(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);

// Reshape to 2x3 matrix (preserves data, changes view)
let mat = vec.reshape(vec![2, 3])?;

// Reshape to 3D tensor
let tensor = vec.reshape(vec![2, 1, 3])?;
```

## Properties

```rust
let tensor = RealTensor::new(data, vec![2, 3, 4])?;

tensor.rank()        // 3 (3D tensor)
tensor.size()        // 24 (total elements)
tensor.shape()       // &[2, 3, 4]
tensor.strides()     // &[12, 4, 1]
tensor.is_scalar()   // false
tensor.is_vector()   // false
tensor.is_matrix()   // false
```

## Memory Layout

All tensors use **row-major** (C-style) memory layout:

```rust
// For a 2x3 matrix:
// [[a, b, c],
//  [d, e, f]]
//
// Stored in memory as: [a, b, c, d, e, f]
//
// Strides: [3, 1]
// - Moving one row = skip 3 elements
// - Moving one column = skip 1 element
```

## Conversions

### From generic Vector to Tensor

```rust
// Vec<Value> with all numbers → RealTensor
let generic = vec![Value::Number(1.0), Value::Number(2.0), Value::Number(3.0)];
let tensor = Value::to_real_tensor(&generic)?;

// Vec<Value> with complex → ComplexTensor
let generic = vec![
    Value::Complex(Complex::new(1.0, 2.0)),
    Value::Number(3.0),  // Auto-promoted to complex
];
let c_tensor = Value::to_complex_tensor(&generic)?;
```

### From Tensor to generic Vector

```rust
// RealTensor (rank 1) → Vec<Value>
let tensor = RealTensor::vector(vec![1.0, 2.0, 3.0]);
let value = Value::from_real_tensor(tensor);
// Result: Value::Vector([Number(1.0), Number(2.0), Number(3.0)])

// RealTensor (rank 2+) → Value::Tensor
let matrix = RealTensor::matrix(2, 2, vec![1.0, 2.0, 3.0, 4.0])?;
let value = Value::from_real_tensor(matrix);
// Result: Value::Tensor(...)
```

## Benefits of Tensor Architecture

### 1. **Unified Implementation**
- Operations like `+`, `*`, `sin()` written once for all ranks
- No separate logic for Vector vs Matrix vs higher dimensions

### 2. **Memory Efficiency**
- Flat `Vec<T>` storage is cache-friendly
- Contiguous memory improves performance
- Minimal overhead (just shape + strides metadata)

### 3. **Type Safety**
- `RealTensor` vs `ComplexTensor` prevents mixing accidentally
- Rust's type system catches errors at compile time

### 4. **Complex Number Support**
- **NEW**: Full support for complex tensors of any rank
- Essential for FFT, signal processing, quantum computing

### 5. **Future-Ready**
- Easy to add GPU support (flat buffer maps to GPU memory)
- Broadcasting implementation benefits all tensor operations
- Slicing/views can be added without changing data structure

## Coexistence with Legacy Types

Currently, both old and new types coexist:

| Legacy Type | New Type | Status |
|-------------|----------|--------|
| `Vector` (struct) | `RealTensor` (rank 1) | Maintained for compatibility |
| `ComplexVector` | `ComplexTensor` (rank 1) | Maintained for compatibility |
| `Matrix` | `RealTensor` (rank 2) | To be phased out |
| `Value::Vector` | `Value::Tensor` | Both exist - Vector for heterogeneous, Tensor for numeric |

**Migration Strategy**:
- New code should use `Tensor`
- Existing code continues to work
- Gradual migration of operations to use `Tensor` internally

## Examples

### Scalar Operations
```rust
let a = RealTensor::scalar(5.0);
let b = RealTensor::scalar(3.0);
// Operations TBD in next phase
```

### Vector Operations
```rust
let v1 = RealTensor::vector(vec![1.0, 2.0, 3.0]);
let v2 = RealTensor::vector(vec![4.0, 5.0, 6.0]);
// Addition, multiplication TBD in next phase
```

### Matrix Operations
```rust
let m = RealTensor::matrix(3, 3, vec![
    1.0, 2.0, 3.0,
    4.0, 5.0, 6.0,
    7.0, 8.0, 9.0,
])?;

// Access element
let elem = m.get(&[1, 2])?;  // 6.0

// Reshape to vector
let flat = m.reshape(vec![9])?;
```

### Complex Tensors
```rust
// Complex FFT output (rank 1)
let fft_result = ComplexTensor::vector(vec![
    Complex::new(1.0, 0.0),
    Complex::new(0.5, 0.866),
    Complex::new(-0.5, 0.866),
]);

// Complex matrix
let hermitian = ComplexTensor::zeros(vec![4, 4]);
```

## Next Steps

The tensor architecture is now in place. Future work includes:

1. **Broadcasting** - NumPy-style automatic dimension matching
2. **Arithmetic Operations** - Implement `+`, `-`, `*`, `/` for tensors
3. **Indexing/Slicing** - Syntax like `tensor[0, :, 2]`
4. **Pipe Operator** - Ergonomic chaining like `data |> map |> filter`
5. **Operation Migration** - Move existing operations to use Tensor internally

## Testing

All tensor functionality includes comprehensive tests:

```bash
cargo test -p achronyme-types tensor::
```

9 tests covering:
- Tensor creation
- Scalars, vectors, matrices
- Get/set operations
- Reshaping
- Zeros/ones/eye constructors
- Complex tensors
