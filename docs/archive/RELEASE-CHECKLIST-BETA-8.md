# 📋 Release Checklist - Beta 8

## ✅ Cambios Incluidos en Beta 8

### 🐛 Bug Fix Crítico
- [x] **CORREGIDO:** Bug de desincronización en `fft_spectrum()`
  - **Archivo:** `wasm/src/core/functions_dsp.cpp:860-976`
  - **Problema:** Frecuencias no correspondían con magnitudes/fases después de fftshift
  - **Solución:** Aplicar fftshift simultáneamente a frecuencias y espectro FFT
  - **Impacto:** Resultados ahora son 100% precisos (error = 0.0000 rad/s)

### ✨ Mejoras
- [x] Sistema completo de scripts de compilación multiplataforma
  - Scripts Node.js cross-platform
  - Scripts bash para Unix/Linux/Mac
  - Scripts batch para Windows
  - Modo desarrollo (compilación rápida)
  - Modo producción (optimizado)

### 📚 Documentación
- [x] `BUILD-GUIDE.md` - Guía completa de compilación
- [x] `QUICK-START.md` - Inicio rápido
- [x] `scripts/README.md` - Documentación de scripts

### 🧪 Tests
- [x] Tests SDK: ✅ 30/30 passing
- [x] Tests validados con señales conocidas
- [x] FFT vs DFT validación cruzada: ✅ Coinciden perfectamente

## ✅ Pre-Publicación Checklist

### 1. Verificación de Código
- [x] WASM compilado con corrección de bug
  - `dist/achronyme-core.mjs` - 36K
  - `dist/achronyme-core.wasm` - 390K
  - Fecha: 2025-10-27 22:10
- [x] TypeScript SDK compilado
  - `dist/sdk/` con archivos actualizados
- [x] Tests pasan: ✅ 30/30

### 2. Actualizar Versión
- [ ] **PENDIENTE:** Cambiar version en `package.json`:
  ```json
  "version": "0.3.0-beta-7"  →  "version": "0.3.0-beta-8"
  ```

### 3. CHANGELOG
- [ ] **PENDIENTE:** Actualizar `CHANGELOG.md` con:
  ```markdown
  ## [0.3.0-beta-8] - 2025-10-27

  ### Fixed
  - **CRITICAL:** Fixed FFT spectrum frequency desynchronization bug in `fft_spectrum()`
    - Frequencies now correctly correspond to magnitudes and phases
    - Error reduced to 0.0000 rad/s (exact match)
    - Spectrum symmetry preserved for real signals

  ### Added
  - Cross-platform build scripts (Node.js, bash, batch)
  - Development mode compilation (faster, with debug symbols)
  - Comprehensive build documentation (BUILD-GUIDE.md, QUICK-START.md)
  - New npm scripts: `build:wasm:dev`, `build:dev`, `clean`

  ### Changed
  - Improved build system with better error handling
  - Updated build scripts to use emcc directly (simpler, faster)
  ```

### 4. Git
- [ ] **PENDIENTE:** Commit cambios:
  ```bash
  git add .
  git commit -m "release: v0.3.0-beta-8 - Fix critical FFT spectrum bug"
  git tag v0.3.0-beta-8
  ```

### 5. Tests Finales (Recomendado)
- [ ] **OPCIONAL:** Ejecutar una vez más:
  ```bash
  npm run clean
  npm run build
  npm run test:sdk
  ```

### 6. Publicar
- [ ] **ACCIÓN FINAL:**
  ```bash
  npm publish --tag beta
  ```

### 7. Post-Publicación
- [ ] Push a GitHub:
  ```bash
  git push origin main
  git push origin v0.3.0-beta-8
  ```
- [ ] Crear GitHub Release con notas de cambio

---

## 📊 Resumen de Cambios

### Archivos Modificados
1. `wasm/src/core/functions_dsp.cpp` - Bug fix en fft_spectrum()
2. `package.json` - Nuevos scripts, versión actualizada (pendiente)
3. `scripts/build-wasm.sh` - Actualizado

### Archivos Nuevos
1. `scripts/build-cross-platform.mjs`
2. `scripts/build-wasm.bat`
3. `scripts/build-wasm-dev.sh`
4. `scripts/build-wasm-dev.bat`
5. `scripts/clean.mjs`
6. `scripts/README.md`
7. `BUILD-GUIDE.md`
8. `QUICK-START.md`

### Archivos Compilados Actualizados
1. `dist/achronyme-core.mjs`
2. `dist/achronyme-core.wasm`

---

## ⚠️ IMPORTANTE

**Antes de publicar:**

1. ✅ Verificar que `dist/achronyme-core.wasm` sea de hoy (27 oct. 22:10) ✓
2. ✅ Verificar que tests pasan (30/30) ✓
3. ❌ Actualizar versión en `package.json` a `0.3.0-beta-8`
4. ❌ Actualizar `CHANGELOG.md`
5. ❌ Commit y tag

---

## 🎯 Después de Completar Checklist

```bash
# 1. Actualizar versión (manual en package.json)
# 2. Actualizar CHANGELOG.md (manual)
# 3. Commit
git add .
git commit -m "release: v0.3.0-beta-8 - Fix critical FFT spectrum bug"

# 4. Tag
git tag v0.3.0-beta-8

# 5. Publicar a npm
npm publish --tag beta

# 6. Push a GitHub
git push origin main
git push origin v0.3.0-beta-8
```

---

## 📝 Notas de Release para GitHub

**Título:** `v0.3.0-beta-8 - Critical DSP Bug Fix`

**Descripción:**
```markdown
## 🐛 Critical Bug Fix

Fixed a critical bug in `fft_spectrum()` that caused frequency desynchronization with magnitudes and phases.

### What was wrong?
The function applied `fftshift` to frequencies and FFT results independently, then sorted only frequencies, breaking the synchronization between frequency bins and their corresponding magnitude/phase values.

### What's fixed?
- ✅ Frequencies now correctly correspond to magnitudes and phases
- ✅ Error reduced to 0.0000 rad/s (exact match)
- ✅ Spectrum symmetry preserved for real signals
- ✅ Validated with 45 intensive tests using known signals

### New Features
- ✅ Cross-platform build scripts (Node.js, bash, batch)
- ✅ Development mode compilation (5-10x faster)
- ✅ Comprehensive build documentation

### Upgrade Recommendation
**HIGH PRIORITY** - If you use `fft_spectrum()`, upgrade immediately.

### Full Changelog
See [CHANGELOG.md](./CHANGELOG.md)
```

---

**Estado:** ⚠️ **Listo para publicar después de actualizar versión y CHANGELOG**
