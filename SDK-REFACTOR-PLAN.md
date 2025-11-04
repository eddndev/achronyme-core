# 🔄 Achronyme SDK Refactor Plan - Rust WASM Migration

**Fecha:** 2025-11-03
**Versión SDK Actual:** 0.4.0 (C++/Emscripten)
**Versión SDK Target:** 1.0.0 (Rust WASM)
**Status:** 📋 Planning → Implementation

---

## 📊 EXECUTIVE SUMMARY

### Objetivo Principal
Migrar el SDK TypeScript de C++/Emscripten a Rust WASM, eliminando memory leaks, mejorando performance, y modernizando la arquitectura con session-based management y zero-copy views.

### Resultados Esperados
- ✅ **Zero Memory Leaks** - Auto-cleanup con FinalizationRegistry
- ✅ **50%+ Performance Boost** - Zero-copy views + Rust optimizations
- ✅ **Better DX** - No dispose() manual, type-safe API
- ✅ **Maintainability** - Modular architecture (<300 LOC/file)
- ✅ **Future-proof** - 100% Rust (no C++ dependencies)

---

## 🔍 ANÁLISIS DEL ESTADO ACTUAL

### Arquitectura C++/Emscripten (v0.4.0)

```
src/sdk/
├── Achronyme.ts          # 1771 LOC ❌ MONOLÍTICO
├── AchronymeValue.ts     # 573 LOC
├── types.ts              # 258 LOC
├── errors.ts             # 116 LOC ✅ KEEP
├── utils.ts              # 254 LOC
└── index.ts              # 50 LOC
```

### ✅ Fortalezas Actuales

1. **Dual Path System** - Implementado correctamente
   ```typescript
   // Fast Path (handles): ≥8 elementos
   const v = ach.vector([...100 elementos...]); // Uses createVectorFromBuffer

   // Slow Path (parser): <8 elementos
   const v = ach.vector([1, 2, 3]); // Uses string expression
   ```

2. **Comprehensive API** - 78 funciones
   - Math: sin, cos, tan, exp, ln, abs, sqrt (28 funcs)
   - DSP: fft, ifft, conv, window funcs (12 funcs)
   - Linalg: LU, QR, SVD, Cholesky, eigen (18 funcs)
   - Vector ops: vadd, vmul, dot, norm (8 funcs)
   - HOF: map, filter, reduce, pipe (4 funcs)
   - Stats: sum, mean, std (3 funcs)
   - Utils: linspace, fftshift, identity (5 funcs)

3. **Fluent API** - Chainable y ergonómica
   ```typescript
   const result = ach.vector([1,2,3,4])
       .sin()
       .fft_mag()
       .toVector();
   ```

4. **Error Handling** - 6 custom error types bien diseñados

### ❌ Problemas Críticos

#### 1. Memory Management Manual (CRÍTICO)

**Problema:**
```typescript
// ❌ ACTUAL: Usuario DEBE recordar dispose()
const x = ach.vector([1, 2, 3]);
const y = x.sin();
x.dispose(); // Si olvida → memory leak!
y.dispose(); // Si olvida → memory leak!

// En un loop → disaster
for (let i = 0; i < 1000; i++) {
    const v = ach.vector([...]);
    const result = v.fft_mag(); // ❌ 2000 handles sin liberar!
    // Usuario olvidó dispose()
}
```

**Impacto:**
- Memory leaks inevitables en producción
- Handles huérfanos acumulándose
- Crashes en apps long-running
- DX terrible (usuario siempre preocupado por dispose)

**Evidencia:**
```typescript
// Estado actual del HandleManager
class Achronyme {
    private handleToVar: Map<Handle, string> = new Map();
    private varToHandle: Map<string, Handle> = new Map();
    // ❌ NO auto-cleanup → crece indefinidamente
}
```

#### 2. Zero-Copy Parcial (PERFORMANCE)

**Problema:**
```typescript
// ❌ toVector() SIEMPRE copia
async toVector(): Promise<number[]> {
    // Fast path attempt
    const view = module.HEAPF64.subarray(ptr / 8, ptr / 8 + length);
    return Array.from(view); // ❌ COPIA innecesaria!
    // Para 10M elementos → ~80ms overhead
}
```

**Impacto:**
- 10M elementos: **80ms de overhead** solo copiando
- No se aprovecha el Fast Path completamente
- WebGL/Canvas ops requieren re-upload

**Benchmark:**
```
Dataset: 10M elementos (76.3 MB)
- Zero-copy view:  ~0ms
- Array.from():    ~80ms  ❌
- Throughput lost: 952 MB/s
```

#### 3. Monolithic Architecture (MAINTAINABILITY)

**Problema:**
```typescript
// Achronyme.ts: 1771 líneas 😱
export class Achronyme {
    // Initialization (67 LOC)
    async init() { ... }

    // Memory management (95 LOC)
    _allocFloat64() { ... }
    _createVectorFast() { ... }

    // Type constructors (68 LOC)
    vector() { ... }
    matrix() { ... }

    // Math functions (350 LOC)
    sin() { ... }
    cos() { ... }
    // ... 26+ more math funcs

    // DSP functions (180 LOC)
    fft() { ... }
    conv() { ... }

    // Linalg (390 LOC)
    lu() { ... }
    qr() { ... }
    svd() { ... }

    // Vector ops (120 LOC)
    vadd() { ... }

    // HOF (80 LOC)
    map() { ... }

    // Stats (45 LOC)
    sum() { ... }

    // Utils (100 LOC)
    linspace() { ... }

    // Constants (76 LOC)
    get PI() { ... }
}
```

