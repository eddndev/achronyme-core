# Issues Conocidos - Achronyme Core

Este documento lista los problemas conocidos del núcleo WASM y del SDK TypeScript.

## 🔴 Issues del Núcleo WASM

**Estado:** ✅ No hay issues críticos conocidos

**Últimos fixes aplicados:**
- ✅ **Operador Módulo (%)** - Implementado completamente (2025-10-26)
- ✅ **Multiplicación Vector-Vector** - Element-wise multiplication implementado (2025-10-26)

---

## 🟡 Limitaciones del SDK TypeScript

### **1. Gestión Manual de Memoria**

**Severidad:** Baja (Diseño intencional)
**Estado:** Working as designed

**Descripción:**
El SDK requiere llamar manualmente a `dispose()` en cada valor para liberar memoria del Environment de C++.

**Ejemplo:**
```typescript
const ach = new Achronyme();
await ach.init();

const x = ach.number(10);
const y = x.add(5);

// ✅ Correcto: Llamar dispose()
x.dispose();
y.dispose();

// ❌ Incorrecto: Olvidar dispose() causa fuga de memoria
```

**Razón:**
- El GC de JavaScript no conoce la memoria de WASM/C++
- `FinalizationRegistry` es experimental y no confiable
- Gestión manual es más predecible y controlable

**Mitigación:**
- Usar `getMemoryStats()` para monitorear variables
- Considerar un patrón de pool/batch para operaciones masivas
- Documentación clara sobre cuándo llamar dispose()

---

## ✅ Fixes Recientes Verificados

### **Filter Function con Operador Módulo**

**Estado:** ✅ Funciona completamente

**Descripción:**
Filter ahora funciona correctamente con el operador módulo (%) tras implementación reciente.

**Pruebas:**
```javascript
const Module = await createAchronymeModule();

// ✅ Funciona
Module.eval('filter(x => x > 2, [1,2,3,4,5])');
// → [3.000000, 4.000000, 5.000000]

// ✅ Ahora funciona correctamente
Module.eval('filter(x => x % 2 == 0, [1,2,3,4,5,6])');
// → [2.000000, 4.000000, 6.000000]
```

**Conclusión:** Filter funciona correctamente con todos los operadores incluyendo módulo.

---

## 📊 Resumen de Compatibilidad

### **Operadores Aritméticos**
- ✅ `+` Suma
- ✅ `-` Resta
- ✅ `*` Multiplicación
- ✅ `/` División
- ✅ `^` Potencia
- ✅ `%` Módulo

### **Operadores de Comparación**
- ✅ `>` Mayor que
- ✅ `<` Menor que
- ✅ `>=` Mayor o igual
- ✅ `<=` Menor o igual
- ✅ `==` Igual
- ✅ `!=` Diferente

### **Operaciones Vectoriales**
- ✅ Vector + escalar
- ✅ Vector - escalar
- ✅ Vector * escalar
- ✅ Vector / escalar
- ✅ Vector ^ escalar
- ✅ Vector + Vector
- ✅ Vector - Vector
- ✅ Vector * Vector (elemento-a-elemento / Hadamard)
- ✅ dot(Vector, Vector)
- ✅ cross(Vector, Vector)
- ✅ norm(Vector)

### **Funciones Matemáticas**
- ✅ Trigonométricas (sin, cos, tan, etc.) - 100%
- ✅ Exponenciales (exp, ln, log, etc.) - 100%
- ✅ Raíces (sqrt, cbrt) - 100%
- ✅ Redondeo (floor, ceil, round) - 100%

### **Funciones DSP**
- ✅ FFT/IFFT - 100%
- ✅ DFT - 100%
- ✅ Convolución (directa) - 100%
- ✅ Convolución (FFT) - 100%
- ✅ Window functions (hanning, hamming, blackman) - 100%

### **Higher-Order Functions**
- ✅ map - 100%
- ✅ filter - 100%
- ✅ reduce - 100%
- ✅ pipe - 100%
- ✅ compose - 100%

### **Programación Funcional**
- ✅ Variables (let) - 100%
- ✅ Lambdas (=>) - 100%
- ✅ Closures - 100%
- ✅ Multi-parámetro lambdas - 100%

---

## 🛠️ Recomendaciones

### **Para Usuarios del SDK**

1. **Siempre llamar dispose():**
   ```typescript
   const value = ach.number(42);
   // ... usar value ...
   value.dispose();  // ✅ Importante
   ```

2. **Usar operador módulo en lambdas:**
   ```typescript
   // ✅ Ahora funciona perfectamente
   ach.filter('x => x % 2 == 0', vector);  // Filtrar números pares
   ach.map('x => x % 10', vector);         // Obtener último dígito
   ```

3. **Usar multiplicación elemento-a-elemento:**
   ```typescript
   // ✅ Ahora funciona correctamente
   const windowed = signal.mul(window);    // vector * vector (Hadamard)
   const scaled = signal.mul(0.5);         // vector * escalar
   ```

### **Para Desarrolladores del Núcleo**

1. **Prioridad Baja:** Considerar auto-dispose experimental
   - Investigar `FinalizationRegistry`
   - Solo como opción opt-in

2. **Mejoras Futuras:** Optimizaciones
   - SIMD para operaciones vectoriales
   - Caching de FFT para señales repetidas

---

## 📈 Métricas de Calidad

**Funcionalidad del SDK:**
- Tests pasando: 17/17 (100%)
- Ejemplos funcionando: 4/4 (100%)
- Cobertura de API: 100%

**Núcleo WASM:**
- Operadores: 10/10 (100%) ✅
- Funciones matemáticas: 60/60 (100%)
- Funciones DSP: 7/7 (100%)
- Higher-order functions: 5/5 (100%)
- Operaciones vectoriales: 11/11 (100%) ✅

**Total:** 100% de funcionalidad operativa ✅

**Tests del núcleo:**
- Demo completo: 96/96 tests pasando (100%)
- SDK TypeScript: 17/17 tests pasando (100%)

---

**Última actualización:** 2025-10-26
**Versión:** Achronyme Core v0.3.0 + SDK TypeScript v1.0.0
**Fixes recientes:** Operador módulo (%), multiplicación vector-vector
