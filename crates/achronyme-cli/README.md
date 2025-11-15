# Achronyme CLI

**Interfaz de línea de comandos interactiva para el lenguaje SOC (Scientific Operations Calculator).**

## 🎯 Responsabilidad

Este crate proporciona tres modos de ejecución para el lenguaje Achronyme:

```
┌─────────────────┐
│  achronyme CLI  │
└─────────────────┘
        ↓
┌─────────────────────────────────┐
│  Modo 1: REPL Interactivo       │
│  achronyme                      │
│  → ach[1]> 2 + 2                │
│  → 4                            │
└─────────────────────────────────┘

┌─────────────────────────────────┐
│  Modo 2: Ejecución de archivo   │
│  achronyme script.soc           │
│  → Ejecuta todo el archivo      │
└─────────────────────────────────┘

┌─────────────────────────────────┐
│  Modo 3: Expresión única        │
│  achronyme "2 + 2"              │
│  → 4                            │
└─────────────────────────────────┘
```

### Funciones principales:
- **REPL interactivo** con historial, syntax highlighting y autocompletado
- **Ejecución de scripts** `.soc` y `.ach`
- **Evaluación de expresiones** desde la línea de comandos
- **Gestión de estado** persistente durante sesiones REPL
- **Multi-line input** con detección inteligente de expresiones incompletas

## 📦 Dependencias

### Internas:
- **`achronyme-parser`** - Parsea código fuente a AST
- **`achronyme-eval`** - Evalúa el AST y ejecuta operaciones
- **`achronyme-types`** - Tipos compartidos (`Value`, `Complex`, `Tensor`, etc.)

### Externas:
- **`rustyline`** (v14.0) - REPL con edición de línea, historial, y autocompletado
- **`nu-ansi-term`** (v0.50) - Syntax highlighting con colores ANSI
- **`pest`** (v2.7) - Usado para detección de expresiones incompletas
- **`dirs`** (v5.0) - Rutas de configuración del sistema (historial)

## 🔌 Usado por

Este es el **binario final** del proyecto:
- **Usuarios finales** - Interfaz principal para ejecutar código SOC
- **Scripts de automatización** - Puede ejecutar archivos `.soc` en pipelines
- **Desarrolladores** - REPL para probar características del lenguaje

## 🏗️ Arquitectura de alto nivel

```
Entrada del usuario
        ↓
┌───────────────────┐
│  main.rs          │  ← Entry point, maneja argumentos
│  - REPL mode      │
│  - File mode      │
│  - Expression mode│
└───────────────────┘
        ↓
┌───────────────────┐
│  ReplHelper       │  ← Integración con rustyline
│  - Highlighter    │     • Syntax highlighting
│  - Completer      │     • Tab completion
│  - Hinter         │     • Sugerencias inline
│  - Validator      │
└───────────────────┘
        ↓
┌───────────────────┐
│  highlighter.rs   │  ← Colorización de sintaxis
│  - Números        │
│  - Operadores     │
│  - Keywords       │
│  - Funciones      │
└───────────────────┘
        ↓
┌───────────────────┐
│  Evaluator        │  ← De achronyme-eval
│  - eval_str()     │
│  - Environment    │
└───────────────────┘
        ↓
    Resultado
```

## 🚀 Uso

### Instalación

```bash
# Compilar desde source
cd crates/achronyme-cli
cargo build --release

# El binario se genera en:
# target/release/achronyme (Linux/macOS)
# target\release\achronyme.exe (Windows)
```

### Modo 1: REPL Interactivo

```bash
$ achronyme
Achronyme REPL v0.1.0
Type 'exit' or 'quit' to exit, 'help' for help, 'clear' to clear screen

ach[1]> let x = 5
ach[2]> let f = y => x * y
ach[3]> f(10)
50
ach[4]> map(x => x^2, [1, 2, 3, 4])
[1, 4, 9, 16]
ach[5]> exit
Goodbye!
```

#### Características del REPL:

**Comandos especiales:**
- `help` - Muestra ayuda sobre comandos y características
- `clear` - Limpia pantalla y reinicia el entorno
- `cls` - Limpia pantalla (mantiene entorno)
- `exit` / `quit` - Sale del REPL

**Historial de comandos:**
- ↑/↓ - Navegar historial
- Ctrl+R - Búsqueda en historial
- Guardado automático en `~/.achronyme_history`

**Syntax Highlighting:**
```
ach[1]> let x = sin(pi/2) + 3.14i
        ^^^ ^   ^^^       ^ ^^^^
         │  │    │        │  │
         │  │    │        │  └─ Números complejos (cyan claro)
         │  │    │        └──── Operadores (rojo bold)
         │  │    └───────────── Funciones built-in (verde bold)
         │  └────────────────── Variables (blanco)
         └───────────────────── Keywords (púrpura bold)
```

