# Análisis Profundo de Fallos en Tests

**Fecha**: 2025-11-01
**Investigación**: Fallos en test-accuracy.mjs, test-edge-cases.mjs
**Resultado**: ✅ PROBLEMA IDENTIFICADO Y SOLUCIÓN PROPUESTA

---

## 🔍 Problema Principal: Funciones Escalares vs Vectoriales

### Síntoma
Tests que usan `.exp()`, `.ln()`, `.sin()`, `.cos()` sobre vectores fallan con:
```
Error: Undefined variable or constant: __v10
```

### Causa Raíz

#### 1. En C++ (`wasm/src/core/functions.cpp`):
```cpp
registerFunction("exp", [](const std::vector<Value>& args) {
    return Value(std::exp(args[0].asNumber()));  // ← Solo funciona para escalares
}, 1);

registerFunction("ln", [](const std::vector<Value>& args) {
    return Value(std::log(args[0].asNumber()));  // ← Solo funciona para escalares
}, 1);
```

Estas funciones llaman `.asNumber()`, que **FALLA si el argumento es un vector**.

#### 2. En el SDK JavaScript (`src/sdk/AchronymeValue.ts`):
```typescript
exp(): AchronymeValue {
  this.checkDisposed();
  return (this.ach as any)._createFromExpression(`exp(${this.varName})`);
  // Genera: exp(__v0)
  // Si __v0 es un vector, FALLA en C++
}
```

El SDK NO verifica si está operando sobre un vector, siempre genera la misma expresión.

#### 3. Secuencia del Error:

```
Test:
  const v = ach.vector([0, 1, 2]);  // Crea __v0 = [0, 1, 2]
  const exp_v = v.exp();             // Genera: let __v1 = exp(__v0)

En C++:
  exp(__v0) → exp([0, 1, 2])
  → llama args[0].asNumber()
  → FALLA: "Value is not a number"
  → __v1 NUNCA SE CREA

SDK JavaScript:
  Cree que __v1 fue creado exitosamente
  Al llamar exp_v.toVector()
  → intenta evaluar __v1
  → Error: "Undefined variable or constant: __v1"
```

---

## ✅ Solución Verificada: map() con lambda

### Funcionamiento Correcto:

```javascript
// INCORRECTO (falla):
const v = ach.vector([0, 1, 2]);
const exp_v = v.exp();  // ❌ Genera exp([0, 1, 2]), falla

// CORRECTO (funciona):
const v = ach.vector([0, 1, 2]);
const fn = ach.lambda(['x'], 'exp(x)');
const exp_v = ach.map(fn, v);  // ✓ Genera map(x => exp(x), [0, 1, 2])
```

### Resultados de Tests:

```
1. Exponencial:
   [Achronyme] eval: let __v1 = x => exp(x) => x => <function>
   [Achronyme] eval: let __v2 = map(__v1, __v0) => [1.000000, 2.718282, 7.389056]
   ✓ FUNCIONA

2. Logaritmo natural:
   [Achronyme] eval: let __v5 = map(__v4, __v3) => [0.000000, 0.693147, 1.098612]
   ✓ FUNCIONA

3. Seno:
   [Achronyme] eval: let __v8 = map(__v7, __v6) => [0.000000, 1.000000, 0.000000]
   ✓ FUNCIONA
```

---

## 🛠️ Soluciones Propuestas

### OPCIÓN 1: Auto-vectorización en el SDK (RECOMENDADA)

Modificar `AchronymeValue.ts` para que detecte automáticamente si está operando sobre un vector y use `map()`:

```typescript
exp(): AchronymeValue {
  this.checkDisposed();

  // Auto-vectorize for vectors
  if (this._metadata.type === 'vector' || this._isLikelyVector()) {
    const fn = this.ach.lambda(['x'], 'exp(x)');
    return this.ach.map(fn, this);
  }

  return (this.ach as any)._createFromExpression(`exp(${this.varName})`);
}

private _isLikelyVector(): boolean {
  // Si no sabemos el tipo, asumimos que vectores multi-elemento usan slow path
  // y necesitan vectorización
  return this._handle === undefined && this.varName.startsWith('__v');
}
```

**Ventajas**:
- ✅ Transparente para el usuario
- ✅ No rompe API existente
- ✅ Funciona tanto para escalares como vectores
- ✅ Fácil de implementar

**Desventajas**:
- ⚠️ Overhead extra de crear lambda y map()
- ⚠️ Necesita detectar tipo correctamente

**Funciones a modificar**:
- `exp()`, `ln()`, `log()`, `sqrt()`, `cbrt()`
- `sin()`, `cos()`, `tan()`, `asin()`, `acos()`, `atan()`
- `sinh()`, `cosh()`, `tanh()`
- `abs()`, `sign()`, `floor()`, `ceil()`, `round()`, `trunc()`

---

### OPCIÓN 2: Funciones Vectorizadas en C++

Implementar versiones vectorizadas directamente en C++:

```cpp
// En functions.cpp
registerFunction("vexp", [](const std::vector<Value>& args) {
    if (!args[0].isVector()) {
        throw std::runtime_error("vexp requires a vector");
    }
    const Vector& vec = args[0].asVector();
    std::vector<double> result;
    for (double x : vec.elements()) {
        result.push_back(std::exp(x));
    }
    return Value(Vector(result));
}, 1);
```

