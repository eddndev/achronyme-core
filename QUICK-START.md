# ⚡ Quick Start - Achronyme Core

## 🎯 3 Pasos para Empezar

### 1. Instalar Dependencias
```bash
npm install
```

### 2. Compilar
```bash
npm run build
```

### 3. Ejecutar Tests
```bash
npm run test:all
```

Si ves `✅ 75 tests passed`, ¡todo funciona correctamente!

---

## 📦 Comandos Más Usados

```bash
# Compilación rápida (desarrollo)
npm run build:dev

# Compilar solo WASM (producción)
npm run build:wasm

# Compilar solo WASM (desarrollo, más rápido)
npm run build:wasm:dev

# Compilar solo TypeScript
npm run build:js

# Tests del SDK
npm run test:sdk

# Tests DSP intensivos
npm run test:dsp

# Limpiar todo
npm run clean
```

---

## 🔧 Requisitos

### Emscripten (Obligatorio)

```bash
# Clonar emsdk
git clone https://github.com/emscripten-core/emsdk.git
cd emsdk

# Instalar
./emsdk install latest      # Linux/Mac
emsdk install latest         # Windows

# Activar
./emsdk activate latest

# Configurar PATH (cada sesión)
source ./emsdk_env.sh        # Linux/Mac
emsdk_env.bat                # Windows
```

### Node.js 18+

Verifica tu versión:
```bash
node --version  # Debe ser >= 18.0.0
```

---

## 🚀 Workflow de Desarrollo

```bash
# 1. Compilar en modo desarrollo (rápido)
npm run build:dev

# 2. Hacer cambios en el código

# 3. Recompilar (solo si cambiaste C++)
npm run build:wasm:dev

# 4. Ejecutar tests
npm run test:sdk
```

---

## 📚 Más Información

- **Guía completa:** [`BUILD-GUIDE.md`](./BUILD-GUIDE.md)
- **Scripts disponibles:** [`scripts/README.md`](./scripts/README.md)
- **Documentación SDK:** [`docs/sdk/`](./docs/sdk/)

---

## ❓ Problemas Comunes

### "emcc: command not found"
```bash
# Activar Emscripten
source /path/to/emsdk/emsdk_env.sh  # Linux/Mac
C:\path\to\emsdk\emsdk_env.bat      # Windows
```

### Tests fallan
```bash
# Recompilar todo limpio
npm run clean
npm run build
npm run test:all
```

### Compilación lenta
```bash
# Usar modo desarrollo (mucho más rápido)
npm run build:dev
```

---

**¿Listo?** ¡Comienza con `npm run build`! 🎉
