# Achronyme Eval - Arquitectura interna

**Documentación técnica del motor de evaluación.**

## 🏗️ Arquitectura general

```
┌─────────────────────────────────────────────────────────────┐
│                        EVALUATOR                            │
│                                                             │
│  ┌─────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │ Environment │  │   Modules    │  │  Constants   │      │
│  │   (scopes)  │  │  (registry)  │  │ (pi, e, ...) │      │
│  └─────────────┘  └──────────────┘  └──────────────┘      │
│                                                             │
│  ┌───────────────────────────────────────────────────┐     │
│  │           DISPATCHER (dispatcher.rs)              │     │
│  │  evaluate(node: &AstNode) -> Result<Value>       │     │
│  └───────────────────────────────────────────────────┘     │
│                         │                                   │
│                         ↓                                   │
│  ┌───────────────────────────────────────────────────┐     │
│  │               HANDLERS (handlers/)                │     │
│  ├───────────────────────────────────────────────────┤     │
│  │ • literals.rs       - Number, String, Array, ...  │     │
│  │ • variables.rs      - Decl, Ref, Mutable          │     │
│  │ • binary_ops/       - +, -, *, /, >, <, ==, ...   │     │
│  │ • unary_ops.rs      - -, NOT                      │     │
│  │ • control_flow.rs   - If, While, Piecewise        │     │
│  │ • functions.rs      - Lambda, apply_lambda (TCO)  │     │
│  │ • function_call.rs  - Dispatcher de funciones     │     │
│  │ • hof.rs           - map, filter, reduce, pipe    │     │
│  │ • numerical.rs     - diff, integral, solve        │     │
│  │ • optimization.rs  - simplex, linprog             │     │
│  │ • indexing/        - array[0], matrix[1,2]        │     │
│  │ • assignment.rs    - x = y                        │     │
│  │ • debug.rs         - describe()                   │     │
│  └───────────────────────────────────────────────────┘     │
│                         │                                   │
│                         ↓                                   │
│  ┌───────────────────────────────────────────────────┐     │
│  │       FUNCTION MODULES (function_modules/)        │     │
│  ├───────────────────────────────────────────────────┤     │
│  │ • array.rs    - len, push, concat, slice, ...     │     │
│  │ • vector.rs   - dot, cross, norm, normalize       │     │
│  │ • matrix.rs   - det, inv, transpose, trace        │     │
│  │ • trig.rs     - sin, cos, tan, asin, ...          │     │
│  │ • complex.rs  - real, imag, magnitude, phase      │     │
│  │ • stats.rs    - mean, median, variance, stdev     │     │
│  │ • strings.rs  - upper, lower, split, join, ...    │     │
│  │ • dsp.rs      - fft, ifft, conv, xcorr            │     │
│  │ • graphs/     - dijkstra, bfs, dfs, mst, pert     │     │
│  │ • io.rs       - print, readFile, writeFile        │     │
│  └───────────────────────────────────────────────────┘     │
└─────────────────────────────────────────────────────────────┘
```

## 📁 Estructura de módulos

### 1. `evaluator/` - Motor de evaluación

#### `mod.rs` - Struct Evaluator
```rust
pub struct Evaluator {
    pub(crate) env: Environment,                    // Variables, scopes
    pub(crate) constants: ConstantsRegistry,        // pi, e, phi, tau
    pub(crate) functions: FunctionRegistry,         // Backward compat
    pub(crate) module_registry: ModuleRegistry,     // Sistema de módulos
    pub(crate) imported_modules: HashMap<...>,      // import tracking
    pub(crate) exported_values: HashMap<...>,       // export tracking
    pub(crate) module_cache: HashMap<...>,          // Caché de módulos cargados
    pub(crate) current_file_dir: Option<String>,    // Para imports relativos
    pub(crate) tco_mode: bool,                      // Flag TCO
}
```

**Responsabilidad**: Estado del evaluador, constructor `new()`.

#### `dispatcher.rs` - Dispatcher principal
```rust
impl Evaluator {
    pub fn eval_str(&mut self, source: &str) -> Result<Value, String>
    pub fn evaluate(&mut self, node: &AstNode) -> Result<Value, String>
    fn evaluate_field_access(...)
    fn evaluate_call_expression(...)
    fn evaluate_sequence(...)
    fn evaluate_do_block(...)
    fn evaluate_import(...)
    fn evaluate_export(...)
}
```

