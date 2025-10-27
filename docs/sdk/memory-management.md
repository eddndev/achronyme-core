# Gestión de Memoria - Achronyme SDK

Guía completa para gestionar memoria eficientemente en Achronyme.

## Tabla de Contenidos

- [Conceptos Fundamentales](#conceptos-fundamentales)
- [Ciclo de Vida de Variables](#ciclo-de-vida-de-variables)
- [Patrones de Gestión](#patrones-de-gestión)
- [Buenas Prácticas](#buenas-prácticas)
- [Anti-Patrones](#anti-patrones)
- [Debugging y Monitoreo](#debugging-y-monitoreo)
- [Casos Especiales](#casos-especiales)

---

## Conceptos Fundamentales

### ¿Por qué gestionar memoria?

Achronyme ejecuta código C++ a través de WebAssembly. Los valores creados viven en el entorno C++ y **no son recolectados automáticamente por el garbage collector de JavaScript**.

```typescript
// ❌ MAL: Sin limpieza
const x = ach.number(42);
const y = x.mul(2);
// x e y permanecen en memoria C++ indefinidamente
```

```typescript
// ✅ BIEN: Con limpieza
const x = ach.number(42);
const y = x.mul(2);
const result = await y.toNumber();
x.dispose();
y.dispose();
// Memoria liberada
```

### Modelo de Memoria

```
┌─────────────────────────────────────┐
│        JavaScript Heap              │
│                                     │
│  ┌────────────────────┐            │
│  │ AchronymeValue     │ (proxy)    │
│  │  _varName: "__v0"  │            │
│  └──────────┬─────────┘            │
└─────────────┼───────────────────────┘
              │ referencia
              ▼
┌─────────────────────────────────────┐
│    C++ WASM Memory (Manual)         │
│                                     │
│  Variables Environment:             │
│    __v0 = 42         ← debe liberar │
│    __v1 = 84         ← debe liberar │
│    myVar = [1,2,3]   ← debe liberar │
└─────────────────────────────────────┘
```

---

## Ciclo de Vida de Variables

### Creación

```typescript
// Crear valor (aloca memoria en C++)
const x = ach.number(42);

// El SDK internamente hace:
// 1. Genera nombre único: "__v0"
// 2. Evalúa: "let __v0 = 42"
// 3. Registra variable en tracking
```

### Uso

```typescript
// Usar valor (no aloca memoria adicional, solo lee)
const value = await x.toNumber();

// Operaciones (crean NUEVOS valores)
const y = x.add(10);  // Crea "__v1 = __v0 + 10"
const z = y.mul(2);   // Crea "__v2 = __v1 * 2"
```

### Disposición

```typescript
// Liberar memoria
x.dispose();
y.dispose();
z.dispose();

// El SDK internamente:
// 1. Marca variable como dispuesta
// 2. Elimina del tracking
// 3. NO elimina del entorno C++ (persiste hasta reset)
```

### Variables Persistentes

```typescript
// Variables con nombre PERSISTEN hasta reset()
ach.let('myVar', 42);

// Aunque no hay referencia en JS, 'myVar' existe en C++
// Para limpiarlo:
ach.reset();  // Limpia TODO el entorno
```

---

## Patrones de Gestión

### Patrón 1: Dispose Inmediato

**Mejor para:** Operaciones simples, valores temporales

```typescript
const x = ach.number(10);
const doubled = x.mul(2);
const result = await doubled.toNumber();

x.dispose();
doubled.dispose();

console.log(result);  // 20 (primitivo, OK)
```

### Patrón 2: Try-Finally

**Mejor para:** Garantizar limpieza incluso con errores

```typescript
const x = ach.number(10);
const y = ach.number(20);

try {
  const sum = x.add(y);
  const result = await sum.toNumber();
  sum.dispose();
  return result;
} finally {
  // Siempre se ejecuta
  x.dispose();
  y.dispose();
}
```

### Patrón 3: Acumular y Limpiar

**Mejor para:** Múltiples operaciones relacionadas

```typescript
function processData(data: number[]) {
  const values: AchronymeValue[] = [];

  try {
    const v = ach.vector(data);
    values.push(v);

    const squared = v.map('x => x^2');
    values.push(squared);

    const sum = ach.sum(squared);
    values.push(sum);

    const result = await sum.toNumber();
    return result;
  } finally {
    // Limpiar todo al final
    values.forEach(v => v.dispose());
  }
}
```

### Patrón 4: RAII (Resource Acquisition Is Initialization)

**Mejor para:** APIs reutilizables

```typescript
async function withValue<T>(
  value: AchronymeValue,
  fn: (v: AchronymeValue) => Promise<T>
): Promise<T> {
  try {
    return await fn(value);
  } finally {
    value.dispose();
  }
}

// Uso
const result = await withValue(
  ach.vector([1, 2, 3, 4, 5]),
  async (v) => {
    const squared = v.map('x => x^2');
    const sum = await ach.sum(squared).toNumber();
    squared.dispose();
    return sum;
  }
);
```

### Patrón 5: Clase Wrapper con Dispose

**Mejor para:** Operaciones complejas, APIs orientadas a objetos

```typescript
class Signal {
  private value: AchronymeValue;
  private disposed = false;

  constructor(private ach: Achronyme, data: number[]) {
    this.value = ach.vector(data);
  }

  async computeFFT() {
    this.checkDisposed();
    const spectrum = this.ach.fft_mag(this.value);
    const result = await spectrum.toVector();
    spectrum.dispose();
    return result;
  }

  dispose() {
    if (!this.disposed) {
      this.value.dispose();
      this.disposed = true;
    }
  }

  private checkDisposed() {
    if (this.disposed) {
      throw new Error('Signal already disposed');
    }
  }
}

// Uso
const signal = new Signal(ach, [1, 2, 3, 4, 5]);
try {
  const spectrum = await signal.computeFFT();
  console.log(spectrum);
} finally {
  signal.dispose();
}
```

---

## Buenas Prácticas

### ✅ DO: Extraer Valores Primitivos

```typescript
// ✅ BIEN: Extraer a primitivo
const x = ach.number(42);
const value = await x.toNumber();
x.dispose();

// Ahora 'value' es un number de JS (GC automático)
console.log(value * 2);
```

### ✅ DO: Dispose en Orden Inverso

```typescript
// ✅ BIEN: Último creado, primero liberado
const a = ach.number(1);
const b = ach.number(2);
const c = a.add(b);

c.dispose();  // Primero el resultado
b.dispose();
a.dispose();  // Último los operandos
```

### ✅ DO: Usar Try-Finally

```typescript
// ✅ BIEN: Garantiza limpieza
const x = ach.vector([1, 2, 3]);
try {
  const result = await processVector(x);
  return result;
} finally {
  x.dispose();
}
```

### ✅ DO: Monitorear Memoria

```typescript
// ✅ BIEN: Revisar estadísticas
setInterval(() => {
  const stats = ach.getMemoryStats();
  if (stats.activeVariables > 1000) {
    console.warn('Alto uso de memoria:', stats);
  }
}, 5000);
```

### ✅ DO: Variables con Nombres Significativos

```typescript
// ✅ BIEN: Nombres descriptivos para variables persistentes
ach.let('sampleRate', 1000);
ach.let('signalData', [1, 2, 3, 4, 5]);
ach.let('windowFunction', 'hanning');

// Fácil de debuggear
const vars = ach.getMemoryStats().variableNames;
console.log(vars);  // ['sampleRate', 'signalData', 'windowFunction']
```

---

## Anti-Patrones

### ❌ DON'T: Olvidar Dispose

```typescript
// ❌ MAL: Fuga de memoria
function leakyFunction() {
  const x = ach.number(42);
  const y = x.mul(2);
  return y.toNumber();  // x e y nunca se liberan
}

// Llamar 1000 veces → 2000 variables en memoria
```

### ❌ DON'T: Dispose Doble

```typescript
// ❌ MAL: Double dispose
const x = ach.number(42);
x.dispose();
x.dispose();  // Error: Variable already disposed
```

### ❌ DON'T: Usar Después de Dispose

```typescript
// ❌ MAL: Usar valor dispuesto
const x = ach.number(42);
x.dispose();
const y = x.mul(2);  // Error: Cannot use disposed value
```

### ❌ DON'T: Acumular Variables Anónimas

```typescript
// ❌ MAL: Muchas variables temporales
for (let i = 0; i < 10000; i++) {
  ach.number(i);  // Crea __v0, __v1, ..., __v9999
}
// 10000 variables sin dispose
```

### ❌ DON'T: Variables Globales Sin Limpieza

```typescript
// ❌ MAL: Variables globales persistentes
ach.let('data', largeArray);  // Permanece hasta reset()
ach.let('temp', intermediateResult);  // Nunca se limpia

// Mejor: usar variables temporales con dispose
```

---

## Debugging y Monitoreo

### Ver Variables Activas

```typescript
const stats = ach.getMemoryStats();

console.log('Total creadas:', stats.totalVariables);
console.log('Activas:', stats.activeVariables);
console.log('Dispuestas:', stats.disposedVariables);
console.log('Nombres:', stats.variableNames);
```

### Detectar Fugas

```typescript
class MemoryTracker {
  private baseline = 0;

  setBaseline() {
    this.baseline = ach.getMemoryStats().activeVariables;
  }

  checkLeak(threshold = 100) {
    const current = ach.getMemoryStats().activeVariables;
    const leaked = current - this.baseline;

    if (leaked > threshold) {
      console.warn(`Posible fuga: ${leaked} variables sin liberar`);
      console.warn('Variables:', ach.getMemoryStats().variableNames);
    }
  }
}

// Uso
const tracker = new MemoryTracker();

tracker.setBaseline();
// ... operaciones ...
tracker.checkLeak();
```

### Modo Debug

```typescript
const ach = new Achronyme({ debug: true });
await ach.init();

// Ahora todas las operaciones se loguean:
// [Achronyme] eval: let __v0 = 42 => 42
// [Achronyme] eval: let __v1 = __v0 * 2 => 84
// [Achronyme] Disposed variable: __v0
```

### Estadísticas en Tiempo Real

```typescript
function printMemoryReport() {
  const stats = ach.getMemoryStats();

  console.log('═══════════════════════════════');
  console.log('  MEMORY REPORT');
  console.log('═══════════════════════════════');
  console.log(`  Total created:  ${stats.totalVariables}`);
  console.log(`  Active:         ${stats.activeVariables}`);
  console.log(`  Disposed:       ${stats.disposedVariables}`);
  console.log(`  Efficiency:     ${(stats.disposedVariables / stats.totalVariables * 100).toFixed(1)}%`);
  console.log('───────────────────────────────');

  if (stats.activeVariables > 0) {
    console.log('  Active variables:');
    stats.variableNames.forEach(name => {
      console.log(`    - ${name}`);
    });
  }

  console.log('═══════════════════════════════');
}

// Llamar periódicamente
setInterval(printMemoryReport, 10000);
```

---

## Casos Especiales

### Variables Persistentes Intencionadas

```typescript
// Configuración global (intencional)
ach.let('config', {
  sampleRate: 1000,
  windowSize: 512
});

// Permanece hasta reset(), esto es OK
```

### Long-Running Applications

```typescript
class DataProcessor {
  private ach: Achronyme;
  private processedCount = 0;

  constructor() {
    this.ach = new Achronyme({
      debug: false,
      maxVariables: 1000
    });
  }

  async init() {
    await this.ach.init();
  }

  async processChunk(data: number[]) {
    const values: AchronymeValue[] = [];

    try {
      const signal = this.ach.vector(data);
      values.push(signal);

      const processed = signal.map('x => x^2');
      values.push(processed);

      const result = await processed.toVector();
      this.processedCount++;

      return result;
    } finally {
      values.forEach(v => v.dispose());
    }
  }

  periodicCleanup() {
    const stats = this.ach.getMemoryStats();

    // Si hay acumulación, hacer reset completo
    if (stats.activeVariables > 500) {
      console.warn('Haciendo reset por alto uso de memoria');
      this.ach.reset();
    }
  }
}

// Uso
const processor = new DataProcessor();
await processor.init();

// Procesar datos continuamente
setInterval(() => {
  processor.periodicCleanup();
}, 60000);
```

### Manejo de Grandes Datasets

```typescript
async function processBigData(data: number[][]) {
  const results: number[][] = [];

  // Procesar en chunks para evitar acumulación
  for (const chunk of data) {
    const signal = ach.vector(chunk);

    try {
      const spectrum = ach.fft_mag(signal);
      const result = await spectrum.toVector();
      results.push(result);

      spectrum.dispose();
    } finally {
      signal.dispose();
    }

    // Opcional: garbage collect de JS cada N chunks
    if (results.length % 100 === 0) {
      await new Promise(resolve => setTimeout(resolve, 0));
    }
  }

  return results;
}
```

### Reset Estratégico

```typescript
// Escenario: Aplicación con múltiples fases

class Application {
  private ach: Achronyme;

  async init() {
    this.ach = new Achronyme();
    await this.ach.init();
  }

  async phase1() {
    // Fase 1: Configuración
    this.ach.let('config', 100);
    // ... operaciones ...
  }

  async phase2() {
    // Fase 2: Procesamiento
    // ... operaciones ...
  }

  async phase3() {
    // Fase 3: Análisis final
    // ... operaciones ...
  }

  async run() {
    await this.phase1();
    await this.phase2();

    // Reset antes de fase final
    // Limpia todo pero reinicializa el entorno
    this.ach.reset();

    await this.phase3();
  }
}
```

---

## Checklist de Memoria

### Antes de Desplegar

- [ ] Todas las funciones llaman a `dispose()` en valores temporales
- [ ] Se usa `try-finally` para garantizar limpieza
- [ ] Variables con nombre son intencionales y documentadas
- [ ] Hay monitoreo de memoria en producción
- [ ] Tests de fugas de memoria pasan
- [ ] No hay bucles que creen variables sin dispose

### En Desarrollo

- [ ] Modo debug habilitado durante testing
- [ ] Se revisan estadísticas periódicamente
- [ ] Se usan herramientas de profiling
- [ ] Se documentan decisiones de memoria

### En Producción

- [ ] Monitoreo activo de `getMemoryStats()`
- [ ] Alertas configuradas para alto uso
- [ ] Plan de reset periódico si aplica
- [ ] Logs de errores de disposición

---

## Recursos Adicionales

- [API Reference](./api-reference.md) - Documentación de `dispose()` y `reset()`
- [Ejemplos](./examples.md) - Patrones de gestión en práctica
- [Tipos](./types.md) - Interface `MemoryStats`

---

## Resumen

**Regla de Oro:** Toda variable creada debe eventualmente ser dispuesta o el entorno debe resetearse.

```typescript
// Patrón básico para toda operación:
const value = ach.someOperation();
try {
  // Usar value
  const result = await value.toSomething();
  return result;
} finally {
  value.dispose();
}
```

Con estas prácticas, tu aplicación Achronyme será eficiente, predecible y libre de fugas de memoria. 🎯
