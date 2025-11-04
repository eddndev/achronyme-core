# Problema: panic_already_borrowed en WASM con RefCell

## Síntomas

1. **Funciones básicas funcionan:** `createVector()`, `getVector()` siempre funcionan
2. **Operaciones matemáticas SIEMPRE fallan:** `sin()`, `cos()`, etc. fallan incluso como primer test
3. **Otras operaciones fallan después de un error:** `linspace()` funciona al inicio, pero falla si se ejecuta después de un test fallido
4. El error específico: `RuntimeError: unreachable` en `core::cell::panic_already_borrowed`
5. Una vez que ocurre el panic, el módulo WASM queda corrupto y todas las operaciones subsecuentes fallan

## Contexto Histórico

**Funcionaba perfectamente antes:** Con `wasm-pack build --target web` todo funcionaba:
- Benchmarks exitosos con 10M elementos
- Sin errores de borrow
- Performance 1.43x más rápido que JavaScript nativo
- FFT procesando 16M samples sin problemas

**Cambio que causó el problema:** Migración de `--target web` a `--target bundler` para integración con TypeScript SDK.

## Arquitectura Rust (WASM)

### Estructura Global

```rust
thread_local! {
    static EVALUATOR: RefCell<Evaluator> = RefCell::new(Evaluator::new());
    static HANDLES: RefCell<HandleManager> = RefCell::new(HandleManager::new());
}

pub type Handle = u32;

struct HandleManager {
    next_handle: Handle,
    values: HashMap<Handle, Value>,
}
```

### Patrón de Operaciones (Ejemplo: sin)

```rust
// Patrón actual que falla:
fn apply_unary_op<F>(handle: Handle, f: F) -> Result<Handle, JsValue>
where F: Fn(&Vector) -> Vector
{
    // Paso 1: Leer valor (borrow inmutable)
    let result_vec = HANDLES.with(|h| {
        let handles = h.borrow();  // ← RefCell::borrow()
        match handles.get(handle) {
            Some(Value::Vector(vec)) => Ok(f(vec)),
            // ...
        }
    })?;

    // Paso 2: Crear nuevo handle (borrow mutable)
    let new_handle = HANDLES.with(|h| {
        h.borrow_mut().create(Value::Vector(result_vec))  // ← RefCell::borrow_mut()
    });

    Ok(new_handle)
}

#[wasm_bindgen]
pub fn sin(handle: Handle) -> Result<Handle, JsValue> {
    apply_unary_op(handle, |v: &Vector| {
        let result: Vec<f64> = v.data().iter().map(|x| x.sin()).collect();
        Vector::new(result)
    })
}
```

### Funciones Afectadas

**Operaciones matemáticas (vía `apply_unary_op`):**
- sin, cos, tan, exp, ln, abs, sqrt

**Operaciones vectoriales (vía `apply_binary_op`):**
- vadd, vsub, vmul, vdiv, dot

**Operaciones DSP:**
- fft, fft_mag, ifft, linspace

**Álgebra lineal:**
- lu_decomposition, qr_decomposition, cholesky_decomposition
- svd_decomposition, eigen_symmetric, power_iteration

## Arquitectura TypeScript (SDK)

### Flujo de Llamada

```
Usuario → Achronyme.sin(vector)
    ↓
  MathOps.sin(value)
    ↓
  RustWASM.sin(handle) → wasm.sin(handle)
    ↓
  WASM sin() → apply_unary_op()
    ↓
  HANDLES.with() → panic_already_borrowed ❌
```

### HandleManager (TypeScript)

```typescript
export class HandleManager {
    private handles = new Map<Handle, Value>();
    private allocatedCount = 0;
    private freedCount = 0;

    register(handle: Handle, value: Value): void {
        this.handles.set(handle, value);
        this.allocatedCount++;
    }

    release(handle: Handle): void {
        if (!this.handles.has(handle)) return;
        this.handles.delete(handle);

        try {
            this.wasm.releaseHandle(handle);  // ← Llama a WASM
            this.freedCount++;
        } catch (error) {
            console.warn(`Failed to release handle ${handle}:`, error);
        }
    }
}
```

### Session (TypeScript)

```typescript
export class AchronymeSession {
    private values = new Set<Value>();

    // NO hay cleanup automático (eliminado para evitar race conditions)
    async use<T>(fn: () => Promise<T> | T): Promise<T> {
        return await fn();  // Sin cleanup en finally
    }

    track(value: Value): void {
        this.values.add(value);
        this.handleManager.register(value.handle, value);
    }
}
```

## Errores Observados

### Stack Trace Típico

