# Achronyme Eval

**Evaluador de AST: Convierte AST → Value ejecutando operaciones y funciones.**

## 🎯 Responsabilidad

Este crate es el **motor de evaluación** que toma el AST producido por el parser y lo ejecuta para producir valores:

```
┌──────────────┐
│    Parser    │ → Produce: Vec<AstNode>
└──────────────┘
       ↓
┌──────────────┐
│  Evaluator   │ → Consume: AstNode, Produce: Value
└──────────────┘
       ↓
┌──────────────┐
│ REPL / CLI   │ → Muestra resultado al usuario
└──────────────┘
```

### Flujo principal:
1. **Parser** genera AST (árbol sintáctico abstracto)
2. **Evaluator** recorre el AST usando handlers
3. **Handlers** procesan cada tipo de nodo (literales, operaciones, funciones)
4. **Result** se devuelve como `Value` al usuario

## 📦 Dependencias

### Internas:
- **`achronyme-types`** - `Value`, `Environment`, `Function`, `Complex`, `Tensor`
- **`achronyme-parser`** - `AstNode`, `parse()` para convertir código → AST
- **`achronyme-solver`** - Optimización lineal, resolución numérica
- **`achronyme-dsp`** - Procesamiento de señales (FFT, filtros)
- **`achronyme-linalg`** - Álgebra lineal (matrices, vectores)
- **`achronyme-numerical`** - Métodos numéricos (integración, derivación)
- **`achronyme-env`** - Gestión de entorno

### Externas:
- Ninguna directa (todas las dependencias son internas)

## 🔌 Usado por

- **`achronyme-repl`** - REPL interactivo
- **`achronyme-cli`** - Ejecución de scripts `.soc`
- **`achronyme`** - Crate principal que re-exporta el evaluador

## 🏗️ Arquitectura

```
achronyme-eval/
├── src/
│   ├── lib.rs                # Re-exports públicos
│   ├── evaluator/            # Motor de evaluación
│   │   ├── mod.rs           # Struct Evaluator, constructor
│   │   ├── dispatcher.rs    # eval_str(), evaluate() - dispatcher principal
│   │   ├── lambda_eval.rs   # apply_lambda(), LambdaEvaluator trait impl
│   │   ├── state.rs         # Getters: environment(), functions(), etc.
│   │   └── modules.rs       # load_user_module() - sistema de módulos
│   │
│   ├── handlers/             # Handlers especializados por tipo de nodo
│   │   ├── mod.rs
│   │   ├── literals.rs      # Number, String, Array, Record, Edge
│   │   ├── variables.rs     # VariableDecl, VariableRef, MutableDecl
│   │   ├── assignment.rs    # Assignment (x = y)
│   │   ├── control_flow.rs  # If, While, Piecewise
│   │   ├── functions.rs     # Lambda, apply_lambda (TCO)
│   │   ├── function_call.rs # Dispatcher de llamadas de función
│   │   ├── hof.rs           # map, filter, reduce, pipe, any, all
│   │   ├── numerical.rs     # diff, integral, solve, newton
│   │   ├── optimization.rs  # simplex, linprog, dual_simplex
│   │   ├── debug.rs         # describe()
│   │   ├── unary_ops.rs     # Negación, NOT
│   │   ├── binary_ops/      # Operaciones binarias (refactorizado)
│   │   │   ├── mod.rs       # Dispatcher apply()
│   │   │   ├── arithmetic.rs # +, -, *, /, ^, %
│   │   │   ├── comparison.rs # >, <, >=, <=, ==, !=
│   │   │   ├── logical.rs   # AND, OR
│   │   │   └── utils.rs     # Helpers
│   │   └── indexing/        # Indexación y slicing
│   │       ├── mod.rs
│   │       └── ...
│   │
│   ├── function_modules/     # Implementación de built-in functions
│   │   ├── mod.rs
│   │   ├── array.rs         # len, push, concat, slice, reverse, etc.
│   │   ├── vector.rs        # dot, cross, norm, normalize
│   │   ├── matrix.rs        # det, inv, transpose, trace
│   │   ├── trig.rs          # sin, cos, tan, asin, acos, atan
│   │   ├── exponential.rs   # exp, ln, log, log10
│   │   ├── complex.rs       # real, imag, conjugate, magnitude, phase
│   │   ├── stats.rs         # mean, median, variance, stdev, sum
│   │   ├── rounding.rs      # round, floor, ceil, trunc, abs
│   │   ├── strings.rs       # upper, lower, split, join, charAt, etc.
│   │   ├── records.rs       # keys, values, hasKey, merge
│   │   ├── dsp.rs           # fft, ifft, conv, xcorr, etc.
│   │   ├── io.rs            # print, println, readFile, writeFile
│   │   ├── utils.rs         # type, equals, clone
│   │   ├── common.rs        # Helpers comunes
│   │   └── graphs/          # Funciones de grafos
│   │       ├── mod.rs
│   │       ├── traversal.rs # bfs, dfs
│   │       ├── shortest_path.rs # dijkstra, bellman_ford, floyd_warshall
│   │       ├── mst.rs       # kruskal, prim
│   │       ├── connectivity.rs # is_connected, connected_components
│   │       ├── cycles.rs    # has_cycle, find_cycles
│   │       ├── topological.rs # topological_sort
│   │       ├── network.rs   # max_flow, min_cut
│   │       ├── helpers.rs   # Conversión Edge → Graph
│   │       └── pert/        # PERT/CPM para gestión de proyectos
│   │           ├── mod.rs
│   │           ├── project.rs
│   │           ├── critical_path.rs
│   │           ├── cpm.rs
│   │           ├── probabilistic.rs
│   │           ├── statistics.rs
│   │           ├── validation.rs
│   │           └── state_detection.rs
│   │
│   ├── modules/              # Sistema de módulos (import/export)
│   │   ├── mod.rs           # Module, ModuleRegistry structs
│   │   └── builtin_registry.rs # create_builtin_registry()
│   │
│   ├── tco/                  # Tail Call Optimization
│   │   ├── mod.rs           # is_tail_position(), is_tail_recursive_function()
│   │   └── tests.rs         # Tests de TCO
│   │
│   ├── constants.rs          # ConstantsRegistry (pi, e, phi, tau, etc.)
│   └── functions.rs          # FunctionRegistry (backward compat)
│
└── tests/
    └── integration_tests.rs
```

