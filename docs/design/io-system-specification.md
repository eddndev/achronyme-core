# Achronyme I/O System Specification

**Version:** 1.0 Draft
**Date:** 2025-01-13
**Status:** Planning Phase

## Executive Summary

This document outlines the design for a comprehensive I/O and module system for Achronyme. The goal is to transform Achronyme from a powerful computational engine into a production-ready tool for scientific computing, data analysis, and automation.

### Core Objectives

1. **File I/O**: Read and write various data formats (CSV, JSON, binary, text)
2. **Module System**: Import/export functionality for code organization
3. **Environment Persistence**: Save and restore computational state
4. **Data Interoperability**: Seamless integration with external data sources
5. **Security**: Safe file operations with appropriate sandboxing

---

## 1. Current State Analysis

### What We Have
- ✅ Powerful computational engine (DSP, linear algebra, numerical analysis)
- ✅ REPL environment with state management
- ✅ Function registry system
- ✅ Utility functions (print, type, str)
- ✅ WebAssembly target capability
- ✅ Serialization dependencies (serde, serde_json)

### What We're Missing
- ❌ File I/O operations
- ❌ Module/import system
- ❌ Environment save/load
- ❌ Structured data format support (CSV, JSON)
- ❌ Standard library organization
- ❌ Package/dependency management

---

## 2. Use Cases and Requirements

### 2.1 Data Science Workflow
```javascript
// Load CSV data
let data = read_csv("experiment_data.csv")

// Process data
let results = pipe(
    data,
    rows => map(row => row.temperature, rows),
    temps => filter(t => t > 20, temps),
    temps => {mean: mean(temps), std: std(temps)}
)

// Save results
write_json("results.json", results)
```

### 2.2 Reusable Libraries
```javascript
// File: math_utils.soc
export let factorial = n => do {
    if n <= 1 { 1 } else { n * factorial(n - 1) }
}

export let fibonacci = n => do {
    if n <= 1 { n } else { fibonacci(n - 1) + fibonacci(n - 2) }
}

// File: main.soc
import {factorial, fibonacci} from "./math_utils.soc"

print(factorial(5))    // 120
print(fibonacci(10))   // 55
```

### 2.3 Environment Persistence
```javascript
// Long computation session
mut workspace = {
    data: load_large_dataset(),
    models: train_models(),
    results: []
}

// Save entire environment
save_env("workspace_2025_01_13.ach")

// Later, restore state
restore_env("workspace_2025_01_13.ach")
print(workspace.results)
```

### 2.4 Configuration Files
```javascript
// config.json
{
    "model_params": {
        "learning_rate": 0.001,
        "epochs": 100
    },
    "data_source": "sensors/temperature.csv"
}

// main.soc
let config = read_json("config.json")
let data = read_csv(config.data_source)
```

---

## 3. System Architecture

### 3.1 Module Hierarchy

```
achronyme-io/          # New crate for I/O operations
├── src/
│   ├── lib.rs
│   ├── file_ops.rs    # Basic file read/write
│   ├── csv.rs         # CSV parsing/writing
│   ├── json.rs        # JSON parsing/writing
│   ├── binary.rs      # Binary data formats
│   └── errors.rs      # I/O error types

achronyme-modules/     # New crate for module system
├── src/
│   ├── lib.rs
│   ├── loader.rs      # Module loading
│   ├── resolver.rs    # Path resolution
│   ├── cache.rs       # Module caching
│   └── exports.rs     # Export tracking

achronyme-env/         # New crate for environment persistence
├── src/
│   ├── lib.rs
│   ├── serialize.rs   # Environment serialization
│   ├── persist.rs     # Save/load operations
│   └── snapshot.rs    # State snapshots
```

### 3.2 Integration with Existing Crates

```
achronyme-eval/
├── function_modules/
│   ├── io.rs          # I/O functions (new)
│   └── ...

achronyme-types/
├── value.rs           # May need extensions for file handles
└── environment.rs     # Environment state management
```

---

## 4. API Design

### 4.1 File Operations

