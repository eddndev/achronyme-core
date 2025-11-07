# Roadmap de Achronyme

**Visión**: Crear un ecosistema de cálculo matemático open-source de clase mundial que pueda competir con sistemas propietarios como Wolfram Mathematica, con enfoque en rendimiento, accesibilidad y extensibilidad.

---

## ✅ Completado (v0.1 - v0.3)

- [x] **Phase 1**: Parser y evaluador de expresiones matemáticas
- [x] **Phase 2**: Operaciones aritméticas básicas (+, -, *, /, ^, %)
- [x] **Phase 3A**: Funciones matemáticas estándar (trigonométricas, exponenciales, logaritmos)
- [x] **Phase 3B**: Tipos complejos (Complex, Vector, Matrix)
- [x] **Phase 4A**: Variables, lambdas, closures, higher-order functions
- [x] **Phase 4B**: DSP básico (DFT, FFT Cooley-Tukey, IFFT, convolución, ventanas)
- [x] **Phase 4C**: Funciones estadísticas nativas (sum, mean, std, max, min)
- [x] **Phase 4D**: SDK TypeScript tipo-seguro con gestión de memoria

---

## ✅ Completado Recientemente (v0.4 - v0.7)

### Álgebra Lineal Avanzada (v0.4)
- [x] **Descomposición de matrices**:
  - [x] LU decomposition (factorización PA = LU)
  - [x] QR decomposition (Gram-Schmidt, Householder)
  - [x] Cholesky decomposition (matrices positivas definidas)
  - [x] SVD - Singular Value Decomposition
- [x] **Eigenvalues y eigenvectors**:
  - [x] Método de potencias (power iteration)
  - [x] Algoritmo QR para eigenvalues
  - [x] Eigenvalues de matrices simétricas

### Cálculo Numérico (v0.5)
- [x] **Derivación numérica**:
  - [x] Diferencias finitas (central)
  - [x] Derivadas de orden superior
- [x] **Integración numérica**:
  - [x] Regla del trapecio
  - [x] Regla de Simpson
  - [x] Cuadratura de Gauss (adaptativa)
  - [x] Integración de Romberg
- [x] **Solución de ecuaciones**:
  - [x] Métodos de bisección y secante
  - [x] Newton-Raphson (una variable)

### Optimización (v0.6)
- [x] **Programación Lineal**:
  - [x] Simplex method
  - [x] Dual Simplex, Two-Phase Simplex, Revised Simplex

### Sistema de Scopes y Refactorización (v0.7)
- [x] **Refactorización del evaluador**:
  - [x] Separación de lógica en handlers especializados
  - [x] Reducción de evaluator.rs (1179 → 203 líneas, 83%)
  - [x] Tests movidos a archivos de integración
- [x] **Variable Shadowing**:
  - [x] Sistema de scopes stack-based
  - [x] Shadowing en parámetros de lambda
  - [x] Redeclaración con `let` en mismo scope
  - [x] Closures preservados correctamente
  - [x] Pipelines de transformación sin duplicar memoria

---

## 🚧 En Desarrollo (v0.7+)

### Álgebra Lineal Avanzada
- [ ] **Operaciones avanzadas**:
  - [ ] Rank, null space, column space
  - [ ] Pseudoinversa (Moore-Penrose)
  - [ ] Normas matriciales (Frobenius, 1-norm, ∞-norm)
  - [ ] Condición de matriz (condition number)

### Cálculo Numérico
- [ ] **Derivación numérica**:
  - [ ] Derivadas parciales
  - [ ] Gradiente, divergencia, curl
- [ ] **Integración numérica**:
  - [ ] Integrales dobles y triples
- [ ] **Solución de ecuaciones**:
  - [ ] Newton-Raphson multidimensional
  - [ ] Métodos de punto fijo
- [ ] **Sistemas de ecuaciones lineales**:
  - [ ] Eliminación gaussiana con pivoteo
  - [ ] Métodos iterativos (Jacobi, Gauss-Seidel)
  - [ ] Gradiente conjugado
  - [ ] Sparse matrices (matrices dispersas)

