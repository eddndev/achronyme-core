# Achronyme Types

**Tipos de datos compartidos y estructuras fundamentales del sistema Achronyme.**

## 🎯 Responsabilidad

Este crate define los **tipos de datos fundamentales** que fluyen a través de todo el sistema Achronyme:

```
┌─────────────┐
│   Parser    │ → Produce: AstNode (en achronyme-parser)
└─────────────┘
       ↓
┌─────────────┐
│  Evaluator  │ → Consume: AstNode, Produce: Value
└─────────────┘
       ↓
┌─────────────┐
│   Solver    │ → Consume: Value (Function, RealTensor)
└─────────────┘
```

### Tipos principales:
- **`Value`** - Tipo principal de runtime (números, funciones, tensores, etc.)
- **`Complex`** - Números complejos (`a + bi`)
- **`RealTensor`** / **`ComplexTensor`** - Tensores N-dimensionales
- **`Function`** - Representación de funciones (built-in y user-defined)
- **`Environment`** - Gestión de scopes y variables
- **`LambdaEvaluator`** - Trait para evaluar funciones

## 📦 Dependencias

### Externas:
- **`serde`** (opcional) - Serialización de valores
- **`indexmap`** - Mapas ordenados para records

### Internas:
- **Ninguna** - Este es el crate más bajo en la jerarquía (foundation)

## 🔌 Usado por

- **`achronyme-parser`** - Usa `Function` en el AST
- **`achronyme-eval`** - **Principal consumidor**: Evalúa AST → `Value`
- **`achronyme-solver`** - Usa `RealTensor`, `Function` para optimización
- **`achronyme-repl`** - Muestra `Value` al usuario
- **Todos los demás crates** - Tipos universales del sistema

## 🏗️ Arquitectura

```
achronyme-types/
├── src/
│   ├── lib.rs                # Re-exports públicos
│   ├── value.rs              # Enum Value (185 LOC)
│   ├── complex.rs            # Tipo Complex (220 LOC)
│   ├── function.rs           # Tipo Function (115 LOC)
│   ├── environment.rs        # Gestión de scopes (500 LOC)
│   ├── lambda_evaluator.rs  # Trait para evaluación (64 LOC)
│   │
│   └── tensor/               # Sistema de tensores N-D
│       ├── mod.rs
│       ├── core.rs           # RealTensor, ComplexTensor structs
│       ├── display.rs        # Formateo para mostrar tensores
│       ├── conversions.rs    # Real ↔ Complex
│       ├── broadcast.rs      # Broadcasting NumPy-style
│       │
│       ├── arithmetic/       # Operaciones +, -, *, /
│       │   ├── real.rs
│       │   └── complex.rs
│       │
│       ├── matrix_ops/       # Operaciones matriciales
│       │   ├── real.rs       # matmul, transpose, det, inv
│       │   └── complex.rs
│       │
│       ├── vector_ops/       # Operaciones vectoriales
│       │   ├── real.rs       # dot, cross, norm
│       │   └── complex.rs
│       │
│       └── constructors/     # Builders (zeros, ones, eye, etc.)
│           ├── real.rs
│           └── complex.rs
│
└── tests/
    └── integration_tests.rs  # Tests de integración
```

## 📚 Tipos principales

### 1. `Value` - El tipo universal de runtime

```rust
pub enum Value {
    Number(f64),                    // 42, 3.14
    Boolean(bool),                  // true, false
    String(String),                 // "hello"
    Complex(Complex),               // 3+4i
    Vector(Vec<Value>),             // [1, 2, 3]
    Tensor(RealTensor),             // [[1,2],[3,4]]
    ComplexTensor(ComplexTensor),   // [[1+2i, 3+4i]]
    Function(Function),             // x => x * 2
    Record(IndexMap<String, Value>), // { name: "Alice" }
    Edge { from, to, directed, properties },  // "A" -> "B"
    MutableRef(Rc<RefCell<Value>>), // mut x = 10
    TailCall(Vec<Value>),           // TCO marker
    EarlyReturn(Box<Value>),        // return value
}
```

**Características**:
- ✅ Tagged union (safe, exhaustive pattern matching)
- ✅ Recursive (Value puede contener otros Values)
- ✅ Cloneable (necesario para closures)
- ✅ Soporta comparación e igualdad

### 2. `Complex` - Números complejos

```rust
pub struct Complex {
    pub re: f64,  // Parte real
    pub im: f64,  // Parte imaginaria
}
```

**Operaciones**:
- Aritméticas: `+`, `-`, `*`, `/`
- Potencias: `pow(f64)`, `pow_complex(&Complex)`
- Conversión: `from_real(f64)`, `to_polar()`, `from_polar(r, θ)`
- Funciones especiales: `exp()`, `ln()`, `sin()`, `cos()`

**Ejemplo**:
```rust
let z1 = Complex::new(3.0, 4.0);  // 3+4i
let z2 = Complex::from_real(2.0); // 2+0i
let z3 = z1 + z2;                 // 5+4i
let mag = z1.magnitude();         // 5.0
```

### 3. `RealTensor` / `ComplexTensor` - Tensores N-D

```rust
pub struct RealTensor {
    data: Vec<f64>,      // Datos en row-major order
    shape: Vec<usize>,   // Dimensiones [rows, cols, ...]
}

pub struct ComplexTensor {
    data: Vec<Complex>,
    shape: Vec<usize>,
}
```

