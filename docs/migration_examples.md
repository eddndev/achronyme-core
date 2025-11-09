# Ejemplos Concretos de Migración Matrix → Tensor

Este documento contiene ejemplos específicos de código **ANTES** y **DESPUÉS** de la migración.

---

## 1. EJEMPLO: Función Simplex Básica

### ANTES (usando Matrix)
```rust
// crates/achronyme-solver/src/linear/simplex.rs

use achronyme_types::matrix::Matrix;

pub struct SimplexSolver {
    tableau: Matrix,
    basis: Vec<usize>,
}

impl SimplexSolver {
    pub fn new(a: Matrix, b: Vec<f64>, c: Vec<f64>) -> Result<Self, String> {
        let m = a.rows;
        let n = a.cols;

        // Crear tableau
        let mut tableau_data = Vec::new();
        for i in 0..m {
            for j in 0..n {
                tableau_data.push(a.get(i, j)?);
            }
            tableau_data.push(b[i]);
        }

        for j in 0..n {
            tableau_data.push(-c[j]);
        }
        tableau_data.push(0.0);

        let tableau = Matrix::new(m + 1, n + 1, tableau_data)?;

        Ok(Self {
            tableau,
            basis: (0..m).collect(),
        })
    }

    pub fn pivot(&mut self, row: usize, col: usize) -> Result<(), String> {
        let pivot_val = self.tableau.get(row, col)?;

        // Dividir fila pivote
        for j in 0..self.tableau.cols {
            let val = self.tableau.get(row, j)?;
            self.tableau.set(row, j, val / pivot_val)?;
        }

        // Actualizar otras filas
        for i in 0..self.tableau.rows {
            if i == row { continue; }

            let factor = self.tableau.get(i, col)?;
            for j in 0..self.tableau.cols {
                let old_val = self.tableau.get(i, j)?;
                let pivot_row_val = self.tableau.get(row, j)?;
                self.tableau.set(i, j, old_val - factor * pivot_row_val)?;
            }
        }

        Ok(())
    }
}
```

### DESPUÉS (usando Tensor)
```rust
// crates/achronyme-solver/src/linear/simplex.rs

use achronyme_types::tensor::RealTensor;

pub struct SimplexSolver {
    tableau: RealTensor,  // ✅ Cambio 1: Matrix → RealTensor
    basis: Vec<usize>,
}

impl SimplexSolver {
    pub fn new(a: RealTensor, b: Vec<f64>, c: Vec<f64>) -> Result<Self, String> {
        // ✅ Cambio 2: Parámetro Matrix → RealTensor

        if !a.is_matrix() {
            return Err("Input must be a matrix (rank-2 tensor)".to_string());
        }

        let m = a.rows();  // ✅ Cambio 3: .rows en lugar de .rows
        let n = a.cols();  // ✅ Cambio 4: .cols() en lugar de .cols

        // Crear tableau
        let mut tableau_data = Vec::new();
        for i in 0..m {
            for j in 0..n {
                tableau_data.push(a.get_matrix(i, j).map_err(|e| e.to_string())?);
                // ✅ Cambio 5: get_matrix() en lugar de get()
            }
            tableau_data.push(b[i]);
        }

        for j in 0..n {
            tableau_data.push(-c[j]);
        }
        tableau_data.push(0.0);

        let tableau = RealTensor::matrix(m + 1, n + 1, tableau_data)
            .map_err(|e| e.to_string())?;
        // ✅ Cambio 6: RealTensor::matrix() en lugar de Matrix::new()

        Ok(Self {
            tableau,
            basis: (0..m).collect(),
        })
    }

    pub fn pivot(&mut self, row: usize, col: usize) -> Result<(), String> {
        let pivot_val = self.tableau.get_matrix(row, col)
            .map_err(|e| e.to_string())?;
        // ✅ Cambio 7: get_matrix() en lugar de get()

        // Dividir fila pivote
        for j in 0..self.tableau.cols() {  // ✅ Cambio 8: cols()
            let val = self.tableau.get_matrix(row, j)
                .map_err(|e| e.to_string())?;
            self.tableau.set_matrix(row, j, val / pivot_val)
                .map_err(|e| e.to_string())?;
            // ✅ Cambio 9: set_matrix() en lugar de set()
        }

        // Actualizar otras filas
        for i in 0..self.tableau.rows() {  // ✅ Cambio 10: rows()
            if i == row { continue; }

            let factor = self.tableau.get_matrix(i, col)
                .map_err(|e| e.to_string())?;
            for j in 0..self.tableau.cols() {
                let old_val = self.tableau.get_matrix(i, j)
                    .map_err(|e| e.to_string())?;
                let pivot_row_val = self.tableau.get_matrix(row, j)
                    .map_err(|e| e.to_string())?;
                self.tableau.set_matrix(i, j, old_val - factor * pivot_row_val)
                    .map_err(|e| e.to_string())?;
            }
        }

        Ok(())
    }
}
```

