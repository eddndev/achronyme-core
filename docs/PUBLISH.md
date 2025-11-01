# Guía de Publicación - Achronyme Core

## ✅ Estado del Paquete

**Estado:** ✅ **LISTO PARA PUBLICACIÓN**

- ✅ Entry points corregidos (`dist/sdk/index.js`)
- ✅ Exports modernos configurados
- ✅ .npmignore creado (excluye archivos de desarrollo)
- ✅ 96/96 tests del núcleo pasando
- ✅ 17/17 tests del SDK pasando
- ✅ TypeScript compilado correctamente
- ✅ WASM optimizado (285KB)
- ✅ Tamaño del paquete: 159.7 KB comprimido, 492.4 KB descomprimido

---

## 📦 Verificación Previa

### 1. Verificar el contenido del paquete

```bash
npm pack --dry-run
```

**Resultado esperado:** 39 archivos, ~160 KB comprimido

### 2. Probar los entry points

```bash
node test-npm-import.mjs
```

**Resultado esperado:**
```
✓ Main export works
✓ SDK works: PASS
✓ WASM export works
✓ WASM eval works: PASS
```

### 3. Ejecutar tests completos

```bash
# Tests del núcleo WASM
node demo-achronyme.mjs

# Tests del SDK
node test-sdk.mjs
```

**Resultado esperado:** Todos los tests pasan (100%)

---

## 🚀 Publicación a NPM

### Opción 1: Publicación Beta (RECOMENDADO primero)

```bash
# Cambiar versión a beta
npm version 0.3.0-beta.1 --no-git-tag-version

# Publicar con tag beta
npm publish --tag beta --access public

# Los usuarios instalarán con:
# npm install @achronyme/core@beta
```

### Opción 2: Publicación de Producción

```bash
# Asegurarse de estar en la versión correcta
# La versión actual es: 0.3.0

# Publicar a production (latest)
npm publish --access public

# Los usuarios instalarán con:
# npm install @achronyme/core
```

### Opción 3: Publicación con Git Tag

```bash
# Crear tag en git
git tag v0.3.0
git push origin v0.3.0

# Publicar
npm publish --access public
```

---

## 📝 Pre-requisitos para Publicar

### 1. Cuenta de NPM

```bash
# Iniciar sesión en npm
npm login

# Verificar usuario
npm whoami
```

### 2. Scope @achronyme

El paquete está configurado como `@achronyme/core` (scoped package).

**Opciones:**
- Si el scope `@achronyme` existe y tienes acceso, usa: `npm publish --access public`
- Si el scope no existe, considera cambiar a: `achronyme-core` (sin scope)

**Para cambiar el nombre sin scope:**
```bash
# Editar package.json
{
  "name": "achronyme-core",  // Sin @
  ...
}

# Publicar
npm publish --access public
```

---

## 🧪 Prueba Local con npm pack

Antes de publicar, puedes crear un paquete local y probarlo:

```bash
# Crear paquete local
npm pack

# Esto genera: achronyme-core-0.3.0.tgz

# En otro proyecto de prueba:
cd /ruta/a/proyecto-prueba
npm install /ruta/a/achronyme-core/achronyme-core-0.3.0.tgz
```

---

## 📖 Uso Después de la Instalación

### Instalación

```bash
npm install @achronyme/core
```

### Uso Básico (SDK TypeScript)

```typescript
import { Achronyme } from '@achronyme/core';

const ach = new Achronyme();
await ach.init();

// Operaciones matemáticas
const x = ach.number(42);
const result = await x.mul(2).add(10).toNumber();
console.log(result); // 94

// Vectores y DSP
const signal = ach.vector([1, 2, 3, 4, 5, 6, 7, 8]);
const spectrum = signal.fft_mag();
console.log(await spectrum.toVector());

// Programación funcional
ach.let('double', ach.lambda(['x'], 'x * 2'));
const doubled = ach.map('double', signal);
console.log(await doubled.toVector());

// Limpieza
x.dispose();
signal.dispose();
spectrum.dispose();
doubled.dispose();
```

