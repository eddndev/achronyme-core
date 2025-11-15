# Handlers - Sistema de evaluación modular

**Handlers especializados para procesar cada tipo de nodo AST.**

## 🎯 Arquitectura del sistema de handlers

El sistema de handlers sigue un **patrón de diseño modular** donde cada tipo de operación o nodo AST tiene su propio handler especializado:

```
                    ┌─────────────────────────┐
                    │   DISPATCHER            │
                    │  (dispatcher.rs)        │
                    │  evaluate(&AstNode)     │
                    └─────────────────────────┘
                              │
              ┌───────────────┴───────────────┐
              │    Pattern match on node      │
              │         type                  │
              └───────────────┬───────────────┘
                              │
        ┌─────────────────────┼─────────────────────┐
        │                     │                     │
        ▼                     ▼                     ▼
┌───────────────┐    ┌───────────────┐    ┌───────────────┐
│   LITERALS    │    │  OPERATIONS   │    │   FUNCTIONS   │
│  (literals.rs)│    │ (binary_ops/, │    │ (functions.rs,│
│               │    │  unary_ops.rs)│    │function_call.rs)│
└───────────────┘    └───────────────┘    └───────────────┘
        │                     │                     │
        ▼                     ▼                     ▼
┌───────────────┐    ┌───────────────┐    ┌───────────────┐
│   VARIABLES   │    │ CONTROL FLOW  │    │      HOF      │
│ (variables.rs,│    │(control_flow.rs)│  │   (hof.rs)    │
│assignment.rs) │    │               │    │               │
└───────────────┘    └───────────────┘    └───────────────┘
        │                     │                     │
        ▼                     ▼                     ▼
┌───────────────┐    ┌───────────────┐    ┌───────────────┐
│   INDEXING    │    │  NUMERICAL    │    │ OPTIMIZATION  │
│ (indexing/)   │    │ (numerical.rs)│    │(optimization.rs)│
└───────────────┘    └───────────────┘    └───────────────┘
```

## 📁 Estructura de handlers

```
handlers/
├── mod.rs              # Exports públicos
│
├── literals.rs         # Number, String, Array, Record, Edge, Complex
├── variables.rs        # VariableDecl, VariableRef, MutableDecl
├── assignment.rs       # Assignment (x = y, array[i] = value)
├── control_flow.rs     # If, While, Piecewise
├── functions.rs        # Lambda, apply_lambda, TCO
├── function_call.rs    # Dispatcher de llamadas de función
│
├── binary_ops/         # Operaciones binarias (refactorizado)
│   ├── mod.rs         # Dispatcher apply()
│   ├── arithmetic.rs  # +, -, *, /, ^, %
│   ├── comparison.rs  # >, <, >=, <=, ==, !=
│   ├── logical.rs     # AND, OR
│   └── utils.rs       # Helpers (promote_numeric, coerce_to_boolean)
│
├── unary_ops.rs        # -, NOT
│
├── indexing/           # Indexación y slicing
│   └── mod.rs         # array[i], matrix[i,j], tensor[...]
│
├── hof.rs             # Higher-Order Functions (map, filter, reduce, pipe, any, all, find)
├── numerical.rs       # diff, integral, solve, newton, gradient
├── optimization.rs    # simplex, linprog, dual_simplex
└── debug.rs           # describe()
```

## 📚 Handlers detallados

### 1. `literals.rs` - Literales

**Responsabilidad**: Convertir literales del AST a `Value`.

#### Funciones públicas:
```rust
pub fn evaluate_number(n: f64) -> Result<Value, String>
pub fn evaluate_boolean(b: bool) -> Result<Value, String>
pub fn evaluate_string(s: &str) -> Result<Value, String>
pub fn evaluate_complex(re: f64, im: f64) -> Result<Value, String>
pub fn evaluate_array(evaluator: &mut Evaluator, elements: &[ArrayElement]) -> Result<Value, String>
pub fn evaluate_record(evaluator: &mut Evaluator, fields: &[RecordFieldOrSpread]) -> Result<Value, String>
pub fn evaluate_edge(evaluator: &mut Evaluator, from: &str, to: &str, directed: bool, metadata: &Option<Box<AstNode>>) -> Result<Value, String>
```

#### `evaluate_number(n: f64)`
Convierte un número literal a `Value::Number`.

