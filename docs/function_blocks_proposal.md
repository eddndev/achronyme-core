# Propuesta: Bloques de Función (Function Blocks)

## Problema
Actualmente las lambdas solo soportan una expresión:
```
let add = (a, b) => a + b
```

Necesitamos soporte para múltiples statements sin romper la sintaxis de records.

## Opciones de Sintaxis

### Opción 1: Usar `do` para bloques explícitos
```javascript
let factorial = n => do {
    let result = 1;
    let i = n;
    while (i > 0) {
        result = result * i;
        i = i - 1;
    };
    result  // último valor es el retorno
}
```

**Ventajas:**
- Clara distinción con records
- Palabra clave `do` indica bloque imperativo
- No ambigüedad

**Desventajas:**
- Requiere nueva palabra clave

### Opción 2: Secuencia con `;` y última expresión es retorno
```javascript
let factorial = n => (
    let result = 1;
    let i = n;
    while (i > 0) {
        result = result * i;
        i = i - 1;
    };
    result
)
```

**Ventajas:**
- Usa paréntesis, familiar para expresiones
- `;` separador de statements
- Última expresión es retorno implícito

**Desventajas:**
- Puede confundirse con agrupación de expresiones

### Opción 3: Array de statements con sintaxis especial
```javascript
let factorial = n => [
    let result = 1,
    let i = n,
    while (i > 0) [
        result = result * i,
        i = i - 1
    ],
    result  // retorno
]
```

**Desventajas:**
- Confuso con arrays literales
- No recomendado

### Opción 4: Bloque con `=>` seguido de `{` requiere `return`
```javascript
let factorial = n => {
    let result = 1;
    let i = n;
    while (i > 0) {
        result = result * i;
        i = i - 1;
    };
    return result;
}
```

**Ventajas:**
- Sintaxis familiar (JavaScript/Rust style)
- `return` explícito distingue de records

**Desventajas:**
- ¿Cómo distinguir `() => { a: 1 }` (record) de `() => { return a; }` (bloque)?

### Opción 5: Híbrido - Detectar por contenido
```javascript
// Si tiene `:` sin `let`/`return`/`;` -> Record
let makePoint = () => { x: 10, y: 20 }

// Si tiene `let`, `return`, `;`, `while`, etc -> Bloque
let factorial = n => {
    let result = 1;
    let i = n;
    while (i > 0) {
        result = result * i;
        i = i - 1;
    };
    return result;
}

// Expresión única sigue igual
let add = (a, b) => a + b
```

**Ventajas:**
- Natural y familiar
- Diferenciación contextual
- Backward compatible

**Desventajas:**
- Parser más complejo
- Casos ambiguos: `() => { x }` ¿es record o bloque?

## Recomendación FINAL: Palabra Reservada `do` (Opción 1)

### Sintaxis Propuesta

**1. Lambda expresión (actual):**
```javascript
let add = (a, b) => a + b
```

**2. Lambda retornando record (sin cambios):**
```javascript
let point = (x, y) => { x: x, y: y }
```

**3. Lambda con bloque usando `do`:**
```javascript
let factorial = n => do {
    let result = 1;
    let i = n;
    // Sin mutabilidad, usamos recursión con helpers
    let helper = (acc, count) =>
        if(count <= 0, acc, rec(acc * count, count - 1));
    helper(result, i)
}

// Ejemplo más simple
let compute = x => do {
    let doubled = x * 2;
    let squared = doubled * doubled;
    squared + 10
}

// Con `rec` para recursión
let fibonacci = n => do {
    if(n <= 1, n, rec(n - 1) + rec(n - 2))
}
```

**Alternativamente, con `fun` keyword:**
```javascript
let factorial = fun(n) {
    let result = 1;
    let helper = (acc, count) =>
        if(count <= 0, acc, rec(acc * count, count - 1));
    helper(result, n)
}
```

### Reglas Simplificadas (con `do`)

1. **`=> do { ... }`** → Siempre es un bloque de función
2. **`=> { ... }`** → Siempre es un record literal
3. **`=> expresión`** → Lambda de expresión única

**No hay ambigüedad.** Simple y claro.

### Comparación de Opciones

#### Opción A: `do` keyword
```javascript
let process = data => do {
    let filtered = filter(x => x > 0, data);
    let doubled = map(x => x * 2, filtered);
    sum(doubled)
}
```

**Ventajas:**
- ✅ Palabra corta y clara
- ✅ `=>` sigue siendo el operador lambda
- ✅ Mantiene consistencia: `param => ...`
- ✅ Zero ambigüedad

#### Opción B: `fun` keyword (estilo function declaration)
```javascript
let process = fun(data) {
    let filtered = filter(x => x > 0, data);
    let doubled = map(x => x * 2, filtered);
    sum(doubled)
}
```