---

## 2. EJEMPLO: Operaciones de Matriz en Handlers

### ANTES (binary_ops.rs)
```rust
// crates/achronyme-eval/src/handlers/binary_ops.rs

pub fn handle_add(left: Value, right: Value) -> Result<Value, String> {
    match (left, right) {
        // Suma de matrices
        (Value::Matrix(a), Value::Matrix(b)) => {
            let result = a.add(&b)
                .map_err(|e| format!("Matrix addition failed: {}", e))?;
            Ok(Value::Matrix(result))
        }

        // Suma de números
        (Value::Number(a), Value::Number(b)) => {
            Ok(Value::Number(a + b))
        }

        _ => Err("Type mismatch in addition".to_string()),
    }
}

pub fn handle_multiply(left: Value, right: Value) -> Result<Value, String> {
    match (left, right) {
        // Multiplicación de matrices
        (Value::Matrix(a), Value::Matrix(b)) => {
            let result = a.mul(&b)
                .map_err(|e| format!("Matrix multiplication failed: {}", e))?;
            Ok(Value::Matrix(result))
        }

        // Escalar por matriz
        (Value::Number(s), Value::Matrix(m)) => {
            let result = m.scale(s);
            Ok(Value::Matrix(result))
        }

        _ => Err("Type mismatch in multiplication".to_string()),
    }
}
```

### DESPUÉS (binary_ops.rs)
```rust
// crates/achronyme-eval/src/handlers/binary_ops.rs

pub fn handle_add(left: Value, right: Value) -> Result<Value, String> {
    match (left, right) {
        // Suma de tensors (incluyendo matrices)
        (Value::Tensor(a), Value::Tensor(b)) => {
            let result = a.add(&b)
                .map_err(|e| format!("Tensor addition failed: {}", e))?;
            Ok(Value::Tensor(result))
        }

        // Suma de números
        (Value::Number(a), Value::Number(b)) => {
            Ok(Value::Number(a + b))
        }

        _ => Err("Type mismatch in addition".to_string()),
    }
}

pub fn handle_multiply(left: Value, right: Value) -> Result<Value, String> {
    match (left, right) {
        // Multiplicación de tensors
        (Value::Tensor(a), Value::Tensor(b)) => {
            // Si ambos son matrices, hacer multiplicación matricial
            if a.is_matrix() && b.is_matrix() {
                let result = a.matmul(&b)
                    .map_err(|e| format!("Matrix multiplication failed: {}", e))?;
                Ok(Value::Tensor(result))
            } else {
                // Multiplicación element-wise (Hadamard)
                let result = a.mul(&b)
                    .map_err(|e| format!("Element-wise multiplication failed: {}", e))?;
                Ok(Value::Tensor(result))
            }
        }

        // Escalar por tensor
        (Value::Number(s), Value::Tensor(t)) => {
            let result = t.scale(s);
            Ok(Value::Tensor(result))
        }

        (Value::Tensor(t), Value::Number(s)) => {
            let result = t.scale(s);
            Ok(Value::Tensor(result))
        }

        _ => Err("Type mismatch in multiplication".to_string()),
    }
}
```

---

## 3. EJEMPLO: Literales de Matriz

