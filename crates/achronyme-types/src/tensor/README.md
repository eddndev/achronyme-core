# Tensor System

**Sistema N-dimensional de tensores para computación numérica eficiente.**

## 🎯 Overview

El sistema de tensores de Achronyme soporta:
- ✅ Tensores N-dimensionales (vectores, matrices, y más)
- ✅ Tipos real (f64) y complejo (Complex)
- ✅ Broadcasting estilo NumPy
- ✅ Álgebra lineal completa
- ✅ Operaciones vectoriales optimizadas

## 📁 Arquitectura modular

```
tensor/
├── mod.rs              # Re-exports públicos
├── core.rs             # RealTensor, ComplexTensor structs
├── display.rs          # Pretty-printing
├── conversions.rs      # Real ↔ Complex
├── broadcast.rs        # Broadcasting rules
│
├── arithmetic/         # +, -, *, / element-wise
│   ├── mod.rs
│   ├── real.rs         # RealTensor arithmetic
│   └── complex.rs      # ComplexTensor arithmetic
│
├── matrix_ops/         # Álgebra lineal
│   ├── mod.rs
│   ├── real.rs         # matmul, transpose, det, inv
│   └── complex.rs      # Versiones complejas
│
├── vector_ops/         # Operaciones vectoriales
│   ├── mod.rs
│   ├── real.rs         # dot, cross, norm
│   └── complex.rs      # Versiones complejas
│
└── constructors/       # Builders
    ├── mod.rs
    ├── real.rs         # zeros, ones, eye, linspace
    └── complex.rs      # Versiones complejas
```

## 🔢 Core Types

### RealTensor

```rust
pub struct RealTensor {
    data: Vec<f64>,      // Almacenamiento contiguo
    shape: Vec<usize>,   // Dimensiones [dim0, dim1, ...]
}
```

**Orden de almacenamiento**: Row-major (estilo C/NumPy)

### ComplexTensor

```rust
pub struct ComplexTensor {
    data: Vec<Complex>,
    shape: Vec<usize>,
}
```

## 🏗️ Construcción

### Métodos de construcción

```rust
// Vector (rank 1)
let vec = RealTensor::vector(vec![1.0, 2.0, 3.0])?;
// shape: [3]

// Matriz (rank 2)
let matrix = RealTensor::matrix(2, 3, vec![
    1.0, 2.0, 3.0,
    4.0, 5.0, 6.0
])?;
// shape: [2, 3]

// Tensor general (rank N)
let tensor = RealTensor::new(
    vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0],
    vec![2, 2, 2]  // Tensor 2×2×2
)?;
```

### Constructores especiales

```rust
// Matriz de zeros
let zeros = RealTensor::zeros(&[3, 3])?;
// [[0, 0, 0],
//  [0, 0, 0],
//  [0, 0, 0]]

// Matriz de ones
let ones = RealTensor::ones(&[2, 2])?;
// [[1, 1],
//  [1, 1]]

// Matriz identidad
let identity = RealTensor::eye(3)?;
// [[1, 0, 0],
//  [0, 1, 0],
//  [0, 0, 1]]

// Vector linspace
let linspace = RealTensor::linspace(0.0, 10.0, 5)?;
// [0.0, 2.5, 5.0, 7.5, 10.0]
```

## 🧮 Operaciones aritméticas

### Element-wise operations

```rust
let a = RealTensor::matrix(2, 2, vec![1.0, 2.0, 3.0, 4.0])?;
let b = RealTensor::matrix(2, 2, vec![5.0, 6.0, 7.0, 8.0])?;

// Suma
let sum = a.add(&b)?;
// [[6, 8], [10, 12]]

// Resta
let diff = a.sub(&b)?;
// [[-4, -4], [-4, -4]]

// Multiplicación element-wise (Hadamard)
let prod = a.mul(&b)?;
// [[5, 12], [21, 32]]

// División element-wise
let quot = a.div(&b)?;
// [[0.2, 0.33...], [0.42..., 0.5]]
```

### Operaciones con escalares

```rust
let matrix = RealTensor::matrix(2, 2, vec![1.0, 2.0, 3.0, 4.0])?;

// Suma escalar
let result = matrix.add_scalar(10.0);
// [[11, 12], [13, 14]]

// Multiplicación escalar
let result = matrix.mul_scalar(2.0);
// [[2, 4], [6, 8]]

// División escalar
let result = matrix.div_scalar(2.0)?;
// [[0.5, 1.0], [1.5, 2.0]]
```

