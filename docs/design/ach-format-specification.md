# Formato .ach - Achronyme Environment Archive

**Version:** 1.0 Draft
**Date:** 2025-01-13
**Purpose:** Especificación del formato binario .ach para persistencia de entornos Achronyme

---

## 1. Visión General

### 1.1 Motivación

El formato `.ach` (Achronyme Archive) permite:

1. **Guardar sesiones de trabajo completas** - Variables, datos, resultados intermedios
2. **Compartir entornos reproducibles** - Otros usuarios pueden cargar tu workspace exacto
3. **Checkpoints durante cómputos largos** - Guardar progreso y retomar después
4. **Datos científicos con contexto** - No solo CSV, sino variables + metadata + código

**Diferencia clave**: `.soc` = código fuente, `.ach` = estado serializado del runtime

### 1.2 Casos de Uso

```javascript
// Sesión 1: Análisis exploratorio largo
let raw_data = read_csv("experiment_1M_rows.csv")  // Tarda 30 segundos
let processed = pipe(raw_data, clean, normalize, filter)  // Tarda 2 minutos
let models = train_multiple_models(processed)  // Tarda 10 minutos

// Guardar todo el workspace
save_env("analysis_session.ach")
// Archivo: 150 MB, contiene raw_data, processed, models

// --- Cerrar REPL, ir a almorzar ---

// Sesión 2: Continuar trabajo
restore_env("analysis_session.ach")  // Carga en 5 segundos
// raw_data, processed, models están disponibles instantáneamente!

let final_model = select_best(models)
let predictions = predict(final_model, new_data)
```

### 1.3 No-Objetivos

- ❌ No es un formato de intercambio de datos (usar CSV/JSON para eso)
- ❌ No es portable entre versiones de Achronyme (puede romperse)
- ❌ No es para almacenamiento a largo plazo (usar formatos estándares)
- ❌ No preserva funciones lambda definidas por usuario (solo builtins)

---

## 2. Formato Binario .ach

### 2.1 Estructura General

```
┌─────────────────────────────────────────────────────────┐
│                    HEADER (64 bytes)                    │
├─────────────────────────────────────────────────────────┤
│                  METADATA SECTION                       │
│  - Versión de Achronyme                                 │
│  - Timestamp de creación                                │
│  - Configuración del entorno                            │
│  - Tabla de contenidos                                  │
├─────────────────────────────────────────────────────────┤
│              BINDINGS SECTION (Valores)                 │
│  - Variable name → Value mappings                       │
│  - Cada Value serializado según su tipo                 │
├─────────────────────────────────────────────────────────┤
│           CONSTANTS SECTION (Opcional)                  │
│  - Constantes definidas (PI, E, i, etc.)                │
├─────────────────────────────────────────────────────────┤
│              FUNCTIONS SECTION (Opcional)               │
│  - Referencias a funciones builtin                      │
│  - AST de funciones user-defined (futuro)               │
├─────────────────────────────────────────────────────────┤
│                 CHECKSUM (32 bytes)                     │
│  - SHA-256 de todo el contenido anterior                │
└─────────────────────────────────────────────────────────┘
```

### 2.2 Header (64 bytes)

```rust
struct AchHeader {
    magic: [u8; 4],          // "ACH\0" - Magic bytes
    version_major: u16,      // Versión mayor del formato
    version_minor: u16,      // Versión menor del formato
    flags: u32,              // Feature flags
    created_timestamp: u64,  // Unix timestamp
    achronyme_version: [u8; 16],  // "v0.1.0\0\0\0..." - Versión del runtime
    compression: u8,         // 0=None, 1=Zstd, 2=Lz4
    reserved: [u8; 31],      // Para uso futuro
}

// Flags (bits):
// 0: Tiene metadata extendida
// 1: Incluye funciones user-defined
// 2: Incluye constantes
// 3: Comprimido
// 4-31: Reservado
```

**Magic Number**: `0x41 0x43 0x48 0x00` ("ACH\0")

### 2.3 Metadata Section