### Optimización
- [ ] **Optimización sin restricciones**:
  - [ ] Gradiente descendente (vanilla, momentum, AdaGrad, Adam)
  - [ ] Método de Newton
  - [ ] Quasi-Newton (BFGS, L-BFGS)
  - [ ] Nelder-Mead (simplex)
  - [ ] Simulated annealing
- [ ] **Optimización con restricciones**:
  - [ ] Interior point methods
  - [ ] Lagrange multipliers
  - [ ] Sequential Quadratic Programming (SQP)
- [ ] **Optimización combinatoria**:
  - [ ] Genetic algorithms
  - [ ] Particle swarm optimization
  - [ ] Ant colony optimization

---

## 🔮 Futuro (v0.8+)

### Phase 8: Estadística y Probabilidad
- [ ] Distribuciones (normal, binomial, Poisson, t, chi-cuadrado, F)
- [ ] Tests estadísticos (t-test, ANOVA, chi-cuadrado)
- [ ] Regresión (lineal, múltiple, polinomial, logística)
- [ ] Correlación y covarianza
- [ ] Análisis de series temporales (ACF, PACF, ARIMA)
- [ ] Bootstrap y métodos de Monte Carlo

### Phase 9: EDOs y EDPs
**Ecuaciones diferenciales ordinarias:**
- [ ] Euler, Runge-Kutta (RK4, RK45)
- [ ] Métodos multipaso (Adams-Bashforth)
- [ ] Stiff solvers (BDF)

**Ecuaciones diferenciales parciales:**
- [ ] Método de diferencias finitas
- [ ] Elementos finitos (FEM)
- [ ] Método espectral

### Phase 10: Cálculo Simbólico
**Álgebra simbólica:**
- [ ] Simplificación de expresiones
- [ ] Expansión y factorización
- [ ] Sustitución y evaluación simbólica

**Cálculo simbólico:**
- [ ] Derivación simbólica (regla de cadena, producto, cociente)
- [ ] Integración simbólica (tablas, sustitución)
- [ ] Límites y series de Taylor

**Ecuaciones simbólicas:**
- [ ] Solución de ecuaciones algebraicas
- [ ] Sistemas de ecuaciones simbólicas

### Phase 11: Procesamiento Avanzado de Señales
**Análisis tiempo-frecuencia:**
- [ ] Short-Time Fourier Transform (STFT)
- [ ] Wavelets (Haar, Daubechies, Morlet)
- [ ] Spectrogram y Mel-spectrogram

**Filtros digitales:**
- [ ] IIR filters (Butterworth, Chebyshev, Elliptic)
- [ ] FIR filter design (windowing, Parks-McClellan)
- [ ] Adaptive filters (LMS, RLS)

**Procesamiento de imágenes:**
- [ ] 2D FFT
- [ ] Convolución 2D
- [ ] Filtros (Gaussian blur, Sobel, Laplacian)

### Phase 12: Machine Learning Básico
- [ ] Redes neuronales (feedforward, backpropagation)
- [ ] k-means clustering
- [ ] PCA (Principal Component Analysis)
- [ ] k-NN, Decision Trees
- [ ] Support Vector Machines

---

## 🌟 Visión de Ecosistema

### @achronyme/core (actual)
Núcleo de cálculo matemático con WebAssembly

### @achronyme/language (futuro)
Procesamiento de lenguaje natural matemático

```javascript
import { AchronymeNLP } from '@achronyme/language';

const nlp = new AchronymeNLP();

nlp.parse("solve x squared plus 5x minus 6 equals 0")
// → "solve(x^2 + 5*x - 6 = 0)"

nlp.parse("integrate x squared from 0 to 10")
// → "integrate(x^2, x, 0, 10)"
```

### @achronyme/plot (futuro)
Visualización matemática

```javascript
import { AchronymePlot } from '@achronyme/plot';

plot.func('x^2', {range: [-10, 10]});
plot.scatter(data);
plot.surface('x^2 + y^2', {x: [-5,5], y: [-5,5]});
```

