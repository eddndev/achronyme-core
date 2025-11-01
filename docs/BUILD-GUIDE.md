# 📘 Guía de Compilación - Achronyme Core

## 🚀 Inicio Rápido

### Instalación de Dependencias

```bash
npm install
```

### Compilación Completa

```bash
# Compilación de producción (optimizada)
npm run build

# Compilación de desarrollo (más rápida, sin optimizar)
npm run build:dev
```

### Ejecutar Tests

```bash
# Tests del SDK completo
npm run test:sdk

# Tests intensivos de DSP
npm run test:dsp

# Todos los tests
npm run test:all
```

---

## 📦 Comandos de Compilación

### WASM (WebAssembly)

#### Modo Producción
```bash
npm run build:wasm
```
- **Optimización:** `-O3` (máxima)
- **Tiempo:** ~30-60 segundos
- **Uso:** Para releases y producción

#### Modo Desarrollo
```bash
npm run build:wasm:dev
```
- **Optimización:** `-O0` (ninguna)
- **Debug:** Símbolos incluidos (`-g`)
- **Tiempo:** ~5-10 segundos
- **Uso:** Para desarrollo rápido

### TypeScript

```bash
npm run build:js
```
- Compila el SDK de TypeScript
- Usa `tsconfig.json` y `tsconfig.sdk.json`
- Salida: `dist/sdk/`

### Compilación Completa

```bash
# Producción: WASM optimizado + TypeScript
npm run build

# Desarrollo: WASM rápido + TypeScript
npm run build:dev
```

---

## 🧪 Tests

### Tests del SDK (30 tests)
```bash
npm run test:sdk
```
Valida:
- Operaciones básicas (números, vectores, matrices)
- Funciones matemáticas
- DSP (FFT, convolución, ventanas)
- Funciones de optimización
- Programación funcional
- Gestión de memoria

### Tests DSP Intensivos (45 tests)
```bash
npm run test:dsp
```
Valida con señales conocidas:
- Sinusoides puras (diferentes frecuencias)
- Señales complejas (suma de sinusoides)
- Señales especiales (DC, impulso)
- FFT vs DFT (validación cruzada)
- Fase y simetría conjugada
- Diferentes tamaños de señal

### Todos los Tests
```bash
npm run test:all
```
Ejecuta ambas suites (75 tests en total).

---

## 🧹 Limpieza

```bash
npm run clean
```

Elimina:
- `dist/` - Archivos compilados
- `wasm/build/` - Archivos intermedios
- `node_modules/.cache/` - Cache de Node

---

## ⚙️ Requisitos del Sistema

### Emscripten (para compilar WASM)

**Instalación:**

```bash
# 1. Clonar emsdk
git clone https://github.com/emscripten-core/emsdk.git
cd emsdk

# 2. Instalar última versión
./emsdk install latest      # Linux/Mac
emsdk install latest         # Windows

# 3. Activar
./emsdk activate latest      # Linux/Mac
emsdk activate latest        # Windows

# 4. Configurar PATH (en cada sesión)
source ./emsdk_env.sh        # Linux/Mac
emsdk_env.bat                # Windows
```

**Verificar instalación:**
```bash
emcc --version
```

### Node.js

- **Mínimo:** Node.js 18.0.0
- **Recomendado:** Node.js 20+ (LTS)

**Verificar instalación:**
```bash
node --version
```

### TypeScript

Se instala automáticamente con `npm install`.

---

## 🔄 Workflow de Desarrollo

### Durante Desarrollo Activo

```bash
# 1. Compilación rápida
npm run build:dev

# 2. Hacer cambios en el código

# 3. Recompilar solo WASM (más rápido)
npm run build:wasm:dev

# 4. Ejecutar tests
npm run test:sdk
```

### Antes de Hacer Commit

```bash
# 1. Limpiar todo
npm run clean

# 2. Compilación completa de producción
npm run build

# 3. Ejecutar todos los tests
npm run test:all
```

Si todos los tests pasan (✅ 75/75), el código está listo para commit.

### Antes de Release/Publish

```bash
# 1. Limpiar
npm run clean

# 2. Compilar
npm run build

# 3. Tests completos
npm run test:all

# 4. Optimización adicional (opcional)
npm run optimize

# 5. Verificar package.json version

# 6. Publish
npm publish
```

---

## 🛠️ Compilación Manual (Alternativa)

Si prefieres compilar manualmente sin usar npm:

### Unix/Linux/Mac

```bash
# Producción
bash scripts/build-wasm.sh

# Desarrollo
bash scripts/build-wasm-dev.sh
```

### Windows (CMD/PowerShell)

```bash
# Producción
scripts\build-wasm.bat

# Desarrollo
scripts\build-wasm-dev.bat
```

### Comando emcc directo

```bash
emcc \
  wasm/src/core/*.cpp \
  wasm/src/parser/*.cpp \
  wasm/src/bindings/main.cpp \
  -I wasm/src \
  -o dist/achronyme-core.mjs \
  -s WASM=1 \
  -s ALLOW_MEMORY_GROWTH=1 \
  -s MODULARIZE=1 \
  -s EXPORT_ES6=1 \
  -s EXPORT_NAME='AchronymeCore' \
  -s ENVIRONMENT='web,worker,node' \
  --bind \
  -fexceptions \
  -O3 \
  -std=c++17
```