#### Basic Text I/O
```javascript
// Read entire file as string
read_text(path: String) -> String

// Write string to file
write_text(path: String, content: String) -> Boolean

// Read file lines as vector
read_lines(path: String) -> Vector<String>

// Append to file
append_text(path: String, content: String) -> Boolean

// Check if file exists
file_exists(path: String) -> Boolean

// Get file metadata
file_info(path: String) -> Record {
    size: Number,
    created: String,
    modified: String,
    is_file: Boolean,
    is_dir: Boolean
}
```

#### Directory Operations
```javascript
// List directory contents
list_dir(path: String) -> Vector<String>

// Create directory
create_dir(path: String) -> Boolean

// Check if directory exists
dir_exists(path: String) -> Boolean

// Get current working directory
cwd() -> String
```

### 4.2 Structured Data Formats

#### CSV Operations
```javascript
// Read CSV as vector of records
read_csv(path: String, options?: Record) -> Vector<Record>
// Options: {
//   delimiter: ",",
//   has_header: true,
//   skip_rows: 0
// }

// Write CSV from vector of records
write_csv(path: String, data: Vector<Record>, options?: Record) -> Boolean

// Example:
let data = read_csv("data.csv")
// data = [
//   {name: "Alice", age: 30, score: 95.5},
//   {name: "Bob", age: 25, score: 87.2}
// ]

let filtered = filter(row => row.score > 90, data)
write_csv("high_scores.csv", filtered)
```

#### JSON Operations
```javascript
// Read JSON file
read_json(path: String) -> Value

// Write JSON file
write_json(path: String, data: Value, pretty?: Boolean) -> Boolean

// Parse JSON string
parse_json(json_string: String) -> Value

// Serialize to JSON string
to_json(value: Value, pretty?: Boolean) -> String

// Example:
let config = read_json("config.json")
config.last_run = timestamp()
write_json("config.json", config, true)  // pretty-printed
```

#### Binary Data
```javascript
// Read binary file as byte array (tensor of integers 0-255)
read_bytes(path: String) -> Tensor

// Write byte array to file
write_bytes(path: String, data: Tensor) -> Boolean

// Read/write numeric arrays efficiently
read_binary_array(path: String) -> Tensor
write_binary_array(path: String, data: Tensor) -> Boolean
```

### 4.3 Module System

#### Import/Export Syntax
```javascript
// --- Exporting (library.soc) ---

// Named exports
export let constant = 3.14159
export let helper = x => x * 2

// Export function
export let process_data = (data) => do {
    map(x => x * 2, data)
}

// --- Importing (main.soc) ---

// Import specific items
import {constant, helper} from "./library.soc"

// Import all as namespace
import * as lib from "./library.soc"
lib.constant  // Access as namespace

// Import with renaming
import {process_data as process} from "./library.soc"

// Relative paths
import {utils} from "./helpers/utils.soc"
import {math} from "../shared/math.soc"

// Standard library (future)
import {csv, json} from "std:io"
import {mean, std} from "std:stats"
```

#### Module Resolution Rules
1. **Relative paths**: `./file.soc`, `../other.soc`
2. **Absolute paths**: `/path/to/module.soc`
3. **Standard library**: `std:module_name`
4. **Search paths**: Configurable module search directories

#### Module Metadata
```javascript
// Each module has implicit metadata
module.path      // Current module's file path
module.dir       // Current module's directory
module.name      // Module name (filename without extension)
```

### 4.4 Environment Persistence

#### Save/Restore Operations
```javascript
// Save current environment state
save_env(path: String, options?: Record) -> Boolean
// Options: {
//   include_functions: true,
//   compress: false
// }

// Restore environment (merges with current)
restore_env(path: String, options?: Record) -> Boolean
// Options: {
//   overwrite: false,  // Overwrite existing bindings
//   namespace: null    // Load into namespace instead
// }

// Create snapshot (non-persistent)
let snapshot = snapshot_env()

// Restore from snapshot
restore_snapshot(snapshot)

// Example workflow:
let x = 42
let data = [1, 2, 3, 4, 5]
save_env("checkpoint.ach")

// ... later or in another session ...
restore_env("checkpoint.ach")
print(x)     // 42
print(data)  // [1, 2, 3, 4, 5]
```

#### Workspace Management
```javascript
// List all bindings in environment
env_bindings() -> Vector<String>

// Get binding value by name
env_get(name: String) -> Value

// Set binding value by name
env_set(name: String, value: Value) -> Boolean

// Remove binding
env_remove(name: String) -> Boolean

// Clear environment (dangerous!)
env_clear() -> Boolean

// Example:
let bindings = env_bindings()
print("Current environment:", bindings)
// ["x", "data", "PI", "sin", "cos", ...]
```

