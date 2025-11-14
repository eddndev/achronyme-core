# SOC Grammar Reference

**Guía completa de la gramática del lenguaje SOC (Scientific Operations Calculator).**

> **Nota**: Este documento complementa `src/grammar.pest`. Para la especificación formal, consulta el archivo `.pest`.

## 📖 Tabla de contenidos

1. [Conceptos básicos](#conceptos-básicos)
2. [Literales](#literales)
3. [Operadores](#operadores)
4. [Funciones](#funciones)
5. [Estructuras de control](#estructuras-de-control)
6. [Colecciones](#colecciones)
7. [Declaraciones](#declaraciones)
8. [Sistema de módulos](#sistema-de-módulos)

---

## Conceptos básicos

### Comentarios
```soc
// Comentarios de línea simple
let x = 42  // comentario al final de línea
```

### Separación de statements
```soc
// Punto y coma explícito
let a = 1; let b = 2; a + b

// Newline implícito (nuevo en SOC)
let a = 1
let b = 2
a + b
```

### Whitespace
- Espacios, tabs y `\r` son **ignorados**
- Newlines `\n` son **significativos** (separan statements)

---

## Literales

### Números
```soc
42          // Entero
3.14        // Float
-17         // Negativo
1.5e-10     // Notación científica
2.5E+3      // También válido
```

**Regla**: `"-"? ~ ASCII_DIGIT+ ~ ("." ~ ASCII_DIGIT+)? ~ (^"e" ~ ("+" | "-")? ~ ASCII_DIGIT+)?`

### Booleanos
```soc
true
false
```

### Complejos
```soc
3i          // Imaginario puro
-2i         // Imaginario negativo
2 + 3i      // Complejo completo (expresión aritmética)
```

**Nota**: `2+3i` se parsea como `BinaryOp(Add, 2, 3i)`, no como literal.

### Strings
```soc
"Hello, world!"
"Line 1\nLine 2"       // Newline
"Tab\tseparated"       // Tab
"Quote: \"Hi\""        // Escaped quote
"Backslash: \\"        // Escaped backslash
```

**Escapes soportados**: `\n`, `\t`, `\r`, `\\`, `\"`

### Identificadores
```soc
x
my_variable
PI
sin
_internal
```

**Regla**: Empieza con letra o `_`, seguido de letras/dígitos/`_`.

**Palabras reservadas** (no pueden ser identificadores):
```
let, mut, rec, self, true, false, import, from, export, as, return, while
```

---

## Operadores

### Precedencia (de mayor a menor)

| Nivel | Operador | Descripción | Ejemplo |
|-------|----------|-------------|---------|
| 1 | `^` | Potencia | `2^3 = 8` |
| 2 | `-` (unario) | Negación | `-x` |
| 3 | `*`, `/`, `%` | Multiplicación, división, módulo | `4 * 5 / 2` |
| 4 | `+`, `-` | Suma, resta | `1 + 2 - 3` |
| 5 | `>`, `<`, `>=`, `<=` | Comparación | `x > 5` |
| 6 | `==`, `!=` | Igualdad | `x == 10` |
| 7 | `!` (unario) | NOT lógico | `!flag` |
| 8 | `&&` | AND lógico | `a && b` |
| 9 | `||` | OR lógico | `a || b` |

### Asociatividad
- **Izquierda**: `+`, `-`, `*`, `/`, `%`
- **Derecha**: `^` (potencia)
- **No asociativo**: Comparaciones (no se puede hacer `a < b < c`)

### Ejemplos
```soc
2 + 3 * 4       // = 14 (no 20)
2^3^2           // = 512 (2^(3^2), asociatividad derecha)
x > 5 && y < 10 // Comparación + lógica
!flag || ready  // NOT tiene precedencia sobre OR
```

---

## Funciones

### Llamadas a funciones
```soc
sin(3.14)
map(f, [1, 2, 3])
max(10, 20, 30)
```

### Lambdas (funciones anónimas)
```soc
// Lambda simple
x => x * 2

// Lambda multi-parámetro
(a, b) => a + b

// Lambda con bloque
x => do {
    let y = x * 2
    y + 10
}

// Currying (múltiples flechas)
x => y => x + y
```

**Sintaxis**: `params => body`
- `params`: Identificador único o `(id1, id2, ...)`
- `body`: Expresión o bloque `do { ... }`

### Recursión
```soc
let factorial = n => if n <= 1 then 1 else n * rec(n - 1)
//                                              ^^^
//                                        auto-referencia
```

**Palabra clave `rec`**: Referencia a la función actual (permite recursión en lambdas anónimas).

### Tail Call Optimization (TCO)
SOC detecta y optimiza llamadas recursivas en posición de cola:
```soc
// TCO: el último retorno es rec(...)
let sum_tail = (n, acc) => if n == 0 then acc else rec(n - 1, acc + n)

// NO TCO: hay operación después de rec
let sum_no_tco = n => if n == 0 then 0 else n + rec(n - 1)
```

---

## Estructuras de control

### If-then-else
```soc
if x > 0 then "positive" else "non-positive"

// Anidado
if x > 0 then
    "positive"
else if x < 0 then
    "negative"
else
    "zero"
```

**Sintaxis**: `if <condition> then <expr> else <expr>`

**Nota**: No hay `if` sin `else` (todo debe producir un valor).

### While loops
```soc
mut i = 0
while i < 10 do {
    print(i)
    i = i + 1
}
```

**Sintaxis**: `while <condition> do <body>`

### Piecewise (pattern matching)
```soc
piecewise {
    x < 0 -> "negative",
    x == 0 -> "zero",
    x > 0 -> "positive",
    _ -> "unreachable"  // default case
}
```

**Sintaxis**:
```
piecewise {
    case1 -> expr1,
    case2 -> expr2,
    _ -> default
}
```

**Evaluación**: Primera condición verdadera gana (cortocircuito).

### Do blocks
```soc
let result = do {
    let a = 10
    let b = 20
    a + b  // último valor = resultado del bloque
}
// result = 30
```

**Sintaxis**: `do { stmt1; stmt2; ...; final_expr }`

**Scope**: Variables declaradas dentro del bloque solo viven ahí.

---

## Colecciones

### Arrays (vectores y tensores)
```soc
[1, 2, 3]              // Vector 1D
[[1, 2], [3, 4]]       // Matriz 2x2
[[[1, 2]], [[3, 4]]]   // Tensor 3D

// Spread operator
let vec = [1, 2]
[0, ...vec, 3]         // = [0, 1, 2, 3]
```

**Indexing**:
```soc
let arr = [10, 20, 30]
arr[0]        // = 10
arr[1:3]      // = [20, 30] (slice)
arr[::2]      // = [10, 30] (step)
```

**Slicing** (estilo Python):
- `arr[start:end]` - Elementos desde `start` hasta `end-1`
- `arr[start:]` - Desde `start` hasta el final
- `arr[:end]` - Desde el inicio hasta `end-1`
- `arr[::step]` - Cada `step` elementos

### Records (objetos)
```soc
let person = {
    name: "Alice",
    age: 30,
    active: true
}

// Acceso a campos
person.name     // = "Alice"
person.age      // = 30
```

**Sintaxis**: `{ field1: expr1, field2: expr2, ... }`

### Edges (grafos)
```soc
// Arista dirigida
"A" -> "B"

// Arista no dirigida
"X" <> "Y"

// Con propiedades
"A" -> "B" { weight: 5, label: "edge1" }
```

**Acceso a propiedades**:
```soc
let edge = "A" -> "B" { weight: 10 }
edge.from      // = "A"
edge.to        // = "B"
edge.directed  // = true
edge.weight    // = 10
```

---

## Declaraciones

### Variables inmutables
```soc
let x = 10
let name = "Alice"
let func = x => x * 2
```

**Sintaxis**: `let <id> = <expr>`

**Shadowing**: Permitido (nueva binding, no modifica la anterior)
```soc
let x = 5
let x = x + 1  // x = 6 (nueva variable)
```

### Variables mutables
```soc
mut counter = 0
counter = counter + 1  // Asignación
```

**Sintaxis**:
- Declaración: `mut <id> = <expr>`
- Asignación: `<id> = <expr>`

### Return temprano
```soc
let process = x => do {
    if x < 0 then return "invalid"
    // más procesamiento
    return x * 2
}
```

**Sintaxis**: `return <expr>`

**Scope**: Solo dentro de funciones/bloques.

---

## Sistema de módulos

### Import
```soc
// Importar funciones específicas
import { sin, cos, tan } from "math"

// Con alias
import { mean as average } from "stats"

// De módulos personalizados
import { helper } from "./utils"
```

**Sintaxis**: `import { id1, id2, id3 as alias } from "module"`

**Búsqueda**:
1. Módulos built-in (`"math"`, `"stats"`, `"array"`, etc.)
2. Archivos relativos (`"./file"` → busca `file.soc`)

### Export
```soc
// stats.soc
let mean = vec => sum(vec) / len(vec)
let std = vec => sqrt(variance(vec))

export { mean, std }
```

**Sintaxis**: `export { id1, id2, id3 as alias }`

**Scope**: Solo valores definidos en el módulo actual.

---

## Gramática BNF simplificada

```ebnf
program     ::= statement*
statement   ::= declaration | expression

declaration ::= "let" id "=" expression
              | "mut" id "=" expression
              | "import" import_list "from" string
              | "export" export_list

expression  ::= primary
              | unary_op expression
              | expression binary_op expression
              | "if" expression "then" expression "else" expression
              | "while" expression "do" block
              | piecewise
              | lambda
              | call_expression

lambda      ::= params "=>" expression
params      ::= id | "(" id ("," id)* ")"

primary     ::= number | boolean | string | complex
              | id | array | record | edge
              | "(" expression ")"

array       ::= "[" (expression ("," expression)*)? "]"
record      ::= "{" (id ":" expression ("," id ":" expression)*)? "}"
edge        ::= string ("->" | "<>") string ("{" field_list "}")?
```

---

## 🧪 Testing de gramática

### Herramienta de debugging
```bash
# Parser interactivo
cargo run --example parse_debug

# Test específico
cargo test test_parse_lambda -- --nocapture
```

### Activar tracing de Pest
```bash
RUST_LOG=pest=trace cargo test
```

---

## 📚 Referencias

- **Archivo fuente**: [`src/grammar.pest`](src/grammar.pest)
- **Pest Book**: https://pest.rs/book/
- **Ejemplos de código**: [`examples/`](../../examples/)

---

## 🔧 Notas de implementación

### Ambigüedades resueltas

1. **Minus vs negación**:
   - `2 - 3` → binario (resta)
   - `-3` → unario (negación)
   - Resuelto por precedencia y contexto

2. **Function call vs paréntesis**:
   - `sin(x)` → FunctionCall
   - `(x + 1)` → ParenExpression
   - Resuelto por lookahead (identificador antes de `(`)

3. **Complex vs multiplicación**:
   - `3i` → ComplexLiteral
   - `3 * i` → BinaryOp
   - Resuelto por tokenización (`3i` es un token único)

### Limitaciones actuales

- **No hay error recovery**: Primer error detiene el parsing
- **Unicode limitado**: Identificadores solo ASCII
- **No hay macros**: No se expanden en parsing
- **Newlines significativos**: Puede causar errores sutiles

### Futuros cambios

- [ ] Soporte Unicode en identificadores
- [ ] Error recovery para mejor UX
- [ ] Parsing incremental (solo re-parsear cambios)
- [ ] Source maps para mejores mensajes de error
