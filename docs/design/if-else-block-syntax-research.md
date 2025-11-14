# Investigación: Implementación de Sintaxis if/else con Bloques

## Problema Identificado

Al intentar implementar la sintaxis `if expr { block } else { block }`, Pest presenta un error de ambigüedad: "expected record_field_or_spread" en el token `{`.

### Causa Raíz

El token `{` en la gramática actual puede iniciar:
1. **Record literal**: `{x: 10, y: 20}`
2. **Do block**: `do { ... }`
3. **If block** (deseado): `if expr { ... }`

Pest utiliza **PEG (Parsing Expression Grammar)**, que es un parser de **ordered choice** que NO hace backtracking tradicional. Cuando encuentra múltiples alternativas, prueba en orden y usa la primera que coincide, sin intentar otras si la elección inicial falla más adelante.

## Investigación Realizada

### Hallazgos Clave

1. **PEG es ordered choice**: El orden de las alternativas en la gramática es CRÍTICO
2. **No backtracking**: Una vez que una regla coincide parcialmente, Pest no vuelve atrás para probar otras alternativas
3. **Ambigüedad de curly braces**: Es un problema común en parsers PEG cuando múltiples construcciones usan `{`

### Soluciones Documentadas en la Comunidad

De la investigación en Stack Overflow y documentación de Pest:

1. **Negative Lookahead para Keywords**:
   ```pest
   keyword = @{ (^"if" | ^"else") ~ !identifier_continue }
   ```

2. **Ordenar alternativas de más específico a menos específico**:
   - Las reglas más específicas deben aparecer PRIMERO
   - Las reglas generales (como `expr`) deben estar al FINAL

3. **Restricciones sintácticas a nivel de diseño del lenguaje**:
   - Lenguajes como Go NO permiten struct literals en condiciones de if
   - Esta es una decisión de diseño del lenguaje para evitar ambigüedad

4. **Usar delimitadores diferentes**:
   - Algunos lenguajes usan `then`/`end` en lugar de `{}`
   - Otros requieren paréntesis obligatorios alrededor de la condición

## Soluciones Propuestas para Achronyme

### Opción 1: Paréntesis Obligatorios + Lookahead (RECOMENDADO)

```pest
// Sintaxis: if(expr) { block } else { block }
if_statement = {
    "if" ~ !ASCII_ALPHANUMERIC ~ "(" ~ expr ~ ")" ~ !"{" ~ if_block ~
    ("else" ~ !ASCII_ALPHANUMERIC ~ (if_statement | if_block))?
}

if_block = @{
    "{" ~ PUSH("") ~ (!("}" ~ PEEK) ~ ANY)* ~ DROP ~ "}"
}
```

**Ventajas**:
- Compatible con muchos lenguajes (JavaScript, Java, C, etc.)
- Claramente distingue la condición del bloque
- La función `if(cond, then, else)` seguiría funcionando

**Desventajas**:
- Sintaxis ligeramente más verbosa que `if expr { }`

### Opción 2: Uso de `do` para Bloques Multi-Declaración

```pest
// Sintaxis: if expr then_expr else else_expr
// Para múltiples declaraciones: if expr do { stmts } else do { stmts }
if_statement = {
    "if" ~ !ASCII_ALPHANUMERIC ~ expr ~
    (do_block | expr) ~
    ("else" ~ !ASCII_ALPHANUMERIC ~ (if_statement | do_block | expr))?
}
```

**Ventajas**:
- Reutiliza la sintaxis `do { }` existente
- No introduce ambigüedad con records
- Sintaxis simple para casos simples: `if x > 5 42 else 0`

**Desventajas**:
- Requiere `do { }` para bloques multi-declaración
- Menos familiar para programadores de C-like languages

### Opción 3: Sintaxis con `then`/`end` (Estilo Ruby/Lua)

```pest
if_statement = {
    "if" ~ !ASCII_ALPHANUMERIC ~ expr ~ "then" ~ sequence ~
    ("elseif" ~ expr ~ "then" ~ sequence)* ~
    ("else" ~ sequence)? ~
    "end"
}
```

**Ventajas**:
- Sin ambigüedad alguna
- Clara delimitación de bloques
- Familiar para usuarios de Ruby, Lua, VBScript

**Desventajas**:
- Sintaxis más verbosa
- Menos familiar para la mayoría de programadores modernos

### Opción 4: Atomic Blocks (Técnica Avanzada de Pest)

```pest
if_statement = {
    "if" ~ !ASCII_ALPHANUMERIC ~ expr ~ atomic_if_block ~
    ("else" ~ !ASCII_ALPHANUMERIC ~ (if_statement | atomic_if_block))?
}

atomic_if_block = ${  // $ = atomic rule
    "{" ~ (sequence | statement | expr) ~ "}"
}
```

**Ventajas**:
- Sintaxis deseada: `if expr { block }`
- Usa características avanzadas de Pest

**Desventajas**:
- Complejidad en la implementación
- Requiere entendimiento profundo de Pest
- Puede tener efectos secundarios inesperados

### Opción 5: Resolver en el Parser Post-Procesamiento

Implementar la gramática con ambigüedad controlada y resolverla en la fase de construcción del AST:

```rust
fn build_primary(pair: Pair<Rule>) -> Result<AstNode, String> {
    match inner.as_rule() {
        Rule::identifier => {
            // Lookahead: si el siguiente token es "(", podría ser if(...)
            // Si el siguiente token es "{"...
            // Aplicar lógica especial de desambiguación
        }
        // ...
    }
}
```

**Ventajas**:
- Control total sobre la resolución de ambigüedad
- Puede manejar casos edge complejos

**Desventajas**:
- Complejidad significativa en el código del parser
- Difícil de mantener
- Pest está diseñado para evitar esto

## Recomendación

**Implementar Opción 1: Paréntesis Obligatorios**

**Sintaxis final**:
```javascript
if(condition) {
    statement1;
    statement2
} else if(other_condition) {
    statement3
} else {
    statement4
}
```

**Razones**:
1. ✅ **Familiar**: Sintaxis estándar en JavaScript, Java, C, C++, Rust (con diferencias menores)
2. ✅ **Sin ambigüedad**: Los paréntesis separan claramente condición de bloque
3. ✅ **Compatible**: La función `if(cond, then, else)` existente sigue funcionando
4. ✅ **Mantenible**: Gramática clara y fácil de entender
5. ✅ **GUI-friendly**: Coincide con el ejemplo en `native-gui-system.md` ajustando levemente:
   ```javascript
   // Era: if ui.button("Increment") { ... }
   // Será: if(ui.button("Increment")) { ... }
   ```

## Próximos Pasos

1. Implementar Opción 1 con paréntesis obligatorios
2. Crear tests exhaustivos
3. Actualizar documentación y ejemplos
4. Considerar: En una versión futura, evaluar si la Opción 2 (do blocks) sería preferible para simplicidad

## Referencias

- [Pest Parser Syntax](https://pest.rs/book/grammars/syntax.html)
- [Stack Overflow: PEG Grammar Ambiguity](https://stackoverflow.com/questions/64018056/ambiguity-of-peg-grammar-with-pest-parser)
- [Stack Overflow: PEG If Statement Grammar](https://stackoverflow.com/questions/63544801/peg-what-is-wrong-wrong-with-my-grammar-for-if-statement)
- [Dangling Else Problem](https://en.wikipedia.org/wiki/Dangling_else)