**Ejemplo**:
```javascript
42       // → Value::Number(42.0)
3.14159  // → Value::Number(3.14159)
```

#### `evaluate_boolean(b: bool)`
Convierte un booleano literal a `Value::Boolean`.

**Ejemplo**:
```javascript
true   // → Value::Boolean(true)
false  // → Value::Boolean(false)
```

#### `evaluate_string(s: &str)`
Convierte un string literal a `Value::String`.

**Ejemplo**:
```javascript
"hello"  // → Value::String("hello".to_string())
```

#### `evaluate_complex(re: f64, im: f64)`
Convierte un complejo literal a `Value::Complex`.

**Ejemplo**:
```javascript
3+4i   // → Value::Complex(Complex { re: 3.0, im: 4.0 })
2i     // → Value::Complex(Complex { re: 0.0, im: 2.0 })
```

#### `evaluate_array(evaluator, elements)`
Evalúa array literals con soporte para:
- ✅ Spread syntax: `[1, ...vec, 2]`
- ✅ Auto-conversión a Tensor si todos los elementos son vectores numéricos del mismo tamaño
- ✅ Type promotion: Number → Complex si hay complejos

**Ejemplos**:
```javascript
[1, 2, 3]           // → Value::Vector([Number(1.0), Number(2.0), Number(3.0)])
[[1,2], [3,4]]      // → Value::Tensor (2x2 matrix)
[1, ...vec, 2]      // → Value::Vector con spread
[1, 2+3i]           // → Value::Vector([Complex(1+0i), Complex(2+3i)])
```

**Algoritmo**:
1. Evaluar todos los elementos, expandiendo spreads
2. Si todos son tensores del mismo shape → Combinar en tensor de mayor dimensión
3. Si todos son vectores numéricos del mismo tamaño → Crear matriz 2D
4. Si hay complejos → Promover todos a Complex
5. Sino → Retornar Vector genérico

#### `evaluate_record(evaluator, fields)`
Evalúa record literals con soporte para:
- ✅ Spread syntax: `{ a: 1, ...other, b: 2 }`
- ✅ Campos mutables: `{ mut x: 10 }`

**Ejemplos**:
```javascript
{ name: "Alice", age: 30 }
{ a: 1, ...other, b: 2 }  // Spread
{ mut x: 10, y: 20 }      // Mutable field
```

#### `evaluate_edge(evaluator, from, to, directed, metadata)`
Evalúa edge literals para grafos.

**Ejemplos**:
```javascript
"A" -> "B"                      // Dirigido sin metadata
"A" -- "B"                      // No dirigido sin metadata
"A" -> "B" { weight: 5 }        // Con metadata
```

**Retorna**: `Value::Edge { from, to, directed, properties }`

---

### 2. `variables.rs` - Variables

**Responsabilidad**: Declaración y lectura de variables.

#### Funciones públicas:
```rust
pub fn evaluate_declaration(evaluator: &mut Evaluator, name: &str, initializer: &AstNode) -> Result<Value, String>
pub fn evaluate_reference(evaluator: &Evaluator, name: &str) -> Result<Value, String>
pub fn evaluate_mutable_declaration(evaluator: &mut Evaluator, name: &str, initializer: &AstNode) -> Result<Value, String>
```

#### `evaluate_declaration(evaluator, name, initializer)`
Declara una variable **inmutable**.

**Ejemplo**:
```javascript
let x = 10
let y = x * 2
```

**Flujo**:
1. Evaluar `initializer` → `Value`
2. `evaluator.environment_mut().define(name, value)`
3. Retornar el valor

#### `evaluate_reference(evaluator, name)`
Lee el valor de una variable.

**Ejemplo**:
```javascript
x  // Lee el valor de x
```

**Flujo**:
1. `evaluator.environment().get(name)`
2. Si es `MutableRef`, auto-deref
3. Retornar el valor

#### `evaluate_mutable_declaration(evaluator, name, initializer)`
Declara una variable **mutable**.

**Ejemplo**:
```javascript
mut x = 10
```

**Flujo**:
1. Evaluar `initializer` → `Value`
2. Envolver en `Value::MutableRef(Rc::new(RefCell::new(value)))`
3. `evaluator.environment_mut().define(name, mutable_value)`
4. Retornar el valor (sin wrapper para el usuario)