Formato: MessagePack o bincode (decidir después)

```rust
struct Metadata {
    // Información de creación
    created_by: String,           // "Achronyme REPL v0.1.0"
    created_at: DateTime,         // ISO 8601
    platform: String,             // "Windows x64", "Linux x64", etc.

    // Estadísticas del entorno
    num_bindings: u32,            // Número de variables
    num_constants: u32,           // Número de constantes
    num_functions: u32,           // Número de funciones

    // Tabla de contenidos
    bindings_offset: u64,         // Offset en bytes a sección bindings
    bindings_size: u64,           // Tamaño en bytes
    constants_offset: u64,
    constants_size: u64,
    functions_offset: u64,
    functions_size: u64,

    // Metadata opcional
    description: Option<String>,   // Usuario puede agregar descripción
    tags: Vec<String>,             // ["experiment_1", "temperature", ...]
    custom: HashMap<String, String>,  // Metadata custom
}
```

### 2.4 Bindings Section

**Formato**: Cada binding es un par `(String, Value)` serializado

```rust
struct Binding {
    name: String,           // Nombre de la variable
    value_type: u8,         // Tag del tipo de Value
    value_data: Vec<u8>,    // Serialización del Value
}

// Serialización de bindings:
// [num_bindings: u32]
// [binding_1_name_len: u32][binding_1_name: bytes]
// [binding_1_value_type: u8][binding_1_value_size: u32][binding_1_value_data: bytes]
// [binding_2_name_len: u32]...
```

### 2.5 Value Serialization

Cada tipo de `Value` se serializa de forma específica:

#### Value Type Tags
```rust
const TAG_NUMBER: u8 = 0x01;
const TAG_BOOLEAN: u8 = 0x02;
const TAG_STRING: u8 = 0x03;
const TAG_COMPLEX: u8 = 0x04;
const TAG_VECTOR: u8 = 0x05;
const TAG_TENSOR: u8 = 0x06;
const TAG_COMPLEX_TENSOR: u8 = 0x07;
const TAG_RECORD: u8 = 0x08;
const TAG_FUNCTION: u8 = 0x09;
const TAG_EDGE: u8 = 0x0A;
const TAG_MUTABLE_REF: u8 = 0x0B;
```

#### Number
```
[TAG_NUMBER: u8][value: f64]
```

#### Boolean
```
[TAG_BOOLEAN: u8][value: u8]  // 0 = false, 1 = true
```

#### String
```
[TAG_STRING: u8][length: u32][utf8_bytes: [u8; length]]
```

#### Complex
```
[TAG_COMPLEX: u8][real: f64][imag: f64]
```

#### Vector (heterogeneous)
```
[TAG_VECTOR: u8][length: u32]
[element_1: Value][element_2: Value]...
```

#### Tensor (numeric array)
```
[TAG_TENSOR: u8]
[num_dims: u32]
[dim_1: u32][dim_2: u32]...[dim_n: u32]
[num_elements: u32]
[element_1: f64][element_2: f64]...[element_n: f64]

// Ejemplo: Tensor 2x3
// [0x06][2][2][3][6][1.0][2.0][3.0][4.0][5.0][6.0]
```

#### ComplexTensor
```
[TAG_COMPLEX_TENSOR: u8]
[num_dims: u32][dim_1: u32]...
[num_elements: u32]
[elem_1_real: f64][elem_1_imag: f64]
[elem_2_real: f64][elem_2_imag: f64]...
```

#### Record
```
[TAG_RECORD: u8][num_fields: u32]
[field_1_name_len: u32][field_1_name: bytes][field_1_value: Value]
[field_2_name_len: u32][field_2_name: bytes][field_2_value: Value]...
```

#### Function
```
[TAG_FUNCTION: u8][function_type: u8]

// function_type:
// 0x01 = Builtin
//   [name_length: u32][name: bytes]
// 0x02 = User-defined (no soportado en v1.0)
//   → Error o marcador especial
```

