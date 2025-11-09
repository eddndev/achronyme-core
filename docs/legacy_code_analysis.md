# Legacy Code Analysis & Cleanup Plan

## Executive Summary

Achronyme actualmente tiene **~87 funciones registradas** con **significativa duplicación** debido a tener tipos separados (`Vector`, `Matrix`, `ComplexVector`). Con el nuevo sistema `Tensor<T>`, podemos eliminar:

- **~400 líneas** de código redundante en `binary_ops.rs`
- **Patrón repetitivo** de conversión Value → Tipo Especializado → Operación → Value
- **Lógica duplicada** para manejar real vs complex en cada función

## Archivos con Código Legacy

### 1. **`handlers/binary_ops.rs`** (~580 líneas)

**Problemas Identificados**:
- **14 funciones** (`apply_add`, `apply_subtract`, `apply_multiply`, etc.)
- **Cada función** repite el mismo patrón para:
  - Number op Number
  - Complex op Complex
  - Matrix op Matrix
  - Vector op Vector (con check real/complex)
  - Broadcasting (4 combinaciones: scalar+vec, vec+scalar, complex+vec, vec+complex)

**Ejemplo de Duplicación**:
```rust
// apply_add tiene ~100 líneas
fn apply_add(left: Value, right: Value) -> Result<Value, String> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
        (Value::Complex(a), Value::Complex(b)) => Ok(Value::Complex(a + b)),
        (Value::Matrix(a), Value::Matrix(b)) => ...,
        (Value::Vector(a), Value::Vector(b)) => {
            // Check if complex
            if has_complex {
                let vec_a = Value::to_complex_vector(a)?;
                let vec_b = Value::to_complex_vector(b)?;
                let result = vec_a.add(&vec_b)?;
                Ok(Value::from_complex_vector(result))
            } else {
                let vec_a = Value::to_real_vector(a)?;
                let vec_b = Value::to_real_vector(b)?;
                let result = vec_a.add(&vec_b)?;
                Ok(Value::from_real_vector(result))
            }
        }
        // Broadcasting cases...
        (Value::Number(s), Value::Vector(v)) => ...,
        (Value::Vector(v), Value::Number(s)) => ...,
        (Value::Complex(c), Value::Vector(v)) => ...,
        (Value::Vector(v), Value::Complex(c)) => ...,
        _ => Err("Incompatible types")
    }
}

// apply_subtract repite EXACTAMENTE el mismo patrón (~100 líneas)
// apply_multiply repite EXACTAMENTE el mismo patrón (~100 líneas)
// apply_divide repite EXACTAMENTE el mismo patrón (~100 líneas)
```

**Con Tensor**:
```rust
fn apply_add(left: Value, right: Value) -> Result<Value, String> {
    match (left, right) {
        (Value::Tensor(a), Value::Tensor(b)) => {
            Tensor::add(&a, &b).map(Value::Tensor)
        }
        (Value::ComplexTensor(a), Value::ComplexTensor(b)) => {
            Tensor::add(&a, &b).map(Value::ComplexTensor)
        }
        // Broadcasting automático en Tensor::add
        // Type promotion automático
        ...
    }
}
```

**Reducción estimada**: ~400 líneas → ~150 líneas (62% menos)

---

### 2. **`function_modules/vector.rs`** (~120 líneas)

**Funciones Actuales**:
```rust
registry.register("dot", dot, 2);
registry.register("cross", cross, 2);
registry.register("norm", norm, 1);
registry.register("normalize", normalize, 1);
```

**Problemas**:
- **Cada función** tiene el patrón:
  1. Match Value::Vector
  2. Check is_numeric_vector
  3. Detectar si hay Complex
  4. Convertir a RealVector o ComplexVector
  5. Ejecutar operación
  6. Convertir de vuelta a Value

**Ejemplo Actual**:
```rust
fn dot(args: &[Value]) -> Result<Value, String> {
    match (&args[0], &args[1]) {
        (Value::Vector(vec1), Value::Vector(vec2)) => {
            if !Value::is_numeric_vector(vec1) || !Value::is_numeric_vector(vec2) {
                return Err("dot() requires numeric vectors");
            }
            let has_complex = vec1.iter().any(complex) || vec2.iter().any(complex);
            if has_complex {
                let v1 = Value::to_complex_vector(vec1)?;
                let v2 = Value::to_complex_vector(vec2)?;
                Ok(Value::Complex(v1.dot(&v2)?))
            } else {
                let v1 = Value::to_real_vector(vec1)?;
                let v2 = Value::to_real_vector(vec2)?;
                Ok(Value::Number(v1.dot(&v2)?))
            }
        }
        _ => Err("dot() requires two vectors")
    }
}
```

