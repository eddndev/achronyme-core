# Parser Implementation

**Documentación interna de la implementación del parser de SOC.**

## 🏛️ Arquitectura de 2 fases

### Fase 1: Pest Parser (Gramática → Parse Tree)
```
Código SOC → [Pest] → Parse Tree (Pairs<Rule>)
```

**Archivo**: `pest_parser.rs` + `grammar.pest`

Pest procesa el código fuente según las reglas de gramática y produce un árbol de pares (Pairs) que representan las coincidencias sintácticas.

### Fase 2: AST Builder (Parse Tree → AST)
```
Parse Tree → [AstParser] → AST (Vec<AstNode>)
```

**Directorio**: `parser/`

Convierte el parse tree de Pest en nuestro AST tipado y estructurado.

## 📁 Estructura de módulos

```
src/
├── lib.rs                    # API pública: parse()
├── grammar.pest              # Gramática PEG del lenguaje SOC
├── pest_parser.rs            # Punto de entrada Pest
├── ast.rs                    # Definición de tipos AST
│
└── parser/                   # AST Builders
    ├── mod.rs                # AstParser struct principal
    ├── primary.rs            # Literales básicos
    ├── expressions.rs        # Expresiones y operadores
    ├── functions.rs          # Lambdas y llamadas
    ├── control_flow.rs       # if, while, piecewise
    ├── collections.rs        # Arrays, records, edges
    ├── statements.rs         # let, mut, import, export
    └── util.rs               # Helpers comunes
```

## 🔄 Flujo de parsing

### 1. Entrada del usuario
```rust
let source = "let x = 2 + 3 * 4";
```

### 2. Pest parsea según gramática
```rust
// pest_parser.rs
pub fn parse(source: &str) -> Result<Vec<AstNode>, String> {
    let pairs = SOCParser::parse(Rule::program, source)?;
    // pairs contiene el parse tree de Pest
}
```

### 3. AstParser construye el AST
```rust
// parser/mod.rs
let mut parser = AstParser::new();
let ast = parser.parse_program(pairs)?;
```

### 4. Delegación a builders especializados
```rust
// parser/statements.rs
fn build_variable_decl(&mut self, pair: Pair<Rule>) -> Result<AstNode, String> {
    // Extrae nombre e inicializador
    // Construye AstNode::VariableDecl
}
```

## 🎯 Responsabilidades de cada módulo

### `primary.rs` - Literales y primitivas
Parsea los elementos más básicos del lenguaje:
- Números: `42`, `3.14`, `1e-10`
- Booleanos: `true`, `false`
- Strings: `"Hello"`, `"Escape: \n"`
- Complex: `3+4i`, `2i`
- Identificadores: `x`, `my_var`, `rec`, `self`

### `expressions.rs` - Operadores y precedencia
Maneja expresiones con operadores:
- **Binarios**: `+`, `-`, `*`, `/`, `^`, `%`
- **Comparación**: `>`, `<`, `>=`, `<=`, `==`, `!=`
- **Lógicos**: `&&`, `||`
- **Unarios**: `-x`, `!flag`
- **Precedencia**: Respeta orden de operaciones matemáticas

### `functions.rs` - Funciones y lambdas
Parsea construcciones funcionales:
- **Lambdas**: `x => x * 2`, `(a, b) => a + b`
- **Currying**: `x => y => x + y`
- **Llamadas**: `sqrt(16)`, `map(f, [1,2,3])`
- **CallExpression**: `f(1, 2)`, `rec(n-1)`

### `control_flow.rs` - Estructuras de control
Gestiona flujo del programa:
- **Condicionales**: `if x > 0 then y else z`
- **Loops**: `while condition do body`
- **Pattern matching**: `piecewise { case1 -> expr1, case2 -> expr2, _ -> default }`
- **Bloques**: `do { stmt1; stmt2; result }`

### `collections.rs` - Estructuras de datos
Parsea colecciones complejas:
- **Arrays**: `[1, 2, 3]`, `[[1,2], [3,4]]`
- **Records**: `{ name: "Alice", age: 30 }`
- **Edges (grafos)**: `"A" -> "B"`, `"X" <> "Y" { weight: 5 }`
- **Indexing**: `arr[0]`, `matrix[i, j]`, `vec[1:5]`

### `statements.rs` - Declaraciones
Maneja statements top-level:
- **Variables**: `let x = 10`
- **Mutables**: `mut counter = 0`
- **Asignación**: `x = x + 1`
- **Imports**: `import { sin, cos } from "math"`
- **Exports**: `export { mean, std }`
- **Return**: `return result`

### `util.rs` - Utilidades
Funciones helper compartidas:
- Extracción de valores de Pairs
- Conversión de tipos
- Manejo de errores
- Validaciones comunes

