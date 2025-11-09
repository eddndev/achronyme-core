# Plan de Refactorización: Migración de Tipos Legacy a Tensors

**Fecha**: 2025-01-09
**Estado**: En Progreso (30% completado)
**Objetivo**: Eliminar completamente `Matrix`, `Vector` (RealVector), y `ComplexVector`, usando solo `Tensor` y `ComplexTensor`

---

## ✅ COMPLETADO

### 1. Tensor API Extendido (100%)
**Archivo**: `crates/achronyme-types/src/tensor.rs`

Agregados métodos de compatibilidad con Matrix:
```rust
// Métodos agregados en RealTensor
- rows() -> usize
- cols() -> usize
- is_square() -> bool
- get_matrix(row, col) -> Result<f64>
- set_matrix(row, col, value) -> Result<()>
- row(index) -> Result<Vec<f64>>
- col(index) -> Result<Vec<f64>>
- scale(scalar) -> RealTensor
- hadamard(other) -> Result<RealTensor>
- determinant() -> Result<f64>
- minor(row, col) -> Result<RealTensor>  // privado
```

### 2. Módulo DSP (100%)
**Archivos**:
- ✅ `crates/achronyme-dsp/src/fft.rs`
- ✅ `crates/achronyme-dsp/src/convolution.rs`
- ✅ `crates/achronyme-dsp/src/windows.rs`
- ✅ `crates/achronyme-eval/src/function_modules/dsp.rs`
- ✅ `crates/achronyme-wasm/src/api/dsp.rs`

**Cambios realizados**:
- Todas las funciones DSP usan `&[f64]` y `&[Complex]` (slices)
- Retornan `Vec<f64>` y `Vec<Complex>`
- Eliminada dependencia de `Vector` y `ComplexVector`

### 3. Value.rs - Limpieza Parcial (50%)
**Archivo**: `crates/achronyme-types/src/value.rs`

**Completado**:
- ✅ Eliminado import de `Vector` y `ComplexVector`
- ✅ Eliminadas funciones helper legacy:
  - `to_real_vector()`
  - `to_complex_vector()`
  - `from_real_vector()`
  - `from_complex_vector()`
- ✅ Operador `Add` migrado a usar Tensors

**Pendiente**:
- ❌ Eliminar `Value::Matrix` del enum
- ❌ Eliminar import de `Matrix`

---

## ❌ PENDIENTE

### FASE 1: Preparación (Prioridad: ALTA)

#### 1.1. Completar limpieza de value.rs
**Archivo**: `crates/achronyme-types/src/value.rs`

**Paso 1**: Eliminar la variante Matrix del enum Value
```rust
// ANTES:
pub enum Value {
    Number(f64),
    Boolean(bool),
    Complex(Complex),
    Vector(Vec<Value>),
    Tensor(RealTensor),
    ComplexTensor(ComplexTensor),
    Matrix(Matrix),  // ❌ ELIMINAR ESTA LÍNEA
    Function(Function),
    String(String),
    Record(HashMap<String, Value>),
    Edge { ... },
}

// DESPUÉS:
pub enum Value {
    Number(f64),
    Boolean(bool),
    Complex(Complex),
    Vector(Vec<Value>),  // Solo para datos heterogéneos
    Tensor(RealTensor),  // Para datos numéricos N-dimensionales
    ComplexTensor(ComplexTensor),
    Function(Function),
    String(String),
    Record(HashMap<String, Value>),
    Edge { ... },
}
```

**Paso 2**: Eliminar import de Matrix
```rust
// ANTES:
use crate::complex::Complex;
use crate::matrix::Matrix;  // ❌ ELIMINAR
use crate::tensor::{RealTensor, ComplexTensor};

// DESPUÉS:
use crate::complex::Complex;
use crate::tensor::{RealTensor, ComplexTensor};
```

---

### FASE 2: Migración de achronyme-linalg (Prioridad: ALTA)

**Archivos a modificar**: `crates/achronyme-linalg/src/*.rs`