**Con Tensor**:
```rust
fn dot(args: &[Value]) -> Result<Value, String> {
    match (&args[0], &args[1]) {
        (Value::Tensor(a), Value::Tensor(b)) => {
            RealTensor::dot(a, b).map(Value::Number)
        }
        (Value::ComplexTensor(a), Value::ComplexTensor(b)) => {
            ComplexTensor::dot(a, b).map(Value::Complex)
        }
        _ => Err("dot() requires tensors")
    }
}
```

**Reducción estimada**: ~120 líneas → ~40 líneas (67% menos)

---

### 3. **`function_modules/matrix.rs`** (~80 líneas)

**Funciones Actuales**:
```rust
registry.register("transpose", transpose, 1);
registry.register("det", det, 1);
registry.register("trace", trace, 1);
```

**Problemas**:
- Operan solo sobre `Value::Matrix`
- No soportan tensores de rango 2 generales
- No hay versión ComplexMatrix

**Con Tensor**:
- `Matrix` se vuelve alias de `RealTensor` con `rank() == 2`
- Operaciones trabajan sobre `Tensor<T>` genérico
- Automáticamente funciona para complejos

---

### 4. **`function_modules/dsp.rs`** (~250 líneas)

**Funciones**:
```rust
registry.register("fft", fft, 1);
registry.register("ifft", ifft, 1);
registry.register("fft_mag", fft_mag, 1);
registry.register("fft_phase", fft_phase, 1);
registry.register("convolve", convolve, 2);
// ... filtros, etc.
```

**Problemas Actuales**:
- Acepta `Vector(Vec<Value>)` y convierte internamente
- Retorna `Vector(Vec<Value>)` con complejos
- **No usa** `ComplexVector` de manera nativa

**Con Tensor**:
```rust
// FFT: RealTensor → ComplexTensor
fn fft(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Tensor(t) if t.is_vector() => {
            let result = dsp::fft(t)?;  // Returns ComplexTensor
            Ok(Value::ComplexTensor(result))
        }
        _ => Err("fft() requires a real tensor")
    }
}
```

**Beneficios**:
- Tipos más claros
- No conversión innecesaria
- Performance (sin allocations intermedias)

---

## Duplicación por Categorías

### A. **Patrón de Conversión Value ↔ Tipo Especializado**

**Actualmente se repite en ~50+ funciones**:
```rust
// Patrón que se repite:
let vec = match &args[0] {
    Value::Vector(v) => {
        if !Value::is_numeric_vector(v) {
            return Err("requires numeric vector");
        }
        if has_complex(v) {
            Value::to_complex_vector(v)?
        } else {
            Value::to_real_vector(v)?
        }
    }
    _ => return Err("requires vector")
};
```

**Con Tensor** (patrón simplificado):
```rust
let tensor = match &args[0] {
    Value::Tensor(t) => t,
    Value::ComplexTensor(t) => t,
    _ => return Err("requires tensor")
};
```

### B. **Broadcasting Manual**

**Actualmente** (~30 líneas por operador):
```rust
// Scalar + Vector
(Value::Number(s), Value::Vector(v)) => {
    let result: Vec<Value> = v.iter().map(|x| match x {
        Value::Number(n) => Value::Number(n + s),
        Value::Complex(c) => Value::Complex(*c + Complex::from_real(s)),
        _ => unreachable!()
    }).collect();
    Ok(Value::Vector(result))
}
// Vector + Scalar (duplicado)
(Value::Vector(v), Value::Number(s)) => { /* mismo código */ }
// Complex + Vector
(Value::Complex(c), Value::Vector(v)) => { /* similar */ }
// Vector + Complex
(Value::Vector(v), Value::Complex(c)) => { /* similar */ }
```

**Con Tensor** (broadcasting automático):
```rust
// Un solo caso - broadcasting automático en Tensor::add
(Value::Tensor(a), Value::Tensor(b)) => {
    Tensor::add_with_broadcast(a, b).map(Value::Tensor)
}
```

### C. **Type Promotion Real ↔ Complex**

**Actualmente** (manual en cada función):
```rust
if has_complex_a || has_complex_b {
    let vec_a = Value::to_complex_vector(a)?;  // Promote if needed
    let vec_b = Value::to_complex_vector(b)?;
    // operate
} else {
    let vec_a = Value::to_real_vector(a)?;
    let vec_b = Value::to_real_vector(b)?;
    // operate
}
```

**Con Tensor** (automático en operación):
```rust
match (a, b) {
    (Value::Tensor(t), Value::ComplexTensor(ct)) => {
        let t_promoted = t.to_complex();  // Automatic promotion
        Tensor::add(&t_promoted, ct)
    }
    ...
}
```

---

## Resumen de Código Redundante

