# Types Implementation

**Documentación interna de la implementación de tipos de Achronyme.**

## 🏛️ Arquitectura de módulos

```
src/
├── lib.rs              # Re-exports públicos
├── value.rs            # Enum Value + helpers (185 LOC)
├── complex.rs          # Struct Complex + operaciones (220 LOC)
├── function.rs         # Enum Function (115 LOC)
├── environment.rs      # Gestión de scopes (500 LOC)
├── lambda_evaluator.rs # Trait LambdaEvaluator (64 LOC)
│
└── tensor/             # Sistema completo de tensores (~800 LOC)
    ├── mod.rs          # Re-exports y módulos públicos
    ├── core.rs         # RealTensor, ComplexTensor structs
    ├── display.rs      # Impl Display para pretty-printing
    ├── conversions.rs  # Real ↔ Complex
    ├── broadcast.rs    # Broadcasting NumPy-style
    │
    ├── arithmetic/     # Operaciones básicas
    │   ├── mod.rs
    │   ├── real.rs     # Add, Sub, Mul, Div para RealTensor
    │   └── complex.rs  # Add, Sub, Mul, Div para ComplexTensor
    │
    ├── matrix_ops/     # Álgebra lineal
    │   ├── mod.rs
    │   ├── real.rs     # matmul, transpose, det, inv
    │   └── complex.rs  # matmul, transpose, det, inv (complex)
    │
    ├── vector_ops/     # Operaciones vectoriales
    │   ├── mod.rs
    │   ├── real.rs     # dot, cross, norm
    │   └── complex.rs  # dot, norm (complex)
    │
    └── constructors/   # Builders especializados
        ├── mod.rs
        ├── real.rs     # zeros, ones, eye, linspace
        └── complex.rs  # zeros, ones, eye (complex)
```

## 📊 El tipo `Value` - Diseño e implementación

### Enum completo

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    // Primitivos
    Number(f64),
    Boolean(bool),
    String(String),

    // Numéricos avanzados
    Complex(Complex),
    Vector(Vec<Value>),           // Vector genérico (puede contener cualquier tipo)
    Tensor(RealTensor),           // Tensor optimizado de f64
    ComplexTensor(ComplexTensor), // Tensor optimizado de Complex

    // Funciones
    Function(Function),

    // Estructuras
    Record(HashMap<String, Value>),  // Objeto/diccionario
    Edge {                           // Arista de grafo
        from: String,
        to: String,
        directed: bool,
        properties: HashMap<String, Value>,
    },

    // Referencias mutables
    MutableRef(Rc<RefCell<Value>>),

    // Markers internos (nunca expuestos al usuario)
    TailCall(Vec<Value>),      // TCO: argumentos para próxima iteración
    EarlyReturn(Box<Value>),   // return: valor a retornar
}
```

### Decisiones de diseño

#### 1. **¿Por qué `Vector` Y `Tensor`?**

**`Vector<Value>`** - Heterogéneo, flexible:
```rust
// Puede contener cualquier tipo mezclado
let mixed = Value::Vector(vec![
    Value::Number(1.0),
    Value::String("hello"),
    Value::Boolean(true),
]);
```

**`Tensor`** - Homogéneo, optimizado:
```rust
// Solo números, almacenamiento contiguo
let matrix = Value::Tensor(RealTensor::matrix(2, 2, vec![
    1.0, 2.0,
    3.0, 4.0
])?);
```

**Ventajas**:
- Vector: Máxima flexibilidad para listas generales
- Tensor: Máximo rendimiento para operaciones numéricas
- El evaluador decide cuál usar según el contexto

#### 2. **Clone everywhere**

Todos los valores son `Clone` porque:
- **Closures**: Capturan el environment por valor
- **Semántica funcional**: Inmutabilidad por defecto
- **Simplicidad**: Evita lifetime complexity

**Trade-off**: Algunas copias innecesarias vs. código más simple.

#### 3. **MutableRef con Rc\<RefCell>**

Para variables mutables (`mut x = 10`):
```rust
MutableRef(Rc<RefCell<Value>>)
```

**Por qué**:
- `Rc`: Permite compartir la referencia (múltiples closures)
- `RefCell`: Permite mutabilidad interior (borrow checking en runtime)
- Mantiene semántica de Rust (checked borrows)

#### 4. **TailCall y EarlyReturn**

Markers internos para control de flujo:
```rust
// TCO: El evaluator detecta esto y hace loop en vez de recursión
TailCall(vec![arg1, arg2, ...])

