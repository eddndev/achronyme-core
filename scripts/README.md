# Build Scripts - Achronyme Core

Este directorio contiene los scripts de compilación para Achronyme Core.

## 📦 Scripts Disponibles

### Compilación WASM

#### `npm run build:wasm`
Compila el código C++ a WebAssembly (modo producción optimizado).

```bash
npm run build:wasm
```

- **Optimización:** `-O3` (máxima optimización)
- **Salida:** `dist/achronyme-core.mjs` y `dist/achronyme-core.wasm`
- **Uso:** Para producción y releases

#### `npm run build:wasm:dev`
Compila el código C++ a WebAssembly (modo desarrollo).

```bash
npm run build:wasm:dev
```

- **Optimización:** `-O0` (sin optimización)
- **Debug:** Incluye símbolos de debug (`-g`)
- **Compilación:** Mucho más rápida
- **Uso:** Durante desarrollo para pruebas rápidas

### Compilación TypeScript

#### `npm run build:js`
Compila el SDK de TypeScript.

```bash
npm run build:js
```

- **Salida:** `dist/sdk/`
- **Usa:** `tsconfig.json` y `tsconfig.sdk.json`

### Compilación Completa

#### `npm run build`
Compila WASM (producción) + TypeScript.

```bash
npm run build
```

Equivalente a:
```bash
npm run build:wasm && npm run build:js
```

#### `npm run build:dev`
Compila WASM (desarrollo) + TypeScript.

```bash
npm run build:dev
```

Equivalente a:
```bash
npm run build:wasm:dev && npm run build:js
```

### Tests

#### `npm run test:sdk`
Ejecuta los tests del SDK completo (30 tests).

```bash
npm run test:sdk
```

#### `npm run test:dsp`
Ejecuta los tests intensivos de DSP (45 tests).

```bash
npm run test:dsp
```

#### `npm run test:all`
Ejecuta todos los tests (SDK + DSP).

```bash
npm run test:all
```

### Limpieza

#### `npm run clean`
Limpia todos los archivos generados.

```bash
npm run clean
```

Elimina:
- `dist/`
- `wasm/build/`
- `node_modules/.cache/`

## 🔧 Scripts Internos

Los siguientes scripts están en este directorio pero no deberían ejecutarse directamente:

### `build-cross-platform.mjs`
Script universal de Node.js que funciona en Windows, Linux y macOS.

- **Usado por:** `npm run build:wasm` y `npm run build:wasm:dev`
- **Detecta:** Sistema operativo automáticamente
- **Verifica:** Instalación de Emscripten

### `build-wasm.sh` / `build-wasm.bat`
Scripts específicos de plataforma (alternativa legacy).

- **Unix/Mac:** `bash scripts/build-wasm.sh`
- **Windows:** `scripts\build-wasm.bat`

### `build-wasm-dev.sh` / `build-wasm-dev.bat`
Scripts de desarrollo específicos de plataforma (alternativa legacy).

- **Unix/Mac:** `bash scripts/build-wasm-dev.sh`
- **Windows:** `scripts\build-wasm-dev.bat`

### `clean.mjs`
Script de limpieza multiplataforma.

- **Usado por:** `npm run clean`

## 📋 Workflow Recomendado

### Durante Desarrollo

```bash
# 1. Compilación rápida (desarrollo)
npm run build:dev

# 2. Ejecutar tests
npm run test:all

# 3. Modificar código C++ y recompilar
npm run build:wasm:dev  # Solo recompila WASM
```

### Antes de Commit

```bash
# 1. Limpieza completa
npm run clean

# 2. Compilación de producción
npm run build

# 3. Todos los tests
npm run test:all
```

### Para Release

```bash
# 1. Limpieza
npm run clean

# 2. Compilación optimizada
npm run build

# 3. Tests
npm run test:all

# 4. Optimización adicional (opcional)
npm run optimize

# 5. Publish
npm publish
```

## ⚙️ Requisitos

### Emscripten

Para compilar WASM, necesitas Emscripten instalado:

```bash
# Clonar emsdk
git clone https://github.com/emscripten-core/emsdk.git
cd emsdk

# Instalar y activar
./emsdk install latest
./emsdk activate latest

# Activar en la sesión actual
source ./emsdk_env.sh  # Linux/Mac
# o
emsdk_env.bat          # Windows
```

### Node.js

- **Versión mínima:** Node.js 18.0.0
- **Recomendado:** Node.js 20+ (LTS)

### TypeScript

Instalado automáticamente como devDependency:

```bash
npm install
```

## 🐛 Troubleshooting

### "emcc: command not found"

**Problema:** Emscripten no está en el PATH.

**Solución:**
```bash
# Activar emsdk en la sesión actual
source /path/to/emsdk/emsdk_env.sh  # Linux/Mac
# o
C:\path\to\emsdk\emsdk_env.bat      # Windows
```

### "Cannot find module"

**Problema:** Dependencias no instaladas.

**Solución:**
```bash
npm install
```

### Build lento

**Problema:** La compilación optimizada (`-O3`) es lenta.

**Solución:** Usa el modo desarrollo durante el desarrollo:
```bash
npm run build:dev  # Mucho más rápido
```

### Tests fallan

**Problema:** El WASM no se compiló o está desactualizado.

**Solución:**
```bash
npm run clean
npm run build
npm run test:all
```

## 📊 Tiempos de Compilación Aproximados

- **Modo desarrollo** (`build:wasm:dev`): ~5-10 segundos
- **Modo producción** (`build:wasm`): ~30-60 segundos
- **TypeScript** (`build:js`): ~2-5 segundos
- **Completo** (`build`): ~35-65 segundos

*Los tiempos varían según el hardware.*

## 🎯 Ejemplos de Uso

### Desarrollo rápido con auto-watch

```bash
# Terminal 1: Compilar en modo dev
npm run build:dev

# Terminal 2: Watch tests
npm run test:watch

# Modificar código y volver a compilar según necesites
```

### Validar todo antes de commit

```bash
npm run clean && npm run build && npm run test:all
```

### Debug de C++

```bash
# Compilar con símbolos de debug
npm run build:wasm:dev

# Los archivos .wasm y .mjs tendrán información de debug
```

## 📝 Notas

- Los scripts de bash (`.sh`) requieren Bash/Git Bash en Windows
- Los scripts de Node.js (`.mjs`) funcionan en todas las plataformas
- **Se recomienda usar los comandos npm** (`npm run build:wasm`) en lugar de ejecutar los scripts directamente
- Los archivos `.bat` son para compatibilidad con CMD de Windows