**Impacto:**
- Difícil navegar y mantener
- Testing complejo (todo acoplado)
- Merge conflicts frecuentes
- No tree-shaking efectivo

#### 4. Type Safety Limitado

**Problema:**
```typescript
// ❌ any everywhere
export interface LUResult {
    L: any; // Debería ser Matrix
    U: any;
    P: any;
}

export interface SVDResult {
    U: any;
    S: any; // Debería ser Vector
    V: any;
}

// ❌ No type inference
const result = ach.map(fn, vector); // result type = AchronymeValue (no info)
```

**Impacto:**
- No type safety en resultados
- Errores en runtime vs compile time
- Auto-complete pobre en IDEs

#### 5. Emscripten Dependencies

**Problema:**
```typescript
// Dependencia fuerte en Emscripten APIs
private _allocFloat64(data: ArrayLike<number>): number {
    const ptr = this.module._malloc(byteLength); // Emscripten
    const heap = this.module.HEAPF64; // Emscripten
    // ...
}

// Incompatible con wasm-bindgen (Rust)
```

**Impacto:**
- Acoplamiento a C++ runtime
- No puede usar Rust WASM directamente
- Migration path bloqueado

---

## 🎯 ARQUITECTURA PROPUESTA

### Estructura Modular

```
src/sdk/
├── core/
│   ├── Session.ts           # ⭐ Session con auto-cleanup (250 LOC)
│   ├── HandleManager.ts     # ⭐ Gestión centralizada + GC (200 LOC)
│   ├── MemoryPool.ts        # Pool de buffers reutilizables (150 LOC)
│   ├── FastPath.ts          # Fast path operations (180 LOC)
│   └── SlowPath.ts          # Parser fallback (120 LOC)
├── values/
│   ├── Value.ts             # Base class abstracta (100 LOC)
│   ├── Scalar.ts            # Numbers (80 LOC)
│   ├── Vector.ts            # ⭐ Zero-copy views (220 LOC)
│   ├── Matrix.ts            # Matrix ops (250 LOC)
│   └── Complex.ts           # Complex numbers (90 LOC)
├── operations/
│   ├── MathOps.ts           # sin, cos, exp, etc. (280 LOC)
│   ├── DSPOps.ts            # fft, conv, window (200 LOC)
│   ├── LinalgOps.ts         # Decompositions (300 LOC)
│   ├── VectorOps.ts         # vadd, vmul, dot (150 LOC)
│   ├── HOFOps.ts            # map, filter, reduce (120 LOC)
│   └── StatsOps.ts          # sum, mean, std (80 LOC)
├── wasm/
│   ├── RustBindings.ts      # ⭐ Rust WASM bindings (250 LOC)
│   └── TypeConverters.ts    # JS ↔ Rust conversions (100 LOC)
├── Achronyme.ts             # ⭐ Facade simplificado (300 LOC)
├── types.ts                 # Types compartidos (300 LOC)
├── errors.ts                # ✅ KEEP AS-IS (116 LOC)
├── utils.ts                 # Helpers (150 LOC)
└── index.ts                 # Public API (80 LOC)

Total: ~3,900 LOC (vs 3,022 LOC actual)
Pero: Modular, testable, maintainable
```

### Características Clave

#### 1️⃣ Session-Based Management

```typescript
/**
 * Session con auto-cleanup
 * Gestiona lifetime de todos los valores creados
 */
class AchronymeSession {
    private handleManager: HandleManager;
    private finalizationRegistry: FinalizationRegistry<Handle>;
    private values = new Set<WeakRef<Value>>();

    /**
     * Scope-based cleanup (RAII style)
     */
    async use<T>(fn: (session: AchronymeSession) => Promise<T>): Promise<T> {
        try {
            return await fn(this);
        } finally {
            this.cleanup(); // Auto cleanup al salir
        }
    }

    /**
     * Manual cleanup (si se necesita)
     */
    cleanup(): void {
        for (const ref of this.values) {
            const value = ref.deref();
            if (value) {
                value.dispose();
            }
        }
        this.values.clear();
    }

    /**
     * Create vector con auto-tracking
     */
    vector(data: number[]): Vector {
        const handle = this.wasm.createVector(data);
        const vec = new Vector(this, handle);

        // Auto-track para cleanup
        this.values.add(new WeakRef(vec));

        // Auto-release cuando GC'd
        this.finalizationRegistry.register(vec, handle, vec);

        return vec;
    }
}

// USO:
await session.use(async (s) => {
    const signal = s.vector([...10M elementos...]);
    const spectrum = signal.fft_mag();

    // ✅ NO dispose() manual!
    // ✅ Auto cleanup al salir del scope
});
```

**Beneficios:**
- ✅ Zero memory leaks
- ✅ RAII-style lifetime management
- ✅ Compatible con async/await
- ✅ Scope-based cleanup automático

#### 2️⃣ Zero-Copy Views