### ANTES (literals.rs)
```rust
// crates/achronyme-eval/src/handlers/literals.rs

use achronyme_types::matrix::Matrix;

pub fn handle_matrix_literal(rows: Vec<Vec<f64>>) -> Result<Value, String> {
    if rows.is_empty() {
        return Err("Empty matrix".to_string());
    }

    let num_rows = rows.len();
    let num_cols = rows[0].len();

    // Verificar que todas las filas tengan el mismo largo
    for row in &rows {
        if row.len() != num_cols {
            return Err("All rows must have the same length".to_string());
        }
    }

    // Aplanar en row-major order
    let data: Vec<f64> = rows.into_iter().flatten().collect();

    let matrix = Matrix::new(num_rows, num_cols, data)
        .map_err(|e| e.to_string())?;

    Ok(Value::Matrix(matrix))
}
```

### DESPUÉS (literals.rs)
```rust
// crates/achronyme-eval/src/handlers/literals.rs

use achronyme_types::tensor::RealTensor;

pub fn handle_matrix_literal(rows: Vec<Vec<f64>>) -> Result<Value, String> {
    if rows.is_empty() {
        return Err("Empty matrix".to_string());
    }

    let num_rows = rows.len();
    let num_cols = rows[0].len();

    // Verificar que todas las filas tengan el mismo largo
    for row in &rows {
        if row.len() != num_cols {
            return Err("All rows must have the same length".to_string());
        }
    }

    // Aplanar en row-major order
    let data: Vec<f64> = rows.into_iter().flatten().collect();

    let tensor = RealTensor::matrix(num_rows, num_cols, data)
        .map_err(|e| e.to_string())?;

    Ok(Value::Tensor(tensor))
}
```

---

## 4. EJEMPLO: Function Module (matrix.rs)

### ANTES
```rust
// crates/achronyme-eval/src/function_modules/matrix.rs

fn det(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Matrix(m) => {
            let determinant = achronyme_linalg::determinant_nd(m)
                .map_err(|e| format!("Determinant failed: {}", e))?;
            Ok(Value::Number(determinant))
        }
        _ => Err("det() requires a matrix".to_string()),
    }
}

fn trace(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Matrix(m) => {
            if m.rows != m.cols {
                return Err("trace() requires a square matrix".to_string());
            }
            let mut sum = 0.0;
            for i in 0..m.rows {
                sum += m.get(i, i).map_err(|e| e.to_string())?;
            }
            Ok(Value::Number(sum))
        }
        _ => Err("trace() requires a matrix".to_string()),
    }
}

fn transpose(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Matrix(m) => {
            let transposed = m.transpose();
            Ok(Value::Matrix(transposed))
        }
        _ => Err("transpose() requires a matrix".to_string()),
    }
}
```

### DESPUÉS
```rust
// crates/achronyme-eval/src/function_modules/matrix.rs

fn det(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Tensor(t) => {
            if !t.is_matrix() {
                return Err("det() requires a rank-2 tensor (matrix)".to_string());
            }
            let determinant = t.determinant()
                .map_err(|e| format!("Determinant failed: {}", e))?;
            Ok(Value::Number(determinant))
        }
        Value::ComplexTensor(t) => {
            if !t.is_matrix() {
                return Err("det() requires a rank-2 tensor (matrix)".to_string());
            }
            // TODO: Implementar determinante para matrices complejas
            Err("Complex matrix determinant not yet implemented".to_string())
        }
        _ => Err("det() requires a tensor".to_string()),
    }
}

fn trace(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Tensor(t) => {
            let result = t.trace()
                .map_err(|e| e.to_string())?;
            Ok(Value::Number(result))
        }
        Value::ComplexTensor(t) => {
            let result = t.trace()
                .map_err(|e| e.to_string())?;
            Ok(Value::Complex(result))
        }
        _ => Err("trace() requires a tensor".to_string()),
    }
}

fn transpose(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Tensor(t) => {
            if !t.is_matrix() {
                return Err("transpose() requires a rank-2 tensor (matrix)".to_string());
            }
            let result = t.transpose()
                .map_err(|e| e.to_string())?;
            Ok(Value::Tensor(result))
        }
        Value::ComplexTensor(t) => {
            if !t.is_matrix() {
                return Err("transpose() requires a rank-2 tensor (matrix)".to_string());
            }
            let result = t.transpose()
                .map_err(|e| e.to_string())?;
            Ok(Value::ComplexTensor(result))
        }
        _ => Err("transpose() requires a tensor".to_string()),
    }
}
```