### @achronyme/cas (futuro)
Sistema de álgebra computacional completo

```javascript
import { AchronymeCAS } from '@achronyme/cas';

cas.simplify('(x+1)^2');           // → x^2 + 2*x + 1
cas.expand('(a+b)*(c+d)');         // → a*c + a*d + b*c + b*d
cas.solve('x^2 + 5*x - 6 = 0');    // → [x = 1, x = -6]
```

---

## 🤝 Cómo Contribuir

Achronyme es open-source y buscamos colaboradores en:
- **C++ developers**: Implementar algoritmos numéricos core
- **TypeScript developers**: SDK, testing, ejemplos
- **Math experts**: Validación de algoritmos, precisión numérica
- **DSP engineers**: Optimización de FFT, nuevos filtros
- **Documentation**: Tutoriales, ejemplos, traducciones
- **Testing**: Benchmarks, validación contra NumPy/MATLAB/Wolfram

**Repositorio**: https://github.com/eddndev/achronyme-core
**Discusiones**: https://github.com/eddndev/achronyme-core/discussions

---

## 🎯 Objetivo Realista

Convertirse en la alternativa open-source líder para cálculo numérico, DSP y álgebra lineal en los próximos 2-3 años. Competir con Wolfram en cálculo simbólico es un objetivo a largo plazo (5-10 años) que requiere una comunidad activa.

---

## 📚 Referencias

- [Comparación con Wolfram](./wolfram-comparison.md)
- [Especificación del Lenguaje](./language-spec.md)
- [Guía del SDK](./sdk-guide.md)
- [README Principal](../README.md)

---

**Versión**: 0.3.0
**Última actualización**: 2025

---

## Propuesta de Sintaxis para Grafos (Futuro)

*Esta es una propuesta para una futura implementación de una sintaxis de grafos en el lenguaje SOC, diseñada para ser extensible y soportar múltiples algoritmos (PERT, Dijkstra, etc.).*

### Principio de Diseño

La sintaxis debe separar la **topología** del grafo (su estructura de nodos y aristas) de los **datos** asociados a un problema específico (pesos, tiempos, costos), permitiendo máxima flexibilidad.

### Sintaxis General Propuesta

Se introduce un nuevo literal `network` y un operador de arista `->`.

```soc
let mi_red = network {
    // Opcional: Definición de nodos y sus propiedades
    nodes: {
        "ID_Nodo_1": { prop1: valor1, ... },
        "ID_Nodo_2": { prop2: valor2, ... }
    },

    // Opcional: Lista de aristas y sus propiedades
    edges: [
        "ID_Nodo_1" -> "ID_Nodo_2" { prop_arista: valor, ... },
        ...
    ]
}
```

### Ejemplos de Casos de Uso

#### 1. Grafo Simple (Topología Pura)
Para algoritmos de conectividad, recorridos (BFS, DFS), etc.

```soc
let grafo_simple = network {
    // Los nodos se pueden inferir de las aristas
    edges: [
        "A" -> "B",
        "B" -> "C",
        "A" -> "C"
    ]
}
```

#### 2. Grafo con Pesos en Aristas (Para Dijkstra, Kruskal)
Se añaden propiedades a las aristas.

```soc
let mapa_distancias = network {
    edges: [
        "Madrid"   -> "Zaragoza"  { distancia: 325 },
        "Zaragoza" -> "Barcelona" { distancia: 290 },
        "Madrid"   -> "Valencia"  { distancia: 360 }
    ]
}

// Uso: dijkstra(mapa_distancias, "Madrid", "Barcelona", { weight: "distancia" })
```

#### 3. Grafo con Propiedades en Nodos (Para PERT)
Se añaden propiedades a los nodos.

```soc
let proyecto_pert = network {
    nodes: {
        "Diseño":     { to: 3, tm: 5, tp: 10 },
        "Backend":    { to: 7, tm: 10, tp: 15 },
        "Frontend":   { to: 6, tm: 8,  tp: 12 }
    },
    edges: [
        "Diseño" -> "Backend",
        "Diseño" -> "Frontend"
    ]
}

// Uso: find_critical_path(proyecto_pert)
```

