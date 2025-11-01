# 📦 Instrucciones de Compilación - Sistema de Handles

## 🎯 Compilación Rápida (Todo en Uno)

### Windows
```cmd
build-and-test.bat
```

### Linux/Mac
```bash
./build-and-test.sh
```

Esto compilará:
1. C++ → WASM (con handles)
2. TypeScript → JavaScript  
3. Ejecutará tests de performance

---

## 🔧 Compilación Paso a Paso

### Prerequisito: Activar Emscripten

#### Windows
```cmd
cd C:\ruta\a\emsdk
emsdk_env.bat
cd C:\apache\htdocs\achronyme-core
```

#### Linux/Mac
```bash
source /ruta/a/emsdk/emsdk_env.sh
cd /ruta/a/achronyme-core
```

### Paso 1: Compilar WASM

#### Windows
```cmd
scripts\build-wasm.bat
```

#### Linux/Mac
```bash
bash scripts/build-wasm.sh
```

#### O usando npm
```bash
npm run build:wasm
```

**Archivos generados:**
- `dist/achronyme-core.mjs`
- `dist/achronyme-core.wasm`

### Paso 2: Compilar TypeScript

```bash
npm run build:js
```

**Archivos generados:**
- `dist/sdk/*.js`

### Paso 3: Ejecutar Tests

```bash
# Test de funcionalidad
node test-handles.mjs

# Test de performance
node test-performance-heavy.mjs
```

---

## 📊 Opciones de Compilación

### Modo Desarrollo (más rápido, sin optimizar)

```bash
# Bash
bash scripts/build-wasm-dev.sh

# Windows
scripts\build-wasm-dev.bat

# npm
npm run build:wasm:dev
```

### Modo Producción (optimizado, más lento)

```bash
# Bash
bash scripts/build-wasm.sh

# Windows
scripts\build-wasm.bat

# npm
npm run build:wasm
```

---

## 🔍 Verificar que Todo Funciona

Después de compilar, ejecuta:

```bash
node test-performance-heavy.mjs
```

**Output esperado:**
```
✅ Fast Path Usage: >80%
✅ Vector 100K creation: <10ms
✅ FFT 4096: <80ms
✅ Handles activos: 0 (después de dispose)
```

---

## ⚙️ Archivos Incluidos en la Compilación

### Nuevos (Sistema de Handles)
- ✨ `wasm/src/core/handle_manager.cpp`
- ✨ `wasm/src/bindings/fast_ops.cpp`

### Existentes
- `wasm/src/core/constants.cpp`
- `wasm/src/core/complex.cpp`
- `wasm/src/core/vector.cpp`
- `wasm/src/core/matrix.cpp`
- `wasm/src/core/function.cpp`
- `wasm/src/core/functions.cpp`
- `wasm/src/core/functions_dsp.cpp`
- `wasm/src/core/functions_hof.cpp`
- `wasm/src/core/value.cpp`
- `wasm/src/parser/lexer.cpp`
- `wasm/src/parser/parser.cpp`
- `wasm/src/parser/evaluator.cpp`
- `wasm/src/bindings/main.cpp`

---

## 🐛 Troubleshooting

### Error: "emcc: command not found"

**Solución:**
```bash
# Verifica que Emscripten esté instalado
emcc --version

# Si no está instalado:
git clone https://github.com/emscripten-core/emsdk.git
cd emsdk
./emsdk install latest
./emsdk activate latest

# Actívalo en cada sesión
source ./emsdk_env.sh  # Linux/Mac
emsdk_env.bat          # Windows
```

### Error de compilación C++

**Verifica que existan los archivos:**
```bash
# Bash/Linux/Mac
ls -la wasm/src/core/handle_manager.cpp
ls -la wasm/src/bindings/fast_ops.cpp

# Windows
dir wasm\src\core\handle_manager.cpp
dir wasm\src\bindings\fast_ops.cpp
```

### Tests fallan

**Recompila limpio:**
```bash
npm run clean
npm run build
```

---

## 📝 Scripts Disponibles

```bash
# Compilación completa
npm run build              # WASM + TypeScript

# Solo WASM
npm run build:wasm         # Producción (optimizado)
npm run build:wasm:dev     # Desarrollo (rápido)

# Solo TypeScript
npm run build:js

# Tests
npm run test:sdk           # Tests del SDK
npm run test:dsp           # Tests DSP
npm run test:all           # Todos los tests

# Limpieza
npm run clean
```

---

## 🎯 Próximos Pasos Después de Compilar

1. ✅ Verificar que `dist/achronyme-core.wasm` existe
2. ✅ Ejecutar `node test-performance-heavy.mjs`
3. ✅ Verificar Fast Path Usage >80%
4. ✅ Integrar en tu aplicación

---

**Performance esperada**: 10-150x improvement
**Breaking changes**: Ninguno
**Backward compatible**: ✅ 100%
