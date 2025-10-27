# Variable Persistence Solutions - Achronyme Core

## 🔍 El Problema

Cada llamada a `Module.eval()` crea un nuevo `Evaluator`, por lo que las variables no persisten:

```javascript
// ❌ Actual (stateless):
Module.eval("let x = 5")   // → "5"
Module.eval("x + 10")      // → "Error: Undefined variable 'x'"
```

**Causa Raíz** (`main.cpp:31`):
```cpp
std::string eval(const std::string& expression) {
    parser::Evaluator evaluator;  // ← Nuevo cada vez, se destruye al salir
    auto result = evaluator.evaluate(ast.get());
    return result.toString();
}
```

---

## 💡 Soluciones

### Solución 1: Evaluador Global ⭐ **[RECOMENDADA]**

**Archivo**: `main_stateful.cpp`

**Implementación**:
```cpp
static parser::Evaluator globalEvaluator;  // ← Persiste

std::string eval(const std::string& expression) {
    // Usa el evaluador global
    auto result = globalEvaluator.evaluate(ast.get());
    return result.toString();
}

std::string reset() {
    globalEvaluator.environment().clear();
    return "Environment cleared";
}
```

**Uso desde JavaScript**:
```javascript
// ✅ Variables persisten:
Module.eval("let x = 5")       // → "5"
Module.eval("x + 10")          // → "15" ✓
Module.eval("let y = [1,2,3]") // → "[1, 2, 3]"
Module.eval("map(n => n^2, y)")// → "[1, 4, 9]" ✓

// Limpiar cuando necesites:
Module.reset()                 // → "Environment cleared"
Module.eval("x")               // → "Error: Undefined variable 'x'" ✓
```

**Ventajas**:
- ✅ Simple de implementar (cambio de 1 línea)
- ✅ Comportamiento intuitivo (como Python REPL, Node.js REPL)
- ✅ Sin overhead de gestión de sesiones
- ✅ Perfecto para uso interactivo (notebooks, REPL)

**Desventajas**:
- ⚠️ Variables globales (pero controladas)
- ⚠️ Necesitas `reset()` para tests aislados
- ⚠️ Un solo contexto global

**Casos de Uso**:
- Jupyter-style notebooks
- Interactive REPL/calculator
- Scripts secuenciales
- Desarrollo/debugging

---

### Solución 2: Sistema de Sesiones

**Archivo**: `main_sessions.cpp`

**Implementación**:
```cpp
static std::unordered_map<std::string, std::shared_ptr<parser::Evaluator>> sessions;
static std::string currentSessionId = "default";

std::string eval(const std::string& expression) {
    auto& evaluator = *sessions[currentSessionId];
    auto result = evaluator.evaluate(ast.get());
    return result.toString();
}
```

**Uso desde JavaScript**:
```javascript
// Session A
Module.createSession("sessionA")
Module.useSession("sessionA")
Module.eval("let x = 5")        // → "5"
Module.eval("x + 10")           // → "15" ✓

// Session B (aislada)
Module.createSession("sessionB")
Module.useSession("sessionB")
Module.eval("let x = 100")      // → "100"
Module.eval("x + 10")           // → "110" ✓

// Volver a Session A
Module.useSession("sessionA")
Module.eval("x")                // → "5" ✓ (no afectada)

// Limpiar
Module.deleteSession("sessionB")
```

**Ventajas**:
- ✅ Múltiples contextos independientes
- ✅ Perfecto para tests paralelos
- ✅ Aislamiento total entre sesiones
- ✅ Escalable (N sesiones)

**Desventajas**:
- ⚠️ Más complejo de usar
- ⚠️ Necesita gestión explícita
- ⚠️ Overhead de memoria (N evaluadores)

**Casos de Uso**:
- Tests paralelos
- Múltiples worksheets/notebooks
- Multi-tenancy
- Sandboxing

---

### Solución 3: Híbrida (Global + Reset)

Combina Solución 1 con limpieza automática:

```cpp
static parser::Evaluator globalEvaluator;
static bool autoReset = false;  // Configurable

std::string eval(const std::string& expression) {
    if (autoReset) {
        globalEvaluator.environment().clear();
    }

    auto result = globalEvaluator.evaluate(ast.get());
    return result.toString();
}

void setAutoReset(bool enabled) {
    autoReset = enabled;
}
```

**Uso**:
```javascript
// Modo interactivo (variables persisten)
Module.setAutoReset(false)
Module.eval("let x = 5")
Module.eval("x + 10")  // → "15" ✓

// Modo tests (limpia cada vez)
Module.setAutoReset(true)
Module.eval("let x = 5")  // → "5"
Module.eval("x")          // → "Error: Undefined" ✓ (limpiado)
```

---

### Solución 4: Stateless (Actual) ✓

Mantener el diseño actual (sin persistencia):

**Ventajas**:
- ✅ Simple
- ✅ Sin estado compartido
- ✅ Thread-safe por diseño
- ✅ Predecible