```typescript
/**
 * Vector con zero-copy TypedArray view
 */
class Vector extends Value {
    /**
     * Direct view sobre WASM memory (zero-copy)
     * ⚠️ Válido solo mientras el handle existe
     */
    get data(): Float64Array {
        this.checkDisposed();

        const ptr = this.wasm.getVectorDataPtr(this.handle);
        const len = this.wasm.getVectorLength(this.handle);

        // ✅ Zero-copy! No allocation
        return this.wasm.HEAPF64.subarray(ptr / 8, ptr / 8 + len);
    }

    /**
     * Copy to JavaScript array (explicit)
     */
    toArray(): number[] {
        return Array.from(this.data);
    }

    /**
     * Get element (bounds-checked)
     */
    get(index: number): number {
        const data = this.data;
        if (index < 0 || index >= data.length) {
            throw new RangeError(`Index ${index} out of bounds [0, ${data.length})`);
        }
        return data[index];
    }

    /**
     * Set element (bounds-checked)
     */
    set(index: number, value: number): void {
        const data = this.data;
        if (index < 0 || index >= data.length) {
            throw new RangeError(`Index ${index} out of bounds [0, ${data.length})`);
        }
        data[index] = value;
    }

    /**
     * Iterate (zero-copy)
     */
    *[Symbol.iterator]() {
        const data = this.data;
        for (let i = 0; i < data.length; i++) {
            yield data[i];
        }
    }
}

// USO:
const v = session.vector([...10M elementos...]);

// ✅ Zero-copy: Instantáneo
const view = v.data; // Float64Array view
for (const x of v) { console.log(x); } // Iterator zero-copy

// ✅ Explicit copy cuando se necesita
const arr = v.toArray(); // Copy to JS array

// ✅ Direct WebGL/Canvas upload
gl.bufferData(gl.ARRAY_BUFFER, v.data, gl.STATIC_DRAW);
```

**Benchmark Comparison:**
```
Operation: Get vector data (10M elements)
┌─────────────────────┬──────────┬────────────┐
│ Method              │ Time     │ Throughput │
├─────────────────────┼──────────┼────────────┤
│ toArray() (actual)  │ 80ms     │ 952 MB/s   │
│ .data (proposed)    │ <1ms     │ ∞ (view)   │
└─────────────────────┴──────────┴────────────┘
Speedup: 80x+ faster! 🚀
```

**Beneficios:**
- ✅ **80x+ faster** para datasets grandes
- ✅ **WebGL/Canvas compatible** - direct upload
- ✅ **Explicit vs implicit** - clarity

#### 3️⃣ HandleManager Centralizado

```typescript
/**
 * Gestión centralizada de handles con auto-cleanup
 */
class HandleManager {
    private handles = new Map<Handle, WeakRef<Value>>();
    private registry: FinalizationRegistry<Handle>;
    private allocatedCount = 0;
    private freedCount = 0;

    constructor(private wasm: RustWASM) {
        // Auto-release handles cuando Value es GC'd
        this.registry = new FinalizationRegistry((handle) => {
            this.release(handle);
        });
    }

    /**
     * Register handle con auto-cleanup
     */
    register(handle: Handle, value: Value): void {
        this.handles.set(handle, new WeakRef(value));
        this.registry.register(value, handle, value);
        this.allocatedCount++;
    }

    /**
     * Release handle
     */
    release(handle: Handle): void {
        const ref = this.handles.get(handle);
        if (!ref) return;

        // Unregister from finalization
        const value = ref.deref();
        if (value) {
            this.registry.unregister(value);
        }

        // Release in WASM
        try {
            this.wasm.releaseHandle(handle);
            this.freedCount++;
        } catch (e) {
            console.warn(`Failed to release handle ${handle}:`, e);
        }

        this.handles.delete(handle);
    }

    /**
     * Get value from handle (if still alive)
     */
    get(handle: Handle): Value | undefined {
        const ref = this.handles.get(handle);
        return ref?.deref();
    }

    /**
     * Memory statistics
     */
    getStats(): HandleStats {
        let activeHandles = 0;
        for (const ref of this.handles.values()) {
            if (ref.deref()) activeHandles++;
        }

        return {
            allocated: this.allocatedCount,
            freed: this.freedCount,
            active: activeHandles,
            leaked: this.allocatedCount - this.freedCount - activeHandles,
        };
    }

    /**
     * Force cleanup of all dead handles
     */
    gc(): number {
        let cleaned = 0;
        for (const [handle, ref] of this.handles) {
            if (!ref.deref()) {
                this.release(handle);
                cleaned++;
            }
        }
        return cleaned;
    }
}
```

**Beneficios:**
- ✅ **Auto-cleanup** con WeakRef + FinalizationRegistry
- ✅ **Memory leak detection** - track allocated vs freed
- ✅ **Force GC** - manual cleanup si necesario
- ✅ **Debugging** - stats detallados

#### 4️⃣ Modular Operations

