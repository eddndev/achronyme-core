# Plan de Implementación: Self-Reference en Records

**Fecha:** 2025-11-09
**Estado:** Planificación
**Prioridad:** Alta

---

## 🎯 Objetivo

Implementar `self` para permitir que los métodos dentro de records accedan a los datos y otros métodos del mismo record.

## 📐 Decisiones de Diseño

### 1. Palabra Clave: `self`
- **Elegido:** `self` (estilo Python/Rust)
- **Razón:** Más consistente con el modelo funcional, evita confusión con JavaScript

### 2. Binding: Léxico (Tiempo de Definición)
- El `self` se captura cuando se crea el record
- Predecible y sin sorpresas
- Se integra naturalmente con closures existentes

### 3. Mutabilidad: ✅ PERMITIDA
- **Decisión crítica:** Achronyme necesita mutabilidad para ser potente
- Los métodos pueden leer Y modificar `self`
- Sintaxis de modificación: `self.field = new_value`

### 4. Recursión: ✅ SOPORTADA
- Los métodos pueden llamarse a sí mismos vía `self`
- Ejemplo: `self.factorial(n - 1)`

---

## 💡 Casos de Uso

### Caso 1: Lectura de Estado
```javascript
let counter = {
  value: 0,
  getValue: () => self.value
}

counter.getValue()  // → 0
```

### Caso 2: Modificación de Estado
```javascript
let counter = {
  value: 0,
  increment: () => self.value = self.value + 1,
  decrement: () => self.value = self.value - 1
}

counter.increment()  // Modifica self.value a 1
counter.getValue()   // → 1
```

### Caso 3: Métodos que Llaman a Otros Métodos
```javascript
let math = {
  square: x => x * x,
  sumOfSquares: (a, b) => self.square(a) + self.square(b)
}

math.sumOfSquares(3, 4)  // → 9 + 16 = 25
```

### Caso 4: Recursión
```javascript
let factorial = {
  compute: n => if(n <= 1, 1, n * self.compute(n - 1))
}

factorial.compute(5)  // → 120
```

### Caso 5: Constructor + Estado Interno
```javascript
let createCounter = initial => ({
  value: initial,
  increment: () => self.value = self.value + 1,
  decrement: () => self.value = self.value - 1,
  reset: () => self.value = initial,  // Captura 'initial' de closure
  get: () => self.value
})

let counter = createCounter(10)
counter.increment()
counter.get()  // → 11
```

---

## 🛠️ Implementación Técnica

### Paso 1: Extender la Gramática
**Archivo:** `crates/achronyme-parser/src/grammar.pest`

Agregar palabra reservada `self`:
```pest
keyword = _{ "let" | "if" | "else" | "self" }

self_ref = { "self" }

primary = {
    boolean
  | string_literal
  | complex
  | number
  | record
  | array
  | lambda
  | function_call
  | self_ref  // ← NUEVO
  | identifier
  | "(" ~ expr ~ ")"
}
```

### Paso 2: Extender el AST
**Archivo:** `crates/achronyme-parser/src/ast.rs`

```rust
pub enum AstNode {
    // ... existentes
    SelfReference,  // ← NUEVO
}
```

### Paso 3: Parser
**Archivo:** `crates/achronyme-parser/src/pest_parser.rs`

```rust
fn build_primary(pair: Pair<Rule>) -> Result<AstNode, String> {
    match inner.as_rule() {
        // ... existentes
        Rule::self_ref => Ok(AstNode::SelfReference),  // ← NUEVO
        // ...
    }
}
```

### Paso 4: Evaluador - Inyección de Self
**Archivo:** `crates/achronyme-eval/src/handlers/literals.rs`

Modificar `evaluate_record()`:
```rust
pub fn evaluate_record(
    evaluator: &mut Evaluator,
    fields: &[(String, AstNode)],
) -> Result<Value, String> {
    let mut map = HashMap::new();

    // PASO 1: Crear record vacío y guardarlo temporalmente
    let record_ref = Rc::new(RefCell::new(HashMap::new()));

    // PASO 2: Inyectar 'self' en el environment
    evaluator.environment_mut().push_scope();
    evaluator.environment_mut().define(
        "self".to_string(),
        Value::Record(record_ref.clone())
    )?;

    // PASO 3: Evaluar campos (ahora pueden usar 'self')
    for (key, value_node) in fields {
        let value = evaluator.evaluate(value_node)?;
        map.insert(key.clone(), value);
    }

    // PASO 4: Actualizar el record con los campos evaluados
    *record_ref.borrow_mut() = map.clone();

    // PASO 5: Limpiar scope
    evaluator.environment_mut().pop_scope();

    Ok(Value::Record(map))
}
```