#### Edge
```
[TAG_EDGE: u8]
[from: String][to: String]
[directed: u8]  // 0 = undirected, 1 = directed
[properties: Record]
```

#### MutableRef
```
[TAG_MUTABLE_REF: u8][inner_value: Value]
```

---

## 3. API de Persistencia

### 3.1 Funciones Principales

#### save_env()
```javascript
save_env(path: String, options?: Record) -> Boolean

// Options:
{
    // Qué incluir
    include_constants: true,      // Incluir PI, E, i, etc.
    include_functions: false,     // Incluir referencias a funciones
    include_builtins: false,      // Incluir sin, cos, map, etc.

    // Filtros
    exclude: ["temp_*", "debug_*"],  // Patrones de nombres a excluir
    include_only: null,              // Solo estos nombres (null = todos)

    // Compresión
    compress: true,               // Usar Zstd compression
    compression_level: 3,         // 1-22 (3 = balance)

    // Metadata
    description: "",              // Descripción del archivo
    tags: [],                     // Tags para categorizar

    // Seguridad
    allow_overwrite: false,       // Permitir sobrescribir archivo existente
}

// Retorno:
// true = guardado exitoso
// false o error = falló

// Ejemplos:
save_env("workspace.ach")
save_env("checkpoint.ach", {compress: false})
save_env("results.ach", {
    include_only: ["data", "results", "model"],
    description: "Final results from experiment A",
    tags: ["experiment_a", "production"]
})
```

#### restore_env()
```javascript
restore_env(path: String, options?: Record) -> Boolean

// Options:
{
    // Modo de restauración
    mode: "merge",               // "merge" | "replace" | "namespace"
    overwrite: false,            // Si merge, sobrescribir variables existentes?

    // Namespace (si mode = "namespace")
    namespace: null,             // Ej: "saved" → saved.data, saved.results

    // Filtros
    include_only: null,          // Solo restaurar estas variables
    exclude: [],                 // Excluir estas variables

    // Validación
    verify_checksum: true,       // Verificar integridad del archivo
    strict_version: false,       // Rechazar si versión de Achronyme no coincide
}

// Retorno:
// true = restaurado exitoso
// false o error = falló

// Ejemplos:
restore_env("workspace.ach")  // Merge con entorno actual

restore_env("old_session.ach", {
    mode: "namespace",
    namespace: "old"
})
// Ahora: old.data, old.results, etc.

restore_env("checkpoint.ach", {
    include_only: ["critical_data"],
    overwrite: true
})
```

#### env_info()
```javascript
env_info(path: String) -> Record

// Retorna metadata sin cargar todo el archivo
{
    version: "1.0",
    created_at: "2025-01-13T10:30:00Z",
    created_by: "Achronyme REPL v0.1.0",
    num_bindings: 42,
    num_constants: 5,
    compressed: true,
    file_size: 15728640,  // bytes
    description: "...",
    tags: ["experiment", "..."],
    bindings: ["data", "results", "model", ...]  // Lista de nombres
}

// Ejemplo:
let info = env_info("workspace.ach")
print("Variables:", info.bindings)
print("Created:", info.created_at)
```

### 3.2 Funciones de Snapshot (In-Memory)

```javascript
// Crear snapshot en memoria (no persiste a disco)
snapshot_env(options?: Record) -> Snapshot

// Restaurar desde snapshot
restore_snapshot(snapshot: Snapshot, options?: Record) -> Boolean

// Ejemplo: Checkpoint temporal durante exploración
let checkpoint = snapshot_env()

// Experimentos peligrosos...
mut data = transform_destructively(data)

// Oh no, no funcionó
restore_snapshot(checkpoint)
// data está de vuelta a su estado anterior
```

### 3.3 Funciones de Gestión de Entorno

