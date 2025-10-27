# Especificación del Lenguaje Achronyme (SOC)

**SOC** - Superior Order Calculator

Achronyme implementa un lenguaje de expresiones matemáticas completo y extensible, diseñado para competir con motores de cálculo como Wolfram Mathematica en el ámbito open-source.

---

## 📋 Tabla de Contenidos

- [Gramática Formal (BNF)](#gramática-formal-bnf)
- [Tipos de Datos](#tipos-de-datos)
- [Operadores y Precedencia](#operadores-y-precedencia)
- [Sintaxis Completa](#sintaxis-completa)
- [Sistema de Tipos](#sistema-de-tipos)
- [Constantes Matemáticas](#constantes-matemáticas)

---

## 🔤 Gramática Formal (BNF)

```bnf
# Declaraciones
statement       → variable_decl | expression

variable_decl   → "let" IDENTIFIER "=" expression

# Expresiones
expression      → lambda | comparison

lambda          → param_list "=>" expression
                | IDENTIFIER "=>" expression

param_list      → "(" IDENTIFIER ("," IDENTIFIER)* ")"

comparison      → additive ( (">" | "<" | ">=" | "<=" | "==" | "!=") additive )*

additive        → term ( ("+" | "-") term )*

term            → factor ( ("*" | "/" | "%") factor )*

factor          → exponent ("^" exponent)*       # Right-associative

exponent        → unary

unary           → "-" unary
                | primary

primary         → NUMBER
                | COMPLEX_LITERAL               # 3i, (2+3)i
                | VECTOR_LITERAL                # [1, 2, 3]
                | MATRIX_LITERAL                # [[1,2], [3,4]]
                | function_call
                | IDENTIFIER                    # Variable or constant
                | "(" expression ")"

function_call   → IDENTIFIER "(" arg_list? ")"

arg_list        → expression ("," expression)*

# Literales complejos
COMPLEX_LITERAL → [NUMBER] "i"
                | expression "i"

# Vectores
VECTOR_LITERAL  → "[" arg_list? "]"

# Matrices
MATRIX_LITERAL  → "[" matrix_rows "]"
matrix_rows     → "[" arg_list "]" ("," "[" arg_list "]")*
```

---

## 🎯 Tipos de Datos

Achronyme soporta 5 tipos de datos fundamentales:

| Tipo | Descripción | Literales | Ejemplos |
|------|-------------|-----------|----------|
| **Number** | Números de punto flotante (64-bit) | `123`, `3.14`, `.5`, `2e-3` | `42`, `-3.14159`, `1.23e-10` |
| **Complex** | Números complejos (real + imaginary) | `3i`, `2+3i` | `(1+2i) * (3+4i)` → `-5+10i` |
| **Vector** | Vectores de números reales | `[1, 2, 3]` | `[1, 2, 3] + [4, 5, 6]` → `[5, 7, 9]` |
| **Matrix** | Matrices 2D | `[[1,2], [3,4]]` | `det([[1,2],[3,4]])` → `-2` |
| **Function** | Funciones lambda | `x => x^2` | `map(x => x*2, [1,2,3])` → `[2,4,6]` |

### Number - Números Reales

```javascript
// Enteros
42
-17
0

// Decimales
3.14159
-2.718
.5              // Equivalente a 0.5

// Notación científica
1e6             // 1,000,000
2.5e-3          // 0.0025
-3.14e2         // -314.0
```

### Complex - Números Complejos

```javascript
// Imaginario puro
3i              // 0 + 3i
-2i             // 0 - 2i

// Complejo completo (suma de real + imaginario)
2 + 3i          // 2 + 3i
5 - 4i          // 5 - 4i

// Operaciones con complejos
(1+2i) + (3+4i)     // → 4+6i
(2+3i) * (1-i)      // → 5+i
abs(3+4i)           // → 5 (magnitud)
arg(1+i)            // → 0.785... (π/4 radianes)
conj(2+3i)          // → 2-3i (conjugado)
```

### Vector - Vectores

```javascript
// Literales de vector
[1, 2, 3]
[sin(0), cos(0), tan(0)]
[1+2i, 3+4i, 5+6i]      // Vector de complejos

// Operaciones vectoriales
[1,2,3] + [4,5,6]       // → [5, 7, 9] (element-wise)
[2,4,6] * [1,2,3]       // → [2, 8, 18] (element-wise)
[1,2,3] ^ 2             // → [1, 4, 9] (broadcast)

// Funciones vectoriales
dot([1,2,3], [4,5,6])   // → 32 (producto punto)
norm([3,4])             // → 5 (norma euclidiana)
cross([1,0,0], [0,1,0]) // → [0,0,1] (producto cruz, solo 3D)
```

### Matrix - Matrices

```javascript
// Literales de matriz
[[1, 2], [3, 4]]
[[1, 0, 0], [0, 1, 0], [0, 0, 1]]   // Identidad 3x3

// Operaciones matriciales
[[1,2],[3,4]] + [[5,6],[7,8]]       // Suma element-wise
[[1,2],[3,4]] * [[5,6],[7,8]]       // Multiplicación matricial

// Funciones matriciales
transpose([[1,2],[3,4]])            // → [[1,3],[2,4]]
det([[1,2],[3,4]])                  // → -2 (determinante)
inverse([[1,2],[3,4]])              // → [[-2,1],[1.5,-0.5]]
```

### Function - Lambdas

```javascript
// Lambda de un parámetro
x => x^2
n => sin(n * PI)

// Lambda de múltiples parámetros
(x, y) => x + y
(a, b, c) => sqrt(a^2 + b^2 + c^2)

// Lambdas como argumentos
map(x => x*2, [1, 2, 3])            // → [2, 4, 6]
filter(n => n > 5, [1,5,10,15])     // → [10, 15]
reduce((a,b) => a+b, 0, [1,2,3,4])  // → 10
```

---

## ⚙️ Operadores y Precedencia

Ordenados de **mayor a menor precedencia** (como en matemáticas estándar):

| Precedencia | Operador | Tipo | Asociatividad | Ejemplo |
|-------------|----------|------|---------------|---------|
| 1 (mayor) | `()` | Agrupación | - | `(2 + 3) * 4` |
| 2 | `f()` | Llamada a función | Izquierda | `sin(PI/2)` |
| 3 | `-` (unario) | Negación | Derecha | `-5`, `-(2+3)` |
| 4 | `^` | Potencia | **Derecha** ⚠️ | `2^3^2 = 512` |
| 5 | `*`, `/`, `%` | Multiplicación, División, Módulo | Izquierda | `6 / 2 * 3` |
| 6 | `+`, `-` | Suma, Resta | Izquierda | `2 + 3 - 1` |
| 7 | `>`, `<`, `>=`, `<=` | Comparación | Izquierda | `x > 5` |
| 8 | `==`, `!=` | Igualdad | Izquierda | `x == 10` |
| 9 | `=>` | Lambda | Derecha | `x => x^2` |
| 10 (menor) | `=` | Asignación | Derecha | `let x = 5` |

### ⚠️ Nota Importante: Potencia es Asociativa a la Derecha

```javascript
2^3^2       // = 2^(3^2) = 2^9 = 512 (correcto)
            // NO es (2^3)^2 = 8^2 = 64

// Para forzar asociatividad izquierda, usar paréntesis:
(2^3)^2     // = 64
```

---

## 📖 Sintaxis Completa

### Variables

```javascript
// Declaración
let x = 10
let result = sin(PI/2)
let vec = [1, 2, 3, 4]
let matrix = [[1,2],[3,4]]

// Uso
x + 5               // → 15
result * 2          // → 2
vec + [1,1,1,1]     // → [2, 3, 4, 5]
```

### Funciones Matemáticas Básicas

```javascript
// Trigonométricas
sin(PI/2)           // → 1
cos(0)              // → 1
tan(PI/4)           // → 1
asin(1)             // → π/2
atan2(1, 1)         // → π/4

// Hiperbólicas
sinh(0)             // → 0
cosh(0)             // → 1
tanh(1)             // → 0.762

// Exponenciales y logaritmos
exp(1)              // → e = 2.718...
ln(E)               // → 1
log10(100)          // → 2
log2(8)             // → 3

// Raíces y potencias
sqrt(16)            // → 4
cbrt(27)            // → 3
pow(2, 10)          // → 1024

// Redondeo
floor(3.7)          // → 3
ceil(3.2)           // → 4
round(3.5)          // → 4

// Utilidades
abs(-5)             // → 5
sign(-3)            // → -1
min(1, 5, 3)        // → 1
max(1, 5, 3)        // → 5
```

### Higher-Order Functions

```javascript
// map: Aplicar función a cada elemento
map(x => x^2, [1, 2, 3, 4])
// → [1, 4, 9, 16]

map((x,y) => x+y, [1,2,3], [4,5,6])
// → [5, 7, 9]

// filter: Filtrar elementos por predicado
filter(x => x > 5, [1, 5, 10, 15])
// → [10, 15]

filter(n => n % 2 == 0, [1,2,3,4,5,6])
// → [2, 4, 6]

// reduce: Reducir a un solo valor
reduce((a,b) => a+b, 0, [1,2,3,4])
// → 10

reduce((acc,x) => acc*x, 1, [2,3,4])
// → 24

// pipe: Composición de funciones (pipeline)
pipe(
  [1, 2, 3, 4],
  v => map(x => x^2, v),
  v => filter(x => x > 5, v),
  v => reduce((a,b) => a+b, 0, v)
)
// [1,2,3,4] → [1,4,9,16] → [9,16] → 25
```

### DSP - Procesamiento de Señales

```javascript
// FFT (requiere potencia de 2)
fft([1, 2, 3, 4, 5, 6, 7, 8])
// → Spectrum complejo

fft_mag([1, 2, 3, 4, 5, 6, 7, 8])
// → [20, 9.65, 5.83, 4.83, ...] (magnitudes)

// DFT (acepta cualquier tamaño)
dft([1, 2, 3, 4, 5])
dft_mag([1, 2, 3, 4, 5])

// FFT inversa
let signal = [1, 2, 3, 4, 5, 6, 7, 8]
let spectrum = fft(signal)
let reconstructed = ifft(spectrum)
// reconstructed ≈ signal (reconstrucción perfecta)

// Ventanas (windowing)
hanning(8)      // → [0, 0.188, 0.612, 0.950, 0.950, 0.612, 0.188, 0]
hamming(8)      // → Ventana de Hamming
blackman(8)     // → Ventana de Blackman

// Aplicar ventana a señal
let windowed = map((s,w) => s*w, signal, hanning(8))

// Convolución (filtrado FIR)
conv([1,2,3,4,5], [0.333, 0.333, 0.333])
// → Filtro de promedio móvil

conv_fft([1,2,3,4,5,6,7,8], [1,2,1])
// → Convolución rápida con FFT
```

### Pipelines Completos

```javascript
// Pipeline de análisis espectral
let analyze = sig => pipe(
  sig,
  s => map((val,w) => val*w, s, hanning(8)),
  s => fft_mag(s),
  s => map(m => m^2, s),
  s => reduce((a,b) => a+b, 0, s)
)

analyze([1,2,3,4,5,6,7,8])
// → Potencia espectral total

// Pipeline de filtrado
let lowpass = sig => pipe(
  sig,
  s => conv(s, [0.25, 0.5, 0.25]),
  s => map(x => round(x*100)/100, s)
)

lowpass([1,5,2,8,3,9])
// → Señal filtrada y redondeada
```

---

## 🔍 Sistema de Tipos

Achronyme implementa **inferencia de tipos dinámica** con **promoción automática**:

### Promoción Automática

```javascript
// Promoción Number → Complex
2 + 3i              // 2 es promovido a 2+0i
                    // → 2+3i

// Promoción Number → Vector (broadcasting)
[1,2,3] + 5         // 5 es broadcast a todos los elementos
                    // → [6, 7, 8]

[2,4,6] * 3         // → [6, 12, 18]

// Tipo común en operaciones
let x = 2           // Number
let y = 3i          // Complex
x + y               // x promovido a Complex → 2+3i
```

### Polimorfismo en Lambdas

```javascript
// Detección de tipos en lambdas
let f = x => x^2    // f acepta cualquier tipo
f(5)                // → 25 (Number)
f(2+3i)             // → -5+12i (Complex)
f([1,2,3])          // → [1,4,9] (Vector)
```

---

## 📐 Constantes Matemáticas

| Constante | Valor | Descripción |
|-----------|-------|-------------|
| `PI` | 3.14159265358979... | Número π (relación circunferencia/diámetro) |
| `E` | 2.71828182845905... | Número e (base de logaritmos naturales) |
| `PHI` | 1.61803398874989... | Razón áurea φ = (1+√5)/2 |
| `TAU` | 6.28318530717959... | τ = 2π (constante del círculo) |

```javascript
// Uso de constantes
sin(PI/2)           // → 1
exp(1) == E         // → true (1 == 1)
log(E)              // → 1
TAU / 2 == PI       // → true (1 == 1)
```

---

## 📚 Referencias

- [Guía del SDK TypeScript](./sdk-guide.md)
- [Roadmap del Proyecto](./roadmap.md)
- [Comparación con Wolfram](./wolfram-comparison.md)
- [README Principal](../README.md)

---

**Versión**: 0.3.0
**Última actualización**: 2025
