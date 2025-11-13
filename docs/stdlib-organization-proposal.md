# Propuesta: Organización de Biblioteca Estándar con Módulos

## Problema

Actualmente todas las funciones built-in (~100+) están en el namespace global. Esto causa:
- Namespace pollution
- Difícil descubrir funcionalidades
- Potenciales conflictos de nombres
- No escala cuando agregamos más funciones

## Solución: Prelude + Módulos Explícitos

### Prelude (Siempre Disponible)

Un conjunto pequeño de funciones fundamentales (~30) que están siempre en scope:

```javascript
// === MATEMÁTICAS BÁSICAS (15 funciones) ===
sin, cos, tan           // Trigonometría básica
sqrt, abs, exp, ln      // Operaciones comunes
pow, ceil, floor        // Redondeo
min, max               // Comparación
pi, e, i               // Constantes matemáticas

// === ARRAYS & HOF (6 funciones) ===
map, filter, reduce    // Funcionales básicas
sum, length            // Array utilities
pipe                   // Composición

// === CONTROL FLOW (2 funciones) ===
if, piecewise          // Condicionales

// === I/O BÁSICO (3 funciones) ===
print                  // Output
str, type              // Conversión e inspección

// === STRINGS (3 funciones) ===
concat, split, join    // Operaciones básicas
```

**Total: ~30 funciones** - Lo suficientemente pequeño para memorizar, lo suficientemente grande para ser útil.

### Módulos Estándar (Requieren Import)

#### `math` - Matemáticas Avanzadas

```javascript
import {
    // Trigonometría inversa
    asin, acos, atan, atan2,

    // Hiperbólicas
    sinh, cosh, tanh,
    asinh, acosh, atanh,

    // Especiales
    gamma, erf, bessel,

    // Redondeo avanzado
    round, trunc, sign,

    // Constantes
    tau, phi
} from "math"
```

#### `stats` - Estadística

```javascript
import {
    mean, median, mode,
    std, variance,
    quantile, percentile,
    covariance, correlation,
    zscore, normalize
} from "stats"
```

#### `linalg` - Álgebra Lineal

```javascript
import {
    dot, cross,
    matmul, transpose,
    det, inv, trace,
    norm, normalize,
    eigenvalues, eigenvectors,
    svd, qr, lu
} from "linalg"
```

#### `dsp` - Procesamiento de Señales

```javascript
import {
    fft, ifft,
    rfft, irfft,
    convolve, correlate,
    hanning, hamming, blackman,
    linspace, arange,
    resample, decimate
} from "dsp"
```

#### `numerical` - Análisis Numérico

```javascript
import {
    diff, diff2, diff3,
    gradient,
    integral, trapz, simpson, romberg,
    solve, bisect, newton, secant,
    derivative
} from "numerical"
```

#### `graph` - Teoría de Grafos

```javascript
import {
    bfs, dfs,
    dijkstra, bellman_ford,
    kruskal, prim,
    topological_sort,
    shortest_path,
    connected_components
} from "graph"
```

#### `optimization` - Optimización

```javascript
import {
    simplex, dual_simplex,
    linprog,
    objective_value, shadow_price,
    sensitivity_c, sensitivity_b,
    basic_variables, nonbasic_variables
} from "optimization"
```

#### `strings` - Manipulación de Strings (avanzado)

```javascript
import {
    // Búsqueda y reemplazo
    contains, starts_with, ends_with,
    index_of, replace, replace_all,

    // Transformación
    upper, lower, capitalize,
    trim, trim_left, trim_right,
    pad_left, pad_right,

    // Parsing
    parse_int, parse_float,
    to_chars, from_chars
} from "strings"
```

#### `arrays` - Utilidades de Arrays (avanzado)

```javascript
import {
    // Transformación
    sort, reverse, shuffle,
    unique, flatten,
    zip, unzip,

    // Búsqueda
    find, find_index,
    contains, count,

    // Agregación
    chunk, partition,
    group_by,

    // Set operations
    union, intersection, difference
} from "arrays"
```

## Comparación: Antes vs Después

### Script Simple - IGUAL

```javascript
// Antes (sin módulos)
let area = pi * sqrt(25)
let doubled = map(x => x * 2, [1, 2, 3])
print(doubled)

// Después (con módulos) - FUNCIONA IGUAL
let area = pi * sqrt(25)
let doubled = map(x => x * 2, [1, 2, 3])
print(doubled)
```

### Script Avanzado - MEJOR

```javascript
// Antes (sin módulos) - TODO GLOBAL
let spectrum = fft(signal)
let mean_val = mean(data)
let inversed = inv(matrix)
// ¿De dónde vienen estas funciones? 🤷

// Después (con módulos) - EXPLÍCITO
import { fft } from "dsp"
import { mean } from "stats"
import { inv } from "linalg"

let spectrum = fft(signal)
let mean_val = mean(data)
let inversed = inv(matrix)
// Claro de dónde viene cada función ✅
```

## Migración y Compatibilidad

### Fase 1: Agregar Imports (No Breaking)

```javascript
// Código viejo sigue funcionando (backward compatible)
let x = mean([1, 2, 3])  // OK: mean está global por ahora

// Código nuevo puede usar imports (recomendado)
import { mean } from "stats"
let x = mean([1, 2, 3])  // OK: import explícito
```

### Fase 2: Deprecation Warnings

```javascript
// Sin import
let x = mean([1, 2, 3])
// Warning: 'mean' used without import. Add: import { mean } from "stats"
//          Global access will be removed in version 2.0
```