```typescript
// ============================================================================
// MathOps.ts - Mathematical operations
// ============================================================================
export class MathOps {
    constructor(private session: AchronymeSession) {}

    /**
     * Sine function
     * Auto-detects fast path
     */
    sin(x: Value | number): Value {
        if (typeof x === 'number') {
            return this.session.scalar(Math.sin(x));
        }

        // ✅ SIEMPRE usa fast path
        const handle = this.session.wasm.sin_fast(x.handle);
        return this.session.createFromHandle(handle);
    }

    cos(x: Value | number): Value { /* similar */ }
    tan(x: Value | number): Value { /* similar */ }
    exp(x: Value | number): Value { /* similar */ }
    // ... 24 more math functions
}

// ============================================================================
// DSPOps.ts - Digital Signal Processing
// ============================================================================
export class DSPOps {
    constructor(private session: AchronymeSession) {}

    /**
     * Fast Fourier Transform
     * Returns complex spectrum as matrix [N x 2]
     */
    fft(signal: Vector): Matrix {
        const handle = this.session.wasm.fft_fast(signal.handle);
        return new Matrix(this.session, handle);
    }

    /**
     * FFT Magnitude spectrum
     */
    fftMag(signal: Vector): Vector {
        const handle = this.session.wasm.fft_mag_fast(signal.handle);
        return new Vector(this.session, handle);
    }

    conv(s1: Vector, s2: Vector): Vector { /* ... */ }
    // ... 10 more DSP functions
}

// ============================================================================
// LinalgOps.ts - Linear Algebra
// ============================================================================
export class LinalgOps {
    constructor(private session: AchronymeSession) {}

    /**
     * LU Decomposition: PA = LU
     * @returns Type-safe result with Matrix types
     */
    lu(matrix: Matrix): LUResult {
        const result = this.session.wasm.lu_decomposition_js(matrix.handle);

        return {
            L: new Matrix(this.session, result.L),
            U: new Matrix(this.session, result.U),
            P: new Matrix(this.session, result.P),
        };
    }

    qr(matrix: Matrix): QRResult { /* ... */ }
    svd(matrix: Matrix): SVDResult { /* ... */ }
    // ... 15 more linalg functions
}

// ============================================================================
// Achronyme.ts - Facade simplificado
// ============================================================================
export class Achronyme {
    private session: AchronymeSession;

    // Operation modules
    readonly math: MathOps;
    readonly dsp: DSPOps;
    readonly linalg: LinalgOps;
    readonly vector: VectorOps;
    readonly hof: HOFOps;
    readonly stats: StatsOps;

    constructor() {
        this.session = new AchronymeSession();

        // Initialize modules
        this.math = new MathOps(this.session);
        this.dsp = new DSPOps(this.session);
        this.linalg = new LinalgOps(this.session);
        this.vector = new VectorOps(this.session);
        this.hof = new HOFOps(this.session);
        this.stats = new StatsOps(this.session);
    }

    // ========================================================================
    // Convenience methods (forward to modules)
    // ========================================================================

    sin(x: Value | number): Value {
        return this.math.sin(x);
    }

    fft(signal: Vector): Matrix {
        return this.dsp.fft(signal);
    }

    // ... etc

    // ========================================================================
    // Type constructors
    // ========================================================================

    vector(data: number[]): Vector {
        return this.session.vector(data);
    }

    matrix(data: number[][]): Matrix {
        return this.session.matrix(data);
    }

    // ========================================================================
    // Session management
    // ========================================================================

    async use<T>(fn: (ach: Achronyme) => Promise<T>): Promise<T> {
        return this.session.use(() => fn(this));
    }

    cleanup(): void {
        this.session.cleanup();
    }
}
```

**File Size Comparison:**
```
ANTES:
  Achronyme.ts: 1771 LOC (monolithic)

DESPUÉS:
  MathOps.ts:    280 LOC
  DSPOps.ts:     200 LOC
  LinalgOps.ts:  300 LOC
  VectorOps.ts:  150 LOC
  HOFOps.ts:     120 LOC
  StatsOps.ts:    80 LOC
  Achronyme.ts:  300 LOC (facade)
  ──────────────────────
  Total:        1430 LOC ✅ 19% reduction

Benefits:
  - Each file <300 LOC
  - Easy to navigate
  - Testable independently
  - Tree-shakeable
```

#### 5️⃣ Type-Safe Results

```typescript
// ANTES:
export interface LUResult {
    L: any; // ❌
    U: any;
    P: any;
}

// DESPUÉS:
export interface LUResult {
    L: Matrix;
    U: Matrix;
    P: Matrix;
}

export interface SVDResult {
    U: Matrix;
    S: Vector; // ✅ Type-safe!
    V: Matrix;
}

// Con generics para flexibilidad
class Vector<T extends number = number> {
    /**
     * Map with type inference
     */
    map<U extends number>(fn: (x: T, i: number) => U): Vector<U> {
        const result = new Float64Array(this.length);
        const data = this.data;
        for (let i = 0; i < data.length; i++) {
            result[i] = fn(data[i], i);
        }
        return this.session.vector(Array.from(result)) as Vector<U>;
    }
}

// USO:
const v = ach.vector([1, 2, 3, 4]);
const squared = v.map(x => x * x); // Type: Vector<number>
const result = await linalg.svd(matrix);
result.S // Type: Vector ✅ (no 'any')
```

---

## 📋 FASES DE IMPLEMENTACIÓN

### **FASE 1: Core Infrastructure** ⏱️ 3-4 días

**Objetivo:** Establecer la base modular y session management

#### Tareas:

- [ ] **1.1 Session.ts** (250 LOC)
  - [ ] AchronymeSession class
  - [ ] use() method con try/finally
  - [ ] cleanup() method
  - [ ] Value tracking con WeakRef
  - [ ] Tests unitarios

- [ ] **1.2 HandleManager.ts** (200 LOC)
  - [ ] FinalizationRegistry setup
  - [ ] register() / release() methods
  - [ ] getStats() method
  - [ ] gc() force cleanup
  - [ ] Tests unitarios

