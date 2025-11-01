# Fast Path vs Slow Path - Diagramas Visuales

## 📊 Arquitectura General

```
┌──────────────────────────────────────────────────────────────────┐
│                      JAVASCRIPT SDK                               │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │  Achronyme Class                                            │  │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────────┐  │  │
│  │  │ handleToVar  │  │ varToHandle  │  │ fastPathCounter  │  │  │
│  │  │ Map<int,str> │  │ Map<str,int> │  │ slowPathCounter  │  │  │
│  │  └──────────────┘  └──────────────┘  └──────────────────┘  │  │
│  │                                                              │  │
│  │  Decision Logic:                                             │  │
│  │  data.length >= threshold ? FAST : SLOW                      │  │
│  └────────────────────────────────────────────────────────────┘  │
└───────────────────────┬──────────────────────────────────────────┘
                        │
            ┌───────────┴───────────┐
            │                       │
    ┌───────▼────────┐      ┌──────▼────────┐
    │   FAST PATH    │      │   SLOW PATH   │
    │   (Handles)    │      │   (Parser)    │
    └───────┬────────┘      └──────┬────────┘
            │                       │
┌───────────▼───────────────────────▼──────────────────────────────┐
│                         WASM MODULE                               │
│  ┌────────────────────────┐  ┌─────────────────────────────────┐ │
│  │   HandleManager        │  │   Evaluator                     │ │
│  │   ┌──────────────────┐ │  │   ┌──────────┐  ┌──────────┐  │ │
│  │   │ Handle 1 → Value │ │  │   │  Lexer   │→ │  Parser  │  │ │
│  │   │ Handle 2 → Value │ │  │   └──────────┘  └──────┬───┘  │ │
│  │   │ Handle 3 → Value │ │  │                         │      │ │
│  │   └──────────────────┘ │  │   ┌─────────────────────▼───┐ │ │
│  │   std::map<int,Value*> │  │   │   Environment           │ │ │
│  └────────────────────────┘  │   │   (__v0, __v1, __v2)    │ │ │
│                               │   └─────────────────────────┘ │ │
│                               └─────────────────────────────────┘ │
└──────────────────────────────────────────────────────────────────┘
```

---

## 🚀 FAST PATH - Flujo Completo

### Creación de Vector

```
JavaScript                    WASM Memory                    C++
──────────────────────────────────────────────────────────────────────

const data = [1..1000]
      │
      ▼
data.length >= 8? ────────────────────┐
      │ YES                            │ NO → SLOW PATH
      ▼                                │
allocFloat64(data)                     │
      │                                │
      ▼                                │
┌─────────────────┐                    │
│ JS Heap         │                    │
│ Float64Array    │                    │
│ [1,2,3,...1000] │                    │
└────────┬────────┘                    │
         │ memcpy                      │
         ▼                             │
   ┌─────────────────┐                 │
   │ WASM Heap       │                 │
   │ ptr: 0x1000     │                 │
   │ [1,2,3,...1000] │                 │
   └────────┬────────┘                 │
            │                          │
            ▼                          │
  createVectorFromBuffer(ptr, 1000)    │
            │                          │
            ▼                          │
     ┌──────────────────┐              │
     │ HandleManager    │              │
     │ create(Vector)   │              │
     └────────┬─────────┘              │
              │                        │
              ▼                        │
         return 1 (handle)             │
              │                        │
              ▼                        │
bindVariableToHandle("__v0", 1)        │
              │                        │
              ▼                        │
    ┌──────────────────────┐           │
    │ Environment          │           │
    │ __v0 → Handle(1)     │           │
    └──────────────────────┘           │
              │                        │
              ▼                        │
  handleToVar.set(1, "__v0") ──────────┘
  varToHandle.set("__v0", 1)
              │
              ▼
  return AchronymeValue("__v0", handle=1)

Total time: ~0.5ms (1000 elements)
```

---

### Operación FFT

```
JavaScript                    WASM Memory                    C++
──────────────────────────────────────────────────────────────────────

signal.fft()
      │
      ▼
varToHandle.get("__v0")
      │
      ▼
   handle = 1
      │
      ▼
module.fft_fast(1)  ───────────────────────┐
                                            │
                              ┌─────────────▼────────────┐
                              │ HandleManager.get(1)     │
                              │                          │
                              │ Return: Vector [1..1000] │
                              └─────────────┬────────────┘
                                            │
                                            ▼
                              ┌─────────────────────────┐
                              │ Apply FFT Algorithm     │
                              │ (Cooley-Tukey)          │
                              │                         │
                              │ Input:  Real Vector     │
                              │ Output: Complex Vector  │
                              └─────────────┬───────────┘
                                            │
                                            ▼
                              ┌─────────────────────────┐
                              │ HandleManager.create()  │
                              │                         │
                              │ Store result            │
                              └─────────────┬───────────┘
                                            │
                                            ▼
                                       return 2 (handle)
                                            │
      ┌─────────────────────────────────────┘
      │
      ▼
_createFromHandle(2)
      │
      ▼
bindVariableToHandle("__v1", 2)
      │
      ▼
return AchronymeValue("__v1", handle=2)

Total time: ~1.2ms (1000 elements)
NO parsing, NO serialization
```

