# Análisis Profundo: Recursión y Mutabilidad

**Fecha:** 2025-11-09
**Estado:** Investigación Completa

---

## 🔍 Hallazgos Clave

### 1. ✅ Shadowing vs Mutabilidad (Correctamente Identificado por el Usuario)

Achronyme actualmente soporta:

**A. Shadowing (Reasignación con `let`):**
```javascript
let x = 10
let x = 5  // ✅ Crea un nuevo binding en el mismo scope
// x ahora es 5
```

**Implementación:** `Environment::define()` (línea 75-82)
```rust
pub fn define(&mut self, name: String, value: Value) -> Result<(), String> {
    let current_scope = self.scopes.last_mut().unwrap();
    current_scope.insert(name, value);  // ← Permite redefinir
    Ok(())
}
```

**B. Mutabilidad (Modificación sin `let`):**
```javascript
let x = 10
x = 5  // ❌ NO IMPLEMENTADO AÚN
```

**Implementación existente:** `Environment::set()` (línea 123-132)
```rust
pub fn set(&mut self, name: &str, value: Value) -> Result<(), String> {
    // Busca la variable y la modifica donde fue definida
    for scope in self.scopes.iter_mut().rev() {
        if scope.contains_key(name) {
            scope.insert(name.to_string(), value);  // ← Modifica sin crear nuevo binding
            return Ok(());
        }
    }
    Err(format!("Cannot assign to undefined variable '{}'", name))
}
```

**Estado:**
- ✅ `set()` existe en Environment
- ❌ No hay sintaxis de asignación `=` sin `let` en el evaluator
- ❌ No hay operador de asignación en la gramática

---

### 2. ✅ Problema de Recursión - Análisis Completo

#### El Ejemplo del Usuario es Correcto:

```javascript
let x = 10
let funcion = x => x + 1
funcion(5)  // → 6 ✅
```

**¿Por qué funciona el shadowing de parámetros?**

Flujo de ejecución:

1. **Evaluación de `let funcion = x => x + 1`:**
   ```rust
   // variables.rs línea 14
   let value = evaluator.evaluate(initializer)?;  // ← Evalúa la lambda
   ```

2. **Dentro de `evaluate_lambda()` (functions.rs:9-21):**
   ```rust
   let captured_vars = evaluator.environment().snapshot();  // ← Captura { x: 10 }
   let function = Function::new(params, body, captured_vars);  // ← Almacena sin evaluar body
   ```

3. **Cuando se llama `funcion(5)`:**
   ```rust
   // apply_lambda() línea 42-50
   *evaluator.environment_mut() = Environment::from_snapshot(captured_vars);  // ← Restaura { x: 10 }
   evaluator.environment_mut().push_scope();  // ← NUEVO SCOPE para parámetros
   define("x", Value::Number(5));  // ← x = 5 SOMBREA el x = 10
   ```

**Resultado:** El parámetro `x` correctamente hace shadow del `x` del closure. ✅

---

#### Entonces, ¿Por qué NO funciona recursión?

```javascript
let factorial = n => if(n <= 1, 1, n * factorial(n - 1))
factorial(5)  // → Error: factorial not defined
```

**Problema:**

1. **Durante `let factorial = ...`:**
   ```rust
   // variables.rs línea 14
   let value = evaluator.evaluate(initializer)?;  // ← Evalúa lambda
   ```

2. **Dentro de `evaluate_lambda()`:**
   ```rust
   let captured_vars = evaluator.environment().snapshot();
   // ↑ Captura environment ACTUAL
   // factorial NO está definido todavía ❌
   ```

3. **La lambda se crea con closure que NO incluye `factorial`**

4. **Cuando `factorial(5)` evalúa el body:**
   ```rust
   // En apply_lambda, se evalúa: n * factorial(n - 1)
   // Se busca 'factorial' en el closure
   // NO ESTÁ ❌
   ```

**Diagrama temporal:**
```
Tiempo →
[1] let factorial = ...
[2]   ↓ evaluate lambda
[3]     ↓ snapshot environment  ← factorial NO existe aquí
[4]     ↓ create Function
[5]   ↓ return Function
[6]   ↓ define "factorial" ← factorial se define AHORA
[7] factorial(5)
[8]   ↓ apply lambda
[9]     ↓ restore snapshot from [3]  ← factorial NO está
[10]    ↓ evaluate body
[11]      ↓ search "factorial"  ← ❌ Error
```