#### 2.1. Archivo: `crates/achronyme-linalg/src/lib.rs`
**Ubicación**: Buscar `pub use` y `pub mod`

```bash
# Buscar en el archivo
grep -n "Matrix\|Vector" crates/achronyme-linalg/src/lib.rs
```

**Transformación**:
```rust
// ANTES:
use achronyme_types::matrix::Matrix;
pub fn determinant_nd(m: &Matrix) -> Result<f64, String> { ... }

// DESPUÉS:
use achronyme_types::tensor::RealTensor;
pub fn determinant_nd(t: &RealTensor) -> Result<f64, String> {
    if !t.is_matrix() {
        return Err("determinant requires a rank-2 tensor".into());
    }
    // Usar t.determinant() directamente si está implementado
    // O reimplementar aquí si es más complejo
}
```

#### 2.2. Buscar todas las funciones que usan Matrix
```bash
grep -rn "Matrix" crates/achronyme-linalg/src/
```

**Patrones comunes de transformación**:

| Operación Matrix | Equivalente Tensor |
|-----------------|-------------------|
| `m.rows` | `t.rows()` |
| `m.cols` | `t.cols()` |
| `m.data` | `t.data()` |
| `m.get(i, j)` | `t.get_matrix(i, j)` |
| `m.set(i, j, v)` | `t.set_matrix(i, j, v)` |
| `m.is_square()` | `t.is_square()` |
| `m.transpose()` | `t.transpose()` |
| `m.scale(s)` | `t.scale(s)` |
| `Matrix::new(r, c, data)` | `RealTensor::matrix(r, c, data)` |
| `Matrix::zeros(r, c)` | `RealTensor::zeros(vec![r, c])` |
| `Matrix::identity(n)` | `RealTensor::eye(n)` |

---

### FASE 3: Migración de achronyme-solver (Prioridad: ALTA)

**Archivos identificados** (42 usos de Matrix):

#### 3.1. Linear Programming
- `crates/achronyme-solver/src/linear/linprog.rs` (4 usos)
- `crates/achronyme-solver/src/linear/simplex.rs` (8 usos)
- `crates/achronyme-solver/src/linear/dual_simplex.rs` (2 usos)
- `crates/achronyme-solver/src/linear/revised_simplex.rs` (2 usos)
- `crates/achronyme-solver/src/linear/two_phase.rs` (4 usos)
- `crates/achronyme-solver/src/linear/sensitivity.rs` (4 usos)
- `crates/achronyme-solver/src/linear/tableau.rs` (6 usos)

#### 3.2. Integer Programming
- `crates/achronyme-solver/src/integer/branch_bound.rs` (12 usos)

**Estrategia de migración**:

**Paso 1**: Empezar por `tableau.rs` (estructura de datos base)
```bash
# Ver el archivo
cat crates/achronyme-solver/src/linear/tableau.rs
```

Transformar:
```rust
// ANTES:
pub struct Tableau {
    matrix: Matrix,
    // ...
}

impl Tableau {
    pub fn new(matrix: Matrix) -> Self { ... }

    pub fn pivot(&mut self, row: usize, col: usize) {
        let pivot_val = self.matrix.get(row, col).unwrap();
        // ...
    }
}

// DESPUÉS:
pub struct Tableau {
    matrix: RealTensor,  // ✅ Cambiar tipo
    // ...
}

impl Tableau {
    pub fn new(matrix: RealTensor) -> Self { ... }

    pub fn pivot(&mut self, row: usize, col: usize) {
        let pivot_val = self.matrix.get_matrix(row, col).unwrap();  // ✅ Usar get_matrix
        // ...
    }
}
```

**Paso 2**: Migrar archivos dependientes en orden:
1. ✅ `tableau.rs` (base)
2. ✅ `simplex.rs` (usa Tableau)
3. ✅ `dual_simplex.rs` (usa Simplex)
4. ✅ `revised_simplex.rs` (usa Simplex)
5. ✅ `two_phase.rs` (usa Simplex)
6. ✅ `sensitivity.rs` (usa resultados de Simplex)
7. ✅ `linprog.rs` (API pública)
8. ✅ `branch_bound.rs` (usa linprog)

