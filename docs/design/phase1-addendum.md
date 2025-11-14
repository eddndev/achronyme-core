# Phase 1 Implementation - Addendum

**Fecha**: 2025-01-14
**Estado**: Ready to implement

## Prerequisites Completed ✅

Antes de comenzar Phase 1, se completaron los siguientes prerequisitos:

- [x] **While loops** (`while(cond) { body }`) - Implementado con 21 tests
- [x] **Return statement** con propagación correcta a través de do blocks
- [x] **Do blocks** con scoping apropiado
- [x] **Mutable variables** (`mut x = value`)

## Cambios Requeridos a la Especificación

### 1. Break Statement (IMPORTANTE)

La especificación original usa `break` en varios lugares, pero **`break` no está implementado**.

**Opciones**:

**Opción A**: Usar `return` en lugar de `break` (recomendado para MVP)

```achronyme
// ANTES (en spec original - línea 137)
let take = (n, iterator) => do {
    mut result = []
    mut i = 0
    while(i < n) {
        let item = iterator.next()
        if(item.done) { break }  // ❌ break no existe
        result = push(result, item.value)
        i = i + 1
    }
    result
}

// DESPUÉS (con return)
let take = (n, iterator) => do {
    mut result = []
    mut i = 0
    while(i < n) {
        let item = iterator.next()
        if(item.done) { return result }  // ✅ return funciona
        result = push(result, item.value)
        i = i + 1
    }
    result
}
```

**Opción B**: Implementar `break` y `continue` antes de Phase 1

Si se elige esta opción, se necesita:
- Agregar `break` y `continue` como keywords
- Implementar `Value::BreakSignal` y `Value::ContinueSignal`
- Manejar estas señales en while/for loops
- Agregar tests

### 2. Keyword Management

**IMPORTANTE**: `for` NO debe ser keyword, similar a `if`.

Razón: La gramática usa positive lookahead para diferenciar:
- `for(x in iter) { ... }` → for loop (nota el `{`)
- `for(a, b, c)` → función llamada "for"

```pest
// Correcto - NO agregar "for" a keywords
keyword = _{
    ("let" | "mut" | "rec" | "self" | "true" | "false" |
     "import" | "from" | "export" | "as" | "return" | "while" |
     "yield" | "generate") ~ !ASCII_ALPHANUMERIC
}

// For loop con lookahead
for_loop = {
    "for" ~ "(" ~ identifier ~ "in" ~ expr ~ ")" ~ &"{" ~ block
}
```

### 3. Issues Conocidos

**TCO Deep Recursion Tests Failing**

Actualmente hay 8 tests en `test_tco_deep_recursion.rs` que fallan con:
```
Parse error: expected lambda_body
```

**Causa**: Lambdas con newline después de `=>` no parsean correctamente:
```achronyme
// ❌ Falla
let sum = (n, acc) =>
    if(n <= 0, acc, rec(n-1, acc+n))

// ✅ Funciona
let sum = (n, acc) => if(n <= 0, acc, rec(n-1, acc+n))
```

**Impacto en Phase 1**: Ninguno, los generators no requieren esta sintaxis.

**Fix futuro**: Actualizar grammar para permitir newlines opcionales después de `=>`.

## Orden de Implementación Recomendado

Basado en las dependencias reales:

### Semana 1: Generators sin For-In

1. Grammar: `yield`, `generate`
2. AST: `Yield`, `GenerateBlock`
3. Value: `Value::Generator`, `GeneratorState`
4. Parser: Build yield y generate blocks
5. Evaluator: Create generators (sin ejecutar)

### Semana 2: Generator Execution

6. Evaluator: `resume_generator()`
7. Evaluator: `execute_until_yield()`
8. Field access: `generator.next()`
9. Tests: Generators básicos (sin for-in)

### Semana 3: For-In y Helpers

10. Grammar: `for_loop` (con lookahead `&"{"`)
11. AST: `ForLoop`
12. Evaluator: `evaluate_for_loop()`
13. Built-in helpers: `take`, `collect`, `map_iter`, `filter_iter`
14. Tests comprehensivos y documentación

## Validación Pre-Implementación

Antes de comenzar, verificar:

```bash
# 1. While loops funcionan
cargo test test_while_loop
# Debe pasar: 24 passed

# 2. Return propagation funciona
cargo test test_return_statement
# Debe pasar: todos los tests

# 3. Build limpio
cargo build
# No debe tener warnings relacionados con while/return
```

## Próximos Pasos

1. Revisar esta addendum
2. Decidir: ¿Implementar `break` primero o usar `return`?
3. Actualizar `phase1-iterators-implementation.md` con los cambios
4. Comenzar implementación siguiendo el plan de 3 semanas

## Notas de Sesión Actual

**Implementado**: While loops completamente funcionales
**Tests**: 21 tests de while pasando
**Documentación**: `docs/language/while-loops.md` creada
**Archivos modificados**:
- `grammar.pest` - Agregado `while_expr`, keyword "while"
- `ast.rs` - Agregado `WhileLoop` node
- `pest_parser.rs` - Agregado `build_while_expr()`
- `evaluator.rs` - Agregado case para `WhileLoop`
- `handlers/control_flow.rs` - Implementado `evaluate_while()`
- `tco.rs` - Agregado soporte para `WhileLoop`
- `tests/test_while_loop.rs` - 21 tests comprehensivos

**No agregado a keywords**: "if", "else" (correctamente evitado por ambigüedad con función `if()`)