## 🔍 Ejemplo de parsing detallado

### Código fuente:
```soc
let factorial = n => if n <= 1 then 1 else n * factorial(n - 1)
```

### Parse Tree (Pest):
```
Rule::variable_decl
├── Rule::identifier ("factorial")
└── Rule::lambda
    ├── Rule::param ("n")
    └── Rule::if_expr
        ├── Rule::condition
        │   └── Rule::binary_op (<=)
        │       ├── Rule::identifier ("n")
        │       └── Rule::number (1)
        ├── Rule::then_expr
        │   └── Rule::number (1)
        └── Rule::else_expr
            └── Rule::binary_op (*)
                ├── Rule::identifier ("n")
                └── Rule::call_expr
                    ├── Rule::identifier ("factorial")
                    └── Rule::args
                        └── Rule::binary_op (-)
```

### AST resultante:
```rust
AstNode::VariableDecl {
    name: "factorial",
    initializer: Box::new(
        AstNode::Lambda {
            params: vec!["n"],
            body: Box::new(
                AstNode::If {
                    condition: Box::new(
                        AstNode::BinaryOp {
                            op: BinaryOp::Lte,
                            left: Box::new(AstNode::VariableRef("n")),
                            right: Box::new(AstNode::Number(1.0))
                        }
                    ),
                    then_expr: Box::new(AstNode::Number(1.0)),
                    else_expr: Box::new(
                        AstNode::BinaryOp {
                            op: BinaryOp::Multiply,
                            left: Box::new(AstNode::VariableRef("n")),
                            right: Box::new(
                                AstNode::CallExpression {
                                    callee: Box::new(AstNode::VariableRef("factorial")),
                                    args: vec![
                                        AstNode::BinaryOp {
                                            op: BinaryOp::Subtract,
                                            left: Box::new(AstNode::VariableRef("n")),
                                            right: Box::new(AstNode::Number(1.0))
                                        }
                                    ]
                                }
                            )
                        }
                    )
                }
            )
        }
    )
}
```

## 🐛 Debugging

### Activar tracing de Pest:
```bash
RUST_LOG=pest=trace cargo test test_parse_lambda
```

### Inspeccionar Parse Tree:
```rust
let pairs = SOCParser::parse(Rule::expression, "2 + 3")?;
for pair in pairs {
    println!("{:#?}", pair);
}
```

## 📐 Patrones de diseño

### Builder Pattern
Cada módulo en `parser/` es un builder que convierte Pairs → AstNode:
```rust
impl AstParser {
    fn build_if_expr(&mut self, pair: Pair<Rule>) -> Result<AstNode, String> {
        // Extrae condición, then, else de los inner pairs
        // Construye AstNode::If recursivamente
    }
}
```

### Recursive Descent
El parser sigue la estructura de la gramática:
- Las reglas de gramática se mapean a funciones
- Las funciones se llaman recursivamente
- La recursión sigue la anidación del código fuente

### Error Recovery
Actualmente **no hay recovery** - el primer error detiene el parsing:
```rust
Err("Parse error at line 5: expected ';' after statement")
```

Para mejor UX, se podría implementar error recovery en el futuro.

## 🧪 Testing

Los tests se encuentran en `tests/parser_tests.rs`:
```bash
# Test completo del parser
cargo test --package achronyme-parser

# Test específico
cargo test test_parse_lambda
```

## 🔧 Extender el parser

### Agregar nuevo tipo de expresión:

1. **Actualizar `grammar.pest`**:
```pest
spread_expr = { "..." ~ expression }
```

2. **Agregar variante en `ast.rs`**:
```rust
pub enum AstNode {
    // ...
    Spread { expr: Box<AstNode> },
}
```

3. **Implementar builder en `parser/expressions.rs`**:
```rust
fn build_spread_expr(&mut self, pair: Pair<Rule>) -> Result<AstNode, String> {
    let inner = pair.into_inner().next().unwrap();
    let expr = self.build_ast_from_expr(inner)?;
    Ok(AstNode::Spread { expr: Box::new(expr) })
}
```

4. **Actualizar dispatcher en `parser/mod.rs`**:
```rust
Rule::spread_expr => self.build_spread_expr(pair),
```

5. **Agregar test**:
```rust
#[test]
fn test_parse_spread() {
    let ast = parse("...[1, 2, 3]").unwrap();
    assert!(matches!(ast[0], AstNode::Spread { .. }));
}
```

## 📚 Referencias

- [Pest Book](https://pest.rs/book/) - Documentación oficial de Pest
- [PEG Wikipedia](https://en.wikipedia.org/wiki/Parsing_expression_grammar) - Teoría de PEG
- [AST Design](../../docs/core/ast-design.md) - Decisiones de diseño del AST (si existe)