**Características**:
- ✅ N-dimensional (vectores, matrices, tensores de orden superior)
- ✅ Broadcasting automático (estilo NumPy)
- ✅ Operaciones matriciales (matmul, transpose, determinant, inverse)
- ✅ Slicing e indexing avanzado

**Ejemplo**:
```rust
// Crear matriz 2x3
let matrix = RealTensor::matrix(2, 3, vec![
    1.0, 2.0, 3.0,
    4.0, 5.0, 6.0
])?;

// Multiplicación matricial
let result = matrix.matmul(&other)?;

// Broadcasting
let scaled = matrix.mul_scalar(2.0); // Multiplica por 2
```

### 4. `Function` - Representación de funciones

```rust
pub enum Function {
    Builtin(String),           // "sin", "cos", "map"
    UserDefined {
        params: Vec<String>,   // ["x", "y"]
        body: Box<AstNode>,    // AST del cuerpo
        closure: Environment,  // Captura de variables
    }
}
```

**Tipos de funciones**:
- **Built-in**: Funciones nativas de Achronyme (`sin`, `cos`, `map`, etc.)
- **User-defined**: Lambdas definidas por el usuario con closures

### 5. `Environment` - Gestión de scopes

```rust
pub struct Environment {
    scopes: Vec<HashMap<String, Value>>
}
```

**Operaciones**:
- `push_scope()` / `pop_scope()` - Entrar/salir de bloques
- `define(name, value)` - Declarar variable
- `get(name)` - Leer variable
- `set(name, value)` - Modificar variable mutable
- `has(name)` - Verificar existencia

**Ejemplo**:
```rust
let mut env = Environment::new();

// Scope global
env.define("x".to_string(), Value::Number(10.0))?;

// Nuevo scope
env.push_scope();
env.define("y".to_string(), Value::Number(20.0))?;
assert!(env.has("x")); // true - hereda del padre
assert!(env.has("y")); // true

// Salir del scope
env.pop_scope();
assert!(env.has("x")); // true
assert!(env.has("y")); // false - ya no existe
```

### 6. `LambdaEvaluator` - Trait para evaluación

```rust
pub trait LambdaEvaluator {
    fn eval_at(&mut self, func: &Function, x: f64) -> Result<f64, String>;
    fn eval_vec_at(&mut self, func: &Function, point: &[f64]) -> Result<f64, String>;
    fn eval_at_nd(&mut self, func: &Function, args: &[f64]) -> Result<f64, String>;
}
```

**Propósito**: Permite a `achronyme-solver` evaluar funciones sin depender de `achronyme-eval`.

## 🚀 Uso

### Crear y manipular valores

```rust
use achronyme_types::value::Value;
use achronyme_types::complex::Complex;

// Números
let num = Value::Number(42.0);

// Complejos
let z = Value::Complex(Complex::new(3.0, 4.0));

// Vectores
let vec = Value::Vector(vec![
    Value::Number(1.0),
    Value::Number(2.0),
    Value::Number(3.0),
]);

// Records
use indexmap::IndexMap;
let mut record = IndexMap::new();
record.insert("name".to_string(), Value::String("Alice".to_string()));
record.insert("age".to_string(), Value::Number(30.0));
let person = Value::Record(record);
```

### Trabajar con tensores

```rust
use achronyme_types::tensor::RealTensor;

// Crear matriz identidad 3x3
let identity = RealTensor::eye(3)?;

// Crear matriz desde datos
let matrix = RealTensor::matrix(2, 2, vec![
    1.0, 2.0,
    3.0, 4.0
])?;

// Operaciones
let transposed = matrix.transpose();
let det = matrix.determinant()?;
let inv = matrix.inverse()?;
```

## 📖 Documentación interna

Para entender la implementación:
- [src/README.md](src/README.md) - Arquitectura interna detallada
- [src/tensor/README.md](src/tensor/README.md) - Sistema de tensores (si existe)

## 🧪 Testing

```bash
# Tests del crate types
cargo test --package achronyme-types

# Tests específicos de tensores
cargo test --package achronyme-types tensor

# Tests de complejos
cargo test --package achronyme-types complex
```

## 🔧 Características opcionales

### Serialización (feature `serde`)
```toml
[dependencies]
achronyme-types = { version = "0.1", features = ["serde"] }
```

Permite serializar `Value` para:
- Guardar/cargar estado del evaluador
- Enviar valores por red
- Cachear resultados

## 📊 Estadísticas

- **Líneas de código**: ~1,100 LOC (core) + ~800 LOC (tensores)
- **Tipos principales**: 6 tipos fundamentales
- **Variantes de Value**: 13 variantes
- **Operaciones de tensor**: 20+ operaciones

## 🎯 Principios de diseño

1. **Sin dependencias pesadas** - Solo `indexmap` y `serde` (opcional)
2. **Cloneable por defecto** - Necesario para closures funcionales
3. **Type-safe** - Uso extensivo de enums y pattern matching
4. **Extensible** - Fácil agregar nuevos tipos a `Value`
5. **Interoperabilidad** - Tipos compartidos entre todos los crates

## 🔗 Ver también

- [achronyme-parser](../achronyme-parser/README.md) - Produce AST que usa estos tipos
- [achronyme-eval](../achronyme-eval/README.md) - Evalúa AST a `Value`
- [achronyme-solver](../achronyme-solver/README.md) - Opera sobre `RealTensor` y `Function`