---

## 5. Implementation Phases

### Phase 1: Basic File I/O (Weeks 1-2)
**Priority: HIGH**

- [ ] Create `achronyme-io` crate
- [ ] Implement text file operations (read_text, write_text, read_lines)
- [ ] Add directory operations (list_dir, create_dir, file_exists)
- [ ] Register I/O functions in function registry
- [ ] Write comprehensive tests
- [ ] Add security/sandboxing considerations
- [ ] Document file I/O functions

**Deliverables:**
- Working file read/write from REPL
- 15-20 basic I/O functions
- Test suite with 50+ tests

### Phase 2: JSON Support (Week 3)
**Priority: HIGH**

- [ ] Implement JSON parsing (leveraging serde_json)
- [ ] Implement JSON serialization
- [ ] Handle all Value types <-> JSON mapping
- [ ] Add pretty-printing option
- [ ] Test with complex nested structures
- [ ] Document JSON functions

**Deliverables:**
- read_json/write_json functions
- parse_json/to_json string operations
- Full Value type compatibility

### Phase 3: CSV Support (Week 4)
**Priority: HIGH**

- [ ] Add CSV parsing library (csv crate)
- [ ] Implement CSV reader with options
- [ ] Implement CSV writer
- [ ] Handle headers, delimiters, quotes
- [ ] Type inference for columns
- [ ] Test with real-world datasets
- [ ] Document CSV functions

**Deliverables:**
- read_csv/write_csv functions
- Configurable parsing options
- Integration with records/tensors

### Phase 4: Module System Foundation (Weeks 5-6)
**Priority: MEDIUM**

- [ ] Create `achronyme-modules` crate
- [ ] Design AST extensions for import/export
- [ ] Implement module loader
- [ ] Implement path resolution
- [ ] Add module caching
- [ ] Handle circular dependencies
- [ ] Test module loading

**Deliverables:**
- Basic import/export syntax
- Module resolution system
- Cache mechanism

### Phase 5: Environment Persistence (Week 7)
**Priority: MEDIUM**

- [ ] Create `achronyme-env` crate
- [ ] Implement environment serialization
- [ ] Implement save_env/restore_env
- [ ] Handle function serialization challenges
- [ ] Add snapshot functionality
- [ ] Test with complex environments
- [ ] Document persistence functions

**Deliverables:**
- Environment save/load system
- Snapshot capabilities
- Workspace management functions

### Phase 6: Binary Data & Performance (Week 8)
**Priority: LOW**

- [ ] Implement binary file operations
- [ ] Add efficient tensor serialization
- [ ] Optimize large file handling
- [ ] Add streaming capabilities
- [ ] Benchmark performance
- [ ] Document binary functions

**Deliverables:**
- Binary data functions
- Performance optimizations
- Benchmarking results

---

## 6. Technical Considerations

### 6.1 Security

#### File System Sandboxing
```rust
// Configurable sandbox mode
pub struct IOConfig {
    pub sandbox_enabled: bool,
    pub allowed_paths: Vec<PathBuf>,
    pub max_file_size: usize,
    pub allow_writes: bool,
}
```

**Options:**
1. **REPL mode**: Full file system access (user responsibility)
2. **WASM mode**: No file system access (use virtual FS or browser APIs)
3. **Embedded mode**: Restricted sandbox with whitelist

#### Path Validation
- Prevent directory traversal attacks
- Validate path components
- Resolve symlinks carefully
- Check permissions before operations

### 6.2 Error Handling

```javascript
// Functions return Results or throw errors
try {
    let data = read_csv("missing.csv")
} catch (error) {
    print("Error:", error.message)
    // Fallback behavior
}

// Or check with conditional
if file_exists("data.csv") {
    let data = read_csv("data.csv")
} else {
    print("File not found")
}
```

### 6.3 Performance Considerations

#### Large Files
- Stream reading for large files
- Lazy loading for CSV/JSON
- Memory-mapped files for binary data
- Chunked processing

#### Caching
- Module cache to avoid reloading
- Parse result caching
- File metadata caching

### 6.4 Cross-Platform Compatibility