### Beneficios de la Propuesta

- **Consistente:** Sigue el estilo declarativo del lenguaje SOC.
- **Flexible:** Permite definir topología pura, datos en nodos, datos en aristas, o una combinación.
- **Extensible:** Los nuevos algoritmos pueden simplemente buscar las propiedades que necesitan en los `records` de los nodos o aristas, sin requerir cambios en la sintaxis.
- **Legible:** La estructura del grafo y sus datos son fáciles de entender de un vistazo.

---

## Propuesta de Sintaxis para Condicionales y Funciones Piecewise (Futuro)

*Esta es una propuesta para implementar funciones definidas por partes (piecewise functions) y condicionales en el lenguaje SOC, fundamentales para optimización, física, DSP y cálculo numérico.*

### Motivación

Las **funciones definidas por partes** son esenciales en matemáticas aplicadas:
- **Optimización:** Costos escalonados, tarifas progresivas
- **Física/Ingeniería:** Condiciones de frontera, cargas distribuidas
- **DSP:** Ventanas rectangulares, funciones indicadoras
- **Machine Learning:** Funciones de activación (ReLU, Leaky ReLU)
- **Economía:** Impuestos progresivos, descuentos por volumen
- **Cálculo Numérico:** Integración/derivación de funciones discontinuas

### Principios de Diseño

1. **Consistente:** Mantener el estilo funcional y declarativo de SOC
2. **Multivariable:** Las condiciones deben soportar múltiples variables (igual que las lambdas)
3. **Simple para casos simples:** `if()` para 2 ramas
4. **Expresivo para casos complejos:** `piecewise()` para 3+ ramas
5. **Default implícito:** Evitar `[true, valor]` explícito cuando sea posible

### Sintaxis Propuesta: `if()` - Condicional Simple

Para casos con 2 ramas (verdadero/falso):

```soc
if(condicion, valor_si_verdadero, valor_si_falso)
```

**Ejemplos:**

```soc
// Valor absoluto manual
let abs_manual = x => if(x < 0, -x, x)

// ReLU (función de activación)
let relu = x => if(x > 0, x, 0)

// Leaky ReLU
let leaky_relu = x => if(x > 0, x, 0.01*x)

// Función por partes simple
let f = x => if(x < 0, x^2, sqrt(x))

// Máximo personalizado
let max_custom = (a, b) => if(a > b, a, b)

// Multivariable: dentro de una región circular
let dentro_circulo = (x, y) => if(x^2 + y^2 < 1, 1, 0)
```

### Sintaxis Propuesta: `piecewise()` - Funciones por Partes

Para casos con 3+ ramas o dominios complejos:

```soc
piecewise(
  [condicion1, valor1],
  [condicion2, valor2],
  [condicion3, valor3],
  valor_default  // último argumento sin [] = caso por defecto
)
```

**Características:**
- **Evaluación en orden:** Las condiciones se evalúan secuencialmente (short-circuit)
- **Default implícito:** El último argumento sin `[]` es el valor por defecto
- **Default opcional:** Si no hay default y ninguna condición se cumple, genera error
- **Multivariable:** Las condiciones pueden usar todas las variables de la lambda

**Ejemplos:**

