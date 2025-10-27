# Achronyme Core

**Motor de cálculo matemático de alto rendimiento con WebAssembly**

[![npm version](https://img.shields.io/npm/v/@achronyme/core)](https://www.npmjs.com/package/@achronyme/core)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Achronyme Core es un motor de computación matemática compilado a WebAssembly que combina rendimiento de C++ con la accesibilidad de JavaScript/TypeScript.

```typescript
import { Achronyme } from '@achronyme/core';

const ach = new Achronyme();
await ach.init();

// DSP en tiempo real
const signal = ach.vector(Array.from({length: 1024}, (_, i) =>
  Math.sin(2 * Math.PI * 50 * i / 1000)
));
const spectrum = ach.fft_mag(signal);

console.log('Dominant frequency:', await spectrum.toVector());
```

---

## ✨ Características

- **🚀 Alto rendimiento**: 5-40x más rápido que math.js en operaciones complejas
- **🔢 Tipos avanzados**: Number, Complex, Vector, Matrix, Function
- **📡 DSP nativo**: FFT Cooley-Tukey, convolución, ventanas, filtros
- **λ Programación funcional**: Lambdas, closures, map/filter/reduce
- **📐 Álgebra lineal**: Operaciones matriciales, determinante, inversa
- **TypeScript SDK**: API tipo-segura con gestión de memoria explícita
- **🌐 Universal**: Web, Node.js, y compilable a binarios nativos

---

## 📦 Instalación

```bash
npm install @achronyme/core
```

---

## 🚀 Inicio Rápido

### Uso Básico con SDK TypeScript

```typescript
import { Achronyme } from '@achronyme/core';

const ach = new Achronyme();
await ach.init();

// Operaciones matemáticas
const result = ach.number(5).mul(2).add(10).div(4);
console.log(await result.toNumber()); // → 5

// Vectores y estadísticas
const data = ach.vector([1, 2, 3, 4, 5]);
const mean = ach.mean(data);
const std = ach.std(data);

console.log('Mean:', await mean.toNumber());
console.log('Std:', await std.toNumber());

// Limpieza de memoria
result.dispose();
data.dispose();
mean.dispose();
std.dispose();
```

### Procesamiento de Señales (DSP)

```typescript
// Generar señal con ruido
const signalData = Array.from({length: 1024}, (_, i) =>
  Math.sin(2 * Math.PI * 50 * i / 1000) +
  0.5 * Math.sin(2 * Math.PI * 120 * i / 1000)
);

const signal = ach.vector(signalData);
const window = ach.hanning(1024);
const windowed = ach.vmul(signal, window);
const spectrum = ach.fft_mag(windowed);

console.log('Spectrum:', await spectrum.toVector());

// Cleanup
signal.dispose();
window.dispose();
windowed.dispose();
spectrum.dispose();
```

### Programación Funcional

```typescript
const numbers = ach.vector([1, 2, 3, 4, 5, 6]);

// Map, filter, reduce
const squared = ach.map('x => x^2', numbers);
const evens = ach.filter('x => x % 2 == 0', numbers);
const sum = ach.reduce('(a,b) => a+b', numbers, ach.number(0));

console.log(await squared.toVector()); // → [1, 4, 9, 16, 25, 36]
console.log(await evens.toVector());   // → [2, 4, 6]
console.log(await sum.toNumber());     // → 21
```

---

## 📊 Rendimiento

Benchmarks reales ejecutados en Chrome 120+ con datasets de producción:

| Operación | Achronyme | math.js | Ventaja |
|-----------|-----------|---------|---------|
| **Pipeline DSP** (32K samples) | 131ms | 705ms | **5.35x más rápido** |
| **FFT** (64K samples) | 85.6ms | 1519ms | **17.75x más rápido** |
| **Estadísticas** (100K elementos) | 0.8ms | 30.5ms | **38.12x más rápido** |

**Por qué es más rápido:**
- C++ compilado a WASM con optimizaciones `-O3`
- Algoritmos nativos (FFT Cooley-Tukey)
- Zero-copy para operaciones encadenadas
- Mantiene datos en WASM durante pipelines

**Cuándo usar Achronyme:**
- ✅ DSP, análisis espectral, procesamiento de señales
- ✅ Pipelines complejos con múltiples operaciones
- ✅ Datasets grandes (10K+ elementos)
- ✅ Aplicaciones de producción que requieren rendimiento

---

## 📚 Documentación

### Guías Completas

- **[Especificación del Lenguaje SOC](./docs/language-spec.md)** - Gramática, tipos, operadores, sintaxis
- **[Guía del SDK TypeScript](./docs/sdk-guide.md)** - API completa, gestión de memoria, ejemplos
- **[Roadmap del Proyecto](./docs/roadmap.md)** - Futuro de Achronyme y ecosistema
- **[Comparación con Wolfram](./docs/wolfram-comparison.md)** - Análisis competitivo realista

### Ejemplos

El proyecto incluye 4 ejemplos completos:

```bash
node examples/basic-usage.mjs
node examples/dsp-example.mjs
node examples/functional-programming.mjs
node examples/advanced-dsp-pipeline.mjs
```

### Tests

```bash
# Test comprehensivo (96 pruebas)
node demo-achronyme.mjs

# Test del SDK
node test-sdk.mjs
```

---

## 🛠️ Compilación desde el Código Fuente

### Requisitos Previos

- **Emscripten SDK** (para compilar C++ a WASM)
- **Node.js 18+**
- **TypeScript** (para compilar el SDK)

### Instalar Emscripten

**Windows:**
```bash
git clone https://github.com/emscripten-core/emsdk.git
cd emsdk
emsdk install latest
emsdk activate latest
emsdk_env.bat
```

**Linux/macOS:**
```bash
git clone https://github.com/emscripten-core/emsdk.git
cd emsdk
./emsdk install latest
./emsdk activate latest
source ./emsdk_env.sh
```

### Compilar

```bash
# Compilar WASM
npm run build:wasm

# Compilar TypeScript
npm run build:js

# Compilar todo
npm run build
```

---

## 🎯 Características del Lenguaje SOC

### Tipos de Datos

- **Number**: Punto flotante 64-bit (`42`, `3.14`, `1e6`)
- **Complex**: Números complejos (`3i`, `2+3i`)
- **Vector**: Arrays matemáticos (`[1, 2, 3]`)
- **Matrix**: Matrices 2D (`[[1,2],[3,4]]`)
- **Function**: Lambdas (`x => x^2`)

### Operaciones DSP

```javascript
fft([1,2,3,4,5,6,7,8])        // FFT Cooley-Tukey O(N log N)
fft_mag(signal)               // Magnitud del espectro
ifft(spectrum)                // FFT inversa
dft(signal)                   // DFT clásica O(N²)
conv(s1, s2)                  // Convolución directa
conv_fft(s1, s2)              // Convolución rápida con FFT
hanning(N)                    // Ventana de Hann
hamming(N)                    // Ventana de Hamming
blackman(N)                   // Ventana de Blackman
```

### Higher-Order Functions

```javascript
map(x => x^2, [1,2,3,4])                    // → [1, 4, 9, 16]
filter(x => x > 5, [1,5,10,15])             // → [10, 15]
reduce((a,b) => a+b, 0, [1,2,3,4])          // → 10
pipe([1,2,3,4], f, g, h)                    // Composición
```

Ver [Especificación completa del lenguaje](./docs/language-spec.md) para sintaxis detallada.

---

## 🗺️ Roadmap

### ✅ Completado (v0.3)

- Parser y evaluador de expresiones
- Tipos complejos (Complex, Vector, Matrix)
- DSP básico (FFT, convolución, ventanas)
- Lambdas y higher-order functions
- SDK TypeScript tipo-seguro

### 🚧 En Desarrollo (v0.4-0.6)

- **Phase 5**: Álgebra lineal avanzada (LU, QR, SVD, eigenvalues)
- **Phase 6**: Cálculo numérico (derivación, integración, EDOs)
- **Phase 7**: Optimización (gradiente, Newton, simplex)

### 🔮 Futuro (v0.7+)

- **Phase 8-12**: Estadística, EDPs, cálculo simbólico, DSP avanzado, ML básico
- **@achronyme/language**: Procesamiento de lenguaje natural matemático
- **@achronyme/plot**: Visualización matemática
- **@achronyme/cas**: Computer Algebra System

Ver [Roadmap completo](./docs/roadmap.md) para detalles.

---

## 🤝 Contribuir

Achronyme es open-source y buscamos colaboradores en:
- **C++ developers**: Algoritmos numéricos core
- **TypeScript developers**: SDK, testing, ejemplos
- **Math experts**: Validación de algoritmos
- **DSP engineers**: Optimización de FFT, filtros
- **Documentation**: Tutoriales, traducciones

**Repositorio**: https://github.com/eddndev/achronyme-core
**Discusiones**: https://github.com/eddndev/achronyme-core/discussions

---

## 📝 Licencia

MIT License - Copyright (c) 2025 Eduardo Alonso

Ver [LICENSE](./LICENSE) para detalles completos.

---

## 🔗 Enlaces

- **[Documentación](./docs/)** - Guías completas
- **[Ejemplos](./examples/)** - Código de ejemplo
- **[GitHub](https://github.com/eddndev/achronyme-core)** - Repositorio
- **[npm](https://www.npmjs.com/package/@achronyme/core)** - Paquete
- **[Website](https://achronyme.com)** - Sitio oficial

---

**Versión actual**: 0.3.0-beta-6

**¿Preguntas?** Abre un issue en GitHub o únete a las discusiones.

---

<p align="center">
  <strong>Construido con ❤️ por la comunidad de Achronyme</strong>
  <br>
  Democratizando el acceso a cálculo matemático de clase mundial
</p>