---

## 🎯 Soluciones Posibles

### Solución 1: Define-Before-Evaluate ❌ NO FUNCIONA

**Idea:**
```rust
pub fn evaluate_declaration(...) -> Result<Value, String> {
    evaluator.environment_mut().define(name, Value::Undefined)?;  // ← Placeholder
    let value = evaluator.evaluate(initializer)?;
    evaluator.environment_mut().set(name, value)?;  // ← Actualiza
    Ok(value)
}
```

**Problema:**
```javascript
let factorial = n => if(n <= 1, 1, n * factorial(n - 1))
```

1. Define `factorial = Undefined`
2. Evalúa lambda
3. Captura snapshot: `{ factorial: Undefined }`  ← ❌ PROBLEMA
4. Actualiza: `factorial = Function`
5. Cuando se llama: restaura snapshot con `factorial: Undefined` ❌

**Resultado:** El closure captura `Undefined`, no la función real.

---

### Solución 2: Y-Combinator Fix Point ⚠️ COMPLEJO

**Idea:** Después de crear la función, inyectarla en su propio closure.

```rust
pub fn evaluate_declaration(...) -> Result<Value, String> {
    let value = evaluator.evaluate(initializer)?;

    // Si es una función, inyectarla en su propio closure
    if let Value::Function(ref mut func) = value {
        func.captured_vars.insert(name.to_string(), value.clone());
    }

    evaluator.environment_mut().define(name, value.clone())?;
    Ok(value)
}
```

**Pros:**
- ✅ Funciona para recursión simple
- ✅ No requiere cambios en gramática
- ✅ Transparente para el usuario

**Contras:**
- ⚠️ Requiere que `Function.captured_vars` sea mutable
- ⚠️ No funciona para recursión mutua fácilmente
- ⚠️ Puede causar referencias circulares

---

### Solución 3: Keyword `rec` ✅ LIMPIA

**Idea:** Marcar explícitamente funciones recursivas.

```javascript
let rec factorial = n => if(n <= 1, 1, n * factorial(n - 1))
```

**Implementación:**

1. **Gramática:**
```pest
let_statement = { "let" ~ "rec"? ~ identifier ~ "=" ~ expr }
```

2. **AST:**
```rust
VariableDecl {
    name: String,
    initializer: Box<AstNode>,
    recursive: bool  // ← NUEVO
}
```

3. **Evaluación:**
```rust
pub fn evaluate_declaration(..., recursive: bool) -> Result<Value, String> {
    let value = evaluator.evaluate(initializer)?;

    if recursive {
        if let Value::Function(ref mut func) = value {
            func.captured_vars.insert(name.to_string(), value.clone());
        }
    }

    evaluator.environment_mut().define(name, value.clone())?;
    Ok(value)
}
```