```javascript
// Listar todas las variables actuales
env_bindings() -> Vector<String>

// Obtener valor por nombre
env_get(name: String) -> Value

// Establecer valor por nombre
env_set(name: String, value: Value) -> Boolean

// Remover binding
env_remove(name: String) -> Boolean

// Limpiar entorno (peligroso!)
env_clear(confirm?: Boolean) -> Boolean

// Ejemplos:
let all_vars = env_bindings()
print("Variables activas:", all_vars)

// Eliminar variables temporales
let temps = filter(name => starts_with(name, "temp_"), all_vars)
map(name => env_remove(name), temps)

// Guardar solo variables importantes
let important = ["data", "model", "results"]
save_env("clean.ach", {include_only: important})
```

---

## 4. Manejo de Tipos Especiales

### 4.1 Funciones

**Problema**: Lambdas user-defined no son fácilmente serializables

**Soluciones**:

#### Opción 1: Solo Builtins (v1.0)
```javascript
let f = sin  // Builtin function
save_env("test.ach")  // OK: guarda referencia a "sin"
restore_env("test.ach")  // f = sin

let g = x => x * 2  // User lambda
save_env("test2.ach")  // WARNING: Variable 'g' (function) not serialized
restore_env("test2.ach")  // g no existe
```

#### Opción 2: AST Serialization (v2.0, futuro)
```javascript
let g = x => x * 2
save_env("test.ach", {include_user_functions: true})
// Guarda: AST de la lambda
restore_env("test.ach")
// g funciona correctamente
```

#### Opción 3: Source Code Embedding
```javascript
// Guardar el código fuente de la lambda como string
// Requiere re-parsing al restaurar
```

**Decisión v1.0**:
- Builtins: ✅ Guardar por nombre
- User lambdas: ⚠️ Warning, no serializar
- Usuario debe re-definir funciones o usar módulos

### 4.2 MutableRef

```javascript
mut x = 42
let ref_x = &x  // MutableRef

// Serialización: Guardar el valor actual, no la referencia
save_env("test.ach")
// Se guarda x = 42

restore_env("test.ach")
// x = 42 (pero ya no hay ref_x, o es un nuevo ref)
```

**Decisión**: Serializar el valor dentro del MutableRef, recrear ref al restaurar

### 4.3 Closures

```javascript
let outer = 10
let closure = x => x + outer

save_env("test.ach")
// ¿Cómo guardar 'outer' capturado?
```

**Decisión v1.0**: No soportar closures user-defined. Error o warning.

### 4.4 Records con self

```javascript
let counter = {
    mut value: 0,
    increment: () => do { self.value = self.value + 1 }
}

save_env("test.ach")
```

**Decisión**:
- `value: 0` → Se serializa normalmente
- `increment` → Es una función, ver 4.1
- Al restaurar, `self` debe reconstruirse

---

## 5. Implementación

### 5.1 Nueva Crate: `achronyme-env`

```
achronyme-env/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── format.rs          # Definición del formato .ach
│   ├── serialize.rs       # Value → bytes
│   ├── deserialize.rs     # bytes → Value
│   ├── persist.rs         # save_env / restore_env
│   ├── snapshot.rs        # In-memory snapshots
│   ├── metadata.rs        # Metadata handling
│   ├── checksum.rs        # SHA-256 verification
│   └── errors.rs          # Error types
└── tests/
    ├── test_serialize.rs
    ├── test_roundtrip.rs
    ├── test_compression.rs
    └── test_versioning.rs
```

### 5.2 Dependencies

```toml
[dependencies]
# Serialización
serde = { workspace = true }
bincode = "1.3"              # Binario eficiente
# O alternativamente:
# rmp-serde = "1.1"          # MessagePack

# Compresión
zstd = "0.13"                # Zstandard compression
lz4 = "1.24"                 # LZ4 (más rápido, menos compresión)

# Hashing
sha2 = "0.10"                # SHA-256

# Utilities
chrono = "0.4"               # Timestamps
thiserror = { workspace = true }

# De otros crates de Achronyme
achronyme-types = { path = "../achronyme-types" }
```

### 5.3 Estructura de Código