**Ventajas:**
- ✅ Familiar para otros lenguajes (OCaml, F#)
- ✅ Zero ambigüedad
- ✅ Sintaxis diferente para bloques vs lambdas

**Desventajas:**
- ❌ Pierde el `=>` (dos sintaxis para funciones)
- ❌ Más verboso

### Recomendación: `do` keyword

Usar `do` porque:
1. **Mantiene `=>` consistente** para todas las funciones
2. **Más corto** que `fun`
3. **Clara intención:** "do this block of computations"
4. **Sin ambigüedad** con records

## Características Propuestas (SIN MUTABILIDAD)

### 1. Múltiples Let Bindings (Inmutables)
```javascript
let process = data => do {
    let filtered = filter(x => x > 0, data);
    let doubled = map(x => x * 2, filtered);
    let total = sum(doubled);
    total  // última expresión es el retorno
}
```

### 2. Variables Locales Inmutables
```javascript
let complex = (a, b) => do {
    let temp1 = a * 2;
    let temp2 = b + 3;
    let result = temp1 + temp2;
    result
}
```

### 3. Recursión con `rec` (CRÍTICO)
```javascript
// `rec` debe funcionar dentro de bloques `do`
let factorial = n => do {
    if(n <= 1,
        1,
        n * rec(n - 1)
    )
}

// Recursión con helper
let fibonacci = n => do {
    let fib_helper = (a, b, count) =>
        if(count <= 0, a, rec(b, a + b, count - 1));
    fib_helper(0, 1, n)
}
```

### 4. Shadowing (No Mutación)
```javascript
let transform = x => do {
    let x = x * 2;      // shadowing, NO mutación
    let x = x + 10;     // shadowing otra vez
    let x = x / 2;      // shadowing otra vez
    x                    // retorna el último valor
}
```

### 5. Compatibilidad con Closures (Ya funciona)
```javascript
let makeAccumulator = start => do {
    // Como NO hay mutabilidad, devolvemos funciones
    // que crean NUEVOS valores en lugar de mutar
    {
        add: x => start + x,
        multiply: x => start * x,
        value: () => start
    }
}
```

### 6. IIFE con bloques
```javascript
let result = (x => do {
    let doubled = x * 2;
    let squared = doubled * doubled;
    squared
})(5)  // result = 100
```

### 7. Condicionales complejos
```javascript
let classify = n => do {
    let abs_n = if(n < 0, -n, n);
    let category = if(abs_n < 10, "small",
                   if(abs_n < 100, "medium", "large"));
    { value: n, category: category }
}
```

## Implementación por Fases

### Fase 1: Grammar Changes (pest)
```pest
// Añadir regla para bloques do
do_block = { "do" ~ "{" ~ block_body ~ "}" }
block_body = { (let_binding | expression) ~ (";" ~ (let_binding | expression))* ~ ";"? }
let_binding = { "let" ~ identifier ~ "=" ~ expression }

// Actualizar lambda_body
lambda_body = { do_block | expression }
```

### Fase 2: AST Changes
```rust
pub enum AstNode {
    // ... existentes

    // Nuevo: Bloque do
    DoBlock {
        statements: Vec<AstNode>,  // Todas las expresiones/bindings
    },

    // Nuevo: Let binding (solo dentro de bloques)
    LetBinding {
        name: String,
        value: Box<AstNode>,
    },
}
```

**NOTA:** NO necesitamos `Return` porque:
- La última expresión del bloque es el retorno implícito
- Sin mutabilidad, no hay early returns útiles

### Fase 3: Parser Changes
```rust
// En pest_parser.rs
fn parse_do_block(pair: Pair<Rule>) -> AstNode {
    let mut statements = Vec::new();

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::let_binding => {
                statements.push(parse_let_binding(inner_pair));
            }
            Rule::expression => {
                statements.push(parse_expression(inner_pair));
            }
            _ => {}
        }
    }

    AstNode::DoBlock { statements }
}
```

### Fase 4: Evaluator Changes
```rust
// En evaluator.rs
AstNode::DoBlock { statements } => {
    if statements.is_empty() {
        return Ok(Value::Number(0.0)); // o error
    }

    // Crear nuevo scope para el bloque
    self.environment_mut().push_scope();

    let mut result = Value::Number(0.0);

    for (i, stmt) in statements.iter().enumerate() {
        match stmt {
            AstNode::LetBinding { name, value } => {
                let val = self.evaluate(value)?;
                self.environment_mut().define(name.clone(), val)?;
            }
            _ => {
                result = self.evaluate(stmt)?;
            }
        }
    }

    // Pop scope
    self.environment_mut().pop_scope();

    Ok(result)
}
```

### Fase 5: Garantizar que `rec` funciona
```rust
// En handlers/functions.rs - apply_lambda
// Ya está implementado: inyectamos 'rec' en cada llamada
evaluator.environment_mut().define("rec".to_string(), Value::Function(function.clone()))?;

// Esto debe funcionar dentro de bloques do también
// porque el scope del bloque es hijo del scope donde está 'rec'
```

### Fase 6: Testing
1. **Test básico:**
   ```javascript
   let test = x => do {
       let y = x * 2;
       y + 10
   }
   assert(test(5) == 20)
   ```

2. **Test recursión con `rec`:**
   ```javascript
   let factorial = n => do {
       if(n <= 1, 1, n * rec(n - 1))
   }
   assert(factorial(5) == 120)
   ```

3. **Test shadowing:**
   ```javascript
   let shadow = x => do {
       let x = x + 1;
       let x = x * 2;
       x
   }
   assert(shadow(5) == 12)  // (5+1)*2
   ```

4. **Test closures:**
   ```javascript
   let makeAdder = n => do {
       let add = x => n + x;
       add
   }
   let add10 = makeAdder(10);
   assert(add10(5) == 15)
   ```

5. **Test bloques vacíos:**
   ```javascript
   let empty = () => do {}  // ¿error o retorna unit/0?
   ```

## Ejemplos de Uso Real (Sin Mutabilidad)

### Ejemplo 1: Pipeline de Datos con Pasos Intermedios
```javascript
let analyzeData = rawData => do {
    let cleaned = filter(x => x != null, rawData);
    let normalized = map(x => x / 100, cleaned);
    let mean = sum(normalized) / length(normalized);
    let stats = {
        mean: mean,
        max: max(normalized),
        min: min(normalized),
        count: length(normalized)
    };
    stats
}
```

### Ejemplo 2: Quicksort (Recursivo)
```javascript
let quicksort = arr => do {
    if(length(arr) <= 1,
        arr,
        do {
            let pivot = arr[0];
            let rest = arr[1..];
            let less = filter(x => x < pivot, rest);
            let greater = filter(x => x >= pivot, rest);
            concat(rec(less), [pivot], rec(greater))
        }
    )
}
```

### Ejemplo 3: Validador con Helpers
```javascript
let makeValidator = rules => do {
    let checkRule = (rule, data) =>
        if(rule.check(data), [], [rule.message]);

    let validateAll = (ruleList, data, errors) =>
        if(length(ruleList) == 0,
            errors,
            rec(
                ruleList[1..],
                data,
                concat(errors, checkRule(ruleList[0], data))
            )
        );

    {
        validate: data => do {
            let errors = validateAll(rules, data, []);
            { valid: length(errors) == 0, errors: errors }
        }
    }
}
```

### Ejemplo 4: Fibonacci con Tail Recursion
```javascript
let fibonacci = n => do {
    let fib_tail = (a, b, count) =>
        if(count <= 0, a, rec(b, a + b, count - 1));

    fib_tail(0, 1, n)
}
```

### Ejemplo 5: Map-Reduce Complejo
```javascript
let processOrders = orders => do {
    let validOrders = filter(o => o.amount > 0, orders);
    let withTax = map(o => { id: o.id, total: o.amount * 1.21 }, validOrders);
    let totalRevenue = sum(map(o => o.total, withTax));

    {
        processed: withTax,
        revenue: totalRevenue,
        count: length(withTax)
    }
}
```

### Ejemplo 6: Parser de Expresiones (Recursivo)
```javascript
let parseExpr = tokens => do {
    let first = tokens[0];
    let rest = tokens[1..];

    let result = if(first == "(",
        do {
            let inner = rec(rest);
            { expr: inner.expr, remaining: inner.remaining[1..] }
        },
        { expr: first, remaining: rest }
    );

    result
}
```

## Notas de Diseño Final

### Características Clave:
- ✅ **Ciudadanos de primera clase:** Las funciones con bloques `do` siguen siendo valores
- ✅ **Tipado dinámico:** El tipo de retorno se determina en runtime
- ✅ **Scope léxico:** Variables capturadas correctamente con `Rc<Environment>` (ya optimizado)
- ✅ **Inmutabilidad:** Solo bindings inmutables con `let` (shadowing permitido)
- ✅ **Recursión:** `rec` funciona dentro de bloques `do`
- ✅ **Sin ambigüedad:** `=> do { }` vs `=> { }` son claramente diferentes

### Limitaciones Aceptadas:
- ❌ **No hay mutación:** Usar recursión y shadowing en su lugar
- ❌ **No hay return temprano:** La última expresión es siempre el retorno
- ❌ **No hay loops imperativos:** Usar recursión y funciones de orden superior

### Beneficios:
- 🚀 **Performance:** El `Rc<Environment>` hace que closures sean O(1)
- 🧩 **Composabilidad:** Los bloques son expresiones, se pueden anidar
- 🔒 **Seguridad:** Sin mutación = sin race conditions ni efectos secundarios inesperados
