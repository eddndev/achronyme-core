# Propuesta: Spread Operator (`...`)

## Motivación

El spread operator es una característica fundamental que falta en Achronyme y que resuelve múltiples problemas:

1. **No hay forma de concatenar vectores** sin usar funciones
2. **No hay forma de extender/mezclar records** eficientemente
3. **Patrones OOP son verbosos** (copiar métodos manualmente)
4. **Actualización inmutable de records** requiere reconstrucción completa

## Sintaxis Propuesta

```javascript
// En Arrays
let combined = [...vec1, ...vec2]
let withExtra = [0, ...numbers, 10]

// En Records
let extended = { ...base, newField: value }
let updated = { ...config, timeout: 5000 }
```

## Fase 1: Implementación Básica

### Soporte Mínimo Viable

1. **Spread en Array Literals**
   ```javascript
   [expr1, ...arrayExpr, expr2]
   ```

2. **Spread en Record Literals**
   ```javascript
   { field1: val1, ...recordExpr, field2: val2 }
   ```

### Grammar Changes

```pest
// Array elements pueden ser expresiones o spread
array_element = { spread_expr | expr }
spread_expr = { "..." ~ expr }

// Lo mismo para records
record_field_or_spread = { spread_expr | record_field }
```

### AST Changes

```rust
pub enum AstNode {
    // Modificar ArrayLiteral
    ArrayLiteral {
        elements: Vec<ArrayElement>,
    },

    // Modificar RecordLiteral
    RecordLiteral {
        fields: Vec<RecordFieldOrSpread>,
    },
}

pub enum ArrayElement {
    Single(AstNode),        // expr
    Spread(Box<AstNode>),   // ...expr
}

pub enum RecordFieldOrSpread {
    Field { name: String, value: AstNode },
    Spread(Box<AstNode>),   // ...expr
}
```

### Evaluator Logic

```rust
// Para Arrays
AstNode::ArrayLiteral { elements } => {
    let mut result = Vec::new();

    for element in elements {
        match element {
            ArrayElement::Single(node) => {
                result.push(evaluate(node)?);
            }
            ArrayElement::Spread(node) => {
                let value = evaluate(node)?;
                match value {
                    Value::Vector(vec) => {
                        result.extend(vec);
                    }
                    _ => return Err("Spread requires array/vector")
                }
            }
        }
    }

    Ok(Value::Vector(result))
}

// Para Records
AstNode::RecordLiteral { fields } => {
    let mut result = HashMap::new();

    for field in fields {
        match field {
            RecordFieldOrSpread::Field { name, value } => {
                result.insert(name, evaluate(value)?);
            }
            RecordFieldOrSpread::Spread(node) => {
                let value = evaluate(node)?;
                match value {
                    Value::Record(map) => {
                        // Merge into result (later values override)
                        result.extend(map);
                    }
                    _ => return Err("Spread requires record")
                }
            }
        }
    }

    Ok(Value::Record(result))
}
```

## Semántica

### Orden de Evaluación

Left-to-right. Último valor gana para claves/índices duplicados.

```javascript
{ a: 1, ...{ a: 2 }, a: 3 }  // { a: 3 }
```

### Type Safety (Runtime)

**Arrays:**
- `...` en array → expr debe evaluar a `Value::Vector`
- `Value::Tensor` → ❌ Error: "Cannot spread Tensor. Use concat() or reshape()"
  - Razón: Tensors tienen shape, necesitan operaciones específicas
  - Excepción futura: Podríamos permitir Tensor 1D si se convierte a Vector
- Tipos mixtos → ❌ Error: "Incompatible types" (ya validado en arrays)
- **Validación de shape:** El spread concatena elementos, luego la validación normal de arrays se aplica:
  ```javascript
  // Números → Se intentan convertir a Tensor → Deben tener shapes compatibles
  let v1 = [[1, 2], [3, 4]]  // 2x2 Tensor
  let v2 = [[5, 6]]          // 1x2 Tensor
  [...v1, ...v2]             // Error: incompatible shapes durante construcción del array resultante

  // Strings → Quedan como Vector<Vector<String>> → Shapes pueden diferir
  let s1 = [["a", "b", "c"], ["d", "e"]]
  let s2 = [["f"]]
  [...s1, ...s2]             // OK: [["a","b","c"], ["d","e"], ["f"]]

  // Records → Quedan como Vector<Record> → Sin restricción de shape
  let r1 = [{ x: 1 }, { x: 2, y: 3 }]
  let r2 = [{ z: 4 }]
  [...r1, ...r2]             // OK: [{x:1}, {x:2,y:3}, {z:4}]
  ```