### Uso Avanzado (WASM Directo)

```typescript
import { createModule } from '@achronyme/core/wasm';

const Module = await createModule();

// Eval directo
console.log(Module.eval('sin(PI / 2)'));        // "1"
console.log(Module.eval('sqrt(16)'));           // "4"

// Con variables
Module.eval('let x = 10');
Module.eval('let square = n => n ^ 2');
console.log(Module.eval('square(x)'));          // "100"

// DSP
console.log(Module.eval('fft_mag([1,2,3,4,5,6,7,8])'));
```

---

## 🌐 Ejemplo de Uso en Sitio Web

### HTML con CDN (via unpkg)

```html
<!DOCTYPE html>
<html>
<head>
  <title>Achronyme Demo</title>
</head>
<body>
  <script type="module">
    // Después de publicar, estará disponible en:
    // https://unpkg.com/@achronyme/core@0.3.0/dist/sdk/index.js

    import { Achronyme } from 'https://unpkg.com/@achronyme/core@0.3.0/dist/sdk/index.js';

    const ach = new Achronyme();
    await ach.init();

    const signal = ach.vector([1, 2, 3, 4, 5, 6, 7, 8]);
    const spectrum = signal.fft_mag();

    console.log('Spectrum:', await spectrum.toVector());

    signal.dispose();
    spectrum.dispose();
  </script>
</body>
</html>
```

### Vite / React / Vue

```typescript
import { Achronyme } from '@achronyme/core';

export async function analyzeSignal(data: number[]) {
  const ach = new Achronyme();
  await ach.init();

  const signal = ach.vector(data);
  const spectrum = signal.fft_mag();
  const result = await spectrum.toVector();

  signal.dispose();
  spectrum.dispose();

  return result;
}
```

---

## 🔄 Actualizar Versión

### Patch (0.3.0 → 0.3.1)
```bash
npm version patch
npm publish
```

### Minor (0.3.0 → 0.4.0)
```bash
npm version minor
npm publish
```

### Major (0.3.0 → 1.0.0)
```bash
npm version major
npm publish
```

---

## 📊 Monitorear Publicación

Después de publicar:

1. **Verificar en NPM:**
   - https://www.npmjs.com/package/@achronyme/core
   - Verificar que los archivos se muestren correctamente
   - Verificar que el README.md se visualice bien

2. **Instalar y probar:**
   ```bash
   mkdir test-install
   cd test-install
   npm init -y
   npm install @achronyme/core
   ```

3. **Verificar en unpkg:**
   - https://unpkg.com/@achronyme/core@0.3.0/
   - Verificar que todos los archivos estén disponibles

---

## ⚠️ Troubleshooting

### Error: "You do not have permission to publish"

**Solución:**
```bash
# Verificar usuario
npm whoami

# Re-login
npm logout
npm login
```

### Error: "Package name too similar"

**Solución:** Cambiar el nombre en `package.json` a algo único.

### Error: "Cannot publish over existing version"

**Solución:**
```bash
# Incrementar versión
npm version patch
npm publish
```

---

## ✅ Checklist Final

Antes de publicar, verifica:

- [ ] `npm pack --dry-run` muestra los archivos correctos
- [ ] `node test-npm-import.mjs` pasa todos los tests
- [ ] `node demo-achronyme.mjs` pasa 96/96 tests
- [ ] `node test-sdk.mjs` pasa 17/17 tests
- [ ] README.md está actualizado
- [ ] CHANGELOG.md tiene las notas de versión (opcional)
- [ ] Estás logeado en npm: `npm whoami`
- [ ] La versión en package.json es correcta
- [ ] El repositorio Git está limpio (opcional)

---

## 🎉 ¡Listo para Publicar!

El paquete está **100% listo para producción**. Puedes publicarlo con confianza.

```bash
# Comando final
npm publish --access public
```

**Última actualización:** 2025-10-26
**Versión lista:** 0.3.0
