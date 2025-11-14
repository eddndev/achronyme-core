# Achronyme Parser

**Convierte código fuente SOC (Scientific Operations Calculator) en un Árbol de Sintaxis Abstracta (AST).**

## 🎯 Responsabilidad

El parser es la primera etapa del pipeline de ejecución de SOC:

```
Texto SOC → [Parser] → AST → [Evaluator] → Resultado
```

### Funciones principales:
- **Lexing/Tokenización** - Divide el texto en tokens significativos
- **Parsing sintáctico** - Valida la gramática usando reglas PEG (Parsing Expression Grammar)
- **Construcción de AST** - Genera una estructura de datos que representa el programa

## 📦 Dependencias

### Externas:
- **`pest`** (v2.7) - Motor de parsing PEG
- **`pest_derive`** - Macros para generar el parser desde gramática

### Internas:
- **`achronyme-types`** - Tipos compartidos (función, complex numbers, etc.)

## 🔌 Usado por

- **`achronyme-eval`** - Evalúa el AST producido por el parser
- **`achronyme-repl`** - Interfaz interactiva que parsea input del usuario
- **`achronyme-cli`** - CLI que parsea archivos `.soc`

## 🏗️ Arquitectura

```
achronyme-parser/
├── src/
│   ├── grammar.pest          # Gramática PEG (formato Pest)
│   ├── ast.rs                # Definición del AST (tipos de nodos)
│   ├── pest_parser.rs        # Parser generado por Pest
│   ├── lib.rs                # API pública (parse function)
│   └── parser/               # Builders del AST (Pest → AST)
│       ├── mod.rs
│       ├── primary.rs        # Literales, variables, identificadores
│       ├── expressions.rs    # Operadores binarios/unarios
│       ├── functions.rs      # Lambdas, llamadas a funciones
│       ├── control_flow.rs   # if, while, piecewise
│       ├── collections.rs    # Arrays, records, edges
│       ├── statements.rs     # let, mut, import, export
│       └── util.rs           # Helpers de parsing
└── tests/
    └── parser_tests.rs       # Tests de integración
```

## 📚 Conceptos clave

### AST (Abstract Syntax Tree)
Representación estructurada del código que abstrae los detalles sintácticos:

```soc
let x = 2 + 3 * 4
```

Se convierte en:

```
VariableDecl {
    name: "x",
    initializer: BinaryOp {
        op: Add,
        left: Number(2),
        right: BinaryOp {
            op: Multiply,
            left: Number(3),
            right: Number(4)
        }
    }
}
```

### PEG (Parsing Expression Grammar)
Sistema de parsing determinístico que:
- No tiene ambigüedades (primera coincidencia gana)
- Soporta lookahead/lookbehind sin backtracking complejo
- Más fácil de mantener que gramáticas LALR/LR

## 🚀 Uso

```rust
use achronyme_parser::parse;

let source = "let x = 2 + 3";
let ast = parse(source)?;

// ast[0] = VariableDecl { name: "x", initializer: BinaryOp { ... } }
```

## 📖 Documentación interna

Para entender la implementación en detalle:
- [src/README.md](src/README.md) - Arquitectura interna del parser
- [src/grammar.pest](src/grammar.pest) - Gramática completa del lenguaje SOC

## 🧪 Testing

```bash
# Ejecutar tests del parser
cargo test --package achronyme-parser

# Ver qué parsea la gramática (debugging)
cargo run --example parse_debug -- "2 + 3"
```

## 🔧 Extensión

Para agregar nuevas características sintácticas:

1. **Actualizar gramática** (`src/grammar.pest`)
2. **Agregar variante al AST** (`src/ast.rs`)
3. **Implementar builder** (en `src/parser/*.rs`)
4. **Agregar tests** (`tests/parser_tests.rs`)

Ver [CONTRIBUTING.md](../../CONTRIBUTING.md) para guías detalladas.

## 📊 Estadísticas

- **Líneas de código**: ~1,200 LOC
- **Nodos AST**: 30+ tipos diferentes
- **Reglas de gramática**: ~80 reglas PEG
- **Cobertura de tests**: >85%