**Records:**
- `...` en record → expr debe evaluar a `Value::Record`
- Claves duplicadas → **Último gana** (left-to-right evaluation)
  ```javascript
  { a: 1, ...{ a: 2 }, a: 3 }  // { a: 3 } ← último literal
  ```
- Otros tipos → ❌ Error

### Shallow Copy

El spread hace shallow copy (copia referencias, no clona profundamente).

```javascript
let base = { inner: { x: 1 } }
let copy = { ...base }
// copy.inner y base.inner son el mismo objeto (compartido via Rc)
```

## Ejemplos de Uso

### Concatenar Vectores

```javascript
let v1 = [1, 2, 3]
let v2 = [4, 5, 6]
let combined = [...v1, ...v2]  // [1, 2, 3, 4, 5, 6]
```

### Extender Records (OOP Pattern)

```javascript
let object_class = {
    doubleThenSquare: () => do {
        let aux = self.value + self.value;
        aux * aux
    },
    getValue: () => self.value
}

// Ahora crear instancias es trivial
let obj1 = { ...object_class, value: 10 }
let obj2 = { ...object_class, value: 20 }
```

### Actualización Inmutable

```javascript
let config = { timeout: 1000, retries: 3, verbose: false }
let updated = { ...config, timeout: 5000 }
// config no cambia, updated es nuevo
```

### Mezclar Múltiples Sources

```javascript
let defaults = { x: 0, y: 0 }
let userPrefs = { y: 10 }
let overrides = { z: 20 }
let final = { ...defaults, ...userPrefs, ...overrides }
// { x: 0, y: 10, z: 20 }
```

### Construir Arrays Dinámicamente

```javascript
let header = [0, 1]
let body = [2, 3, 4]
let footer = [5]
let full = [...header, ...body, ...footer]  // [0,1,2,3,4,5]
```

### Agregar a Arrays (inmutable)

```javascript
let numbers = [1, 2, 3]
let withZero = [0, ...numbers]      // [0, 1, 2, 3]
let withTen = [...numbers, 10]      // [1, 2, 3, 10]
let withBoth = [0, ...numbers, 10]  // [0, 1, 2, 3, 10]
```

## Limitaciones Conocidas (Fase 1)

### NO soportado en Fase 1:

1. **Function calls con spread**
   ```javascript
   myFunc(...args)  // ❌ No (por ahora)
   ```

2. **Destructuring con rest**
   ```javascript
   let { x, ...rest } = obj  // ❌ No (feature separada)
   ```

3. **Spread de strings**
   ```javascript
   [...'hello']  // ❌ No (podría agregarse después)
   ```

4. **Spread de tensors**
   ```javascript
   [...tensor]  // ❌ No (requiere decisión de diseño)
   ```

## Testing

### Tests Básicos

```javascript
// Test 1: Array concat
let t1 = [...[1,2], ...[3,4]]  // [1,2,3,4] ✅

// Test 2: Record merge
let t2 = { ...{a:1}, ...{b:2} }  // {a:1, b:2} ✅

// Test 3: Override simple
let t3 = { a:1, ...{a:2} }  // {a:2} ✅

// Test 4: Multiple spreads
let t4 = [...[1], ...[2], ...[3]]  // [1,2,3] ✅

// Test 5: Mixed literals and spreads
let t5 = [0, ...[1,2], 3]  // [0,1,2,3] ✅

// Test 6: OOP pattern
let base = { m: () => 1 }
let inst = { ...base, v: 10 }
inst.m()  // 1 ✅
inst.v    // 10 ✅
```

### Tests de Precedencia (Records)

```javascript
// Test 7: Orden importa
let defaults = { a: 1, b: 2, c: 3 }
let overrides = { b: 10, c: 20 }

let r1 = { ...defaults, ...overrides }
// { a: 1, b: 10, c: 20 } ✅ - overrides gana

let r2 = { ...overrides, ...defaults }
// { a: 1, b: 2, c: 3 } ✅ - defaults gana

// Test 8: Triple override
let r3 = { ...{a:1}, ...{a:2}, ...{a:3} }
// { a: 3 } ✅ - último gana

// Test 9: Spread + literal override
let r4 = { ...{a:1, b:2}, a: 3 }
// { a: 3, b: 2 } ✅ - literal a:3 gana

let r5 = { a: 1, ...{a:2, b:3} }
// { a: 2, b: 3 } ✅ - spread a:2 gana
```