#### format.rs
```rust
use serde::{Deserialize, Serialize};

pub const MAGIC: [u8; 4] = [0x41, 0x43, 0x48, 0x00]; // "ACH\0"
pub const FORMAT_VERSION_MAJOR: u16 = 1;
pub const FORMAT_VERSION_MINOR: u16 = 0;

#[repr(C)]
pub struct AchHeader {
    pub magic: [u8; 4],
    pub version_major: u16,
    pub version_minor: u16,
    pub flags: u32,
    pub created_timestamp: u64,
    pub achronyme_version: [u8; 16],
    pub compression: u8,
    pub reserved: [u8; 31],
}

#[derive(Serialize, Deserialize)]
pub struct Metadata {
    pub created_by: String,
    pub created_at: String,
    pub platform: String,
    pub num_bindings: u32,
    pub description: Option<String>,
    pub tags: Vec<String>,
    // ... etc
}
```

#### serialize.rs
```rust
use achronyme_types::value::Value;
use std::io::Write;

pub trait Serializable {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<(), SerError>;
}

impl Serializable for Value {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<(), SerError> {
        match self {
            Value::Number(n) => {
                writer.write_u8(TAG_NUMBER)?;
                writer.write_f64::<LittleEndian>(*n)?;
            }
            Value::String(s) => {
                writer.write_u8(TAG_STRING)?;
                writer.write_u32::<LittleEndian>(s.len() as u32)?;
                writer.write_all(s.as_bytes())?;
            }
            Value::Tensor(t) => {
                writer.write_u8(TAG_TENSOR)?;
                serialize_tensor(t, writer)?;
            }
            // ... otros tipos
        }
        Ok(())
    }
}
```

#### persist.rs
```rust
use achronyme_types::Environment;
use std::path::Path;

pub fn save_environment(
    env: &Environment,
    path: impl AsRef<Path>,
    options: SaveOptions,
) -> Result<(), PersistError> {
    // 1. Filtrar bindings según options
    let bindings = filter_bindings(env, &options);

    // 2. Crear metadata
    let metadata = create_metadata(&bindings, &options);

    // 3. Serializar a buffer
    let mut buffer = Vec::new();
    write_header(&mut buffer)?;
    write_metadata(&mut buffer, &metadata)?;
    write_bindings(&mut buffer, &bindings)?;

    // 4. Comprimir si se solicita
    let final_data = if options.compress {
        compress_zstd(&buffer, options.compression_level)?
    } else {
        buffer
    };

    // 5. Calcular checksum
    let checksum = calculate_sha256(&final_data);

    // 6. Escribir a archivo
    std::fs::write(path, &final_data)?;

    Ok(())
}

pub fn restore_environment(
    env: &mut Environment,
    path: impl AsRef<Path>,
    options: RestoreOptions,
) -> Result<(), PersistError> {
    // 1. Leer archivo
    let data = std::fs::read(path)?;

    // 2. Verificar checksum
    if options.verify_checksum {
        verify_checksum(&data)?;
    }

    // 3. Descomprimir si necesario
    let decompressed = detect_and_decompress(&data)?;

    // 4. Parsear header
    let header = parse_header(&decompressed)?;
    verify_version(&header, options.strict_version)?;

    // 5. Parsear metadata
    let metadata = parse_metadata(&decompressed)?;

    // 6. Deserializar bindings
    let bindings = deserialize_bindings(&decompressed)?;

    // 7. Aplicar a entorno según mode
    apply_bindings(env, bindings, &options)?;

    Ok(())
}
```

### 5.4 Integración con `achronyme-eval`

#### Registrar funciones I/O en function_modules/io.rs

```rust
// crates/achronyme-eval/src/function_modules/io.rs
use achronyme_env::{save_environment, restore_environment};

pub fn register_functions(registry: &mut FunctionRegistry) {
    // ... otras funciones I/O ...

    registry.register_special("save_env", save_env_wrapper, -1);
    registry.register_special("restore_env", restore_env_wrapper, -1);
    registry.register("env_info", env_info, 1);
    registry.register("env_bindings", env_bindings, 0);
    // ... etc
}

fn save_env_wrapper(args: &[Value], env: &Environment) -> Result<Value, String> {
    // Parse args: path, options
    let path = extract_string(&args[0])?;
    let options = if args.len() > 1 {
        parse_save_options(&args[1])?
    } else {
        SaveOptions::default()
    };

    save_environment(env, path, options)
        .map(|_| Value::Boolean(true))
        .map_err(|e| format!("save_env error: {}", e))
}
```