---

## 🐌 SLOW PATH - Flujo Completo

### Creación de Vector

```
JavaScript                    WASM                        C++
─────────────────────────────────────────────────────────────────

const data = [1,2,3]
      │
      ▼
data.length >= 8?
      │ NO
      ▼
formatVector([1,2,3])
      │
      ▼
  "[1, 2, 3]"  ← String serialization
      │
      ▼
eval("let __v0 = [1, 2, 3]")
      │                           ┌─────────────────┐
      └───────────────────────────► Lexer          │
                                  │ Tokenize string │
                                  └────────┬────────┘
                                           │
                                  ┌────────▼────────┐
                                  │ Tokens:         │
                                  │ LET, ID(__v0),  │
                                  │ EQ, LBRACK,     │
                                  │ NUM(1), COMMA,  │
                                  │ NUM(2), COMMA,  │
                                  │ NUM(3), RBRACK  │
                                  └────────┬────────┘
                                           │
                                  ┌────────▼────────┐
                                  │ Parser          │
                                  │ Build AST       │
                                  └────────┬────────┘
                                           │
                                  ┌────────▼────────┐
                                  │ AST:            │
                                  │ VarDecl         │
                                  │  ├─ name: __v0  │
                                  │  └─ value:      │
                                  │      VectorLit  │
                                  │      [1, 2, 3]  │
                                  └────────┬────────┘
                                           │
                                  ┌────────▼────────┐
                                  │ Evaluator       │
                                  │ Create Value    │
                                  └────────┬────────┘
                                           │
                                  ┌────────▼────────┐
                                  │ Value:          │
                                  │ Vector([1,2,3]) │
                                  └────────┬────────┘
                                           │
                                  ┌────────▼────────┐
                                  │ Environment     │
                                  │ set(__v0, val)  │
                                  └─────────────────┘
                                           │
      ┌────────────────────────────────────┘
      │
      ▼
return AchronymeValue("__v0", handle=null)

Total time: ~10-50μs (small vector)
```

---

### Operación FFT

```
JavaScript                    WASM                        C++
─────────────────────────────────────────────────────────────────

signal.fft()
      │
      ▼
varToHandle.get("__v0")
      │
      ▼
   handle = undefined (no handle!)
      │
      ▼
eval("let __v1 = fft(__v0)")
      │                           ┌─────────────────┐
      └───────────────────────────► Lexer          │
                                  └────────┬────────┘
                                           │
                                  ┌────────▼────────┐
                                  │ Tokens:         │
                                  │ LET, ID(__v1),  │
                                  │ EQ, ID(fft),    │
                                  │ LPAREN, ID(__v0)│
                                  │ RPAREN          │
                                  └────────┬────────┘
                                           │
                                  ┌────────▼────────┐
                                  │ Parser          │
                                  └────────┬────────┘
                                           │
                                  ┌────────▼────────┐
                                  │ Evaluator       │
                                  │ 1. Lookup __v0  │
                                  │ 2. Apply fft()  │
                                  │ 3. Store __v1   │
                                  └─────────────────┘
      ┌────────────────────────────────────┘
      │
      ▼
return AchronymeValue("__v1", handle=null)

Total time: ~5-10ms (parsing overhead)
```

---

## 🔄 Comparación Lado a Lado

### Vector Grande (100K elementos)

```
FAST PATH                             SLOW PATH
═══════════════════════════════════   ═══════════════════════════════════
Total: ~0.5ms                         Total: ~450ms

┌─────────────────────┐               ┌─────────────────────┐
│ Float64Array        │               │ Float64Array        │
│ 100,000 elements    │               │ 100,000 elements    │
└──────────┬──────────┘               └──────────┬──────────┘
           │                                     │
           │ memcpy (0.3ms)                     │ formatVector (200ms)
           ▼                                     ▼
    ┌──────────────┐                    ┌──────────────────┐
    │ WASM Heap    │                    │ String:          │
    │ Binary data  │                    │ "[1,2,3,...,     │
    │ 800KB        │                    │  100000]"        │
    └──────┬───────┘                    │ 600KB            │
           │                             └──────┬───────────┘
           │ createHandle (0.1ms)               │
           ▼                                     │ parse (250ms)
    ┌──────────────┐                            ▼
    │ Handle: 1    │                    ┌──────────────────┐
    └──────────────┘                    │ Vector in memory │
                                        └──────────────────┘

MEJORA: 900x más rápido
```

