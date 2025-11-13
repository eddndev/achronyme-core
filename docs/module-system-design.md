# Sistema de Módulos para Achronyme - Diseño Completo

## Resumen Ejecutivo

Este documento describe el diseño completo del sistema de módulos para Achronyme, incluyendo imports, exports, namespacing, resolución de rutas, y consideraciones de implementación.

## Objetivos

1. **Organización de código**: Permitir dividir programas en múltiples archivos
2. **Reutilización**: Compartir código entre proyectos
3. **Encapsulación**: Control sobre qué se expone públicamente
4. **Claridad**: Dependencias explícitas y trazables
5. **Compatibilidad**: Integración con sistema existente (REPL, WASM, CLI)

## Filosofía de Diseño

- **Simple y explícito**: Preferir claridad sobre brevedad
- **Estáticamente resoluble**: Imports conocidos en tiempo de parsing (no dinámicos)
- **Compatible con REPL**: Permitir imports interactivos
- **Sin side effects en imports**: Los módulos son valores, no ejecutan código automáticamente

---

## 1. Sintaxis Básica

### 1.1 Export

#### Export de Declaraciones

```javascript
// math.soc
export let pi = 3.14159
export let e = 2.71828

export let square = x => x^2
export let cube = x => x^3

// Multiple exports en una línea
export let add = (a, b) => a + b, multiply = (a, b) => a * b
```

#### Export de Records

```javascript
// geometry.soc
export let Point = {
    new: (x, y) => {x: x, y: y},
    distance: (p1, p2) => sqrt((p2.x - p1.x)^2 + (p2.y - p1.y)^2)
}

export let Circle = {
    new: (center, radius) => {center: center, radius: radius},
    area: (circle) => pi * circle.radius^2
}
```

#### Export Default

```javascript
// counter.soc
export default {
    mut value: 0,
    increment: () => do { self.value = self.value + 1 },
    get: () => self.value
}
```

#### Re-export

```javascript
// index.soc - barrel exports
export { Point, Circle } from "./geometry.soc"
export { square, cube } from "./math.soc"
export { default as Counter } from "./counter.soc"
```

### 1.2 Import

#### Import Named

```javascript
// app.soc
import { pi, square } from "./math.soc"

let area = pi * square(5)
```

#### Import All as Namespace

```javascript
import * as math from "./math.soc"

let result = math.square(math.pi)
```

#### Import Default

```javascript
import Counter from "./counter.soc"

let c = Counter
c.increment()
```

#### Import with Alias

```javascript
import { square as sq, cube as cb } from "./math.soc"

let x = sq(5)  // 25
let y = cb(3)  // 27
```

#### Mixed Imports

```javascript
import Counter, { pi, square } from "./utils.soc"
```

---

## 2. Resolución de Rutas

### 2.1 Tipos de Rutas

#### Rutas Relativas (recomendado)

```javascript
// Mismo directorio
import { helper } from "./helper.soc"

// Directorio padre
import { config } from "../config.soc"

// Anidado
import { Button } from "./components/ui/button.soc"
```

#### Rutas Absolutas (desde project root)

```javascript
// Desde raíz del proyecto
import { math } from "/lib/math.soc"
import { ui } from "/components/ui.soc"
```

#### Imports de Biblioteca Estándar

```javascript
// Built-in modules (sin path)
import { sin, cos, tan } from "math"
import { fft, ifft } from "dsp"
import { sum, mean, std } from "stats"
```

### 2.2 Extensión de Archivo

- **Requerida**: Siempre `.soc` para claridad
- **Sin omisión**: No `import from "./math"` → debe ser `"./math.soc"`

### 2.3 Index Files

```javascript
// components/index.soc
export { Button } from "./button.soc"
export { Input } from "./input.soc"
export { Label } from "./label.soc"

// app.soc
import { Button, Input } from "./components/index.soc"
// O shorthand:
import { Button, Input } from "./components"  // busca index.soc automáticamente
```

### 2.4 Resolución de Conflictos

```javascript
// Error: nombre duplicado
import { x } from "./a.soc"
import { x } from "./b.soc"  // ERROR: 'x' already imported

// Solución: alias
import { x as x1 } from "./a.soc"
import { x as x2 } from "./b.soc"
```

---

## 3. Namespacing

### 3.1 Module Object

Cuando usas `import * as name`, obtienes un record con todas las exports:

```javascript
// math.soc
export let pi = 3.14159
export let square = x => x^2

// app.soc
import * as math from "./math.soc"

// math es un record: {pi: 3.14159, square: <function>}
math.pi      // 3.14159
math.square(5)  // 25
```