---

## 6. Casos de Uso Detallados

### 6.1 Data Science Workflow

```javascript
// Sesión de análisis largo

// 1. Cargar datos (lento)
let raw_data = read_csv("large_dataset.csv")  // 5 min
print("Loaded", len(raw_data), "rows")

// 2. Procesamiento (lento)
let cleaned = pipe(
    raw_data,
    remove_duplicates,
    handle_missing,
    normalize_columns
)  // 10 min

// 3. Feature engineering (lento)
let features = extract_features(cleaned)  // 15 min
let labels = extract_labels(cleaned)

// Checkpoint 1: Datos procesados
save_env("01_processed_data.ach", {
    include_only: ["cleaned", "features", "labels"],
    description: "Cleaned and processed data"
})

// 4. Entrenar modelos (muy lento)
let model_1 = train_lr(features, labels)  // 20 min
let model_2 = train_rf(features, labels)  // 30 min
let model_3 = train_nn(features, labels)  // 45 min

// Checkpoint 2: Modelos entrenados
save_env("02_trained_models.ach", {
    description: "All trained models",
    tags: ["models", "experiment_v1"]
})

// 5. Evaluación
let results = {
    lr: evaluate(model_1, test_data),
    rf: evaluate(model_2, test_data),
    nn: evaluate(model_3, test_data)
}

// Checkpoint final
save_env("03_final_results.ach")

// --- En otra sesión ---
restore_env("02_trained_models.ach")
// model_1, model_2, model_3 disponibles instantáneamente
// Sin re-entrenar!

let best = select_best([model_1, model_2, model_3])
export_model(best, "production_model.ach")
```

### 6.2 Colaboración en Equipo

```javascript
// Investigador A
let experiment_data = run_experiment()
let initial_analysis = analyze(experiment_data)

save_env("share_with_team.ach", {
    include_only: ["experiment_data", "initial_analysis"],
    description: "Experiment results from Lab A",
    tags: ["team", "temperature_study"]
})

// Enviar por email/cloud

// --- Investigador B ---
restore_env("share_with_team.ach")
// experiment_data e initial_analysis están disponibles

let extended_analysis = advanced_stats(experiment_data)
let visualizations = create_plots(experiment_data)

save_env("extended_analysis.ach", {
    description: "Extended analysis by Lab B"
})
```

### 6.3 Desarrollo Iterativo

```javascript
// Exploración interactiva en REPL

ach[1]> let data = load_data()
ach[2]> let model = train(data)
ach[3]> save_env("before_changes.ach")

// Experimentar con cambios
ach[4]> mut model = modify_risky(model)
ach[5]> test(model)  // Oh no, rompió todo!

// Restaurar estado anterior
ach[6]> restore_env("before_changes.ach", {overwrite: true})
ach[7]> test(model)  // Funciona de nuevo
```

### 6.4 Checkpoints Automáticos

```javascript
// Script de procesamiento largo con auto-save

let process_in_chunks = (dataset) => do {
    let results = []
    let chunks = split_into_chunks(dataset, 100)

    map((chunk, idx) => do {
        let processed = expensive_processing(chunk)
        results = [...results, processed]

        // Auto-checkpoint cada 10 chunks
        if idx % 10 == 0 {
            save_env("checkpoint_" + str(idx) + ".ach", {
                include_only: ["results"]
            })
            print("Checkpoint saved at chunk", idx)
        }
    }, chunks)

    results
}

// Si falla, restaurar último checkpoint
let last_checkpoint = find_latest_checkpoint()
restore_env(last_checkpoint)
```

---

## 7. Seguridad y Validación

### 7.1 Verificación de Integridad