---

### FASE 4: Migración de function_modules (Prioridad: MEDIA)

#### 4.1. Archivo: `crates/achronyme-eval/src/function_modules/matrix.rs`

**Buscar usos**:
```bash
grep -n "Value::Matrix" crates/achronyme-eval/src/function_modules/matrix.rs
```

**Transformación**:
```rust
// FUNCIÓN: transpose()
// ANTES:
fn transpose(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Tensor(t) => { ... }
        Value::ComplexTensor(t) => { ... }
        Value::Matrix(m) => {  // ❌ ELIMINAR ESTE BRANCH
            let transposed = m.transpose();
            Ok(Value::Matrix(transposed))
        }
        _ => Err("transpose() requires a matrix or tensor".to_string()),
    }
}

// DESPUÉS:
fn transpose(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Tensor(t) => {
            if !t.is_matrix() {
                return Err("transpose() requires a rank-2 tensor (matrix)".to_string());
            }
            let result = t.transpose().map_err(|e| e.to_string())?;
            Ok(Value::Tensor(result))
        }
        Value::ComplexTensor(t) => {
            if !t.is_matrix() {
                return Err("transpose() requires a rank-2 tensor (matrix)".to_string());
            }
            let result = t.transpose().map_err(|e| e.to_string())?;
            Ok(Value::ComplexTensor(result))
        }
        _ => Err("transpose() requires a tensor".to_string()),
    }
}
```

**Similar para**:
- `det()` - Usar `t.determinant()`
- `trace()` - Usar `t.trace()`

---

### FASE 5: Migración de handlers (Prioridad: MEDIA)

#### 5.1. Archivo: `crates/achronyme-eval/src/handlers/binary_ops.rs`

**Buscar**:
```bash
grep -n "Value::Matrix" crates/achronyme-eval/src/handlers/binary_ops.rs
```

**Operaciones a migrar**:
- Suma de matrices → suma de tensors
- Multiplicación de matrices → multiplicación de tensors
- etc.

**Patrón de transformación**:
```rust
// ANTES:
pub fn handle_multiply(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Matrix(a), Value::Matrix(b)) => {
            let result = a.mul(&b)?;
            Ok(Value::Matrix(result))
        }
        // ...
    }
}

// DESPUÉS:
pub fn handle_multiply(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Tensor(a), Value::Tensor(b)) => {
            if a.is_matrix() && b.is_matrix() {
                let result = a.matmul(&b)?;  // Multiplicación de matrices
                Ok(Value::Tensor(result))
            } else {
                let result = a.mul(&b)?;  // Hadamard (element-wise)
                Ok(Value::Tensor(result))
            }
        }
        // ...
    }
}
```

#### 5.2. Archivo: `crates/achronyme-eval/src/handlers/literals.rs`

**Buscar creación de matrices**:
```bash
grep -n "Matrix::" crates/achronyme-eval/src/handlers/literals.rs
```

**Transformación**:
```rust
// ANTES:
fn handle_matrix_literal(...) -> Result<Value> {
    let matrix = Matrix::new(rows, cols, data)?;
    Ok(Value::Matrix(matrix))
}

// DESPUÉS:
fn handle_matrix_literal(...) -> Result<Value> {
    let tensor = RealTensor::matrix(rows, cols, data)?;
    Ok(Value::Tensor(tensor))
}
```

#### 5.3. Otros handlers a revisar:
- `crates/achronyme-eval/src/handlers/unary_ops.rs`
- `crates/achronyme-eval/src/handlers/optimization.rs`

---

### FASE 6: Migración de WASM bindings (Prioridad: MEDIA)

#### 6.1. Archivos a modificar:

**API Linear Algebra**:
- `crates/achronyme-wasm/src/api/linalg.rs`