### 3.2 Nested Namespaces

```javascript
// lib/geometry/shapes.soc
export let Circle = {...}

// lib/geometry/index.soc
export * as shapes from "./shapes.soc"

// app.soc
import * as geometry from "./lib/geometry/index.soc"
geometry.shapes.Circle  // Acceso anidado
```

### 3.3 Flat vs Nested Imports

```javascript
// Opción 1: Flat (recomendado para uso frecuente)
import { Circle, Point, Line } from "./geometry.soc"

// Opción 2: Namespace (recomendado para librerías grandes)
import * as geom from "./geometry.soc"
let c = geom.Circle.new(0, 0, 10)
```

---

## 4. Scope y Visibilidad

### 4.1 Module Scope

Cada módulo tiene su propio scope:

```javascript
// utils.soc
let privateHelper = x => x * 2  // No exportado = privado
export let publicHelper = x => privateHelper(x) + 1

// app.soc
import { publicHelper } from "./utils.soc"
publicHelper(5)  // OK: 11
// privateHelper(5)  // ERROR: privateHelper no está disponible
```

### 4.2 Export vs No Export

```javascript
// config.soc
let SECRET_KEY = "abc123"  // Privado
export let API_URL = "https://api.example.com"  // Público

let validateKey = key => key == SECRET_KEY  // Privado
export let checkAccess = key => validateKey(key)  // Público
```

### 4.3 Shadowing en Imports

```javascript
let x = 10  // Variable local

import { x } from "./values.soc"  // ERROR: 'x' ya existe en scope

// Solución: alias
import { x as importedX } from "./values.soc"
```

---

## 5. Circular Dependencies

### 5.1 Detección

El sistema debe detectar y rechazar ciclos:

```javascript
// a.soc
import { b } from "./b.soc"
export let a = () => b()

// b.soc
import { a } from "./a.soc"  // ERROR: Circular dependency detected
export let b = () => a()     // a.soc → b.soc → a.soc
```

### 5.2 Soluciones

#### Opción 1: Refactor para extraer dependencia común

```javascript
// shared.soc
export let helper = x => x * 2

// a.soc
import { helper } from "./shared.soc"
export let a = x => helper(x)

// b.soc
import { helper } from "./shared.soc"
export let b = x => helper(x) + 1
```

#### Opción 2: Lazy evaluation (futuro)

```javascript
// a.soc
export let a = () => require("./b.soc").b()  // Dynamic import (fase 2)
```

---

## 6. Module Resolution Algorithm

### 6.1 Pasos de Resolución

```
1. Parsear import statement
2. Determinar tipo de ruta (relativa/absoluta/built-in)
3. Resolver ruta a archivo físico
4. Verificar si módulo ya está cargado (cache)
5. Si no está cargado:
   a. Leer archivo
   b. Parsear módulo
   c. Resolver imports del módulo (recursivo)
   d. Detectar ciclos
   e. Evaluar módulo
   f. Guardar exports en cache
6. Extraer exports solicitados
7. Agregar a scope actual
```

### 6.2 Module Cache

```rust
// Estructura conceptual
struct ModuleCache {
    // Path absoluto → exports del módulo
    modules: HashMap<PathBuf, Record>,

    // Para detectar ciclos durante resolución
    loading: HashSet<PathBuf>,
}
```

### 6.3 Ejemplo de Resolución

```javascript
// Archivo: /project/src/app.soc
import { helper } from "./utils.soc"

// Resolución:
// 1. Path relativo "./utils.soc"
// 2. Directorio base: /project/src/
// 3. Resolver a: /project/src/utils.soc
// 4. Verificar cache...
// 5. No en cache, cargar archivo
// 6. Parsear y evaluar utils.soc
// 7. Extraer export 'helper'
// 8. Agregar a scope de app.soc
```

---

## 7. Built-in Modules

### 7.1 Standard Library

Módulos que vienen con el lenguaje:

```javascript
// Matemáticas básicas
import { sin, cos, sqrt, abs } from "math"

// Álgebra lineal
import { dot, cross, det, inv } from "linalg"

// Estadística
import { sum, mean, std, median } from "stats"

// DSP
import { fft, ifft, convolve } from "dsp"

// Análisis numérico
import { diff, integral, solve } from "numerical"

// Strings
import { split, join, trim, upper, lower } from "strings"

// Arrays
import { map, filter, reduce, sort } from "arrays"
```

### 7.2 Resolución de Built-ins