---

### 3. `assignment.rs` - Asignaciones

**Responsabilidad**: Actualizar variables mutables.

#### Funciones públicas:
```rust
pub fn evaluate_assignment(evaluator: &mut Evaluator, target: &AstNode, value: &AstNode) -> Result<Value, String>
```

#### `evaluate_assignment(evaluator, target, value)`
Actualiza una variable mutable.

**Ejemplos**:
```javascript
mut x = 10
x = 20          // Simple assignment

mut arr = [1, 2, 3]
arr[0] = 10     // Indexed assignment

mut rec = { x: 10 }
rec.x = 20      // Field assignment
```

**Flujo**:
1. Evaluar `value` → `new_value`
2. Match en `target`:
   - `VariableRef(name)` → `env.set(name, new_value)`
   - `IndexAccess { object, indices }` → Actualizar elemento
   - `FieldAccess { record, field }` → Actualizar campo
3. Retornar `new_value`

**Validación**: Solo se pueden asignar variables mutables (envueltas en `MutableRef`).

---

### 4. `control_flow.rs` - Flujo de control

**Responsabilidad**: Condicionales y loops.

#### Funciones públicas:
```rust
pub fn evaluate_if(evaluator: &mut Evaluator, condition: &AstNode, then_expr: &AstNode, else_expr: &AstNode) -> Result<Value, String>
pub fn evaluate_while(evaluator: &mut Evaluator, condition: &AstNode, body: &AstNode) -> Result<Value, String>
pub fn evaluate_piecewise(evaluator: &mut Evaluator, cases: &[(AstNode, AstNode)], default: &Option<Box<AstNode>>) -> Result<Value, String>
```

#### `evaluate_if(evaluator, condition, then_expr, else_expr)`
Condicional ternario.

**Ejemplo**:
```javascript
if(x > 0, "positive", "negative")
```

**Flujo**:
1. Evaluar `condition` → `cond_value`
2. Convertir a Boolean
3. Si true → evaluar `then_expr`
4. Si false → evaluar `else_expr`
5. Retornar resultado

#### `evaluate_while(evaluator, condition, body)`
Loop mientras condición sea true.

**Ejemplo**:
```javascript
mut i = 0
while(i < 10, do { i = i + 1 })
```

**Flujo**:
1. Loop:
   - Evaluar `condition` → `cond_value`
   - Si false → break
   - Evaluar `body`
   - Continuar loop
2. Retornar último valor de body (o Boolean(false) si no hubo iteraciones)

**Early return**: Si `body` retorna `EarlyReturn`, propagar inmediatamente.

#### `evaluate_piecewise(evaluator, cases, default)`
Pattern matching de casos.

**Ejemplo**:
```javascript
piecewise(
    (x < 0, -1),
    (x == 0, 0),
    (x > 0, 1)
)
```

**Flujo**:
1. Para cada `(condition, result)` en `cases`:
   - Evaluar `condition`
   - Si true → retornar `result`
2. Si ningún caso match → evaluar `default` (o error si no hay default)

---

### 5. `functions.rs` - Funciones y lambdas

**Responsabilidad**: Crear y aplicar funciones, TCO.

#### Funciones públicas:
```rust
pub fn evaluate_lambda(evaluator: &Evaluator, params: &[String], body: &AstNode) -> Result<Value, String>
pub fn apply_lambda(evaluator: &mut Evaluator, function: &Function, args: Vec<Value>) -> Result<Value, String>
pub fn eval_lambda_at(evaluator: &mut Evaluator, func: &Function, x: f64) -> Result<f64, String>
```

#### `evaluate_lambda(evaluator, params, body)`
Crea una lambda con closure.

**Ejemplo**:
```javascript
x => x * 2
(x, y) => x + y
```

**Flujo**:
1. Capturar entorno: `closure_env = evaluator.environment().to_rc()`
2. Crear función: `Function::new_with_env(params, body, closure_env)`
3. Retornar `Value::Function(function)`

**Optimización**: Usa `Rc<RefCell<Environment>>` en vez de clonar todo (100x más rápido).

#### `apply_lambda(evaluator, function, args)`
Aplica una función a argumentos.

