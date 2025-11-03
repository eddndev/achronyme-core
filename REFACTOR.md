# Plan de Refactorización: Migración de C++ a Rust
## Achronyme Core - Compilador y Motor de Ejecución

**Versión:** 1.0
**Fecha:** Noviembre 2025
**Estado:** Propuesta inicial
**Autor:** Análisis arquitectónico completo

---

## 📋 Tabla de Contenidos

1. [Resumen Ejecutivo](#resumen-ejecutivo)
2. [Análisis de Arquitectura Actual (C++)](#análisis-de-arquitectura-actual-c)
3. [Arquitectura Propuesta (Rust)](#arquitectura-propuesta-rust)
4. [Plan de Migración por Fases](#plan-de-migración-por-fases)
5. [Impacto en el SDK TypeScript](#impacto-en-el-sdk-typescript)
6. [Mapeo de Componentes C++ → Rust](#mapeo-de-componentes-c-rust)
7. [Herramientas y Dependencias](#herramientas-y-dependencias)
8. [Beneficios y Ventajas](#beneficios-y-ventajas)
9. [Riesgos y Mitigaciones](#riesgos-y-mitigaciones)
10. [Cronograma Estimado](#cronograma-estimado)

---

## 🎯 Resumen Ejecutivo

Este documento describe el plan completo para migrar el núcleo de Achronyme de **C++20 con Emscripten** a **Rust con wasm-bindgen/wasm-pack**, manteniendo la compatibilidad con el SDK TypeScript existente.

### Motivación de la Migración

1. **Seguridad de memoria**: Rust elimina vulnerabilidades de memoria en tiempo de compilación
2. **Mejor experiencia de desarrollo**: Sistema de tipos más robusto, mejor gestión de errores
3. **Ecosistema WASM maduro**: wasm-bindgen proporciona mejor integración con JavaScript
4. **Performance predecible**: Sin necesidad de garbage collection, mejor control de memoria
5. **Mantenibilidad**: Código más expresivo y menos propenso a errores

### Alcance

- **Migración completa** del compilador y motor de ejecución
- **Mantenimiento de API** compatible con el SDK TypeScript actual
- **Mejora de rendimiento** especialmente en operaciones de fast path
- **Modernización** del sistema de tipos y gestión de errores

---

## 🏗️ Análisis de Arquitectura Actual (C++)

### Estructura de Directorios

```
wasm/
├── src/
│   ├── parser/          # Compilador (Lexer, Parser, AST, Evaluator)
│   ├── core/            # Tipos de datos y valores
│   ├── linalg/          # Álgebra lineal avanzada
│   ├── dsp/             # Procesamiento de señales
│   ├── numerical/       # Métodos numéricos
│   ├── optimization/    # Optimización
│   ├── soc/             # Superior Order Calculator
│   └── bindings/        # Emscripten bindings
├── include/
│   └── achronyme/       # Headers públicos
└── CMakeLists.txt       # Build con CMake + Emscripten
```

### Componentes Principales

#### 1. **Parser Pipeline** (`wasm/src/parser/`)

```cpp
Lexer (lexer.cpp/.hpp)
  ↓ tokens
Parser (parser.cpp/.hpp)
  ↓ AST
Evaluator (evaluator.cpp/.hpp)
  ↓ Value
```

**Características:**
- Lexer basado en caracteres con lookahead
- Parser recursivo descendente
- AST polimórfico con herencia (`ASTNode` abstracto)
- Evaluador con ambiente persistente (variables)
- Soporte para lambdas con closures

**Tipos de AST:**
- `NumberNode`, `BinaryOpNode`, `UnaryOpNode`
- `FunctionCallNode`, `ComplexLiteralNode`
- `VectorLiteralNode`, `MatrixLiteralNode`
- `VariableDeclarationNode`, `VariableReferenceNode`
- `LambdaNode`

#### 2. **Core Types** (`wasm/src/core/`)

```cpp
Value (value.cpp/.hpp)        // std::variant<double, Complex, Vector, Matrix, Function>
├── Complex                   // Números complejos
├── Vector                    // Vectores matemáticos
├── Matrix                    // Matrices (row-major)
└── Function                  // Lambdas con closures

Constants (constants.cpp/.hpp)
Functions (functions.cpp/.hpp)
HandleManager (handle_manager.cpp/.hpp)
```

**Sistema de tipos:**
- `Value` usa `std::variant` para polimorfismo
- Promoción automática de tipos (Number → Complex)
- Operadores sobrecargados para aritmética
- `HandleManager` para fast path API

#### 3. **Linear Algebra** (`wasm/src/linalg/`)

```cpp
decompositions.cpp/.hpp
├── LU decomposition (con/sin pivoting)
├── QR decomposition (Householder, Gram-Schmidt)
├── Cholesky decomposition
└── SVD decomposition

eigensolvers.cpp/.hpp
├── Power iteration
├── QR algorithm
└── Symmetric eigensolvers
```

#### 4. **Bindings** (`wasm/src/bindings/`)

**Dos APIs diferenciadas:**

1. **Expression API** (parsing-based):
   ```cpp
   eval(expression: string) → string
   reset() → void
   ```

2. **Fast Path API** (handle-based):
   ```cpp
   createVectorFromBuffer(ptr, len) → Handle
   fft_fast(handle) → Handle
   vadd_fast(h1, h2) → Handle
   lu_decomposition_js(matrix) → {L, U, P}
   ```

**Emscripten Features:**
- `EMSCRIPTEN_BINDINGS` macro
- Gestión manual de memoria (`_malloc`, `_free`)
- Acceso directo a heaps (`HEAPF64`, `HEAPU32`)

### Problemas Identificados en C++

1. **Gestión de memoria manual**: Propenso a leaks y use-after-free
2. **std::variant** limitado: No tan ergonómico como Rust enums
3. **Herencia con punteros**: `std::unique_ptr<ASTNode>` complica ownership
4. **Emscripten bindings**: Sintaxis verbosa, errores en runtime
5. **Thread safety**: Variables globales (`globalEvaluator`) problemáticas
6. **Error handling**: Excepciones atraviesan la barrera WASM-JS

---

## 🦀 Arquitectura Propuesta (Rust)

### Estructura de Crates

```
achronyme-core/
├── Cargo.toml
└── crates/
    ├── achronyme-parser/      # Lexer, Parser, AST
    ├── achronyme-eval/        # Evaluador e intérprete
    ├── achronyme-types/       # Value, Complex, Vector, Matrix
    ├── achronyme-linalg/      # Álgebra lineal
    ├── achronyme-dsp/         # DSP
    ├── achronyme-numerical/   # Métodos numéricos
    ├── achronyme-wasm/        # Bindings WASM (wasm-bindgen)
    └── achronyme-cli/         # CLI nativa (opcional)
```

### Ventajas del Diseño en Rust

#### 1. **Parser con tipos algebraicos (ADT)**

```rust
// AST como enum nativo - mucho más limpio que herencia
pub enum AstNode {
    Number(f64),
    BinaryOp {
        op: BinaryOp,
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    UnaryOp {
        op: UnaryOp,
        operand: Box<AstNode>,
    },
    FunctionCall {
        name: String,
        args: Vec<AstNode>,
    },
    ComplexLiteral { re: f64, im: f64 },
    VectorLiteral(Vec<AstNode>),
    MatrixLiteral(Vec<Vec<AstNode>>),
    VariableDecl {
        name: String,
        initializer: Box<AstNode>,
    },
    VariableRef(String),
    Lambda {
        params: Vec<String>,
        body: Box<AstNode>,
    },
}

// Pattern matching exhaustivo - el compilador verifica todos los casos
impl AstNode {
    pub fn evaluate(&self, env: &mut Environment) -> Result<Value, EvalError> {
        match self {
            AstNode::Number(n) => Ok(Value::Number(*n)),
            AstNode::BinaryOp { op, left, right } => {
                let l = left.evaluate(env)?;
                let r = right.evaluate(env)?;
                op.apply(l, r)
            }
            // ... resto de casos
        }
    }
}
```

#### 2. **Value System con Enums**

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    Complex(Complex),
    Vector(Vector),
    Matrix(Matrix),
    Function(Function),
}

// Conversiones automáticas con From/Into
impl From<f64> for Value {
    fn from(n: f64) -> Self {
        Value::Number(n)
    }
}

// Operadores sobrecargados de forma segura
impl std::ops::Add for Value {
    type Output = Result<Value, TypeError>;

    fn add(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
            (Value::Vector(a), Value::Vector(b)) => a.add(&b).map(Value::Vector),
            // Type promotion
            (Value::Number(a), Value::Complex(b)) => {
                Ok(Value::Complex(Complex::new(a, 0.0) + b))
            }
            _ => Err(TypeError::IncompatibleTypes),
        }
    }
}
```

#### 3. **Error Handling Tipado**

```rust
#[derive(Debug, thiserror::Error)]
pub enum AchronymeError {
    #[error("Parse error at position {pos}: {message}")]
    ParseError { pos: usize, message: String },

    #[error("Type error: {0}")]
    TypeError(#[from] TypeError),

    #[error("Undefined variable: {0}")]
    UndefinedVariable(String),

    #[error("Function {name} expects {expected} arguments, got {got}")]
    ArityMismatch {
        name: String,
        expected: usize,
        got: usize,
    },

    #[error("Linear algebra error: {0}")]
    LinAlgError(String),
}

// Propagación de errores con ? operator
pub fn eval(expr: &str) -> Result<Value, AchronymeError> {
    let tokens = Lexer::new(expr).tokenize()?;
    let ast = Parser::new(tokens).parse()?;
    let value = Evaluator::new().evaluate(&ast)?;
    Ok(value)
}
```

#### 4. **WASM Bindings con wasm-bindgen**

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Achronyme {
    evaluator: Evaluator,
}

#[wasm_bindgen]
impl Achronyme {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        // Mejor manejo de panics en WASM
        console_error_panic_hook::set_once();

        Self {
            evaluator: Evaluator::new(),
        }
    }

    /// Evalúa una expresión y retorna el resultado
    #[wasm_bindgen]
    pub fn eval(&mut self, expression: &str) -> Result<String, JsValue> {
        self.evaluator
            .eval(expression)
            .map(|v| v.to_string())
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Crea un vector desde un Float64Array
    #[wasm_bindgen]
    pub fn create_vector(&self, data: &[f64]) -> VectorHandle {
        let vector = Vector::new(data.to_vec());
        VectorHandle::new(vector)
    }
}

// Handles con seguridad de tipos
#[wasm_bindgen]
pub struct VectorHandle {
    inner: Rc<RefCell<Vector>>,
}

#[wasm_bindgen]
impl VectorHandle {
    /// FFT rápido
    #[wasm_bindgen]
    pub fn fft(&self) -> Result<VectorHandle, JsValue> {
        self.inner
            .borrow()
            .fft()
            .map(|v| VectorHandle::new(v))
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Obtiene datos como Float64Array
    #[wasm_bindgen(getter)]
    pub fn data(&self) -> Vec<f64> {
        self.inner.borrow().data().to_vec()
    }
}
```

#### 5. **Linear Algebra con ndarray**

```rust
use ndarray::{Array1, Array2};
use ndarray_linalg::*;

pub struct Matrix {
    data: Array2<f64>,
}

impl Matrix {
    /// LU decomposition usando ndarray-linalg
    pub fn lu_decomposition(&self) -> Result<LUDecomposition, LinAlgError> {
        let (l, u, p) = self.data.lu()?;
        Ok(LUDecomposition {
            l: Matrix { data: l },
            u: Matrix { data: u },
            p: p.into_iter().collect(),
        })
    }

    /// QR decomposition
    pub fn qr_decomposition(&self) -> Result<QRDecomposition, LinAlgError> {
        let (q, r) = self.data.qr()?;
        Ok(QRDecomposition {
            q: Matrix { data: q },
            r: Matrix { data: r },
        })
    }

    /// SVD
    pub fn svd(&self) -> Result<SVDDecomposition, LinAlgError> {
        let svd = self.data.svd(true, true)?;
        Ok(SVDDecomposition {
            u: svd.u.map(|u| Matrix { data: u }),
            s: svd.s.to_vec(),
            vt: svd.vt.map(|vt| Matrix { data: vt }),
        })
    }
}
```

#### 6. **Environment Thread-Safe**

```rust
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct Environment {
    // Thread-safe con Arc<RwLock>
    bindings: Arc<RwLock<HashMap<String, Value>>>,
    parent: Option<Box<Environment>>,
}

impl Environment {
    pub fn define(&self, name: String, value: Value) -> Result<(), EvalError> {
        let mut bindings = self.bindings.write()
            .map_err(|_| EvalError::LockError)?;
        bindings.insert(name, value);
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        // Lee desde current scope o parent
        let bindings = self.bindings.read().ok()?;
        bindings.get(name).cloned()
            .or_else(|| self.parent.as_ref()?.get(name))
    }
}
```

---

## 📅 Plan de Migración por Fases

### Fase 0: Preparación (Semana 1-2)

**Objetivos:**
- Setup del proyecto Rust
- Configuración de herramientas
- Pruebas de concepto

**Tareas:**
1. Crear workspace de Cargo con estructura de crates
2. Setup wasm-pack y wasm-bindgen
3. Configurar CI/CD para Rust (GitHub Actions)
4. Crear suite de tests de integración basada en casos C++ existentes
5. Proof of concept: Parser mínimo con lexer y AST

**Entregables:**
- `Cargo.toml` workspace configurado
- Pipeline CI/CD funcional
- POC: Lexer + Parser + eval("2 + 2")

---

### Fase 1: Core Types y Parser (Semana 3-5)

**Objetivos:**
- Migrar sistema de tipos (Value, Complex, Vector, Matrix)
- Implementar parser completo
- Tests unitarios

**Tareas:**

#### 1.1 Core Types (`achronyme-types`)
```rust
// value.rs
pub enum Value { Number, Complex, Vector, Matrix, Function }

// complex.rs
pub struct Complex { re: f64, im: f64 }

// vector.rs
pub struct Vector(Vec<f64>);

// matrix.rs
pub struct Matrix { rows: usize, cols: usize, data: Vec<f64> }

// function.rs
pub struct Function { params: Vec<String>, body: AstNode, closure: Environment }
```

#### 1.2 Parser (`achronyme-parser`)
```rust
// token.rs
pub enum Token { Number(f64), Identifier(String), Plus, ... }

// lexer.rs
pub struct Lexer { source: &str, pos: usize }

// ast.rs
pub enum AstNode { Number, BinaryOp, ... }

// parser.rs
pub struct Parser { tokens: Vec<Token>, current: usize }
```

**Tests:**
- Unit tests para cada tipo
- Property-based tests con `proptest` o `quickcheck`
- Parser tests: validar AST generado vs. C++

---

### Fase 2: Evaluador y Environment (Semana 6-7)

**Objetivos:**
- Implementar evaluador de AST
- Sistema de variables y closures
- Funciones built-in básicas

**Tareas:**

#### 2.1 Evaluator (`achronyme-eval`)
```rust
// evaluator.rs
pub struct Evaluator {
    env: Environment,
}

impl Evaluator {
    pub fn eval(&mut self, ast: &AstNode) -> Result<Value, EvalError> {
        match ast {
            AstNode::Number(n) => Ok(Value::Number(*n)),
            AstNode::BinaryOp { op, left, right } => {
                let l = self.eval(left)?;
                let r = self.eval(right)?;
                self.eval_binary_op(op, l, r)
            }
            // ...
        }
    }
}

// environment.rs
pub struct Environment {
    bindings: HashMap<String, Value>,
    parent: Option<Box<Environment>>,
}

// functions.rs
pub struct FunctionRegistry {
    functions: HashMap<String, BuiltinFunction>,
}
```

**Tests:**
- Expresiones aritméticas básicas
- Variables y declaraciones
- Lambdas y closures
- Funciones matemáticas (sin, cos, sqrt, etc.)

---

### Fase 3: Linear Algebra y DSP (Semana 8-10)

**Objetivos:**
- Migrar álgebra lineal avanzada
- Implementar DSP (FFT, convolución)
- Aprovechar crates existentes

**Tareas:**

#### 3.1 Linear Algebra (`achronyme-linalg`)
```rust
// Dependencias
ndarray = "0.15"
ndarray-linalg = "0.16"
blas-src = { version = "0.8", features = ["openblas"] }
openblas-src = { version = "0.10", features = ["cblas", "system"] }

// decompositions.rs
impl Matrix {
    pub fn lu(&self) -> Result<LUDecomp, LinAlgError>
    pub fn qr(&self) -> Result<QRDecomp, LinAlgError>
    pub fn cholesky(&self) -> Result<Matrix, LinAlgError>
    pub fn svd(&self) -> Result<SVDDecomp, LinAlgError>
}

// eigensolvers.rs
impl Matrix {
    pub fn power_iteration(&self, max_iter: usize) -> Result<EigenPair, LinAlgError>
    pub fn qr_eigenvalues(&self) -> Result<Vec<f64>, LinAlgError>
    pub fn eigen_symmetric(&self) -> Result<EigenDecomp, LinAlgError>
}
```

#### 3.2 DSP (`achronyme-dsp`)
```rust
// Dependencias
rustfft = "6.0"
num-complex = "0.4"

// fft.rs
pub fn fft(signal: &[f64]) -> Vec<Complex<f64>>
pub fn ifft(spectrum: &[Complex<f64>]) -> Vec<f64>
pub fn fft_mag(signal: &[f64]) -> Vec<f64>
pub fn fft_phase(signal: &[f64]) -> Vec<f64>

// convolution.rs
pub fn convolve(x: &[f64], h: &[f64]) -> Vec<f64>
pub fn convolve_fft(x: &[f64], h: &[f64]) -> Vec<f64>
```

**Tests:**
- Comparar resultados con implementación C++
- Tests de precisión numérica
- Benchmarks de performance

---

### Fase 4: WASM Bindings (Semana 11-13)

**Objetivos:**
- Crear bindings WASM con wasm-bindgen
- Mantener compatibilidad con SDK TypeScript
- Optimizar fast path API

**Tareas:**

#### 4.1 Expression API
```rust
#[wasm_bindgen]
pub struct AchronymeCore {
    evaluator: Evaluator,
}

#[wasm_bindgen]
impl AchronymeCore {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self { ... }

    #[wasm_bindgen]
    pub fn eval(&mut self, expr: &str) -> Result<String, JsValue> { ... }

    #[wasm_bindgen]
    pub fn reset(&mut self) { ... }
}
```

#### 4.2 Fast Path API
```rust
#[wasm_bindgen]
pub struct VectorHandle {
    inner: Rc<RefCell<Vector>>,
}

#[wasm_bindgen]
impl VectorHandle {
    #[wasm_bindgen(js_name = createFromBuffer)]
    pub fn from_buffer(data: &[f64]) -> Self { ... }

    #[wasm_bindgen(js_name = fft)]
    pub fn fft(&self) -> Result<VectorHandle, JsValue> { ... }

    #[wasm_bindgen(js_name = add)]
    pub fn add(&self, other: &VectorHandle) -> Result<VectorHandle, JsValue> { ... }
}

#[wasm_bindgen]
pub struct MatrixHandle { ... }

#[wasm_bindgen]
impl MatrixHandle {
    #[wasm_bindgen(js_name = luDecomposition)]
    pub fn lu(&self) -> Result<LUResult, JsValue> { ... }
}
```

#### 4.3 Build System
```toml
# Cargo.toml
[package]
name = "achronyme-wasm"
version = "0.4.0"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
wasm-bindgen = "0.2"
js-sys = "0.3"
wasm-bindgen-futures = "0.4"
console_error_panic_hook = "0.1"
serde = { version = "1.0", features = ["derive"] }
serde-wasm-bindgen = "0.6"

[dependencies.web-sys]
version = "0.3"
features = ["console"]

[profile.release]
opt-level = 3
lto = true
```

**Build script:**
```bash
#!/bin/bash
# scripts/build-wasm.sh

wasm-pack build \
  --target web \
  --out-dir ../dist \
  --scope achronyme \
  crates/achronyme-wasm
```

---

### Fase 5: Integración con SDK TypeScript (Semana 14-15)

**Objetivos:**
- Adaptar SDK para usar bindings Rust
- Mantener compatibilidad de API
- Testing end-to-end

**Cambios mínimos en SDK:**

```typescript
// src/sdk/Achronyme.ts

// ANTES (C++ con Emscripten)
import createAchronymeModule from '../achronyme-core.mjs';
const moduleInstance = await createAchronymeModule();
this.module = moduleInstance as unknown as WasmModule;

// DESPUÉS (Rust con wasm-bindgen)
import init, { AchronymeCore } from '../pkg/achronyme_wasm.js';
await init(); // Inicializa WASM
this.core = new AchronymeCore();
```

**API permanece igual:**
```typescript
// Usuario no ve cambios
const ach = new Achronyme();
await ach.init();

const result = ach.eval("sin(PI/2)"); // ✓ Funciona igual
const vec = ach.vector([1, 2, 3, 4]);
const fft = vec.fft_mag();
```

**Adaptaciones necesarias:**

1. **Gestión de memoria:**
```typescript
// C++ usaba _malloc/_free manual
// Rust usa Drop trait automático - más seguro
// NO hay cambios en la API pública
```

2. **Error handling:**
```typescript
// ANTES: Excepciones de C++ envueltas
// DESPUÉS: Result<T, JsValue> de Rust - mejor tipado
try {
  const result = ach.eval("invalid syntax");
} catch (e) {
  // Mismo manejo de errores
  console.error(e);
}
```

3. **Fast path handles:**
```typescript
// La API de handles se mantiene
// Internamente usa Rc<RefCell<T>> en lugar de raw pointers
// Más seguro, sin cambios en la interfaz
```

---

### Fase 6: Testing, Optimización y Despliegue (Semana 16-18)

**Objetivos:**
- Suite completa de tests
- Benchmarks y optimizaciones
- Documentación
- Release

**Tareas:**

#### 6.1 Testing
```bash
# Tests unitarios
cargo test --all

# Tests de integración
cargo test --test integration

# Tests WASM
wasm-pack test --headless --firefox
wasm-pack test --headless --chrome

# Tests del SDK TypeScript
npm test
```

#### 6.2 Benchmarks
```rust
// benches/eval_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_eval(c: &mut Criterion) {
    let mut eval = Evaluator::new();

    c.bench_function("eval simple", |b| {
        b.iter(|| eval.eval(black_box("2 + 2")))
    });

    c.bench_function("eval complex", |b| {
        b.iter(|| eval.eval(black_box("sin(PI/2) + cos(PI/4)^2")))
    });
}

criterion_group!(benches, benchmark_eval);
criterion_main!(benches);
```

#### 6.3 Optimizaciones
- Profiling con `cargo flamegraph`
- Optimizar hot paths identificados
- SIMD para operaciones vectoriales (`packed_simd`)
- Considerar `wasm-opt` para reducir tamaño del binario

#### 6.4 Documentación
```rust
//! # Achronyme Core
//!
//! Achronyme es un compilador y motor de ejecución para cálculos matemáticos avanzados.
//!
//! ## Características
//! - Números complejos, vectores, matrices
//! - Álgebra lineal avanzada (LU, QR, SVD, eigenvalues)
//! - DSP (FFT, convolución)
//! - Lambdas y funciones de orden superior
//!
//! ## Ejemplo
//! ```rust
//! use achronyme_core::Evaluator;
//!
//! let mut eval = Evaluator::new();
//! let result = eval.eval("sin(PI/2)").unwrap();
//! assert_eq!(result, Value::Number(1.0));
//! ```

// Generar docs
cargo doc --open
```

---

## 📦 Impacto en el SDK TypeScript

### Cambios Mínimos Requeridos

#### 1. **Inicialización del módulo**

**ANTES (Emscripten):**
```typescript
import createAchronymeModule from '../achronyme-core.mjs';

async init(): Promise<void> {
  const moduleInstance = await createAchronymeModule();
  this.module = moduleInstance as unknown as WasmModule;
}
```

**DESPUÉS (wasm-bindgen):**
```typescript
import init, { AchronymeCore } from '../pkg/achronyme_wasm.js';

async init(): Promise<void> {
  await init(); // Inicializa el módulo WASM
  this.core = new AchronymeCore();
}
```

#### 2. **API Expression (eval)**

**Sin cambios en la interfaz:**
```typescript
// API permanece idéntica
_eval(expression: string): string {
  // ANTES: this.module.eval(expression)
  // DESPUÉS: this.core.eval(expression)
  return this.core.eval(expression);
}
```

#### 3. **Fast Path API (Handles)**

**ANTES (gestión manual de memoria):**
```typescript
private _allocFloat64(data: ArrayLike<number>): number {
  const byteLength = data.length * 8;
  const ptr = this.module._malloc(byteLength);
  const heap = this.module.HEAPF64.subarray(ptr / 8, ptr / 8 + data.length);
  for (let i = 0; i < data.length; i++) {
    heap[i] = data[i];
  }
  return ptr;
}
```

**DESPUÉS (Rust gestiona memoria automáticamente):**
```typescript
// Simplificado - wasm-bindgen maneja la conversión
vector(data: number[]): AchronymeValue {
  const handle = this.core.create_vector(new Float64Array(data));
  return new AchronymeValue(this, handle);
}
```

#### 4. **Error Handling**

**ANTES:**
```typescript
try {
  const result = this.module.eval(expression);
  return result;
} catch (e: any) {
  throw wrapCppError(e.message || String(e), expression);
}
```

**DESPUÉS (más tipado):**
```typescript
try {
  // Rust retorna Result<String, JsValue>
  // wasm-bindgen convierte automáticamente
  const result = this.core.eval(expression);
  return result;
} catch (e: any) {
  // Los errores de Rust ya tienen formato estructurado
  throw new AchronymeError(e);
}
```

### Mejoras Automáticas del SDK

1. **Mejor gestión de memoria**
   - No más leaks por `_malloc`/`_free` olvidados
   - Rust's `Drop` trait libera recursos automáticamente

2. **TypeScript types más precisos**
   - wasm-bindgen genera `.d.ts` exactos
   - Mejor autocomplete y type checking

3. **Mensajes de error mejorados**
   - Errores estructurados desde Rust
   - Stack traces más útiles

4. **Mejor performance**
   - Menos overhead de conversión
   - Optimizaciones de Rust compiler

### Plan de Compatibilidad

**Estrategia: Dual Build Temporal**

Durante la transición, mantener ambas versiones:

```json
{
  "exports": {
    "./cpp": {
      "import": "./dist/achronyme-core.mjs",
      "types": "./dist/sdk/index.d.ts"
    },
    "./rust": {
      "import": "./pkg/achronyme_wasm.js",
      "types": "./pkg/achronyme_wasm.d.ts"
    },
    ".": {
      "import": "./pkg/achronyme_wasm.js",
      "types": "./pkg/achronyme_wasm.d.ts"
    }
  }
}
```

**Testing:**
```typescript
describe('Compatibility Tests', () => {
  it('Rust produces same results as C++', async () => {
    const cppCore = await initCppCore();
    const rustCore = await initRustCore();

    const testCases = [
      "2 + 2",
      "sin(PI/2)",
      "[1, 2, 3] + [4, 5, 6]",
      // ... 100+ casos
    ];

    for (const expr of testCases) {
      const cppResult = cppCore.eval(expr);
      const rustResult = rustCore.eval(expr);
      expect(rustResult).toBe(cppResult);
    }
  });
});
```

---

## 🗺️ Mapeo de Componentes C++ → Rust

### Parser Components

| C++ | Rust | Notas |
|-----|------|-------|
| `lexer.hpp/cpp` | `lexer.rs` | Similar, pero con iteradores |
| `parser.hpp/cpp` | `parser.rs` | Match expressions más limpias |
| `ast.hpp` | `ast.rs` | Enum en lugar de herencia |
| `evaluator.hpp/cpp` | `evaluator.rs` | Pattern matching exhaustivo |
| `environment.hpp/cpp` | `environment.rs` | Arc<RwLock> thread-safe |

### Core Types

| C++ | Rust | Equivalencia |
|-----|------|--------------|
| `std::variant<...>` | `enum Value` | Rust enums son más ergonómicos |
| `std::unique_ptr<T>` | `Box<T>` | Ownership directo |
| `std::shared_ptr<T>` | `Rc<T>` / `Arc<T>` | Single/Multi-thread |
| `std::vector<T>` | `Vec<T>` | Muy similar |
| `std::string` | `String` | Similar |

### Linear Algebra

| C++ (custom) | Rust (ndarray) | Ventajas |
|--------------|----------------|----------|
| Custom LU | `ndarray-linalg::LU` | Más optimizado, BLAS backend |
| Custom QR | `ndarray-linalg::QR` | Múltiples algoritmos |
| Custom SVD | `ndarray-linalg::SVD` | LAPACK backend |
| Gaussian elim | `ndarray` ops | Vectorizado, SIMD |

### DSP

| C++ | Rust | Crate |
|-----|------|-------|
| Custom FFT | `rustfft` | Fastest FFT implementation |
| Custom convolution | `rustfft` + custom | Optimizado |

### WASM Bindings

| Emscripten | wasm-bindgen | Ventajas |
|------------|--------------|----------|
| `EMSCRIPTEN_BINDINGS` | `#[wasm_bindgen]` | Más declarativo |
| `_malloc`/`_free` | Automático | Sin leaks |
| `val` manual | `JsValue` auto | Type-safe |
| Runtime checks | Compile-time | Más rápido |

---

## 🛠️ Herramientas y Dependencias

### Rust Toolchain

```bash
# Instalar Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Instalar WASM target
rustup target add wasm32-unknown-unknown

# Instalar wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Herramientas adicionales
cargo install cargo-watch      # Auto-rebuild
cargo install cargo-flamegraph # Profiling
cargo install wasm-opt         # Optimize WASM size
```

### Dependencias del Proyecto

```toml
[workspace]
members = [
    "crates/achronyme-parser",
    "crates/achronyme-eval",
    "crates/achronyme-types",
    "crates/achronyme-linalg",
    "crates/achronyme-dsp",
    "crates/achronyme-wasm",
]

[workspace.dependencies]
# Error handling
thiserror = "1.0"
anyhow = "1.0"

# Linear algebra
ndarray = { version = "0.15", features = ["blas", "serde"] }
ndarray-linalg = "0.16"
blas-src = { version = "0.8", features = ["openblas"] }
lapack-src = { version = "0.8", features = ["openblas"] }

# DSP
rustfft = "6.0"
num-complex = "0.4"

# WASM bindings
wasm-bindgen = "0.2"
js-sys = "0.3"
web-sys = { version = "0.3", features = ["console"] }
console_error_panic_hook = "0.1"
wasm-bindgen-futures = "0.4"

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde-wasm-bindgen = "0.6"
serde_json = "1.0"

# Testing
proptest = "1.0"       # Property-based testing
criterion = "0.5"      # Benchmarking
```

### CI/CD Pipeline

```yaml
# .github/workflows/rust.yml
name: Rust CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable

      # Cache
      - uses: Swatinem/rust-cache@v2

      # Tests
      - name: Run tests
        run: cargo test --all

      # Clippy (linter)
      - name: Run clippy
        run: cargo clippy -- -D warnings

      # Format check
      - name: Check formatting
        run: cargo fmt -- --check

  wasm:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-unknown

      # wasm-pack
      - name: Install wasm-pack
        run: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

      # Build WASM
      - name: Build WASM
        run: |
          cd crates/achronyme-wasm
          wasm-pack build --target web

      # Test WASM (headless browser)
      - name: Test WASM
        run: |
          cd crates/achronyme-wasm
          wasm-pack test --headless --firefox

  benchmark:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable

      - name: Run benchmarks
        run: cargo bench --all
```

---

## ⚡ Beneficios y Ventajas

### 1. **Seguridad de Memoria**

**C++ (propenso a errores):**
```cpp
// Memory leak potencial
double* data = new double[size];
// ... código que puede lanzar excepción
// delete[] data; // ← puede no ejecutarse
```

**Rust (imposible sin unsafe):**
```rust
// Drop automático garantizado
let data = vec![0.0; size];
// ... código que puede retornar error
// } ← drop llamado automáticamente
```

### 2. **Eliminación de Data Races**

**C++ (undefined behavior):**
```cpp
// Global mutable - data race posible
parser::Evaluator globalEvaluator;

// Multiples threads accediendo sin sincronización
```

**Rust (compilador previene):**
```rust
// No compila si hay data race
// Arc<RwLock> garantiza thread safety
let evaluator = Arc::new(RwLock::new(Evaluator::new()));
```

### 3. **Error Handling Robusto**

**C++ (excepciones atraviesan WASM):**
```cpp
std::string eval(const std::string& expr) {
    try {
        // ... código
    } catch (const std::exception& e) {
        return std::string("Error: ") + e.what();
    }
}
```

**Rust (tipado, exhaustivo):**
```rust
pub fn eval(&self, expr: &str) -> Result<Value, AchronymeError> {
    let tokens = self.tokenize(expr)?;
    let ast = self.parse(tokens)?;
    Ok(self.evaluate(ast)?)
}
// Compilador fuerza manejar errores
```

### 4. **Mejor Ecosistema WASM**

| Aspecto | Emscripten (C++) | wasm-bindgen (Rust) |
|---------|------------------|---------------------|
| Type safety | Runtime | Compile-time |
| Memory management | Manual | Automático |
| JS interop | `val` manual | `JsValue` seamless |
| Async support | Callback hell | `async`/`await` |
| Bundle size | ~200KB (típico) | ~100KB (optimizado) |
| Debug info | Limitado | Excelente |

### 5. **Performance**

**Optimizaciones automáticas:**
- LLVM backend con más optimizaciones
- Zero-cost abstractions
- Inlining agresivo
- SIMD automático con `packed_simd`

**Benchmarks esperados:**
```
Operación           C++       Rust      Mejora
-------------------------------------------------
eval("2+2")         1.2μs     0.8μs     1.5x
FFT (1024)          45μs      38μs      1.2x
Matrix mult (100x)  850μs     720μs     1.2x
LU decomp (50x50)   1.2ms     0.95ms    1.3x
```

### 6. **Mantenibilidad**

**Código más expresivo:**
```rust
// Pattern matching exhaustivo
match ast_node {
    AstNode::Number(n) => Value::Number(n),
    AstNode::BinaryOp { op, left, right } => {
        let l = self.eval(left)?;
        let r = self.eval(right)?;
        op.apply(l, r)?
    }
    // Compilador verifica todos los casos
}
```

vs.

```cpp
// Herencia con dynamic cast
switch (node->type()) {
    case ASTNodeType::NUMBER:
        return evaluateNumber(static_cast<const NumberNode*>(node));
    case ASTNodeType::BINARY_OP:
        return evaluateBinaryOp(static_cast<const BinaryOpNode*>(node));
    // Fácil olvidar un caso
}
```

### 7. **Testing y Documentación**

**Built-in test framework:**
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_eval_simple() {
        let result = eval("2 + 2").unwrap();
        assert_eq!(result, Value::Number(4.0));
    }

    #[test]
    #[should_panic(expected = "Division by zero")]
    fn test_div_zero() {
        eval("1 / 0").unwrap();
    }
}
```

**Doc tests ejecutables:**
```rust
/// Evalúa una expresión matemática
///
/// # Ejemplo
/// ```
/// # use achronyme::eval;
/// let result = eval("sin(PI/2)").unwrap();
/// assert_eq!(result, 1.0);
/// ```
pub fn eval(expr: &str) -> Result<Value, Error> { ... }
```

---

## ⚠️ Riesgos y Mitigaciones

### Riesgo 1: Curva de Aprendizaje

**Descripción:** El equipo necesita aprender Rust

**Mitigación:**
- Semana 1-2: Training intensivo de Rust
- Pair programming Rust experto + equipo
- Code reviews rigurosos
- Documentación interna extensiva
- Recursos: [The Rust Book](https://doc.rust-lang.org/book/), [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

### Riesgo 2: Incompatibilidad con SDK

**Descripción:** Cambios en WASM API rompen SDK TypeScript

**Mitigación:**
- Suite de tests de compatibilidad (ver Fase 5)
- Dual build temporal (C++ + Rust en paralelo)
- Feature flags para activar Rust gradualmente
- Rollback plan: mantener C++ como fallback

### Riesgo 3: Regresión de Performance

**Descripción:** Implementación Rust más lenta que C++ optimizado

**Mitigación:**
- Benchmarks continuos vs. C++ baseline
- Profiling con `cargo flamegraph`
- Usar crates probados (ndarray-linalg, rustfft)
- Optimizaciones SIMD donde sea necesario
- Criterio de aceptación: no más de 5% slower (idealmente faster)

### Riesgo 4: Tamaño del Bundle WASM

**Descripción:** WASM binario demasiado grande

**Mitigación:**
```toml
[profile.release]
opt-level = 'z'      # Optimizar para tamaño
lto = true           # Link-time optimization
codegen-units = 1    # Mejor optimización
strip = true         # Strip símbolos
```

```bash
# Post-procesamiento
wasm-opt -Oz -o output.wasm input.wasm
wasm-strip output.wasm
```

**Target: < 150KB gzipped** (vs. ~180KB actual C++)

### Riesgo 5: Dependencias de Terceros

**Descripción:** Bugs o mantenimiento de crates externos

**Mitigación:**
- Usar crates maduros y mantenidos activamente
- Revisar dependency tree regularmente
- `cargo audit` para vulnerabilidades
- Tener plan B para dependencias críticas
- Considerar fork si es necesario

### Riesgo 6: Timeline Underestimation

**Descripción:** Migración toma más tiempo del estimado

**Mitigación:**
- Cronograma conservador (18 semanas)
- Buffer de 4 semanas adicionales
- Enfoque iterativo: MVP primero, optimizaciones después
- Milestones claros y medibles
- Weekly progress reviews

---

## 📊 Cronograma Estimado

### Overview (18 semanas + 4 buffer)

```
Semanas 1-2:   Preparación y POC
Semanas 3-5:   Core Types y Parser
Semanas 6-7:   Evaluador y Environment
Semanas 8-10:  Linear Algebra y DSP
Semanas 11-13: WASM Bindings
Semanas 14-15: Integración SDK
Semanas 16-18: Testing, Optimización, Release
Semanas 19-22: Buffer para issues inesperados
```

### Detalles por Fase

#### Fase 0: Preparación (2 semanas)
- [x] Setup proyecto Rust (1 día)
- [x] Configurar CI/CD (2 días)
- [x] POC Lexer básico (3 días)
- [x] POC Parser mínimo (4 días)
- [x] Tests iniciales (2 días)

#### Fase 1: Core Types y Parser (3 semanas)
- [ ] Implementar Value, Complex, Vector, Matrix (1 semana)
- [ ] Implementar Lexer completo (2 días)
- [ ] Implementar Parser completo (5 días)
- [ ] Tests exhaustivos (3 días)

#### Fase 2: Evaluador (2 semanas)
- [ ] Environment y variables (3 días)
- [ ] Evaluador básico (4 días)
- [ ] Funciones built-in (3 días)
- [ ] Lambdas y closures (4 días)

#### Fase 3: LinAlg y DSP (3 semanas)
- [ ] Integrar ndarray (2 días)
- [ ] Descomposiciones (LU, QR, Cholesky, SVD) (1 semana)
- [ ] Eigensolvers (4 días)
- [ ] FFT y DSP (5 días)

#### Fase 4: WASM Bindings (3 semanas)
- [ ] Expression API (1 semana)
- [ ] Fast Path API (handles) (1 semana)
- [ ] LinAlg bindings (1 semana)

#### Fase 5: SDK Integration (2 semanas)
- [ ] Adaptar SDK TypeScript (1 semana)
- [ ] Tests end-to-end (1 semana)

#### Fase 6: Polish (3 semanas)
- [ ] Suite completa de tests (1 semana)
- [ ] Benchmarks y optimizaciones (1 semana)
- [ ] Documentación y release (1 semana)

### Milestones

| Milestone | Fecha (desde inicio) | Deliverable |
|-----------|----------------------|-------------|
| M1 | Semana 2 | POC funcional (eval básico) |
| M2 | Semana 5 | Parser completo pasa todos los tests |
| M3 | Semana 7 | Evaluador soporta variables y lambdas |
| M4 | Semana 10 | LinAlg y DSP equivalente a C++ |
| M5 | Semana 13 | WASM build funcional |
| M6 | Semana 15 | SDK TypeScript integrado |
| M7 | Semana 18 | Release candidate |
| **RELEASE** | **Semana 22** | **v0.5.0 Rust Edition** |

---

## 📈 Métricas de Éxito

### Criterios de Aceptación

1. **Funcionalidad:**
   - ✓ 100% de tests C++ porteados y pasando
   - ✓ Compatibilidad total con SDK TypeScript existente
   - ✓ Sin regresiones en features

2. **Performance:**
   - ✓ Benchmarks: max 5% slower que C++ (idealmente faster)
   - ✓ Bundle size: < 150KB gzipped
   - ✓ Tiempo de inicialización: < 100ms

3. **Calidad:**
   - ✓ 90%+ code coverage
   - ✓ 0 warnings de Clippy
   - ✓ 0 vulnerabilidades (cargo audit)
   - ✓ Documentación completa (rustdoc)

4. **Mantenibilidad:**
   - ✓ Código pasa `cargo fmt` sin cambios
   - ✓ Tests ejecutan en < 30s
   - ✓ Build WASM en < 2min

---

## 🚀 Próximos Pasos

### Inmediato (Esta Semana)

1. **Aprobación del plan:** Review con stakeholders
2. **Setup inicial:**
   ```bash
   cargo new --lib achronyme-core
   cd achronyme-core
   cargo init --lib crates/achronyme-parser
   cargo init --lib crates/achronyme-types
   # ...
   ```
3. **POC sprint:** Demostrar eval("2+2") funciona end-to-end

### Próximas 2 Semanas

1. Implementar Value types
2. Lexer completo
3. Parser básico
4. CI/CD pipeline

### Largo Plazo

- Completar migración según cronograma
- Release v0.5.0 Rust Edition
- Deprecar versión C++ (v0.6.0)
- Continuar mejoras en Rust

---

## 📚 Referencias y Recursos

### Documentación Rust

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [The Cargo Book](https://doc.rust-lang.org/cargo/)
- [The rustc Book](https://doc.rust-lang.org/rustc/)

### WASM con Rust

- [Rust and WebAssembly Book](https://rustwasm.github.io/docs/book/)
- [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/)
- [wasm-pack Documentation](https://rustwasm.github.io/wasm-pack/)

### Dependencias Clave

- [ndarray](https://docs.rs/ndarray/) - N-dimensional arrays
- [ndarray-linalg](https://docs.rs/ndarray-linalg/) - Linear algebra
- [rustfft](https://docs.rs/rustfft/) - FFT implementation
- [thiserror](https://docs.rs/thiserror/) - Error handling
- [criterion](https://docs.rs/criterion/) - Benchmarking

### Ejemplos de Proyectos Similares

- [num-rs](https://github.com/rust-num/num) - Numeric types
- [nalgebra](https://nalgebra.org/) - Linear algebra
- [polars](https://github.com/pola-rs/polars) - DataFrame library (Rust → Python/JS)

---

## 💬 Preguntas y Respuestas

### ¿Por qué Rust y no mantener C++?

**Respuesta:** Rust ofrece las mismas garantías de performance que C++ pero con seguridad de memoria en tiempo de compilación, mejor ecosistema WASM, y código más mantenible. El costo inicial de migración se recupera en reducción de bugs y mayor velocidad de desarrollo.

### ¿Qué pasa si encontramos un blocker crítico?

**Respuesta:** Tenemos 4 semanas de buffer y mantenemos C++ como fallback. Además, el plan es iterativo: podemos lanzar un MVP y seguir mejorando.

### ¿Impacta a los usuarios finales?

**Respuesta:** No. La API del SDK TypeScript permanece idéntica. Los usuarios solo verán mejoras de performance y mensajes de error más claros.

### ¿Cuánto cuesta en términos de recursos?

**Respuesta:** Estimado 1-2 desarrolladores full-time durante 5-6 meses. ROI positivo en 1 año por reducción de bugs y mayor productividad.

---

## ✅ Conclusión

La migración de C++ a Rust es una inversión estratégica que modernizará la base de código de Achronyme, mejorará la seguridad, mantenibilidad y performance, mientras mantiene compatibilidad total con el ecosistema existente.

**Recomendación:** ✅ **Proceder con la migración** siguiendo el plan de 18 semanas con 4 de buffer.

---

**Documento preparado por:** Análisis arquitectónico de Achronyme Core
**Última actualización:** Noviembre 2025
**Versión:** 1.0
**Estado:** Propuesta para aprobación
