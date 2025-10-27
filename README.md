# Achronyme Core

**Motor de cálculo matemático con capacidades DSP y programación funcional**

Achronyme Core es un motor de computación matemática compilado a WebAssembly que combina:
- Operaciones matemáticas avanzadas
- Procesamiento digital de señales (DSP)
- Programación funcional con lambdas y closures
- Tipos complejos (Complex, Vector, Matrix)

---

## 📋 Tabla de Contenidos

- [Características](#-características)
- [Requisitos Previos](#-requisitos-previos)
- [Instalación](#-instalación)
- [Compilación](#-compilación)
- [Uso Rápido](#-uso-rápido)
- [SDK TypeScript (Recomendado)](#-sdk-typescript-recomendado)
- [Ejecutar Tests](#-ejecutar-tests)
- [Estructura del Proyecto](#-estructura-del-proyecto)
- [API Reference](#-api-reference)
- [Ejemplos Avanzados](#-ejemplos-avanzados)
- [Solución de Problemas](#-solución-de-problemas)

---

## ✨ Características

### **Operaciones Básicas**
- **Aritmética**: `+`, `-`, `*`, `/`, `^`, unary `-`
- **Trigonometría**: `sin`, `cos`, `tan`, `asin`, `acos`, `atan`, `atan2`, `sinh`, `cosh`, `tanh`
- **Exponenciales**: `exp`, `log`, `log10`, `log2`, `ln`
- **Raíces**: `sqrt`, `cbrt`, `pow`
- **Redondeo**: `floor`, `ceil`, `round`, `trunc`
- **Utilidades**: `abs`, `sign`, `min`, `max`
- **Constantes**: `PI`, `E`, `PHI`, `TAU`

### **Tipos Complejos**
- **Complex**: Números complejos con aritmética completa (`3i`, `2+3i`)
- **Vector**: Operaciones vectoriales, producto punto, norma (`[1, 2, 3]`)
- **Matrix**: Matrices con transpose, determinante, inversa (`[[1,2],[3,4]]`)

### **Programación Funcional**
- **Variables**: Estado persistente entre evaluaciones (`let x = 10`)
- **Lambdas**: Funciones anónimas de uno o múltiples parámetros (`x => x^2`)
- **Closures**: Captura de scope externo
- **Higher-order functions**: `map`, `filter`, `reduce`, `pipe`
- **Comparaciones**: `>`, `<`, `>=`, `<=`, `==`, `!=`

### **Procesamiento Digital de Señales (DSP)**
- **DFT**: Transformada Discreta de Fourier O(N²)
- **FFT**: Transformada Rápida de Fourier O(N log N) (algoritmo Cooley-Tukey)
- **IFFT**: FFT Inversa con reconstrucción perfecta
- **Convolución**:
  - `conv()` - Método directo O(N×M)
  - `conv_fft()` - Basado en FFT O((N+M) log(N+M))
- **Ventanas**:
  - `hanning()` - Ventana de Hann (supresión -31 dB)
  - `hamming()` - Ventana de Hamming (supresión -43 dB)
  - `blackman()` - Ventana de Blackman (supresión -58 dB)

---

## 🔧 Requisitos Previos

### **1. Emscripten SDK**

Emscripten es necesario para compilar C++ a WebAssembly.

#### **Windows:**
```bash
# Descargar e instalar Emscripten
git clone https://github.com/emscripten-core/emsdk.git
cd emsdk
emsdk install latest
emsdk activate latest

# Activar en la terminal actual
emsdk_env.bat
```

#### **Linux/macOS:**
```bash
# Descargar e instalar Emscripten
git clone https://github.com/emscripten-core/emsdk.git
cd emsdk
./emsdk install latest
./emsdk activate latest

# Activar en la terminal actual
source ./emsdk_env.sh
```

**Verificar instalación:**
```bash
emcc --version
# Debería mostrar: emcc (Emscripten gcc/clang-like replacement) X.X.X
```

### **2. Node.js**

Node.js v16+ es necesario para ejecutar los tests.

**Descargar desde:** https://nodejs.org/

**Verificar instalación:**
```bash
node --version
# Debería mostrar: v16.x.x o superior
```

---

## 📦 Instalación

```bash
# Clonar el repositorio (o descomprimir el archivo)
cd achronyme-core

# No hay dependencias npm, el proyecto es standalone
```

---

## 🔨 Compilación

### **Compilación Completa**

#### **Windows:**
```bash
# Asegurarse de estar en el directorio raíz del proyecto
cd achronyme-core

# Entrar al directorio wasm
cd wasm

# Compilar con Emscripten
emcc src/core/complex.cpp ^
     src/core/vector.cpp ^
     src/core/matrix.cpp ^
     src/core/function.cpp ^
     src/core/value.cpp ^
     src/parser/lexer.cpp ^
     src/parser/parser.cpp ^
     src/parser/evaluator.cpp ^
     src/bindings/main.cpp ^
     src/core/constants.cpp ^
     src/core/functions.cpp ^
     src/core/functions_hof.cpp ^
     src/core/functions_dsp.cpp ^
     -o ../dist/achronyme-core.mjs ^
     -O3 ^
     -s MODULARIZE=1 ^
     -s EXPORT_NAME="createAchronymeModule" ^
     -s EXPORTED_FUNCTIONS="['_malloc','_free']" ^
     -s EXPORTED_RUNTIME_METHODS="['ccall','cwrap']" ^
     --bind ^
     -s ALLOW_MEMORY_GROWTH=1 ^
     -std=c++17

# Volver al directorio raíz
cd ..
```

#### **Linux/macOS:**
```bash
cd achronyme-core/wasm

emcc src/core/complex.cpp \
     src/core/vector.cpp \
     src/core/matrix.cpp \
     src/core/function.cpp \
     src/core/value.cpp \
     src/parser/lexer.cpp \
     src/parser/parser.cpp \
     src/parser/evaluator.cpp \
     src/bindings/main.cpp \
     src/core/constants.cpp \
     src/core/functions.cpp \
     src/core/functions_hof.cpp \
     src/core/functions_dsp.cpp \
     -o ../dist/achronyme-core.mjs \
     -O3 \
     -s MODULARIZE=1 \
     -s EXPORT_NAME="createAchronymeModule" \
     -s EXPORTED_FUNCTIONS="['_malloc','_free']" \
     -s EXPORTED_RUNTIME_METHODS="['ccall','cwrap']" \
     --bind \
     -s ALLOW_MEMORY_GROWTH=1 \
     -std=c++17

cd ..
```

### **Resultado de la Compilación**

Después de compilar exitosamente, encontrarás:
- `dist/achronyme-core.mjs` - Módulo ES6 de WebAssembly
- `dist/achronyme-core.wasm` - Binario WebAssembly

---

## 🚀 Uso Rápido

### **Modo Básico**

```javascript
import createAchronymeModule from './dist/achronyme-core.mjs';

// Cargar el módulo WASM
const Module = await createAchronymeModule();

// Evaluar expresiones
console.log(Module.eval('2 + 2'));              // → "4"
console.log(Module.eval('sin(PI / 2)'));        // → "1"
console.log(Module.eval('sqrt(16)'));           // → "4"

// Resetear el entorno (limpiar variables)
Module.reset();
```

### **Con Variables y Funciones**

```javascript
const Module = await createAchronymeModule();
Module.reset();

// Definir variables
Module.eval('let x = 10');
Module.eval('let y = 20');
console.log(Module.eval('x + y'));              // → "30"

// Definir funciones lambda
Module.eval('let square = n => n ^ 2');
console.log(Module.eval('square(5)'));          // → "25"

// Higher-order functions
Module.eval('let double = n => n * 2');
console.log(Module.eval('map(double, [1,2,3])'));
// → "[2.000000, 4.000000, 6.000000]"
```

### **DSP: Análisis Espectral**

```javascript
const Module = await createAchronymeModule();
Module.reset();

// FFT de una señal
const signal = '[1, 2, 3, 4, 5, 6, 7, 8]';
const spectrum = Module.eval(`fft_mag(${signal})`);
console.log('Spectrum:', spectrum);

// Con ventana de Hanning
const windowed = Module.eval(`
  map((s, w) => s * w, ${signal}, hanning(8))
`);
const windowedSpectrum = Module.eval(`fft_mag(${windowed})`);
console.log('Windowed spectrum:', windowedSpectrum);
```

### **Convolución (Filtrado FIR)**

```javascript
const Module = await createAchronymeModule();
Module.reset();

// Filtro de promedio móvil
const result = Module.eval(`
  conv([1, 2, 3, 4, 5], [0.333, 0.333, 0.333])
`);
console.log('Filtered:', result);

// Convolución rápida con FFT (para señales grandes)
const fastResult = Module.eval(`
  conv_fft([1, 2, 3, 4, 5, 6, 7, 8], [1, 1, 1])
`);
console.log('Fast convolution:', fastResult);
```

---

## 🎯 SDK TypeScript (Recomendado)

**Nuevo**: Achronyme ahora incluye un SDK de TypeScript que proporciona una API tipo-segura y mucho más ergonómica sobre el núcleo WASM.

### **¿Por qué usar el SDK?**

En lugar de construir strings de comandos manualmente:
```javascript
// ❌ Antiguo: eval() directo (engorroso)
Module.eval("let sig = [1, 2, 3, 4, 5, 6, 7, 8]");
Module.eval("let spec = fft_mag(sig)");
const result = Module.eval("spec");
```

Usa una API idiomática de TypeScript:
```typescript
// ✅ Nuevo: SDK TypeScript (limpio y tipo-seguro)
const ach = new Achronyme();
await ach.init();

const sig = ach.vector([1, 2, 3, 4, 5, 6, 7, 8]);
const spec = sig.fft_mag();
console.log(await spec.toVector());

// Limpieza de memoria
sig.dispose();
spec.dispose();
```

### **Beneficios del SDK**

✅ **Type Safety**: TypeScript detecta errores en desarrollo
✅ **Autocompletado**: IntelliSense muestra todas las funciones
✅ **API Fluent**: Encadenamiento intuitivo de operaciones
✅ **Manejo de Errores**: Excepciones personalizadas (AchronymeSyntaxError, AchronymeTypeError, etc.)
✅ **Gestión de Memoria**: Control explícito con `dispose()`
✅ **Sin Overhead**: Internamente usa el mismo Environment de C++

### **Instalación del SDK**

```bash
# El SDK está incluido en el proyecto
# Solo necesitas compilar el TypeScript

npm install typescript --save-dev
npx tsc --project tsconfig.sdk.json
```

### **Ejemplo Básico**

```typescript
import { Achronyme } from './src/sdk';

const ach = new Achronyme();
await ach.init();

// Operaciones matemáticas
const x = ach.number(5);
const y = x.mul(2).add(10).div(4);
console.log(await y.toNumber()); // 5

// Vectores
const v = ach.vector([1, 2, 3, 4]);
const squared = v.pow(2);
console.log(await squared.toVector()); // [1, 4, 9, 16]

// DSP
const signal = ach.vector([1, 2, 3, 4, 5, 6, 7, 8]);
const spectrum = signal.fft_mag();
console.log(await spectrum.toVector());

// Limpieza
x.dispose();
y.dispose();
v.dispose();
squared.dispose();
signal.dispose();
spectrum.dispose();
```

### **Gestión de Memoria**

El SDK usa **gestión manual explícita** mediante `dispose()`:

```typescript
// ✅ Correcto: Llamar dispose() cuando termines
const x = ach.number(10);
const y = x.add(5);
console.log(await y.toNumber());
x.dispose();
y.dispose();

// ❌ Incorrecto: No disponer puede causar fugas de memoria
const z = ach.vector([1, 2, 3]);
// ... usar z ...
// (olvidaste dispose) ← Fuga de memoria en C++
```

**¿Por qué gestión manual?**

- El GC de JavaScript no conoce la memoria de WASM/C++
- `FinalizationRegistry` es experimental y no garantiza limpieza inmediata
- `dispose()` manual es explícito, confiable y te da control total
- Evita fugas de memoria cuando hay muchas variables intermedias

**Estadísticas de memoria:**
```typescript
const stats = ach.getMemoryStats();
console.log('Variables activas:', stats.activeVariables);
console.log('Variables eliminadas:', stats.disposedVariables);

// Limpiar todo (CUIDADO: invalida todos los AchronymeValue)
ach.disposeAll();
```

### **Ejemplos Completos**

El proyecto incluye 4 ejemplos del SDK:

```bash
# Ejemplo 1: Operaciones básicas
node examples/basic-usage.mjs

# Ejemplo 2: DSP (FFT, ventanas, convolución)
node examples/dsp-example.mjs

# Ejemplo 3: Programación funcional (map, filter, reduce)
node examples/functional-programming.mjs

# Ejemplo 4: Pipeline DSP avanzado
node examples/advanced-dsp-pipeline.mjs
```

### **Test del SDK**

```bash
# Ejecutar test del SDK (20+ tests)
node test-sdk.mjs
```

### **API Completa del SDK**

#### **Clase Achronyme**

**Inicialización:**
```typescript
const ach = new Achronyme(options?);
await ach.init();
```

**Constructores de tipos:**
```typescript
ach.number(42)
ach.vector([1, 2, 3, 4])
ach.matrix([[1, 2], [3, 4]])
ach.complex(2, 3)  // 2+3i
```

**Funciones matemáticas:**
```typescript
ach.sin(x), ach.cos(x), ach.tan(x)
ach.sqrt(x), ach.exp(x), ach.ln(x), ach.log(x)
ach.abs(x), ach.floor(x), ach.ceil(x), ach.round(x)
ach.min(...values), ach.max(...values)
// ... +60 funciones más
```

**Funciones DSP:**
```typescript
ach.fft(signal), ach.fft_mag(signal), ach.ifft(spectrum)
ach.dft(signal), ach.dft_mag(signal), ach.dft_phase(signal)
ach.conv(sig1, sig2), ach.conv_fft(sig1, sig2)
ach.hanning(n), ach.hamming(n), ach.blackman(n)
```

**Higher-order functions:**
```typescript
ach.map('x => x^2', vector)
ach.filter('x => x > 5', vector)
ach.reduce('a, b => a + b', vector, 0)
```

**Variables y lambdas:**
```typescript
ach.let('x', 10)
ach.get('x')
ach.lambda(['x', 'y'], 'x + y')
```

**Constantes:**
```typescript
ach.PI, ach.E, ach.PHI, ach.TAU
```

#### **Clase AchronymeValue**

**Extracción de valores:**
```typescript
await value.toNumber()      // → number
await value.toVector()      // → number[]
await value.toMatrix()      // → number[][]
await value.toComplex()     // → {re, im}
await value.value<T>()      // → T (auto-detect)
```

**Operaciones aritméticas:**
```typescript
value.add(other), value.sub(other), value.mul(other)
value.div(other), value.pow(other), value.neg()
```

**Comparaciones:**
```typescript
value.gt(other), value.lt(other), value.eq(other)
value.gte(other), value.lte(other), value.neq(other)
```

**Funciones matemáticas:**
```typescript
value.sin(), value.cos(), value.tan()
value.sqrt(), value.abs(), value.ln(), value.exp()
value.floor(), value.ceil(), value.round()
```

**DSP:**
```typescript
value.fft(), value.fft_mag(), value.ifft()
value.dft(), value.dft_mag(), value.dft_phase()
```

**Vector/Matrix:**
```typescript
value.dot(other), value.cross(other), value.norm()
value.transpose(), value.det(), value.inverse()
```

**Gestión:**
```typescript
value.dispose()
value.isDisposed()
value.getMetadata()
```

### **Manejo de Errores**

El SDK envuelve errores de C++ en clases específicas:

```typescript
try {
  const x = ach.number(5);
  const y = x.div(0);  // División por cero
} catch (e) {
  if (e instanceof AchronymeRuntimeError) {
    console.error('Error de runtime:', e.message);
  } else if (e instanceof AchronymeSyntaxError) {
    console.error('Error de sintaxis:', e.message);
  } else if (e instanceof AchronymeTypeError) {
    console.error('Error de tipo:', e.message);
  }
}
```

**Tipos de error disponibles:**
- `AchronymeError` - Base
- `AchronymeSyntaxError` - Error de sintaxis
- `AchronymeRuntimeError` - Error de ejecución
- `AchronymeTypeError` - Error de tipo
- `AchronymeDisposedError` - Operación en valor disposed
- `AchronymeNotInitializedError` - Módulo no inicializado
- `AchronymeArgumentError` - Argumentos inválidos

---

## 🧪 Ejecutar Tests

El proyecto incluye un test comprehensivo con **96 pruebas**:

```bash
# Ejecutar el test completo
node demo-achronyme.mjs
```

**Resultado esperado:**
```
╔═══════════════════════════════════════════════════════════════╗
║         ACHRONYME CORE - COMPREHENSIVE DEMO & TESTS          ║
╚═══════════════════════════════════════════════════════════════╝

...

╔═══════════════════════════════════════════════════════════════╗
║                        TEST SUMMARY                           ║
╚═══════════════════════════════════════════════════════════════╝

  Total tests run:    96
  Tests passed:       96
  Tests failed:       0
  Success rate:       100.0%

🎉 ALL TESTS PASSED! 🎉
```

---

## 📁 Estructura del Proyecto

```
achronyme-core/
├── wasm/                          # Código fuente C++
│   └── src/
│       ├── core/                  # Tipos de datos y funciones
│       │   ├── value.hpp/cpp      # Tipo Value (polimórfico)
│       │   ├── complex.hpp/cpp    # Números complejos
│       │   ├── vector.hpp/cpp     # Vectores
│       │   ├── matrix.hpp/cpp     # Matrices
│       │   ├── function.hpp/cpp   # Funciones lambda
│       │   ├── constants.hpp/cpp  # Constantes (PI, E, etc.)
│       │   ├── functions.hpp/cpp  # Registro de funciones
│       │   ├── functions_hof.cpp  # Higher-order functions
│       │   └── functions_dsp.cpp  # Funciones DSP
│       ├── parser/                # Parser y evaluador
│       │   ├── lexer.hpp/cpp      # Análisis léxico
│       │   ├── parser.hpp/cpp     # Análisis sintáctico
│       │   ├── ast.hpp            # Árbol sintáctico abstracto
│       │   ├── evaluator.hpp/cpp  # Evaluador de expresiones
│       │   └── environment.hpp    # Entorno de variables
│       └── bindings/              # Bindings para JavaScript
│           └── main.cpp           # Interfaz WASM
├── dist/                          # Módulo WASM compilado
│   ├── achronyme-core.mjs         # Módulo ES6
│   └── achronyme-core.wasm        # Binario WASM
├── demo-achronyme.mjs             # Test comprehensivo (96 tests)
└── README.md                      # Este archivo
```

---

## 📖 API Reference

### **Módulo Principal**

```javascript
import createAchronymeModule from './dist/achronyme-core.mjs';

const Module = await createAchronymeModule();

// Evaluar una expresión
Module.eval(expression: string): string

// Resetear el entorno (limpiar todas las variables)
Module.reset(): string

// Listar variables (próximamente)
Module.listVariables(): string
```

### **Funciones Matemáticas Básicas**

| Función | Descripción | Ejemplo |
|---------|-------------|---------|
| `sin(x)`, `cos(x)`, `tan(x)` | Trigonométricas | `sin(PI/2)` → 1 |
| `asin(x)`, `acos(x)`, `atan(x)` | Trigonométricas inversas | `asin(1)` → π/2 |
| `sinh(x)`, `cosh(x)`, `tanh(x)` | Hiperbólicas | `sinh(0)` → 0 |
| `exp(x)`, `log(x)`, `ln(x)` | Exponenciales | `exp(1)` → 2.718... |
| `log10(x)`, `log2(x)` | Logaritmos | `log10(100)` → 2 |
| `sqrt(x)`, `cbrt(x)`, `pow(x,y)` | Raíces y potencias | `sqrt(16)` → 4 |
| `abs(x)`, `sign(x)` | Valor absoluto y signo | `abs(-5)` → 5 |
| `floor(x)`, `ceil(x)`, `round(x)` | Redondeo | `floor(3.7)` → 3 |
| `min(...)`, `max(...)` | Mínimo/Máximo (variádicas) | `max(1,5,3)` → 5 |

### **Constantes**

| Constante | Valor | Descripción |
|-----------|-------|-------------|
| `PI` | 3.14159... | Número π |
| `E` | 2.71828... | Número e (base natural) |
| `PHI` | 1.61803... | Razón áurea |
| `TAU` | 6.28318... | τ = 2π |

### **Tipos Complejos**

| Operación | Ejemplo | Resultado |
|-----------|---------|-----------|
| Crear complejo | `3i` | 0+3i |
| Complejo completo | `2+3i` | 2+3i |
| Sumar complejos | `(2+3i) + (1+4i)` | 3+7i |
| Magnitud | `abs(3+4i)` | 5 |
| Fase | `arg(1+i)` | π/4 |
| Conjugado | `conj(2+3i)` | 2-3i |
| Vector | `[1, 2, 3]` | Vector de 3 elementos |
| Producto punto | `dot([1,2], [3,4])` | 11 |
| Norma | `norm([3,4])` | 5 |
| Matriz | `[[1,2], [3,4]]` | Matriz 2×2 |
| Transpuesta | `transpose([[1,2],[3,4]])` | [[1,3],[2,4]] |
| Determinante | `det([[1,2],[3,4]])` | -2 |

### **Programación Funcional**

| Función | Sintaxis | Descripción | Ejemplo |
|---------|----------|-------------|---------|
| `let` | `let var = expr` | Define variable | `let x = 10` |
| Lambda | `params => expr` | Función anónima | `x => x^2` |
| Multi-param | `(a,b) => expr` | Múltiples parámetros | `(a,b) => a+b` |
| `map` | `map(f, v1, ...)` | Aplicar función a cada elemento | `map(x => x*2, [1,2,3])` → [2,4,6] |
| `filter` | `filter(pred, vec)` | Filtrar elementos | `filter(x => x>0, [-1,1,2])` → [1,2] |
| `reduce` | `reduce(f, init, vec)` | Reducir a un valor | `reduce((a,b) => a+b, 0, [1,2,3])` → 6 |
| `pipe` | `pipe(val, f1, f2, ...)` | Composición de funciones | `pipe([1,2], f, g, h)` |

### **DSP - Procesamiento de Señales**

| Función | Descripción | Complejidad | Ejemplo |
|---------|-------------|-------------|---------|
| `dft(signal)` | DFT clásica | O(N²) | `dft([1,0,0,0])` |
| `dft_mag(signal)` | Magnitud DFT | O(N²) | `dft_mag([1,1,1,1])` |
| `dft_phase(signal)` | Fase DFT | O(N²) | `dft_phase([1,0,0,0])` |
| `fft(signal)` | FFT Cooley-Tukey | O(N log N) | `fft([1,2,3,4])` |
| `fft_mag(signal)` | Magnitud FFT | O(N log N) | `fft_mag([1,1,1,1,1,1,1,1])` |
| `ifft(spectrum)` | FFT inversa | O(N log N) | `ifft(fft([1,2,3,4]))` |
| `conv(s1, s2)` | Convolución directa | O(N×M) | `conv([1,2,3], [1,1])` |
| `conv_fft(s1, s2)` | Convolución FFT | O((N+M) log(N+M)) | `conv_fft([1,2,3,4,5], [1,2,1])` |
| `hanning(N)` | Ventana de Hann | O(N) | `hanning(8)` |
| `hamming(N)` | Ventana de Hamming | O(N) | `hamming(8)` |
| `blackman(N)` | Ventana de Blackman | O(N) | `blackman(8)` |

---

## 💡 Ejemplos Avanzados

### **Ejemplo 1: Biblioteca DSP Personalizada**

```javascript
const Module = await createAchronymeModule();
Module.reset();

// Crear funciones reutilizables
Module.eval('let power = x => x ^ 2');
Module.eval('let sum_vec = v => reduce((a,b) => a+b, 0, v)');
Module.eval('let power_all = v => map(power, v)');
Module.eval('let rms = v => sqrt(sum_vec(power_all(v)) / 8)');

// Usar la biblioteca
const signal = '[1, 2, 3, 4, 5, 6, 7, 8]';
const rmsValue = Module.eval(`rms(${signal})`);
console.log('RMS:', rmsValue);  // → "5.04975..."
```

### **Ejemplo 2: Análisis Espectral con Ventanas**

```javascript
const Module = await createAchronymeModule();
Module.reset();

// Pipeline de análisis
Module.eval(`
  let analyze = sig => fft_mag(
    map((s,w) => s*w, sig, hanning(8))
  )
`);

// Analizar señal
const spectrum = Module.eval('analyze([1,2,3,4,5,6,7,8])');
console.log('Spectrum:', spectrum);
```

### **Ejemplo 3: Filtrado FIR con Convolución**

```javascript
const Module = await createAchronymeModule();
Module.reset();

// Definir filtro paso-bajo
Module.eval('let lowpass = [0.25, 0.5, 0.25]');
Module.eval('let filter_signal = sig => conv(sig, lowpass)');

// Aplicar filtro
const filtered = Module.eval('filter_signal([1,2,3,4,5,6,7,8])');
console.log('Filtered:', filtered);
```

### **Ejemplo 4: Pipeline Complejo con Pipe**

```javascript
const Module = await createAchronymeModule();
Module.reset();

// Crear pipeline de procesamiento
const result = Module.eval(`
  pipe(
    [1, 2, 3, 4, 5, 6, 7, 8],
    v => map((s,w) => s*w, v, hanning(8)),
    v => fft_mag(v),
    v => map(m => m^2, v),
    v => reduce((a,b) => a+b, 0, v)
  )
`);

console.log('Total spectral power:', result);
```

### **Ejemplo 5: Comparación de Ventanas**

```javascript
const Module = await createAchronymeModule();
Module.reset();

const signal = '[1,2,3,4,5,6,7,8]';

// Definir funciones de análisis
Module.eval(`
  let analyze_hann = s => fft_mag(map((sig,w) => sig*w, s, hanning(8)))
`);
Module.eval(`
  let analyze_hamm = s => fft_mag(map((sig,w) => sig*w, s, hamming(8)))
`);
Module.eval(`
  let analyze_black = s => fft_mag(map((sig,w) => sig*w, s, blackman(8)))
`);

// Comparar resultados
console.log('Hanning:', Module.eval(`analyze_hann(${signal})`));
console.log('Hamming:', Module.eval(`analyze_hamm(${signal})`));
console.log('Blackman:', Module.eval(`analyze_black(${signal})`));
```

---

## 🐛 Solución de Problemas

### **Error: "emcc: command not found"**

**Causa:** Emscripten no está en el PATH.

**Solución:**

Windows:
```bash
cd C:\ruta\a\emsdk
emsdk_env.bat
```

Linux/macOS:
```bash
cd /ruta/a/emsdk
source ./emsdk_env.sh
```

### **Error: Module compilation failed**

**Causa:** No se incluyeron todos los archivos .cpp en la compilación.

**Solución:** Asegúrate de que el comando `emcc` incluya TODOS los archivos listados en la sección de compilación.

### **Error: "Cannot find module './dist/achronyme-core.mjs'"**

**Causa:** El módulo no ha sido compilado.

**Solución:**
```bash
cd wasm
# Ejecutar el comando de compilación completo
cd ..
```

### **Error: Variables no persisten**

**Causa:** Llamar a `Module.reset()` borra todas las variables.

**Solución:** Solo llama a `reset()` cuando quieras limpiar el entorno intencionalmente.

---

## 📊 Rendimiento y Benchmarks

### **Benchmarks Reales: Achronyme vs math.js**

Hemos realizado benchmarks exhaustivos comparando Achronyme con math.js (la librería de matemáticas más popular de JavaScript) en escenarios reales de producción.

#### **🏆 Pipeline DSP Avanzado (Caso de Uso Real)**

**Escenario**: Procesamiento multi-etapa de señales (generación → ventana Hanning → FFT → filtro band-pass → estadísticas)

**Resultados con 32,768 samples:**

```
🔷 ACHRONYME:          131.80 ms
🟦 MATH.JS:            705.40 ms

Winner: Achronyme - 5.35x más rápido (435% de ganancia)
```

**Desglose detallado:**

| Operación | Achronyme | math.js | Ventaja |
|-----------|-----------|---------|---------|
| FFT | 18.10 ms | 686.40 ms | **37.9x más rápido** 🔥 |
| Estadísticas (sum, mean, max, std) | 0.90 ms | 10.10 ms | **11.2x más rápido** |
| Windowing | 21.10 ms | 5.60 ms | math.js 3.8x más rápido |
| Filtering | 22.70 ms | 3.20 ms | math.js 7.1x más rápido |

#### **📊 Funciones Estadísticas Nativas (100K elementos)**

```
🔷 ACHRONYME:          0.80 ms
🟦 MATH.JS:           30.50 ms

Winner: Achronyme - 38.12x más rápido
```

#### **📡 FFT Pura (65,536 samples)**

```
🔷 ACHRONYME:         85.60 ms
🟦 MATH.JS:         1519.60 ms

Winner: Achronyme - 17.75x más rápido
```

### **🎯 ¿Cuándo usar Achronyme vs math.js?**

#### **✅ Usa Achronyme cuando:**

- **Procesamiento DSP**: FFT, convolución, análisis espectral, filtros digitales
- **Pipelines complejos**: Múltiples operaciones encadenadas sobre grandes datasets
- **Cómputo científico**: Álgebra lineal, análisis numérico, simulaciones
- **Producción**: Aplicaciones que requieren máximo rendimiento (audio, video, sensores)
- **Datasets grandes**: 10K+ elementos con operaciones complejas

**Por qué Achronyme domina aquí:**
- Algoritmos optimizados en C++ (FFT Cooley-Tukey, estadísticas nativas)
- Todo el pipeline se ejecuta en WASM sin cruzar a JavaScript
- Compilador optimizado (Emscripten -O3)
- Zero-copy operations para resultados intermedios

#### **⚠️ Usa math.js cuando:**

- **Operaciones atómicas simples**: Una suma o multiplicación aislada
- **Scripts rápidos**: Prototipado, pruebas one-off
- **Datasets pequeños**: <1K elementos con operaciones básicas
- **Integración con ecosistema JS**: Cuando necesitas compatibilidad total con arrays nativos

**Por qué math.js es más rápido aquí:**
- Las operaciones simples en arrays nativos de JavaScript están altamente optimizadas por V8
- No hay overhead del boundary WASM↔JavaScript
- Para una sola operación, el costo de transferir datos a/desde WASM no vale la pena

### **🔬 Análisis Técnico: El Boundary Crossing**

Achronyme está construido sobre WebAssembly (C++ compilado), lo que significa que hay un "puente" entre JavaScript y WASM.

**Cuando haces `ach.vadd(v1, v2)` con 100K elementos:**

```
1. JavaScript → WASM call              ~0.1 ms
2. C++ ejecuta suma optimizada         ~5 ms
3. Extraer resultado: WASM → JS        ~70 ms  ← 93% del tiempo
   (copiar 800KB de memoria WASM a JS array)
────────────────────────────────────────────────
   TOTAL:                               ~75 ms
```

**Cuando haces `math.add(arr1, arr2)` con 100K elementos:**

```
1. Loop directo en V8 sobre arrays nativos  ~8 ms
────────────────────────────────────────────────
   TOTAL:                                   ~8 ms
```

**Entonces, ¿cómo Achronyme gana en pipelines?**

Cuando encadenas operaciones, Achronyme **mantiene todo en WASM**:

```javascript
// ❌ Operación aislada (math.js más rápido)
const result = ach.vadd(v1, v2);  // 75ms (crossing overhead)

// ✅ Pipeline (Achronyme domina)
const result = signal
  .applyWindow()    // Se queda en WASM
  .fft()            // Se queda en WASM (37x más rápido)
  .filter()         // Se queda en WASM
  .statistics();    // Solo al final cruza el puente (11x más rápido)

// Total: 131ms vs 705ms en math.js
```

### **🎯 Conclusión**

**Achronyme no pretende reemplazar math.js en todos los casos**. Cada herramienta tiene su lugar:

- **math.js**: Excelente para JavaScript puro, scripts rápidos, operaciones simples
- **Achronyme**: Diseñado para cómputo pesado, DSP, ciencia, producción

Si tu aplicación hace análisis espectral, procesamiento de audio, simulaciones científicas o cualquier pipeline complejo sobre datos grandes, **Achronyme te dará 5-40x mejor rendimiento**.

Si solo necesitas sumar dos arrays ocasionalmente, math.js es perfectamente válido.

### **📈 Tabla de Referencia Rápida**

| Operación | Tamaño | Achronyme | math.js | Ganador |
|-----------|--------|-----------|---------|---------|
| **Pipeline DSP completo** | 32K | 131.80 ms | 705.40 ms | **Achronyme 5.35x** |
| **FFT** | 64K | 85.60 ms | 1519.60 ms | **Achronyme 17.75x** |
| **Estadísticas nativas** | 100K | 0.80 ms | 30.50 ms | **Achronyme 38.12x** |
| **Operación vectorial simple** | 100K | 74.60 ms | 8.20 ms | **math.js 9.1x** |

*Benchmarks ejecutados en Chrome 120+, CPU moderna (2024)*

---

## 🗺️ Roadmap

### ✅ **Completado**

- [x] **Phase 1-2**: Operaciones matemáticas básicas
- [x] **Phase 3**: Tipos complejos (Complex, Vector, Matrix)
- [x] **Phase 4A**: Variables, lambdas, closures, HOF
- [x] **Phase 4B**: DSP (DFT, FFT, IFFT, convolución, ventanas)

### 📅 **Próximas Fases**

- [ ] **Phase 5**: Álgebra lineal avanzada (eigenvalues, SVD, QR)
- [ ] **Phase 6**: Métodos numéricos (integración, derivación, raíces)
- [ ] **Phase 7**: Optimización (Simplex, gradiente descendente)
- [ ] **Phase 8**: Estadística y probabilidad

---

## 📝 Licencia

[Especificar licencia del proyecto]

---

## 👥 Autores

Desarrollado como parte del proyecto Achronyme Core.

**Contacto:** contacto@eddndev.com

---

## 🔗 Enlaces Útiles

- [Emscripten Documentation](https://emscripten.org/docs/)
- [WebAssembly MDN](https://developer.mozilla.org/en-US/docs/WebAssembly)
- [C++ Reference](https://en.cppreference.com/)
- [DSP Guide](https://www.dspguide.com/)

---

**¡Disfruta usando Achronyme Core!** 🚀