- Use `std::path` for path handling
- Handle Windows/Unix path differences
- Test on multiple platforms
- Document platform-specific behavior

### 6.5 WASM Considerations

**Challenge**: WASM has no direct file system access

**Solutions:**
1. **Browser**: Use File API for user-selected files
2. **Virtual FS**: In-memory file system for testing
3. **HTTP**: Fetch data from URLs
4. **IndexedDB**: Persistent storage in browser

```javascript
// Browser-specific (future consideration)
let data = read_url("https://api.example.com/data.json")
let local = read_browser_file()  // Opens file picker
```

---

## 7. Data Format Specifications

### 7.1 CSV Format

**Reading:**
```
name,age,score
Alice,30,95.5
Bob,25,87.2
```
→
```javascript
[
    {name: "Alice", age: 30, score: 95.5},
    {name: "Bob", age: 25, score: 87.2}
]
```

**Type Inference:**
- Numbers: Parse as f64 if contains `.` or scientific notation
- Integers: Parse as f64 (Achronyme uses unified Number type)
- Booleans: "true"/"false" → Boolean
- Strings: Everything else

**Options:**
- `delimiter`: "," (default), "\t", ";"
- `has_header`: true (default), false
- `skip_rows`: Number of initial rows to skip
- `quote_char`: '"' (default)
- `escape_char`: '\\' (default)

### 7.2 JSON Mapping

**JSON → Achronyme Value**

| JSON Type | Achronyme Type | Example |
|-----------|----------------|---------|
| number | Number | `42`, `3.14` |
| string | String | `"hello"` |
| boolean | Boolean | `true`, `false` |
| null | Special null value? | TBD |
| array | Vector or Tensor | `[1, 2, 3]` |
| object | Record | `{x: 10, y: 20}` |

**Achronyme Value → JSON**

| Achronyme Type | JSON Representation |
|----------------|---------------------|
| Number | number |
| String | string |
| Boolean | boolean |
| Vector | array |
| Tensor | array (flattened or nested based on shape) |
| Record | object |
| Complex | object: `{"re": 3, "im": 4}` |
| Function | error or `{"_type": "function", "name": "..."}` |

**Handling Non-Serializable Types:**
- Functions: Error by default, or special marker
- TailCall: Error
- MutableRef: Serialize inner value

### 7.3 Binary Format (.ach - Achronyme Binary)

**Custom binary format for efficient storage:**

```
Header:
- Magic bytes: "ACH\0" (4 bytes)
- Version: u16 (2 bytes)
- Flags: u16 (2 bytes)

Sections:
1. Value table
2. Environment bindings
3. Function definitions (bytecode or serialized AST?)
4. Metadata

Compression: Optional zstd compression
```

---

## 8. Standard Library Organization

### Future Standard Library Structure

```
std:core         # Core functions (always available)
std:math         # Mathematical functions
std:stats        # Statistical functions
std:linalg       # Linear algebra
std:dsp          # Digital signal processing
std:io           # I/O operations
std:csv          # CSV utilities
std:json         # JSON utilities
std:string       # String utilities
std:array        # Array utilities
std:graph        # Graph algorithms
std:optimization # Optimization functions
```

**Usage:**
```javascript
import {mean, std, median} from "std:stats"
import {read_csv, write_csv} from "std:csv"
```

---

## 9. Examples and Testing

### 9.1 Integration Test Examples

```javascript
// Test: CSV workflow
let data = [
    {name: "Alice", score: 95},
    {name: "Bob", score: 87},
    {name: "Carol", score: 92}
]
write_csv("test.csv", data)
let loaded = read_csv("test.csv")
assert(len(loaded) == 3)

// Test: JSON round-trip
let config = {
    version: 1.0,
    enabled: true,
    params: [1, 2, 3]
}
write_json("config.json", config)
let restored = read_json("config.json")
assert(restored.version == 1.0)

// Test: Module import
// file1.soc:
export let add = (a, b) => a + b

// file2.soc:
import {add} from "./file1.soc"
assert(add(2, 3) == 5)
```

### 9.2 Benchmark Scenarios

1. **Large CSV**: 1M rows, 10 columns
2. **Nested JSON**: 10 levels deep, 1000 objects
3. **Binary tensors**: 1000x1000 matrix
4. **Module loading**: 100 modules with dependencies