```bash
grep -n "Matrix\|Value::Matrix" crates/achronyme-wasm/src/api/linalg.rs
```

**Transformación**:
```rust
// ANTES:
#[wasm_bindgen]
pub fn matrix_multiply(a_handle: Handle, b_handle: Handle) -> Result<Handle, JsValue> {
    match (value_a, value_b) {
        (Value::Matrix(a), Value::Matrix(b)) => {
            let result = a.mul(&b)?;
            Ok(Value::Matrix(result))
        }
    }
}

// DESPUÉS:
#[wasm_bindgen]
pub fn matrix_multiply(a_handle: Handle, b_handle: Handle) -> Result<Handle, JsValue> {
    match (value_a, value_b) {
        (Value::Tensor(a), Value::Tensor(b)) => {
            if !a.is_matrix() || !b.is_matrix() {
                return Err(JsValue::from_str("Both arguments must be matrices"));
            }
            let result = a.matmul(&b).map_err(|e| JsValue::from_str(&e.to_string()))?;
            Ok(Value::Tensor(result))
        }
        _ => Err(JsValue::from_str("Invalid types"))
    }
}
```

**API Solver**:
- `crates/achronyme-wasm/src/api/solver.rs`
- `crates/achronyme-wasm/src/api/memory.rs`
- `crates/achronyme-wasm/src/api/utils.rs`

---

### FASE 7: Actualización de Tests (Prioridad: ALTA)

#### 7.1. Tests de evaluador
**Archivo**: `crates/achronyme-eval/tests/test_evaluator.rs`

```bash
grep -n "Value::Matrix" crates/achronyme-eval/tests/test_evaluator.rs
```

**Patrón de transformación**:
```rust
// ANTES:
#[test]
fn test_matrix_creation() {
    let result = eval("[1,2; 3,4]");
    match result {
        Value::Matrix(m) => {
            assert_eq!(m.rows, 2);
            assert_eq!(m.cols, 2);
        }
        _ => panic!("Expected matrix")
    }
}

// DESPUÉS:
#[test]
fn test_matrix_creation() {
    let result = eval("[1,2; 3,4]");
    match result {
        Value::Tensor(t) => {
            assert!(t.is_matrix());
            assert_eq!(t.rows(), 2);
            assert_eq!(t.cols(), 2);
        }
        _ => panic!("Expected tensor")
    }
}
```

#### 7.2. Otros tests a revisar:
```bash
# Buscar todos los tests que usan Matrix
find crates -name "*.rs" -type f -exec grep -l "Value::Matrix" {} \;
```

**Lista identificada**:
- `crates/achronyme-cli/src/main.rs`
- Todos los tests en `crates/achronyme-solver/src/linear/*.rs`
- Todos los tests en `crates/achronyme-solver/src/integer/*.rs`

---

### FASE 8: Eliminación de archivos legacy (Prioridad: BAJA - AL FINAL)

**Solo hacer DESPUÉS de que todo compile sin errores**

#### 8.1. Archivos a eliminar:
```bash
# 1. Tipos legacy
rm crates/achronyme-types/src/matrix.rs
rm crates/achronyme-types/src/vector.rs
rm crates/achronyme-types/src/complex_vector.rs
```

#### 8.2. Actualizar lib.rs
**Archivo**: `crates/achronyme-types/src/lib.rs`

```rust
// ANTES:
pub mod value;
pub mod complex;
pub mod vector;           // ❌ ELIMINAR
pub mod complex_vector;   // ❌ ELIMINAR
pub mod matrix;           // ❌ ELIMINAR
pub mod tensor;
pub mod function;
pub mod lambda_evaluator;

// DESPUÉS:
pub mod value;
pub mod complex;
pub mod tensor;
pub mod function;
pub mod lambda_evaluator;

pub use lambda_evaluator::LambdaEvaluator;
```

---

## HERRAMIENTAS DE BÚSQUEDA