---

## 📈 Threshold Decision Tree

```
                    ┌─────────────────┐
                    │ ach.vector(data)│
                    └────────┬────────┘
                             │
                   ┌─────────▼─────────┐
                   │ data.length >= 8? │
                   └─────┬──────────┬──┘
                         │          │
                    YES  │          │ NO
                         │          │
            ┌────────────▼──┐   ┌──▼────────────┐
            │  FAST PATH    │   │  SLOW PATH    │
            ├───────────────┤   ├───────────────┤
            │ • memcpy      │   │ • formatVector│
            │ • createHandle│   │ • parse       │
            │ • ~0.5ms      │   │ • ~10-50μs    │
            └───────────────┘   └───────────────┘
                    │                   │
                    └─────────┬─────────┘
                              │
                    ┌─────────▼──────────┐
                    │ AchronymeValue     │
                    │ _varName: "__v0"   │
                    │ _handle: 1 or null │
                    └────────────────────┘

Ejemplos:
• [1, 2, 3, 4]               → SLOW (4 < 8)
• [1, 2, 3, 4, 5, 6, 7, 8]   → FAST (8 ≥ 8)
• Float64Array(100000)       → FAST (100000 ≥ 8)
```

---

## 🧮 Operaciones Encadenadas

### Fast Path Dominante

```
Pipeline: linspace → sin → fft → mag
═══════════════════════════════════════════════════════

JavaScript                              WASM Memory
──────────────────────────────────────────────────────

const t = ach.linspace(0, 10, 1024);
           │                            ┌──────────────────┐
           └────────────────────────────► Handle 1:       │
                                        │ [0, 0.01, ...]  │
                                        └──────────────────┘
const signal = t.sin();
           │                            ┌──────────────────┐
           └────────────────────────────► Handle 2:       │
                                        │ [sin(0), ...]   │
                                        └──────────────────┘
const spectrum = signal.fft_mag();
           │                            ┌──────────────────┐
           └────────────────────────────► Handle 3:       │
                                        │ [mag0, mag1,...] │
                                        └──────────────────┘

const data = await spectrum.toVector();
           │
           └─── Solo AQUI se serializa y pasa a JS
                (cuando realmente necesitas los datos)

Total: ~2-3ms
Handles pasados: 3 (int32)
Serialización: 1 sola vez al final
```

### Slow Path Equivalente

```
Pipeline: linspace → sin → fft → mag
═══════════════════════════════════════════════════════

eval("let __v0 = linspace(0, 10, 1024)")
     └─ Parse, eval, stringify ────► ~10ms

eval("let __v1 = sin(__v0)")
     └─ Parse, lookup, eval ───────► ~10ms

eval("let __v2 = fft(__v1)")
     └─ Parse, lookup, eval ───────► ~15ms

eval("let __v3 = fft_mag(__v2)")
     └─ Parse, lookup, eval ───────► ~15ms

Total: ~50ms
Parsing: 4 veces
Lookups: 4 veces

DIFERENCIA: 16-25x más lento
```

---

## 💾 Gestión de Memoria

### Fast Path - Handle Lifecycle

```
Creation:                    Memory State:
────────────────────────────────────────────────────

const v1 = ach.vector(data); ┌─────────────────────┐
                              │ HandleManager       │
                              │ Handle 1 → Vector A │
                              └─────────────────────┘

const v2 = v1.add(v1);       ┌─────────────────────┐
                              │ HandleManager       │
                              │ Handle 1 → Vector A │
                              │ Handle 2 → Vector B │
                              └─────────────────────┘

v1.dispose();                ┌─────────────────────┐
                              │ HandleManager       │
                              │ Handle 2 → Vector B │
                              └─────────────────────┘
                              ▲ Handle 1 released
                                Memory freed

v2.dispose();                ┌─────────────────────┐
                              │ HandleManager       │
                              │ (empty)             │
                              └─────────────────────┘
                              ▲ Handle 2 released
                                All memory freed
```

### Slow Path - Environment Lifecycle