## 📐 Broadcasting

Implementa reglas de NumPy para operaciones automáticas:

### Reglas

1. **Alinear shapes por la derecha**
2. **Dimensiones de tamaño 1 se expanden**
3. **Dimensiones faltantes se tratan como 1**
4. **Shapes incompatibles → error**

### Ejemplos

```rust
// Scalar broadcasting
let vec = RealTensor::vector(vec![1.0, 2.0, 3.0])?;
let result = vec.add_scalar(10.0);
// [11, 12, 13]

// Vector + Matrix
// shape: [3] + [2, 3] → error (incompatible)
// shape: [1, 3] + [2, 3] → [2, 3] (OK, expande primera dim)

let row = RealTensor::matrix(1, 3, vec![1.0, 2.0, 3.0])?;
let matrix = RealTensor::matrix(2, 3, vec![
    10.0, 20.0, 30.0,
    40.0, 50.0, 60.0
])?;
let result = matrix.add(&row)?;
// [[11, 22, 33],
//  [41, 52, 63]]
```

### Verificación de compatibilidad

```rust
fn are_broadcastable(shape1: &[usize], shape2: &[usize]) -> bool {
    let max_len = shape1.len().max(shape2.len());

    for i in 0..max_len {
        let dim1 = shape1.get(shape1.len().saturating_sub(max_len - i)).unwrap_or(&1);
        let dim2 = shape2.get(shape2.len().saturating_sub(max_len - i)).unwrap_or(&1);

        if dim1 != dim2 && *dim1 != 1 && *dim2 != 1 {
            return false;
        }
    }
    true
}
```

## 🔲 Operaciones matriciales

### Multiplicación matricial

```rust
// (m × n) · (n × p) → (m × p)
let a = RealTensor::matrix(2, 3, vec![
    1.0, 2.0, 3.0,
    4.0, 5.0, 6.0
])?;

let b = RealTensor::matrix(3, 2, vec![
    7.0, 8.0,
    9.0, 10.0,
    11.0, 12.0
])?;

let c = a.matmul(&b)?;
// shape: [2, 2]
// [[58, 64], [139, 154]]
```

**Algoritmo**: Naive O(n³) - suficiente para matrices pequeñas/medianas.

### Transposición

```rust
let matrix = RealTensor::matrix(2, 3, vec![
    1.0, 2.0, 3.0,
    4.0, 5.0, 6.0
])?;

let transposed = matrix.transpose();
// shape: [3, 2]
// [[1, 4], [2, 5], [3, 6]]
```

### Determinante

```rust
let matrix = RealTensor::matrix(3, 3, vec![
    1.0, 2.0, 3.0,
    0.0, 1.0, 4.0,
    5.0, 6.0, 0.0
])?;

let det = matrix.determinant()?;
// = 1 * (1*0 - 4*6) - 2 * (0*0 - 4*5) + 3 * (0*6 - 1*5)
// = -24 + 40 - 15 = 1
```

**Algoritmo**: Expansión de Laplace recursiva.

### Inversa

```rust
let matrix = RealTensor::matrix(2, 2, vec![
    4.0, 7.0,
    2.0, 6.0
])?;

let inverse = matrix.inverse()?;
// [[0.6, -0.7], [-0.2, 0.4]]

// Verificación: A * A⁻¹ = I
let identity = matrix.matmul(&inverse)?;
// [[1, 0], [0, 1]] (con errores de redondeo)
```

**Algoritmo**: Gauss-Jordan elimination.

## 🎯 Operaciones vectoriales

### Producto punto (dot product)

```rust
let a = RealTensor::vector(vec![1.0, 2.0, 3.0])?;
let b = RealTensor::vector(vec![4.0, 5.0, 6.0])?;

let dot = a.dot(&b)?;
// = 1*4 + 2*5 + 3*6 = 32
```

### Producto cruz (cross product)

```rust
let a = RealTensor::vector(vec![1.0, 0.0, 0.0])?;
let b = RealTensor::vector(vec![0.0, 1.0, 0.0])?;

let cross = a.cross(&b)?;
// = [0, 0, 1] (perpendicular a ambos)
```