```soc
// 1. Función signo
let signo = x => piecewise(
  [x < 0, -1],
  [x > 0, 1],
  0  // cuando x == 0
)

// 2. Tarifa eléctrica escalonada (caso real de optimización)
let tarifa = kwh => piecewise(
  [kwh <= 100, 0.10 * kwh],
  [kwh <= 300, 10 + 0.08 * (kwh - 100)],
  [kwh <= 500, 26 + 0.06 * (kwh - 300)],
  38 + 0.05 * (kwh - 500)  // más de 500 kWh
)

// 3. Función matemática compleja por dominios
let f = x => piecewise(
  [x < -1, 0],
  [x >= -1 && x < 0, x^2 + 2*x + 1],
  [x >= 0 && x < 1, sin(PI*x)],
  exp(-x)  // x >= 1
)

// 4. Costo de producción con economías de escala
let costo_produccion = unidades => piecewise(
  [unidades <= 1000, 50*unidades],
  [unidades <= 5000, 50000 + 45*(unidades-1000)],
  230000 + 40*(unidades-5000)
)

// 5. Multivariable: Regiones en el plano
let region = (x, y) => piecewise(
  [x^2 + y^2 < 1, 1],            // círculo interior
  [abs(x) < 2 && abs(y) < 2, 2], // cuadrado exterior
  0                               // fuera del cuadrado
)

// 6. Física: Fuerza con fricción
let fuerza = v => piecewise(
  [v == 0, 0],
  [v > 0, -0.5*v^2],  // fricción en dirección positiva
  0.5*v^2             // fricción en dirección negativa
)

// 7. DSP: Ventana rectangular personalizada
let ventana = t => piecewise(
  [t >= 0 && t <= 1, 1],
  0  // fuera del intervalo
)

// 8. Sin default (error si condición no se cumple)
let f_parcial = x => piecewise(
  [x >= 0 && x < 1, x^2],
  [x >= 1 && x < 2, 2*x - 1]
  // Error si x < 0 o x >= 2
)
```

### Operadores Lógicos Requeridos

Para condiciones complejas, se necesitan operadores lógicos:

```soc
// AND lógico
x >= 0 && x <= 1

// OR lógico
x < -1 || x > 1

// NOT lógico
!(x == 0)

// Combinaciones
(x > 0 && y > 0) || (x < 0 && y < 0)
```

### Casos de Uso Desbloqueados

#### 1. Integración Numérica con Discontinuidades

```soc
// Función con discontinuidad
let f = x => piecewise(
  [x < 0, 0],
  [x >= 0 && x <= PI, sin(x)],
  0
)

// Integrar correctamente
let area = simpson(f, -1, 4, 100)
```

#### 2. Optimización con Costos por Tramos

```soc
// Minimizar costo total con función piecewise
let costo_total = x => piecewise(
  [x <= 100, 10*x],
  [x <= 500, 1000 + 8*(x-100)],
  4200 + 5*(x-500)
)

// Usar en optimización (futuro)
let optimo = minimize(costo_total, 0, 1000)
```

#### 3. Impuestos Progresivos (Economía)

```soc
let impuesto = ingreso => piecewise(
  [ingreso <= 10000, 0],
  [ingreso <= 50000, 0.10 * (ingreso - 10000)],
  [ingreso <= 100000, 4000 + 0.20 * (ingreso - 50000)],
  14000 + 0.30 * (ingreso - 100000)
)
```

#### 4. Derivación Numérica de Funciones Discontinuas

```soc
// Función con discontinuidad
let g = x => if(x < 0, x^2, x^3)

// Derivar numéricamente (cuidado en x=0)
let dg = x => diff(g, x, 1e-5)
```

### Implementación Técnica (Resumen)

**Nuevos tokens necesarios:**
- `&&` (AND), `||` (OR), `!` (NOT)
- `true`, `false` (booleanos)

**Nuevos nodos AST:**
```rust
pub enum AstNode {
    // ... existentes ...

    If {
        condition: Box<AstNode>,
        then_expr: Box<AstNode>,
        else_expr: Box<AstNode>,
    },

    Piecewise {
        cases: Vec<(AstNode, AstNode)>,  // (condicion, valor)
        default: Option<Box<AstNode>>,
    },

    Boolean(bool),
}

pub enum BinaryOp {
    // ... existentes ...
    And,  // &&
    Or,   // ||
}

pub enum UnaryOp {
    Negate,  // existente
    Not,     // ! (nuevo)
}
```

**Evaluación:**
- Las condiciones evalúan a booleanos
- `if()` evalúa condición, retorna rama correspondiente
- `piecewise()` evalúa condiciones en orden (short-circuit)
- Si ninguna condición se cumple y no hay default → error

### Razón para Posponer: Migración a Pest

**Problema actual:** El AST está creciendo rápidamente con un parser hand-written, lo que dificulta el mantenimiento y la adición de nuevas features.