```
Creation:                    Memory State:
────────────────────────────────────────────────────

const v1 = ach.vector(data); ┌─────────────────────┐
                              │ Environment         │
                              │ __v0 → Vector A     │
                              └─────────────────────┘

const v2 = v1.add(v1);       ┌─────────────────────┐
                              │ Environment         │
                              │ __v0 → Vector A     │
                              │ __v1 → Vector B     │
                              └─────────────────────┘

v1.dispose();                ┌─────────────────────┐
                              │ Environment         │
                              │ __v1 → Vector B     │
                              └─────────────────────┘
                              ▲ __v0 removed
                                Memory freed

v2.dispose();                ┌─────────────────────┐
                              │ Environment         │
                              │ (empty)             │
                              └─────────────────────┘
                              ▲ __v1 removed
                                All memory freed
```

---

## 📊 Stats Tracking

```javascript
const stats = ach.getMemoryStats();
```

### Estructura Interna

```
┌──────────────────────────────────────────────────────────┐
│                    Achronyme Instance                     │
├──────────────────────────────────────────────────────────┤
│                                                           │
│  Tracking Maps:                                           │
│  ┌─────────────────────┐  ┌─────────────────────┐        │
│  │ handleToVar         │  │ varToHandle         │        │
│  ├─────────────────────┤  ├─────────────────────┤        │
│  │ 1 → "__v0"          │  │ "__v0" → 1          │        │
│  │ 2 → "__v1"          │  │ "__v1" → 2          │        │
│  │ 3 → "__v3"          │  │ "__v3" → 3          │        │
│  └─────────────────────┘  └─────────────────────┘        │
│                                                           │
│  Counters:                                                │
│  ┌──────────────────────────────────────────┐            │
│  │ fastPathOperationsCount:  185            │            │
│  │ slowPathOperationsCount:  15             │            │
│  │                                          │            │
│  │ Ratio: 185/(185+15) = 92.5%              │            │
│  └──────────────────────────────────────────┘            │
│                                                           │
│  Variables Set:                                           │
│  ┌──────────────────────────────────────────┐            │
│  │ variables = {"__v0", "__v1", "__v2", ...}│            │
│  │ Size: 5 active                           │            │
│  └──────────────────────────────────────────┘            │
└──────────────────────────────────────────────────────────┘
```

### Output Example

```javascript
{
  activeVariables: 5,           // Variables en environment
  activeHandles: 3,             // Handles en HandleManager
  totalVariablesCreated: 100,   // Contador de __v0, __v1...
  totalHandlesCreated: 80,      // Total de handles creados
  fastPathUsagePercent: 92.5,   // % operaciones fast path
  fastPathOperations: 185,      // Contador fast path
  slowPathOperations: 15        // Contador slow path
}
```

---

## 🎯 Decision Matrix

```
┌─────────────────────────────────────────────────────────────────┐
│                      DECISION MATRIX                             │
├──────────────┬──────────────────┬───────────────────────────────┤
│ Tamaño Vector│ Recommended Path │ Razón                         │
├──────────────┼──────────────────┼───────────────────────────────┤
│ 1-7 elem     │ SLOW PATH        │ Parsing es más rápido que     │
│              │                  │ crear handle (overhead)       │
├──────────────┼──────────────────┼───────────────────────────────┤
│ 8-100 elem   │ FAST PATH        │ Handle ya es más eficiente    │
├──────────────┼──────────────────┼───────────────────────────────┤
│ 100-10K elem │ FAST PATH        │ Parsing se vuelve muy lento   │
├──────────────┼──────────────────┼───────────────────────────────┤
│ >10K elem    │ FAST PATH        │ CRÍTICO - parsing tomaría     │
│              │                  │ cientos de ms                 │
└──────────────┴──────────────────┴───────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│                   OPERATION TYPE MATRIX                          │
├──────────────────────┬──────────────────┬───────────────────────┤
│ Tipo de Operación    │ Preferred Path   │ Razón                 │
├──────────────────────┼──────────────────┼───────────────────────┤
│ Creación aislada     │ AUTO             │ Decide por threshold  │
├──────────────────────┼──────────────────┼───────────────────────┤
│ Pipeline DSP         │ FAST PATH        │ Múltiples operaciones │
│ (3+ operaciones)     │                  │ sin serialización     │
├──────────────────────┼──────────────────┼───────────────────────┤
│ Debugging/Explorar   │ SLOW PATH        │ Más fácil inspección  │
├──────────────────────┼──────────────────┼───────────────────────┤
│ Expresiones complejas│ SLOW PATH        │ Parser más flexible   │
│ ad-hoc               │                  │                       │
└──────────────────────┴──────────────────┴───────────────────────┘
```

---

**Autor**: eddndev@achronymelabs
**Fecha**: 2025-11-01
**Propósito**: Documentación visual del sistema Fast Path vs Slow Path