## 🧩 Conceptos clave

### 1. Handler System (Sistema de handlers)

El evaluador usa un **patrón de handlers especializados** para procesar cada tipo de nodo AST:

```rust
// Dispatcher principal en evaluator/dispatcher.rs
pub fn evaluate(&mut self, node: &AstNode) -> Result<Value, String> {
    match node {
        AstNode::Number(n) => handlers::literals::evaluate_number(*n),
        AstNode::BinaryOp { op, left, right } => {
            let left_val = self.evaluate(left)?;
            let right_val = self.evaluate(right)?;
            handlers::binary_ops::apply(op, left_val, right_val)
        }
        AstNode::FunctionCall { name, args } => {
            handlers::function_call::dispatch(self, name, args)
        }
        // ... 20+ tipos de nodos más
    }
}
```

**Ventajas**:
- ✅ Separación de responsabilidades (cada handler = 1 tipo de nodo)
- ✅ Fácil de testear (cada handler se prueba independientemente)
- ✅ Fácil de extender (agregar nuevo handler = agregar nuevo tipo de nodo)

### 2. Post-Order Traversal (Recorrido post-orden)

El evaluador recorre el AST en **post-orden** (hijos antes que padres):

```
       +
      / \
     2   *
        / \
       3   4

Orden de evaluación:
  1. eval(2) → 2
  2. eval(3) → 3
  3. eval(4) → 4
  4. eval(3*4) → 12
  5. eval(2+12) → 14
```

Esto garantiza que los operandos estén evaluados antes de aplicar la operación.

### 3. Tail Call Optimization (TCO)

El evaluador implementa **TCO** para permitir recursión infinita en funciones tail-recursive:

```javascript
// Factorial tail-recursive - OPTIMIZADO con TCO
let factorial = (n, acc) => if(n <= 1, acc, rec(n-1, acc*n))
factorial(100000, 1) // ✅ No stack overflow!

// Factorial NO tail-recursive - SIN TCO
let factorial2 = (n) => if(n <= 1, 1, n * rec(n-1))
factorial2(100000) // ❌ Stack overflow (sin TCO esto causaría error)
```