---

## 5. EJEMPLO: WASM Bindings

### ANTES
```rust
// crates/achronyme-wasm/src/api/linalg.rs

use achronyme_types::matrix::Matrix;

#[wasm_bindgen]
pub fn matrix_transpose(handle: Handle) -> Result<Handle, JsValue> {
    HANDLES.with(|h| {
        let result = {
            let handles = h.borrow();
            let value = handles.get(handle)
                .ok_or_else(|| JsValue::from_str("Invalid handle"))?;

            match value {
                Value::Matrix(m) => {
                    let transposed = m.transpose();
                    Ok(Value::Matrix(transposed))
                }
                _ => Err(JsValue::from_str("Expected matrix"))
            }
        }?;

        Ok(h.borrow_mut().create(result))
    })
}

#[wasm_bindgen]
pub fn matrix_determinant(handle: Handle) -> Result<f64, JsValue> {
    HANDLES.with(|h| {
        let handles = h.borrow();
        let value = handles.get(handle)
            .ok_or_else(|| JsValue::from_str("Invalid handle"))?;

        match value {
            Value::Matrix(m) => {
                let det = achronyme_linalg::determinant_nd(m)
                    .map_err(|e| JsValue::from_str(&e))?;
                Ok(det)
            }
            _ => Err(JsValue::from_str("Expected matrix"))
        }
    })
}
```

### DESPUÉS
```rust
// crates/achronyme-wasm/src/api/linalg.rs

use achronyme_types::tensor::RealTensor;

#[wasm_bindgen]
pub fn matrix_transpose(handle: Handle) -> Result<Handle, JsValue> {
    HANDLES.with(|h| {
        let result = {
            let handles = h.borrow();
            let value = handles.get(handle)
                .ok_or_else(|| JsValue::from_str("Invalid handle"))?;

            match value {
                Value::Tensor(t) => {
                    if !t.is_matrix() {
                        return Err(JsValue::from_str("Expected matrix (rank-2 tensor)"));
                    }
                    let transposed = t.transpose()
                        .map_err(|e| JsValue::from_str(&e.to_string()))?;
                    Ok(Value::Tensor(transposed))
                }
                _ => Err(JsValue::from_str("Expected tensor"))
            }
        }?;

        Ok(h.borrow_mut().create(result))
    })
}

#[wasm_bindgen]
pub fn matrix_determinant(handle: Handle) -> Result<f64, JsValue> {
    HANDLES.with(|h| {
        let handles = h.borrow();
        let value = handles.get(handle)
            .ok_or_else(|| JsValue::from_str("Invalid handle"))?;

        match value {
            Value::Tensor(t) => {
                if !t.is_matrix() {
                    return Err(JsValue::from_str("Expected matrix (rank-2 tensor)"));
                }
                let det = t.determinant()
                    .map_err(|e| JsValue::from_str(&e.to_string()))?;
                Ok(det)
            }
            _ => Err(JsValue::from_str("Expected tensor"))
        }
    })
}
```

---

## 6. EJEMPLO: Tests

### ANTES
```rust
// tests/test_evaluator.rs

#[test]
fn test_matrix_transpose() {
    let mut eval = Evaluator::new();
    let result = eval.eval_str("[[1,2],[3,4]]'").unwrap();

    match result {
        Value::Matrix(m) => {
            assert_eq!(m.rows, 2);
            assert_eq!(m.cols, 2);
            assert_eq!(m.get(0, 0).unwrap(), 1.0);
            assert_eq!(m.get(0, 1).unwrap(), 3.0);
            assert_eq!(m.get(1, 0).unwrap(), 2.0);
            assert_eq!(m.get(1, 1).unwrap(), 4.0);
        }
        _ => panic!("Expected matrix")
    }
}

#[test]
fn test_matrix_multiplication() {
    let mut eval = Evaluator::new();
    let result = eval.eval_str("[[1,2],[3,4]] * [[5,6],[7,8]]").unwrap();

    match result {
        Value::Matrix(m) => {
            assert_eq!(m.rows, 2);
            assert_eq!(m.cols, 2);
            // [[1*5+2*7, 1*6+2*8], [3*5+4*7, 3*6+4*8]]
            // [[19, 22], [43, 50]]
            assert_eq!(m.get(0, 0).unwrap(), 19.0);
            assert_eq!(m.get(0, 1).unwrap(), 22.0);
            assert_eq!(m.get(1, 0).unwrap(), 43.0);
            assert_eq!(m.get(1, 1).unwrap(), 50.0);
        }
        _ => panic!("Expected matrix")
    }
}
```

