# Ejemplos Prácticos - Achronyme SDK

Colección de ejemplos completos y casos de uso comunes.

## Tabla de Contenidos

- [Ejemplos Básicos](#ejemplos-básicos)
- [Álgebra Lineal](#álgebra-lineal)
- [Procesamiento de Señales](#procesamiento-de-señales)
- [Análisis Espectral](#análisis-espectral)
- [Programación Funcional](#programación-funcional)
- [Casos de Uso Avanzados](#casos-de-uso-avanzados)

---

## Ejemplos Básicos

### Inicialización y Primeros Pasos

```typescript
import { Achronyme } from '@achronyme/core';

// Crear e inicializar
const ach = new Achronyme({ debug: false });
await ach.init();

// Operaciones simples
const x = ach.number(10);
const y = x.add(5).mul(2);
console.log(await y.toNumber());  // 30

// Limpieza
x.dispose();
y.dispose();
```

### Trabajar con Vectores

```typescript
// Crear vector
const v = ach.vector([1, 2, 3, 4, 5]);

// Operaciones
const doubled = v.mul(2);
const squared = v.map('x => x ^ 2');
const filtered = v.filter('x => x > 2');

// Extraer valores
const doubValues = await doubled.toVector();    // [2, 4, 6, 8, 10]
const sqValues = await squared.toVector();      // [1, 4, 9, 16, 25]
const filtValues = await filtered.toVector();   // [3, 4, 5]

// Limpieza
v.dispose();
doubled.dispose();
squared.dispose();
filtered.dispose();
```

### Trabajar con Matrices

```typescript
// Crear matriz 3x3
const m = ach.matrix([
  [1, 2, 3],
  [4, 5, 6],
  [7, 8, 9]
]);

// Transposición
const mt = m.transpose();
const matrixT = await mt.toMatrix();
// [[1, 4, 7], [2, 5, 8], [3, 6, 9]]

// Operaciones
const scaled = m.mul(2);
const matrixScaled = await scaled.toMatrix();

m.dispose();
mt.dispose();
scaled.dispose();
```

---

## Álgebra Lineal

### Resolver Sistema de Ecuaciones

```typescript
// Resolver Ax = b
// A = [[2, 1], [1, 3]]
// b = [5, 7]
// Solución: x = A^(-1) * b

const A = ach.matrix([[2, 1], [1, 3]]);
const b = ach.vector([5, 7]);

// Calcular inversa
const Ainv = A.inverse();

// Convertir b a matriz columna para multiplicación
const bMatrix = ach.matrix([[5], [7]]);
const x = Ainv.mul(bMatrix);

const solution = await x.toMatrix();
console.log('Solución:', solution);  // [[1], [3]]

A.dispose();
b.dispose();
Ainv.dispose();
bMatrix.dispose();
x.dispose();
```

### Producto Punto y Cruz

```typescript
// Producto punto
const v1 = ach.vector([1, 2, 3]);
const v2 = ach.vector([4, 5, 6]);

const dotProduct = ach.dot(v1, v2);
console.log(await dotProduct.toNumber());  // 32

// Producto cruz (vectores 3D)
const a = ach.vector([1, 0, 0]);
const b = ach.vector([0, 1, 0]);

const cross = ach.cross(a, b);
console.log(await cross.toVector());  // [0, 0, 1]

// Limpieza
v1.dispose();
v2.dispose();
dotProduct.dispose();
a.dispose();
b.dispose();
cross.dispose();
```

### Transformaciones Lineales

```typescript
// Matriz de rotación 2D
function createRotationMatrix(angle: number) {
  const cos = Math.cos(angle);
  const sin = Math.sin(angle);
  return ach.matrix([
    [cos, -sin],
    [sin, cos]
  ]);
}

// Rotar vector 45 grados
const angle = Math.PI / 4;  // 45 grados
const rotMatrix = createRotationMatrix(angle);
const point = ach.matrix([[1], [0]]);  // Punto (1, 0)

const rotated = rotMatrix.mul(point);
const result = await rotated.toMatrix();
console.log('Punto rotado:', result);
// [[0.707], [0.707]]

rotMatrix.dispose();
point.dispose();
rotated.dispose();
```

---

## Procesamiento de Señales

### Generar Señal Sinusoidal

```typescript
// Parámetros
const fs = 1000;        // Frecuencia de muestreo (Hz)
const f = 10;           // Frecuencia de señal (Hz)
const duration = 1;     // Duración (segundos)
const N = fs * duration; // Número de muestras

// Generar vector de tiempo usando linspace optimizado
const t = ach.linspace(0, duration, N);

// Generar señal: sin(2πft)
const signal = t.map(`t => sin(2 * PI * ${f} * t)`);

// Extraer primeras 10 muestras
const samples = await signal.toVector();
console.log('Primeras muestras:', samples.slice(0, 10));

t.dispose();
signal.dispose();
```

### Aplicar Ventana y FFT

```typescript
const N = 512;

// Generar señal
const t = ach.linspace(0, 1, N);
const signal = t.map('t => sin(2*PI*10*t) + 0.5*sin(2*PI*25*t)');

// Aplicar ventana de Hanning
const window = ach.hanning(N);
const windowedSignal = ach.vmul(signal, window);  // Multiplicación optimizada

// Calcular FFT
const spectrum = ach.fft_mag(windowedSignal);

// Procesar resultado
const spectrumValues = await spectrum.toVector();
console.log('Primeros bins:', spectrumValues.slice(0, 20));

// Limpieza
t.dispose();
signal.dispose();
window.dispose();
windowedSignal.dispose();
spectrum.dispose();
```

### Filtrado por Convolución

```typescript
// Señal original
const signal = ach.vector([1, 2, 3, 4, 5, 4, 3, 2, 1]);

// Filtro de media móvil (3 puntos)
const kernel = ach.vector([1/3, 1/3, 1/3]);

// Aplicar filtro
const filtered = ach.conv(signal, kernel);

const result = await filtered.toVector();
console.log('Señal filtrada:', result);

signal.dispose();
kernel.dispose();
filtered.dispose();
```

---

## Análisis Espectral

### Análisis Espectral Básico

```typescript
const N = 1000;
const fs = 1000;  // 1 kHz

// Generar señal con múltiples frecuencias
const t = ach.linspace(0, 1, N);
const signal = t.map('t => sin(2*PI*50*t) + 0.5*sin(2*PI*120*t)');

// Análisis espectral completo en una operación
const spectrum = ach.fft_spectrum(signal, fs, true, true, 200);
const result = await spectrum.toMatrix();

// Extraer componentes
const omega = result.map(row => row[0]);      // Frecuencia (rad/s)
const magnitude = result.map(row => row[1]);  // Magnitud
const phase = result.map(row => row[2]);      // Fase

// Encontrar picos (en JavaScript)
const peaks = magnitude
  .map((mag, i) => ({ freq: omega[i] / (2 * Math.PI), mag }))
  .filter(p => p.mag > 100)
  .sort((a, b) => b.mag - a.mag);

console.log('Picos detectados:', peaks.slice(0, 5));

t.dispose();
signal.dispose();
spectrum.dispose();
```

### Análisis de Señal Compleja

```typescript
async function analyzeSignal(
  signal: AchronymeValue,
  fs: number,
  showPlot: boolean = false
) {
  // Estadísticas básicas
  const mean = await ach.mean(signal).toNumber();
  const std = await ach.std(signal).toNumber();

  console.log(`Media: ${mean.toFixed(4)}`);
  console.log(`Desviación estándar: ${std.toFixed(4)}`);

  // Análisis espectral
  const spectrum = ach.fft_spectrum(signal, fs, true, true, -1);
  const specData = await spectrum.toMatrix();

  // Magnitud máxima
  const magnitudes = specData.map(row => row[1]);
  const maxMag = Math.max(...magnitudes);
  const maxIdx = magnitudes.indexOf(maxMag);
  const dominantFreq = specData[maxIdx][0] / (2 * Math.PI);

  console.log(`Frecuencia dominante: ${dominantFreq.toFixed(2)} Hz`);
  console.log(`Magnitud máxima: ${maxMag.toFixed(2)}`);

  spectrum.dispose();

  return {
    mean,
    std,
    dominantFreq,
    maxMag
  };
}

// Uso
const t = ach.linspace(0, 1, 1000);
const testSignal = t.map('t => sin(2*PI*25*t)');
const analysis = await analyzeSignal(testSignal, 1000);

t.dispose();
testSignal.dispose();
```

### Espectrograma Simple

```typescript
function computeSpectrogram(
  signal: AchronymeValue,
  fs: number,
  windowSize: number,
  hopSize: number
) {
  const spectrogram: number[][] = [];

  // Este es un ejemplo simplificado
  // En producción, usarías STFT (Short-Time Fourier Transform)

  return spectrogram;
}
```

---

## Programación Funcional

### Map, Filter, Reduce

```typescript
const data = ach.vector([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

// Map: elevar al cuadrado
const squared = data.map('x => x^2');
console.log(await squared.toVector());
// [1, 4, 9, 16, 25, 36, 49, 64, 81, 100]

// Filter: solo pares
const evens = data.filter('x => x % 2 == 0');
console.log(await evens.toVector());
// [2, 4, 6, 8, 10]

// Reduce: suma total
const sum = ach.reduce('a, b => a + b', data, 0);
console.log(await sum.toNumber());
// 55

data.dispose();
squared.dispose();
evens.dispose();
sum.dispose();
```

### Composición de Funciones

```typescript
// Definir funciones
const double = ach.lambda(['x'], 'x * 2');
const addTen = ach.lambda(['x'], 'x + 10');
const square = ach.lambda(['x'], 'x ^ 2');

// Pipe: izquierda a derecha
const value = ach.number(5);
const result1 = ach.pipe(double, addTen, square, value);
// ((5 * 2) + 10) ^ 2 = 20 ^ 2 = 400
console.log(await result1.toNumber());

// Compose: derecha a izquierda
const composed = ach.compose(square, addTen, double);
// Crea función: x => square(addTen(double(x)))

double.dispose();
addTen.dispose();
square.dispose();
value.dispose();
result1.dispose();
composed.dispose();
```

### Procesamiento de Datos con HOF

```typescript
// Datos de temperatura
const temps = ach.vector([18, 21, 23, 19, 22, 25, 28, 26, 24, 20]);

// Convertir Celsius a Fahrenheit
const toFahrenheit = temps.map('c => c * 9/5 + 32');

// Filtrar días cálidos (> 75°F)
const warmDays = toFahrenheit.filter('f => f > 75');

// Temperatura promedio de días cálidos
const warmTemps = await warmDays.toVector();
const avgWarm = warmTemps.reduce((a, b) => a + b, 0) / warmTemps.length;

console.log('Días cálidos:', warmTemps);
console.log('Temperatura promedio días cálidos:', avgWarm.toFixed(1) + '°F');

temps.dispose();
toFahrenheit.dispose();
warmDays.dispose();
```

---

## Casos de Uso Avanzados

### Detector de Frecuencia Cardíaca

```typescript
async function detectHeartRate(ecgSignal: number[], fs: number) {
  const ach = new Achronyme();
  await ach.init();

  // Cargar señal
  const signal = ach.vector(ecgSignal);

  // Filtrado básico (eliminar DC)
  const mean = await ach.mean(signal).toNumber();
  const centered = signal.sub(mean);

  // Análisis espectral
  const spectrum = ach.fft_spectrum(centered, fs, true, true, 10);
  const specData = await spectrum.toMatrix();

  // Buscar pico en rango de frecuencia cardíaca (0.5 - 3 Hz)
  const hrRange = specData.filter(row => {
    const freqHz = row[0] / (2 * Math.PI);
    return freqHz >= 0.5 && freqHz <= 3;
  });

  // Encontrar máximo
  const maxMag = Math.max(...hrRange.map(r => r[1]));
  const maxRow = hrRange.find(r => r[1] === maxMag);
  const hrFreq = maxRow![0] / (2 * Math.PI);
  const bpm = hrFreq * 60;

  console.log(`Frecuencia cardíaca: ${bpm.toFixed(1)} BPM`);

  // Limpieza
  signal.dispose();
  centered.dispose();
  spectrum.dispose();

  return bpm;
}
```

### Análisis de Armónicos

```typescript
async function analyzeHarmonics(
  signal: AchronymeValue,
  fs: number,
  fundamental: number
) {
  const spectrum = ach.fft_spectrum(signal, fs, true, true, -1);
  const specData = await spectrum.toMatrix();

  // Buscar armónicos (múltiplos de la fundamental)
  const harmonics = [];
  for (let n = 1; n <= 5; n++) {
    const targetFreq = fundamental * n;
    const targetOmega = targetFreq * 2 * Math.PI;

    // Encontrar bin más cercano
    let minDist = Infinity;
    let closestRow = specData[0];

    for (const row of specData) {
      const dist = Math.abs(row[0] - targetOmega);
      if (dist < minDist) {
        minDist = dist;
        closestRow = row;
      }
    }

    harmonics.push({
      n,
      freq: closestRow[0] / (2 * Math.PI),
      magnitude: closestRow[1],
      phase: closestRow[2]
    });
  }

  spectrum.dispose();

  console.log('Armónicos detectados:');
  harmonics.forEach(h => {
    console.log(`  ${h.n}º armónico: ${h.freq.toFixed(2)} Hz, mag: ${h.magnitude.toFixed(2)}`);
  });

  return harmonics;
}
```

### Sistema de Ecuaciones Diferenciales (Euler)

```typescript
// Resolver dy/dt = f(t, y) con método de Euler
async function eulerMethod(
  f: string,              // Función f(t, y)
  t0: number,            // Tiempo inicial
  y0: number,            // Condición inicial
  tEnd: number,          // Tiempo final
  steps: number          // Número de pasos
): Promise<{ t: number[], y: number[] }> {
  const ach = new Achronyme();
  await ach.init();

  const dt = (tEnd - t0) / steps;
  const t = [t0];
  const y = [y0];

  // Crear lambda para f
  const fLambda = ach.lambda(['t', 'y'], f);

  for (let i = 0; i < steps; i++) {
    const ti = ach.number(t[i]);
    const yi = ach.number(y[i]);

    // k = f(ti, yi)
    const k = ach.evalValue(`${f.replace(/t/g, ti._varName).replace(/y/g, yi._varName)}`);
    const kVal = await k.toNumber();

    // y_{i+1} = y_i + dt * k
    const yNext = y[i] + dt * kVal;
    const tNext = t[i] + dt;

    t.push(tNext);
    y.push(yNext);

    ti.dispose();
    yi.dispose();
    k.dispose();
  }

  fLambda.dispose();

  return { t, y };
}

// Ejemplo: dy/dt = -y (decaimiento exponencial)
const solution = await eulerMethod('0 - y', 0, 1, 5, 100);
console.log('Solución:', solution.y.slice(0, 10));
```

### Análisis de Correlación

```typescript
async function crossCorrelation(
  signal1: AchronymeValue,
  signal2: AchronymeValue
) {
  // La correlación cruzada es equivalente a convolución
  // con la segunda señal invertida

  // Normalizar señales
  const mean1 = await ach.mean(signal1).toNumber();
  const mean2 = await ach.mean(signal2).toNumber();
  const std1 = await ach.std(signal1).toNumber();
  const std2 = await ach.std(signal2).toNumber();

  const norm1 = signal1.sub(mean1).div(std1);
  const norm2 = signal2.sub(mean2).div(std2);

  // Calcular correlación usando FFT (más rápido)
  const corr = ach.conv_fft(norm1, norm2);

  const corrValues = await corr.toVector();

  norm1.dispose();
  norm2.dispose();
  corr.dispose();

  return corrValues;
}
```

---

## Patrones de Gestión de Memoria

### Patrón 1: Limpieza Inmediata

```typescript
// Crear, usar, y limpiar inmediatamente
const x = ach.number(42);
const result = await x.mul(2).toNumber();
x.dispose();
// Usar result (número primitivo)
```

### Patrón 2: Batch Disposal

```typescript
// Acumular valores y limpiar al final
const values: AchronymeValue[] = [];

values.push(ach.number(1));
values.push(ach.number(2));
values.push(ach.vector([1, 2, 3]));

// Usar valores...

// Limpiar todo
values.forEach(v => v.dispose());
```

### Patrón 3: Try-Finally

```typescript
const x = ach.number(10);
const y = ach.number(20);

try {
  const result = x.add(y);
  const value = await result.toNumber();
  console.log(value);
  result.dispose();
} finally {
  x.dispose();
  y.dispose();
}
```

### Patrón 4: Función Helper

```typescript
async function withValues<T>(
  creator: (ach: Achronyme) => AchronymeValue[],
  user: (values: AchronymeValue[]) => Promise<T>
): Promise<T> {
  const values = creator(ach);
  try {
    return await user(values);
  } finally {
    values.forEach(v => v.dispose());
  }
}

// Uso
const result = await withValues(
  ach => [ach.number(10), ach.number(20)],
  async ([x, y]) => {
    const sum = x.add(y);
    const value = await sum.toNumber();
    sum.dispose();
    return value;
  }
);
```

---

## Ver También

- [API Reference](./api-reference.md) - Documentación completa de funciones
- [Tipos](./types.md) - Tipos y estructuras de datos
- [Gestión de Memoria](./memory-management.md) - Buenas prácticas
- [Funciones de Optimización](./optimization-functions.md) - Alto rendimiento
