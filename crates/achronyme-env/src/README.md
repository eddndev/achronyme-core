# achronyme-env Implementation Guide

Technical documentation for contributors and maintainers of the environment persistence system.

## Table of Contents
- [Architecture Overview](#-architecture-overview)
- [Module Breakdown](#-module-breakdown)
- [Implementation Details](#-implementation-details)
- [Design Patterns](#-design-patterns)
- [Binary Format Specification](#-binary-format-specification)
- [Error Handling Strategy](#-error-handling-strategy)
- [Testing Strategy](#-testing-strategy)
- [Extension Guide](#-extension-guide)
- [Performance Optimization](#-performance-optimization)
- [Memory Management](#-memory-management)

---

## Architecture Overview

### System Layers

The crate is organized into three conceptual layers:

```
┌─────────────────────────────────────────────────────────────────┐
│ Layer 3: Public API (lib.rs)                                    │
│ ┌───────────────────────────────────────────────────────────┐   │
│ │ save_environment(env, path, options) -> Result<()>        │   │
│ │ restore_environment(path, options) -> Result<Environment> │   │
│ │ get_metadata(path) -> Result<Metadata>                    │   │
│ └───────────────────────────────────────────────────────────┘   │
└───────────────────────┬─────────────────────────────────────────┘
                        │
┌───────────────────────▼─────────────────────────────────────────┐
│ Layer 2: Orchestration (persist.rs)                             │
│ ┌───────────────────────────────────────────────────────────┐   │
│ │ 1. Environment snapshot extraction                        │   │
│ │ 2. Filter application (include_only, exclude)             │   │
│ │ 3. Value → SerializedValue conversion                     │   │
│ │ 4. Metadata construction                                  │   │
│ │ 5. MessagePack encoding                                   │   │
│ │ 6. Optional compression                                   │   │
│ │ 7. Checksum calculation                                   │   │
│ │ 8. Header + Body + Checksum → File                        │   │
│ └───────────────────────────────────────────────────────────┘   │
└───────────────────────┬─────────────────────────────────────────┘
                        │
┌───────────────────────▼─────────────────────────────────────────┐
│ Layer 1: Primitives                                              │
│ ┌──────────────┐ ┌──────────────┐ ┌──────────────┐             │
│ │ serialize.rs │ │  format.rs   │ │ checksum.rs  │             │
│ │              │ │              │ │              │             │
│ │ - from_value │ │ - AchHeader  │ │ - SHA-256    │             │
│ │ - to_value   │ │ - read/write │ │ - verify     │             │
│ └──────────────┘ └──────────────┘ └──────────────┘             │
│                   ┌──────────────┐ ┌──────────────┐             │
│                   │ metadata.rs  │ │  errors.rs   │             │
│                   │              │ │              │             │
│                   │ - builder    │ │ - EnvError   │             │
│                   │ - serde      │ │ - Result     │             │
│                   └──────────────┘ └──────────────┘             │
└─────────────────────────────────────────────────────────────────┘
```

### Data Flow

#### Save Path

```
Environment
    │
    │ .snapshot()
    ▼
HashMap<String, Value>
    │
    │ Filter (include_only, exclude)
    ▼
HashMap<String, Value> (filtered)
    │
    │ SerializedValue::from_value() for each
    ▼
HashMap<String, SerializedValue>
    │
    │ Construct AchBody { metadata, bindings }
    ▼
AchBody
    │
    │ rmp_serde::to_vec()
    ▼
Vec<u8> (MessagePack bytes)
    │
    │ Optional: zstd::encode_all()
    ▼
Vec<u8> (possibly compressed)
    │
    │ calculate_checksum()
    ▼
[u8; 32] (SHA-256)
    │
    │ Write: Header → Body → Checksum
    ▼
.ach File
```

#### Load Path

```
.ach File
    │
    │ Read header (64 bytes)
    ▼
AchHeader
    │
    │ Verify magic, version
    │ Read rest of file
    ▼
Vec<u8> (Body + Checksum)
    │
    │ Split: data (all but last 32), checksum (last 32)
    │ verify_checksum()
    ▼
Vec<u8> (verified body)
    │
    │ Optional: zstd::decode_all()
    ▼
Vec<u8> (decompressed MessagePack)
    │
    │ rmp_serde::from_slice()
    ▼
AchBody { metadata, bindings }
    │
    │ Filter (include_only, exclude)
    │ For each binding:
    │   SerializedValue::to_value()
    ▼
HashMap<String, Value>
    │
    │ Environment::new() + env.define() for each
    ▼
Environment
```

---

## Module Breakdown

### lib.rs (43 lines)

**Purpose**: Public API surface and module declarations.

**Exports**:
```rust
pub use errors::{EnvError, Result};
pub use format::{AchHeader, MAGIC, FORMAT_VERSION_MAJOR, FORMAT_VERSION_MINOR};
pub use metadata::Metadata;
pub use persist::{save_environment, restore_environment, get_metadata,
                  SaveOptions, RestoreOptions, RestoreMode};
pub use serialize::{serialize_value, deserialize_value};
```

**Design notes**:
- Minimal re-exports to keep API surface small
- No `pub use checksum` (internal implementation detail)
- Follows Rust API guidelines for crate organization

---

### errors.rs (67 lines)

**Purpose**: Centralized error handling.

**Key types**:
```rust
pub enum EnvError {
    Io(std::io::Error),
    Serialization(String),
    Deserialization(String),
    InvalidFormat(String),
    VersionMismatch { file_version: String, current_version: String },
    ChecksumMismatch,
    Compression(String),
    Decompression(String),
    BindingNotFound(String),
    InvalidOption(String),
    Custom(String),
}
```

**Design patterns**:
- Uses `thiserror` for automatic `Display` and `Error` implementations
- `From<std::io::Error>` for `?` operator ergonomics
- Custom `From` implementations for `rmp_serde` errors
- Structured errors (e.g., `VersionMismatch`) over string-only errors

**Error conversion chain**:
```
std::io::Error → EnvError::Io (automatic via #[from])
rmp_serde::encode::Error → EnvError::Serialization (custom From impl)
rmp_serde::decode::Error → EnvError::Deserialization (custom From impl)
```

---

### format.rs (268 lines)

**Purpose**: Binary format definition and header serialization.

#### Header Layout (64 bytes)

```
Offset  Size  Field              Type      Notes
──────────────────────────────────────────────────────────────────
0       4     magic              [u8; 4]   "ACH\0" (0x41 0x43 0x48 0x00)
4       2     version_major      u16       Little-endian
6       2     version_minor      u16       Little-endian
8       4     flags              u32       Bit flags (see below)
12      8     created_timestamp  u64       Unix epoch seconds
20      16    achronyme_version  [u8; 16]  Null-terminated string
36      1     compression        u8        0=None, 1=Zstd
37      27    reserved           [u8; 27]  Padding to 64 bytes
──────────────────────────────────────────────────────────────────
Total: 64 bytes
```

#### Flag Bits

```
Bit  Mask     Flag            Meaning
─────────────────────────────────────────────────────────────
0    0x00001  COMPRESSED      Body is Zstd compressed
1    0x00002  HAS_METADATA    Extended metadata (reserved)
2    0x00004  HAS_FUNCTIONS   User functions (reserved)
3-31          (reserved)      Future use
```

#### Key Methods

```rust
impl AchHeader {
    pub fn new() -> Self;  // Creates default header
    pub fn write<W: Write>(&self, writer: &mut W) -> Result<()>;
    pub fn read<R: Read>(reader: &mut R) -> Result<Self>;
    pub fn is_compressed(&self) -> bool;
    pub fn set_compressed(&mut self, compressed: bool);
    pub fn verify_version(&self, strict: bool) -> Result<()>;
}
```

**Design decisions**:
1. **Fixed 64-byte size**: Easy to skip, aligns with disk sectors
2. **Little-endian**: Most common architecture (x86, ARM)
3. **Magic bytes**: Enables `file` command detection
4. **Reserved space**: Allows adding fields without breaking compatibility
5. **Null-terminated version**: Human-readable in hex dumps

**Version compatibility logic**:
```rust
// Strict mode: Exact major version match required
if strict && self.version_major != FORMAT_VERSION_MAJOR {
    return Err(VersionMismatch);
}

// Non-strict: Allow older major versions (backward compat)
// Reject newer major versions (forward compat impossible)
if self.version_major > FORMAT_VERSION_MAJOR {
    return Err(VersionMismatch);
}
```

---

### metadata.rs (125 lines)

**Purpose**: File metadata tracking.

**Structure**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub created_by: String,         // "Achronyme v0.1.0"
    pub created_at: String,         // ISO 8601 timestamp
    pub platform: String,           // "windows x86_64"
    pub num_bindings: u32,          // Count of variables
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub custom: HashMap<String, String>,
    pub binding_names: Vec<String>, // Sorted list
}
```

**Builder pattern**:
```rust
let meta = Metadata::new()
    .with_description("My experiment")
    .with_tags(vec!["ml".into(), "training".into()])
    .with_custom("experiment_id", "exp_001")
    .with_bindings(vec!["model".into(), "data".into()]);
```

**Design notes**:
- `#[serde(default)]` on optional fields for backward compatibility
- ISO 8601 timestamps for cross-platform consistency
- `binding_names` sorted for deterministic output
- `custom` HashMap allows user-defined metadata without format changes

**Default values**:
```rust
Metadata::new() creates:
  created_by:     "Achronyme v{CARGO_PKG_VERSION}"
  created_at:     Utc::now().to_rfc3339()
  platform:       "{std::env::consts::OS} {std::env::consts::ARCH}"
  num_bindings:   0
  description:    None
  tags:           []
  custom:         {}
  binding_names:  []
```

---

### serialize.rs (289 lines)

**Purpose**: Value ↔ SerializedValue conversion.

#### SerializedValue Enum

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SerializedValue {
    Number(f64),
    Boolean(bool),
    String(String),
    Complex(f64, f64),                              // (re, im)
    Vector(Vec<SerializedValue>),                   // Recursive
    Tensor(Vec<usize>, Vec<f64>),                   // (shape, data)
    ComplexTensor(Vec<usize>, Vec<(f64, f64)>),     // (shape, complex_data)
    Record(HashMap<String, SerializedValue>),       // Recursive
    Edge(String, String, bool, HashMap<String, SerializedValue>),  // Graph edges
    BuiltinFunction(String),                        // Function name only
    Unsupported(String),                            // Type name for error
}
```

**Why intermediate representation?**

1. **Non-serializable types**: Functions, mutable refs cannot be directly serialized
2. **Type mapping**: Achronyme `Value` → Portable representation
3. **Version evolution**: Can add new variants without breaking old files
4. **Validation**: Can reject unsupported types before serialization

#### Conversion Logic

**Value → SerializedValue** (`.from_value()`):
```rust
match value {
    Value::Number(n) => SerializedValue::Number(*n),  // Direct
    Value::Complex(c) => SerializedValue::Complex(c.re, c.im),  // Decompose

    Value::Vector(vec) => SerializedValue::Vector(
        vec.iter().map(SerializedValue::from_value).collect()  // Recursive
    ),

    Value::Tensor(t) => SerializedValue::Tensor(
        t.shape().to_vec(),   // Extract shape
        t.data().to_vec()     // Flatten data
    ),

    Value::Function(func) => {
        if let Some(name) = func.builtin_name() {
            SerializedValue::BuiltinFunction(name.to_string())
        } else {
            SerializedValue::Unsupported("user-defined function".to_string())
        }
    },

    Value::MutableRef(rc) => {
        match rc.try_borrow() {
            Ok(inner) => SerializedValue::from_value(&inner),  // Dereference
            Err(_) => SerializedValue::Unsupported("borrowed mutable reference".to_string())
        }
    },

    // Runtime-only constructs
    Value::TailCall(_) => SerializedValue::Unsupported("tail call".to_string()),
    Value::EarlyReturn(_) => SerializedValue::Unsupported("early return".to_string()),
}
```

**SerializedValue → Value** (`.to_value()`):
```rust
match self {
    SerializedValue::Number(n) => Ok(Value::Number(*n)),

    SerializedValue::Complex(re, im) =>
        Ok(Value::Complex(Complex::new(*re, *im))),

    SerializedValue::Tensor(shape, data) => {
        let tensor = RealTensor::new(data.clone(), shape.clone())
            .map_err(|e| EnvError::Deserialization(format!("Invalid tensor: {}", e)))?;
        Ok(Value::Tensor(tensor))
    },

    SerializedValue::BuiltinFunction(_name) => {
        // Future: Look up in function registry
        Err(EnvError::Deserialization("Function restoration not yet supported".to_string()))
    },

    SerializedValue::Unsupported(type_name) => {
        Err(EnvError::Deserialization(format!("Cannot deserialize: {}", type_name)))
    },
}
```

**Design decisions**:

1. **Tuple variants over struct variants**: Better MessagePack compatibility
   ```rust
   // Preferred
   Complex(f64, f64)          // Encoded as [re, im]

   // Avoided
   Complex { re: f64, im: f64 }  // Encoded as {re: ..., im: ...} - more bytes
   ```

2. **Flattened tensors**: Store as `(shape, flat_data)` instead of nested arrays
   - More compact
   - Easier to reconstruct
   - Compatible with row-major layout

3. **Recursive serialization**: Vectors and Records recursively convert nested values
   - Enables arbitrary nesting depth
   - Type preservation through the tree

4. **Graceful degradation**: Unsupported types → `SerializedValue::Unsupported`
   - Warning printed during save
   - Error only on restore (if not filtered out)

---

### persist.rs (491 lines)

**Purpose**: Main orchestration logic for save/restore operations.

#### SaveOptions

```rust
pub struct SaveOptions {
    pub compress: bool,              // Enable Zstd compression
    pub compression_level: i32,      // 1-22 (default: 3)
    pub description: Option<String>, // File description
    pub tags: Vec<String>,           // Categorization tags
    pub include_only: Option<Vec<String>>,  // Whitelist
    pub exclude: Vec<String>,        // Blacklist (supports patterns)
    pub allow_overwrite: bool,       // Safety check
}
```

**Default**:
```rust
SaveOptions {
    compress: true,              // Compression on by default
    compression_level: 3,        // Fast compression
    description: None,
    tags: vec![],
    include_only: None,          // Save all variables
    exclude: vec![],             // Exclude nothing
    allow_overwrite: false,      // Prevent accidental overwrites
}
```

#### RestoreOptions

```rust
pub struct RestoreOptions {
    pub mode: RestoreMode,             // Merge | Replace | Namespace
    pub overwrite: bool,               // For Merge mode
    pub namespace: Option<String>,     // For Namespace mode
    pub include_only: Option<Vec<String>>,
    pub exclude: Vec<String>,
    pub verify_checksum: bool,         // Integrity check
    pub strict_version: bool,          // Exact version match
}
```

**RestoreMode**:
```rust
pub enum RestoreMode {
    Merge,      // Add to current environment (default)
    Replace,    // Clear and restore
    Namespace,  // Restore into nested namespace (reserved)
}
```

#### Pattern Matching

```rust
fn matches_pattern(name: &str, pattern: &str) -> bool {
    if pattern.ends_with('*') {
        let prefix = &pattern[..pattern.len() - 1];
        name.starts_with(prefix)
    } else {
        name == pattern
    }
}

// Usage
exclude: vec!["temp_*".into(), "cache_*".into()]
// Matches: "temp_x", "temp_result", "cache_data", ...
// Doesn't match: "temperature", "cached", ...
```

#### Save Implementation

```rust
pub fn save_environment(
    env: &Environment,
    path: impl AsRef<Path>,
    options: SaveOptions,
) -> Result<()> {
    // 1. Check if file exists
    if path.exists() && !options.allow_overwrite {
        return Err(EnvError::Custom("File already exists".to_string()));
    }

    // 2. Get environment snapshot
    let snapshot = env.snapshot();  // HashMap<String, Value>

    // 3. Filter bindings
    let mut bindings_to_save = HashMap::new();
    for (name, value) in snapshot.iter() {
        // Apply include_only filter
        if let Some(ref include_only) = options.include_only {
            if !include_only.contains(&name.to_string()) {
                continue;
            }
        }

        // Apply exclude filter
        if options.exclude.iter().any(|p| matches_pattern(name, p)) {
            continue;
        }

        // Convert to SerializedValue
        let serialized = SerializedValue::from_value(value);

        // Skip unsupported values
        if matches!(serialized, SerializedValue::Unsupported { .. }) {
            eprintln!("Warning: Variable '{}' is not serializable", name);
            continue;
        }

        bindings_to_save.insert(name.clone(), serialized);
    }

    // 4. Create metadata
    let metadata = Metadata::new()
        .with_bindings(binding_names)
        .with_description(options.description)
        .with_tags(options.tags);

    // 5. Create body
    let body = AchBody { metadata, bindings: bindings_to_save };

    // 6. Serialize to MessagePack
    let body_bytes = rmp_serde::to_vec(&body)?;

    // 7. Compress if requested
    let (final_bytes, compression) = if options.compress {
        let compressed = zstd::encode_all(&body_bytes[..], options.compression_level)
            .map_err(|e| EnvError::Compression(e.to_string()))?;
        (compressed, CompressionType::Zstd)
    } else {
        (body_bytes, CompressionType::None)
    };

    // 8. Calculate checksum
    let checksum = calculate_checksum(&final_bytes);

    // 9. Create header
    let mut header = AchHeader::new();
    header.compression = compression;
    header.set_compressed(options.compress);

    // 10. Write to file
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    header.write(&mut writer)?;
    writer.write_all(&final_bytes)?;
    writer.write_all(&checksum)?;
    writer.flush()?;

    Ok(())
}
```

#### Restore Implementation

```rust
pub fn restore_environment(
    path: impl AsRef<Path>,
    options: RestoreOptions,
) -> Result<Environment> {
    // 1. Read header
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let header = AchHeader::read(&mut reader)?;

    // 2. Verify version
    header.verify_version(options.strict_version)?;

    // 3. Read body + checksum
    let mut body_bytes = Vec::new();
    reader.read_to_end(&mut body_bytes)?;

    // 4. Split data and checksum
    let data_end = body_bytes.len() - CHECKSUM_SIZE;
    let data_bytes = &body_bytes[..data_end];
    let checksum_bytes = &body_bytes[data_end..];

    // 5. Verify checksum
    if options.verify_checksum {
        if !verify_checksum(data_bytes, checksum_bytes) {
            return Err(EnvError::ChecksumMismatch);
        }
    }

    // 6. Decompress if needed
    let decompressed = if header.is_compressed() {
        zstd::decode_all(data_bytes)
            .map_err(|e| EnvError::Decompression(e.to_string()))?
    } else {
        data_bytes.to_vec()
    };

    // 7. Deserialize body
    let body: AchBody = rmp_serde::from_slice(&decompressed)?;

    // 8. Create environment
    let mut env = Environment::new();

    // 9. Restore bindings (with filters)
    for (name, serialized_value) in body.bindings {
        // Apply filters (same logic as save)
        if should_skip(&name, &options) { continue; }

        // Deserialize value
        match serialized_value.to_value() {
            Ok(value) => {
                if let Err(e) = env.define(name.clone(), value) {
                    eprintln!("Warning: Failed to define '{}': {}", name, e);
                }
            }
            Err(e) => {
                eprintln!("Warning: Failed to restore '{}': {}", name, e);
            }
        }
    }

    Ok(env)
}
```

#### Get Metadata (Fast Inspection)

```rust
pub fn get_metadata(path: impl AsRef<Path>) -> Result<Metadata> {
    // Read and decompress body (same as restore)
    // But only deserialize the metadata field
    // MessagePack allows partial deserialization

    let body: AchBody = rmp_serde::from_slice(&decompressed)?;
    Ok(body.metadata)  // Don't process bindings
}
```

**Note**: Currently deserializes full body because MessagePack doesn't support partial deserialization of arbitrary structs. Future optimization could use custom deserializer.

---

### checksum.rs (52 lines)

**Purpose**: SHA-256 checksum calculation and verification.

**Implementation**:
```rust
use sha2::{Sha256, Digest};

pub const CHECKSUM_SIZE: usize = 32;

pub fn calculate_checksum(data: &[u8]) -> [u8; CHECKSUM_SIZE] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

pub fn verify_checksum(data: &[u8], expected: &[u8; CHECKSUM_SIZE]) -> bool {
    let computed = calculate_checksum(data);
    computed == *expected
}
```

**Design notes**:
- Uses `sha2` crate (pure Rust implementation)
- Constant-time comparison for security (via array equality)
- Returns `[u8; 32]` array, not `Vec<u8>` (fixed size)

**Why SHA-256?**
- Widely supported
- Fast enough for our use case
- Strong collision resistance
- 256 bits = 32 bytes (reasonable size)

Alternatives considered:
- **SHA-1**: Deprecated (collision vulnerabilities)
- **SHA-512**: Overkill (64 bytes checksum)
- **CRC32**: Too weak (32-bit collision space)
- **BLAKE3**: Better performance, but less standard

---

## Design Patterns

### 1. Builder Pattern (Metadata, Options)

```rust
// Instead of large constructor
let options = SaveOptions::new(true, 3, Some("desc"), vec![], None, vec![], false);

// Use builder pattern
let options = SaveOptions {
    compress: true,
    description: Some("desc".into()),
    ..Default::default()
};

// Or for Metadata
let meta = Metadata::new()
    .with_description("desc")
    .with_tags(vec!["tag1".into()]);
```

**Benefits**:
- Readable at call site
- Easy to add new fields
- Backward compatible (new fields get defaults)

### 2. Newtype Pattern (Result)

```rust
pub type Result<T> = std::result::Result<T, EnvError>;
```

**Benefits**:
- Shorter type signatures
- Consistent error type across crate
- Easy to change error type in future

### 3. Try-From Pattern (CompressionType)

```rust
impl CompressionType {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(CompressionType::None),
            1 => Some(CompressionType::Zstd),
            _ => None,
        }
    }
}
```

**Benefits**:
- Explicit error handling (returns Option)
- Cannot construct invalid values
- Forward compatible (unknown values rejected)

### 4. Visitor Pattern (Serialization)

```rust
// Conceptual - serde does this internally
impl Serialize for SerializedValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            SerializedValue::Number(n) => serializer.serialize_f64(*n),
            SerializedValue::String(s) => serializer.serialize_str(s),
            // ...
        }
    }
}
```

**Benefits** (provided by serde):
- Format-agnostic (JSON, MessagePack, etc.)
- Zero-copy deserialization
- Automatic derive for simple types

---

## Binary Format Specification

### Complete .ach File Structure

```
┌────────────────────────────────────────────────────────────────┐
│ HEADER (64 bytes)                                              │
├────────────────────────────────────────────────────────────────┤
│ Offset │ Size │ Field              │ Type   │ Value           │
├────────┼──────┼────────────────────┼────────┼─────────────────┤
│ 0      │ 4    │ magic              │ u8[4]  │ [0x41,0x43,0x48,│
│        │      │                    │        │  0x00]          │
│ 4      │ 2    │ version_major      │ u16le  │ 1               │
│ 6      │ 2    │ version_minor      │ u16le  │ 0               │
│ 8      │ 4    │ flags              │ u32le  │ 0x00000001 if   │
│        │      │                    │        │ compressed      │
│ 12     │ 8    │ created_timestamp  │ u64le  │ Unix epoch      │
│ 20     │ 16   │ achronyme_version  │ u8[16] │ "0.1.0\0..."    │
│ 36     │ 1    │ compression        │ u8     │ 0=None, 1=Zstd  │
│ 37     │ 27   │ reserved           │ u8[27] │ [0x00, ...]     │
└────────┴──────┴────────────────────┴────────┴─────────────────┘

┌────────────────────────────────────────────────────────────────┐
│ BODY (variable size)                                           │
├────────────────────────────────────────────────────────────────┤
│ MessagePack-encoded AchBody:                                   │
│                                                                │
│ AchBody {                                                      │
│   metadata: Metadata {                                         │
│     created_by: "Achronyme v0.1.0",                           │
│     created_at: "2025-01-15T10:30:00Z",                       │
│     platform: "windows x86_64",                               │
│     num_bindings: 2,                                          │
│     description: Some("Test"),                                │
│     tags: ["experiment"],                                     │
│     custom: {},                                               │
│     binding_names: ["x", "y"]                                 │
│   },                                                          │
│   bindings: {                                                 │
│     "x": SerializedValue::Number(42.0),                       │
│     "y": SerializedValue::String("hello")                     │
│   }                                                           │
│ }                                                             │
│                                                                │
│ (May be Zstd compressed)                                       │
└────────────────────────────────────────────────────────────────┘

┌────────────────────────────────────────────────────────────────┐
│ CHECKSUM (32 bytes)                                            │
├────────────────────────────────────────────────────────────────┤
│ SHA-256 hash of BODY bytes (before checksum)                   │
│ [0xab, 0xcd, 0xef, ..., 0x12] (32 bytes total)                │
└────────────────────────────────────────────────────────────────┘
```

### MessagePack Encoding Examples

**Number**:
```
SerializedValue::Number(42.0)
  → MessagePack: [0xcb, 0x40, 0x45, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
  (0xcb = float64 prefix, followed by IEEE 754 double)
```

**String**:
```
SerializedValue::String("hello")
  → MessagePack: [0xa5, 0x68, 0x65, 0x6c, 0x6c, 0x6f]
  (0xa5 = fixstr of length 5, followed by UTF-8 bytes)
```

**Complex**:
```
SerializedValue::Complex(3.0, 4.0)
  → MessagePack: [0x92, 0xcb, <re bytes>, 0xcb, <im bytes>]
  (0x92 = fixarray of length 2, two float64s)
```

**Tensor**:
```
SerializedValue::Tensor(vec![2, 2], vec![1.0, 2.0, 3.0, 4.0])
  → MessagePack: [
      0x92,                   // fixarray of length 2
      0x92, 0x02, 0x02,      // shape: [2, 2]
      0x94, 0xcb, ..., ...   // data: [1.0, 2.0, 3.0, 4.0]
    ]
```

---

## Error Handling Strategy

### Error Propagation

```rust
// Use ? operator for automatic error conversion
pub fn save_environment(...) -> Result<()> {
    let file = File::create(path)?;  // io::Error → EnvError::Io
    let bytes = rmp_serde::to_vec(&body)?;  // encode::Error → EnvError::Serialization
    let compressed = zstd::encode_all(...).map_err(|e| EnvError::Compression(...))?;
    Ok(())
}
```

### Error Context

```rust
// Add context to generic errors
let tensor = RealTensor::new(data, shape)
    .map_err(|e| EnvError::Deserialization(
        format!("Invalid tensor: {}", e)
    ))?;
```

### Graceful Degradation

```rust
// Print warnings, don't fail entire operation
for (name, value) in snapshot {
    let serialized = SerializedValue::from_value(value);
    if matches!(serialized, SerializedValue::Unsupported { .. }) {
        eprintln!("Warning: Variable '{}' is not serializable", name);
        continue;  // Skip this binding, save others
    }
    bindings_to_save.insert(name, serialized);
}
```

### Recovery Options

```rust
// Checksum verification can be disabled
let options = RestoreOptions {
    verify_checksum: false,  // For corrupted-but-parseable files
    ..Default::default()
};
```

---

## Testing Strategy

### Unit Tests

Each module has tests for its core functionality:

**format.rs**:
```rust
#[test]
fn test_header_size() {
    // Verify fixed 64-byte size
}

#[test]
fn test_header_roundtrip() {
    // Write → Read → Compare
}

#[test]
fn test_invalid_magic() {
    // Verify magic byte checking
}
```

**serialize.rs**:
```rust
#[test]
fn test_serialize_number() {
    // Value → bytes → Value
}

#[test]
fn test_serialize_nested() {
    // Record containing Record
}
```

### Integration Tests

**persist.rs**:
```rust
#[test]
fn test_save_and_restore_basic() {
    // End-to-end: Create env → Save → Restore → Verify
}

#[test]
fn test_checksum_verification() {
    // Save → Corrupt file → Restore (should fail)
}
```

### Property-Based Testing (Future)

```rust
// Using proptest or quickcheck
#[test]
fn prop_roundtrip_preserves_values(value: Value) {
    let serialized = SerializedValue::from_value(&value);
    if let Ok(restored) = serialized.to_value() {
        assert_eq!(value, restored);
    }
}
```

### Fuzzing (Future)

```rust
// Using cargo-fuzz
#[test]
fn fuzz_header_parsing(data: &[u8]) {
    let _ = AchHeader::read(&mut Cursor::new(data));
    // Should not panic
}
```

---

## Extension Guide

### Adding New Serializable Type

**Step 1**: Add variant to `SerializedValue` (serialize.rs):
```rust
pub enum SerializedValue {
    // ... existing variants ...
    MyNewType(String, Vec<f64>),  // Example: labeled vector
}
```

**Step 2**: Implement `from_value()`:
```rust
Value::MyNewType { label, data } =>
    SerializedValue::MyNewType(label.clone(), data.clone()),
```

**Step 3**: Implement `to_value()`:
```rust
SerializedValue::MyNewType(label, data) => {
    Ok(Value::MyNewType {
        label: label.clone(),
        data: data.clone()
    })
}
```

**Step 4**: Add tests:
```rust
#[test]
fn test_serialize_my_new_type() {
    let value = Value::MyNewType { ... };
    let bytes = serialize_value(&value).unwrap();
    let restored = deserialize_value(&bytes).unwrap();
    assert_eq!(value, restored);
}
```

### Adding New Header Field

**Step 1**: Use reserved space in header (format.rs):
```rust
// Before (27 bytes reserved):
pub reserved: [u8; 27],

// After (26 bytes reserved, 1 for new field):
pub my_new_field: u8,
pub reserved: [u8; 26],
```

**Step 2**: Update read/write methods:
```rust
pub fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
    // ... existing writes ...
    writer.write_u8(self.my_new_field)?;
    writer.write_all(&self.reserved)?;
    Ok(())
}
```

**Step 3**: Bump minor version:
```rust
pub const FORMAT_VERSION_MINOR: u16 = 1;  // Was 0
```

**Step 4**: Handle old files:
```rust
// In read():
let my_new_field = if version_minor >= 1 {
    reader.read_u8()?
} else {
    0  // Default for old files
};
```

### Adding New Flag Bit

```rust
// In format.rs
pub mod flags {
    // ... existing flags ...
    pub const MY_NEW_FLAG: u32 = 1 << 3;  // Next available bit
}

// Usage
impl AchHeader {
    pub fn set_my_flag(&mut self, enabled: bool) {
        if enabled {
            self.flags |= flags::MY_NEW_FLAG;
        } else {
            self.flags &= !flags::MY_NEW_FLAG;
        }
    }

    pub fn has_my_flag(&self) -> bool {
        self.flags & flags::MY_NEW_FLAG != 0
    }
}
```

---

## Performance Optimization

### Memory Pooling (Future)

Current: Allocates new Vec for each serialization
```rust
let body_bytes = rmp_serde::to_vec(&body)?;
```

Future: Reuse buffers
```rust
struct BufferPool {
    buffers: Vec<Vec<u8>>,
}

impl BufferPool {
    fn get(&mut self) -> Vec<u8> {
        self.buffers.pop().unwrap_or_else(Vec::new)
    }

    fn release(&mut self, mut buf: Vec<u8>) {
        buf.clear();
        self.buffers.push(buf);
    }
}
```

### Streaming Serialization (Future)

Current: Serialize entire body to memory, then write
```rust
let body_bytes = rmp_serde::to_vec(&body)?;
writer.write_all(&body_bytes)?;
```

Future: Stream directly to file
```rust
rmp_serde::encode::write(&mut writer, &body)?;
```

**Benefits**:
- Lower peak memory usage
- Faster for large environments

### Incremental Checksums (Future)

Current: Calculate checksum after full serialization
```rust
let checksum = calculate_checksum(&body_bytes);
```

Future: Calculate during serialization
```rust
struct ChecksumWriter<W> {
    writer: W,
    hasher: Sha256,
}

impl<W: Write> Write for ChecksumWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.hasher.update(buf);
        self.writer.write(buf)
    }
}
```

### Parallel Compression (Current)

Zstd supports multi-threaded compression:
```rust
let compressed = zstd::stream::encode_all(
    &body_bytes[..],
    options.compression_level
)?;
```

Already uses multiple threads internally for levels > 10.

---

## Memory Management

### Allocation Hot Spots

1. **Environment snapshot**: `env.snapshot()` clones all values
2. **MessagePack buffer**: `rmp_serde::to_vec()` allocates Vec
3. **Compression buffer**: `zstd::encode_all()` allocates output
4. **Decompression buffer**: `zstd::decode_all()` allocates output

### Peak Memory During Save

```
Original data:           size(Environment) = E
Snapshot (clone):        E
SerializedValue:         ~E (similar size)
MessagePack buffer:      ~1.2 × E
Compression buffer:      ~1.5 × E (temporary)
──────────────────────────────────────────
Peak:                    ~4.7 × E
```

### Peak Memory During Load

```
File read:               size(file) = F
Decompression buffer:    ~10 × F (if compressed 10:1)
MessagePack buffer:      Same as decompression buffer
Environment:             ~size(decompressed)
──────────────────────────────────────────
Peak:                    ~11 × F (worst case)
```

### Optimization Strategies

**1. Streaming I/O**:
```rust
// Instead of reading entire file
let mut file_data = Vec::new();
reader.read_to_end(&mut file_data)?;

// Stream decompression
let decompressor = zstd::Decoder::new(reader)?;
let body: AchBody = rmp_serde::from_reader(decompressor)?;
```

**2. Selective Loading** (Future):
```rust
// Load only specific variables
let options = RestoreOptions {
    include_only: Some(vec!["result".into()]),
    ..Default::default()
};
```

**3. Memory-Mapped Files** (Future):
```rust
use memmap2::Mmap;
let mmap = unsafe { Mmap::map(&file)? };
let header = AchHeader::read(&mut &mmap[..64])?;
```

---

## Implementation References

### Standards and Formats

1. **MessagePack Specification**: https://msgpack.org/
   - Binary serialization format
   - Type system mapping

2. **Zstandard Specification**: https://github.com/facebook/zstd
   - Compression algorithm
   - Level recommendations

3. **SHA-256 Specification**: FIPS 180-4
   - Cryptographic hash function
   - Collision resistance properties

### Similar Implementations

1. **Serde**: https://serde.rs/
   - Serialization framework design
   - Visitor pattern

2. **rkyv**: https://rkyv.org/
   - Zero-copy deserialization
   - Archive format design

3. **bincode**: https://github.com/bincode-org/bincode
   - Simple binary encoding
   - Rust-specific format

### Design Inspirations

1. **PNG Format**: https://www.w3.org/TR/PNG/
   - Chunk-based structure
   - CRC integrity checks

2. **HDF5**: https://www.hdfgroup.org/
   - Scientific data storage
   - Metadata management

3. **SQLite Format**: https://sqlite.org/fileformat.html
   - Header design
   - Version management

---

## Future Work

### Planned Features

**v1.1 - Function Serialization**:
- Serialize user-defined functions (AST + captured environment)
- Function registry for builtin restoration
- Closure capture tracking

**v1.2 - Namespace Mode**:
- Restore into nested namespace
- Multiple file merging
- Workspace composition

**v2.0 - Advanced Features**:
- Incremental saves (delta encoding)
- Transaction log for session replay
- Optional AES encryption
- Streaming API for huge files

**v2.1 - Optimization**:
- Memory-mapped I/O
- Parallel serialization
- Custom MessagePack encoder (skip intermediate SerializedValue)

### Known Limitations

1. **No cross-version compatibility**: v1.x can't read v2.x files
2. **No function restoration**: Builtin functions can't be restored yet
3. **Full deserialization for metadata**: `get_metadata()` reads entire file
4. **Memory overhead**: Peak usage ~5x data size
5. **No incremental saves**: Must save entire environment each time

---

## Contributing Guidelines

When modifying this crate:

1. **Maintain format stability**:
   - Don't change header layout without major version bump
   - Use reserved bytes for new fields
   - Add flag bits for new features

2. **Add tests for new code**:
   - Unit tests for new functions
   - Integration tests for new workflows
   - Roundtrip tests for new serialization types

3. **Update documentation**:
   - Update this README for implementation changes
   - Update crate README for API changes
   - Add examples for new features

4. **Benchmark performance**:
   - Use criterion for microbenchmarks
   - Test on large files (> 100 MB)
   - Check memory usage (valgrind, heaptrack)

5. **Consider backward compatibility**:
   - Old files should load in new versions
   - Graceful degradation for unsupported features
   - Clear error messages for incompatibilities

---

**Navigation**: [Crate README](../README.md) | [Achronyme Core](../../../README.md)
