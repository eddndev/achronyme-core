# Achronyme Core

**Motor de cálculo matemático de alto rendimiento con WebAssembly**

[![npm version](https://img.shields.io/npm/v/@achronyme/core)](https://www.npmjs.com/package/@achronyme/core)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Achronyme Core es un motor de computación matemática compilado a WebAssembly que combina rendimiento de C++ con la accesibilidad de JavaScript/TypeScript. Presenta un **SDK TypeScript v2.0** moderno y un potente **lenguaje de expresiones SOC** para cálculos eficientes.

```typescript
import { Achronyme } from '@achronyme/core';

const ach = new Achronyme();
await ach.init();

// DSP en tiempo real con gestión de memoria automática
await ach.use(async () => {
  const signal = ach.vector(Array.from({length: 1024}, (_, i) =>
    Math.sin(2 * Math.PI * 50 * i / 1000)
  ));
  const spectrum = ach.dsp.fftMag(signal);

  console.log('Dominant frequency (first 5 values):', spectrum.data.slice(0, 5));
  // signal y spectrum se limpian automáticamente al salir de ach.use()
});
```

---

## ⚡ Performance

**Benchmarks reales** (100K elementos, promedio de 100 iteraciones):

| Operación | Achronyme | math.js | Speedup |
|-----------|-----------|---------|---------|
| **Operaciones vectorizadas** | 447ms | 622ms | **🏆 1.39x más rápido** |
| **FFT (4K samples)** | 26ms | 2032ms | **🚀 78x más rápido** |
| **Vector operations** | 3.7ms | 9.7ms | **⚡ 2.6x más rápido** |

*Nota: Achronyme usa WASM compilado con -O3 y sistema de handles zero-copy. math.js es JavaScript puro. Benchmarks ejecutados en Chrome V8.*

**Fast Path Usage**: El SDK v2.0 maximiza el uso de rutas optimizadas en WASM, incluyendo **vistas zero-copy** para acceso instantáneo a los datos.

---

## ✨ Características

- **🚀 Alto rendimiento**: **4.45x más rápido que JS Nativo** en operaciones matemáticas vectorizadas, **202.01x en FFT**.
- **🧠 Gestión de Memoria por Sesiones**: El patrón `ach.use()` garantiza la limpieza automática de recursos WASM, previniendo fugas de memoria.
- **💾 Vistas Zero-Copy**: Acceso instantáneo a los datos en memoria WASM (`Float64Array`) sin costosas copias.
- **🔢 Tipos avanzados**: Number, Complex, Vector, Matrix, Function.
- **📡 DSP nativo**: FFT Cooley-Tukey, convolución, ventanas, filtros.
- **λ Programación funcional**: Lambdas, closures, map/filter/reduce.
- **📐 Álgebra lineal**: Operaciones matriciales, determinante, inversa, y descomposiciones avanzadas (LU, QR, SVD, Cholesky, Eigenvalues).
- **📝 Lenguaje de Expresiones SOC**: Un potente lenguaje string-based para ejecutar pipelines complejos en una sola llamada a WASM.
- **TypeScript SDK v2.0**: API tipo-segura y modular.
- **🌐 Universal**: Web, Node.js, y compilable a binarios nativos.

---

## 📦 Instalación

```bash
npm install @achronyme/core
```

---

## 🚀 Inicio Rápido

### Uso Básico con SDK TypeScript (v2.0)

El patrón recomendado es usar `ach.use()` para la gestión automática de memoria.

```typescript
import { Achronyme } from '@achronyme/core';

const ach = new Achronyme();
await ach.init();

await ach.use(async () => {
  // Operaciones matemáticas
  const x = ach.scalar(5);
  const result = ach.math.add(ach.math.mul(x, 2), 10); // (5 * 2) + 10 = 20
  console.log('Resultado:', x.value); // 20

  // Vectores y estadísticas
  const data = ach.vector([1, 2, 3, 4, 5]);
  const mean = ach.stats.mean(data);
  const std = ach.stats.std(data);

  console.log('Mean:', mean);
  console.log('Std:', std);

  // x, result, data, mean, std se limpian automáticamente al salir de ach.use()
});
```

### Procesamiento de Señales (DSP)

```typescript
import { Achronyme } from '@achronyme/core';

const ach = new Achronyme();
await ach.init();

await ach.use(async () => {
  // Generar señal con ruido
  const signalData = Array.from({length: 1024}, (_, i) =>
    Math.sin(2 * Math.PI * 50 * i / 1000) +
    0.5 * Math.sin(2 * Math.PI * 120 * i / 1000)
  );

  const signal = ach.vector(signalData);
  const window = ach.dsp.hanning(1024);
  const windowed = ach.vecOps.vmul(signal, window); // Multiplicación elemento a elemento
  const spectrum = ach.dsp.fftMag(windowed);

  console.log('Spectrum (first 10 values):', spectrum.data.slice(0, 10));

  // signal, window, windowed, spectrum se limpian automáticamente
});
```

### Programación Funcional con el Lenguaje SOC

```typescript
import { Achronyme } from '@achronyme/core';

const ach = new Achronyme();
await ach.init();

await ach.use(async () => {
  const numbers = ach.vector([1, 2, 3, 4, 5, 6]);

  // Map, filter, reduce usando el lenguaje SOC
  const squared = ach.eval("map(x => x^2, [1,2,3,4,5,6])");
  const evens = ach.eval("filter(x => x % 2 == 0, [1,2,3,4,5,6])");
  const sum = ach.eval("reduce((a,b) => a+b, 0, [1,2,3,4,5,6])");

  console.log('Squared:', squared); // → "[1, 4, 9, 16, 25, 36]"
  console.log('Evens:', evens);     // → "[2, 4, 6]"
  console.log('Sum:', sum);         // → "21" 
});
```

---

## 📊 Rendimiento

**Benchmarks de producción** - Ejecutados en Chrome V8 con datasets reales:

### Operaciones Matemáticas Vectorizadas
*(10.000.000 elementos × 5 iteraciones, 3 operaciones: sin, cos, exp)*

| Librería | Tiempo Total | Speedup vs JS Native | Resultado |
|----------|--------------|----------------------|-----------|
| **Achronyme (WASM)** | 2239.20ms | **4.45x más rápido** | 🏆 **Ganador** |
| JS Nativo (V8) | 9971.90ms | 1.00x (baseline) | Referencia |

### DSP y Operaciones Complejas

| Operación | Achronyme | math.js | Speedup |
|-----------|-----------|---------|---------|
| **FFT (8K samples)** | 7.40ms | 1494.90ms | **🚀 202.01x más rápido** |
| **Operaciones Vectoriales (200K)** | 550.10ms | 1649.30ms | **3.00x más rápido** |
| **Pipeline DSP Completo (16K)** | 5.10ms | 313.90ms | **61.55x más rápido** |

### Fast Path Efficiency
- **99.9%** de operaciones usan path optimizado (zero-copy)
- **0.1%** fallback a parser (casos edge)

**Por qué Achronyme es más rápido que math.js:**
- ✅ C++ compilado a WASM con `-O3` (vs JavaScript interpretado)
- ✅ Algoritmos nativos especializados (FFT Cooley-Tukey optimizado)
- ✅ Sistema de **handles zero-copy** (sin serialización JS ↔ WASM)
- ✅ Mantiene datos en memoria WASM durante pipelines

**Por qué Achronyme compite con JS nativo:**
- ⚡ Overhead JS-WASM minimalizado, especialmente en operaciones complejas.
- ⚡ Operaciones vectorizadas sin abstracciones.
- ⚡ Sin overhead de librerías (math.js tiene múltiples capas).
- **Nota**: Para operaciones vectoriales muy simples, JavaScript nativo (V8) puede ser marginalmente más rápido, pero Achronyme supera a JS nativo en operaciones matemáticas complejas y pipelines DSP.

---

## 📚 Documentación

### Guías Completas

- **[Guía del SDK TypeScript v2.0](./docs/sdk/README.md)** - Visión general del SDK, características clave y ejemplos.
- **[Referencia de API del SDK](./docs/sdk/api-reference.md)** - Documentación detallada de todas las clases, métodos y funciones del SDK.
- **[Gestión de Memoria del SDK](./docs/sdk/memory-management.md)** - Patrones y mejores prácticas para el manejo de memoria en el SDK.
- **[Ejemplos del SDK](./docs/sdk/examples.md)** - Casos de uso prácticos y código de ejemplo para el SDK.
- **[Tipos de Datos del SDK](./docs/sdk/types.md)** - Definiciones de tipos TypeScript y estructuras de datos del SDK.
- **[Especificación del Lenguaje SOC](./docs/language-spec.md)** - Gramática, tipos, operadores y funciones del lenguaje de expresiones SOC.
- **[Guía de Rendimiento](./docs/sdk/optimization-functions.md)** - Estrategias para optimizar el rendimiento y minimizar el overhead JS-WASM.
- **[Roadmap del Proyecto](./docs/roadmap.md)** - Futuro de Achronyme y ecosistema.
- **[Comparación con Wolfram](./docs/wolfram-comparison.md)** - Análisis competitivo realista.

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

El lenguaje SOC permite ejecutar expresiones matemáticas complejas directamente en el motor WASM.

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

### ✅ Completado (v0.4)

- **Phase 5**: Álgebra lineal avanzada (LU, QR, SVD, eigenvalues)
- Parser y evaluador de expresiones
- Tipos complejos (Complex, Vector, Matrix)
- DSP básico (FFT, convolución, ventanas)
- Lambdas y higher-order functions
- SDK TypeScript tipo-seguro

### 🚧 En Desarrollo (v0.5-0.6)

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

**Repositorio**: https://github.com/achronyme/achronyme-core
**Discusiones**: https://github.com/achronyme/achronyme-core/discussions

---

## 📝 Licencia

MIT License - Copyright (c) 2025 Eduardo Alonso

Ver [LICENSE](./LICENSE) para detalles completos.

---

## 🔗 Enlaces

- **[Documentación](./docs/)** - Guías completas
- **[Ejemplos](./examples/)** - Código de ejemplo
- **[GitHub](https://github.com/achronyme/achronyme-core)** - Repositorio
- **[npm](https://www.npmjs.com/package/@achronyme/core)** - Paquete
Website: https://achrony.me

---

**Versión actual**: 0.4.0

**Reproduce los benchmarks tú mismo:**
```bash
cd test-npm-install/demo
npm install
npm run dev
# Abre http://localhost:5173 y ejecuta "Extreme Stress Test"
```

**¿Preguntas?** Abre un issue en GitHub o únete a las discusiones.

---

<p align="center">
  <strong>Construido con ❤️ por la comunidad de Achronyme</strong>
  <br>
  Democratizando el acceso a cálculo matemático de clase mundial
</p>