### Fase 3: Remover Globales (Breaking, v2.0)

```javascript
// Sin import - ERROR
let x = mean([1, 2, 3])
// Error: 'mean' is not defined. Import it: import { mean } from "stats"

// Con import - OK
import { mean } from "stats"
let x = mean([1, 2, 3])
```

## Ventajas de Esta Propuesta

### 1. Scripts Simples Siguen Siendo Simples

```javascript
// Fibonacci sin imports
let fib = n => if(n <= 1, n, rec(n-1) + rec(n-2))
print(map(fib, [1, 2, 3, 4, 5]))
// Funciona, sin boilerplate
```

### 2. Scripts Complejos Son Más Claros

```javascript
// Análisis de señales
import { fft, ifft, hanning } from "dsp"
import { mean, std } from "stats"

let windowed = apply_window(signal, hanning(length(signal)))
let spectrum = fft(windowed)
let magnitude = map(abs, spectrum)
let avg = mean(magnitude)
let deviation = std(magnitude)
```

### 3. Escalabilidad

A medida que agregamos funciones, no infla el namespace global:

```javascript
// Futuro: machine learning module
import { neural_network, train, predict } from "ml"

// Futuro: graphics module
import { plot, scatter, histogram } from "graphics"

// Futuro: database module
import { connect, query } from "db"
```

### 4. Evita Conflictos

```javascript
// Usuario puede definir su propia 'mean' sin conflicto
let mean = data => sum(data) / length(data)  // Custom implementation

// O importar la estándar con alias
import { mean as std_mean } from "stats"
```

### 5. Descubrimiento Mejorado

```javascript
// Antes: ¿Qué funciones de stats existen?
// Respuesta: Buscar en docs 🤷

// Después: IDE puede autocompletar
import { |  } from "stats"
//         ↑ IDE sugiere: mean, median, std, variance, etc.
```

## Implementación Técnica

### Registry de Built-ins

```rust
// Estructura en evaluator
struct BuiltinRegistry {
    // Prelude: siempre en scope
    prelude: HashMap<String, BuiltinFunction>,

    // Módulos: requieren import
    modules: HashMap<String, Module>,
}

struct Module {
    name: String,
    exports: HashMap<String, BuiltinFunction>,
}

// Ejemplo
registry.prelude.insert("sin", sin_builtin);
registry.prelude.insert("map", map_builtin);

let mut math_module = Module::new("math");
math_module.exports.insert("gamma", gamma_builtin);
math_module.exports.insert("erf", erf_builtin);
registry.modules.insert("math", math_module);
```

### Resolución de Imports

```rust
fn resolve_import(module: &str, name: &str) -> Result<BuiltinFunction> {
    // 1. Buscar en módulos built-in
    if let Some(module) = registry.modules.get(module) {
        if let Some(func) = module.exports.get(name) {
            return Ok(func.clone());
        }
        return Err(format!("'{}' not found in module '{}'", name, module));
    }

    // 2. Buscar en filesystem (módulos de usuario)
    load_user_module(module, name)
}
```

## Prelude: Decisión de Diseño

### Criterios para Incluir en Prelude

Una función va en prelude si cumple **2 de 3**:

1. **Frecuencia**: Usada en >50% de scripts típicos
2. **Fundamental**: Parte del core del lenguaje (como `if`, `map`)
3. **Educacional**: Común en ejemplos introductorios

### Ejemplos de Decisiones

| Función | Prelude? | Razón |
|---------|----------|-------|
| `sin` | ✅ Sí | Frecuente, educacional |
| `map` | ✅ Sí | Fundamental HOF |
| `pi` | ✅ Sí | Frecuente, educacional |
| `gamma` | ❌ No | Avanzada, poco frecuente |
| `fft` | ❌ No | Especializada |
| `mean` | ❌ No | Stats específica |

## REPL Considerations

### Prelude en REPL

```javascript
ach[1]> sin(pi)
0.0
ach[2]> map(x => x^2, [1, 2, 3])
[1, 4, 9]
// Funciona sin imports ✅
```

### Imports en REPL

```javascript
ach[1]> import { mean } from "stats"
ach[2]> mean([1, 2, 3])
2.0
ach[3]> mean([4, 5, 6])
5.0
// Import persiste en la sesión ✅
```

### Auto-suggest en REPL

```javascript
ach[1]> f|
// Suggestions:
//   - filter (prelude)
//   - fft (import from "dsp")
//   - flatten (import from "arrays")
```

## Resumen

### Propuesta Final

1. **Prelude pequeño** (~30 funciones) siempre disponible
2. **Módulos estándar** organizados por dominio
3. **Imports explícitos** para funciones avanzadas
4. **Backward compatible** en fase de transición
5. **Escalable** para futuras funcionalidades

### Beneficios Clave

- ✅ Namespace limpio
- ✅ Código auto-documentado (imports muestran dependencias)
- ✅ REPL sigue siendo conveniente (prelude)
- ✅ Escalabilidad para crecimiento
- ✅ Evita conflictos de nombres
- ✅ Mejor descubrimiento de funcionalidades

### Timeline

- **Fase 1** (Semanas 1-3): Implementar sistema de módulos
- **Fase 2** (Semana 4): Organizar built-ins en módulos
- **Fase 3** (Semana 5): Deprecation warnings
- **Fase 4** (v2.0): Remover acceso global (breaking change)

---

**Pregunta para ti**: ¿Esta organización te parece razonable? ¿Cambiarías algo del prelude?