**Autocompletado:**
```
ach[1]> si<TAB>
sin  sinh

ach[1]> di<TAB>
diff

ach[1]> map(x => x^2, lin<TAB>
linspace
```

**Multi-line input:**
```
ach[1]> let f = x => do {
     ...>     let squared = x^2
     ...>     let cubed = x^3
     ...>     squared + cubed
     ...> }
<function>
```

El REPL detecta automáticamente cuando una expresión está incompleta (paréntesis sin cerrar, bloques `do`, etc.) y permite continuar en la siguiente línea.

### Modo 2: Ejecución de archivos

```bash
# Ejecutar un script .soc
$ achronyme examples/01-vector-operations.soc
50

# Ejecutar un script .ach
$ achronyme script.ach
[1, 4, 9, 16, 25]
```

**Ejemplo de script** (`vector_ops.soc`):
```javascript
// Vector operations demo
let v = [1, 2, 3, 4, 5]
let v_squared = map(x => x^2, v)
let v_filtered = filter(x => x > 10, v_squared)
reduce((acc, x) => acc + x, 0, v_filtered)
```

**Salida:**
```bash
$ achronyme vector_ops.soc
50
```

### Modo 3: Expresión única

```bash
# Expresiones simples
$ achronyme "2 + 2"
4

$ achronyme "sin(pi/2)"
1

# Expresiones complejas
$ achronyme "map(x => x^2, [1, 2, 3, 4])"
[1, 4, 9, 16]

# Numerical calculus
$ achronyme "diff(x => x^2, 3, 1e-5)"
6.000009999243267

# Linear programming
$ achronyme "linprog([3, 5], [[1, 0], [0, 2], [3, 2]], [4, 12, 18], [0, 0])"
{objective: 36, solution: [2, 6], status: "optimal"}
```

## 📊 Características clave

### 1. Syntax Highlighting en vivo

El REPL usa un **lexer simplificado** para colorear el código mientras escribes:

- **Keywords** (`let`, `true`, `false`) → Púrpura bold
- **Funciones built-in** (`sin`, `map`, `diff`) → Verde bold
- **Números** (`123`, `3.14`) → Cyan
- **Números complejos** (`2+3i`) → Cyan claro
- **Operadores** (`+`, `-`, `*`, `/`, `^`) → Rojo bold
- **Comparadores** (`==`, `!=`, `<`, `>`) → Rojo bold
- **Brackets** (`[]`, `()`) → Azul claro bold
- **Strings** → Blanco
- **Variables** → Blanco

### 2. Detección inteligente de multi-line

El REPL usa una **estrategia híbrida** para detectar si una expresión está completa:

```rust
// 1. Fast check: balance de delimitadores
if !has_balanced_delimiters(input) {
    return true; // Definitivamente incompleto
}

// 2. Parser check: confirmar completitud
match parse(input) {
    Ok(_) => false,  // Completo
    Err(e) if e.contains("EOI") => true,  // Incompleto
    Err(_) => false, // Completo pero inválido (mostrará error)
}
```

**Ejemplos:**

```
ach[1]> let x = [1, 2, 3
     ...> ]                    ← Continúa porque '[' sin cerrar
ach[2]> [1, 2, 3, 4]
[1, 2, 3, 4]                   ← Evaluado inmediatamente

ach[3]> if(true, {
     ...>     let x = 5
     ...>     x * 2
     ...> })                   ← Continúa hasta cerrar el bloque
10
```

### 3. Formateo de resultados

El CLI formatea resultados de manera legible:

```javascript
// Números
ach[1]> 42
42

// Complejos
ach[2]> 3 + 4i
3+4i

ach[3]> 3 - 4i
3-4i

// Vectores
ach[4]> [1, 2, 3, 4]
[1, 2, 3, 4]

// Matrices
ach[5]> [[1, 2], [3, 4]]
[[1, 2],
 [3, 4]]

// Records
ach[6]> { name: "Alice", age: 30 }
{ age: 30, name: "Alice" }  // Ordenado alfabéticamente

// Edges (grafos)
ach[7]> "A" -> "B" { weight: 5 }
A -> B: { weight: 5 }

// Funciones
ach[8]> x => x^2
<function>
```

### 4. Gestión de errores

```
ach[1]> 2 / 0
Error: Division by zero

ach[2]> let x = 5
ach[3]> y + 10
Error: Variable 'y' not found

ach[4]> map(x => x^2, 123)
Error: Expected array, found Number
```

### 5. Persistencia de historial

El historial de comandos se guarda automáticamente en:
- **Linux/macOS**: `~/.achronyme_history`
- **Windows**: `%USERPROFILE%\.achronyme_history`

El historial persiste entre sesiones, permitiendo acceder a comandos previos con ↑/↓.

## 🧪 Testing