**Flujo**:
1. Verificar arity: `args.len() == params.len()`
2. **TCO Check**: `tco::is_tail_recursive_function(body)`?
   - Si sí → `apply_lambda_tco(evaluator, function, args)`
   - Si no → `apply_lambda_regular(evaluator, params, body, closure_env, args)`

#### `apply_lambda_regular(evaluator, params, body, closure_env, args)`
Aplicación regular (sin TCO).

**Flujo**:
1. Guardar entorno actual
2. Restaurar entorno de closure
3. Inyectar `rec` (self-reference para recursión)
4. Inyectar `self` (si está disponible, para métodos)
5. Push nuevo scope para parámetros
6. Bind parámetros a argumentos
7. Evaluar `body`
8. Pop scope
9. Restaurar entorno original
10. Si `EarlyReturn` → unwrap, sino retornar resultado

#### `apply_lambda_tco(evaluator, function, args)`
Aplicación con **Tail Call Optimization**.

**Flujo**:
1. Guardar entorno original
2. Restaurar entorno de closure
3. Inyectar `rec` y `self`
4. **Activar TCO mode**: `set_tco_mode(true)`
5. **TCO Loop**:
   ```rust
   loop {
       // Push scope
       // Bind parameters to args
       let value = evaluate(body)?;
       // Pop scope

       match value {
           Value::TailCall(new_args) => {
               // Tail call! Actualizar args y continuar
               args = new_args;
               continue;  // NO recurre!
           }
           Value::EarlyReturn(value) => break Ok(*value),
           other => break Ok(other),  // Caso base
       }
   }
   ```
6. **Desactivar TCO mode**: `set_tco_mode(false)`
7. Restaurar entorno original
8. Retornar resultado

**Beneficio**: Recursión infinita sin stack overflow.

**Ejemplo**:
```javascript
// Sin TCO: Stack overflow después de ~10,000 llamadas
let factorial = n => if(n <= 1, 1, n * rec(n-1))

// Con TCO: Puede manejar 100,000+ iteraciones
let factorial_tco = (n, acc) => if(n <= 1, acc, rec(n-1, acc*n))
```

---

### 6. `function_call.rs` - Dispatcher de llamadas

**Responsabilidad**: Resolver y despachar llamadas de función.

#### Función pública:
```rust
pub fn dispatch(evaluator: &mut Evaluator, name: &str, args: &[AstNode]) -> Result<Value, String>
```

#### `dispatch(evaluator, name, args)`
Dispatcher central de llamadas de función.

**Orden de resolución**:
1. **Field access**: `record.method()` → Inyectar `self`
2. **Constants** (sin args): `pi`, `e`, `tau`, `phi`
3. **Variables** (lambdas almacenadas): `let f = x => x*2; f(5)`
4. **Higher-Order Functions**: `map`, `filter`, `reduce`, `pipe`, `any`, `all`, `find`, `findIndex`, `count`
5. **Numerical functions**: `diff`, `integral`, `solve`, `newton`, `gradient`
6. **Debug functions**: `describe`
7. **Optimization**: `simplex`, `linprog`, `dual_simplex`, etc.
8. **Module system**:
   - Prelude (siempre disponible)
   - Imported modules
   - ~~Global fallback~~ (eliminado)
9. **FunctionRegistry** (backward compat)

**Ejemplo - Field access**:
```javascript
let person = {
    name: "Alice",
    greet: () => "Hello, " + self.name
}
person.greet()  // dispatch() inyecta 'self'
```

**Flujo**:
1. Detectar `name.contains('.')` → Field access
2. Navegar por campos anidados
3. Si el valor final es una función:
   - Push scope
   - Inyectar `self` (el record padre)
   - Evaluar argumentos
   - Aplicar lambda
   - Pop scope
4. Retornar resultado

---

### 7. `binary_ops/` - Operaciones binarias (refactorizado)

**Responsabilidad**: Aplicar operaciones binarias (+, -, *, /, ^, %, >, <, ==, AND, OR).

#### Estructura modular:
```
binary_ops/
├── mod.rs           # Dispatcher apply()
├── arithmetic.rs    # +, -, *, /, ^, %
├── comparison.rs    # >, <, >=, <=, ==, !=
├── logical.rs       # AND, OR
└── utils.rs         # Helpers
```

#### `mod.rs` - Dispatcher
```rust
pub fn apply(op: &BinaryOp, left: Value, right: Value) -> Result<Value, String>
```

