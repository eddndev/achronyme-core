# Problema: Recursión Directa en Funciones

**Fecha:** 2025-11-09
**Estado:** Investigado - Solución planificada

---

## 🐛 El Problema

```javascript
let factorial = n => if(n <= 1, 1, n * factorial(n - 1))
factorial(5)  // → Error: Unknown function or constant: factorial
```

**Pero curiosamente:**
```javascript
factorial(1)  // → 1 ✅ (funciona porque no necesita recursión)
```

---

## 🔍 Causa Raíz

**Ubicación:** `crates/achronyme-eval/src/handlers/variables.rs:8-21`

```rust
pub fn evaluate_declaration(
    evaluator: &mut Evaluator,
    name: &str,
    initializer: &AstNode,
) -> Result<Value, String> {
    // PASO 1: Evalúa el initializer
    let value = evaluator.evaluate(initializer)?;  // ← La lambda se evalúa AQUÍ

    // PASO 2: Define la variable (DESPUÉS de evaluar)
    evaluator.environment_mut().define(name.to_string(), value.clone())?;  // ← Definición TARDÍA

    Ok(value)
}
```

### Flujo de Ejecución:

1. Parser ve: `let factorial = n => if(n <= 1, 1, n * factorial(n - 1))`
2. Llama a `evaluate_declaration("factorial", <lambda_node>)`
3. **Línea 14:** Evalúa la lambda `n => ...`
   - Durante esta evaluación, el cuerpo de la lambda se analiza
   - Se encuentra la referencia a `factorial`
   - Se intenta resolver `factorial` en el environment
   - **❌ FALLA:** `factorial` todavía no está definido (línea 17 aún no se ejecuta)
4. **Error:** "Undefined variable or constant: factorial"

### ¿Por qué funciona `factorial(1)`?

```javascript
factorial(1)
→ if(1 <= 1, 1, 1 * factorial(0))
→ 1  // ← Rama corta! Nunca llama a factorial recursivamente
```

No necesita evaluar la parte recursiva, por eso funciona.

---

## 🎯 Soluciones Posibles

### Solución 1: Definir ANTES de Evaluar (Simple)

**Modificar** `evaluate_declaration()`:

```rust
pub fn evaluate_declaration(
    evaluator: &mut Evaluator,
    name: &str,
    initializer: &AstNode,
) -> Result<Value, String> {
    // PASO 1: Definir variable con placeholder
    evaluator.environment_mut().define(
        name.to_string(),
        Value::Undefined  // Placeholder
    )?;

    // PASO 2: Evaluar el initializer (ahora 'name' está en scope)
    let value = evaluator.evaluate(initializer)?;

    // PASO 3: Actualizar con el valor real
    evaluator.environment_mut().set(name.to_string(), value.clone())?;

    Ok(value)
}
```

**Pros:**
- ✅ Simple de implementar
- ✅ Funciona para recursión directa
- ✅ No requiere cambios en gramática

**Contras:**
- ⚠️ Requiere agregar `Value::Undefined`
- ⚠️ Requiere método `set()` en Environment (además de `define()`)

---

### Solución 2: Keyword `rec` (Explícito)

**Agregar** sintaxis especial para funciones recursivas:

```javascript
let rec factorial = n => if(n <= 1, 1, n * factorial(n - 1))
```

**Implementación:**

1. Modificar gramática:
```pest
let_statement = { "let" ~ "rec"? ~ identifier ~ "=" ~ expr }
```

2. Extender AST:
```rust
VariableDecl {
    name: String,
    initializer: Box<AstNode>,
    recursive: bool  // ← NUEVO
}
```

3. Manejar recursión solo si `recursive == true`

**Pros:**
- ✅ Explícito y claro
- ✅ Común en lenguajes funcionales (OCaml, F#)
- ✅ No afecta rendimiento de funciones no-recursivas

**Contras:**
- ⚠️ Requiere que el usuario indique recursión
- ⚠️ Más verboso

---

### Solución 3: Y-Combinator (Avanzado)

Usar el combinador de punto fijo para lograr recursión sin nombres:

```javascript
let Y = f => (x => f(x(x)))(x => f(x(x)))
let factorial = Y(self => n => if(n <= 1, 1, n * self(n - 1)))
```

**Pros:**
- ✅ Teóricamente elegante
- ✅ No requiere cambios en el lenguaje

**Contras:**
- ❌ Complejidad innecesaria
- ❌ Difícil de entender para usuarios
- ❌ Problemas con evaluación eager

---

### Solución 4: Self en Records (Implementado Next)

**Solución alternativa:** Usar records con `self`:

```javascript
let factorial = {
  compute: n => if(n <= 1, 1, n * self.compute(n - 1))
}

factorial.compute(5)  // → 120 ✅
```

**Pros:**
- ✅ Funciona con el sistema de `self` que estamos implementando
- ✅ Organiza código en namespaces

**Contras:**
- ⚠️ Más verboso para funciones simples
- ⚠️ Requiere acceso vía `.compute()`

---

## 🎯 Decisión Recomendada

**Implementar AMBAS soluciones 1 y 2:**

1. **Corto plazo:** Solución 1 (definir antes de evaluar)
   - Permite recursión inmediata
   - Cambio mínimo en el código

2. **Mediano plazo:** Solución 2 (keyword `rec`)
   - Hace la recursión explícita
   - Mejor para el usuario

3. **Ya disponible:** Solución 4 (self en records)
   - Para casos más complejos con estado

---

## 📋 Plan de Implementación

### Fase 1: Fix Inmediato (Solución 1)
1. Agregar `Value::Undefined` al enum
2. Agregar método `set()` a Environment
3. Modificar `evaluate_declaration()`
4. Tests de recursión

### Fase 2: Keyword `rec` (Solución 2)
1. Modificar gramática
2. Extender AST
3. Actualizar parser
4. Actualizar evaluator
5. Tests y documentación

### Fase 3: Self-Reference (Ya planificado)
- Ver `PLAN_SELF_REFERENCE.md`

---

## 🧪 Tests Necesarios

```javascript
// Test 1: Recursión directa simple
let rec factorial = n => if(n <= 1, 1, n * factorial(n - 1))
factorial(5)  // → 120

// Test 2: Recursión mutua
let rec isEven = n => if(n == 0, true, isOdd(n - 1))
let rec isOdd = n => if(n == 0, false, isEven(n - 1))
isEven(10)  // → true

// Test 3: Fibonacci
let rec fib = n => if(n <= 1, n, fib(n-1) + fib(n-2))
fib(10)  // → 55

// Test 4: Error si no es rec
let fact = n => factorial(n - 1)  // → Error: factorial no definido
```

---

## 📊 Comparación con Otros Lenguajes

| Lenguaje | Recursión Directa | Requiere Keyword |
|----------|-------------------|------------------|
| JavaScript | ✅ Sí | No |
| Python | ✅ Sí | No |
| OCaml | ✅ Sí | Sí (`let rec`) |
| F# | ✅ Sí | Sí (`let rec`) |
| Haskell | ✅ Sí | No |
| **Achronyme (actual)** | ❌ No | - |
| **Achronyme (propuesto)** | ✅ Sí | Opcional (`let rec`) |

---

**Autor:** Asistente
**Revisado por:** Usuario
**Estado:** Documentado - Pendiente de implementación