```rust
// Checksum SHA-256 al final del archivo
fn calculate_checksum(data: &[u8]) -> [u8; 32] {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

fn verify_file(path: &Path) -> Result<bool, Error> {
    let data = std::fs::read(path)?;
    let (content, stored_checksum) = split_checksum(&data);
    let computed_checksum = calculate_checksum(content);
    Ok(stored_checksum == computed_checksum)
}
```

### 7.2 Versioning

```javascript
// Al restaurar archivo viejo
restore_env("old_v0.1.ach")
// Warning: File created with Achronyme v0.1.0, current version is v0.2.0
// Some features may not work correctly.
// Continue? (y/n)

// Con strict_version
restore_env("old.ach", {strict_version: true})
// Error: Version mismatch. File: v0.1.0, Current: v0.2.0
```

### 7.3 Límites de Tamaño

```rust
pub struct SaveOptions {
    // ...
    pub max_file_size: Option<usize>,  // Default: None (sin límite)
    pub warn_large_values: bool,       // Warn si variable > 100 MB
}

// Ejemplo de uso
save_env("huge.ach", {
    max_file_size: 1_000_000_000  // 1 GB max
})
```

---

## 8. Performance y Optimización

### 8.1 Compresión

**Benchmarks estimados** (1M element tensor):

| Compresión | Ratio | Tiempo Save | Tiempo Restore | Tamaño |
|------------|-------|-------------|----------------|--------|
| None | 1.0x | 50ms | 30ms | 8 MB |
| Zstd-3 | 2.5x | 150ms | 80ms | 3.2 MB |
| Zstd-9 | 3.5x | 500ms | 100ms | 2.3 MB |
| LZ4 | 1.8x | 80ms | 50ms | 4.4 MB |

**Recomendación**: Zstd nivel 3 por defecto (buen balance)

### 8.2 Streaming para Archivos Grandes

```rust
// Para archivos >1GB, usar streaming
pub fn save_environment_streaming(
    env: &Environment,
    path: impl AsRef<Path>,
    options: SaveOptions,
) -> Result<(), PersistError> {
    let file = BufWriter::new(File::create(path)?);
    let mut encoder = ZstdEncoder::new(file, options.compression_level)?;

    write_header(&mut encoder)?;

    // Escribir bindings uno por uno, no todo en memoria
    for (name, value) in env.bindings() {
        write_binding(&mut encoder, name, value)?;
    }

    encoder.finish()?;
    Ok(())
}
```

### 8.3 Lazy Loading (Futuro)

```javascript
// Cargar solo metadata
let info = env_info("huge.ach")
print(info.bindings)  // ["data", "model", "results"]

// Cargar solo variable específica
let data = env_load_var("huge.ach", "data")
// No carga model ni results
```

---

## 9. Testing Strategy

### 9.1 Unit Tests

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_serialize_number() {
        let val = Value::Number(42.0);
        let bytes = serialize_value(&val).unwrap();
        let restored = deserialize_value(&bytes).unwrap();
        assert_eq!(val, restored);
    }

    #[test]
    fn test_serialize_tensor() {
        let tensor = Tensor::from_vec(vec![1.0, 2.0, 3.0], vec![3]);
        let val = Value::Tensor(tensor);
        let roundtrip = roundtrip_value(&val).unwrap();
        assert_eq!(val, roundtrip);
    }

    #[test]
    fn test_compression() {
        let data = vec![0u8; 10000];
        let compressed = compress_zstd(&data, 3).unwrap();
        let decompressed = decompress_zstd(&compressed).unwrap();
        assert_eq!(data, decompressed);
    }
}
```

### 9.2 Integration Tests

```rust
#[test]
fn test_save_restore_environment() {
    let mut env = Environment::new();
    env.set("x", Value::Number(42.0));
    env.set("name", Value::String("test".into()));

    let temp = temp_file();
    save_environment(&env, &temp, SaveOptions::default()).unwrap();

    let mut restored = Environment::new();
    restore_environment(&mut restored, &temp, RestoreOptions::default()).unwrap();

    assert_eq!(restored.get("x"), Some(&Value::Number(42.0)));
    assert_eq!(restored.get("name"), Some(&Value::String("test".into())));
}
```

### 9.3 Benchmark Tests

```rust
#[bench]
fn bench_save_large_tensor(b: &mut Bencher) {
    let tensor = create_random_tensor(1_000_000);
    let env = env_with_binding("data", Value::Tensor(tensor));

    b.iter(|| {
        save_environment(&env, "bench.ach", SaveOptions::default()).unwrap();
    });
}
```

---

## 10. Documentación para Usuarios

### 10.1 Guía Rápida

```markdown
# Guardar y Restaurar Entornos

