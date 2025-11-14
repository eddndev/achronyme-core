# Roadmap: Async/Await y Sistema Reactivo para GUIs

**Versión**: 1.0
**Fecha**: 2025-01-14
**Estado**: Planificación
**Objetivo**: Habilitar GUIs nativas y componentes reactivos en Achronyme

## Tabla de Contenidos

- [Visión General](#visión-general)
- [Motivación](#motivación)
- [Arquitectura General](#arquitectura-general)
- [Fases de Implementación](#fases-de-implementación)
  - [Fase 1: Iteradores y Lazy Evaluation](#fase-1-iteradores-y-lazy-evaluation)
  - [Fase 2: Event Loop y Futures](#fase-2-event-loop-y-futures)
  - [Fase 3: Async/Await Syntax](#fase-3-asyncawait-syntax)
  - [Fase 4: Concurrency Primitives](#fase-4-concurrency-primitives)
  - [Fase 5: Sistema Reactivo](#fase-5-sistema-reactivo)
  - [Fase 6: GUI Bindings Nativos](#fase-6-gui-bindings-nativos)
- [Timeline y Dependencias](#timeline-y-dependencias)
- [Plan MVP](#plan-mvp)
- [Referencias](#referencias)

---

## Visión General

Este documento describe el plan de implementación para transformar Achronyme de un lenguaje de computación científica sincrónico a un lenguaje con capacidades completas de programación asíncrona, concurrencia y reactividad, específicamente diseñado para crear GUIs nativas fluidas.

### Objetivos Clave

1. **No bloquear el UI thread**: Operaciones I/O y cómputo pesado deben ser asíncronas
2. **Sistema reactivo**: Actualización automática de UI cuando cambian los datos
3. **Concurrency segura**: Primitivas para evitar data races
4. **Ergonomía**: Sintaxis async/await similar a JS/Rust/C#
5. **Performance**: Event loop eficiente, batched updates, minimal overhead

### Casos de Uso Habilitados

```achronyme
// GUI reactiva con async I/O
let UserDashboard = (props) => component {
    let user = signal(null)
    let loading = signal(true)

    async_effect(async () => do {
        loading.set(true)
        let data = await http_get("/api/users/" + props.user_id)
        user.set(data)
        loading.set(false)
    }, [props.user_id])

    view {
        if(loading.value) {
            Spinner()
        } else {
            div {
                h1 { text(user.value.name) }
                p { text(user.value.email) }
            }
        }
    }
}
```

---

## Motivación

### El Problema

Las GUIs modernas requieren:
- **Event loop single-threaded**: El rendering ocurre en un solo thread
- **60 FPS**: ~16ms por frame, no puede bloquearse
- **I/O asíncrono**: Network requests, file I/O, database queries
- **Reactividad**: Cambios en datos deben propagarse automáticamente al UI

Sin async/await, cualquier operación que tome >16ms congela la UI:

```achronyme
// ❌ MALO: Bloquea el UI thread
button.on_click(() => do {
    let data = http_get("api.com/large-dataset")  // Bloquea 2 segundos
    let processed = expensive_computation(data)    // Bloquea 1 segundo
    update_label(processed)                        // UI congelada 3 segundos total
})
```

### La Solución

Con async/await, las operaciones yields al event loop:

```achronyme
// ✅ BUENO: No bloquea, UI fluido
button.on_click(async () => do {
    let data = await http_get("api.com/large-dataset")  // Yields, UI responde
    let processed = await compute_async(data)            // Yields, UI responde
    update_label(processed)                              // Se ejecuta cuando esté listo
})
```

---

## Arquitectura General

### Componentes del Sistema

```
┌─────────────────────────────────────────────────────────────┐
│                    Achronyme Runtime                        │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌──────────────┐  ┌──────────────┐  ┌─────────────────┐  │
│  │   Async/     │  │  Reactive    │  │   GUI           │  │
│  │   Await      │  │  System      │  │   Bindings      │  │
│  │   Engine     │──▶│  (Signals)   │──▶│   (Native)      │  │
│  └──────────────┘  └──────────────┘  └─────────────────┘  │
│         │                  │                               │
│         ▼                  ▼                               │
│  ┌──────────────────────────────────────────────────┐     │
│  │           Concurrency Primitives                 │     │
│  │  (Channels, Mutex, RwLock, Select)              │     │
│  └──────────────────────────────────────────────────┘     │
│         │                                                  │
│         ▼                                                  │
│  ┌──────────────────────────────────────────────────┐     │
│  │         Tokio Event Loop (Rust FFI)              │     │
│  └──────────────────────────────────────────────────┘     │
│         │                                                  │
│         ▼                                                  │
│  ┌──────────────────────────────────────────────────┐     │
│  │      Generators / Iterators (Base)               │     │
│  └──────────────────────────────────────────────────┘     │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### Flujo de Ejecución

1. **Generators**: Funciones que pueden pausarse y resumirse (yield)
2. **Event Loop**: Scheduler que ejecuta tasks asíncronos
3. **Futures**: Computaciones que eventualmente producen un valor
4. **Async/Await**: Syntax sugar sobre generators + futures
5. **Reactive Signals**: Auto-tracking de dependencias + batched updates
6. **GUI Bindings**: Bridge entre signals y widgets nativos

---

## Fases de Implementación

## Fase 1: Iteradores y Lazy Evaluation

**Duración estimada**: 2-3 semanas
**Dependencias**: Ninguna (se puede empezar ya)
**Propósito**: Fundamento para generators y async

### 1.1 Iterator Protocol

Definir un protocolo estándar para iteradores:

```achronyme
// Interface del iterador
{
    next: () => {value: T, done: Boolean}
}
```

#### Cambios Necesarios

**Grammar (`grammar.pest`)**:
```pest
// Ningún cambio necesario - los iteradores son solo records con método next
```

**AST (`ast.rs`)**:
```rust
// Ningún cambio en AST - iteradores son valores normales
```

**Value (`value.rs`)**:
```rust
// Potencialmente añadir tipo Iterator nativo para optimización
pub enum Value {
    // ... existing variants

    // Optional: Native iterator for performance
    Iterator(Box<dyn Iterator<Item = Value>>),
}
```

**Built-in Functions**:
```achronyme
// Crear iterador manual
let fibonacci = () => do {
    let state = {mut a: 0, mut b: 1}
    {
        next: () => do {
            let current = state.a
            state.a = state.b
            state.b = current + state.b
            {value: current, done: false}
        }
    }
}

// Helpers
let take = (n, iterator) => do {
    let result = []
    let mut i = 0
    while(i < n) {
        let item = iterator.next()
        if(item.done) { break }
        result = push(result, item.value)
        i = i + 1
    }
    result
}

let collect = (iterator) => do {
    let result = []
    while(true) {
        let item = iterator.next()
        if(item.done) { break }
        result = push(result, item.value)
    }
    result
}
```

### 1.2 Generator Functions (yield)

Funciones que pueden pausarse y retornar múltiples valores.

#### Sintaxis Propuesta

```achronyme
let range = (start, end) => generate {
    let mut i = start
    while(i < end) {
        yield i
        i = i + 1
    }
}

// Uso
let iter = range(0, 10)
for(x in iter) {
    print(x)  // 0, 1, 2, ..., 9
}
```

#### Cambios Necesarios

**Grammar**:
```pest
// Yield statement
yield_statement = {
    "yield" ~ expr
}

// Generate block (generator function body)
generate_block = {
    "generate" ~ block
}

// Actualizar statement
statement = {
    import_statement
  | export_statement
  | let_statement
  | mut_statement
  | return_statement
  | yield_statement    // NUEVO
  | assignment
  | expr
}

// Actualizar lambda_body
lambda_body = {
    generate_block | do_block | expr  // NUEVO generate_block
}
```

**AST**:
```rust
pub enum AstNode {
    // ... existing variants

    /// Yield statement: yield expr
    Yield {
        value: Box<AstNode>,
    },

    /// Generate block: generate { statements }
    GenerateBlock {
        statements: Vec<AstNode>,
    },
}
```

**Value**:
```rust
pub enum Value {
    // ... existing variants

    /// Generator: suspended function that can be resumed
    Generator(Rc<RefCell<GeneratorState>>),
}

pub struct GeneratorState {
    /// The generator's stack frame (captured scope)
    pub env: Environment,

    /// Current execution position (statement index)
    pub position: usize,

    /// Statements in the generator
    pub statements: Vec<AstNode>,

    /// Is the generator exhausted?
    pub done: bool,

    /// Last yielded value
    pub last_value: Option<Value>,
}
```

**Evaluator**:
```rust
impl Evaluator {
    // Crear generator
    fn evaluate_generate_block(&mut self, statements: &[AstNode]) -> Result<Value, String> {
        let captured_env = self.env.clone();

        Ok(Value::Generator(Rc::new(RefCell::new(GeneratorState {
            env: captured_env,
            position: 0,
            statements: statements.to_vec(),
            done: false,
            last_value: None,
        }))))
    }

    // Resumir generator (llamado por .next())
    fn resume_generator(&mut self, gen: &Rc<RefCell<GeneratorState>>)
        -> Result<Value, String>
    {
        let mut state = gen.borrow_mut();

        if state.done {
            return Ok(record!{
                "value" => Value::Null,
                "done" => Value::Boolean(true)
            });
        }

        // Restaurar el entorno del generator
        let saved_env = std::mem::replace(&mut self.env, state.env.clone());

        // Ejecutar hasta el próximo yield
        let result = self.execute_until_yield(&mut state);

        // Guardar el entorno actualizado
        state.env = std::mem::replace(&mut self.env, saved_env);

        result
    }

    fn execute_until_yield(&mut self, state: &mut GeneratorState)
        -> Result<Value, String>
    {
        while state.position < state.statements.len() {
            let stmt = &state.statements[state.position];
            state.position += 1;

            match stmt {
                AstNode::Yield { value } => {
                    let yielded = self.evaluate(value)?;
                    state.last_value = Some(yielded.clone());

                    return Ok(record!{
                        "value" => yielded,
                        "done" => Value::Boolean(false)
                    });
                }

                AstNode::Return { value } => {
                    let returned = self.evaluate(value)?;
                    state.done = true;

                    return Ok(record!{
                        "value" => returned,
                        "done" => Value::Boolean(true)
                    });
                }

                _ => {
                    // Ejecutar statement normal
                    self.evaluate(stmt)?;
                }
            }
        }

        // Generator terminó sin yield/return explícito
        state.done = true;
        Ok(record!{
            "value" => Value::Null,
            "done" => Value::Boolean(true)
        })
    }
}
```

### 1.3 For-In Loop

Sintaxis para iterar sobre iteradores:

```achronyme
for(x in iterator) {
    // body
}
```

**Grammar**:
```pest
for_loop = {
    "for" ~ "(" ~ identifier ~ "in" ~ expr ~ ")" ~ block
}

control_flow_expr = {
    if_expr
  | for_loop  // NUEVO
}
```

**AST**:
```rust
pub enum AstNode {
    // ... existing

    ForLoop {
        variable: String,
        iterable: Box<AstNode>,
        body: Box<AstNode>,
    },
}
```

### Tests Fase 1

```rust
// tests/test_generators.rs

#[test]
fn test_manual_iterator() {
    let code = r#"
        let fib = () => do {
            let state = {mut a: 0, mut b: 1}
            {
                next: () => do {
                    let current = state.a
                    state.a = state.b
                    state.b = current + state.b
                    {value: current, done: false}
                }
            }
        }

        let iter = fib()
        let a = iter.next().value
        let b = iter.next().value
        let c = iter.next().value
        [a, b, c]
    "#;

    let result = eval(code).unwrap();
    assert_eq!(result, vector![0.0, 1.0, 1.0]);
}

#[test]
fn test_generator_yield() {
    let code = r#"
        let range = (n) => generate {
            let mut i = 0
            while(i < n) {
                yield i
                i = i + 1
            }
        }

        let gen = range(3)
        [gen.next().value, gen.next().value, gen.next().value]
    "#;

    let result = eval(code).unwrap();
    assert_eq!(result, vector![0.0, 1.0, 2.0]);
}

#[test]
fn test_for_in_loop() {
    let code = r#"
        let range = (n) => generate {
            let mut i = 0
            while(i < n) {
                yield i
                i = i + 1
            }
        }

        let mut sum = 0
        for(x in range(5)) {
            sum = sum + x
        }
        sum
    "#;

    let result = eval(code).unwrap();
    assert_eq!(result, Value::Number(10.0)); // 0+1+2+3+4
}
```

---

## Fase 2: Event Loop y Futures

**Duración estimada**: 3-4 semanas
**Dependencias**: Fase 1 completada
**Propósito**: Runtime asíncrono basado en Tokio

### 2.1 Integrar Tokio Runtime

**Cargo.toml**:
```toml
[dependencies]
tokio = { version = "1.35", features = ["full"] }
futures = "0.3"
```

**Evaluator con Runtime**:
```rust
use tokio::runtime::Runtime;

pub struct Evaluator {
    pub env: Environment,
    pub runtime: Runtime,  // NUEVO
    // ... other fields
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
            runtime: Runtime::new().expect("Failed to create Tokio runtime"),
            // ... other fields
        }
    }

    pub fn block_on<F>(&self, future: F) -> F::Output
    where
        F: std::future::Future,
    {
        self.runtime.block_on(future)
    }
}
```

### 2.2 Future Type

**Value**:
```rust
use tokio::sync::oneshot;

pub enum Value {
    // ... existing

    /// Future: asynchronous computation
    Future(Arc<Mutex<FutureState>>),
}

pub enum FutureState {
    /// Future is pending
    Pending {
        receiver: oneshot::Receiver<Value>,
    },

    /// Future completed successfully
    Resolved(Value),

    /// Future failed with error
    Rejected(String),
}

impl Value {
    pub fn is_future(&self) -> bool {
        matches!(self, Value::Future(_))
    }

    pub fn as_future(&self) -> Option<&Arc<Mutex<FutureState>>> {
        match self {
            Value::Future(f) => Some(f),
            _ => None,
        }
    }
}
```

### 2.3 Built-in Async Functions

**Módulo `async`**:
```rust
// crates/achronyme-eval/src/modules/async_module.rs

use tokio::time::{sleep, Duration};

pub fn register_async_functions(registry: &mut FunctionRegistry) {
    // sleep(ms) - Async sleep
    registry.register("sleep", |args| {
        if args.len() != 1 {
            return Err("sleep requires 1 argument".to_string());
        }

        let ms = args[0].as_number()
            .ok_or("sleep requires a number")?;

        let (sender, receiver) = oneshot::channel();

        tokio::spawn(async move {
            sleep(Duration::from_millis(ms as u64)).await;
            let _ = sender.send(Value::Null);
        });

        Ok(Value::Future(Arc::new(Mutex::new(FutureState::Pending { receiver }))))
    });

    // spawn(fn) - Run function in background
    registry.register("spawn", |args| {
        if args.len() != 1 {
            return Err("spawn requires 1 argument".to_string());
        }

        let func = args[0].clone();
        let (sender, receiver) = oneshot::channel();

        tokio::spawn(async move {
            // Execute function in background
            let result = apply_function(&func, &[])?;
            let _ = sender.send(result);
            Ok::<(), String>(())
        });

        Ok(Value::Future(Arc::new(Mutex::new(FutureState::Pending { receiver }))))
    });

    // Promise.all([futures]) - Wait for all futures
    registry.register("Promise.all", |args| {
        // Implementation
    });

    // Promise.race([futures]) - Wait for first future
    registry.register("Promise.race", |args| {
        // Implementation
    });
}
```

### 2.4 Await Manual (sin syntax)

Antes de implementar `await` keyword, permitir `.await()` como método:

```achronyme
// Manual await
let future = sleep(1000)
let result = future.await()  // Bloquea hasta que complete
```

**Implementation**:
```rust
// En el evaluador, cuando se llama .await() en un Future
fn await_future(&mut self, future: &Arc<Mutex<FutureState>>) -> Result<Value, String> {
    let mut state = future.lock().unwrap();

    match &mut *state {
        FutureState::Resolved(value) => Ok(value.clone()),
        FutureState::Rejected(error) => Err(error.clone()),
        FutureState::Pending { receiver } => {
            // Block until future completes
            let value = self.runtime.block_on(async {
                receiver.await.map_err(|_| "Future cancelled".to_string())
            })?;

            *state = FutureState::Resolved(value.clone());
            Ok(value)
        }
    }
}
```

### Tests Fase 2

```rust
#[test]
fn test_sleep_future() {
    let code = r#"
        let start = now()
        let future = sleep(100)
        future.await()
        let elapsed = now() - start
        elapsed >= 100
    "#;

    let result = eval(code).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_spawn_background() {
    let code = r#"
        let task = spawn(() => do {
            sleep(50).await()
            42
        })

        task.await()
    "#;

    let result = eval(code).unwrap();
    assert_eq!(result, Value::Number(42.0));
}

#[test]
fn test_promise_all() {
    let code = r#"
        let tasks = [
            spawn(() => 1),
            spawn(() => 2),
            spawn(() => 3)
        ]

        let results = Promise.all(tasks).await()
        sum(results)
    "#;

    let result = eval(code).unwrap();
    assert_eq!(result, Value::Number(6.0));
}
```

---

## Fase 3: Async/Await Syntax

**Duración estimada**: 4-5 semanas
**Dependencias**: Fase 2 completada
**Propósito**: Syntax ergonómico para async

### 3.1 Async Functions

**Grammar**:
```pest
// Async keyword
async_keyword = @{ "async" ~ !ASCII_ALPHANUMERIC }

// Async lambda
async_lambda = {
    async_keyword ~ lambda_params ~ "=>" ~ lambda_body
}

// Actualizar primary para incluir async_lambda
primary = {
    boolean
  | string_literal
  | complex
  | number
  | control_flow_expr
  | record
  | array
  | do_block
  | async_lambda  // NUEVO
  | lambda
  | self_ref
  | rec_ref
  | identifier
  | "(" ~ expr ~ ")"
}

// Await expression
await_expr = {
    "await" ~ expr
}

// Actualizar unary para incluir await
unary = {
    "-" ~ unary
  | "!" ~ unary
  | await_expr  // NUEVO
  | power
}
```

**AST**:
```rust
pub enum AstNode {
    // ... existing

    /// Async lambda: async (x) => body
    AsyncLambda {
        params: Vec<String>,
        body: Box<AstNode>,
    },

    /// Await expression: await expr
    Await {
        future: Box<AstNode>,
    },
}
```

**Desugaring Strategy**:

```achronyme
// Source code
let fetch = async (url) => do {
    let response = await http_get(url)
    let data = await response.json()
    data
}

// Desugared to
let fetch = (url) => Future.new((resolve, reject) => do {
    // Transform to generator that yields on each await
    spawn(async_generator(() => do {
        let response = yield http_get(url)
        let data = yield response.json()
        resolve(data)
    }))
})
```

**Implementation**:
```rust
impl Evaluator {
    fn evaluate_async_lambda(
        &mut self,
        params: &[String],
        body: &AstNode
    ) -> Result<Value, String> {
        // Capture current environment
        let captured_env = self.env.clone();

        // Create async function that returns a Future
        let async_fn = move |args: Vec<Value>| {
            // Create new evaluator with captured env
            let mut eval = Evaluator::with_env(captured_env.clone());

            // Bind parameters
            for (param, arg) in params.iter().zip(args) {
                eval.env.define(param, arg);
            }

            // Execute body, transforming awaits into yields
            let (sender, receiver) = oneshot::channel();

            tokio::spawn(async move {
                let result = eval.evaluate_async_body(body).await;
                let _ = sender.send(result);
            });

            Ok(Value::Future(Arc::new(Mutex::new(
                FutureState::Pending { receiver }
            ))))
        };

        Ok(Value::Function(Rc::new(async_fn)))
    }

    async fn evaluate_async_body(&mut self, body: &AstNode) -> Result<Value, String> {
        match body {
            AstNode::Await { future } => {
                // Evaluate the future expression
                let future_value = self.evaluate(future)?;

                // Wait for it to complete
                self.await_future_async(future_value).await
            }

            AstNode::DoBlock { statements } => {
                let mut result = None;

                for stmt in statements {
                    result = Some(self.evaluate_async_stmt(stmt).await?);
                }

                result.ok_or("Empty do block".to_string())
            }

            _ => self.evaluate(body)
        }
    }

    async fn await_future_async(&self, future: Value) -> Result<Value, String> {
        match future {
            Value::Future(state) => {
                let mut guard = state.lock().unwrap();

                match &mut *guard {
                    FutureState::Resolved(v) => Ok(v.clone()),
                    FutureState::Rejected(e) => Err(e.clone()),
                    FutureState::Pending { receiver } => {
                        // Actually await the future
                        let result = receiver.await
                            .map_err(|_| "Future cancelled".to_string())?;

                        *guard = FutureState::Resolved(result.clone());
                        Ok(result)
                    }
                }
            }
            _ => Err("await requires a Future".to_string())
        }
    }
}
```

### 3.2 Async Blocks

```achronyme
// Async block expression
let task = async do {
    let a = await fetch("api.com/a")
    let b = await fetch("api.com/b")
    a + b
}

let result = await task
```

**Grammar**:
```pest
async_block = {
    async_keyword ~ do_block
}

// Añadir a primary
primary = {
    // ...
  | async_block  // NUEVO
    // ...
}
```

### 3.3 Error Handling

```achronyme
// Try-catch para async
let safe_fetch = async (url) => do {
    try {
        let data = await http_get(url)
        {ok: true, data: data}
    } catch(error) {
        {ok: false, error: error}
    }
}
```

**Grammar**:
```pest
try_catch = {
    "try" ~ block ~ "catch" ~ "(" ~ identifier ~ ")" ~ block
}
```

### Tests Fase 3

```rust
#[test]
fn test_async_function() {
    let code = r#"
        let fetch = async (delay) => do {
            await sleep(delay)
            42
        }

        await fetch(10)
    "#;

    let result = eval(code).unwrap();
    assert_eq!(result, Value::Number(42.0));
}

#[test]
fn test_async_sequential() {
    let code = r#"
        let process = async () => do {
            let a = await sleep(10).then(() => 1)
            let b = await sleep(10).then(() => 2)
            a + b
        }

        await process()
    "#;

    let result = eval(code).unwrap();
    assert_eq!(result, Value::Number(3.0));
}

#[test]
fn test_async_parallel() {
    let code = r#"
        let parallel = async () => do {
            let tasks = [
                async do { await sleep(10); 1 },
                async do { await sleep(10); 2 },
                async do { await sleep(10); 3 }
            ]

            let results = await Promise.all(tasks)
            sum(results)
        }

        await parallel()
    "#;

    let result = eval(code).unwrap();
    assert_eq!(result, Value::Number(6.0));
}
```

---

## Fase 4: Concurrency Primitives

**Duración estimada**: 2-3 semanas
**Dependencias**: Fase 3 completada
**Propósito**: Primitivas para sincronización segura

### 4.1 Channels (MPSC)

```achronyme
let (sender, receiver) = channel()

// Producer
spawn(async () => do {
    for(i in range(0, 10)) {
        await sender.send(i)
    }
    sender.close()
})

// Consumer
spawn(async () => do {
    while(true) {
        let msg = await receiver.recv()
        if(msg.done) { break }
        print(msg.value)
    }
})
```

**Implementation**:
```rust
use tokio::sync::mpsc;

pub enum Value {
    // ... existing

    Sender(Arc<Mutex<mpsc::UnboundedSender<Value>>>),
    Receiver(Arc<Mutex<mpsc::UnboundedReceiver<Value>>>),
}

// Built-in function
registry.register("channel", |args| {
    let (tx, rx) = mpsc::unbounded_channel();

    Ok(Value::Array(vec![
        Value::Sender(Arc::new(Mutex::new(tx))),
        Value::Receiver(Arc::new(Mutex::new(rx))),
    ]))
});
```

### 4.2 Async Mutex

```achronyme
let counter = AsyncMutex.new(0)

let increment = async () => do {
    let guard = await counter.lock()
    guard.value = guard.value + 1
    // Auto-unlock on scope exit
}
```

**Implementation**:
```rust
use tokio::sync::Mutex as TokioMutex;

pub enum Value {
    // ... existing

    AsyncMutex(Arc<TokioMutex<Value>>),
    MutexGuard {
        inner: Arc<TokioMutex<Value>>,
        // Guard automatically unlocks on drop
    },
}
```

### 4.3 Select Expression

```achronyme
select {
    case(value, receiver1.recv()) => {
        print("From receiver1: " + value)
    }
    case(value, receiver2.recv()) => {
        print("From receiver2: " + value)
    }
    timeout(1000) => {
        print("Timeout!")
    }
}
```

**Grammar**:
```pest
select_expr = {
    "select" ~ "{" ~ select_case+ ~ "}"
}

select_case = {
    "case" ~ "(" ~ identifier ~ "," ~ expr ~ ")" ~ "=>" ~ block
  | "timeout" ~ "(" ~ expr ~ ")" ~ "=>" ~ block
}
```

### Tests Fase 4

```rust
#[test]
fn test_channel_communication() {
    let code = r#"
        let (tx, rx) = channel()

        spawn(async () => do {
            await tx.send(42)
        })

        let msg = await rx.recv()
        msg.value
    "#;

    let result = eval(code).unwrap();
    assert_eq!(result, Value::Number(42.0));
}

#[test]
fn test_async_mutex() {
    let code = r#"
        let counter = AsyncMutex.new(0)

        let tasks = map(
            (i) => async do {
                let guard = await counter.lock()
                guard.value = guard.value + 1
            },
            range(0, 100)
        )

        await Promise.all(tasks)

        let final_guard = await counter.lock()
        final_guard.value
    "#;

    let result = eval(code).unwrap();
    assert_eq!(result, Value::Number(100.0));
}
```

---

## Fase 5: Sistema Reactivo

**Duración estimada**: 3-4 semanas
**Dependencias**: Fase 4 completada
**Propósito**: Signals y auto-tracking para GUIs

### 5.1 Reactive Signals

**Conceptos**:
- **Signal**: Valor reactivo que notifica cambios
- **Computed**: Valor derivado que se recalcula automáticamente
- **Effect**: Side-effect que se re-ejecuta cuando sus dependencias cambian

```achronyme
let count = signal(0)
let doubled = computed(() => count.value * 2)

effect(() => do {
    print("Count: " + count.value)
    print("Doubled: " + doubled.value)
})

count.set(5)  // Dispara el effect
// Output: "Count: 5"
//         "Doubled: 10"
```

**Implementation Overview**:
```rust
pub struct Signal<T> {
    value: Arc<RwLock<T>>,
    subscribers: Arc<RwLock<Vec<Weak<dyn Subscriber>>>>,
}

pub struct Computed<T> {
    signal: Signal<T>,
    dependencies: Vec<Signal<dyn Any>>,
    compute_fn: Rc<dyn Fn() -> T>,
}

pub struct Effect {
    effect_fn: Rc<dyn Fn()>,
    dependencies: Arc<RwLock<Vec<Signal<dyn Any>>>>,
}

// Auto-tracking: record which signals are read during effect execution
thread_local! {
    static TRACKING_CONTEXT: RefCell<Option<Vec<Signal<dyn Any>>>> = RefCell::new(None);
}
```

### 5.2 Batched Updates

```achronyme
// Sin batching: 3 re-renders
count.set(1)  // render
count.set(2)  // render
count.set(3)  // render

// Con batching: 1 re-render
batch(() => do {
    count.set(1)
    count.set(2)
    count.set(3)
})  // Single render with final state
```

**Implementation**:
```rust
thread_local! {
    static BATCHING: Cell<bool> = Cell::new(false);
    static PENDING_EFFECTS: RefCell<Vec<Rc<dyn Fn()>>> = RefCell::new(Vec::new());
}

pub fn batch<F>(f: F)
where
    F: FnOnce(),
{
    BATCHING.with(|b| b.set(true));
    f();
    BATCHING.with(|b| b.set(false));

    // Flush pending effects
    PENDING_EFFECTS.with(|effects| {
        for effect in effects.borrow_mut().drain(..) {
            effect();
        }
    });
}
```

### 5.3 Async Effects

```achronyme
let user_id = signal(1)
let user = signal(null)

async_effect(async () => do {
    let data = await fetch("/api/users/" + user_id.value)
    user.set(data)
}, [user_id])  // Re-run when user_id changes
```

### Tests Fase 5

```rust
#[test]
fn test_signal_reactivity() {
    let code = r#"
        let count = signal(0)
        let mut effect_count = 0

        effect(() => do {
            let _ = count.value  // Track dependency
            effect_count = effect_count + 1
        })

        count.set(1)
        count.set(2)

        effect_count
    "#;

    let result = eval(code).unwrap();
    assert_eq!(result, Value::Number(3.0)); // Initial + 2 updates
}

#[test]
fn test_computed_signal() {
    let code = r#"
        let count = signal(5)
        let doubled = computed(() => count.value * 2)

        let initial = doubled.value
        count.set(10)
        let updated = doubled.value

        [initial, updated]
    "#;

    let result = eval(code).unwrap();
    assert_eq!(result, vector![10.0, 20.0]);
}
```

---

## Fase 6: GUI Bindings Nativos

**Duración estimada**: 6-8 semanas
**Dependencias**: Fase 5 completada
**Propósito**: Conectar con toolkits GUI nativos

### 6.1 Backend Abstracto

**Arquitectura**:
```
┌─────────────────────────────────────────────┐
│         Achronyme GUI API (Abstract)        │
│  Widget, Window, Button, Label, etc.        │
└─────────────────────────────────────────────┘
                    │
        ┌───────────┼───────────┬───────────┐
        ▼           ▼           ▼           ▼
    ┌───────┐  ┌───────┐  ┌────────┐  ┌────────┐
    │  GTK  │  │  Qt   │  │  Iced  │  │ Tauri  │
    │Backend│  │Backend│  │Backend │  │Backend │
    └───────┘  └───────┘  └────────┘  └────────┘
```

**API Example**:
```achronyme
import { Window, Button, Label, VBox } from "gui"

let app = async () => do {
    let window = Window.new({
        title: "Counter App",
        width: 400,
        height: 300
    })

    let count = signal(0)

    window.mount(
        VBox(spacing: 10, padding: 20) {
            Label(text: computed(() => "Count: " + count.value))
            Button(
                text: "Increment",
                on_click: () => count.set(count.value + 1)
            )
            Button(
                text: "Async Fetch",
                on_click: async () => do {
                    let data = await fetch("api.com/data")
                    count.set(data.value)
                }
            )
        }
    )

    await window.run()
}

await app()
```

### 6.2 GTK Backend (Primera Implementación)

**Cargo.toml**:
```toml
[dependencies]
gtk4 = "0.7"
glib = "0.18"
```

**Implementation**:
```rust
// crates/achronyme-gui-gtk/src/lib.rs

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button, Box as GtkBox, Label};

pub struct GtkBackend {
    app: Application,
}

impl GtkBackend {
    pub fn new() -> Self {
        let app = Application::builder()
            .application_id("com.achronyme.app")
            .build();

        Self { app }
    }

    pub fn create_window(&self, title: &str, width: i32, height: i32) -> GtkWindow {
        let window = ApplicationWindow::builder()
            .application(&self.app)
            .title(title)
            .default_width(width)
            .default_height(height)
            .build();

        GtkWindow { window }
    }
}

pub struct GtkWindow {
    window: ApplicationWindow,
}

impl GtkWindow {
    pub fn mount(&self, widget: GtkWidget) {
        self.window.set_child(Some(&widget.native));
    }

    pub async fn run(&self) {
        self.window.present();
        // Run GTK event loop
    }
}
```

### 6.3 Reactive Binding

Conectar signals de Achronyme con widgets GTK:

```rust
// Cuando un signal cambia, actualizar el widget
signal.subscribe(|new_value| {
    gtk_label.set_text(&new_value.to_string());
});

// Cuando el widget cambia, actualizar el signal
gtk_button.connect_clicked(|_| {
    signal.set(new_value);
});
```

### Tests Fase 6

```rust
#[test]
fn test_window_creation() {
    let code = r#"
        import { Window } from "gui"

        let window = Window.new({
            title: "Test",
            width: 400,
            height: 300
        })

        window.title
    "#;

    let result = eval(code).unwrap();
    assert_eq!(result, Value::String("Test".to_string()));
}

#[test]
fn test_reactive_label() {
    let code = r#"
        import { Label } from "gui"

        let count = signal(0)
        let label = Label(text: computed(() => "Count: " + count.value))

        let initial = label.text
        count.set(5)
        let updated = label.text

        [initial, updated]
    "#;

    let result = eval(code).unwrap();
    // Verify reactivity works
}
```

---

## Timeline y Dependencias

### Diagrama de Dependencias

```
Fase 1: Iteradores/Generators (2-3 semanas)
    │
    ▼
Fase 2: Event Loop/Futures (3-4 semanas)
    │
    ▼
Fase 3: Async/Await Syntax (4-5 semanas)
    │
    ▼
Fase 4: Concurrency Primitives (2-3 semanas)
    │
    ▼
Fase 5: Sistema Reactivo (3-4 semanas)
    │
    ▼
Fase 6: GUI Bindings (6-8 semanas)
```

### Timeline Total

| Fase | Duración | Inicio | Fin |
|------|----------|--------|-----|
| 1    | 3 sem    | Sem 1  | Sem 3 |
| 2    | 4 sem    | Sem 4  | Sem 7 |
| 3    | 5 sem    | Sem 8  | Sem 12 |
| 4    | 3 sem    | Sem 13 | Sem 15 |
| 5    | 4 sem    | Sem 16 | Sem 19 |
| 6    | 8 sem    | Sem 20 | Sem 27 |

**Total**: ~27 semanas (~6-7 meses)

---

## Plan MVP

Para un **Minimum Viable Product** más rápido:

### MVP: Async + GUI Básico (3 meses)

**Incluye**:
1. ✅ Fase 2: Event Loop + Futures (solo lo esencial)
2. ✅ Fase 3: Async/Await Syntax (sin generators complejos)
3. ✅ Fase 5: Signals básicos (sin batching sofisticado)
4. ✅ Fase 6: Un solo backend GUI (GTK o Iced)

**Omite**:
- ❌ Generators complejos (Fase 1)
- ❌ Concurrency avanzada (Fase 4)
- ❌ Optimizaciones de performance

**Timeline MVP**: ~12 semanas (3 meses)

### Hello World GUI en MVP

```achronyme
import { Window, Button, Label, VBox } from "gui"

let main = async () => do {
    let window = Window.new({title: "Hello Achronyme", width: 300, height: 200})
    let count = signal(0)

    window.mount(
        VBox {
            Label(text: computed(() => "Clicks: " + count.value))
            Button(
                text: "Click me!",
                on_click: () => count.set(count.value + 1)
            )
        }
    )

    await window.run()
}

await main()
```

---

## Referencias

### Lenguajes para Inspiración

- **JavaScript/TypeScript**: async/await, Promises
- **Rust**: Tokio, async/await, futures
- **C#**: async/await, Tasks
- **Swift**: async/await, actors
- **SolidJS**: Signals, reactive system
- **Vue 3**: Composition API, reactivity

### Recursos Técnicos

- [Tokio Documentation](https://tokio.rs/)
- [async-book (Rust)](https://rust-lang.github.io/async-book/)
- [SolidJS Reactivity](https://www.solidjs.com/docs/latest/api#reactivity)
- [GTK4-rs](https://gtk-rs.org/)

### Documentos Relacionados

- `docs/design/native-gui-system.md`
- `docs/design/ui-system-architecture.md`
- `docs/design/ui-system-records-based.md`

---

## Siguiente Paso

**Acción inmediata**: Implementar **Fase 1: Iteradores y Generators**

Crear:
1. `docs/design/phase1-iterators-implementation.md` - Detalles de implementación
2. `crates/achronyme-parser/tests/test_generators.rs` - Tests
3. Modificar grammar para `yield`, `generate`
4. Implementar `GeneratorState` en evaluator

**Fecha objetivo Fase 1**: 3 semanas desde inicio