- [ ] **1.3 MemoryPool.ts** (150 LOC)
  - [ ] Buffer pool para reusar memoria
  - [ ] acquire() / release() API
  - [ ] Auto-resize strategy
  - [ ] Tests unitarios

- [ ] **1.4 RustBindings.ts** (250 LOC)
  - [ ] Import Rust WASM module
  - [ ] Type definitions para todas las funciones
  - [ ] Error handling wrapper
  - [ ] Tests de integración

**Criterios de Éxito:**
```typescript
// ✅ Este código debe funcionar:
const session = new AchronymeSession();
await session.use(async () => {
    const handle = session.wasm.createVector([1, 2, 3]);
    // Auto-cleanup al salir
});

// ✅ Memory stats correctos:
const stats = session.handleManager.getStats();
assert(stats.leaked === 0);
```

---

### **FASE 2: Value Types** ⏱️ 4-5 días

**Objetivo:** Implementar tipos de valores con zero-copy

#### Tareas:

- [ ] **2.1 Value.ts** (100 LOC)
  - [ ] Abstract base class
  - [ ] handle property
  - [ ] dispose() method
  - [ ] checkDisposed() helper
  - [ ] Metadata tracking

- [ ] **2.2 Vector.ts** (220 LOC)
  - [ ] Zero-copy data getter
  - [ ] toArray() explicit copy
  - [ ] get/set with bounds checking
  - [ ] Iterator implementation
  - [ ] length property
  - [ ] Tests exhaustivos

- [ ] **2.3 Matrix.ts** (250 LOC)
  - [ ] Zero-copy data getter
  - [ ] row() / col() accessors
  - [ ] get/set 2D indexing
  - [ ] rows/cols properties
  - [ ] Tests exhaustivos

- [ ] **2.4 Scalar.ts** (80 LOC)
  - [ ] Simple number wrapper
  - [ ] value getter
  - [ ] valueOf() para coercion
  - [ ] Tests

- [ ] **2.5 Complex.ts** (90 LOC)
  - [ ] re/im properties
  - [ ] magnitude/phase getters
  - [ ] toString() formatting
  - [ ] Tests

**Criterios de Éxito:**
```typescript
// ✅ Zero-copy debe funcionar:
const v = session.vector([...10_000_000]);
const view = v.data; // <1ms
assert(view instanceof Float64Array);

// ✅ Iteration debe funcionar:
let sum = 0;
for (const x of v) {
    sum += x;
}

// ✅ Bounds checking:
assertThrows(() => v.get(-1));
assertThrows(() => v.get(10_000_000));
```

---

### **FASE 3: Operations Modules** ⏱️ 5-6 días

**Objetivo:** Migrar todas las operaciones a módulos separados

#### Tareas:

- [ ] **3.1 MathOps.ts** (280 LOC)
  - [ ] Trigonometric: sin, cos, tan, asin, acos, atan, atan2
  - [ ] Hyperbolic: sinh, cosh, tanh
  - [ ] Exponential: exp, ln, log, log10, log2, pow
  - [ ] Rounding: floor, ceil, round, trunc
  - [ ] Other: sqrt, cbrt, abs, sign
  - [ ] Tests para cada función

- [ ] **3.2 DSPOps.ts** (200 LOC)
  - [ ] FFT: fft, fftMag, fftPhase, ifft
  - [ ] DFT: dft, dftMag, dftPhase
  - [ ] Convolution: conv, convFFT
  - [ ] Windows: hanning, hamming, blackman
  - [ ] Utils: fftshift, ifftshift, fftSpectrum
  - [ ] Tests para cada función

- [ ] **3.3 LinalgOps.ts** (300 LOC)
  - [ ] Decompositions: lu, qr, cholesky, svd
  - [ ] Eigenvalues: powerIteration, eigenvalues, eig
  - [ ] Utils: isSymmetric, isPositiveDefinite, identity
  - [ ] Matrix ops: det, inverse, transpose
  - [ ] Tests para cada función

- [ ] **3.4 VectorOps.ts** (150 LOC)
  - [ ] Arithmetic: vadd, vsub, vmul, vdiv, vscale
  - [ ] Products: dot, cross
  - [ ] Norms: norm, normL1
  - [ ] Tests para cada función

- [ ] **3.5 HOFOps.ts** (120 LOC)
  - [ ] map, filter, reduce
  - [ ] pipe, compose
  - [ ] Tests para cada función

- [ ] **3.6 StatsOps.ts** (80 LOC)
  - [ ] sum, mean, std
  - [ ] min, max
  - [ ] Tests para cada función

**Criterios de Éxito:**
```typescript
// ✅ Cada módulo debe ser independiente:
const mathOps = new MathOps(session);
const result = mathOps.sin(vector);

// ✅ Fast path debe ser default:
const v = session.vector([...1000]);
const sinV = mathOps.sin(v);
assert(sinV.metadata.usedFastPath === true);

// ✅ Type safety:
const { L, U, P } = linalgOps.lu(matrix);
assert(L instanceof Matrix);
```

---

### **FASE 4: Facade & Integration** ⏱️ 2-3 días

**Objetivo:** Crear API unificada y backward compatibility

#### Tareas:

- [ ] **4.1 Achronyme.ts Facade** (300 LOC)
  - [ ] Constructor con module initialization
  - [ ] Property accessors: math, dsp, linalg, etc.
  - [ ] Convenience methods forwarding
  - [ ] Type constructors: vector(), matrix(), etc.
  - [ ] Session management: use(), cleanup()
  - [ ] Tests de integración

- [ ] **4.2 Backward Compatibility Layer** (200 LOC)
  - [ ] Legacy API wrapper
  - [ ] Auto-dispose() when using legacy API
  - [ ] Migration guide
  - [ ] Deprecation warnings

- [ ] **4.3 Documentation**
  - [ ] API reference (autogenerated)
  - [ ] Migration guide from v0.4
  - [ ] Performance guide
  - [ ] Examples collection

- [ ] **4.4 Performance Benchmarks**
  - [ ] Benchmark suite vs v0.4
  - [ ] Memory leak tests
  - [ ] Zero-copy verification
  - [ ] Throughput comparison

**Criterios de Éxito:**
```typescript
// ✅ New API debe funcionar:
const ach = new Achronyme();
await ach.use(async (a) => {
    const v = a.vector([1, 2, 3]);
    const result = a.sin(v);
});

// ✅ Backward compat debe funcionar:
const ach = new Achronyme();
const v = ach.vector([1, 2, 3]);
const result = ach.sin(v);
// Auto-cleanup cuando ach se va del scope

// ✅ Performance debe mejorar:
// Zero-copy: 80x faster
// No memory leaks: 100%
```

---

### **FASE 5: Rust WASM Migration** ⏱️ 3-4 días

**Objetivo:** Reemplazar C++ WASM con Rust WASM

#### Tareas:

- [ ] **5.1 Update Rust WASM Bindings**
  - [ ] Verify all functions exported
  - [ ] Add missing functions if needed
  - [ ] Update TypeScript bindings

- [ ] **5.2 Remove Emscripten Dependencies**
  - [ ] Replace _malloc/_free with Rust equivalents
  - [ ] Replace HEAPF64 with wasm-bindgen memory
  - [ ] Update MemoryPool implementation

- [ ] **5.3 Testing & Validation**
  - [ ] Full test suite pass
  - [ ] Performance regression tests
  - [ ] Memory leak tests
  - [ ] Cross-browser testing

- [ ] **5.4 Optimization**
  - [ ] Profile hot paths
  - [ ] Optimize memory allocations
  - [ ] Benchmark vs C++ version

**Criterios de Éxito:**
```
✅ All tests pass (100%)
✅ Performance ≥ C++ version
✅ Zero memory leaks
✅ Works in all browsers (Chrome, Firefox, Safari, Edge)
✅ Bundle size ≤ C++ version
```

---

## 📊 API EXAMPLES - Before/After

### Example 1: Basic Operations

#### BEFORE (v0.4.0):
```typescript
const ach = new Achronyme();
await ach.init();

// ❌ Manual memory management
const x = ach.vector([1, 2, 3, 4, 5]);
const y = ach.sin(x);
const z = y.fft_mag();

// ❌ MUST call dispose() or leak memory
x.dispose();
y.dispose();
z.dispose();
```

#### AFTER (v1.0.0):
```typescript
const ach = new Achronyme();

// ✅ Auto-cleanup with session
await ach.use(async (a) => {
    const x = a.vector([1, 2, 3, 4, 5]);
    const y = a.sin(x);
    const z = y.fftMag();

    // ✅ NO dispose() needed!
    // ✅ Auto-cleanup when scope exits
});
```

---

### Example 2: Large Dataset Processing

#### BEFORE:
```typescript
// ❌ Always copies array
const signal = ach.vector([...10_000_000]);
const result = await signal.fft_mag().toVector();
// toVector() → Array.from() → 80ms overhead

// ❌ WebGL requires re-upload
const buffer = new Float64Array(result);
gl.bufferData(gl.ARRAY_BUFFER, buffer, gl.STATIC_DRAW);
```

#### AFTER:
```typescript
// ✅ Zero-copy view
const signal = ach.vector([...10_000_000]);
const result = signal.fftMag();

// ✅ Direct view (0ms)
const view = result.data; // Float64Array view

// ✅ Direct WebGL upload (no intermediate copy)
gl.bufferData(gl.ARRAY_BUFFER, result.data, gl.STATIC_DRAW);

// ✅ Explicit copy only when needed
const array = result.toArray(); // Copy if you need it
```

---

### Example 3: DSP Pipeline

#### BEFORE:
```typescript
// ❌ Many intermediate variables to dispose
const t = ach.linspace(0, 1, 1000);
const signal = ach.sin(t);
const windowed = signal.mul(ach.hanning(1000));
const spectrum = windowed.fft_mag();
const result = await spectrum.toVector();

// ❌ Must dispose all
t.dispose();
signal.dispose();
windowed.dispose();
spectrum.dispose();
```

#### AFTER:
```typescript
await ach.use(async (a) => {
    const spectrum = a.linspace(0, 1, 1000)
        .map(t => Math.sin(2 * Math.PI * 10 * t))
        .mul(a.dsp.hanning(1000))
        .fftMag();

    // ✅ Zero-copy iteration
    for (const magnitude of spectrum) {
        console.log(magnitude);
    }

    // ✅ Auto-cleanup all intermediate values
});
```

---

### Example 4: Linear Algebra