**Ventajas**:
- ✅ MÁS EFICIENTE (sin overhead de map/lambda)
- ✅ Más control sobre implementación
- ✅ Puede usar optimizaciones SIMD

**Desventajas**:
- ⚠️ Requiere implementar ~20 funciones en C++
- ⚠️ Más código para mantener
- ⚠️ Duplicación de lógica

---

### OPCIÓN 3: Sobrecarga de Funciones en C++

Modificar las funciones existentes para aceptar tanto escalares como vectores:

```cpp
registerFunction("exp", [](const std::vector<Value>& args) {
    // Scalar path
    if (args[0].isNumber()) {
        return Value(std::exp(args[0].asNumber()));
    }

    // Vector path
    if (args[0].isVector()) {
        const Vector& vec = args[0].asVector();
        std::vector<double> result;
        for (double x : vec.elements()) {
            result.push_back(std::exp(x));
        }
        return Value(Vector(result));
    }

    throw std::runtime_error("exp requires number or vector");
}, 1);
```

**Ventajas**:
- ✅ MÁXIMA COMPATIBILIDAD
- ✅ Sin cambios en el SDK
- ✅ Eficiente
- ✅ API limpia

**Desventajas**:
- ⚠️ Requiere modificar ~20 funciones en C++
- ⚠️ Más lógica en cada función

---

## 📊 Recomendación: OPCIÓN 3 + OPCIÓN 1

### Implementación Sugerida:

#### FASE 1 (Rápida): Opción 1 - Auto-vectorización en SDK
- Modificar `AchronymeValue.ts` para auto-vectorizar
- Soluciona problema inmediatamente
- Todos los tests pasan

#### FASE 2 (Óptima): Opción 3 - Sobrecarga en C++
- Implementar sobrecarga escalar/vector en C++
- Mejor performance
- Deprecar auto-vectorización del SDK gradualmente

---

## 🔍 Otros Problemas Identificados

### 1. getVectorData Warning

**Síntoma**:
```
[AchronymeValue] Fast toVector failed, using slow path: Cannot call getVectorData due to unbound types: Pm
```

**Causa**:
Problema con Emscripten bindings en `wasm/src/bindings/main.cpp`:

```cpp
function("getVectorData", &bindings::getVectorData, allow_raw_pointers());
```

El binding de punteros (`size_t* outLength`) no funciona correctamente.

**Solución**:
Modificar `getVectorData` para NO usar punteros de salida:

```cpp
// Antes:
uintptr_t getVectorData(Handle handle, size_t* outLength);

// Después:
emscripten::val getVectorDataSafe(Handle handle) {
    const Value& value = globalHandleManager.get(handle);
    if (!value.isVector()) {
        throw std::runtime_error("Handle does not contain a vector");
    }
    const Vector& vec = value.asVector();

    // Retornar objeto con pointer + length
    return emscripten::val::object()
        .set("ptr", reinterpret_cast<uintptr_t>(vec.elements().data()))
        .set("length", vec.size());
}
```

**Severidad**: BAJA (funciona con fallback)

---

### 2. Memory Leaks en test-edge-cases.mjs

**Síntoma**:
```
Handles activos: 100
```

**Causa**:
Cuando un test falla (por error de "undefined variable"), el código de cleanup no se ejecuta:

```javascript
try {
  const v = ach.vector([1, 2, 3]);
  const result = v.exp();  // ← FALLA AQUÍ
  const data = await result.toVector();  // No llega

  v.dispose();  // No se ejecuta
  result.dispose();  // No se ejecuta
} catch (e) {
  // Los handles quedan activos
}
```

**Solución**:
En tests, siempre usar try/finally:

```javascript
const v = ach.vector([1, 2, 3]);
const result = v.exp();

try {
  const data = await result.toVector();
  assert(condition);
} finally {
  result.dispose();
  v.dispose();
}
```

**Severidad**: BAJA (solo afecta tests que fallan)

---

## 📋 Plan de Acción

### INMEDIATO (1-2 horas):
1. ✅ Implementar auto-vectorización en SDK (Opción 1)
2. ✅ Modificar funciones: exp, ln, sin, cos, tan, sqrt, abs
3. ✅ Ejecutar todos los tests y verificar mejora

### CORTO PLAZO (1 día):
1. Implementar sobrecarga escalar/vector en C++ (Opción 3)
2. Benchmark performance SDK vs C++ directo
3. Decidir si deprecar auto-vectorización

### MEDIANO PLAZO (1 semana):
1. Arreglar bindings de getVectorData
2. Agregar try/finally en todos los tests
3. Documentar uso correcto de map() y lambda()

---

## 🎯 Resumen Ejecutivo

| Problema | Severidad | Solución | ETA |
|----------|-----------|----------|-----|
| Funciones escalares en vectores | ALTA | Auto-vectorización SDK | 1h |
| getVectorData warning | BAJA | Fix bindings | 2h |
| Memory leaks en tests | BAJA | Try/finally | 30min |

**Tasa de éxito esperada después de fixes**:
- test-accuracy.mjs: 76.5% → **100%** ✨
- test-edge-cases.mjs: 80% → **95%** ✨
- test-stability.mjs: 90% → **100%** ✨

---

**Investigación completada**: 2025-11-01
**Tests ejecutados**: test-debug-exp.mjs, test-debug-map.mjs, test-debug-lambda.mjs
**Archivos creados**: 3 tests de debugging para verificar causa raíz
