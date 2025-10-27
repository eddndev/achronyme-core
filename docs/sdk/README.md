# Achronyme SDK Documentation

Documentación completa del SDK TypeScript de Achronyme para análisis matemático y procesamiento de señales.

## Tabla de Contenidos

- [Tipos de Datos](./types.md) - Tipos TypeScript, interfaces y estructuras de datos
- [Referencia de API](./api-reference.md) - Documentación completa de todas las funciones
- [Funciones de Optimización](./optimization-functions.md) - Funciones de alto rendimiento para reducir overhead JS-WASM
- [Ejemplos Prácticos](./examples.md) - Casos de uso y ejemplos completos
- [Gestión de Memoria](./memory-management.md) - Buenas prácticas y patrones de memoria

## Inicio Rápido

### Instalación

```bash
npm install @achronyme/core
```

### Uso Básico

```typescript
import { Achronyme } from '@achronyme/core';

// Inicializar
const ach = new Achronyme();
await ach.init();

// Crear valores
const x = ach.number(42);
const v = ach.vector([1, 2, 3, 4, 5]);

// Operaciones
const result = x.mul(2);
console.log(await result.toNumber()); // 84

// Limpieza
x.dispose();
v.dispose();
result.dispose();
```

## Características Principales

### 🧮 Matemáticas

- Operaciones aritméticas básicas (+, -, *, /, ^)
- Funciones trigonométricas (sin, cos, tan, asin, acos, atan, atan2)
- Funciones hiperbólicas (sinh, cosh, tanh)
- Funciones exponenciales y logarítmicas (exp, ln, log, log2, log10)
- Funciones de redondeo (floor, ceil, round, trunc)
- Valor absoluto, signo, potencias

### 📊 Álgebra Lineal

- Vectores con operaciones elemento a elemento
- Matrices con multiplicación, transposición, inversión
- Producto punto y producto cruz
- Determinante y traza
- Operaciones vectoriales optimizadas (vadd, vsub, vmul, vdiv, vscale)

### 📡 Procesamiento Digital de Señales (DSP)

- FFT (Fast Fourier Transform) - O(N log N)
- DFT (Discrete Fourier Transform) - O(N²)
- IFFT (Inverse Fast Fourier Transform)
- Espectros de magnitud y fase
- Convolución (directa y con FFT)
- Funciones ventana (Hanning, Hamming, Blackman)
- **Funciones optimizadas de alto rendimiento** (fft_spectrum, linspace, fftshift)

### 🔢 Estadísticas

- Suma, media, desviación estándar
- Mínimo y máximo (variádico y vectorial)

### λ Programación Funcional

- Lambdas y funciones de orden superior
- map, filter, reduce
- Composición de funciones (compose, pipe)

### 💾 Gestión de Memoria

- Sistema de dispose explícito
- Estadísticas de memoria en tiempo real
- Reset completo del entorno
- Variables persistentes entre evaluaciones

## Arquitectura

```
┌─────────────────────────────────────────────┐
│         TypeScript SDK (High Level)         │
│  ┌─────────────┐      ┌─────────────────┐  │
│  │  Achronyme  │──────│ AchronymeValue  │  │
│  └─────────────┘      └─────────────────┘  │
└──────────────────┬──────────────────────────┘
                   │ eval()
┌──────────────────▼──────────────────────────┐
│           C++ WASM Core (Low Level)         │
│  ┌──────────┐  ┌────────┐  ┌─────────────┐ │
│  │  Parser  │──│ Value  │──│  Functions  │ │
│  └──────────┘  └────────┘  └─────────────┘ │
└─────────────────────────────────────────────┘
```

## Conceptos Clave

### Values vs Primitivos

El SDK trabaja con dos tipos de valores:

1. **TypeScript Primitivos** - `number`, `number[]`, `number[][]`
2. **AchronymeValue** - Proxy a valores en el entorno C++

```typescript
// Primitivo → AchronymeValue
const x = ach.number(42);              // AchronymeValue
const v = ach.vector([1, 2, 3]);       // AchronymeValue

// AchronymeValue → Primitivo
const num = await x.toNumber();        // number
const arr = await v.toVector();        // number[]
```

### Variables Persistentes

Los valores se mantienen en el entorno C++ entre operaciones:

```typescript
const x = ach.let('myVar', 42);        // Crear variable 'myVar'
const y = ach.get('myVar');            // Obtener referencia
const z = y.mul(2);                    // Usar la variable
```

### Fluent API

Las operaciones son encadenables:

```typescript
const result = ach.vector([1, 2, 3, 4, 5])
  .mul(2)           // [2, 4, 6, 8, 10]
  .add(10)          // [12, 14, 16, 18, 20]
  .map('x => x^2'); // [144, 196, 256, 324, 400]
```

## Rendimiento

### Funciones Optimizadas ⚡

Para análisis DSP de alto rendimiento, use las funciones optimizadas que minimizan cruces JS↔WASM:

```typescript
// ❌ Lento: Múltiples cruces JS↔WASM
const t = generateTimeSamples();  // Bucle JS
const signal = ach.vector(t);
const fft = ach.fft(signal);
const mag = await fft.toMatrix();
const magnitude = mag.map(r => Math.sqrt(r[0]**2 + r[1]**2)); // Bucle JS

// ✅ Rápido: Todo en C++
const t = ach.linspace(0, 10, 1000);
const signal = t.map('t => sin(2*PI*5*t)');
const spectrum = ach.fft_spectrum(signal, 1000, true, true, 100);
// Una sola operación, ~90% más rápido
```

Ver [Funciones de Optimización](./optimization-functions.md) para más detalles.

## Gestión de Errores

El SDK lanza errores tipados para diferentes situaciones:

```typescript
try {
  const result = ach.eval('1 / 0');
} catch (error) {
  if (error instanceof AchronymeRuntimeError) {
    console.error('Error en runtime:', error.message);
  } else if (error instanceof AchronymeSyntaxError) {
    console.error('Error de sintaxis:', error.message);
  }
}
```

Ver tipos de error completos en [Tipos de Datos](./types.md).

## Recursos Adicionales

- [GitHub Repository](https://github.com/anthropics/achronyme-core)
- [Guía LLM](../llm-sdk-guide.md) - Instrucciones para LLMs
- [Especificación del Lenguaje](../language-spec.md) - Sintaxis SOC
- [Arquitectura](../ARCHITECTURE.md) - Detalles técnicos

## Licencia

MIT License - Ver LICENSE para más detalles.
