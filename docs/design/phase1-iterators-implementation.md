# Fase 1: Iteradores y Generators - Plan de Implementación

**Versión**: 1.0
**Fecha**: 2025-01-14
**Estado**: Por implementar
**Duración estimada**: 2-3 semanas
**Prerequisito para**: Fase 2 (Event Loop y Futures)

## Tabla de Contenidos

- [Objetivos](#objetivos)
- [Motivación Técnica](#motivación-técnica)
- [Especificación de Iteradores](#especificación-de-iteradores)
- [Especificación de Generators](#especificación-de-generators)
- [Cambios en el Sistema](#cambios-en-el-sistema)
- [Plan de Implementación](#plan-de-implementación)
- [Tests Requeridos](#tests-requeridos)
- [Ejemplos de Uso](#ejemplos-de-uso)

---

## Objetivos

### Objetivos Principales

1. **Protocolo de Iteradores**: Definir y implementar un protocolo estándar para iteración
2. **Generator Functions**: Funciones que pueden pausarse (`yield`) y resumirse
3. **For-In Loops**: Sintaxis para iterar sobre iteradores
4. **Fundamento para Async**: Los generators son la base para async/await

### No-Objetivos (Fuera de Alcance)

- ❌ Async/await (Fase 3)
- ❌ Event loop (Fase 2)
- ❌ Concurrency (Fase 4)
- ❌ Optimizaciones de performance avanzadas

---

## Motivación Técnica

### Por Qué Iteradores Primero

Los iteradores son fundamentales porque:

1. **Lazy Evaluation**: Procesar datos bajo demanda, no todo en memoria
2. **Composabilidad**: Encadenar operaciones sin crear arrays intermedios
3. **Infinitas secuencias**: `fibonacci()` puede generar valores infinitamente
4. **Base para Async**: Un `async function` es básicamente un generator que yields futures

### Relación con Async/Await

```achronyme
// Generator (Fase 1)
let countdown = (n) => generate {
    let mut i = n
    while(i > 0) {
        yield i
        i = i - 1
    }
}

// Async function (Fase 3) - Conceptualmente similar
let fetchUsers = async (ids) => {
    for(id in ids) {
        let user = await fetch("/users/" + id)  // yield + await
        yield user  // Si soportáramos async generators
    }
}
```

**Clave**: Un `async function` es un generator que:
- Yields `Future` en lugar de valores
- `await expr` es azúcar sintáctico para `yield future; resume_with_result`

---

## Especificación de Iteradores

### Protocolo de Iterator

Un iterador es cualquier valor con un método `next()`:

```achronyme
{
    next: () => {value: T, done: Boolean}
}
```

**Contrato**:
- `next()` retorna un record con dos campos:
  - `value`: El próximo valor en la secuencia
  - `done`: `true` si no hay más valores, `false` si hay más
- Cuando `done` es `true`, `value` puede ser cualquier cosa (típicamente `null`)
- Llamar `next()` después de `done: true` debe seguir retornando `{done: true}`

### Ejemplo Manual

```achronyme
// Iterator manual (sin generator syntax)
let range = (start, end) => do {
    mut state = {mut current: start}

    {
        next: () => do {
            if(state.current >= end) {
                {value: null, done: true}
            } else {
                let value = state.current
                state.current = state.current + 1
                {value: value, done: false}
            }
        }
    }
}

// Uso
let iter = range(0, 3)
print(iter.next())  // {value: 0, done: false}
print(iter.next())  // {value: 1, done: false}
print(iter.next())  // {value: 2, done: false}
print(iter.next())  // {value: null, done: true}
```

### Built-in Iterator Helpers

Funciones utilitarias para trabajar con iteradores:

```achronyme
// Consumir n elementos
let take = (n, iterator) => do {
    mut result = []
    mut i = 0
    while(i < n) {
        let item = iterator.next()
        if(item.done) { break }
        result = push(result, item.value)
        i = i + 1
    }
    result
}

// Consumir todos los elementos
let collect = (iterator) => do {
    mut result = []
    while(true) {
        let item = iterator.next()
        if(item.done) { break }
        result = push(result, item.value)
    }
    result
}

// Map lazy
let map_iter = (f, iterator) => ({
    next: () => do {
        let item = iterator.next()
        if(item.done) {
            item
        } else {
            {value: f(item.value), done: false}
        }
    }
})

// Filter lazy
let filter_iter = (predicate, iterator) => ({
    next: () => do {
        while(true) {
            let item = iterator.next()
            if(item.done) { return item }
            if(predicate(item.value)) { return item }
        }
    }
})
```

---

## Especificación de Generators

### Sintaxis: `generate` Block

```achronyme
let <name> = (<params>) => generate {
    <statements>
    yield <expr>
    <more statements>
}
```

### Sintaxis: `yield` Statement

```achronyme
yield <expression>
```

**Semántica**:
- `yield expr` suspende la ejecución del generator
- El valor de `expr` se retorna en `{value: expr, done: false}`
- La próxima llamada a `next()` resume justo después del `yield`
- Las variables locales y el estado se preservan entre yields

### Ejemplo Básico

```achronyme
let countdown = (n) => generate {
    mut i = n
    while(i > 0) {
        yield i
        i = i - 1
    }
}

let gen = countdown(3)
print(gen.next())  // {value: 3, done: false}
print(gen.next())  // {value: 2, done: false}
print(gen.next())  // {value: 1, done: false}
print(gen.next())  // {value: null, done: true}
```

### Return en Generators

```achronyme
let example = () => generate {
    yield 1
    yield 2
    return 42  // Termina el generator con un valor final
}

let gen = example()
print(gen.next())  // {value: 1, done: false}
print(gen.next())  // {value: 2, done: false}
print(gen.next())  // {value: 42, done: true}
print(gen.next())  // {value: 42, done: true} - sticky
```

**Semántica de `return`**:
- `return expr` en un generator marca el generator como done
- El valor se retorna como `{value: expr, done: true}`
- Subsecuentes llamadas a `next()` retornan el mismo valor (sticky done)

### Generators Infinitos

```achronyme
let fibonacci = () => generate {
    mut a = 0
    mut b = 1
    while(true) {
        yield a
        let temp = a
        a = b
        b = temp + b
    }
}

// Uso con take
let first_10 = take(10, fibonacci())
print(first_10)  // [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
```

### Generators con Parámetros

```achronyme
let repeat = (value, times) => generate {
    mut i = 0
    while(i < times) {
        yield value
        i = i + 1
    }
}

let threes = repeat(3, 5)
print(collect(threes))  // [3, 3, 3, 3, 3]
```

---

## Cambios en el Sistema

### 1. Grammar (`grammar.pest`)

```pest
// ============================================================================
// Generators y Yield
// ============================================================================

// Yield statement
yield_statement = {
    "yield" ~ expr
}

// Generate block (generator function body)
generate_block = {
    "generate" ~ block
}

// Actualizar keywords
keyword = _{
    ("let" | "mut" | "rec" | "self" | "true" | "false" |
     "import" | "from" | "export" | "as" | "return" |
     "yield" | "generate") ~ !ASCII_ALPHANUMERIC
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

// ============================================================================
// For-In Loop
// ============================================================================

// For loop
for_loop = {
    "for" ~ "(" ~ identifier ~ "in" ~ expr ~ ")" ~ block
}

// Añadir a control_flow_expr
control_flow_expr = {
    if_expr
  | for_loop  // NUEVO
}
```

### 2. AST (`ast.rs`)

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

    /// For-in loop: for(x in iter) { body }
    ForLoop {
        variable: String,
        iterable: Box<AstNode>,
        body: Box<AstNode>,
    },
}
```

### 3. Value (`value.rs`)

```rust
use std::rc::Rc;
use std::cell::RefCell;

pub enum Value {
    // ... existing variants

    /// Generator: suspended function that can be resumed
    Generator(Rc<RefCell<GeneratorState>>),
}

/// State of a generator
pub struct GeneratorState {
    /// The generator's environment (captured scope)
    pub env: Environment,

    /// Current execution position (statement index)
    pub position: usize,

    /// Statements in the generator body
    pub statements: Vec<AstNode>,

    /// Is the generator exhausted?
    pub done: bool,

    /// Value returned by last `return` statement (sticky)
    pub return_value: Option<Value>,
}

impl Value {
    pub fn is_generator(&self) -> bool {
        matches!(self, Value::Generator(_))
    }

    pub fn as_generator(&self) -> Option<&Rc<RefCell<GeneratorState>>> {
        match self {
            Value::Generator(g) => Some(g),
            _ => None,
        }
    }
}
```

### 4. Parser (`pest_parser.rs`)

```rust
fn build_ast_from_statement(pair: Pair<Rule>) -> Result<AstNode, String> {
    match pair.as_rule() {
        // ... existing cases

        Rule::yield_statement => {
            let mut inner = pair.into_inner();
            let value = inner.next()
                .ok_or("Missing value in yield statement")?;
            Ok(AstNode::Yield {
                value: Box::new(build_ast_from_expr(value)?),
            })
        }

        // ... other cases
    }
}

fn build_lambda_body(pair: Pair<Rule>) -> Result<AstNode, String> {
    match pair.as_rule() {
        Rule::generate_block => {
            let inner = pair.into_inner().next()
                .ok_or("Missing block in generate")?;
            let statements = build_block_statements(inner)?;
            Ok(AstNode::GenerateBlock { statements })
        }

        Rule::do_block => {
            // ... existing
        }

        _ => build_ast_from_expr(pair)
    }
}

fn build_control_flow(pair: Pair<Rule>) -> Result<AstNode, String> {
    match pair.as_rule() {
        Rule::if_expr => {
            // ... existing
        }

        Rule::for_loop => {
            let mut inner = pair.into_inner();

            let variable = inner.next()
                .ok_or("Missing variable in for loop")?
                .as_str()
                .to_string();

            let iterable = inner.next()
                .ok_or("Missing iterable in for loop")?;

            let body = inner.next()
                .ok_or("Missing body in for loop")?;

            Ok(AstNode::ForLoop {
                variable,
                iterable: Box::new(build_ast_from_expr(iterable)?),
                body: Box::new(build_ast_from_statement(body)?),
            })
        }

        _ => Err(format!("Unknown control flow: {:?}", pair.as_rule()))
    }
}
```

### 5. Evaluator (`evaluator.rs`)

```rust
impl Evaluator {
    pub fn evaluate(&mut self, node: &AstNode) -> Result<Value, String> {
        match node {
            // ... existing cases

            AstNode::GenerateBlock { statements } => {
                self.evaluate_generate_block(statements)
            }

            AstNode::Yield { .. } => {
                Err("yield can only be used inside a generator".to_string())
            }

            AstNode::ForLoop { variable, iterable, body } => {
                self.evaluate_for_loop(variable, iterable, body)
            }

            // ... other cases
        }
    }

    fn evaluate_generate_block(
        &mut self,
        statements: &[AstNode]
    ) -> Result<Value, String> {
        // Capture current environment
        let captured_env = self.env.clone();

        // Create generator state
        let state = GeneratorState {
            env: captured_env,
            position: 0,
            statements: statements.to_vec(),
            done: false,
            return_value: None,
        };

        // Return generator value with .next() method
        let gen_rc = Rc::new(RefCell::new(state));
        Ok(Value::Generator(gen_rc))
    }

    fn evaluate_for_loop(
        &mut self,
        variable: &str,
        iterable: &AstNode,
        body: &AstNode
    ) -> Result<Value, String> {
        // Evaluate iterable
        let iter_value = self.evaluate(iterable)?;

        // Check if it has a next() method
        let iter_record = match &iter_value {
            Value::Record(map) => map,
            _ => return Err("for-in requires an iterable (object with next method)".to_string())
        };

        let next_fn = iter_record.get("next")
            .ok_or("Iterable must have a 'next' method")?
            .clone();

        // Create new scope for loop
        self.env.push_scope();

        let mut last_value = Value::Null;

        loop {
            // Call next()
            let result = self.apply_function(&next_fn, &[])?;

            // Check if it's a valid iterator result
            let result_record = match &result {
                Value::Record(map) => map,
                _ => return Err("next() must return {value, done}".to_string())
            };

            let done = result_record.get("done")
                .and_then(|v| v.as_boolean())
                .ok_or("next() must return {done: Boolean}")?;

            if done {
                break;
            }

            let value = result_record.get("value")
                .ok_or("next() must return {value: T}")?
                .clone();

            // Bind loop variable
            self.env.define(variable, value);

            // Execute body
            last_value = self.evaluate(body)?;

            // Check for early return
            if matches!(last_value, Value::EarlyReturn(_)) {
                self.env.pop_scope();
                return Ok(last_value);
            }
        }

        self.env.pop_scope();
        Ok(last_value)
    }
}
```

### 6. Generator Execution (Built-in `next` method)

Cuando se accede a `generator.next()`, necesitamos ejecutar el generator:

```rust
// En el handler de field access
impl Evaluator {
    fn handle_generator_method_call(
        &mut self,
        gen: &Rc<RefCell<GeneratorState>>,
        method: &str,
        args: &[Value]
    ) -> Result<Value, String> {
        match method {
            "next" => {
                if !args.is_empty() {
                    return Err("next() takes no arguments".to_string());
                }
                self.resume_generator(gen)
            }
            _ => Err(format!("Generators don't have method: {}", method))
        }
    }

    fn resume_generator(
        &mut self,
        gen: &Rc<RefCell<GeneratorState>>
    ) -> Result<Value, String> {
        let mut state = gen.borrow_mut();

        // If already done, return sticky value
        if state.done {
            return Ok(self.make_iterator_result(
                state.return_value.clone().unwrap_or(Value::Null),
                true
            ));
        }

        // Restore generator's environment
        let saved_env = std::mem::replace(&mut self.env, state.env.clone());

        // Execute until yield or end
        let result = self.execute_until_yield(&mut state);

        // Save updated environment
        state.env = std::mem::replace(&mut self.env, saved_env);

        result
    }

    fn execute_until_yield(
        &mut self,
        state: &mut GeneratorState
    ) -> Result<Value, String> {
        while state.position < state.statements.len() {
            let stmt = &state.statements[state.position].clone();
            state.position += 1;

            match stmt {
                AstNode::Yield { value } => {
                    // Evaluate and yield value
                    let yielded = self.evaluate(&value)?;
                    return Ok(self.make_iterator_result(yielded, false));
                }

                AstNode::Return { value } => {
                    // Generator returns (done)
                    let returned = self.evaluate(&value)?;
                    state.done = true;
                    state.return_value = Some(returned.clone());
                    return Ok(self.make_iterator_result(returned, true));
                }

                _ => {
                    // Execute normal statement
                    self.evaluate(&stmt)?;
                }
            }
        }

        // Generator exhausted naturally (no explicit return)
        state.done = true;
        state.return_value = Some(Value::Null);
        Ok(self.make_iterator_result(Value::Null, true))
    }

    fn make_iterator_result(&self, value: Value, done: bool) -> Value {
        let mut map = std::collections::HashMap::new();
        map.insert("value".to_string(), value);
        map.insert("done".to_string(), Value::Boolean(done));
        Value::Record(Rc::new(map))
    }
}
```

---

## Plan de Implementación

### Semana 1: Infraestructura Básica

**Día 1-2: Grammar y Parser**
- [ ] Añadir `yield` y `generate` a keywords
- [ ] Implementar reglas `yield_statement` y `generate_block`
- [ ] Actualizar `lambda_body` para soportar `generate_block`
- [ ] Tests: Parsear generators simples

**Día 3-4: AST y Value**
- [ ] Añadir variantes `Yield` y `GenerateBlock` al AST
- [ ] Implementar `Value::Generator` y `GeneratorState`
- [ ] Actualizar pattern matching exhaustivo
- [ ] Tests: Crear valores generator

**Día 5: Build y Validación**
- [ ] Asegurar que todo compile sin warnings
- [ ] Tests de integración básicos
- [ ] Documentar cambios en CHANGELOG

### Semana 2: Ejecución de Generators

**Día 1-2: Evaluator - Create Generators**
- [ ] Implementar `evaluate_generate_block()`
- [ ] Capturar environment en generator state
- [ ] Tests: Crear generators sin ejecutarlos

**Día 3-5: Evaluator - Resume Generators**
- [ ] Implementar `resume_generator()`
- [ ] Implementar `execute_until_yield()`
- [ ] Manejar `yield` correctamente
- [ ] Manejar `return` en generators
- [ ] Preservar estado entre yields
- [ ] Tests: Ejecutar generators simples

### Semana 3: For-In y Pulimiento

**Día 1-2: For-In Loop**
- [ ] Grammar para `for_loop`
- [ ] AST `ForLoop` node
- [ ] Implementar `evaluate_for_loop()`
- [ ] Validar protocolo iterator en runtime
- [ ] Tests: for-in con generators

**Día 3: Built-in Helpers**
- [ ] Implementar `take()` en Achronyme
- [ ] Implementar `collect()` en Achronyme
- [ ] Implementar `map_iter()` en Achronyme
- [ ] Implementar `filter_iter()` en Achronyme
- [ ] Tests: Composición de iteradores

**Día 4-5: Tests Comprehensivos y Docs**
- [ ] Suite completa de tests (ver sección Tests)
- [ ] Benchmark de performance
- [ ] Documentar en `docs/language/`
- [ ] Ejemplos en `examples/generators/`
- [ ] Actualizar CHANGELOG y README

---

## Tests Requeridos

### Tests de Parser

```rust
// tests/test_generator_parsing.rs

#[test]
fn test_parse_yield_statement() {
    let code = "yield 42";
    let ast = parse(code).unwrap();
    // Verify Yield node
}

#[test]
fn test_parse_generate_block() {
    let code = r#"
        let gen = () => generate {
            yield 1
            yield 2
        }
    "#;
    let ast = parse(code).unwrap();
    // Verify GenerateBlock node
}

#[test]
fn test_parse_for_loop() {
    let code = r#"
        for(x in iterator) {
            print(x)
        }
    "#;
    let ast = parse(code).unwrap();
    // Verify ForLoop node
}
```

### Tests de Evaluación

```rust
// tests/test_generator_execution.rs

#[test]
fn test_simple_generator() {
    let code = r#"
        let gen = () => generate {
            yield 1
            yield 2
            yield 3
        }

        let g = gen()
        [g.next().value, g.next().value, g.next().value]
    "#;

    let result = eval(code).unwrap();
    assert_eq!(result, vector![1.0, 2.0, 3.0]);
}

#[test]
fn test_generator_done_state() {
    let code = r#"
        let gen = () => generate {
            yield 1
        }

        let g = gen()
        let a = g.next()
        let b = g.next()
        [a.done, b.done]
    "#;

    let result = eval(code).unwrap();
    assert_eq!(result, vector![false, true]);
}

#[test]
fn test_generator_with_state() {
    let code = r#"
        let countdown = (n) => generate {
            mut i = n
            while(i > 0) {
                yield i
                i = i - 1
            }
        }

        let gen = countdown(3)
        [gen.next().value, gen.next().value, gen.next().value]
    "#;

    let result = eval(code).unwrap();
    assert_eq!(result, vector![3.0, 2.0, 1.0]);
}

#[test]
fn test_fibonacci_generator() {
    let code = r#"
        let fibonacci = () => generate {
            mut a = 0
            mut b = 1
            while(true) {
                yield a
                let temp = a
                a = b
                b = temp + b
            }
        }

        let fib = fibonacci()
        [
            fib.next().value,
            fib.next().value,
            fib.next().value,
            fib.next().value,
            fib.next().value
        ]
    "#;

    let result = eval(code).unwrap();
    assert_eq!(result, vector![0.0, 1.0, 1.0, 2.0, 3.0]);
}

#[test]
fn test_generator_return() {
    let code = r#"
        let gen = () => generate {
            yield 1
            yield 2
            return 42
        }

        let g = gen()
        g.next()
        g.next()
        let final_result = g.next()
        [final_result.value, final_result.done]
    "#;

    let result = eval(code).unwrap();
    assert_eq!(result, vector![42.0, true]);
}

#[test]
fn test_for_in_loop() {
    let code = r#"
        let range = (n) => generate {
            mut i = 0
            while(i < n) {
                yield i
                i = i + 1
            }
        }

        mut sum = 0
        for(x in range(5)) {
            sum = sum + x
        }
        sum
    "#;

    let result = eval(code).unwrap();
    assert_eq!(result, Value::Number(10.0)); // 0+1+2+3+4
}

#[test]
fn test_take_helper() {
    let code = r#"
        let take = (n, iterator) => do {
            mut result = []
            mut i = 0
            while(i < n) {
                let item = iterator.next()
                if(item.done) { break }
                result = push(result, item.value)
                i = i + 1
            }
            result
        }

        let fibonacci = () => generate {
            mut a = 0
            mut b = 1
            while(true) {
                yield a
                let temp = a
                a = b
                b = temp + b
            }
        }

        take(7, fibonacci())
    "#;

    let result = eval(code).unwrap();
    assert_eq!(result, vector![0.0, 1.0, 1.0, 2.0, 3.0, 5.0, 8.0]);
}

#[test]
fn test_lazy_map() {
    let code = r#"
        let map_iter = (f, iterator) => ({
            next: () => do {
                let item = iterator.next()
                if(item.done) {
                    item
                } else {
                    {value: f(item.value), done: false}
                }
            }
        })

        let range = (n) => generate {
            mut i = 0
            while(i < n) {
                yield i
                i = i + 1
            }
        }

        let doubled = map_iter(x => x * 2, range(3))
        [doubled.next().value, doubled.next().value, doubled.next().value]
    "#;

    let result = eval(code).unwrap();
    assert_eq!(result, vector![0.0, 2.0, 4.0]);
}
```

---

## Ejemplos de Uso

### Ejemplo 1: Range Generator

```achronyme
let range = (start, end, step) => generate {
    mut current = start
    if(step > 0) {
        while(current < end) {
            yield current
            current = current + step
        }
    } else {
        while(current > end) {
            yield current
            current = current + step
        }
    }
}

// Uso
for(x in range(0, 10, 2)) {
    print(x)  // 0, 2, 4, 6, 8
}
```

### Ejemplo 2: Procesamiento Lazy de Archivos

```achronyme
let readLines = (filename) => generate {
    let file = open(filename)
    while(!file.eof()) {
        yield file.readLine()
    }
    file.close()
}

// Procesar solo las primeras 100 líneas
let first100 = take(100, readLines("huge_file.txt"))
for(line in first100) {
    process(line)
}
```

### Ejemplo 3: Pipeline de Transformaciones

```achronyme
let numbers = range(1, 1000000)

let pipeline = numbers
    |> map_iter(x => x * 2)
    |> filter_iter(x => x % 3 == 0)
    |> take(10)

print(collect(pipeline))
// Solo calcula lo necesario, no crea arrays intermedios
```

---

## Siguientes Pasos

Una vez completada la Fase 1:

1. **Validar con usuarios**: Obtener feedback sobre la API
2. **Optimizar**: Profile y mejorar performance si es necesario
3. **Documentar**: Guía completa de iteradores y generators
4. **Preparar Fase 2**: Event Loop y Futures requieren generators funcionando

**Fecha objetivo**: 3 semanas desde hoy
**Milestone**: Generators + For-In completamente funcionales