### Compilar y ejecutar

```bash
# Compilar
cargo build --package achronyme-cli

# Ejecutar (modo REPL)
cargo run --package achronyme-cli

# Ejecutar con archivo
cargo run --package achronyme-cli -- examples/soc/01-vector-operations.soc

# Ejecutar expresión
cargo run --package achronyme-cli -- "2 + 2"
```

### Testing interactivo

```bash
# Iniciar REPL
$ cargo run --package achronyme-cli

# Probar funciones básicas
ach[1]> 2 + 2
4

# Probar variables
ach[2]> let x = 10
ach[3]> x * 2
20

# Probar lambdas
ach[4]> let f = x => x^2
ach[5]> f(5)
25

# Probar higher-order functions
ach[6]> map(f, [1, 2, 3, 4])
[1, 4, 9, 16]
```

## 🔧 Configuración

### Features disponibles

El CLI usa un **feature flag** para habilitar dependencias opcionales:

```toml
[features]
default = ["cli"]
cli = ["dep:rustyline", "dep:nu-ansi-term", "dep:pest", "dep:dirs"]
```

Para compilar sin las características del CLI (solo library):

```bash
cargo build --package achronyme-cli --no-default-features
```

### Personalización del REPL

El archivo `repl_helper.rs` define las funciones disponibles para autocompletado. Para agregar nuevas funciones:

```rust
// En repl_helper.rs
pub fn new() -> Self {
    let functions = vec![
        // ... funciones existentes ...
        "new_function",  // ← Agregar aquí
    ];
    // ...
}
```

## 📖 Documentación interna

Para entender la implementación en detalle:
- [src/README.md](src/README.md) - Arquitectura técnica interna

## 🎨 Ejemplos avanzados

### REPL: Numerical calculus

```javascript
ach[1]> // Derivative of x^2 at x=3
ach[2]> diff(x => x^2, 3, 1e-5)
6.000009999243267

ach[3]> // Integral of sin from 0 to pi
ach[4]> integral(sin, 0, 3.14159, 100)
1.9999983550656628
```

### REPL: Linear programming

```javascript
ach[1]> // Maximize 3x + 5y subject to constraints
ach[2]> let objective = [3, 5]
ach[3]> let constraints = [[1, 0], [0, 2], [3, 2]]
ach[4]> let rhs = [4, 12, 18]
ach[5]> let bounds = [0, 0]
ach[6]> linprog(objective, constraints, rhs, bounds)
{objective: 36, solution: [2, 6], status: "optimal"}
```

### REPL: Graph algorithms

```javascript
ach[1]> let graph = [
     ...>     "A" -> "B" { weight: 4 },
     ...>     "A" -> "C" { weight: 2 },
     ...>     "B" -> "C" { weight: 1 },
     ...>     "B" -> "D" { weight: 5 },
     ...>     "C" -> "D" { weight: 3 }
     ...> ]
ach[2]> import { dijkstra } from "graphs"
ach[3]> dijkstra(graph, "A", "D")
{distance: 6, path: ["A", "C", "D"]}
```

### Archivo: Data analysis pipeline

```javascript
// analysis.soc
let data = [1.2, 2.3, 3.4, 4.5, 5.6, 6.7, 7.8, 8.9]

// Estadísticas básicas
import { mean, median, std } from "stats"
let avg = mean(data)
let med = median(data)
let stdev = std(data)

print("Mean: " + avg)
print("Median: " + med)
print("Std Dev: " + stdev)

// FFT analysis
import { fft_mag } from "dsp"
let spectrum = fft_mag(data)
print("Spectrum: ")
spectrum
```

## 📊 Estadísticas

- **Líneas de código**: ~410 LOC
- **Archivos fuente**: 4 archivos (.rs)
- **Dependencias externas**: 4 crates
- **Built-in functions autocomplete**: 40+ funciones
- **Comandos REPL**: 4 comandos especiales

## 🎯 Principios de diseño

1. **User-friendly REPL** - Experiencia interactiva fluida con historial y colores
2. **Multi-mode execution** - REPL, archivo, o expresión única según necesidad
3. **Smart multi-line** - Detección automática de expresiones incompletas
4. **Rich formatting** - Output legible para todos los tipos de valores
5. **Cross-platform** - Funciona en Windows, Linux y macOS
6. **Minimal dependencies** - Solo lo necesario para UX excepcional

## 🔗 Ver también

- [achronyme-parser](../achronyme-parser/README.md) - Parsea código fuente a AST
- [achronyme-eval](../achronyme-eval/README.md) - Motor de evaluación usado por el CLI
- [achronyme-types](../achronyme-types/README.md) - Tipos de valores (`Value`, `Complex`, `Tensor`)
- [Ejemplos SOC](../../examples/soc/) - Colección de scripts de ejemplo