**Flujo de evaluate()**:
```rust
pub fn evaluate(&mut self, node: &AstNode) -> Result<Value, String> {
    match node {
        // LITERALS (handlers/literals.rs)
        AstNode::Number(n) => handlers::literals::evaluate_number(*n),
        AstNode::Boolean(b) => handlers::literals::evaluate_boolean(*b),
        AstNode::StringLiteral(s) => handlers::literals::evaluate_string(s),
        AstNode::ComplexLiteral { re, im } => handlers::literals::evaluate_complex(*re, *im),
        AstNode::ArrayLiteral(elements) => handlers::literals::evaluate_array(self, elements),
        AstNode::RecordLiteral(fields) => handlers::literals::evaluate_record(self, fields),

        // VARIABLES (handlers/variables.rs)
        AstNode::VariableDecl { name, initializer } =>
            handlers::variables::evaluate_declaration(self, name, initializer),
        AstNode::VariableRef(name) =>
            handlers::variables::evaluate_reference(self, name),
        AstNode::MutableDecl { name, initializer } =>
            handlers::variables::evaluate_mutable_declaration(self, name, initializer),

        // ASSIGNMENT (handlers/assignment.rs)
        AstNode::Assignment { target, value } =>
            handlers::assignment::evaluate_assignment(self, target, value),

        // CONTROL FLOW (handlers/control_flow.rs)
        AstNode::If { condition, then_expr, else_expr } =>
            handlers::control_flow::evaluate_if(self, condition, then_expr, else_expr),
        AstNode::WhileLoop { condition, body } =>
            handlers::control_flow::evaluate_while(self, condition, body),
        AstNode::Piecewise { cases, default } =>
            handlers::control_flow::evaluate_piecewise(self, cases, default),

        // OPERATIONS (handlers/binary_ops/, handlers/unary_ops.rs)
        AstNode::BinaryOp { op, left, right } => {
            let left_val = self.evaluate(left)?;
            let right_val = self.evaluate(right)?;
            handlers::binary_ops::apply(op, left_val, right_val)
        }
        AstNode::UnaryOp { op, operand } => {
            let operand_val = self.evaluate(operand)?;
            handlers::unary_ops::apply(op, operand_val)
        }

        // FUNCTIONS (handlers/function_call.rs, handlers/functions.rs)
        AstNode::FunctionCall { name, args } =>
            handlers::function_call::dispatch(self, name, args),
        AstNode::CallExpression { callee, args } =>
            self.evaluate_call_expression(callee, args),
        AstNode::Lambda { params, body } =>
            handlers::functions::evaluate_lambda(self, params, body),

        // INDEXING (handlers/indexing/)
        AstNode::IndexAccess { object, indices } =>
            handlers::indexing::evaluate_index_access(self, object, indices),

        // SPECIAL
        AstNode::Return { value } => {
            let return_value = self.evaluate(value)?;
            Ok(Value::EarlyReturn(Box::new(return_value)))
        }
        AstNode::SelfReference => self.env.get("self").map_err(...),
        AstNode::RecReference => self.env.get("rec").map_err(...),
        AstNode::FieldAccess { record, field } =>
            self.evaluate_field_access(record, field),

        // MODULE SYSTEM
        AstNode::Import { items, module_path } =>
            self.evaluate_import(items, module_path),
        AstNode::Export { items } =>
            self.evaluate_export(items),

        // BLOCKS
        AstNode::Sequence { statements } => self.evaluate_sequence(statements),
        AstNode::DoBlock { statements } => self.evaluate_do_block(statements),

        // GRAPHS
        AstNode::Edge { from, to, directed, metadata } =>
            handlers::literals::evaluate_edge(self, from, to, *directed, metadata),
    }
}
```

**Orden de evaluación**: Post-order traversal (hijos antes que padres).

#### `lambda_eval.rs` - LambdaEvaluator trait impl
```rust
impl LambdaEvaluator for Evaluator {
    fn eval_at(&mut self, func: &Function, x: f64) -> Result<f64, String>
    fn eval_vec_at(&mut self, func: &Function, point: &[f64]) -> Result<f64, String>
    fn eval_at_nd(&mut self, func: &Function, args: &[f64]) -> Result<f64, String>
}

impl Evaluator {
    pub fn apply_lambda(&mut self, function: &Function, args: Vec<Value>) -> Result<Value, String>
}
```