### Comandos útiles para encontrar código legacy:

```bash
# 1. Encontrar TODOS los usos de Matrix
grep -rn "Matrix" crates/ --include="*.rs" | grep -v "docs/" | grep -v "target/"

# 2. Encontrar TODOS los usos de Vector (RealVector)
grep -rn "use.*vector::Vector" crates/ --include="*.rs"

# 3. Encontrar TODOS los usos de ComplexVector
grep -rn "ComplexVector" crates/ --include="*.rs"

# 4. Encontrar TODOS los Value::Matrix
grep -rn "Value::Matrix" crates/ --include="*.rs"

# 5. Contar archivos afectados
grep -rl "Value::Matrix" crates/ --include="*.rs" | wc -l

# 6. Ver qué funciones usan Matrix en un archivo específico
grep -n "fn.*Matrix" crates/achronyme-solver/src/linear/simplex.rs
```

---

## CHECKLIST DE ARCHIVOS POR MIGRAR

### Prioridad ALTA ⚠️

#### achronyme-types
- [ ] `src/value.rs` - Eliminar `Value::Matrix`, completar limpieza
- [ ] `src/lib.rs` - Actualizar exports cuando se eliminen archivos

#### achronyme-linalg
- [ ] `src/lib.rs` - Migrar todas las funciones
- [ ] `src/*.rs` - Revisar cada archivo del módulo

#### achronyme-solver (8 archivos)
- [ ] `src/linear/tableau.rs` - Estructura base
- [ ] `src/linear/simplex.rs` - Algoritmo simplex
- [ ] `src/linear/dual_simplex.rs` - Dual simplex
- [ ] `src/linear/revised_simplex.rs` - Revised simplex
- [ ] `src/linear/two_phase.rs` - Two-phase method
- [ ] `src/linear/sensitivity.rs` - Sensitivity analysis
- [ ] `src/linear/linprog.rs` - API pública
- [ ] `src/integer/branch_bound.rs` - Branch and bound

### Prioridad MEDIA 📊

#### achronyme-eval
- [ ] `src/function_modules/matrix.rs` - Funciones de matriz
- [ ] `src/handlers/binary_ops.rs` - Operaciones binarias
- [ ] `src/handlers/literals.rs` - Literales de matriz
- [ ] `src/handlers/unary_ops.rs` - Operaciones unarias
- [ ] `src/handlers/optimization.rs` - Optimización

#### achronyme-wasm
- [ ] `src/api/linalg.rs` - Bindings de álgebra lineal
- [ ] `src/api/solver.rs` - Bindings de solver
- [ ] `src/api/memory.rs` - Manejo de memoria
- [ ] `src/api/utils.rs` - Utilidades

### Prioridad BAJA ✅

#### Tests
- [ ] `crates/achronyme-eval/tests/test_evaluator.rs`
- [ ] Todos los tests en `achronyme-solver`
- [ ] Todos los tests en `achronyme-linalg`

#### Documentación
- [ ] `docs/legacy_code_analysis.md` - Actualizar
- [ ] `docs/tensor_architecture.md` - Actualizar
- [ ] `README.md` - Si menciona Matrix

#### CLI
- [ ] `crates/achronyme-cli/src/main.rs` - Si usa Matrix para ejemplos

---

## PATRONES DE TRANSFORMACIÓN COMUNES

### Patrón 1: Creación de Matrix → Tensor
```rust
// ANTES
let m = Matrix::new(rows, cols, data)?;
let m = Matrix::zeros(rows, cols);
let m = Matrix::identity(n);

// DESPUÉS
let t = RealTensor::matrix(rows, cols, data)?;
let t = RealTensor::zeros(vec![rows, cols]);
let t = RealTensor::eye(n);
```

### Patrón 2: Acceso a elementos
```rust
// ANTES
let val = matrix.get(i, j)?;
matrix.set(i, j, value)?;

// DESPUÉS
let val = tensor.get_matrix(i, j)?;
tensor.set_matrix(i, j, value)?;
```

