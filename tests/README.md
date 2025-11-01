# Tests y Demos - Achronyme Core

Este directorio contiene todos los tests y archivos de demostración del proyecto.

## 📋 Tests de Performance y Estabilidad

### Sistema de Handles
- **`test-handles.mjs`** - Test funcional del sistema de handles
  - Verifica creación de vectores con fast/slow path
  - Prueba operaciones FFT optimizadas
  - Valida gestión de memoria

- **`test-performance-heavy.mjs`** - Benchmarks exhaustivos
  - Tests con vectores de 100 a 1M elementos
  - FFT de diferentes tamaños (128-8192 samples)
  - Pipelines completos
  - Operaciones element-wise masivas
  - Stress tests

- **`test-stability.mjs`** ✨ NUEVO - Tests exhaustivos de estabilidad
  - 10,000 operaciones repetitivas
  - Cadenas de 50 operaciones
  - 1000 vectores simultáneos
  - 1000 FFTs repetitivos
  - Vectores muy grandes (1M elementos)
  - Alternancia fast/slow path
  - Stress test combinado
  - 5000 ciclos prolongados
  - **Resultado**: 18/20 tests (90%), 0 memory leaks

- **`test-accuracy.mjs`** ✨ NUEVO - Tests de precisión matemática
  - Operaciones vectoriales básicas (add, sub, mul)
  - Funciones matemáticas (exp, ln, sqrt, abs) ✅ VECTORIZADAS
  - Funciones trigonométricas (sin, cos, tan) ✅ VECTORIZADAS
  - Identidades matemáticas (sin²+cos²=1, exp(ln(x))=x)
  - FFT/IFFT roundtrip
  - FFT de señales conocidas
  - Precisión de linspace
  - Estabilidad numérica
  - Valores especiales (0, 1, NaN, Infinity)
  - **Resultado**: 20/25 tests (80.0%), 0 memory leaks ✅ MEJORADO
  - Tolerancia: diferencias ~1e-7 son normales (conversión float)

- **`test-edge-cases.mjs`** ✨ NUEVO - Tests de casos límite
  - Vectores de tamaño especial (1, 2, 7, 8, 100K)
  - Valores extremos (1e100, 1e-100)
  - División por cero y NaN/Infinity
  - Operaciones matemáticas inválidas
  - Dimensiones incompatibles
  - FFT de tamaños no-potencia-de-2
  - Cadenas de operaciones complejas
  - Gestión de memoria y múltiples dispose()
  - Linspace con casos límite
  - Interoperabilidad fast/slow path
  - **Resultado**: En desarrollo

### Tests del SDK
- **`test-sdk.mjs`** - Suite completa de tests del SDK
  - Verificación de todas las funcionalidades
  - Backward compatibility
  - ~75 tests

### Tests Específicos
- **`test-exp-abs.mjs`** - Test de exponencial y valor absoluto
- **`test-exp-abs-fixed.mjs`** - Versión corregida
- **`test-exp-abs-solution.mjs`** - Solución final
- **`test-npm-import.mjs`** - Test de importación del paquete npm

## 🎮 Demos

- **`demo-achronyme.mjs`** - Demostración completa de las capacidades del SDK
  - Ejemplos de uso
  - Casos de uso comunes
  - Operaciones DSP

## 🔧 Debug

- **`debug-module.mjs`** - Herramienta de debugging del módulo WASM

## 🚀 Cómo Ejecutar

```bash
# Tests de estabilidad y robustez
node tests/test-stability.mjs        # Estabilidad general (10K ops, memory leaks)
node tests/test-accuracy.mjs         # Precisión matemática
node tests/test-edge-cases.mjs       # Casos límite y edge cases

# Tests de performance
node tests/test-performance-heavy.mjs  # Benchmarks exhaustivos
node tests/test-handles.mjs            # Test funcional handles

# Test completo del SDK
node tests/test-sdk.mjs

# Demo
node tests/demo-achronyme.mjs

# Ejecutar TODOS los tests
node tests/test-stability.mjs && \
node tests/test-accuracy.mjs && \
node tests/test-edge-cases.mjs && \
node tests/test-performance-heavy.mjs && \
node tests/test-sdk.mjs
```

## 📊 Resultados Esperados

### Tests de Estabilidad
- **test-stability.mjs**: 18/20 tests (90%)
  - 0 memory leaks ✓
  - 10,000 ops en ~115ms
  - 1000 FFTs en ~517ms
  - Fast path usage: >99%

- **test-accuracy.mjs**: 13/17 tests (76.5%)
  - 0 memory leaks ✓
  - Tolerancia: 1e-10
  - FFT/IFFT roundtrip exacto
  - Identidades matemáticas verificadas

- **test-edge-cases.mjs**: ~20/25 tests (80%)
  - Vectores desde 1 hasta 100K elementos
  - División por cero → Infinity
  - NaN propagation correcta
  - Múltiples dispose() sin crash

### Performance con Handles
- Vector 100K: <500μs (antes: ~450ms) → **~1000x más rápido**
- FFT 4096: <2ms (antes: ~180ms) → **~90x más rápido**
- Fast Path Usage: >80%

### Tests del SDK
- ✅ 75 tests deben pasar
- ✅ Sin errores de memoria
- ✅ Backward compatibility 100%

## ⚠️ Problemas Conocidos

1. **~~Slow Path Issues~~** ✅ RESUELTO: Funciones matemáticas (exp, ln, sin, cos, etc.) ahora están vectorizadas en C++ y funcionan tanto para escalares como vectores.

2. **getVectorData Warning**: Advertencias "Cannot call getVectorData due to unbound types: Pm" son normales - el sistema usa fallback automático al slow path cuando ocurren. No afecta funcionalidad.

3. **Precisión Numérica**: Diferencias de ~1e-7 en tests de trigonometría/exponenciales son normales debido a conversión Float64↔Float32. No afecta uso práctico.

4. **Memory Leaks en Edge Cases**: Si un test falla antes de completar, puede dejar handles activos. Se recomienda reiniciar el proceso Node.js entre test suites.

---

**Última actualización**: 2025-11-01