**Solución:** Migrar a **Pest** (parser generator con PEG) antes de implementar condicionales.

**Beneficios de migrar primero:**
1. Gramática declarativa más fácil de extender
2. Mejor manejo de errores
3. Parsing más robusto
4. Menos código manual que mantener
5. Preparación para features futuras (pattern matching, loops, etc.)

**Plan recomendado:**
1. ✅ Documentar propuesta de condicionales (este documento)
2. ✅ Migrar parser actual a Pest (completado en v0.5.3)
3. ✅ Validar migración con todos los ejemplos SOC (13/15 funcionando)
4. 🔜 Remover parser hand-written (lexer.rs, parser.rs) - deprecado
5. 🔜 Implementar condicionales sobre la base de Pest
6. 🔜 Implementar piecewise functions
7. 🔜 Implementar sintaxis de grafos para algoritmos de redes

### Referencias

- Sintaxis actual de lambdas: `x => x^2`, `(x, y) => x + y`
- Operadores comparación existentes: `>`, `<`, `>=`, `<=`, `==`, `!=`
- Funciones de orden superior existentes: `map`, `filter`, `reduce`

---

## ✅ Estado de Migración a Pest (Completado v0.5.3)

**MIGRACIÓN COMPLETADA** - El parser de Pest está ahora funcionando como parser principal.

### ✅ Logros Completados

1. **Parser Pest Implementado**
   - ✅ Gramática completa en `grammar.pest` (~150 líneas)
   - ✅ Módulo `pest_parser.rs` con generación de AST
   - ✅ API `eval_str()` para evaluación directa
   - ✅ Manejo correcto de precedencia y asociatividad
   - ✅ Soporte para comentarios y multi-línea

2. **CLI Actualizado**
   - ✅ Usa `eval_str()` en lugar de Lexer→Parser→Evaluator
   - ✅ Procesa archivos completos (no línea por línea)
   - ✅ 13 de 15 ejemplos funcionando correctamente

3. **Tests Validados**
   - ✅ 8 tests de parser específicos
   - ✅ 12 tests de evaluador con Pest
   - ✅ Todos los ejemplos de optimización funcionando

### 🔜 Próximos Pasos (v0.6.0)

**Prioridad Alta:**
1. ⏳ Implementar condicionales (`if()`) en gramática Pest
2. ⏳ Implementar funciones por partes (`piecewise()`)
3. ⏳ Agregar operadores lógicos (`&&`, `||`, `!`)
4. ⏳ Agregar sintaxis de grafos para algoritmos de redes

**Limpieza del Código:**
5. 🔜 Remover parser hand-written (lexer.rs, parser.rs)
6. 🔜 Deprecar exports del parser antiguo
7. 🔜 Limpiar dependencias no usadas
8. 🔜 Actualizar toda la documentación

### Archivos a Remover (Deprecados)

```
crates/achronyme-parser/src/
├── lexer.rs          ← REMOVER (deprecado)
├── parser.rs         ← REMOVER (deprecado)
├── token.rs          ← REMOVER (deprecado)
└── pest_parser.rs    ← MANTENER (parser principal)
```

---

## Propuesta Original: Parser Hand-Written → Pest

*Esta sección documenta la propuesta original. Ver arriba para el estado actual.*

### Motivación Original

**Problema:** El parser hand-written actual (`achronyme-parser`) estaba creciendo en complejidad:
- AST con 10+ variantes de nodos
- Parsing manual de tokens
- Difícil agregar nuevas features (condicionales, loops, pattern matching)
- Propenso a errores de precedencia y asociatividad
- Difícil de mantener y testear

**Solución Implementada:** Migración a **Pest** - un parser generator basado en PEG (Parsing Expression Grammars).

### Beneficios de Pest

1. **Gramática Declarativa:**
   ```pest
   expr = { term ~ (("+" | "-") ~ term)* }
   term = { factor ~ (("*" | "/") ~ factor)* }
   factor = { number | "(" ~ expr ~ ")" }
   ```