### DESPUÉS
```rust
// tests/test_evaluator.rs

#[test]
fn test_matrix_transpose() {
    let mut eval = Evaluator::new();
    let result = eval.eval_str("[[1,2],[3,4]]'").unwrap();

    match result {
        Value::Tensor(t) => {
            assert!(t.is_matrix());
            assert_eq!(t.rows(), 2);
            assert_eq!(t.cols(), 2);
            assert_eq!(t.get_matrix(0, 0).unwrap(), 1.0);
            assert_eq!(t.get_matrix(0, 1).unwrap(), 3.0);
            assert_eq!(t.get_matrix(1, 0).unwrap(), 2.0);
            assert_eq!(t.get_matrix(1, 1).unwrap(), 4.0);
        }
        _ => panic!("Expected tensor")
    }
}

#[test]
fn test_matrix_multiplication() {
    let mut eval = Evaluator::new();
    let result = eval.eval_str("[[1,2],[3,4]] * [[5,6],[7,8]]").unwrap();

    match result {
        Value::Tensor(t) => {
            assert!(t.is_matrix());
            assert_eq!(t.rows(), 2);
            assert_eq!(t.cols(), 2);
            // [[1*5+2*7, 1*6+2*8], [3*5+4*7, 3*6+4*8]]
            // [[19, 22], [43, 50]]
            assert_eq!(t.get_matrix(0, 0).unwrap(), 19.0);
            assert_eq!(t.get_matrix(0, 1).unwrap(), 22.0);
            assert_eq!(t.get_matrix(1, 0).unwrap(), 43.0);
            assert_eq!(t.get_matrix(1, 1).unwrap(), 50.0);
        }
        _ => panic!("Expected tensor")
    }
}
```

---

## 7. RESUMEN DE CAMBIOS COMUNES

| Operación | ANTES (Matrix) | DESPUÉS (Tensor) |
|-----------|----------------|------------------|
| **Tipo** | `Matrix` | `RealTensor` |
| **Creación** | `Matrix::new(r, c, data)` | `RealTensor::matrix(r, c, data)` |
| **Zeros** | `Matrix::zeros(r, c)` | `RealTensor::zeros(vec![r, c])` |
| **Identity** | `Matrix::identity(n)` | `RealTensor::eye(n)` |
| **Filas** | `m.rows` (campo) | `t.rows()` (método) |
| **Columnas** | `m.cols` (campo) | `t.cols()` (método) |
| **Get** | `m.get(i, j)` | `t.get_matrix(i, j)` |
| **Set** | `m.set(i, j, v)` | `t.set_matrix(i, j, v)` |
| **Data** | `m.data` (campo) | `t.data()` (método) |
| **Transponer** | `m.transpose()` | `t.transpose()` |
| **Determinante** | `m.determinant()` | `t.determinant()` |
| **Traza** | `m.trace()` | `t.trace()` |
| **Escalar** | `m.scale(s)` | `t.scale(s)` |
| **Verificar** | `m.is_square()` | `t.is_square()` |
| **Verificar tipo** | - | `t.is_matrix()` ⚠️ NUEVO |
| **Value** | `Value::Matrix(m)` | `Value::Tensor(t)` |

---

## NOTAS IMPORTANTES

1. **Siempre verificar `is_matrix()`** cuando trabajas con Tensors que esperas sean matrices
2. **Los métodos de Tensor retornan `Result`** cuando pueden fallar
3. **Use `map_err(|e| e.to_string())`** para convertir TensorError a String
4. **Campos vs Métodos**: Matrix usa campos públicos, Tensor usa métodos
5. **Nomenclatura**: `get_matrix()` y `set_matrix()` son específicos para matrices, `get()` y `set()` son para acceso general con índices N-dimensionales