**Responsabilidad**: Implementación del trait `LambdaEvaluator` para uso en `achronyme-solver`.

#### `state.rs` - State management
```rust
impl Evaluator {
    pub fn environment(&self) -> &Environment
    pub fn environment_mut(&mut self) -> &mut Environment
    pub fn constants(&self) -> &ConstantsRegistry
    pub fn functions(&self) -> &FunctionRegistry
    pub fn module_registry(&self) -> &ModuleRegistry
    pub fn imported_modules(&self) -> &HashMap<...>
    pub fn is_tco_mode(&self) -> bool
    pub fn set_tco_mode(&mut self, enabled: bool)
    pub fn set_current_file_dir(&mut self, file_path: &str)
}
```

**Responsabilidad**: Getters y setters para el estado del evaluador.

#### `modules.rs` - User module loading
```rust
impl Evaluator {
    fn load_user_module(&mut self, module_path: &str) -> Result<HashMap<String, Value>, String>
}
```

**Responsabilidad**: Cargar módulos definidos por el usuario desde archivos `.soc`.

### 2. `handlers/` - Handlers especializados

Cada handler maneja un tipo específico de operación.

#### `literals.rs` - Literales
```rust
pub fn evaluate_number(n: f64) -> Result<Value, String>
pub fn evaluate_boolean(b: bool) -> Result<Value, String>
pub fn evaluate_string(s: &str) -> Result<Value, String>
pub fn evaluate_complex(re: f64, im: f64) -> Result<Value, String>
pub fn evaluate_array(evaluator: &mut Evaluator, elements: &[ArrayElement]) -> Result<Value, String>
pub fn evaluate_record(evaluator: &mut Evaluator, fields: &[RecordFieldOrSpread]) -> Result<Value, String>
pub fn evaluate_edge(evaluator: &mut Evaluator, from: &str, to: &str, directed: bool, metadata: &Option<Box<AstNode>>) -> Result<Value, String>
```

**Responsabilidad**:
- Convertir literales del AST a `Value`
- Soportar spread syntax (`[1, ...vec, 2]`, `{ a: 1, ...other }`)
- Auto-conversión de vectores a matrices/tensores cuando es apropiado
- Type promotion (Number → Complex cuando sea necesario)

#### `variables.rs` - Variables
```rust
pub fn evaluate_declaration(evaluator: &mut Evaluator, name: &str, initializer: &AstNode) -> Result<Value, String>
pub fn evaluate_reference(evaluator: &Evaluator, name: &str) -> Result<Value, String>
pub fn evaluate_mutable_declaration(evaluator: &mut Evaluator, name: &str, initializer: &AstNode) -> Result<Value, String>
```

**Responsabilidad**:
- `let x = 10` → Define variable inmutable
- `mut x = 10` → Define variable mutable (wrapped en `MutableRef`)
- `x` → Lee variable del entorno

#### `assignment.rs` - Asignaciones
```rust
pub fn evaluate_assignment(evaluator: &mut Evaluator, target: &AstNode, value: &AstNode) -> Result<Value, String>
```

**Responsabilidad**:
- `x = 20` → Actualiza variable mutable
- `array[i] = value` → Actualiza elemento de array
- `record.field = value` → Actualiza campo de record
- Validación: solo se pueden asignar variables mutables

#### `control_flow.rs` - Flujo de control
```rust
pub fn evaluate_if(evaluator: &mut Evaluator, condition: &AstNode, then_expr: &AstNode, else_expr: &AstNode) -> Result<Value, String>
pub fn evaluate_while(evaluator: &mut Evaluator, condition: &AstNode, body: &AstNode) -> Result<Value, String>
pub fn evaluate_piecewise(evaluator: &mut Evaluator, cases: &[(AstNode, AstNode)], default: &Option<Box<AstNode>>) -> Result<Value, String>
```

**Responsabilidad**:
- `if(condition, then, else)` → Condicional ternario
- `while(condition, body)` → Loop mientras condición sea true
- `piecewise((c1, v1), (c2, v2), default)` → Match de casos

#### `functions.rs` - Funciones y lambdas
```rust
pub fn evaluate_lambda(evaluator: &Evaluator, params: &[String], body: &AstNode) -> Result<Value, String>
pub fn apply_lambda(evaluator: &mut Evaluator, function: &Function, args: Vec<Value>) -> Result<Value, String>
fn apply_lambda_regular(...) -> Result<Value, String>
fn apply_lambda_tco(...) -> Result<Value, String>
pub fn eval_lambda_at(evaluator: &mut Evaluator, func: &Function, x: f64) -> Result<f64, String>
```