---

## 10. Migration Path

### Backward Compatibility

- All existing code continues to work
- New features are additive
- No breaking changes to core language

### Deprecation Policy

- Functions marked deprecated for 2 releases
- Clear migration guides
- Automated migration tools if possible

---

## 11. Open Questions

### Q1: Module Syntax
- Use `import/export` keywords (JavaScript-style)?
- Use `use/pub` keywords (Rust-style)?
- Use function calls: `import("./module.soc")`?

**Recommendation**: JavaScript-style for familiarity

### Q2: Null Handling
- Add explicit null/nil value?
- Use empty record `{}` as null?
- Use special marker value?

**Recommendation**: Add explicit null, map to JSON null

### Q3: Function Serialization
- Serialize function AST?
- Only builtin functions?
- Error on function serialization?

**Recommendation**: Phase 1 - only builtins, Phase 2 - AST serialization

### Q4: Paths in WASM
- Virtual file system?
- Browser file API only?
- Network URLs only?

**Recommendation**: All three options, user selects

### Q5: Async I/O
- Synchronous only initially?
- Add async/await later?
- Promise-based?

**Recommendation**: Sync only for Phase 1-3, async in future

---

## 12. Success Metrics

### Phase 1-3 Success Criteria
- [ ] Can read/write CSV files with >1M rows
- [ ] JSON round-trip preserves all supported types
- [ ] I/O operations are documented and tested
- [ ] Performance benchmarks meet targets
- [ ] Zero security vulnerabilities in path handling

### Phase 4-5 Success Criteria
- [ ] Can create reusable libraries
- [ ] Module system handles circular dependencies
- [ ] Environment persistence works for complex sessions
- [ ] 100+ real-world use cases validated

### Overall Success
- [ ] Users can perform complete data analysis workflows
- [ ] Code organization matches industry best practices
- [ ] Achronyme competitive with Python/Julia for scientific computing
- [ ] No critical bugs in production use

---

## 13. Timeline Summary

| Phase | Duration | Deliverables | Risk Level |
|-------|----------|--------------|------------|
| 1. Basic File I/O | 2 weeks | Text operations, directory ops | Low |
| 2. JSON Support | 1 week | JSON read/write, parsing | Low |
| 3. CSV Support | 1 week | CSV read/write with options | Medium |
| 4. Module System | 2 weeks | Import/export, resolution | High |
| 5. Env Persistence | 1 week | Save/restore environment | Medium |
| 6. Binary & Perf | 1 week | Binary data, optimization | Low |
| **Total** | **8 weeks** | Full I/O system | - |

---

## 14. Next Steps

### Immediate Actions (This Week)
1. ✅ Create this specification document
2. ⏳ Review and discuss with stakeholders
3. ⏳ Create `achronyme-io` crate skeleton
4. ⏳ Implement `read_text` and `write_text` as POC
5. ⏳ Write first integration test

### Week 2
1. Complete basic file I/O functions
2. Add directory operations
3. Comprehensive testing
4. Documentation

### Week 3
1. JSON support implementation
2. Integration with existing Value types
3. Testing with complex data structures

---

## Appendix A: Related Technologies

### Similar Systems for Inspiration

**Python:**
- `open()`, `read()`, `write()`
- `json` module
- `csv` module
- `pickle` for object serialization

**Julia:**
- `CSV.jl`
- `JSON.jl`
- FileIO.jl ecosystem
- `save()` / `load()` for workspace

**R:**
- `read.csv()` / `write.csv()`
- `readRDS()` / `saveRDS()`
- `save.image()` for workspace

**JavaScript/Node.js:**
- `fs` module
- `require()` / `import`
- `JSON.parse()` / `JSON.stringify()`

---

## Appendix B: Code Examples Repository

**Location**: `examples/io/`

Planned examples:
1. `01-basic-file-io.soc` - Read/write text files
2. `02-csv-analysis.soc` - Load and analyze CSV data
3. `03-json-config.soc` - Configuration management
4. `04-data-pipeline.soc` - Complete ETL pipeline
5. `05-module-system.soc` - Creating reusable libraries
6. `06-workspace-persistence.soc` - Save/restore sessions

---

## Revision History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 Draft | 2025-01-13 | Claude | Initial specification |

