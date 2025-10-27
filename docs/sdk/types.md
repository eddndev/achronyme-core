# Tipos de Datos - Achronyme SDK

Referencia completa de tipos TypeScript, interfaces y estructuras de datos del SDK.

## Tabla de Contenidos

- [Tipos Principales](#tipos-principales)
- [Interfaces de Configuración](#interfaces-de-configuración)
- [Tipos de Error](#tipos-de-error)
- [Tipos de Valor](#tipos-de-valor)
- [Tipos Auxiliares](#tipos-auxiliares)

---

## Tipos Principales

### `Achronyme`

Clase principal del SDK que proporciona acceso a todas las funcionalidades.

```typescript
class Achronyme {
  constructor(options?: AchronymeOptions);

  // Lifecycle
  init(): Promise<void>;
  reset(): void;
  isInitialized(): boolean;

  // Type Constructors
  number(value: number): AchronymeValue;
  vector(data: number[]): AchronymeValue;
  matrix(data: number[][]): AchronymeValue;
  complex(re: number, im: number): AchronymeValue;

  // Variables
  let(name: string, value: AchronymeValue | number | number[] | ComplexNumber): AchronymeValue;
  get(name: string): AchronymeValue;
  lambda(params: string[], body: string): AchronymeValue;

  // Memory Management
  disposeAll(): void;
  getMemoryStats(): MemoryStats;

  // Math Functions
  sin(x: AchronymeValue | number): AchronymeValue;
  cos(x: AchronymeValue | number): AchronymeValue;
  // ... (ver API Reference para lista completa)

  // DSP Functions
  fft(signal: AchronymeValue): AchronymeValue;
  fft_mag(signal: AchronymeValue): AchronymeValue;
  fft_phase(signal: AchronymeValue): AchronymeValue;
  ifft(spectrum: AchronymeValue): AchronymeValue;
  // ... (ver API Reference)

  // Optimization Functions
  linspace(start: number, end: number, n: number): AchronymeValue;
  fftshift(vector: AchronymeValue): AchronymeValue;
  ifftshift(vector: AchronymeValue): AchronymeValue;
  fft_spectrum(
    signal: AchronymeValue,
    fs: number,
    shift?: boolean,
    angular?: boolean,
    omegaRange?: number
  ): AchronymeValue;

  // Higher-Order Functions
  map(fn: string | AchronymeValue, arr: AchronymeValue): AchronymeValue;
  filter(predicate: string | AchronymeValue, arr: AchronymeValue): AchronymeValue;
  reduce(fn: string | AchronymeValue, arr: AchronymeValue, initial: number): AchronymeValue;

  // Direct Evaluation
  eval(expression: string): string;
  evalValue(expression: string): AchronymeValue;
}
```

### `AchronymeValue`

Proxy que representa un valor en el entorno C++. Proporciona API fluida para operaciones encadenables.

```typescript
class AchronymeValue {
  // Arithmetic Operations
  add(other: AchronymeValue | number): AchronymeValue;
  sub(other: AchronymeValue | number): AchronymeValue;
  mul(other: AchronymeValue | number): AchronymeValue;
  div(other: AchronymeValue | number): AchronymeValue;
  pow(exponent: AchronymeValue | number): AchronymeValue;
  mod(other: AchronymeValue | number): AchronymeValue;

  // Math Functions
  sin(): AchronymeValue;
  cos(): AchronymeValue;
  tan(): AchronymeValue;
  sqrt(): AchronymeValue;
  exp(): AchronymeValue;
  ln(): AchronymeValue;
  abs(): AchronymeValue;

  // DSP Functions
  fft(): AchronymeValue;
  fft_mag(): AchronymeValue;
  fft_phase(): AchronymeValue;
  ifft(): AchronymeValue;

  // Higher-Order Functions
  map(fn: string): AchronymeValue;
  filter(predicate: string): AchronymeValue;

  // Matrix/Vector Operations
  transpose(): AchronymeValue;
  det(): AchronymeValue;
  inverse(): AchronymeValue;
  norm(): AchronymeValue;

  // Value Extraction
  toNumber(): Promise<number>;
  toVector(): Promise<number[]>;
  toMatrix(): Promise<number[][]>;
  toComplex(): Promise<ComplexNumber>;
  toString(): Promise<string>;

  // Memory Management
  dispose(): void;
  isDisposed(): boolean;

  // Internal (do not use directly)
  readonly _varName: string;
}
```

---

## Interfaces de Configuración

### `AchronymeOptions`

Opciones de configuración para inicializar Achronyme.

```typescript
interface AchronymeOptions {
  /**
   * Habilitar modo debug con logging detallado
   * @default false
   */
  debug?: boolean;

  /**
   * Número máximo de variables antes de mostrar advertencia
   * @default 10000
   */
  maxVariables?: number;
}
```

**Ejemplo:**
```typescript
const ach = new Achronyme({
  debug: true,
  maxVariables: 5000
});
```

### `MemoryStats`

Estadísticas de uso de memoria.

```typescript
interface MemoryStats {
  /**
   * Total de variables creadas desde el inicio
   */
  totalVariables: number;

  /**
   * Número de variables actualmente activas
   */
  activeVariables: number;

  /**
   * Número de variables que han sido dispuestas
   */
  disposedVariables: number;

  /**
   * Lista de nombres de variables activas
   */
  variableNames: string[];
}
```

**Ejemplo:**
```typescript
const stats = ach.getMemoryStats();
console.log(`Active: ${stats.activeVariables}, Disposed: ${stats.disposedVariables}`);
console.log('Variables:', stats.variableNames);
```

### `ComplexNumber`

Representación de número complejo.

```typescript
interface ComplexNumber {
  /**
   * Parte real
   */
  re: number;

  /**
   * Parte imaginaria
   */
  im: number;
}
```

**Ejemplo:**
```typescript
const z: ComplexNumber = { re: 3, im: 4 };
const complexValue = ach.complex(z.re, z.im);
```

---

## Tipos de Error

El SDK proporciona errores tipados para diferentes situaciones:

### `AchronymeError`

Clase base para todos los errores de Achronyme.

```typescript
class AchronymeError extends Error {
  readonly code: string;

  constructor(message: string, code?: string);
}
```

### `AchronymeNotInitializedError`

Se lanza cuando se intenta usar el SDK antes de llamar a `init()`.

```typescript
class AchronymeNotInitializedError extends AchronymeError {
  readonly code: 'NOT_INITIALIZED';
}
```

**Ejemplo:**
```typescript
try {
  const x = ach.number(42); // Sin init() antes
} catch (error) {
  if (error instanceof AchronymeNotInitializedError) {
    console.error('Debe llamar a ach.init() primero');
  }
}
```

### `AchronymeDisposedError`

Se lanza al intentar usar un valor ya dispuesto.

```typescript
class AchronymeDisposedError extends AchronymeError {
  readonly code: 'DISPOSED';
  readonly variableName: string;
}
```

**Ejemplo:**
```typescript
const x = ach.number(42);
x.dispose();

try {
  await x.toNumber(); // Error: valor dispuesto
} catch (error) {
  if (error instanceof AchronymeDisposedError) {
    console.error(`Variable ${error.variableName} ya fue dispuesta`);
  }
}
```

### `AchronymeArgumentError`

Se lanza cuando los argumentos de una función son inválidos.

```typescript
class AchronymeArgumentError extends AchronymeError {
  readonly code: 'ARGUMENT_ERROR';
}
```

**Ejemplo:**
```typescript
try {
  ach.vector("not an array"); // Tipo incorrecto
} catch (error) {
  if (error instanceof AchronymeArgumentError) {
    console.error('Argumento inválido:', error.message);
  }
}
```

### `AchronymeRuntimeError`

Se lanza cuando ocurre un error en el runtime de C++.

```typescript
class AchronymeRuntimeError extends AchronymeError {
  readonly code: 'RUNTIME_ERROR';
  readonly expression?: string;
}
```

**Ejemplo:**
```typescript
try {
  ach.eval('1 / 0'); // División por cero
} catch (error) {
  if (error instanceof AchronymeRuntimeError) {
    console.error('Error en runtime:', error.message);
    console.error('Expresión:', error.expression);
  }
}
```

### `AchronymeSyntaxError`

Se lanza cuando hay un error de sintaxis en una expresión.

```typescript
class AchronymeSyntaxError extends AchronymeError {
  readonly code: 'SYNTAX_ERROR';
  readonly expression: string;
}
```

**Ejemplo:**
```typescript
try {
  ach.eval('2 +'); // Sintaxis incompleta
} catch (error) {
  if (error instanceof AchronymeSyntaxError) {
    console.error('Error de sintaxis:', error.message);
    console.error('Expresión:', error.expression);
  }
}
```

### `AchronymeTypeError`

Se lanza cuando hay un error de tipos en una operación.

```typescript
class AchronymeTypeError extends AchronymeError {
  readonly code: 'TYPE_ERROR';
  readonly expected?: string;
  readonly received?: string;
}
```

**Ejemplo:**
```typescript
try {
  const x = ach.number(42);
  await x.toMatrix(); // Tipo incorrecto
} catch (error) {
  if (error instanceof AchronymeTypeError) {
    console.error(`Esperaba ${error.expected}, recibió ${error.received}`);
  }
}
```

---

## Tipos de Valor

### Valores Escalares

```typescript
// Number
const x: AchronymeValue = ach.number(42);
const value: number = await x.toNumber();
```

### Vectores

```typescript
// Vector (array 1D)
const v: AchronymeValue = ach.vector([1, 2, 3, 4, 5]);
const arr: number[] = await v.toVector();
```

### Matrices

```typescript
// Matrix (array 2D)
const m: AchronymeValue = ach.matrix([
  [1, 2, 3],
  [4, 5, 6]
]);
const matrix: number[][] = await m.toMatrix();
```

### Números Complejos

```typescript
// Complex Number
const z: AchronymeValue = ach.complex(3, 4); // 3 + 4i
const complex: ComplexNumber = await z.toComplex();
// complex = { re: 3, im: 4 }
```

### Funciones (Lambdas)

```typescript
// Lambda Function
const square: AchronymeValue = ach.lambda(['x'], 'x ^ 2');

// Usar con map
const v = ach.vector([1, 2, 3, 4]);
const squared = ach.map(square, v);
// o con string inline
const squared2 = v.map('x => x ^ 2');
```

---

## Tipos Auxiliares

### `WasmModule`

Interface interna para el módulo WASM (no usar directamente).

```typescript
interface WasmModule {
  eval(expression: string): string;
  reset?(): void;
  listVariables?(): string;
}
```

### Type Guards

Funciones útiles para verificar tipos:

```typescript
// Verificar si un valor es AchronymeValue
function isAchronymeValue(value: any): value is AchronymeValue {
  return value instanceof AchronymeValue;
}

// Verificar si es un número
function isNumber(value: any): value is number {
  return typeof value === 'number' && !isNaN(value);
}

// Verificar si es un vector (array de números)
function isVector(value: any): value is number[] {
  return Array.isArray(value) && value.every(isNumber);
}

// Verificar si es una matriz (array 2D de números)
function isMatrix(value: any): value is number[][] {
  return Array.isArray(value) &&
         value.length > 0 &&
         value.every(isVector);
}

// Verificar si es un número complejo
function isComplexNumber(value: any): value is ComplexNumber {
  return typeof value === 'object' &&
         value !== null &&
         're' in value &&
         'im' in value &&
         isNumber(value.re) &&
         isNumber(value.im);
}
```

---

## Patrones de Uso

### Conversión de Tipos

```typescript
// Primitivo → AchronymeValue
const num: number = 42;
const value: AchronymeValue = ach.number(num);

// Array → Vector
const arr: number[] = [1, 2, 3];
const vector: AchronymeValue = ach.vector(arr);

// AchronymeValue → Primitivo
const backToNum: number = await value.toNumber();
const backToArr: number[] = await vector.toVector();
```

### Trabajar con Resultados

```typescript
// Operación asíncrona
const result = ach.vector([1, 2, 3]).mul(2);
const values = await result.toVector(); // [2, 4, 6]
result.dispose();

// Encadenamiento
const result2 = ach.number(5)
  .add(3)      // 8
  .mul(2)      // 16
  .pow(2);     // 256

const final = await result2.toNumber(); // 256
result2.dispose();
```

### Manejo de Variables

```typescript
// Variables con nombre
ach.let('x', 10);
ach.let('y', 20);

const sum = ach.get('x').add(ach.get('y'));
const result = await sum.toNumber(); // 30

sum.dispose();
// 'x' y 'y' permanecen en el entorno
```

---

## Tipos en Errores

### Captura Específica

```typescript
try {
  const result = await someOperation();
} catch (error) {
  if (error instanceof AchronymeNotInitializedError) {
    // SDK no inicializado
  } else if (error instanceof AchronymeSyntaxError) {
    // Error de sintaxis
    console.error(error.expression);
  } else if (error instanceof AchronymeTypeError) {
    // Error de tipo
    console.error(`Expected: ${error.expected}, Got: ${error.received}`);
  } else if (error instanceof AchronymeRuntimeError) {
    // Error en runtime
    console.error(error.message);
  } else {
    // Otro error
    console.error(error);
  }
}
```

### Captura Genérica

```typescript
try {
  const result = await someOperation();
} catch (error) {
  if (error instanceof AchronymeError) {
    // Cualquier error de Achronyme
    console.error(`Achronyme Error [${error.code}]: ${error.message}`);
  } else {
    // Error no relacionado con Achronyme
    throw error;
  }
}
```

---

## Ver También

- [API Reference](./api-reference.md) - Lista completa de funciones
- [Ejemplos](./examples.md) - Casos de uso prácticos
- [Gestión de Memoria](./memory-management.md) - Buenas prácticas