**Flujo**:
```rust
match op {
    BinaryOp::Add => arithmetic::apply_add(left, right),
    BinaryOp::Subtract => arithmetic::apply_subtract(left, right),
    BinaryOp::Multiply => arithmetic::apply_multiply(left, right),
    BinaryOp::Divide => arithmetic::apply_divide(left, right),
    BinaryOp::Power => arithmetic::apply_power(left, right),
    BinaryOp::Modulo => arithmetic::apply_modulo(left, right),
    BinaryOp::Gt => comparison::apply_gt(left, right),
    BinaryOp::Lt => comparison::apply_lt(left, right),
    BinaryOp::Gte => comparison::apply_gte(left, right),
    BinaryOp::Lte => comparison::apply_lte(left, right),
    BinaryOp::Eq => comparison::apply_eq(left, right),
    BinaryOp::Neq => comparison::apply_neq(left, right),
    BinaryOp::And => logical::apply_and(left, right),
    BinaryOp::Or => logical::apply_or(left, right),
}
```

#### `arithmetic.rs` - Operaciones aritméticas

**Funciones**:
- `apply_add(left, right)` - Suma (+)
- `apply_subtract(left, right)` - Resta (-)
- `apply_multiply(left, right)` - Multiplicación (*)
- `apply_divide(left, right)` - División (/)
- `apply_power(left, right)` - Potencia (^)
- `apply_modulo(left, right)` - Módulo (%)

**Soporte**:
- ✅ Number: `2 + 3` → `5`
- ✅ Complex: `(2+3i) + (1+1i)` → `3+4i`
- ✅ String (solo `+`): `"hello" + " world"` → `"hello world"`
- ✅ Vector (element-wise): `[1,2] + [3,4]` → `[4,6]`
- ✅ Tensor (element-wise + broadcasting): `[[1,2],[3,4]] + 10` → `[[11,12],[13,14]]`

**Type promotion**: Number + Complex → Complex
```javascript
2 + 3+4i  // → 5+4i (2 se promueve a 2+0i)
```

**Broadcasting** (estilo NumPy):
```javascript
[[1,2],[3,4]] + 10  // → [[11,12],[13,14]]
```

#### `comparison.rs` - Operaciones de comparación

**Funciones**:
- `apply_gt(left, right)` - Mayor que (>)
- `apply_lt(left, right)` - Menor que (<)
- `apply_gte(left, right)` - Mayor o igual (>=)
- `apply_lte(left, right)` - Menor o igual (<=)
- `apply_eq(left, right)` - Igual (==)
- `apply_neq(left, right)` - No igual (!=)

**Soporte**:
- ✅ Number: `2 > 3` → `false`
- ✅ Complex (por magnitud): `|2+3i| > |1+1i|` → `true`
- ✅ String (lexicográfico): `"abc" < "def"` → `true`
- ✅ Boolean: `true == true` → `true`

**Retorna**: `Value::Boolean`

#### `logical.rs` - Operaciones lógicas

**Funciones**:
- `apply_and(left, right)` - AND lógico
- `apply_or(left, right)` - OR lógico

**Soporte**:
- ✅ Boolean: `true AND false` → `false`
- ❌ Short-circuit evaluation (se evalúan ambos operandos)

**Retorna**: `Value::Boolean`

#### `utils.rs` - Helpers

**Funciones**:
```rust
pub fn promote_numeric(left: Value, right: Value) -> (Value, Value)
pub fn coerce_to_boolean(value: Value) -> Result<bool, String>
```

**`promote_numeric(left, right)`**:
Promueve Number → Complex si uno de los operandos es Complex.

**Ejemplo**:
```rust
promote_numeric(Number(2.0), Complex(3.0, 4.0))
  → (Complex(2.0, 0.0), Complex(3.0, 4.0))
```

**`coerce_to_boolean(value)`**:
Convierte Value a Boolean.

**Reglas**:
- `Boolean(b)` → `b`
- `Number(0.0)` → `false`, sino `true`
- Otros tipos → Error

---

### 8. `unary_ops.rs` - Operaciones unarias

**Responsabilidad**: Aplicar operaciones unarias (-, NOT).

#### Función pública:
```rust
pub fn apply(op: &UnaryOp, operand: Value) -> Result<Value, String>
```

