# API Reference - Achronyme SDK

Referencia completa de todas las funciones disponibles en el SDK.

## Tabla de Contenidos

- [Inicialización y Configuración](#inicialización-y-configuración)
- [Constructores de Tipos](#constructores-de-tipos)
- [Operaciones Aritméticas](#operaciones-aritméticas)
- [Funciones Matemáticas](#funciones-matemáticas)
- [Álgebra Lineal](#álgebra-lineal)
- [Procesamiento Digital de Señales (DSP)](#procesamiento-digital-de-señales-dsp)
- [Funciones de Optimización](#funciones-de-optimización)
- [Estadísticas](#estadísticas)
- [Programación Funcional](#programación-funcional)
- [Variables y Lambdas](#variables-y-lambdas)
- [Gestión de Memoria](#gestión-de-memoria)
- [Constantes](#constantes)

---

## Inicialización y Configuración

### `constructor(options?: AchronymeOptions)`

Crea una nueva instancia de Achronyme.

**Parámetros:**
- `options` (opcional): Objeto de configuración
  - `debug?: boolean` - Habilitar modo debug (default: false)
  - `maxVariables?: number` - Límite de variables antes de advertencia (default: 10000)

**Ejemplo:**
```typescript
const ach = new Achronyme({ debug: true, maxVariables: 5000 });
```

### `init(): Promise<void>`

Inicializa el módulo WASM. **Debe llamarse antes de cualquier otra operación**.

**Retorna:** Promise que se resuelve cuando el módulo está listo.

**Ejemplo:**
```typescript
const ach = new Achronyme();
await ach.init();
```

### `isInitialized(): boolean`

Verifica si el módulo está inicializado.

**Retorna:** `true` si está inicializado, `false` en caso contrario.

---

## Constructores de Tipos

### `number(value: number): AchronymeValue`

Crea un valor escalar.

**Parámetros:**
- `value`: Número a encapsular

**Retorna:** AchronymeValue que representa el número

**Ejemplo:**
```typescript
const x = ach.number(42);
const y = ach.number(Math.PI);
```

### `vector(data: number[]): AchronymeValue`

Crea un vector (array 1D).

**Parámetros:**
- `data`: Array de números

**Retorna:** AchronymeValue que representa el vector

**Ejemplo:**
```typescript
const v = ach.vector([1, 2, 3, 4, 5]);
```

### `matrix(data: number[][]): AchronymeValue`

Crea una matriz (array 2D).

**Parámetros:**
- `data`: Array 2D de números

**Retorna:** AchronymeValue que representa la matriz

**Ejemplo:**
```typescript
const m = ach.matrix([
  [1, 2, 3],
  [4, 5, 6],
  [7, 8, 9]
]);
```

### `complex(re: number, im: number): AchronymeValue`

Crea un número complejo.

**Parámetros:**
- `re`: Parte real
- `im`: Parte imaginaria

**Retorna:** AchronymeValue que representa el número complejo

**Ejemplo:**
```typescript
const z1 = ach.complex(3, 4);      // 3 + 4i
const z2 = ach.complex(0, 5);      // 5i
const z3 = ach.complex(-2, -3);    // -2 - 3i
```

---

## Operaciones Aritméticas

Todas las operaciones están disponibles como métodos de `AchronymeValue` y como funciones del SDK.

### `add(other: AchronymeValue | number): AchronymeValue`

Suma dos valores.

**Parámetros:**
- `other`: Valor a sumar

**Retorna:** Resultado de la suma

**Ejemplo:**
```typescript
// Como método
const x = ach.number(10);
const y = x.add(5);  // 15

// Con otro AchronymeValue
const a = ach.number(3);
const b = ach.number(7);
const c = a.add(b);  // 10
```

### `sub(other: AchronymeValue | number): AchronymeValue`

Resta dos valores.

**Ejemplo:**
```typescript
const x = ach.number(10);
const y = x.sub(3);  // 7
```

### `mul(other: AchronymeValue | number): AchronymeValue`

Multiplica dos valores.

**Nota:** Para vectores y matrices, esta es multiplicación elemento a elemento. Para multiplicación matricial, los tipos deben coincidir.

**Ejemplo:**
```typescript
const x = ach.number(5);
const y = x.mul(4);  // 20

// Vectores (elemento a elemento)
const v1 = ach.vector([1, 2, 3]);
const v2 = ach.vector([2, 3, 4]);
const v3 = v1.mul(v2);  // [2, 6, 12]

// Matrices (multiplicación matricial)
const m1 = ach.matrix([[1, 2], [3, 4]]);
const m2 = ach.matrix([[5, 6], [7, 8]]);
const m3 = m1.mul(m2);  // [[19, 22], [43, 50]]
```

### `div(other: AchronymeValue | number): AchronymeValue`

Divide dos valores.

**Ejemplo:**
```typescript
const x = ach.number(20);
const y = x.div(4);  // 5
```

### `pow(exponent: AchronymeValue | number): AchronymeValue`

Eleva un valor a una potencia.

**Ejemplo:**
```typescript
const x = ach.number(2);
const y = x.pow(10);  // 1024

// Con vectores
const v = ach.vector([1, 2, 3, 4]);
const v2 = v.pow(2);  // [1, 4, 9, 16]
```

### `mod(other: AchronymeValue | number): AchronymeValue`

Calcula el módulo (resto de división).

**Ejemplo:**
```typescript
const x = ach.number(10);
const y = x.mod(3);  // 1
```

---

## Funciones Matemáticas

### Funciones Trigonométricas

#### `sin(x: AchronymeValue | number): AchronymeValue`

Seno.

**Ejemplo:**
```typescript
const x = ach.number(Math.PI / 2);
const y = ach.sin(x);  // 1

// Como método
const z = x.sin();  // 1
```

#### `cos(x: AchronymeValue | number): AchronymeValue`

Coseno.

#### `tan(x: AchronymeValue | number): AchronymeValue`

Tangente.

#### `asin(x: AchronymeValue | number): AchronymeValue`

Arco seno (sin⁻¹).

#### `acos(x: AchronymeValue | number): AchronymeValue`

Arco coseno (cos⁻¹).

#### `atan(x: AchronymeValue | number): AchronymeValue`

Arco tangente (tan⁻¹).

#### `atan2(y: AchronymeValue | number, x: AchronymeValue | number): AchronymeValue`

Arco tangente de dos argumentos. Calcula el ángulo del punto (x, y).

**Ejemplo:**
```typescript
const angle = ach.atan2(1, 1);  // π/4
```

### Funciones Hiperbólicas

#### `sinh(x: AchronymeValue | number): AchronymeValue`

Seno hiperbólico.

#### `cosh(x: AchronymeValue | number): AchronymeValue`

Coseno hiperbólico.

#### `tanh(x: AchronymeValue | number): AchronymeValue`

Tangente hiperbólica.

### Funciones Exponenciales y Logarítmicas

#### `exp(x: AchronymeValue | number): AchronymeValue`

Exponencial (e^x).

**Ejemplo:**
```typescript
const x = ach.number(1);
const y = ach.exp(x);  // e ≈ 2.718
```

#### `ln(x: AchronymeValue | number): AchronymeValue`

Logaritmo natural (base e).

#### `log(x: AchronymeValue | number): AchronymeValue`
#### `log10(x: AchronymeValue | number): AchronymeValue`

Logaritmo base 10.

#### `log2(x: AchronymeValue | number): AchronymeValue`

Logaritmo base 2.

### Funciones de Potencia

#### `sqrt(x: AchronymeValue | number): AchronymeValue`

Raíz cuadrada.

**Ejemplo:**
```typescript
const x = ach.number(16);
const y = ach.sqrt(x);  // 4
```

#### `cbrt(x: AchronymeValue | number): AchronymeValue`

Raíz cúbica.

#### `pow(base: AchronymeValue | number, exponent: AchronymeValue | number): AchronymeValue`

Potencia (base^exponent).

### Funciones de Redondeo

#### `abs(x: AchronymeValue | number): AchronymeValue`

Valor absoluto.

**Ejemplo:**
```typescript
const x = ach.number(-5);
const y = ach.abs(x);  // 5
```

#### `sign(x: AchronymeValue | number): AchronymeValue`

Signo del número (-1, 0, o 1).

#### `floor(x: AchronymeValue | number): AchronymeValue`

Redondeo hacia abajo.

#### `ceil(x: AchronymeValue | number): AchronymeValue`

Redondeo hacia arriba.

#### `round(x: AchronymeValue | number): AchronymeValue`

Redondeo al entero más cercano.

#### `trunc(x: AchronymeValue | number): AchronymeValue`

Truncamiento (elimina parte decimal).

---

## Álgebra Lineal

### Operaciones con Vectores

#### `dot(v1: AchronymeValue, v2: AchronymeValue): AchronymeValue`

Producto punto de dos vectores.

**Ejemplo:**
```typescript
const v1 = ach.vector([1, 2, 3]);
const v2 = ach.vector([4, 5, 6]);
const dot = ach.dot(v1, v2);  // 1*4 + 2*5 + 3*6 = 32
```

#### `cross(v1: AchronymeValue, v2: AchronymeValue): AchronymeValue`

Producto cruz de dos vectores 3D.

**Ejemplo:**
```typescript
const v1 = ach.vector([1, 0, 0]);
const v2 = ach.vector([0, 1, 0]);
const cross = ach.cross(v1, v2);  // [0, 0, 1]
```

#### `norm(v: AchronymeValue): AchronymeValue`

Norma (magnitud) de un vector.

**Ejemplo:**
```typescript
const v = ach.vector([3, 4]);
const length = ach.norm(v);  // 5
```

### Operaciones Vectoriales Optimizadas

Estas funciones son **mucho más rápidas** que las operaciones elemento a elemento genéricas.

#### `vadd(v1: AchronymeValue, v2: AchronymeValue): AchronymeValue`

Suma de vectores (elemento a elemento).

**Ejemplo:**
```typescript
const v1 = ach.vector([1, 2, 3]);
const v2 = ach.vector([4, 5, 6]);
const result = ach.vadd(v1, v2);  // [5, 7, 9]
```

#### `vsub(v1: AchronymeValue, v2: AchronymeValue): AchronymeValue`

Resta de vectores.

#### `vmul(v1: AchronymeValue, v2: AchronymeValue): AchronymeValue`

Multiplicación elemento a elemento.

#### `vdiv(v1: AchronymeValue, v2: AchronymeValue): AchronymeValue`

División elemento a elemento.

#### `vscale(v: AchronymeValue, scalar: number): AchronymeValue`

Multiplicación por escalar.

**Ejemplo:**
```typescript
const v = ach.vector([1, 2, 3]);
const scaled = ach.vscale(v, 2.5);  // [2.5, 5, 7.5]
```

### Operaciones con Matrices

#### `transpose(m: AchronymeValue): AchronymeValue`

Transponer matriz.

**Ejemplo:**
```typescript
const m = ach.matrix([[1, 2, 3], [4, 5, 6]]);
const mt = ach.transpose(m);
// mt = [[1, 4], [2, 5], [3, 6]]

// Como método
const mt2 = m.transpose();
```

#### `det(m: AchronymeValue): AchronymeValue`

Determinante de una matriz cuadrada.

**Ejemplo:**
```typescript
const m = ach.matrix([[1, 2], [3, 4]]);
const d = ach.det(m);  // -2

// Como método
const d2 = m.det();
```

#### `inverse(m: AchronymeValue): AchronymeValue`

Inversa de una matriz.

**Ejemplo:**
```typescript
const m = ach.matrix([[1, 2], [3, 4]]);
const inv = ach.inverse(m);
// inv = [[-2, 1], [1.5, -0.5]]
```

---

## Procesamiento Digital de Señales (DSP)

### Transformadas de Fourier

#### `fft(signal: AchronymeValue): AchronymeValue`

Fast Fourier Transform. Retorna matriz [N x 2] con [real, imaginaria].

**Complejidad:** O(N log N)

**Ejemplo:**
```typescript
const signal = ach.vector([1, 2, 3, 4, 5, 6, 7, 8]);
const spectrum = ach.fft(signal);
const matrix = await spectrum.toMatrix();
// matrix[i] = [real, imag]
```

#### `fft_mag(signal: AchronymeValue): AchronymeValue`

Espectro de magnitud FFT. Retorna vector con magnitudes.

**Ejemplo:**
```typescript
const signal = ach.vector([1, 2, 3, 4, 5, 6, 7, 8]);
const magnitude = ach.fft_mag(signal);
const mags = await magnitude.toVector();
```

#### `fft_phase(signal: AchronymeValue): AchronymeValue`

Espectro de fase FFT. Retorna vector con fases en radianes.

**Ejemplo:**
```typescript
const signal = ach.vector([1, 2, 3, 4, 5, 6, 7, 8]);
const phase = ach.fft_phase(signal);
const phases = await phase.toVector();
// Valores en rango [-π, π]
```

#### `ifft(spectrum: AchronymeValue): AchronymeValue`

Inverse Fast Fourier Transform. Convierte del dominio de frecuencia al dominio del tiempo.

**Parámetros:**
- `spectrum`: Matriz [N x 2] con [real, imaginaria]

**Retorna:** Vector con señal en dominio del tiempo

**Ejemplo:**
```typescript
const signal = ach.vector([1, 2, 3, 4, 5, 6, 7, 8]);
const spectrum = ach.fft(signal);
const recovered = ach.ifft(spectrum);
// recovered ≈ signal original
```

#### `dft(signal: AchronymeValue): AchronymeValue`

Discrete Fourier Transform (implementación directa).

**Complejidad:** O(N²) - Usar FFT para señales grandes

**Ejemplo:**
```typescript
const signal = ach.vector([1, 2, 3, 4]);
const spectrum = ach.dft(signal);
```

#### `dft_mag(signal: AchronymeValue): AchronymeValue`

Espectro de magnitud DFT.

#### `dft_phase(signal: AchronymeValue): AchronymeValue`

Espectro de fase DFT.

### Convolución

#### `conv(signal1: AchronymeValue, signal2: AchronymeValue): AchronymeValue`

Convolución lineal (método directo).

**Complejidad:** O(N*M)

**Retorna:** Vector de longitud N + M - 1

**Ejemplo:**
```typescript
const signal = ach.vector([1, 2, 3]);
const kernel = ach.vector([1, 1]);
const result = ach.conv(signal, kernel);
// result = [1, 3, 5, 3]
```

#### `conv_fft(signal1: AchronymeValue, signal2: AchronymeValue): AchronymeValue`

Convolución usando FFT (mucho más rápida para señales grandes).

**Complejidad:** O((N+M) log(N+M))

**Ejemplo:**
```typescript
const signal = ach.vector([1, 2, 3, 4, 5]);
const kernel = ach.vector([1, 1, 1]);
const result = ach.conv_fft(signal, kernel);
```

### Funciones Ventana

#### `hanning(n: number): AchronymeValue`

Genera ventana de Hanning de tamaño N.

**Propiedades:**
- Transiciones suaves a 0 en los extremos
- Supresión de lóbulos laterales: -31 dB
- Uso: Análisis espectral general

**Ejemplo:**
```typescript
const window = ach.hanning(8);
const w = await window.toVector();
// [0, 0.188, 0.611, 0.950, 0.950, 0.611, 0.188, 0]
```

#### `hamming(n: number): AchronymeValue`

Genera ventana de Hamming de tamaño N.

**Propiedades:**
- No llega a 0 en los extremos
- Supresión de lóbulos laterales: -43 dB (mejor que Hanning)
- Uso: Cuando la supresión de lóbulos es importante

**Ejemplo:**
```typescript
const window = ach.hamming(8);
```

#### `blackman(n: number): AchronymeValue`

Genera ventana de Blackman de tamaño N.

**Propiedades:**
- Excelente supresión de lóbulos: -58 dB
- Lóbulo principal más ancho (peor resolución)
- Uso: Máxima reducción de fuga espectral

**Ejemplo:**
```typescript
const window = ach.blackman(8);
```

---

## Funciones de Optimización

Funciones de alto rendimiento que minimizan cruces JS↔WASM.

### `linspace(start: number, end: number, n: number): AchronymeValue`

Genera N muestras uniformemente espaciadas entre start y end.

**Parámetros:**
- `start`: Valor inicial
- `end`: Valor final
- `n`: Número de muestras

**Retorna:** Vector con N muestras

**Ejemplo:**
```typescript
const t = ach.linspace(0, 10, 100);
// 100 muestras de 0 a 10
const samples = await t.toVector();
// [0, 0.101, 0.202, ..., 9.899, 10]
```

### `fftshift(vector: AchronymeValue): AchronymeValue`

Reordena espectro FFT para centrar la frecuencia cero.

**Ejemplo:**
```typescript
const spectrum = ach.fft_mag(signal);
const centered = ach.fftshift(spectrum);
```

### `ifftshift(vector: AchronymeValue): AchronymeValue`

Invierte la operación de fftshift.

**Ejemplo:**
```typescript
const original = ach.ifftshift(shifted);
```

### `fft_spectrum(signal, fs, shift?, angular?, omegaRange?): AchronymeValue`

**Función TODO-EN-UNO de alto rendimiento** para análisis espectral completo.

**Parámetros:**
- `signal`: Vector de señal
- `fs`: Frecuencia de muestreo (Hz)
- `shift` (opcional): Aplicar fftshift (default: true)
- `angular` (opcional): Convertir Hz → rad/s (default: true)
- `omegaRange` (opcional): Filtrar a [-range, range] (default: sin filtro)

**Retorna:** Matriz [N x 3] donde cada fila es [omega, magnitude, phase]

**Ejemplo:**
```typescript
const signal = ach.vector([...]);  // 1000 muestras
const fs = 1000;  // 1 kHz

// Análisis completo en una operación
const spectrum = ach.fft_spectrum(signal, fs, true, true, 50);
const result = await spectrum.toMatrix();

// Extraer componentes
const omega = result.map(row => row[0]);      // rad/s
const magnitude = result.map(row => row[1]);
const phase = result.map(row => row[2]);
```

**Beneficios:**
- ⚡⚡⚡ ~90% más rápido que múltiples operaciones
- Elimina 5+ cruces JS↔WASM
- Todo procesado en C++ en una sola pasada

Ver [Funciones de Optimización](./optimization-functions.md) para más detalles.

---

## Estadísticas

### `sum(arr: AchronymeValue): AchronymeValue`

Suma de todos los elementos de un vector.

**Ejemplo:**
```typescript
const v = ach.vector([1, 2, 3, 4, 5]);
const total = await ach.sum(v).toNumber();  // 15
```

### `mean(arr: AchronymeValue): AchronymeValue`

Media (promedio) de un vector.

**Ejemplo:**
```typescript
const v = ach.vector([1, 2, 3, 4, 5]);
const avg = await ach.mean(v).toNumber();  // 3
```

### `std(arr: AchronymeValue): AchronymeValue`

Desviación estándar de un vector.

**Ejemplo:**
```typescript
const v = ach.vector([2, 4, 4, 4, 5, 5, 7, 9]);
const stdDev = await ach.std(v).toNumber();
```

### `min(...values): AchronymeValue`

Valor mínimo. Soporta variádico y vectores.

**Ejemplo:**
```typescript
// Variádico
const m1 = await ach.min(5, 3, 8, 1).toNumber();  // 1

// Vector
const v = ach.vector([5, 3, 8, 1]);
const m2 = await ach.min(v).toNumber();  // 1
```

### `max(...values): AchronymeValue`

Valor máximo. Soporta variádico y vectores.

**Ejemplo:**
```typescript
// Variádico
const m1 = await ach.max(5, 3, 8, 1).toNumber();  // 8

// Vector
const v = ach.vector([5, 3, 8, 1]);
const m2 = await ach.max(v).toNumber();  // 8
```

---

## Programación Funcional

### `lambda(params: string[], body: string): AchronymeValue`

Crea una función lambda.

**Parámetros:**
- `params`: Array con nombres de parámetros
- `body`: Expresión del cuerpo de la función

**Retorna:** Función lambda

**Ejemplo:**
```typescript
const square = ach.lambda(['x'], 'x ^ 2');
const add = ach.lambda(['a', 'b'], 'a + b');
const distance = ach.lambda(['x', 'y'], 'sqrt(x^2 + y^2)');
```

### `map(fn, arr): AchronymeValue`

Aplica una función a cada elemento de un vector.

**Parámetros:**
- `fn`: String con lambda o AchronymeValue con función
- `arr`: Vector a mapear

**Retorna:** Nuevo vector con resultados

**Ejemplo:**
```typescript
const v = ach.vector([1, 2, 3, 4]);

// Con string inline
const squared = ach.map('x => x ^ 2', v);
// [1, 4, 9, 16]

// Con lambda definida
const square = ach.lambda(['x'], 'x ^ 2');
const squared2 = ach.map(square, v);

// Como método
const squared3 = v.map('x => x ^ 2');
```

### `filter(predicate, arr): AchronymeValue`

Filtra elementos de un vector según un predicado.

**Parámetros:**
- `predicate`: Función que retorna booleano
- `arr`: Vector a filtrar

**Retorna:** Nuevo vector con elementos que cumplen el predicado

**Ejemplo:**
```typescript
const v = ach.vector([1, 2, 3, 4, 5, 6]);

// Números pares
const evens = ach.filter('x => x % 2 == 0', v);
// [2, 4, 6]

// Mayores a 3
const greaterThan3 = v.filter('x => x > 3');
// [4, 5, 6]
```

### `reduce(fn, arr, initial): AchronymeValue`

Reduce un vector a un valor único.

**Parámetros:**
- `fn`: Función reductora (toma acumulador y elemento actual)
- `arr`: Vector a reducir
- `initial`: Valor inicial del acumulador

**Retorna:** Valor reducido

**Ejemplo:**
```typescript
const v = ach.vector([1, 2, 3, 4, 5]);

// Suma
const sum = ach.reduce('a, b => a + b', v, 0);
// 15

// Producto
const product = ach.reduce('a, b => a * b', v, 1);
// 120
```

### `pipe(...fnsAndValue): AchronymeValue`

Composición de funciones de izquierda a derecha (f(g(h(x)))).

**Ejemplo:**
```typescript
const double = ach.lambda(['x'], 'x * 2');
const addTen = ach.lambda(['x'], 'x + 10');

const result = ach.pipe(double, addTen, ach.number(5));
// (5 * 2) + 10 = 20
```

### `compose(...fns): AchronymeValue`

Composición de funciones de derecha a izquierda.

**Ejemplo:**
```typescript
const double = ach.lambda(['x'], 'x * 2');
const addTen = ach.lambda(['x'], 'x + 10');

const composed = ach.compose(addTen, double);
// Crea función: x => addTen(double(x))
```

---

## Variables y Lambdas

### `let(name: string, value): AchronymeValue`

Crea una variable con nombre en el entorno.

**Parámetros:**
- `name`: Nombre de la variable
- `value`: Valor inicial (AchronymeValue, number, number[], o ComplexNumber)

**Retorna:** Referencia a la variable

**Ejemplo:**
```typescript
ach.let('x', 10);
ach.let('pi', Math.PI);
ach.let('signal', [1, 2, 3, 4, 5]);
```

### `get(name: string): AchronymeValue`

Obtiene referencia a una variable existente.

**Ejemplo:**
```typescript
ach.let('x', 42);
const xRef = ach.get('x');
const doubled = xRef.mul(2);
```

---

## Gestión de Memoria

### `dispose(): void`

Libera un valor (método de AchronymeValue).

**Ejemplo:**
```typescript
const x = ach.number(42);
const y = x.mul(2);

x.dispose();
y.dispose();
```

### `isDisposed(): boolean`

Verifica si un valor ha sido dispuesto.

**Ejemplo:**
```typescript
const x = ach.number(42);
console.log(x.isDisposed());  // false

x.dispose();
console.log(x.isDisposed());  // true
```

### `disposeAll(): void`

Libera todas las variables rastreadas.

**⚠️ Advertencia:** Esto invalidará todos los AchronymeValue existentes.

**Ejemplo:**
```typescript
ach.disposeAll();
```

### `reset(): void`

Reinicia completamente el entorno C++ y el SDK.

**Ejemplo:**
```typescript
ach.reset();
```

### `getMemoryStats(): MemoryStats`

Obtiene estadísticas de uso de memoria.

**Retorna:** Objeto con información de memoria

**Ejemplo:**
```typescript
const stats = ach.getMemoryStats();
console.log(`Active: ${stats.activeVariables}`);
console.log(`Disposed: ${stats.disposedVariables}`);
console.log('Variables:', stats.variableNames);
```

---

## Constantes

### `PI: AchronymeValue`

Constante π (pi).

**Ejemplo:**
```typescript
const pi = ach.PI;
const value = await pi.toNumber();  // 3.14159...
```

### `E: AchronymeValue`

Constante e (número de Euler).

**Ejemplo:**
```typescript
const e = ach.E;
const value = await e.toNumber();  // 2.71828...
```

### `PHI: AchronymeValue`

Constante φ (número áureo).

**Ejemplo:**
```typescript
const phi = ach.PHI;
const value = await phi.toNumber();  // 1.61803...
```

### `TAU: AchronymeValue`

Constante τ (tau = 2π).

**Ejemplo:**
```typescript
const tau = ach.TAU;
const value = await tau.toNumber();  // 6.28318...
```

---

## Evaluación Directa

### `eval(expression: string): string`

Evalúa una expresión y retorna el resultado como string.

**Ejemplo:**
```typescript
const result = ach.eval('2 + 2 * 3');  // "8"
const result2 = ach.eval('sin(PI / 4)');
```

### `evalValue(expression: string): AchronymeValue`

Evalúa una expresión y envuelve el resultado en AchronymeValue.

**Ejemplo:**
```typescript
const x = ach.evalValue('sqrt(16) + 3');
const value = await x.toNumber();  // 7
```

---

## Ver También

- [Tipos](./types.md) - Tipos y estructuras de datos
- [Funciones de Optimización](./optimization-functions.md) - Funciones de alto rendimiento
- [Ejemplos](./examples.md) - Casos de uso prácticos
- [Gestión de Memoria](./memory-management.md) - Buenas prácticas