**Workaround**:
```javascript
// Combinar expresiones en una sola llamada:
Module.eval("let x = 5; x + 10")  // → "15" ✓

// Usar funciones:
Module.eval("pipe(5, x => x + 10)")  // → "15" ✓
```

---

## 📊 Comparación

| Característica | Stateless | Global | Sesiones | Híbrida |
|----------------|-----------|--------|----------|---------|
| Persistencia | ❌ | ✅ | ✅ | ✅ |
| Simplicidad | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐ |
| Multi-contexto | ❌ | ❌ | ✅ | ❌ |
| Uso memoria | Bajo | Bajo | Alto | Bajo |
| Tests aislados | ✅ | Necesita reset | ✅ | Configurable |
| REPL/notebook | ⚠️ Limitado | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| API complexity | Bajo | Bajo | Alto | Medio |

---

## 🎯 Recomendación por Caso de Uso

### Para Uso Interactivo (REPL, Notebooks)
➡️ **Solución 1: Evaluador Global**
```bash
# Compilar con:
emcc ... wasm/src/bindings/main_stateful.cpp ...
```

### Para Tests Automatizados
➡️ **Solución 4: Stateless (actual)** o **Solución 3: Híbrida**
```javascript
// Tests actuales funcionan bien:
test('test-1', 'fft_mag([1,2,3,4])', /10/)  // ✓ Sin estado
```

### Para Aplicaciones Multi-Usuario
➡️ **Solución 2: Sesiones**
```javascript
users.forEach(user => {
    Module.createSession(user.id)
    Module.useSession(user.id)
    // Cada usuario tiene su contexto
})
```

---

## 🚀 Implementación Recomendada

**Para Achronyme Core**: Usar **Solución 1 (Global)**

### Paso 1: Reemplazar `main.cpp`

```bash
# Backup actual
mv wasm/src/bindings/main.cpp wasm/src/bindings/main_stateless.cpp

# Usar versión stateful
mv wasm/src/bindings/main_stateful.cpp wasm/src/bindings/main.cpp
```

### Paso 2: Recompilar

```bash
emcc wasm/src/core/complex.cpp \
     wasm/src/core/vector.cpp \
     wasm/src/core/matrix.cpp \
     wasm/src/core/value.cpp \
     wasm/src/parser/lexer.cpp \
     wasm/src/parser/parser.cpp \
     wasm/src/parser/evaluator.cpp \
     wasm/src/bindings/main.cpp \
     wasm/src/core/functions.cpp \
     wasm/src/core/functions_hof.cpp \
     wasm/src/core/functions_dsp.cpp \
     wasm/src/core/function.cpp \
     wasm/src/core/constants.cpp \
     -o dist/achronyme-core.mjs \
     -std=c++17 -O2 \
     -s WASM=1 \
     -s MODULARIZE=1 \
     -s EXPORT_ES6=1 \
     -s EXPORTED_FUNCTIONS="['_malloc', '_free']" \
     -s EXPORTED_RUNTIME_METHODS="['ccall', 'cwrap', 'UTF8ToString', 'stringToUTF8', 'lengthBytesUTF8']" \
     --bind \
     -I wasm/src
```

### Paso 3: Probar

```javascript
// Test persistencia
Module.eval("let x = 5")           // → "5"
Module.eval("x + 10")              // → "15" ✓
Module.eval("let v = [1,2,3]")     // → "[1, 2, 3]"
Module.eval("map(n => n^2, v)")    // → "[1, 4, 9]" ✓

// Test reset
Module.reset()                     // → "Environment cleared"
Module.eval("x")                   // → "Error: Undefined variable 'x'" ✓
```

### Paso 4: Actualizar Tests

```javascript
// Agregar reset antes de cada test suite
beforeEach(() => {
    Module.reset()
})

test('variables persist within session', () => {
    Module.eval("let x = 5")
    expect(Module.eval("x + 10")).toBe("15")
})
```

---

## 🔒 Consideraciones de Seguridad

### Con Persistencia Global:
- Variables pueden acumularse
- Posible memory leak si no se limpia
- Necesitas `reset()` periódicamente

### Mitigaciones:
```javascript
// Límite de variables (opcional)
if (Module.getVariableCount() > 1000) {
    Module.reset()
}

// Timeout de sesión
setInterval(() => {
    Module.reset()
}, 3600000)  // Reset cada hora
```

---

## 📝 Conclusión

Para **Achronyme Core como calculadora científica/DSP**:
- ✅ Usar **Solución 1: Evaluador Global**
- ✅ Agregar función `reset()` para limpieza
- ✅ Documentar en README.md
- ✅ Mantener `main_stateless.cpp` como backup

Esto da el mejor balance entre:
- Usabilidad (como Python/MATLAB)
- Simplicidad (1 línea de cambio)
- Performance (sin overhead)
- Funcionalidad (REPL interactivo)
