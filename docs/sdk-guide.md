# Guía del SDK TypeScript de Achronyme

El SDK de TypeScript proporciona una API tipo-segura y ergonómica sobre el núcleo WebAssembly de Achronyme.

---

## 📋 Tabla de Contenidos

- [¿Por qué usar el SDK?](#por-qué-usar-el-sdk)
- [Instalación](#instalación)
- [Inicio Rápido](#inicio-rápido)
- [Gestión de Memoria](#gestión-de-memoria)
- [API Completa](#api-completa)
- [Ejemplos](#ejemplos)
- [Manejo de Errores](#manejo-de-errores)

---

## 🎯 ¿Por qué usar el SDK?

### Antiguo: eval() directo (engorroso)

```javascript
// ❌ Construir strings manualmente
Module.eval("let sig = [1, 2, 3, 4, 5, 6, 7, 8]");
Module.eval("let spec = fft_mag(sig)");
const result = Module.eval("spec");
```

### Nuevo: SDK TypeScript (limpio y tipo-seguro)

```typescript
// ✅ API idiomática de TypeScript
const ach = new Achronyme();
await ach.init();

const sig = ach.vector([1, 2, 3, 4, 5, 6, 7, 8]);
const spec = sig.fft_mag();
console.log(await spec.toVector());

// Limpieza de memoria
sig.dispose();
spec.dispose();
```

### Beneficios del SDK

✅ **Type Safety**: TypeScript detecta errores en desarrollo
✅ **Autocompletado**: IntelliSense muestra todas las funciones
✅ **API Fluent**: Encadenamiento intuitivo de operaciones
✅ **Manejo de Errores**: Excepciones personalizadas (AchronymeSyntaxError, AchronymeTypeError, etc.)
✅ **Gestión de Memoria**: Control explícito con `dispose()`
✅ **Sin Overhead**: Internamente usa el mismo Environment de C++

---

## 📦 Instalación

```bash
# Instalar el paquete
npm install @achronyme/core

# Si quieres compilar desde el código fuente
npm install typescript --save-dev
npx tsc
```

---

## 🚀 Inicio Rápido

### Ejemplo Básico

```typescript
import { Achronyme } from '@achronyme/core';

const ach = new Achronyme();
await ach.init();

// Operaciones matemáticas
const x = ach.number(5);
const y = x.mul(2).add(10).div(4);
console.log(await y.toNumber()); // → 5

// Vectores
const v = ach.vector([1, 2, 3, 4]);
const squared = v.pow(2);
console.log(await squared.toVector()); // → [1, 4, 9, 16]

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

---

## 💾 Gestión de Memoria

El SDK usa **gestión manual explícita** mediante `dispose()`:

### ✅ Correcto: Llamar dispose() cuando termines

```typescript
const x = ach.number(10);
const y = x.add(5);
console.log(await y.toNumber());
x.dispose();
y.dispose();
```

### ❌ Incorrecto: No disponer causa fugas de memoria

```typescript
const z = ach.vector([1, 2, 3]);
// ... usar z ...
// (olvidaste dispose) ← Fuga de memoria en C++
```

### ¿Por qué gestión manual?

- El GC de JavaScript no conoce la memoria de WASM/C++
- `FinalizationRegistry` es experimental y no garantiza limpieza inmediata
- `dispose()` manual es explícito, confiable y te da control total
- Evita fugas de memoria cuando hay muchas variables intermedias

### Estadísticas de memoria

```typescript
const stats = ach.getMemoryStats();
console.log('Variables activas:', stats.activeVariables);
console.log('Variables eliminadas:', stats.disposedVariables);

// Limpiar todo (CUIDADO: invalida todos los AchronymeValue)
ach.disposeAll();
```

---

## 📖 API Completa

### Clase `Achronyme`

#### Inicialización

```typescript
const ach = new Achronyme(options?);
await ach.init();
```

#### Constructores de tipos

```typescript
ach.number(42)                      // Number
ach.vector([1, 2, 3, 4])           // Vector
ach.matrix([[1, 2], [3, 4]])       // Matrix
ach.complex(2, 3)                   // Complex: 2+3i
```

#### Funciones matemáticas básicas

```typescript
// Trigonométricas
ach.sin(x), ach.cos(x), ach.tan(x)
ach.asin(x), ach.acos(x), ach.atan(x), ach.atan2(y, x)
ach.sinh(x), ach.cosh(x), ach.tanh(x)

// Exponenciales y logaritmos
ach.exp(x), ach.ln(x), ach.log(x), ach.log10(x), ach.log2(x)

// Raíces y potencias
ach.sqrt(x), ach.cbrt(x), ach.pow(base, exp)

// Redondeo
ach.floor(x), ach.ceil(x), ach.round(x), ach.trunc(x)

// Utilidades
ach.abs(x), ach.sign(x)
ach.min(...values), ach.max(...values)

// Estadísticas (nativas, optimizadas)
ach.sum(vector)
ach.mean(vector)
ach.std(vector)
```

#### Funciones DSP

```typescript
// FFT (Fast Fourier Transform) - requiere potencia de 2
ach.fft(signal)
ach.fft_mag(signal)
ach.fft_phase(signal)
ach.ifft(spectrum)

// DFT (Discrete Fourier Transform) - acepta cualquier tamaño
ach.dft(signal)
ach.dft_mag(signal)
ach.dft_phase(signal)

// Convolución
ach.conv(signal1, signal2)          // Método directo O(N×M)
ach.conv_fft(signal1, signal2)      // Basado en FFT O((N+M)log(N+M))

// Ventanas
ach.hanning(N)
ach.hamming(N)
ach.blackman(N)
```

#### Operaciones vectoriales optimizadas

```typescript
// Nativas (evitan el parser, muy rápidas)
ach.vadd(v1, v2)        // Vector addition
ach.vsub(v1, v2)        // Vector subtraction
ach.vmul(v1, v2)        // Element-wise multiplication
ach.vdiv(v1, v2)        // Element-wise division
ach.vscale(v, scalar)   // Scalar multiplication
```

#### Higher-order functions

```typescript
ach.map('x => x^2', vector)
ach.map('(x,y) => x+y', vector1, vector2)

ach.filter('x => x > 5', vector)

ach.reduce('(a,b) => a+b', vector, initialValue)

ach.pipe(value, fn1, fn2, fn3, ...)
```

#### Variables y lambdas

```typescript
ach.let('x', 10)                    // Declarar variable
ach.get('x')                        // Obtener variable
ach.lambda(['x', 'y'], 'x + y')     // Crear función lambda
```

#### Constantes matemáticas

```typescript
ach.PI      // 3.14159265358979...
ach.E       // 2.71828182845905...
ach.PHI     // 1.61803398874989...
ach.TAU     // 6.28318530717959...
```

### Clase `AchronymeValue`

Todos los métodos de `Achronyme` que retornan valores matemáticos devuelven instancias de `AchronymeValue`, que soporta:

#### Extracción de valores

```typescript
await value.toNumber()      // → number
await value.toVector()      // → number[]
await value.toMatrix()      // → number[][]
await value.toComplex()     // → {re: number, im: number}
await value.value<T>()      // → T (auto-detect type)
```

#### Operaciones aritméticas (fluent API)

```typescript
value.add(other)
value.sub(other)
value.mul(other)
value.div(other)
value.pow(other)
value.mod(other)
value.neg()         // Unary minus
```

#### Comparaciones

```typescript
value.gt(other)     // >
value.lt(other)     // <
value.gte(other)    // >=
value.lte(other)    // <=
value.eq(other)     // ==
value.neq(other)    // !=
```

#### Funciones matemáticas

```typescript
value.sin(), value.cos(), value.tan()
value.sqrt(), value.abs(), value.ln(), value.exp()
value.floor(), value.ceil(), value.round()
// ... todas las funciones disponibles en Achronyme
```

#### Funciones DSP

```typescript
value.fft()
value.fft_mag()
value.fft_phase()
value.ifft()
value.dft()
value.dft_mag()
value.dft_phase()
```

#### Vector/Matrix específicas

```typescript
value.dot(other)        // Producto punto
value.cross(other)      // Producto cruz
value.norm()            // Norma euclidiana

value.transpose()       // Transpuesta
value.det()            // Determinante
value.inverse()        // Inversa
```

#### Gestión de memoria

```typescript
value.dispose()         // Liberar memoria C++
value.isDisposed()      // Verificar si fue disposed
value.getMetadata()     // Información de debug
```

---

## 💡 Ejemplos

### Ejemplo 1: Operaciones Matemáticas Básicas

```typescript
import { Achronyme } from '@achronyme/core';

const ach = new Achronyme();
await ach.init();

// Cálculos encadenados
const result = ach.number(5)
  .mul(2)       // 5 * 2 = 10
  .add(10)      // 10 + 10 = 20
  .div(4);      // 20 / 4 = 5

console.log(await result.toNumber()); // → 5
result.dispose();
```

### Ejemplo 2: Vectores y Estadísticas

```typescript
const data = ach.vector([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

const sum = ach.sum(data);
const mean = ach.mean(data);
const std = ach.std(data);
const max = ach.max(data);

console.log('Sum:', await sum.toNumber());      // → 55
console.log('Mean:', await mean.toNumber());    // → 5.5
console.log('Std:', await std.toNumber());      // → 2.87...
console.log('Max:', await max.toNumber());      // → 10

// Cleanup
data.dispose();
sum.dispose();
mean.dispose();
std.dispose();
max.dispose();
```

### Ejemplo 3: Pipeline DSP Completo

```typescript
// Generar señal
const signalData = Array.from({length: 1024}, (_, i) =>
  Math.sin(2 * Math.PI * 50 * i / 1000) +
  0.5 * Math.sin(2 * Math.PI * 120 * i / 1000)
);

const signal = ach.vector(signalData);

// Pipeline: Windowing → FFT → Magnitude
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

### Ejemplo 4: Higher-Order Functions

```typescript
const numbers = ach.vector([1, 2, 3, 4, 5, 6]);

// Map: elevar al cuadrado
const squared = ach.map('x => x^2', numbers);
console.log(await squared.toVector()); // → [1, 4, 9, 16, 25, 36]

// Filter: solo pares
const evens = ach.filter('x => x % 2 == 0', numbers);
console.log(await evens.toVector()); // → [2, 4, 6]

// Reduce: suma total
const sum = ach.reduce('(a,b) => a+b', numbers, ach.number(0));
console.log(await sum.toNumber()); // → 21

// Cleanup
numbers.dispose();
squared.dispose();
evens.dispose();
sum.dispose();
```

### Ejemplo 5: Álgebra Lineal

```typescript
const A = ach.matrix([[1, 2], [3, 4]]);
const B = ach.matrix([[5, 6], [7, 8]]);

// Suma de matrices
const C = A.add(B);
console.log(await C.toMatrix());
// → [[6, 8], [10, 12]]

// Multiplicación matricial
const D = A.mul(B);
console.log(await D.toMatrix());
// → [[19, 22], [43, 50]]

// Transpuesta
const At = A.transpose();
console.log(await At.toMatrix());
// → [[1, 3], [2, 4]]

// Determinante
const det = A.det();
console.log(await det.toNumber());
// → -2

// Cleanup
A.dispose();
B.dispose();
C.dispose();
D.dispose();
At.dispose();
det.dispose();
```

---

## 🚨 Manejo de Errores

El SDK envuelve errores de C++ en clases específicas de TypeScript:

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

### Tipos de Error Disponibles

| Clase | Descripción |
|-------|-------------|
| `AchronymeError` | Clase base para todos los errores |
| `AchronymeSyntaxError` | Error de sintaxis en expresiones |
| `AchronymeRuntimeError` | Error durante la ejecución |
| `AchronymeTypeError` | Error de tipo de dato |
| `AchronymeDisposedError` | Operación en valor ya disposed |
| `AchronymeNotInitializedError` | Módulo WASM no inicializado |
| `AchronymeArgumentError` | Argumentos inválidos |

---

## 📚 Ejemplos Incluidos

El proyecto incluye 4 ejemplos completos del SDK:

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

---

## 🧪 Testing

```bash
# Ejecutar test del SDK (20+ tests)
node test-sdk.mjs
```

---

## 🔗 Referencias

- [Especificación del Lenguaje](./language-spec.md)
- [Roadmap del Proyecto](./roadmap.md)
- [README Principal](../README.md)

---

**Versión**: 0.3.0
**Última actualización**: 2025