#### BEFORE:
```typescript
const A = ach.matrix([[4, 3], [6, 3]]);
const { L, U, P } = ach.lu(A);

// ❌ Type: any (no type safety)
const lData = await L.toMatrix();

// ❌ Manual disposal
A.dispose();
L.dispose();
U.dispose();
P.dispose();
```

#### AFTER:
```typescript
await ach.use(async (a) => {
    const A = a.matrix([[4, 3], [6, 3]]);
    const { L, U, P } = a.linalg.lu(A);

    // ✅ Type-safe: L is Matrix
    const lData = L.data; // Type: Float64Array

    // ✅ Auto-cleanup
});
```

---

## 🎯 MÉTRICAS DE ÉXITO

### Performance Targets

```
┌──────────────────────────────┬──────────┬──────────┬──────────┐
│ Metric                       │ v0.4.0   │ v1.0.0   │ Target   │
├──────────────────────────────┼──────────┼──────────┼──────────┤
│ Vector creation (10M)        │ 350ms    │ 350ms    │ ≤350ms   │
│ toVector() copy (10M)        │ 80ms     │ <1ms     │ <5ms     │
│ FFT (16M samples)            │ 2957ms   │ 2957ms   │ ≤3000ms  │
│ DSP pipeline (16M)           │ 2808ms   │ 2808ms   │ ≤2800ms  │
│ Memory cleanup               │ Manual   │ Auto     │ Auto     │
│ Memory leaks                 │ Possible │ Zero     │ Zero     │
│ Fast path coverage           │ 60%      │ 95%      │ >90%     │
└──────────────────────────────┴──────────┴──────────┴──────────┘
```

### Code Quality Targets

```
┌──────────────────────────────┬──────────┬──────────┐
│ Metric                       │ v0.4.0   │ v1.0.0   │
├──────────────────────────────┼──────────┼──────────┤
│ Largest file (LOC)           │ 1771     │ <300     │
│ Type safety (any types)      │ 12       │ 0        │
│ Test coverage                │ 60%      │ >90%     │
│ Bundle size (gzip)           │ ~45KB    │ <50KB    │
│ Tree-shakeable               │ No       │ Yes      │
└──────────────────────────────┴──────────┴──────────┘
```

### DX (Developer Experience) Targets

```
✅ No manual dispose() required
✅ Zero memory leaks by design
✅ Type-safe API (no 'any')
✅ Clear error messages
✅ Auto-complete in IDEs
✅ Migration guide available
✅ Performance guide available
```

---

## 🚀 MIGRATION GUIDE (v0.4 → v1.0)

### Breaking Changes

#### 1. Session-based API

**Old (v0.4):**
```typescript
const ach = new Achronyme();
await ach.init();
const x = ach.vector([1, 2, 3]);
x.dispose();
```

**New (v1.0):**
```typescript
const ach = new Achronyme();
// No init() needed

// Option 1: Session-based (recommended)
await ach.use(async (a) => {
    const x = a.vector([1, 2, 3]);
    // Auto-cleanup
});

// Option 2: Global instance (backward compat)
const x = ach.vector([1, 2, 3]);
// Auto-cleanup with GC
```

#### 2. toVector() → data property

**Old:**
```typescript
const result = await vector.toVector(); // Array<number>
```

**New:**
```typescript
const view = vector.data;      // Float64Array (zero-copy)
const array = vector.toArray(); // Array<number> (copy)
```

#### 3. Modular operations

**Old:**
```typescript
const result = ach.sin(x);
```

**New:**
```typescript
// Option 1: Convenience method
const result = ach.sin(x);

// Option 2: Module method
const result = ach.math.sin(x);
```

### Non-Breaking Changes

- All v0.4 APIs still work (with deprecation warnings)
- dispose() still available (but not required)
- Type constructors unchanged

---

## 📝 TESTING STRATEGY

### Unit Tests

```typescript
// Each module has its own test suite
describe('Vector', () => {
    test('zero-copy data getter', () => {
        const v = session.vector([1, 2, 3]);
        const data = v.data;
        expect(data).toBeInstanceOf(Float64Array);
        expect(Array.from(data)).toEqual([1, 2, 3]);
    });

    test('bounds checking', () => {
        const v = session.vector([1, 2, 3]);
        expect(() => v.get(-1)).toThrow();
        expect(() => v.get(3)).toThrow();
    });

    test('iteration', () => {
        const v = session.vector([1, 2, 3]);
        const result = [...v];
        expect(result).toEqual([1, 2, 3]);
    });
});

describe('MathOps', () => {
    test('sin uses fast path', () => {
        const v = session.vector([0, Math.PI/2]);
        const result = mathOps.sin(v);
        expect(result.metadata.usedFastPath).toBe(true);
    });
});
```

### Integration Tests

```typescript
describe('DSP Pipeline', () => {
    test('complete pipeline', async () => {
        await session.use(async () => {
            const result = session.linspace(0, 1, 1000)
                .map(t => Math.sin(2 * Math.PI * 10 * t))
                .fftMag();

            expect(result).toBeInstanceOf(Vector);
            expect(result.length).toBe(1000);
        });

        // Verify no memory leaks
        const stats = session.handleManager.getStats();
        expect(stats.leaked).toBe(0);
    });
});
```

### Performance Tests