---

## 🐛 Solución de Problemas

### Error: "emcc: command not found"

**Causa:** Emscripten no está en el PATH.

**Solución:**
```bash
# Activar emsdk en la sesión actual
source /path/to/emsdk/emsdk_env.sh  # Linux/Mac
C:\path\to\emsdk\emsdk_env.bat      # Windows
```

### Error: "Cannot find module"

**Causa:** Dependencias no instaladas.

**Solución:**
```bash
npm install
```

### Tests fallan después de modificar C++

**Causa:** WASM no recompilado.

**Solución:**
```bash
npm run build:wasm:dev  # Recompilar solo WASM
npm run test:all        # Re-ejecutar tests
```

### Compilación muy lenta

**Causa:** Modo producción usa `-O3` (optimización máxima).

**Solución:** Usar modo desarrollo durante el desarrollo:
```bash
npm run build:dev  # 5-10 segundos vs 30-60 segundos
```

### Error: "ENOENT: no such file or directory, open 'dist/...'"

**Causa:** Directorio `dist/` no existe.

**Solución:**
```bash
mkdir dist
npm run build:wasm
```

---

## 📊 Estructura de Archivos

```
achronyme-core/
├── dist/                          # Salida de compilación
│   ├── achronyme-core.mjs        # Módulo WASM (ES6)
│   ├── achronyme-core.wasm       # Binario WASM
│   └── sdk/                      # SDK TypeScript compilado
│       ├── index.js
│       ├── index.d.ts
│       └── ...
├── wasm/                          # Código fuente C++
│   └── src/
│       ├── core/                 # Núcleo (vectores, matrices, DSP)
│       ├── parser/               # Parser y evaluador
│       └── bindings/             # Bindings Emscripten
├── src/                           # Código fuente TypeScript
│   └── sdk/                      # SDK TypeScript
├── scripts/                       # Scripts de compilación
│   ├── build-cross-platform.mjs  # Script universal Node.js
│   ├── build-wasm.sh             # Script Unix/Mac
│   ├── build-wasm.bat            # Script Windows
│   ├── build-wasm-dev.sh         # Script desarrollo Unix/Mac
│   ├── build-wasm-dev.bat        # Script desarrollo Windows
│   ├── clean.mjs                 # Script limpieza
│   └── README.md                 # Documentación scripts
├── test-sdk.mjs                   # Suite tests SDK
├── test-dsp-intensive.mjs         # Suite tests DSP
├── package.json                   # Configuración npm
├── tsconfig.json                  # Configuración TypeScript
└── BUILD-GUIDE.md                 # Esta guía
```

---

## 📝 Notas Importantes

1. **Siempre activa Emscripten** antes de compilar WASM:
   ```bash
   source /path/to/emsdk/emsdk_env.sh  # Cada sesión nueva
   ```

2. **Usa modo desarrollo** durante el desarrollo para compilaciones rápidas:
   ```bash
   npm run build:dev
   ```

3. **Ejecuta tests** después de cada cambio importante:
   ```bash
   npm run test:all
   ```

4. **Limpia antes de compilar** para evitar problemas de cache:
   ```bash
   npm run clean && npm run build
   ```

5. **Los scripts de Node.js** (`npm run ...`) funcionan en todas las plataformas (recomendado).

6. **Los scripts bash/bat** son alternativos y específicos de plataforma.

---

## 🎯 Ejemplo Completo de Sesión de Desarrollo

```bash
# 1. Clonar repositorio
git clone https://github.com/eddndev/achronyme-core.git
cd achronyme-core

# 2. Instalar dependencias
npm install

# 3. Activar Emscripten
source /path/to/emsdk/emsdk_env.sh  # Linux/Mac
# o
C:\path\to\emsdk\emsdk_env.bat      # Windows

# 4. Compilar todo (desarrollo)
npm run build:dev

# 5. Ejecutar tests
npm run test:all

# 6. Hacer cambios en wasm/src/core/functions_dsp.cpp

# 7. Recompilar solo WASM
npm run build:wasm:dev

# 8. Re-ejecutar tests DSP
npm run test:dsp

# 9. Todo correcto? Compilar producción
npm run clean
npm run build
npm run test:all

# 10. Commit y push
git add .
git commit -m "feat: Mejorar función FFT"
git push
```

---

## 📚 Referencias

- **Emscripten:** https://emscripten.org/
- **WebAssembly:** https://webassembly.org/
- **Node.js:** https://nodejs.org/
- **TypeScript:** https://www.typescriptlang.org/

---

## ✅ Checklist Pre-Release

- [ ] `npm run clean`
- [ ] `npm run build`
- [ ] `npm run test:all` (75/75 tests pasan)
- [ ] Actualizar `package.json` version
- [ ] Actualizar `CHANGELOG.md`
- [ ] Commit y tag de versión
- [ ] `npm publish`
- [ ] Crear GitHub release

---

**¿Problemas?** Abre un issue en: https://github.com/eddndev/achronyme-core/issues