### Paso 5: Evaluador - Resolución de Self
**Archivo:** `crates/achronyme-eval/src/evaluator.rs`

```rust
AstNode::SelfReference => {
    // Buscar 'self' en el environment
    match self.environment().get("self") {
        Ok(value) => Ok(value),
        Err(_) => Err("'self' can only be used inside record methods".to_string())
    }
}
```

### Paso 6: Soporte para Mutabilidad
**Archivo:** `crates/achronyme-eval/src/handlers/binary_ops.rs`

Agregar operador de asignación `=` para `self.field`:
```rust
// Detectar asignación: self.field = value
if let AstNode::FieldAccess { record, field } = left {
    if matches!(**record, AstNode::SelfReference) {
        // Modificar el record en el environment
        // ...
    }
}
```

---

## 🧪 Plan de Testing

### Test 1: Self-reference básico (lectura)
```javascript
let obj = {
  x: 10,
  getX: () => self.x
}
obj.getX()  // → 10
```

### Test 2: Self-reference con modificación
```javascript
let obj = {
  x: 0,
  inc: () => self.x = self.x + 1
}
obj.inc()
obj.x  // → 1
```

### Test 3: Métodos llamando métodos
```javascript
let obj = {
  double: x => x * 2,
  quadruple: x => self.double(self.double(x))
}
obj.quadruple(5)  // → 20
```

### Test 4: Recursión
```javascript
let fib = {
  compute: n => if(n <= 1, n, self.compute(n-1) + self.compute(n-2))
}
fib.compute(10)  // → 55
```

### Test 5: Error fuera de record
```javascript
let x = self.value  // → Error: 'self' can only be used inside record methods
```

### Test 6: Nested records
```javascript
let outer = {
  x: 10,
  inner: {
    y: 20,
    getX: () => ???  // ← Necesita definición clara
  }
}
```

---

## ⚠️ Problemas Conocidos a Resolver

### 1. Nested Records
**Pregunta:** ¿`self` en un record anidado se refiere al padre o al hijo?

**Opción A:** Cada record tiene su propio `self`
```javascript
let outer = {
  x: 10,
  inner: {
    y: 20,
    getY: () => self.y  // → 20 (self = inner)
  }
}
```

**Opción B:** `self` se propaga desde el padre
```javascript
let outer = {
  x: 10,
  inner: {
    y: 20,
    getX: () => self.x  // → 10 (self = outer)
  }
}
```

**Decisión:** Opción A (cada record tiene su propio `self`)

### 2. Recursión Directa de Funciones
**Problema actual:**
```javascript
let factorial = n => if(n <= 1, 1, factorial(n - 1))
//                                 ↑ Error: factorial no está definido
```

**Causa:** El nombre `factorial` no está disponible dentro del cuerpo de la función porque la asignación aún no se completa.

**Solución temporal:** Usar records con `self`
```javascript
let factorial = {
  compute: n => if(n <= 1, 1, self.compute(n - 1))
}
factorial.compute(5)  // ✅ Funciona
```

**Solución futura:** Implementar `rec` keyword para funciones recursivas:
```javascript
let rec factorial = n => if(n <= 1, 1, factorial(n - 1))
```

---

## 📊 Estado de Implementación

- [ ] Paso 1: Extender gramática
- [ ] Paso 2: Extender AST
- [ ] Paso 3: Modificar parser
- [ ] Paso 4: Inyección de self en records
- [ ] Paso 5: Resolución de SelfReference
- [ ] Paso 6: Soporte para asignación/mutabilidad
- [ ] Paso 7: Tests unitarios
- [ ] Paso 8: Documentación de usuario

---

## 🚀 Siguiente Fase (Futuro)

Después de implementar `self`, considerar:

1. **`rec` keyword** para funciones recursivas standalone
2. **Pattern matching** en records
3. **Destructuring** para extraer campos
4. **Spread operator** `{...obj, x: 10}`
5. **Method chaining** fluido

---

**Aprobado por:** Usuario
**Fecha de aprobación:** 2025-11-09