### Patrón 3: Propiedades
```rust
// ANTES
let r = matrix.rows;
let c = matrix.cols;
let sq = matrix.is_square();

// DESPUÉS
let r = tensor.rows();
let c = tensor.cols();
let sq = tensor.is_square();
```

### Patrón 4: Operaciones
```rust
// ANTES
let t = matrix.transpose();
let d = matrix.determinant()?;
let tr = matrix.trace()?;
let s = matrix.scale(2.0);

// DESPUÉS
let t = tensor.transpose()?;
let d = tensor.determinant()?;
let tr = tensor.trace()?;
let s = tensor.scale(2.0);
```

### Patrón 5: Value enum
```rust
// ANTES
match value {
    Value::Matrix(m) => { ... }
}

// DESPUÉS
match value {
    Value::Tensor(t) => {
        if !t.is_matrix() {
            return Err("Expected matrix (rank-2 tensor)");
        }
        // ...
    }
}
```

---

## ORDEN RECOMENDADO DE MIGRACIÓN

1. ✅ **DSP** (Completado)
2. ✅ **Tensor API** (Completado)
3. 🔄 **value.rs** (50% - eliminar Matrix del enum)
4. ⏳ **achronyme-linalg** (base para solver)
5. ⏳ **achronyme-solver/linear/tableau.rs** (estructura base)
6. ⏳ **Resto de achronyme-solver** (en orden de dependencias)
7. ⏳ **function_modules/matrix.rs**
8. ⏳ **handlers** (binary_ops, literals, etc.)
9. ⏳ **WASM bindings**
10. ⏳ **Tests** (actualizar conforme se rompen)
11. ⏳ **Eliminar archivos legacy** (al final, cuando todo compile)

---

## VALIDACIÓN FINAL

### Antes de hacer commit:

```bash
# 1. Compilar todo
cargo build --all

# 2. Ejecutar todos los tests
cargo test --all

# 3. Verificar que no quedan referencias a tipos legacy
grep -r "use.*matrix::Matrix" crates/ --include="*.rs" | grep -v "docs/" | grep -v "target/"
grep -r "use.*vector::Vector" crates/ --include="*.rs" | grep -v "docs/" | grep -v "target/"
grep -r "use.*ComplexVector" crates/ --include="*.rs" | grep -v "docs/" | grep -v "target/"

# 4. Verificar que Value::Matrix no existe
grep -r "Value::Matrix" crates/ --include="*.rs" | grep -v "docs/" | grep -v "target/"

# 5. Verificar limpieza de lib.rs
cat crates/achronyme-types/src/lib.rs | grep -E "matrix|vector|complex_vector"
```

### Tests críticos a pasar:
- ✅ `cargo test --package achronyme-types`
- ⏳ `cargo test --package achronyme-linalg`
- ⏳ `cargo test --package achronyme-solver`
- ⏳ `cargo test --package achronyme-eval`
- ⏳ `cargo test --package achronyme-wasm`

---

## NOTAS IMPORTANTES

1. **NO eliminar archivos legacy** hasta que todo compile sin errores
2. **Migrar tests** conforme se rompen, no todos al final
3. **Commits incrementales** después de cada fase exitosa
4. **Mantener compatibilidad** en WASM bindings hasta el final
5. **Documentar cambios** en CHANGELOG.md

---

## PROGRESO ACTUAL

```
Total: ~60 archivos a modificar
Completados: ~18 archivos (30%)
Pendientes: ~42 archivos (70%)

Tiempo estimado restante: 2-3 horas
```

---

## SOPORTE

Si encuentras errores de compilación no listados aquí:

1. Busca el patrón en esta guía
2. Si es nuevo, sigue el patrón más similar
3. Documenta el nuevo patrón en este archivo
4. Los métodos de Tensor están diseñados para ser drop-in replacements de Matrix

**Recuerda**: `RealTensor` con `is_matrix()` == true es funcionalmente equivalente a `Matrix`