**Responsabilidad**:
- Crear lambdas con closures: `x => x * 2`
- Aplicar funciones a argumentos
- **TCO** (Tail Call Optimization): Detectar recursión tail y optimizar
- Inyectar `rec` (self-reference) y `self` (para métodos de records)

**TCO Flow**:
```
1. tco::is_tail_recursive_function(body) → bool
   ↓ true
2. apply_lambda_tco() - Usa loop iterativo
   ↓
3. set_tco_mode(true) - Activa modo TCO
   ↓
4. evaluate(body) - Evalúa cuerpo
   ↓
5. rec(...) en tail position → Value::TailCall(args)
   ↓
6. Loop: actualiza args, continúa (NO recurre)
   ↓
7. Caso base → Ok(value)
```

**Optimización de closures**:
```rust
// ANTES (LENTO): Clonar todo el entorno
let closure = evaluator.environment().snapshot();  // O(n) clonación profunda

// AHORA (RÁPIDO): Rc<RefCell<Environment>>
let closure_env = evaluator.environment().to_rc();  // O(1) increment refcount
```

#### `function_call.rs` - Dispatcher de llamadas
```rust
pub fn dispatch(evaluator: &mut Evaluator, name: &str, args: &[AstNode]) -> Result<Value, String>
```

**Responsabilidad**: Resolver y despachar llamadas de función.

**Orden de resolución**:
1. Constantes (sin argumentos): `pi`, `e`, `tau`
2. Variables (funciones almacenadas): `let f = x => x*2; f(5)`
3. Methods de records: `person.greet()`
4. Higher-order functions: `map`, `filter`, `reduce`, `pipe`
5. Funciones numéricas: `diff`, `integral`, `solve`, `newton`
6. Funciones de debug: `describe`
7. Optimización: `simplex`, `linprog`, `dual_simplex`
8. **Module system**: Prelude → Imported modules → FunctionRegistry (fallback)

#### `hof.rs` - Higher-Order Functions
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

**Responsabilidad**:
- `map(f, array)` → Aplica `f` a cada elemento
- `filter(f, array)` → Filtra elementos que cumplen `f`
- `reduce(f, acc, array)` → Reduce array a un valor
- `pipe(f, g, h)(x)` → Composición de funciones
- Predicados: `any`, `all`, `find`, `findIndex`, `count`

#### `numerical.rs` - Cálculo numérico
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

**Responsabilidad**: Delegar a `achronyme-numerical` para derivación, integración, root-finding.

#### `optimization.rs` - Optimización lineal
```rust
pub fn handle_simplex(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String>
pub fn handle_linprog(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String>
pub fn handle_dual_simplex(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String>
pub fn handle_two_phase_simplex(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String>
pub fn handle_revised_simplex(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String>
pub fn handle_objective_value(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String>
pub fn handle_shadow_price(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String>
pub fn handle_sensitivity_c(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String>
pub fn handle_sensitivity_b(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String>
```

**Responsabilidad**: Delegar a `achronyme-solver` para optimización lineal.

#### `debug.rs` - Debug utilities
```rust
pub fn handle_describe(evaluator: &mut Evaluator, args: &[AstNode]) -> Result<Value, String>
```

**Responsabilidad**: `describe(value)` → Imprime información detallada sobre un valor.

#### `binary_ops/` - Operaciones binarias (refactorizado)

**Estructura modular**:
```
binary_ops/
├── mod.rs           # Dispatcher apply(op, left, right)
├── arithmetic.rs    # +, -, *, /, ^, %
├── comparison.rs    # >, <, >=, <=, ==, !=
├── logical.rs       # AND, OR
└── utils.rs         # promote_numeric(), coerce_to_boolean()
```

**mod.rs** - Dispatcher:
```rust
pub fn apply(op: &BinaryOp, left: Value, right: Value) -> Result<Value, String> {
    match op {
        BinaryOp::Add => apply_add(left, right),
        BinaryOp::Subtract => apply_subtract(left, right),
        BinaryOp::Multiply => apply_multiply(left, right),
        BinaryOp::Divide => apply_divide(left, right),
        BinaryOp::Power => apply_power(left, right),
        BinaryOp::Modulo => apply_modulo(left, right),
        BinaryOp::Gt => apply_gt(left, right),
        BinaryOp::Lt => apply_lt(left, right),
        // ... más operadores
    }
}
```