// Early return: Se propaga hacia arriba hasta encontrar lambda boundary
EarlyReturn(Box::new(value))
```

**Importante**: Nunca deben escaparse al usuario. El evaluator los consume.

### Conversiones y helpers

```rust
impl Value {
    // Conversión a tensor real
    pub fn to_real_tensor(vec: &[Value]) -> Result<RealTensor, TypeError> {
        // Extrae números de un Vector y crea RealTensor
    }

    // Conversión a tensor complejo
    pub fn to_complex_tensor(vec: &[Value]) -> Result<ComplexTensor, TypeError> {
        // Extrae números/complejos y crea ComplexTensor
    }

    // Verificación de tipo
    pub fn is_numeric_vector(vec: &[Value]) -> bool {
        // true si todos son Number o Complex
    }

    // Dereferenciar MutableRef automáticamente
    pub fn deref(&self) -> Result<Value, String> {
        match self {
            Value::MutableRef(r) => Ok(r.borrow().clone()),
            _ => Ok(self.clone())
        }
    }
}
```

## 🔢 Complex - Números complejos

### Representación

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    pub re: f64,  // Parte real
    pub im: f64,  // Parte imaginaria
}
```

### Operaciones aritméticas

```rust
impl std::ops::Add for Complex {
    type Output = Complex;
    fn add(self, other: Complex) -> Complex {
        Complex {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}

// Similar para Sub, Mul, Div
```

**División de complejos**:
```
(a + bi) / (c + di) = [(ac + bd) + (bc - ad)i] / (c² + d²)
```

### Funciones especiales

```rust
impl Complex {
    // Constructores
    pub fn new(re: f64, im: f64) -> Self
    pub fn from_real(re: f64) -> Self
    pub fn from_polar(r: f64, theta: f64) -> Self

    // Propiedades
    pub fn magnitude(&self) -> f64       // |z| = √(re² + im²)
    pub fn argument(&self) -> f64        // arg(z) = atan2(im, re)
    pub fn conjugate(&self) -> Complex   // z* = re - im·i

    // Potencias
    pub fn pow(&self, n: f64) -> Complex         // z^n (n real)
    pub fn pow_complex(&self, w: &Complex) -> Complex  // z^w (general)

    // Funciones exponenciales
    pub fn exp(&self) -> Complex         // e^z
    pub fn ln(&self) -> Complex          // ln(z) (rama principal)

    // Trigonométricas
    pub fn sin(&self) -> Complex
    pub fn cos(&self) -> Complex
    // ... más funciones
}
```

**Fórmula de Euler**: `e^(iθ) = cos(θ) + i·sin(θ)`

## 🧮 Tensor System - Arquitectura

### Core types

```rust
pub struct RealTensor {
    data: Vec<f64>,      // Almacenamiento contiguo (row-major)
    shape: Vec<usize>,   // Dimensiones: [rows, cols, depth, ...]
}

pub struct ComplexTensor {
    data: Vec<Complex>,
    shape: Vec<usize>,
}
```

### Row-major ordering

Para una matriz 2x3:
```
┌───────┐
│ 1 2 3 │
│ 4 5 6 │
└───────┘

data = [1, 2, 3, 4, 5, 6]
shape = [2, 3]

Element [i, j] → data[i * cols + j]
```

### Broadcasting

Implementa reglas de NumPy:

```rust
// Scalar + Vector
[1, 2, 3] + 10 → [11, 12, 13]

// Vector + Matrix (expande el vector)
[1, 2] + [[10, 20],  →  [[11, 22],
           [30, 40]]      [31, 42]]

// Reglas:
// 1. Alinea shapes por la derecha
// 2. Dimensiones de tamaño 1 se expanden
// 3. Dimensiones incompatibles → error
```

### Operaciones matriciales

**Multiplicación matricial** (matmul):
```
(m × n) · (n × p) → (m × p)

C[i,j] = Σ(k) A[i,k] * B[k,j]
```

**Determinante** (Laplace expansion):
```rust
// Base case: 2x2
det([[a,b], [c,d]]) = ad - bc

// Recursive: expandir por primera fila
det(A) = Σ(j) (-1)^j · A[0,j] · det(Minor[0,j])
```

**Inversa** (Gauss-Jordan):
```
A · A⁻¹ = I

1. Aumenta [A | I]
2. Aplica eliminación Gaussiana
3. Resultado: [I | A⁻¹]
```

## 🔄 Environment - Gestión de scopes

### Estructura

```rust
pub struct Environment {
    scopes: Vec<HashMap<String, Value>>
}
```

**Stack de scopes**:
```
┌──────────────┐ ← Top (scope actual)
│ { y: 20 }    │
├──────────────┤
│ { x: 10 }    │
├──────────────┤
│ { prelude }  │ ← Base (funciones built-in)
└──────────────┘
```

### Operaciones principales

#### Define (declarar variable)
```rust
pub fn define(&mut self, name: String, value: Value) -> Result<(), String> {
    // Solo agrega al scope actual (top)
    if let Some(scope) = self.scopes.last_mut() {
        scope.insert(name, value);
        Ok(())
    } else {
        Err("No scope available")
    }
}
```

#### Get (leer variable)
```rust
pub fn get(&self, name: &str) -> Result<Value, String> {
    // Busca desde el top hacia abajo (shadowing)
    for scope in self.scopes.iter().rev() {
        if let Some(value) = scope.get(name) {
            return Ok(value.clone());
        }
    }
    Err(format!("Variable '{}' not found", name))
}
```

#### Set (modificar mutable)
```rust
pub fn set(&mut self, name: &str, value: Value) -> Result<(), String> {
    // Busca y modifica solo MutableRef
    for scope in self.scopes.iter_mut().rev() {
        if let Some(existing) = scope.get_mut(name) {
            match existing {
                Value::MutableRef(r) => {
                    *r.borrow_mut() = value;
                    return Ok(());
                }
                _ => return Err(format!("'{}' is not mutable", name))
            }
        }
    }
    Err(format!("Variable '{}' not found", name))
}
```

### Shadowing

```rust
env.define("x", Value::Number(10.0));  // Scope 0: x = 10

env.push_scope();
env.define("x", Value::Number(20.0));  // Scope 1: x = 20 (shadows)
assert_eq!(env.get("x"), 20.0);        // Lee el más reciente

env.pop_scope();
assert_eq!(env.get("x"), 10.0);        // Vuelve al original
```

## 🎭 Function - Representación

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Function {
    // Función nativa de Achronyme
    Builtin(String),  // "sin", "cos", "map", etc.

    // Lambda definida por usuario
    UserDefined {
        params: Vec<String>,      // ["x", "y"]
        body: Box<AstNode>,       // AST del cuerpo
        closure: Environment,     // Environment capturado
    }
}
```

### Closures

Las funciones user-defined **capturan** su environment:

```rust
let x = 10
let adder = y => x + y  // Captura 'x' del scope exterior
adder(5)  // = 15
```

**Implementación**:
```rust
Function::UserDefined {
    params: vec!["y"],
    body: AstNode::BinaryOp { ... },
    closure: env.clone(),  // ← Copia del environment actual
}
```

Cuando se llama `adder(5)`:
1. Se crea un nuevo scope
2. Se copia el `closure` como base
3. Se define `y = 5`
4. Se evalúa el `body`

## 🔌 LambdaEvaluator Trait

```rust
pub trait LambdaEvaluator {
    // Evaluar f(x) donde x es escalar
    fn eval_at(&mut self, func: &Function, x: f64) -> Result<f64, String>;