**Limitación**: Solo para vectores 3D.

### Norma

```rust
let vec = RealTensor::vector(vec![3.0, 4.0])?;

let norm = vec.norm();
// = √(3² + 4²) = 5.0
```

**Fórmula**: L2 norm (Euclidean)

## 🔄 Conversiones

### Real ↔ Complex

```rust
// Real → Complex
let real_tensor = RealTensor::vector(vec![1.0, 2.0, 3.0])?;
let complex_tensor = real_tensor.to_complex();
// [1+0i, 2+0i, 3+0i]

// Complex → Real (solo si partes imaginarias son ~0)
let complex_tensor = ComplexTensor::vector(vec![
    Complex::new(1.0, 0.0),
    Complex::new(2.0, 0.0),
])?;
let real_tensor = complex_tensor.to_real()?;
// [1.0, 2.0]
```

## 🖨️ Display / Pretty-printing

```rust
let matrix = RealTensor::matrix(3, 3, vec![
    1.0, 2.0, 3.0,
    4.0, 5.0, 6.0,
    7.0, 8.0, 9.0
])?;

println!("{}", matrix);
```

**Output**:
```
[[1, 2, 3],
 [4, 5, 6],
 [7, 8, 9]]
```

Automático para tensores rank ≤ 2. Para rank > 2, muestra formato compacto.

## 🧪 Testing

### Test de construcción

```rust
#[test]
fn test_matrix_construction() {
    let matrix = RealTensor::matrix(2, 3, vec![
        1.0, 2.0, 3.0,
        4.0, 5.0, 6.0
    ]).unwrap();

    assert_eq!(matrix.shape(), &[2, 3]);
    assert_eq!(matrix.data().len(), 6);
}
```

### Test de operaciones

```rust
#[test]
fn test_matrix_multiplication() {
    let a = RealTensor::matrix(2, 2, vec![1.0, 2.0, 3.0, 4.0]).unwrap();
    let b = RealTensor::matrix(2, 2, vec![5.0, 6.0, 7.0, 8.0]).unwrap();

    let c = a.matmul(&b).unwrap();

    // Verificar resultado
    assert_eq!(c.shape(), &[2, 2]);
    assert!((c.data()[0] - 19.0).abs() < 1e-10);  // 1*5 + 2*7
    assert!((c.data()[1] - 22.0).abs() < 1e-10);  // 1*6 + 2*8
}
```

## 🚀 Optimizaciones futuras

### Paralelización
- [ ] Usar `rayon` para operaciones element-wise
- [ ] Multiplicación matricial paralela (Strassen)

### SIMD
- [ ] Usar instrucciones vectoriales (AVX2/AVX512)
- [ ] Alineamiento de memoria

### Algoritmos avanzados
- [ ] QR decomposition
- [ ] SVD (Singular Value Decomposition)
- [ ] Eigenvalues/Eigenvectors
- [ ] LU decomposition

### Sparse tensors
- [ ] Representación CSR/CSC para matrices dispersas
- [ ] Operaciones optimizadas para sparse

## 📚 Referencias

- [NumPy Broadcasting](https://numpy.org/doc/stable/user/basics.broadcasting.html)
- [BLAS](http://www.netlib.org/blas/) - Referencia para operaciones matriciales
- [ndarray crate](https://docs.rs/ndarray/) - Inspiración para API

## 🎯 Notas de implementación

### Indexing

**Row-major order** (estilo C):
```
Matrix 2×3:
┌─────────┐
│ 0  1  2 │
│ 3  4  5 │
└─────────┘

data = [0, 1, 2, 3, 4, 5]

Index [i, j] → i * cols + j
```

**Column-major order** (estilo Fortran):
```
data = [0, 3, 1, 4, 2, 5]

Index [i, j] → j * rows + i
```

Achronyme usa **row-major** para compatibilidad con NumPy/Rust ecosistema.

### Error handling

Todas las operaciones retornan `Result<T, String>`:
- `Ok(tensor)` - Operación exitosa
- `Err(msg)` - Error descriptivo

**Errores comunes**:
- Shape mismatch en operaciones binarias
- División por cero
- Matriz singular (no invertible)
- Dimensiones incorrectas para operación específica