2. **Mejor Manejo de Errores:**
   - Mensajes de error precisos con ubicación
   - Stack trace de reglas de parsing
   - Fácil debugging

3. **Más Robusto:**
   - Precedencia de operadores clara
   - Asociatividad explícita
   - Whitespace handling automático

4. **Fácil de Extender:**
   - Agregar condicionales: solo agregar regla `if_expr`
   - Agregar loops: solo agregar regla `for_expr`
   - Agregar pattern matching: solo agregar regla `match_expr`

5. **Menos Código:**
   - ~300-500 líneas de gramática Pest
   - vs ~1000+ líneas de parser hand-written
   - Menos bugs, más mantenible

### Ejemplo: Gramática Actual en Pest

```pest
// grammar.pest - Lenguaje SOC

WHITESPACE = _{ " " | "\t" | "\n" | "\r" }
COMMENT = _{ "//" ~ (!"\n" ~ ANY)* }

// Literales
number = @{ "-"? ~ ASCII_DIGIT+ ~ ("." ~ ASCII_DIGIT+)? ~ (^"e" ~ ("+" | "-")? ~ ASCII_DIGIT+)? }
complex = { number ~ "i" }
identifier = @{ ASCII_ALPHA ~ (ASCII_ALPHANUMERIC | "_")* }

// Vectores y matrices
vector = { "[" ~ expr ~ ("," ~ expr)* ~ "]" }
matrix = { "[" ~ vector ~ ("," ~ vector)* ~ "]" }

// Expresiones
primary = {
    number
  | complex
  | vector
  | matrix
  | identifier
  | lambda
  | function_call
  | "(" ~ expr ~ ")"
}

// Operadores (con precedencia implícita)
power = { primary ~ ("^" ~ primary)* }
unary = { ("-" | "!")? ~ power }
factor = { unary ~ (("*" | "/" | "%") ~ unary)* }
term = { factor ~ (("+" | "-") ~ factor)* }
comparison = { term ~ ((">" | "<" | ">=" | "<=" | "==" | "!=") ~ term)? }
logical_and = { comparison ~ ("&&" ~ comparison)* }
logical_or = { logical_and ~ ("||" ~ logical_and)* }

expr = { logical_or }

// Lambdas
lambda = { lambda_params ~ "=>" ~ expr }
lambda_params = { identifier | ("(" ~ identifier ~ ("," ~ identifier)* ~ ")") }

// Function calls
function_call = { identifier ~ "(" ~ (expr ~ ("," ~ expr)*)? ~ ")" }

// Statements
let_stmt = { "let" ~ identifier ~ "=" ~ expr }
statement = { let_stmt | expr }

// Program
program = { SOI ~ statement ~ (statement)* ~ EOI }
```

### Plan de Migración

**Fase 1: Setup Pest**
1. Agregar dependencia `pest = "2.7"` y `pest_derive = "2.7"`
2. Crear `grammar.pest` con gramática básica
3. Crear `pest_parser.rs` con generación de AST

**Fase 2: Migrar Features Existentes**
1. Migrar aritmética básica (`+`, `-`, `*`, `/`, `^`)
2. Migrar funciones matemáticas
3. Migrar vectores y matrices
4. Migrar lambdas y HOF
5. Migrar variables (`let`)

**Fase 3: Validación**
1. Ejecutar todos los tests existentes
2. Comparar AST generado (Pest vs hand-written)
3. Benchmarks de performance

**Fase 4: Nuevas Features** (Próxima Prioridad)
1. ⏳ Implementar condicionales (`if`, `piecewise`)
2. ⏳ Implementar operadores lógicos (`&&`, `||`, `!`)
3. ⏳ Agregar tests para nuevas features

**Fase 5: Cleanup** (Después de Condicionales)
1. 🔜 Remover parser hand-written (lexer.rs, parser.rs)
2. 🔜 Deprecar exports del parser antiguo
3. 🔜 Actualizar documentación para reflejar solo Pest
4. 🔜 Release v0.6.0 con condicionales y sin parser legacy

### Estimación de Esfuerzo