    // Evaluar f(point) donde point es vector
    fn eval_vec_at(&mut self, func: &Function, point: &[f64]) -> Result<f64, String>;

    // Evaluar f(x1, x2, ...) con múltiples argumentos
    fn eval_at_nd(&mut self, func: &Function, args: &[f64]) -> Result<f64, String>;
}
```

**Propósito**: Permite a `achronyme-solver` evaluar funciones sin tener que importar todo `achronyme-eval`.

**Implementación**: El `Evaluator` implementa este trait.

## 🧪 Testing patterns

### Test de conversiones

```rust
#[test]
fn test_vector_to_tensor() {
    let vec = vec![
        Value::Number(1.0),
        Value::Number(2.0),
        Value::Number(3.0),
    ];

    let tensor = Value::to_real_tensor(&vec).unwrap();
    assert_eq!(tensor.shape(), &[3]);
    assert_eq!(tensor.data(), &[1.0, 2.0, 3.0]);
}
```

### Test de operaciones complejas

```rust
#[test]
fn test_complex_multiplication() {
    let z1 = Complex::new(3.0, 4.0);  // 3+4i
    let z2 = Complex::new(1.0, 2.0);  // 1+2i
    let result = z1 * z2;              // = -5+10i

    assert!((result.re - (-5.0)).abs() < 1e-10);
    assert!((result.im - 10.0).abs() < 1e-10);
}
```

### Test de scopes

```rust
#[test]
fn test_environment_shadowing() {
    let mut env = Environment::new();

    env.define("x".to_string(), Value::Number(10.0)).unwrap();
    env.push_scope();
    env.define("x".to_string(), Value::Number(20.0)).unwrap();

    assert_eq!(env.get("x").unwrap(), Value::Number(20.0));

    env.pop_scope();
    assert_eq!(env.get("x").unwrap(), Value::Number(10.0));
}
```

## 🔧 Extender el sistema de tipos

### Agregar nueva variante a Value

1. **Agregar al enum**:
```rust
pub enum Value {
    // ... existentes
    DateTime(chrono::DateTime<Utc>),  // Nueva
}
```

2. **Implementar Clone y PartialEq** (si no derivan automáticamente)

3. **Agregar pattern matching** en evaluator

4. **Implementar Display** para pretty-printing

5. **Tests**

### Agregar nueva operación a tensores

1. **Definir en `tensor/arithmetic/real.rs`**:
```rust
impl RealTensor {
    pub fn my_operation(&self, other: &RealTensor) -> Result<RealTensor, String> {
        // Implementación
    }
}
```

2. **Definir en `tensor/arithmetic/complex.rs`** (versión compleja)

3. **Re-exportar en `tensor/mod.rs`**

4. **Agregar handler** en `achronyme-eval`

5. **Tests**

## 📚 Referencias

- [value.rs](value.rs) - Definición de Value
- [complex.rs](complex.rs) - Implementación de Complex
- [tensor/](tensor/) - Sistema completo de tensores
- [environment.rs](environment.rs) - Gestión de scopes

## 🎯 Notas de rendimiento

### Cloning

- `Value::Number`, `Value::Boolean`, `Value::Complex` son baratos (Copy)
- `Value::Vector`, `Value::Tensor` clonan todo el contenido (costoso)
- `Value::Function` con closure grande puede ser costoso
- **Optimización futura**: Usar `Rc` para tipos grandes

### MutableRef

- El `RefCell` hace borrow checking en **runtime**
- Panic si se violan reglas de borrowing
- **Trade-off**: Seguridad vs. overhead runtime

### Tensor operations

- Operaciones in-place cuando sea posible
- Broadcasting evita copias innecesarias
- **Bottleneck**: Multiplicación matricial (O(n³))