**Operaciones**:
- `-x` → Negación (Number, Complex, Vector, Tensor)
- `!x` → NOT lógico (Boolean)

**Ejemplos**:
```javascript
-5           // → -5
-(2+3i)      // → -2-3i
-[1,2,3]     // → [-1,-2,-3]
!true        // → false
```

---

### 9. `indexing/` - Indexación y slicing

**Responsabilidad**: Acceder elementos de arrays, matrices, tensores.

#### Función pública:
```rust
pub fn evaluate_index_access(evaluator: &mut Evaluator, object: &AstNode, indices: &[IndexArg]) -> Result<Value, String>
```

**Soporte**:
- ✅ Single index: `array[0]`
- ✅ Multi-index: `matrix[1, 2]`
- ✅ Slicing: `array[0:5]`, `array[1:]`, `array[:3]`
- ✅ Negative indices: `array[-1]` (último elemento)
- ✅ Tensor slicing: `tensor[0, :, 1:3]`

**Ejemplos**:
```javascript
let arr = [10, 20, 30, 40]
arr[0]      // → 10
arr[-1]     // → 40
arr[1:3]    // → [20, 30]

let matrix = [[1,2,3],[4,5,6]]
matrix[0,1]  // → 2
matrix[1,:]  // → [4,5,6]
```

---

### 10. `hof.rs` - Higher-Order Functions

**Responsabilidad**: Funciones que toman funciones como argumentos.

#### Funciones públicas:
```rust
pub fn handle_map(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String>
pub fn handle_filter(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String>
pub fn handle_reduce(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String>
pub fn handle_pipe(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String>
pub fn handle_any(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String>
pub fn handle_all(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String>
pub fn handle_find(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String>
pub fn handle_find_index(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String>
pub fn handle_count(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String>
```

#### `handle_map(evaluator, args)`
`map(f, array)` - Aplica `f` a cada elemento.

**Ejemplo**:
```javascript
map(x => x * 2, [1, 2, 3])  // → [2, 4, 6]
```

#### `handle_filter(evaluator, args)`
`filter(f, array)` - Filtra elementos que cumplen `f`.

**Ejemplo**:
```javascript
filter(x => x > 2, [1, 2, 3, 4])  // → [3, 4]
```

#### `handle_reduce(evaluator, args)`
`reduce(f, acc, array)` - Reduce array a un valor.

**Ejemplo**:
```javascript
reduce((acc, x) => acc + x, 0, [1, 2, 3, 4])  // → 10
```

#### `handle_pipe(evaluator, args)`
`pipe(f, g, h)(x)` - Composición de funciones.

**Ejemplo**:
```javascript
pipe(x => x * 2, x => x + 1)(5)  // → 11
```

#### `handle_any(evaluator, args)`
`any(f, array)` - ¿Algún elemento cumple `f`?

**Ejemplo**:
```javascript
any(x => x > 5, [1, 2, 3, 4])  // → false
any(x => x > 2, [1, 2, 3, 4])  // → true
```

#### `handle_all(evaluator, args)`
`all(f, array)` - ¿Todos los elementos cumplen `f`?

**Ejemplo**:
```javascript
all(x => x > 0, [1, 2, 3, 4])  // → true
all(x => x > 2, [1, 2, 3, 4])  // → false
```

#### `handle_find(evaluator, args)`
`find(f, array)` - Encuentra el primer elemento que cumple `f`.

**Ejemplo**:
```javascript
find(x => x > 2, [1, 2, 3, 4])  // → 3
```

#### `handle_find_index(evaluator, args)`
`findIndex(f, array)` - Encuentra el índice del primer elemento que cumple `f`.

**Ejemplo**:
```javascript
findIndex(x => x > 2, [1, 2, 3, 4])  // → 2
```

#### `handle_count(evaluator, args)`
`count(f, array)` - Cuenta cuántos elementos cumplen `f`.

**Ejemplo**:
```javascript
count(x => x > 2, [1, 2, 3, 4])  // → 2
```

---

### 11. `numerical.rs` - Cálculo numérico

**Responsabilidad**: Delegar a `achronyme-numerical` para derivación, integración, root-finding.