**Implementación**:
1. `tco::is_tail_recursive_function(body)` detecta si la función es tail-recursive
2. Si sí, `apply_lambda_tco()` usa un **loop iterativo** en lugar de recursión
3. Las llamadas a `rec` en tail position retornan `Value::TailCall(args)`
4. El loop actualiza los argumentos y continúa (en vez de apilar frames)

**Beneficios**:
- ✅ Recursión ilimitada para patrones tail-recursive
- ✅ Uso constante de memoria (O(1) en vez de O(n))
- ✅ Transparente para el usuario (optimización automática)

### 4. Closures y Environment Capture

Las lambdas capturan su entorno de definición usando **closures**:

```javascript
let x = 10
let f = y => x + y  // Captura x=10
let x = 20          // Cambiar x no afecta a f
f(5)                // → 15 (usa el x=10 capturado)
```

**Implementación**:
```rust
// En handlers/functions.rs
pub fn evaluate_lambda(...) -> Result<Value, String> {
    // OPTIMIZACIÓN: Rc<RefCell<Environment>> en vez de clonar todo
    let closure_env = evaluator.environment().to_rc();

    let function = Function::new_with_env(
        params.to_vec(),
        body.clone(),
        closure_env  // ← Captura el entorno actual
    );

    Ok(Value::Function(function))
}
```

**Optimización reciente**:
- **Antes**: `snapshot()` clonaba TODAS las variables (O(n), costoso)
- **Ahora**: `to_rc()` solo incrementa un contador (O(1), barato)
- **Mejora**: 100x más rápido en entornos grandes (REPLs con muchas variables)

### 5. Module System (Sistema de módulos)

El evaluador soporta un **sistema de módulos** para organizar funciones:

```javascript
// Prelude: ~39 funciones siempre disponibles (sin import)
sin(pi/2)  // ✅ Funciona sin import

// Módulos: Requieren import explícito
import { mean, median } from "stats"
mean([1, 2, 3, 4, 5])  // → 3

// Alias
import { mean as avg } from "stats"
avg([1, 2, 3])  // → 2
```

**Estructura**:
```rust
ModuleRegistry {
    prelude: HashMap<String, (BuiltinFunction, i32)>,  // Siempre disponible
    modules: HashMap<String, Module>,                   // Requieren import
}
```

**Módulos disponibles**: `math`, `stats`, `dsp`, `linalg`, `graphs`, `io`, etc.

### 6. Binary Operations Refactor

Las operaciones binarias fueron **refactorizadas modularmente**:

```
handlers/binary_ops/
├── mod.rs           # Dispatcher apply(op, left, right)
├── arithmetic.rs    # +, -, *, /, ^, %
├── comparison.rs    # >, <, >=, <=, ==, !=
├── logical.rs       # AND, OR
└── utils.rs         # promote_numeric(), coerce_to_boolean()
```

**Ventajas**:
- ✅ 1 archivo por categoría de operación
- ✅ Reutilización de lógica (type promotion, coercion)
- ✅ Más fácil de testear y mantener

## 🚀 Uso

### Evaluación básica

```rust
use achronyme_eval::Evaluator;

let mut evaluator = Evaluator::new();

// Evaluar expresiones
let result = evaluator.eval_str("2 + 3 * 4")?;
assert_eq!(result, Value::Number(14.0));

// Variables
evaluator.eval_str("let x = 10")?;
let result = evaluator.eval_str("x * 2")?;
assert_eq!(result, Value::Number(20.0));

// Funciones
evaluator.eval_str("let f = x => x * x")?;
let result = evaluator.eval_str("f(5)")?;
assert_eq!(result, Value::Number(25.0));
```

### Evaluación de AST directamente

```rust
use achronyme_parser::parse;
use achronyme_eval::Evaluator;

let ast = parse("2 + 3")?;
let mut evaluator = Evaluator::new();

for node in &ast {
    let result = evaluator.evaluate(node)?;
    println!("Result: {:?}", result);
}
```

### Acceso al entorno

```rust
let mut evaluator = Evaluator::new();
evaluator.eval_str("let x = 42")?;

// Leer variable directamente
let x_value = evaluator.environment().get("x")?;
assert_eq!(x_value, Value::Number(42.0));

// Definir variable desde Rust
evaluator.environment_mut().define(
    "rust_var".to_string(),
    Value::String("Hello from Rust".to_string())
)?;
```