**arithmetic.rs** - Operaciones aritméticas:
- Soporta: Number, Complex, Vector, Tensor, String (concat)
- Type promotion: Number + Complex → Complex
- Broadcasting: Tensor + Number → Tensor

**comparison.rs** - Operaciones de comparación:
- Soporta: Number, Complex, Boolean, String
- Retorna: Boolean

**logical.rs** - Operaciones lógicas:
- Soporta: Boolean
- Retorna: Boolean

**utils.rs** - Helpers:
- `promote_numeric(left, right)` → Promueve Number a Complex si es necesario
- `coerce_to_boolean(value)` → Convierte Value a Boolean

#### `unary_ops.rs` - Operaciones unarias
```rust
pub fn apply(op: &UnaryOp, operand: Value) -> Result<Value, String>
```

**Responsabilidad**:
- `-x` → Negación (Number, Complex, Vector, Tensor)
- `!x` → NOT lógico (Boolean)

#### `indexing/` - Indexación y slicing
```rust
pub fn evaluate_index_access(evaluator: &mut Evaluator, object: &AstNode, indices: &[IndexArg]) -> Result<Value, String>
```

**Responsabilidad**:
- `array[0]` → Acceder elemento
- `matrix[1, 2]` → Acceder elemento de matriz
- `array[0:5]` → Slicing de rango
- `tensor[0, :, 1:3]` → Slicing avanzado

### 3. `function_modules/` - Built-in functions

Implementación de las funciones built-in del lenguaje.

#### `array.rs` - Operaciones de arrays
```rust
pub fn len(args: &[Value], _env: &mut Environment) -> Result<Value, String>
pub fn push(args: &[Value], _env: &mut Environment) -> Result<Value, String>
pub fn pop(args: &[Value], _env: &mut Environment) -> Result<Value, String>
pub fn concat(args: &[Value], _env: &mut Environment) -> Result<Value, String>
pub fn slice(args: &[Value], _env: &mut Environment) -> Result<Value, String>
pub fn reverse(args: &[Value], _env: &mut Environment) -> Result<Value, String>
pub fn sort(args: &[Value], _env: &mut Environment) -> Result<Value, String>
pub fn unique(args: &[Value], _env: &mut Environment) -> Result<Value, String>
// ... más funciones
```

#### `vector.rs` - Operaciones vectoriales
```rust
pub fn dot(args: &[Value], _env: &mut Environment) -> Result<Value, String>
pub fn cross(args: &[Value], _env: &mut Environment) -> Result<Value, String>
pub fn norm(args: &[Value], _env: &mut Environment) -> Result<Value, String>
pub fn normalize(args: &[Value], _env: &mut Environment) -> Result<Value, String>
```

#### `matrix.rs` - Operaciones matriciales
```rust
pub fn det(args: &[Value], _env: &mut Environment) -> Result<Value, String>
pub fn inv(args: &[Value], _env: &mut Environment) -> Result<Value, String>
pub fn transpose(args: &[Value], _env: &mut Environment) -> Result<Value, String>
pub fn trace(args: &[Value], _env: &mut Environment) -> Result<Value, String>
```

#### `trig.rs` - Funciones trigonométricas
```rust
pub fn sin(args: &[Value], _env: &mut Environment) -> Result<Value, String>
pub fn cos(args: &[Value], _env: &mut Environment) -> Result<Value, String>
pub fn tan(args: &[Value], _env: &mut Environment) -> Result<Value, String>
pub fn asin(args: &[Value], _env: &mut Environment) -> Result<Value, String>
pub fn acos(args: &[Value], _env: &mut Environment) -> Result<Value, String>
pub fn atan(args: &[Value], _env: &mut Environment) -> Result<Value, String>
```

#### `exponential.rs` - Funciones exponenciales
```rust
pub fn exp(args: &[Value], _env: &mut Environment) -> Result<Value, String>
pub fn ln(args: &[Value], _env: &mut Environment) -> Result<Value, String>
pub fn log(args: &[Value], _env: &mut Environment) -> Result<Value, String>
pub fn log10(args: &[Value], _env: &mut Environment) -> Result<Value, String>
```