## Guardar tu trabajo
```javascript
save_env("my_work.ach")
```

## Restaurar después
```javascript
restore_env("my_work.ach")
```

## Ver qué hay en un archivo
```javascript
let info = env_info("my_work.ach")
print(info.bindings)  // Lista de variables
```

## Guardar solo variables importantes
```javascript
save_env("results.ach", {
    include_only: ["final_model", "test_results"]
})
```

## Restaurar en namespace
```javascript
restore_env("old.ach", {
    mode: "namespace",
    namespace: "old"
})
// Acceso: old.variable_name
```
```

---

## 11. Timeline de Implementación

### Semana 1: Fundamentos
- [ ] Crear crate `achronyme-env`
- [ ] Definir formato .ach (header, metadata)
- [ ] Implementar serialización básica (Number, Boolean, String)
- [ ] Tests unitarios

### Semana 2: Tipos Complejos
- [ ] Serialización de Tensor, Vector, ComplexTensor
- [ ] Serialización de Record
- [ ] Serialización de Complex
- [ ] Tests de roundtrip

### Semana 3: Persistencia
- [ ] Implementar `save_env()` básico
- [ ] Implementar `restore_env()` básico
- [ ] Manejo de opciones básicas
- [ ] Integration tests

### Semana 4: Features Avanzados
- [ ] Compresión (Zstd)
- [ ] Checksums
- [ ] Metadata extendida
- [ ] `env_info()`

### Semana 5: Integración
- [ ] Registrar funciones en `achronyme-eval`
- [ ] REPL integration
- [ ] CLI support
- [ ] Documentación

### Semana 6: Polish
- [ ] Optimización de performance
- [ ] Manejo de errores mejorado
- [ ] Ejemplos completos
- [ ] Benchmarks

**Total: 6 semanas**

---

## 12. Open Questions

### Q1: ¿Formato binario custom o usar formato existente?
- **Opción A**: Formato .ach custom (control total)
- **Opción B**: MessagePack (estándar, herramientas existentes)
- **Opción C**: Protocol Buffers (schema, evolutivo)

**Recomendación**: Formato custom para v1.0, usar MessagePack internamente para metadata

### Q2: ¿Cómo manejar funciones user-defined?
- **v1.0**: Solo builtins, warnings para user functions
- **v2.0**: Guardar AST, restaurar y re-evaluar
- **v3.0**: Bytecode compilation + serialización

### Q3: ¿Compresión por defecto?
- **Pro**: Archivos más pequeños (2-3x)
- **Con**: Slightly slower, más complejo
- **Recomendación**: Sí, Zstd nivel 3 por defecto

### Q4: ¿Versionado de formato?
- ¿Cómo manejar cambios incompatibles?
- ¿Mantener backward compatibility?
- **Recomendación**: Major version bump para cambios incompatibles

---

## Next Steps

1. **Review**: Este documento con el equipo
2. **Prototype**: Implementar serialización básica esta semana
3. **Validate**: Con casos de uso reales
4. **Iterate**: Refinar basado en feedback

---

**Preguntas para discutir**:
1. ¿Te parece bien el formato .ach propuesto?
2. ¿Los casos de uso cubren tus necesidades?
3. ¿Prefieres empezar con v1.0 simple o incluir más features?
4. ¿Qué prioridad tiene esto vs el sistema de módulos?