- **Fase 1-2:** ~2-3 días (gramática + migración)
- **Fase 3:** ~1 día (validación)
- **Fase 4:** ~1-2 días (condicionales)
- **Fase 5:** ~0.5 días (cleanup)
- **Total:** ~1 semana de trabajo

### Riesgos y Mitigación

**Riesgo 1:** Cambios en AST rompen evaluador
- **Mitigación:** Mantener estructura de AST existente, solo cambiar generación

**Riesgo 2:** Performance regression
- **Mitigación:** Benchmarks antes/después, Pest es muy eficiente

**Riesgo 3:** Bugs en migración
- **Mitigación:** Test suite completo, migración incremental

### Decisión Recomendada

**Proceder con migración a Pest ANTES de implementar condicionales.**

Esto nos dará una base sólida para agregar:
- Condicionales y piecewise functions
- Loops (`for`, `while`)
- Pattern matching (`match`)
- Bloques de código
- Imports/modules
- Y cualquier feature futura

### Referencias

- [Pest Book](https://pest.rs/book/)
- [Pest GitHub](https://github.com/pest-parser/pest)
- [Pest Examples](https://github.com/pest-parser/pest/tree/master/pest/examples)

---

## Limitaciones Conocidas y Diseño Intencional

### Programación Lineal: Forma Estándar Requerida

**Estado Actual (v0.5.3):**

Todos los solvers de LP (`linprog`, `simplex`, `dual_simplex`, `revised_simplex`, `two_phase_simplex`) requieren que el usuario convierta su problema a **forma estándar**:

```
maximize/minimize z = c^T × x
subject to: Ax ≤ b, x ≥ 0
```

**Restricciones:**
- TODAS las restricciones deben ser `Ax ≤ b` (menor-o-igual)
- TODOS los valores en `b` deben ser no-negativos (b ≥ 0)
- El usuario es responsable de convertir restricciones mixtas (≥, =) a esta forma

**Conversiones Requeridas:**

| Tipo | Original | Forma Estándar |
|------|----------|----------------|
| Mayor-igual | `x₁ + x₂ ≥ 5` | `-x₁ - x₂ ≤ -5` |
| Igualdad | `x₁ + x₂ = 5` | Dos restricciones: `x₁ + x₂ ≤ 5` Y `-x₁ - x₂ ≤ -5` |
| RHS negativo | `x₁ ≤ -3` | `-x₁ ≤ 3` |

**Filosofía de Diseño:**

Esta es una **decisión intencional**, no un bug:

1. **Simplicidad**: Mantiene la sintaxis del lenguaje limpia y matemática
2. **Control**: El usuario mantiene control total sobre la formulación del problema
3. **Educación**: Fuerza comprensión de conceptos fundamentales de LP
4. **Minimalismo**: Evita sintaxis mágica específica del dominio
5. **Extensibilidad**: Más fácil agregar nuevos métodos sin complicar la API

**Opciones Consideradas (Rechazadas por Ahora):**

```javascript
// Opción A: Vector de tipos de restricción
linprog(c, A, b, ["<=", ">=", "="], sense)  // ❌ Complejo

// Opción B: Matriz extendida
let constraints = [[[1,1], 5, "="], [[2,1], 4, ">="]]  // ❌ Confuso

// Opción C: Funciones separadas
linprog_mixed(c, A_le, b_le, A_ge, b_ge, A_eq, b_eq, sense)  // ❌ Verboso
```

**¿Cuándo Podría Cambiar?**

Si en el futuro Achronyme implementa un sistema de modelado de optimización de alto nivel (como AMPL o GAMS), podríamos considerar sintaxis más expresiva. Pero para el núcleo del lenguaje matemático, mantenemos la simplicidad.

**Workaround Actual:**

Los ejemplos en `examples/soc/` muestran cómo convertir problemas comunes a forma estándar. Ver especialmente:
- `08-simple-linprog-test.soc` - Maximización estándar
- `09-production-problem.soc` - Problema de producción
- `11-two-phase-example.soc` - Minimización con conversión documentada