#### `complex.rs` - Operaciones con complejos
```rust
pub fn real(args: &[Value], _env: &mut Environment) -> Result<Value, String>
pub fn imag(args: &[Value], _env: &mut Environment) -> Result<Value, String>
pub fn conjugate(args: &[Value], _env: &mut Environment) -> Result<Value, String>
pub fn magnitude(args: &[Value], _env: &mut Environment) -> Result<Value, String>
pub fn phase(args: &[Value], _env: &mut Environment) -> Result<Value, String>
```

#### `stats.rs` - Estadísticas
```rust
pub fn mean(args: &[Value], _env: &mut Environment) -> Result<Value, String>
pub fn median(args: &[Value], _env: &mut Environment) -> Result<Value, String>
pub fn variance(args: &[Value], _env: &mut Environment) -> Result<Value, String>
pub fn stdev(args: &[Value], _env: &mut Environment) -> Result<Value, String>
pub fn sum(args: &[Value], _env: &mut Environment) -> Result<Value, String>
```

#### `strings.rs` - Operaciones con strings
```rust
pub fn upper(args: &[Value], _env: &mut Environment) -> Result<Value, String>
pub fn lower(args: &[Value], _env: &mut Environment) -> Result<Value, String>
pub fn split(args: &[Value], _env: &mut Environment) -> Result<Value, String>
pub fn join(args: &[Value], _env: &mut Environment) -> Result<Value, String>
pub fn charAt(args: &[Value], _env: &mut Environment) -> Result<Value, String>
pub fn substring(args: &[Value], _env: &mut Environment) -> Result<Value, String>
// ... más funciones
```

#### `dsp.rs` - Digital Signal Processing
```rust
pub fn fft(args: &[Value], _env: &mut Environment) -> Result<Value, String>
pub fn ifft(args: &[Value], _env: &mut Environment) -> Result<Value, String>
pub fn conv(args: &[Value], _env: &mut Environment) -> Result<Value, String>
pub fn xcorr(args: &[Value], _env: &mut Environment) -> Result<Value, String>
// ... más funciones
```

#### `graphs/` - Algoritmos de grafos
```
graphs/
├── mod.rs
├── traversal.rs       # bfs, dfs
├── shortest_path.rs   # dijkstra, bellman_ford, floyd_warshall
├── mst.rs            # kruskal, prim (Minimum Spanning Tree)
├── connectivity.rs    # is_connected, connected_components
├── cycles.rs         # has_cycle, find_cycles
├── topological.rs    # topological_sort
├── network.rs        # max_flow, min_cut
├── helpers.rs        # Edge → Graph conversion
└── pert/             # PERT/CPM (Project Evaluation and Review Technique)
    ├── mod.rs
    ├── project.rs
    ├── critical_path.rs
    ├── cpm.rs
    ├── probabilistic.rs
    ├── statistics.rs
    ├── validation.rs
    └── state_detection.rs
```

#### `io.rs` - Input/Output
```rust
pub fn print(args: &[Value], _env: &mut Environment) -> Result<Value, String>
pub fn println(args: &[Value], _env: &mut Environment) -> Result<Value, String>
pub fn readFile(args: &[Value], _env: &mut Environment) -> Result<Value, String>
pub fn writeFile(args: &[Value], _env: &mut Environment) -> Result<Value, String>
```

### 4. `modules/` - Sistema de módulos

#### `mod.rs` - Module, ModuleRegistry
```rust
pub struct Module {
    pub name: String,
    pub exports: HashMap<String, (BuiltinFunction, i32)>,  // (función, arity)
}

pub struct ModuleRegistry {
    prelude: HashMap<String, (BuiltinFunction, i32)>,     // Siempre disponible
    modules: HashMap<String, Module>,                      // Requieren import
}

impl ModuleRegistry {
    pub fn resolve(&self, name: &str, imported_modules: &HashMap<...>) -> Option<(BuiltinFunction, i32)>
}
```

**Orden de resolución**:
1. **Prelude** (siempre disponible): ~39 funciones
2. **Imported modules** (requieren `import { ... } from "module"`)
3. ~~Global fallback~~ (eliminado en Phase 3)

#### `builtin_registry.rs` - create_builtin_registry()
```rust
pub fn create_builtin_registry() -> ModuleRegistry
```

**Responsabilidad**: Crear el registro de módulos built-in con todas las funciones organizadas.