### Higher-Order Functions

```rust
let mut evaluator = Evaluator::new();

// map
let result = evaluator.eval_str("map(x => x * 2, [1, 2, 3])")?;
// → [2, 4, 6]

// filter
let result = evaluator.eval_str("filter(x => x > 2, [1, 2, 3, 4])")?;
// → [3, 4]

// reduce
let result = evaluator.eval_str("reduce((acc, x) => acc + x, 0, [1, 2, 3, 4])")?;
// → 10

// pipe
let result = evaluator.eval_str("pipe(x => x * 2, x => x + 1)(5)")?;
// → 11
```

### Módulos

```rust
let mut evaluator = Evaluator::new();

// Importar funciones
evaluator.eval_str("import { mean, median } from \"stats\"")?;
let result = evaluator.eval_str("mean([1, 2, 3, 4, 5])")?;
// → 3

// Importar con alias
evaluator.eval_str("import { variance as var } from \"stats\"")?;
let result = evaluator.eval_str("var([1, 2, 3, 4, 5])")?;
// → 2.5
```

## 🧪 Testing

### Ejecutar tests

```bash
# Todos los tests del evaluador
cargo test --package achronyme-eval

# Tests específicos de handlers
cargo test --package achronyme-eval handlers

# Tests de TCO
cargo test --package achronyme-eval tco

# Tests de módulos
cargo test --package achronyme-eval modules
```

### Estructura de tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_arithmetic() {
        let mut eval = Evaluator::new();
        assert_eq!(
            eval.eval_str("2 + 3").unwrap(),
            Value::Number(5.0)
        );
    }

    #[test]
    fn test_lambda_closure() {
        let mut eval = Evaluator::new();
        eval.eval_str("let x = 10").unwrap();
        eval.eval_str("let f = y => x + y").unwrap();
        assert_eq!(
            eval.eval_str("f(5)").unwrap(),
            Value::Number(15.0)
        );
    }
}
```

## 📖 Documentación interna

Para entender la implementación en detalle:
- [src/README.md](src/README.md) - Arquitectura técnica interna
- [src/handlers/README.md](src/handlers/README.md) - Sistema de handlers detallado

## 🔧 Características avanzadas

### 1. Recursión tail-call optimizada
Ver sección TCO arriba.

### 2. Mutable references
```javascript
mut x = 10
x = 20  // Modifica x
x       // → 20
```

### 3. Records con métodos
```javascript
let person = {
    name: "Alice",
    greet: () => "Hello, " + self.name
}
person.greet()  // → "Hello, Alice"
```

### 4. Early return
```javascript
let f = x => do {
    if(x < 0, return 0, false)
    x * x
}
f(-5)  // → 0 (early return)
f(5)   // → 25
```

### 5. Edges para grafos
```javascript
let graph = [
    "A" -> "B" { weight: 5 },
    "B" -> "C" { weight: 3 },
    "A" -- "C" { weight: 8 }  // No dirigido
]

import { dijkstra } from "graphs"
dijkstra(graph, "A", "C")
```

## 📊 Estadísticas

- **Líneas de código**: ~8,000 LOC
- **Handlers**: 15+ handlers especializados
- **Built-in functions**: 150+ funciones
- **Módulos**: 10+ módulos (math, stats, dsp, graphs, etc.)
- **Archivos fuente**: 76 archivos .rs

## 🎯 Principios de diseño

1. **Separation of Concerns** - Cada handler maneja 1 tipo de operación
2. **Post-Order Traversal** - Evaluar operandos antes que operadores
3. **Tail Call Optimization** - Permitir recursión infinita cuando sea posible
4. **Closure Capture Eficiente** - Rc<RefCell> en vez de clonación profunda
5. **Module System** - Organizar funciones en namespaces
6. **Type Promotion** - Number → Complex automático cuando sea necesario

## 🔗 Ver también

- [achronyme-parser](../achronyme-parser/README.md) - Genera el AST que este evaluador consume
- [achronyme-types](../achronyme-types/README.md) - Define `Value`, `Environment`, `Function`
- [achronyme-solver](../achronyme-solver/README.md) - Usa el evaluador para optimización numérica