```
1. Si path no empieza con '.', '/', ni '../' → es built-in
2. Buscar en registro de módulos estándar
3. Si no existe → error "Unknown module"
```

---

## 8. Integración con Sistemas Existentes

### 8.1 REPL

#### Imports Interactivos

```javascript
ach[1]> import { pi } from "math"
ach[2]> pi
3.14159
ach[3]> import { square } from "./utils.soc"
ach[4]> square(pi)
9.8696...
```

#### State Management

- Los imports en REPL son persistentes en la sesión
- Cada import agrega al scope global del REPL
- No se permite re-import del mismo módulo (error o warning)

### 8.2 WASM

#### Filesystem Virtual

Para WASM, necesitamos un sistema de archivos virtual:

```rust
// JavaScript API
const fs = {
    "/lib/math.soc": "export let pi = 3.14159...",
    "/app.soc": "import { pi } from '/lib/math.soc'..."
};

const result = achronyme.eval(fs, "/app.soc");
```

#### Bundle Support

```javascript
// Opción: bundle todos los módulos en uno
// build-tool empaqueta todo en un solo archivo WASM
```

### 8.3 CLI

#### File Execution

```bash
achronyme run src/main.soc
```

Automáticamente resuelve imports relativos desde el directorio del archivo.

#### Project Structure

```
myproject/
├── src/
│   ├── main.soc          # Entry point
│   ├── utils.soc
│   └── components/
│       ├── index.soc
│       ├── button.soc
│       └── input.soc
├── lib/
│   └── helpers.soc
└── achronyme.toml        # Config (futuro)
```

---

## 9. Syntax Summary

### 9.1 Export Syntax

```javascript
// Declaración directa
export let x = 10
export let f = () => 42

// Default export
export default {...}

// Re-export
export { x, y } from "./other.soc"
export * from "./utils.soc"
export * as utils from "./utils.soc"
```

### 9.2 Import Syntax

```javascript
// Named imports
import { x, y } from "./module.soc"

// Alias
import { x as a, y as b } from "./module.soc"

// Namespace
import * as mod from "./module.soc"

// Default
import Thing from "./module.soc"

// Mixed
import Default, { x, y } from "./module.soc"
```

---

## 10. Error Handling

### 10.1 Tipos de Errores

#### Module Not Found

```javascript
import { x } from "./nonexistent.soc"
// Error: Module not found: ./nonexistent.soc
//        Searched in: /project/src/nonexistent.soc
```

#### Export Not Found

```javascript
// math.soc exports: pi, e
import { tau } from "./math.soc"
// Error: 'tau' is not exported by ./math.soc
//        Available exports: pi, e
```

#### Circular Dependency

```javascript
// Error: Circular dependency detected:
//        a.soc → b.soc → c.soc → a.soc
```

#### Duplicate Import

```javascript
import { x } from "./a.soc"
import { x } from "./b.soc"
// Error: Identifier 'x' already imported from ./a.soc
//        Use alias: import { x as x2 } from "./b.soc"
```

### 10.2 Warnings (Non-blocking)

```javascript
import { x, y, z } from "./utils.soc"
// Si solo usas 'x':
// Warning: Unused imports: y, z
```

---

## 11. Implementation Plan

### Phase 1: Core Module System (2-3 weeks)

#### Week 1: Parser & AST
- [ ] Agregar nodos AST: `ImportDecl`, `ExportDecl`
- [ ] Extender grammar con import/export syntax
- [ ] Tests de parsing

#### Week 2: Module Resolver
- [ ] `ModuleLoader` struct
- [ ] Path resolution algorithm
- [ ] Module cache
- [ ] Circular dependency detection

#### Week 3: Evaluator Integration
- [ ] Evaluate modules
- [ ] Handle imports in environment
- [ ] Export collection
- [ ] Integration tests

### Phase 2: Advanced Features (1-2 weeks)

- [ ] Re-exports
- [ ] Default exports
- [ ] Built-in module registry
- [ ] REPL integration
- [ ] WASM virtual filesystem

### Phase 3: Tooling (1 week)

- [ ] CLI improvements (`run`, `check`)
- [ ] Error messages
- [ ] Documentation
- [ ] Examples

---

## 12. Examples

### 12.1 Simple Library

```javascript
// lib/math.soc
export let pi = 3.14159265359
export let e = 2.71828182846

export let square = x => x^2
export let cube = x => x^3
export let sqrt = x => x^0.5

export let Circle = {
    area: r => pi * square(r),
    circumference: r => 2 * pi * r
}
```