**Módulos**:
- `math`: asin, acos, atan, sinh, cosh, tanh, asinh, acosh, atanh
- `stats`: mean, median, mode, variance, stdev, min, max
- `dsp`: fft, ifft, conv, xcorr, hamming, hanning, blackman
- `linalg`: det, inv, transpose, trace, rank, solve_linear
- `graphs`: dijkstra, bfs, dfs, kruskal, prim, has_cycle, topological_sort
- `io`: readFile, writeFile, readJson, writeJson
- ... más módulos

### 5. `tco/` - Tail Call Optimization

#### `mod.rs` - TCO detection
```rust
pub fn is_tail_position(node: &AstNode) -> bool
pub fn is_tail_recursive_function(body: &AstNode) -> bool
fn contains_rec(node: &AstNode) -> bool
fn all_rec_are_tail(node: &AstNode) -> bool
fn all_rec_are_tail_helper(node: &AstNode, in_tail_position: bool) -> bool
```

**Algoritmo**:
1. `contains_rec(body)` → ¿Usa `rec`?
2. `all_rec_are_tail(body)` → ¿Todas las llamadas a `rec` están en tail position?
3. Si ambos son true → `is_tail_recursive_function()` retorna true

**Tail position**:
- ✅ Último statement de un bloque
- ✅ Último statement de una rama if/else
- ✅ Último resultado de piecewise
- ❌ Dentro de operaciones binarias (`n * rec(n-1)`)
- ❌ Dentro de array literals (`[rec(n-1), n]`)
- ❌ Dentro de while loops (el cuerpo no es tail position)

### 6. `constants.rs` - ConstantsRegistry
```rust
pub struct ConstantsRegistry {
    constants: HashMap<String, f64>
}

impl ConstantsRegistry {
    pub fn new() -> Self  // Registra pi, e, phi, tau, sqrt2, etc.
    pub fn has(&self, name: &str) -> bool
    pub fn get(&self, name: &str) -> Result<f64, String>
}
```

### 7. `functions.rs` - FunctionRegistry (backward compat)
```rust
pub struct FunctionRegistry {
    functions: HashMap<String, (BuiltinFunction, i32)>
}

pub type BuiltinFunction = fn(&[Value], &mut Environment) -> Result<Value, String>;
```

**Responsabilidad**: Registro global de funciones (backward compatibility). Ahora se usa `ModuleRegistry` preferentemente.

## 🔄 Flujo de evaluación completo

### Ejemplo: `2 + 3 * 4`

```
1. Parser → AST:
   BinaryOp {
       op: Add,
       left: Number(2),
       right: BinaryOp {
           op: Multiply,
           left: Number(3),
           right: Number(4)
       }
   }

2. Evaluator.evaluate(AST):
   match BinaryOp { op: Add, left, right } => {
       // Evaluar left (post-order)
       left_val = evaluate(Number(2)) → Value::Number(2.0)

       // Evaluar right (post-order)
       right_val = evaluate(BinaryOp { Multiply, ... })
           ↓
           left_val = evaluate(Number(3)) → Value::Number(3.0)
           right_val = evaluate(Number(4)) → Value::Number(4.0)
           handlers::binary_ops::apply(Multiply, 3.0, 4.0) → Value::Number(12.0)

       // Aplicar operación
       handlers::binary_ops::apply(Add, Value::Number(2.0), Value::Number(12.0))
           → Value::Number(14.0)
   }

3. Resultado: Value::Number(14.0)
```

### Ejemplo: `map(x => x * 2, [1, 2, 3])`

```
1. Parser → AST:
   FunctionCall {
       name: "map",
       args: [
           Lambda { params: ["x"], body: BinaryOp { Multiply, ... } },
           ArrayLiteral([Number(1), Number(2), Number(3)])
       ]
   }

2. Evaluator.evaluate(FunctionCall):
   handlers::function_call::dispatch(evaluator, "map", args)
       ↓
       match "map" => handlers::hof::handle_map(evaluator, args)
           ↓
           1. Evaluar lambda → Value::Function(UserDefined { ... })
           2. Evaluar array → Value::Vector([1.0, 2.0, 3.0])
           3. Para cada elemento:
              apply_lambda(func, [1.0]) → Value::Number(2.0)
              apply_lambda(func, [2.0]) → Value::Number(4.0)
              apply_lambda(func, [3.0]) → Value::Number(6.0)
           4. Retornar: Value::Vector([2.0, 4.0, 6.0])
```

### Ejemplo: Factorial tail-recursive con TCO

```javascript
let factorial = (n, acc) => if(n <= 1, acc, rec(n-1, acc*n))
factorial(100000, 1)
```