| Archivo | Líneas Actuales | Funciones | Patrón Repetitivo | Reducción Estimada |
|---------|-----------------|-----------|-------------------|-------------------|
| `binary_ops.rs` | ~580 | 14 | Conversión + Match exhaustivo | ~400 → ~150 (-62%) |
| `vector.rs` | ~120 | 4 | Conversión + check complex | ~120 → ~40 (-67%) |
| `matrix.rs` | ~80 | 3 | Solo Matrix, sin complex | ~80 → ~30 (-63%) |
| `dsp.rs` | ~250 | 10+ | Conversión Vec<Value> ↔ tipos | ~250 → ~120 (-52%) |
| `stats.rs` | ~100 | 6 | Similar a vector.rs | ~100 → ~40 (-60%) |
| `common.rs` | ~150 | 8 | Conversión repetitiva | ~150 → ~60 (-60%) |

**Total estimado**: ~1,280 líneas → ~440 líneas (**-65% de código**)

---

## Plan de Limpieza (Fases)

### **Fase 1: Implementar Operaciones en Tensor** (Este paso)

Antes de eliminar legacy, necesitamos que Tensor tenga las operaciones:

1. **Aritmética básica en `tensor.rs`**:
   ```rust
   impl<T: Add<Output=T>> Tensor<T> {
       pub fn add(&self, other: &Tensor<T>) -> Result<Tensor<T>, TensorError>
   }
   ```

2. **Operaciones vectoriales**:
   - `dot()`, `cross()`, `norm()`, `normalize()`

3. **Operaciones matriciales**:
   - `transpose()`, `det()`, `trace()`, `matmul()`

4. **Broadcasting**:
   - Implementar reglas NumPy en operaciones

### **Fase 2: Actualizar Handlers para Usar Tensor** (No ahora - después)

Modificar `binary_ops.rs` y handlers para:
1. Detectar `Value::Tensor` y `Value::ComplexTensor`
2. Llamar a operaciones de `Tensor<T>`
3. Mantener backward compatibility con `Vector` genérico

### **Fase 3: Migrar Funciones Módulo por Módulo** (No ahora - después)

En orden de prioridad:
1. `vector.rs` → usa Tensor
2. `matrix.rs` → usa Tensor (rank 2)
3. `dsp.rs` → FFT usa RealTensor → ComplexTensor
4. `stats.rs` → mean, std, etc. sobre Tensor

### **Fase 4: Deprecar y Eliminar Legacy** (Futuro)

1. Marcar `Value::Matrix` como `#[deprecated]`
2. Documentar path de migración
3. Eliminar tras 1-2 versiones

---

## Ambigüedades a Resolver

### 1. **Vec<Value> vs Tensor**

**Problema**: `Value::Vector` puede ser:
- Numérico: `[1, 2, 3]` → debería ser `Tensor`
- Heterogéneo: `["A", "B", "C"]` → debe seguir siendo `Vector`

**Solución Propuesta**:
- Parser detecta si literal es numérico → crea `Value::Tensor`
- Funciones como `map()` sobre vectores heterogéneos → mantienen `Vector`
- Conversión explícita cuando sea ambiguo

### 2. **Matrix vs Tensor(rank=2)**

**Problema**: `Value::Matrix` y `Value::Tensor` con `rank() == 2` son semánti

camente iguales.

**Solución Propuesta**:
- Fase out gradual de `Matrix`
- Alias de compatibilidad: `type Matrix = RealTensor` (con validación rank==2)
- Migración automática en parser

### 3. **ComplexVector vs ComplexTensor(rank=1)**

**Problema**: Mismo que #2 pero para complejos.

**Solución**: Igual que Matrix - phase out gradual.

---

## Beneficios de la Limpieza

1. **Mantenibilidad**:
   - Una operación implementada → funciona para todos los ranks
   - Menos código = menos bugs

2. **Consistencia**:
   - Mismo comportamiento para vectores, matrices, tensores
   - Broadcasting funciona uniformemente

3. **Performance**:
   - Sin conversiones `Value` ↔ Tipo especializado
   - Memoria contigua (cache-friendly)

4. **Extensibilidad**:
   - Nuevas operaciones solo se escriben una vez
   - Fácil agregar GPU support

---

## Next Steps

**AHORA** (antes de migración):
1. ✅ Tensor<T> estructura creada
2. ⏳ Implementar operaciones aritméticas en Tensor
3. ⏳ Implementar broadcasting
4. ⏳ Tests completos

**DESPUÉS** (migración gradual):
1. Actualizar binary_ops.rs
2. Migrar function_modules uno por uno
3. Deprecar legacy types
4. Eliminar código redundante

---

## Métricas de Éxito

- [ ] Reducción de ~840 líneas de código redundante
- [ ] Todas las operaciones funcionan con Tensor
- [ ] Tests pasan (backward compatibility)
- [ ] Performance igual o mejor
- [ ] Documentación actualizada