#### Funciones públicas:
```rust
pub fn handle_diff(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String>
pub fn handle_diff2(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String>
pub fn handle_diff3(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String>
pub fn handle_gradient(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String>
pub fn handle_integral(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String>
pub fn handle_simpson(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String>
pub fn handle_romberg(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String>
pub fn handle_quad(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String>
pub fn handle_solve(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String>
pub fn handle_newton(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String>
pub fn handle_secant(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String>
```

**Ejemplos**:
```javascript
// Derivada numérica
diff(x => x^2, 3)  // → 6.0

// Integral numérica
integral(x => x^2, 0, 1)  // → 0.333...

// Root finding
solve(x => x^2 - 2, 1, 2)  // → 1.414... (sqrt(2))
```

---

### 12. `optimization.rs` - Optimización lineal

**Responsabilidad**: Delegar a `achronyme-solver` para optimización lineal.

#### Funciones públicas:
```rust
pub fn handle_simplex(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String>
pub fn handle_linprog(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String>
pub fn handle_dual_simplex(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String>
// ... más funciones
```

**Ejemplo**:
```javascript
// Maximizar: 3x + 2y
// Sujeto a: x + y <= 4, 2x + y <= 5, x >= 0, y >= 0
let result = simplex(
    [3, 2],           // Coeficientes objetivo
    [[1,1],[2,1]],    // Restricciones (lado izquierdo)
    [4, 5],           // Restricciones (lado derecho)
    "max"
)
```

---

### 13. `debug.rs` - Debug utilities

**Responsabilidad**: Funciones de debug para inspeccionar valores.

#### Función pública:
```rust
pub fn handle_describe(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String>
```

**Ejemplo**:
```javascript
describe([1, 2, 3])
// Imprime:
// Type: Vector
// Length: 3
// Elements: [Number(1.0), Number(2.0), Number(3.0)]
```

---

## 🎯 Patrones comunes

### 1. Handler signature
```rust
pub fn handle_xxx(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String>
```

### 2. Validación de argumentos
```rust
if args.len() != 2 {
    return Err("function expects 2 arguments".to_string());
}
```

### 3. Evaluación de argumentos
```rust
let arg1 = evaluator.evaluate(&args[0])?;
let arg2 = evaluator.evaluate(&args[1])?;
```

### 4. Extracción de tipos
```rust
let num = match arg1 {
    Value::Number(n) => n,
    _ => return Err("Expected number".to_string()),
};
```

### 5. Retorno de resultado
```rust
Ok(Value::Number(result))
```

## 🔧 Extensión de handlers

### Agregar un nuevo handler

1. **Crear archivo**: `handlers/my_handler.rs`
```rust
use crate::evaluator::Evaluator;
use achronyme_types::value::Value;
use achronyme_parser::ast::AstNode;

pub fn handle_my_operation(
    evaluator: &mut Evaluator,
    args: &[AstNode]
) -> Result<Value, String> {
    // Validar argumentos
    if args.len() != 1 {
        return Err("my_operation expects 1 argument".to_string());
    }

    // Evaluar argumentos
    let arg = evaluator.evaluate(&args[0])?;

    // Extraer valor
    let num = match arg {
        Value::Number(n) => n,
        _ => return Err("Expected number".to_string()),
    };

    // Computar resultado
    let result = num * 2.0;

    // Retornar
    Ok(Value::Number(result))
}
```

2. **Registrar en `mod.rs`**:
```rust
pub mod my_handler;
```

3. **Llamar desde dispatcher** (si es necesario):
```rust
// En evaluator/dispatcher.rs o function_call.rs
match name {
    "my_operation" => handlers::my_handler::handle_my_operation(self, args),
    // ...
}
```

## 📊 Estadísticas

- **Total handlers**: 15+ handlers especializados
- **Binary ops**: 14 operadores (+, -, *, /, ^, %, >, <, >=, <=, ==, !=, AND, OR)
- **Unary ops**: 2 operadores (-, NOT)
- **HOF**: 9 funciones (map, filter, reduce, pipe, any, all, find, findIndex, count)
- **Numerical**: 11 funciones (diff, diff2, diff3, gradient, integral, simpson, romberg, quad, solve, newton, secant)
- **Optimization**: 8+ funciones (simplex, linprog, dual_simplex, etc.)

## 🔗 Ver también

- [../README.md](../README.md) - Documentación general del evaluador
- [../evaluator/README.md](../evaluator/README.md) - Motor de evaluación (si existe)