**Pros:**
- ✅ Explícito y claro
- ✅ Común en lenguajes funcionales (OCaml, F#, Reason)
- ✅ Sin impacto en rendimiento de funciones no-recursivas
- ✅ Documentación auto-explicativa

**Contras:**
- ⚠️ Más verboso (pero mínimamente)

---

### Solución 4: Self-Reference en Records ✅ YA PLANIFICADO

```javascript
let factorial = {
  compute: n => if(n <= 1, 1, n * self.compute(n - 1))
}
factorial.compute(5)  // → 120 ✅
```

**Pros:**
- ✅ Ya en el plan de implementación
- ✅ Organiza funciones relacionadas
- ✅ Soporte para estado mutable

**Contras:**
- ⚠️ Más verboso para funciones simples
- ⚠️ Diferente sintaxis de llamada

---

## 📊 Comparación de Soluciones

| Solución | Complejidad | Recursión Simple | Recursión Mutua | Impacto Usuario | Recomendado |
|----------|-------------|------------------|-----------------|-----------------|-------------|
| 1. Define-before | Baja | ❌ No funciona | ❌ No | Ninguno | ❌ No |
| 2. Y-Combinator | Media | ✅ Sí | ⚠️ Difícil | Transparente | ⭐ Aceptable |
| 3. `let rec` | Media | ✅ Sí | ✅ Sí | Explícito | ✅ **Mejor** |
| 4. Self en records | Alta | ✅ Sí | ✅ Sí | Más verboso | ✅ Complementario |

---

## 🎯 Decisión Final

### Implementar AMBAS Soluciones 2 y 3:

**Fase 1 (Corto Plazo): Y-Combinator Fix Point**
- Permitir recursión simple sin cambios de sintaxis
- Fix mínimo en `evaluate_declaration`
- Funciona para 90% de casos

**Fase 2 (Mediano Plazo): Keyword `rec`**
- Hacer recursión explícita
- Mejor para documentación
- Soporta recursión mutua

**Fase 3 (Ya Planificado): Self-Reference**
- Para records con métodos recursivos
- Soporta estado mutable

---

## 🛠️ Implementación Recomendada

### Paso 1: Y-Combinator Fix (Inmediato)

**Modificar:** `crates/achronyme-eval/src/handlers/variables.rs`

```rust
use achronyme_types::function::Function;

pub fn evaluate_declaration(
    evaluator: &mut Evaluator,
    name: &str,
    initializer: &AstNode,
) -> Result<Value, String> {
    // Evaluate the initializer
    let mut value = evaluator.evaluate(initializer)?;

    // If it's a function, inject self-reference for recursion
    if let Value::Function(ref func) = value {
        // Clone the function and inject self
        let mut captured_vars = func.captured_vars.clone();
        captured_vars.insert(name.to_string(), value.clone());

        // Create new function with updated closure
        let new_func = Function::new(
            func.params.clone(),
            func.body.clone(),
            captured_vars
        );
        value = Value::Function(new_func);
    }

    // Define the variable in the environment
    evaluator.environment_mut().define(name.to_string(), value.clone())?;

    Ok(value)
}
```

**Tests necesarios:**
```javascript
// Test 1: Recursión simple
let factorial = n => if(n <= 1, 1, n * factorial(n - 1))
factorial(5)  // → 120

// Test 2: Fibonacci
let fib = n => if(n <= 1, n, fib(n-1) + fib(n-2))
fib(10)  // → 55

// Test 3: Shadowing de parámetros sigue funcionando
let x = 10
let f = x => x + 1
f(5)  // → 6

// Test 4: Closures normales siguen funcionando
let outer = 10
let add_outer = x => x + outer
add_outer(5)  // → 15
```

---

### Paso 2: Keyword `rec` (Futuro)

**Modificar:** `crates/achronyme-parser/src/grammar.pest`

```pest
let_statement = {
    "let" ~ "rec"? ~ identifier ~ "=" ~ expr
}
```

**AST:**
```rust
VariableDecl {
    name: String,
    initializer: Box<AstNode>,
    recursive: bool,
}
```

**Evaluator:**
- Solo aplicar fix-point si `recursive == true`
- Permite optimizar funciones no-recursivas

---

## ⚠️ Consideraciones Finales

### 1. Recursión Mutua

Con Y-Combinator simple, esto **NO funcionará automáticamente:**
```javascript
let isEven = n => if(n == 0, true, isOdd(n - 1))
let isOdd = n => if(n == 0, false, isEven(n - 1))
```

**Solución:** Usar `let rec` en Fase 2, o definir en un record:
```javascript
let parity = {
  isEven: n => if(n == 0, true, self.isOdd(n - 1)),
  isOdd: n => if(n == 0, false, self.isEven(n - 1))
}
```

### 2. Performance

El Y-Combinator approach clona el closure, lo cual tiene un pequeño costo. Para la mayoría de casos es insignificante.

### 3. Mutabilidad para `self.field = value`

Para implementar mutabilidad en records:
1. Agregar `Assignment` operator en gramática
2. Distinguir entre:
   - `let x = 5` → Shadowing/define
   - `x = 5` → Mutabilidad/set
   - `self.field = 5` → Mutabilidad en record

---

**Conclusión:** El análisis del usuario era 100% correcto. Shadowing funciona perfectamente gracias a `push_scope()`. La recursión falla por captura temprana del closure. Solución: Inyectar self-reference después de crear la función.

**Próximo paso:** Implementar Y-Combinator fix en `variables.rs`.