### Tests de Tipos (Arrays)

```javascript
// Test 10: Vectores simples OK
let nums1 = [1, 2, 3]
let nums2 = [4, 5, 6]
let t10 = [...nums1, ...nums2]  // [1,2,3,4,5,6] ✅

// Test 11: Vectores heterogéneos OK (si Value::Vector lo permite)
let v1 = [1, "hello", true]
let v2 = [3.14, { x: 1 }]
let t11 = [...v1, ...v2]  // [1, "hello", true, 3.14, {x:1}] ✅

// Test 12: Strings en vectores OK
let strs1 = ["a", "b"]
let strs2 = ["c", "d"]
let t12 = [...strs1, ...strs2]  // ["a","b","c","d"] ✅
```

### Tests de Error

```javascript
// Test E1: Tipo incorrecto
let e1 = [...5]         // ❌ Error: Cannot spread non-iterable value

// Test E2: Record en array
let e2 = [...{a:1}]     // ❌ Error: Cannot spread Record in array context

// Test E3: Array en record
let e3 = { ...[1,2] }   // ❌ Error: Cannot spread Vector in record context

// Test E4: Number en record
let e4 = { ...42 }      // ❌ Error: Cannot spread non-record value

// Test E5: Tensor multidimensional (FASE 1)
let tensor = [[1, 2], [3, 4]]  // Tensor 2D
let e5 = [...tensor]    // ❌ Error: Cannot spread Tensor. Use concat() or reshape()

// Test E6: Undefined variable
let e6 = [...nonExistent]  // ❌ Error: Undefined variable 'nonExistent'

// Test E7: Función (no iterable)
let fn = x => x * 2
let e7 = [...fn]        // ❌ Error: Cannot spread Function
```

### Tests Avanzados (OOP)

```javascript
// Test A1: Constructor pattern completo
let Animal = {
    name: "Unknown",
    speak: () => do {
        let sound = self.sound;
        sound
    }
}

let Dog = { ...Animal, sound: "Woof" }
let Cat = { ...Animal, sound: "Meow" }

Dog.speak()  // "Woof" ✅
Cat.speak()  // "Meow" ✅

// Test A2: Composición de comportamientos
let Walkable = { walk: () => "Walking..." }
let Swimmable = { swim: () => "Swimming..." }
let Duck = { ...Walkable, ...Swimmable, name: "Duck" }

Duck.walk()  // "Walking..." ✅
Duck.swim()  // "Swimming..." ✅
Duck.name    // "Duck" ✅

// Test A3: Override de método
let Base = { method: () => "base" }
let Derived = { ...Base, method: () => "derived" }

Derived.method()  // "derived" ✅ - override funciona
```

## Plan de Implementación

### Paso 1: Grammar (pest)
- Añadir `spread_expr = { "..." ~ expr }`
- Modificar `array` y `record` rules

### Paso 2: AST
- Crear `ArrayElement` enum
- Crear `RecordFieldOrSpread` enum
- Modificar `ArrayLiteral` y `RecordLiteral`

### Paso 3: Parser
- Parsear `...` en arrays y records
- Construir nuevos AST nodes

### Paso 4: Evaluator
- Implementar lógica de spread para arrays
- Implementar lógica de spread para records
- Validación de tipos en runtime

### Paso 5: Testing
- Unit tests
- Integration tests
- Ejemplos de uso real

## Fase 2 (Futuro)

- Spread en function calls: `func(...args)`
- Spread de strings: `[..."hello"]`
- Rest parameters en destructuring
- Spread de tensors (si tiene sentido)

## Notas de Diseño

- **Inmutabilidad preservada:** Spread siempre crea nuevos valores
- **Performance con Rc:** Records ya usan Rc internamente para Values, así que spread es eficiente
- **Consistencia:** Sintaxis y semántica alineadas con JavaScript/TypeScript
- **Extensibilidad:** Fácil añadir más contextos de spread en el futuro
