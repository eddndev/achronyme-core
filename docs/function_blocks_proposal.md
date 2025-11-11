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

### Fase 1: AST Changes
- Añadir `AstNode::Block { statements: Vec<AstNode> }`
- Añadir `AstNode::Return { value: Box<AstNode> }`
- Añadir `AstNode::LetStatement { name: String, value: Box<AstNode> }`

### Fase 2: Parser Changes
- Detectar `=>` seguido de `{`
- Analizar contenido para determinar si es record o bloque
- Parsear múltiples statements separados por `;`

### Fase 3: Evaluator Changes
- Evaluar `Block`: ejecutar statements secuencialmente
- Evaluar `Return`: retornar inmediatamente
- Manejar scope local para `let` dentro de bloques

### Fase 4: Testing
- Tests de bloques simples
- Tests de desambiguación record vs bloque
- Tests de closures con bloques
- Tests de recursión con bloques

## Ejemplos de Uso

```javascript
// Análisis de datos con pasos intermedios
let analyzeData = rawData => {
    let cleaned = filter(x => x != null, rawData);
    let normalized = map(x => x / 100, cleaned);
    let stats = {
        mean: sum(normalized) / length(normalized),
        max: max(normalized),
        min: min(normalized)
    };
    stats
}

// Algoritmo complejo
let quicksort = arr => {
    if (length(arr) <= 1) {
        return arr;
    };

    let pivot = arr[0];
    let rest = arr[1..];
    let less = filter(x => x < pivot, rest);
    let greater = filter(x => x >= pivot, rest);

    concat(rec(less), [pivot], rec(greater))
}

// Constructor con lógica
let makeValidator = rules => {
    let ruleCount = length(rules);
    {
        validate: data => {
            let errors = [];
            let i = 0;
            while (i < ruleCount) {
                let rule = rules[i];
                if (!rule.check(data)) {
                    errors = concat(errors, [rule.message]);
                };
                i = i + 1;
            };
            { valid: length(errors) == 0, errors: errors }
        }
    }
}
```

## Notas de Diseño

- **Ciudadanos de primera clase:** Las funciones con bloques siguen siendo valores
- **Tipado dinámico:** El tipo de retorno se determina en runtime
- **Scope léxico:** Variables capturadas correctamente (ya implementado con Rc<Environment>)
- **No side effects globales:** Usar closures y records para estado mutable