```javascript
// app.soc
import { pi, Circle } from "./lib/math.soc"

let radius = 5
let area = Circle.area(radius)
let circ = Circle.circumference(radius)
```

### 12.2 Component Library

```javascript
// components/button.soc
export let Button = {
    new: (text, onClick) => {
        text: text,
        mut clicks: 0,
        onClick: () => do {
            self.clicks = self.clicks + 1
            onClick(self.clicks)
        }
    }
}
```

```javascript
// components/input.soc
export let Input = {
    new: (placeholder) => {
        placeholder: placeholder,
        mut value: "",
        onChange: (text) => do {
            self.value = text
        }
    }
}
```

```javascript
// components/index.soc
export { Button } from "./button.soc"
export { Input } from "./input.soc"
```

```javascript
// app.soc
import { Button, Input } from "./components/index.soc"

let myButton = Button.new("Click me", clicks => {
    print("Clicked " + str(clicks) + " times")
})

let myInput = Input.new("Enter name...")
```

### 12.3 Nested Modules

```javascript
// lib/geometry/point.soc
export let Point = {
    new: (x, y) => {x: x, y: y},
    distance: (p1, p2) => sqrt((p2.x-p1.x)^2 + (p2.y-p1.y)^2)
}
```

```javascript
// lib/geometry/index.soc
export { Point } from "./point.soc"
export { Circle } from "./circle.soc"
export { Line } from "./line.soc"
```

```javascript
// app.soc
import * as geom from "./lib/geometry/index.soc"

let p1 = geom.Point.new(0, 0)
let p2 = geom.Point.new(3, 4)
let dist = geom.Point.distance(p1, p2)  // 5
```

---

## 13. Comparison with Other Languages

### JavaScript/TypeScript

```javascript
// JavaScript
export const x = 10
import { x } from './module'

// Achronyme - similar!
export let x = 10
import { x } from "./module.soc"
```

### Python

```python
# Python
from math import pi, sqrt
import numpy as np

# Achronyme
import { pi, sqrt } from "math"
import * as numpy from "numpy"
```

### Rust

```rust
// Rust
pub fn helper() {}
use crate::helper;

// Achronyme
export let helper = () => {}
import { helper } from "./module.soc"
```

---

## 14. Open Questions

### Q1: Dynamic Imports?

```javascript
// Futuro: import dinámico?
let moduleName = if(condition, "./a.soc", "./b.soc")
import { x } from moduleName  // ¿Permitir?
```

**Decisión**: No en Phase 1. Imports deben ser estáticos.

### Q2: Conditional Exports?

```javascript
// ¿Exports condicionales?
if(DEBUG) {
    export let debugHelper = () => {...}
}
```

**Decisión**: No. Todos los exports deben ser top-level.

### Q3: Side Effects?

```javascript
// module.soc
print("Module loaded!")  // ¿Se ejecuta al importar?
export let x = 10
```

**Decisión**: Sí, se permite. Código top-level se ejecuta una vez al cargar el módulo.

### Q4: Hoisting?

```javascript
// ¿Esto funciona?
import { x } from "./b.soc"
let y = x + 10

export let z = 20  // ¿Disponible antes de esta línea?
```

**Decisión**: Imports se procesan primero, exports después. Dentro de un módulo, orden normal (no hoisting).

---

## 15. Migration Path

### Existing Code

Código sin módulos sigue funcionando:

```javascript
// old-style.soc
let x = 10
let helper = () => 42
// Funciona igual que antes
```

### Gradual Adoption

```javascript
// Puedes mezclar old-style con imports
let localVar = 10

import { helper } from "./utils.soc"

let result = helper(localVar)
```

---

## 16. Summary

### Key Decisions

1. **Sintaxis**: Similar a ES6 modules (familiar)
2. **Resolución**: Estática, en tiempo de parsing
3. **Extensiones**: `.soc` siempre requerida
4. **Ciclos**: Detectados y rechazados
5. **Scope**: Cada módulo tiene su propio scope
6. **Cache**: Módulos se cargan una sola vez
7. **Built-ins**: Módulos estándar sin path prefix

### Implementation Priority

1. 🔴 **Critical**: Basic import/export
2. 🟡 **High**: Module resolution, cache, cycle detection
3. 🟢 **Medium**: Re-exports, default exports
4. 🔵 **Low**: Advanced features, tooling

### Timeline Estimate

- **Core system**: 3-4 weeks
- **Advanced features**: 1-2 weeks
- **Tooling & polish**: 1 week
- **Total**: 5-7 weeks

---

**Next Steps**: Review this design, gather feedback, then start implementation with Phase 1.