```
RuntimeError: unreachable
  at achronyme_wasm.wasm.__rustc::__rust_abort
  at achronyme_wasm.wasm.core::cell::panic_already_borrowed
  at achronyme_wasm.wasm.sin
```

### Secuencia de Error

```
Test 1 (Basic Operations) - Ejecutado primero:
  ✅ createVector([1,2,3,4,5]) → handle=1
  ✅ linspace(0, 10, 5) → handle=2
  ✅ getVector(1) → [1,2,3,4,5]

Test 2 (Math Operations) - Ejecutado segundo O como primer test:
  ✅ createVector([0, π/4, π/2, π]) → handle=3
  ❌ sin(handle=3) → panic_already_borrowed (FALLA SIEMPRE)

Test 3 (DSP Operations) - Después de error en Test 2:
  ❌ linspace(0, 2π, 64) → panic_already_borrowed (WASM corrupto)

Test Math Operations - Ejecutado como PRIMER test (sin otros tests antes):
  ✅ createVector([0, π/4, π/2, π]) → handle=1
  ❌ sin(handle=1) → panic_already_borrowed (FALLA IGUAL)
```

## Intentos de Solución (Fallidos)

### 1. Eliminar FinalizationRegistry
- **Acción:** Removido auto-cleanup de JavaScript GC
- **Resultado:** Sin cambio, error persiste

### 2. Eliminar cleanup automático de session.use()
- **Acción:** Removido `finally { cleanup() }` en session.use()
- **Resultado:** Sin cambio, error persiste

### 3. Mover `?` operator fuera del scope de borrow
- **Acción:** Evitar early return mientras RefCell está borrowed
- **Resultado:** Sin cambio, error persiste

### 4. Separar HANDLES.with() en dos llamadas
- **Acción:** Primera llamada para leer, segunda para crear handle
- **Resultado:** Sin cambio, error persiste

## Observaciones Clave

1. **Funciones simples SIEMPRE funcionan** - `createVector()` y `getVector()` nunca fallan
2. **Operaciones matemáticas SIEMPRE fallan** - `sin()`, `cos()` fallan incluso como PRIMER test ejecutado
3. **El patrón común:** Funciones que fallan usan `apply_unary_op` o `apply_binary_op` que hacen DOS llamadas a `HANDLES.with()`
4. **El test directo al WASM también falla** - No es un problema del SDK TypeScript, el error está en el WASM
5. **Con `--target web` funcionaba perfectamente** - El problema apareció con migración a TypeScript SDK
6. **No hay cleanup automático activo** - Eliminado completamente para evitar race conditions, error persiste

## Configuración de Compilación

### Cargo.toml

```toml
[package]
name = "achronyme-wasm"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
js-sys = "0.3"
serde = { version = "1.0", features = ["derive"] }
serde-wasm-bindgen = "0.6"
getrandom = { version = "0.2", features = ["js"] }

[profile.release]
opt-level = 3
lto = true
codegen-units = 1

[package.metadata.wasm-pack.profile.release]
wasm-opt = false
```

### Comando de Compilación

```bash
wasm-pack build crates/achronyme-wasm --target web --out-dir ../../dist
```

## Pregunta Central

**¿Por qué operaciones que usan `apply_unary_op` (dos llamadas a `HANDLES.with()`) SIEMPRE fallan con `panic_already_borrowed`, mientras que funciones simples que hacen UNA sola llamada a `HANDLES.with()` funcionan correctamente?**

**Diferencia clave:**

```rust
// ✅ FUNCIONA (una sola llamada a HANDLES.with):
pub fn createVector(data: Vec<f64>) -> Handle {
    let vector = Vector::new(data);
    HANDLES.with(|h| h.borrow_mut().create(Value::Vector(vector)))
}

// ❌ SIEMPRE FALLA (dos llamadas a HANDLES.with):
fn apply_unary_op(handle: Handle, f: F) -> Result<Handle, JsValue> {
    // Primera llamada
    let result = HANDLES.with(|h| { h.borrow().get(handle) })?;

    // Segunda llamada
    let new_handle = HANDLES.with(|h| { h.borrow_mut().create(...) });
}
```

**Contexto adicional:**
1. No hay cleanup automático que pueda interferir
2. Los dos `HANDLES.with()` están completamente separados
3. El `?` operator está FUERA de ambos `.with()`
4. Funciones con UNA sola llamada a `HANDLES.with()` funcionan perfectamente
5. El mismo código con DOS llamadas funcionaba con `--target web`

**¿Hay alguna diferencia en cómo `wasm-bindgen` o `thread_local!` manejan múltiples accesos a RefCell entre `--target web` y `--target bundler`?**