```
1. evaluate(Lambda { params: ["n", "acc"], body: If { ... } })
   ↓
   handlers::functions::evaluate_lambda(evaluator, ["n", "acc"], body)
       ↓
       closure_env = evaluator.environment().to_rc()  // O(1)
       function = Function::new_with_env(["n", "acc"], body, closure_env)
       → Value::Function(UserDefined { ... })

2. evaluate(CallExpression { callee: factorial, args: [100000, 1] })
   ↓
   evaluator.apply_lambda(function, [Value::Number(100000.0), Value::Number(1.0)])
       ↓
       tco::is_tail_recursive_function(body) → true ✅
       ↓
       apply_lambda_tco(evaluator, function, [100000.0, 1.0])
           ↓
           set_tco_mode(true)
           loop {
               // Iter 1: n=100000, acc=1
               evaluate(if(n <= 1, acc, rec(n-1, acc*n)))
                   → evaluate(rec(99999, 100000))
                   → Value::TailCall([99999.0, 100000.0])  // TCO marker!

               // Iter 2: n=99999, acc=100000
               args = [99999.0, 100000.0]
               continue loop  // NO recurre, solo actualiza args

               // ... 99998 iteraciones más ...

               // Iter 100000: n=1, acc=factorial(100000)
               evaluate(if(n <= 1, acc, rec(...)))
                   → Value::Number(acc)  // Caso base!
               break Ok(Value::Number(acc))
           }
           set_tco_mode(false)
           → Ok(Value::Number(factorial(100000)))
```

## 🎯 Patrones de diseño

### 1. Handler Pattern
Cada tipo de nodo AST tiene su propio handler. Facilita testing y extensión.

### 2. Post-Order Traversal
Evaluar hijos antes que padres. Garantiza que operandos estén listos.

### 3. Tail Call Optimization
Convertir recursión tail en loop iterativo. Permite recursión infinita.

### 4. Module System
Organizar funciones en namespaces (prelude + modules). Evita namespace pollution.

### 5. Closure Capture Eficiente
Usar `Rc<RefCell<Environment>>` en vez de clonar todo. 100x más rápido.

### 6. Type Promotion
Promover Number → Complex automáticamente cuando sea necesario.

### 7. Broadcasting
Operaciones tensor/scalar se broadcaste automáticamente (estilo NumPy).

## 🔧 Extensión del evaluador

### Agregar un nuevo handler

1. Crear `handlers/my_handler.rs`:
```rust
use crate::evaluator::Evaluator;
use achronyme_types::value::Value;

pub fn handle_my_operation(evaluator: &mut Evaluator, ...) -> Result<Value, String> {
    // Implementación
}
```

2. Registrar en `handlers/mod.rs`:
```rust
pub mod my_handler;
```

3. Llamar desde `dispatcher.rs`:
```rust
AstNode::MyOperation { ... } => handlers::my_handler::handle_my_operation(self, ...),
```

### Agregar una nueva función built-in

1. Implementar en `function_modules/my_module.rs`:
```rust
pub fn my_func(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    // Validar argumentos
    if args.len() != 2 {
        return Err("my_func expects 2 arguments".to_string());
    }

    // Extraer valores
    let a = match &args[0] {
        Value::Number(n) => n,
        _ => return Err("First argument must be a number".to_string()),
    };

    // Computar resultado
    Ok(Value::Number(a * 2.0))
}
```

2. Registrar en `modules/builtin_registry.rs`:
```rust
let mut my_module = Module::new("my_module");
my_module.register("my_func", function_modules::my_module::my_func, 2);
registry.register_module(my_module);
```

3. Usar en SOC:
```javascript
import { my_func } from "my_module"
my_func(10, 20)
```

## 📊 Estadísticas

- **Handlers**: 15+ handlers especializados
- **Built-in functions**: 150+ funciones
- **Function modules**: 17 módulos (array, vector, matrix, trig, stats, dsp, graphs, etc.)
- **Graph algorithms**: 15+ algoritmos (dijkstra, bfs, dfs, kruskal, prim, etc.)
- **PERT/CPM**: 8 submódulos para gestión de proyectos
- **Lines of code**: ~8,000 LOC
- **Files**: 76 archivos .rs

## 🔗 Ver también

- [README.md](../README.md) - Documentación general del crate
- [handlers/README.md](handlers/README.md) - Sistema de handlers detallado