```typescript
describe('Performance', () => {
    test('zero-copy is 80x faster', () => {
        const v = session.vector([...Array(10_000_000)]);

        // Zero-copy
        const t1 = performance.now();
        const view = v.data;
        const t2 = performance.now();
        expect(t2 - t1).toBeLessThan(5); // <5ms

        // Copy
        const t3 = performance.now();
        const array = v.toArray();
        const t4 = performance.now();
        expect(t4 - t3).toBeGreaterThan(50); // >50ms
    });
});
```

### Memory Leak Tests

```typescript
describe('Memory Management', () => {
    test('session cleanup releases all handles', async () => {
        const before = session.handleManager.getStats();

        await session.use(async () => {
            for (let i = 0; i < 1000; i++) {
                session.vector([...Array(1000)]);
            }
        });

        const after = session.handleManager.getStats();
        expect(after.active).toBe(before.active);
        expect(after.leaked).toBe(0);
    });
});
```

---

## 🔧 TOOLING & AUTOMATION

### Build Process

```json
{
  "scripts": {
    "build:rust": "bash scripts/build-wasm.sh",
    "build:ts": "tsc",
    "build": "npm run build:rust && npm run build:ts",
    "test": "vitest run",
    "test:watch": "vitest",
    "bench": "vitest bench",
    "lint": "eslint src/",
    "format": "prettier --write src/",
    "typecheck": "tsc --noEmit"
  }
}
```

### CI/CD Pipeline

```yaml
# .github/workflows/test.yml
name: Test
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: wasm32-unknown-unknown

      - run: npm install
      - run: npm run build
      - run: npm test
      - run: npm run bench

      # Memory leak detection
      - name: Check memory leaks
        run: |
          npm run test:memory
          if [ $? -ne 0 ]; then exit 1; fi
```

---

## 📅 TIMELINE

```
┌────────────┬──────────────────────────────────────┬─────────┐
│ Week       │ Phase                                │ Status  │
├────────────┼──────────────────────────────────────┼─────────┤
│ Week 1     │ Phase 1: Core Infrastructure         │ 🔜 TODO │
│ Week 2     │ Phase 2: Value Types                 │ 🔜 TODO │
│ Week 3     │ Phase 3: Operations Modules          │ 🔜 TODO │
│ Week 4     │ Phase 4: Facade & Integration        │ 🔜 TODO │
│ Week 5     │ Phase 5: Rust WASM Migration         │ 🔜 TODO │
│ Week 6     │ Testing, Docs, Polish                │ 🔜 TODO │
└────────────┴──────────────────────────────────────┴─────────┘

Total Duration: 5-6 weeks
Release Target: v1.0.0
```

---

## 🎓 LEARNING RESOURCES

### Concepts

- **WeakRef & FinalizationRegistry**: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakRef
- **TypedArrays**: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray
- **RAII Pattern**: https://en.wikipedia.org/wiki/Resource_acquisition_is_initialization
- **wasm-bindgen**: https://rustwasm.github.io/docs/wasm-bindgen/

### Best Practices

- **Zero-copy techniques**: Avoid Array.from(), use views
- **Memory management**: Prefer automatic cleanup over manual
- **Type safety**: Use generics and strict types
- **Modularity**: Keep files <300 LOC

---

## ✅ CHECKLIST COMPLETO

### Fase 1: Core Infrastructure

- [ ] Session.ts implementado
- [ ] HandleManager.ts implementado
- [ ] MemoryPool.ts implementado
- [ ] RustBindings.ts implementado
- [ ] Tests unitarios pasando (>90% coverage)
- [ ] Session auto-cleanup funcional
- [ ] FinalizationRegistry funcionando

### Fase 2: Value Types

- [ ] Value.ts (base class)
- [ ] Vector.ts con zero-copy
- [ ] Matrix.ts con zero-copy
- [ ] Scalar.ts
- [ ] Complex.ts
- [ ] Tests exhaustivos
- [ ] Zero-copy benchmark (80x+ faster)

### Fase 3: Operations

- [ ] MathOps.ts (28 functions)
- [ ] DSPOps.ts (12 functions)
- [ ] LinalgOps.ts (18 functions)
- [ ] VectorOps.ts (8 functions)
- [ ] HOFOps.ts (4 functions)
- [ ] StatsOps.ts (3 functions)
- [ ] All tests passing

### Fase 4: Integration

- [ ] Achronyme.ts facade
- [ ] Backward compatibility layer
- [ ] Migration guide
- [ ] API documentation
- [ ] Performance benchmarks
- [ ] Examples collection

### Fase 5: Rust WASM

- [ ] Update Rust bindings
- [ ] Remove Emscripten deps
- [ ] Full test suite pass
- [ ] Performance validation
- [ ] Cross-browser testing
- [ ] Bundle optimization

### Final

- [ ] All tests passing (100%)
- [ ] Zero memory leaks verified
- [ ] Performance targets met
- [ ] Documentation complete
- [ ] Examples working
- [ ] Migration guide reviewed
- [ ] v1.0.0 released 🚀

---

## 📞 SUPPORT & QUESTIONS

Si tienes preguntas durante la implementación:

1. Revisar este documento primero
2. Consultar los ejemplos de código
3. Revisar los tests unitarios
4. Consultar documentación de Rust WASM
5. Preguntar en el proyecto

---

**END OF REFACTOR PLAN**

*Este documento es un living document - se actualizará según progrese la implementación.*
