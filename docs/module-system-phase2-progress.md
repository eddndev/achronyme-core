# Module System - Phase 2: Module Resolution Infrastructure 🔄 IN PROGRESS

## Summary

Phase 2 is underway! The core Module Registry infrastructure is complete and tested. Next step is organizing built-in functions into prelude and modules.

## What's Been Implemented

### 1. Module Registry Infrastructure ✅

Created `crates/achronyme-eval/src/modules/mod.rs` with:

#### `Module` struct
```rust
pub struct Module {
    pub name: String,
    pub exports: HashMap<String, (BuiltinFunction, i32)>,
}
```

**Methods**:
- `new(name)` - Create empty module
- `register(name, func, arity)` - Add function to module
- `has(name)` - Check if function exists
- `get(name)` - Retrieve function
- `function_names()` - List all exported functions

#### `ModuleRegistry` struct
```rust
pub struct ModuleRegistry {
    prelude: HashMap<String, (BuiltinFunction, i32)>,
    modules: HashMap<String, Module>,
}
```

**Methods**:
- `new()` - Create empty registry
- `register_prelude(name, func, arity)` - Add to prelude
- `register_module(module)` - Register a module
- `resolve(name, imports)` - Resolve function name with import context
- `is_prelude(name)` - Check if in prelude
- `get_module(name)` - Retrieve module by name
- `module_names()` / `prelude_names()` - List available items

#### Resolution Logic

The `resolve()` method implements 3-tier resolution:

1. **Prelude** (always available) - Checked first
2. **Imported modules** - Uses import mapping
3. **Global fallback** (optional, for backward compat) - Behind `global-fallback` feature flag

```rust
pub fn resolve(
    &self,
    name: &str,
    imported_modules: &HashMap<String, (String, String)>,
) -> Option<(BuiltinFunction, i32)>
```

**Import mapping format**:
- Key: local_name (what user calls it)
- Value: (module_name, original_name)

**Examples**:
```rust
// import { sin } from "math"
imports.insert("sin", ("math", "sin"));

// import { mean as average } from "stats"
imports.insert("average", ("stats", "mean"));
```

### 2. Integration with Evaluator ✅ (Placeholder)

Added AST node handling in `evaluator.rs`:

```rust
AstNode::Import { items, module_path } => {
    // TODO: Implement import handling
    Err(format!("Import statements not yet implemented: {}", module_path))
}

AstNode::Export { items } => {
    // TODO: Implement export handling
    Err("Export statements not yet implemented".to_string())
}
```

**Status**: Placeholder error messages prevent crashes. Full implementation pending.

### 3. TCO Integration ✅

Updated `tco.rs` to handle new AST nodes:

```rust
// Import/Export are NOT tail positions (they're module declarations)
AstNode::Import { .. } => false,
AstNode::Export { .. } => false,
```

### 4. Comprehensive Tests ✅

Created 7 unit tests in `modules/mod.rs`:

- ✅ `test_module_creation` - Module creation and registration
- ✅ `test_registry_prelude` - Prelude registration
- ✅ `test_registry_modules` - Module registration
- ✅ `test_resolve_prelude` - Prelude resolution
- ✅ `test_resolve_imported` - Import resolution
- ✅ `test_resolve_aliased` - Alias resolution
- ✅ `test_prelude_takes_precedence` - Priority testing

**All 7 tests passing!** ✅

## What's Working

1. ✅ **Module creation** - Can create and register modules
2. ✅ **Prelude registration** - Can register functions in prelude
3. ✅ **Function resolution** - Can resolve functions from prelude or imports
4. ✅ **Alias support** - Import aliasing works correctly
5. ✅ **Priority system** - Prelude takes precedence over modules
6. ✅ **AST integration** - Import/Export nodes don't crash evaluator

## What Doesn't Work Yet

1. ❌ **Actual imports** - Import statements don't execute yet
2. ❌ **Populated modules** - No built-in functions organized yet
3. ❌ **Evaluator integration** - Module registry not connected to evaluator
4. ❌ **Function availability** - Can't actually use imported functions

## Next Steps

### Step 1: Organize Built-in Functions (In Progress)

Need to create a function that populates the ModuleRegistry with all built-in functions organized into prelude and modules.

**Prelude** (39 functions):
```javascript
// Math (15)
sin, cos, tan, sqrt, exp, ln, pow, abs, floor, ceil, round, min, max, pi, e

// Arrays & HOF (14)
map, filter, reduce, pipe, any, all, find, findIndex, count, sum, len, range, contains

// Control (2)
if, piecewise

// I/O (3)
print, type, str

// Strings (5)
concat, split, join, upper, lower
```

**Modules** to create:
- `math` - Advanced math (asin, acos, atan, atan2, sinh, cosh, tanh, log10, log2, etc.)
- `stats` - Statistics (mean, std, variance, median, etc.)
- `linalg` - Linear algebra (dot, cross, transpose, det, trace, norm)
- `dsp` - Signal processing (fft, ifft, fft_mag, conv, windows, linspace)
- `numerical` - Calculus (diff, diff2, diff3, integral, solve, newton, etc.)
- `graph` - Graph algorithms (bfs, dfs, dijkstra, kruskal, prim, etc.)
- `pert` - Project management (pert_analysis, critical_path, etc.)
- `optimization` - Linear programming (simplex, linprog, etc.)
- `complex` - Complex numbers (complex, real, imag, arg, conj)
- `strings` - Advanced strings (trim, starts_with, ends_with, replace, pad, etc.)
- `arrays` - Advanced arrays (reverse, product)
- `records` - Record utilities (keys, values, has_field)

### Step 2: Integrate with Evaluator

Replace `FunctionRegistry` with `ModuleRegistry` in the evaluator:

```rust
pub struct Evaluator {
    env: Environment,
    module_registry: ModuleRegistry,  // ← Replace functions: FunctionRegistry
    imported_modules: HashMap<String, (String, String)>,  // ← New: track imports
    // ... rest of fields
}
```

### Step 3: Implement Import Handling

Handle `AstNode::Import` in evaluator:

```rust
AstNode::Import { items, module_path } => {
    // 1. Check if module exists
    if !self.module_registry.has_module(module_path) {
        return Err(format!("Module '{}' not found", module_path));
    }

    // 2. Add each import to imported_modules map
    for item in items {
        let local_name = item.local_name();
        let original_name = &item.name;
        self.imported_modules.insert(
            local_name.to_string(),
            (module_path.clone(), original_name.clone())
        );
    }

    // 3. Return unit/success value
    Ok(Value::Boolean(true))
}
```

### Step 4: Update Function Resolution

Modify function call handling to use module registry:

```rust
// In handlers/function_call.rs
pub fn dispatch(evaluator: &mut Evaluator, name: &str, args: &[AstNode])
    -> Result<Value, String>
{
    // Resolve using module registry
    if let Some((func, arity)) = evaluator.module_registry.resolve(
        name,
        &evaluator.imported_modules
    ) {
        // Evaluate args and call function
        // ...
    } else {
        Err(format!("Undefined function: {}", name))
    }
}
```

## Files Created/Modified

### Created:
- `crates/achronyme-eval/src/modules/mod.rs` ✅

### Modified:
- `crates/achronyme-eval/src/lib.rs` ✅ (added modules export)
- `crates/achronyme-eval/src/evaluator.rs` ✅ (placeholder for Import/Export)
- `crates/achronyme-eval/src/tco.rs` ✅ (handle Import/Export in TCO)

## Compilation Status

✅ **Compiles successfully** with 3 minor warnings:
1. `unexpected_cfgs` for `global-fallback` feature (expected, will add to Cargo.toml later)
2. `unused variable` for `items` in Import handler (will be used in Step 3)
3. `unused variable` for `items` in Export handler (for future user modules)

✅ **All tests pass** (7/7 module tests + 8/8 parser tests)

## Backward Compatibility

✅ **Still fully backward compatible**
- Old code continues to work
- No breaking changes
- Import statements parsed but not required yet

## Architecture Decisions

### Why HashMap<String, (String, String)> for imports?

Format: `local_name -> (module_name, original_name)`

**Advantages**:
- O(1) lookup by local name
- Supports aliasing naturally
- Simple to implement
- Efficient for evaluator

**Example**:
```javascript
import { mean as avg, std } from "stats"
```

Maps to:
```rust
{
    "avg": ("stats", "mean"),
    "std": ("stats", "std"),
}
```

### Why prelude takes precedence?

Design decision: Prelude functions should never be shadowed by imports.

**Rationale**:
- Predictable behavior
- Prevents confusion
- Core functions always work
- Users can still alias if needed

**Example**:
```javascript
// Even if a module exports 'sum', prelude 'sum' is used
import { sum } from "custom"
let x = sum([1, 2, 3])  // Uses prelude sum, not custom.sum
```

## Technical Details

### Module Resolution Flow

```
User calls: average([1, 2, 3])
              ↓
evaluator.module_registry.resolve("average", imports)
              ↓
1. Check prelude["average"] → Not found
              ↓
2. Check imports["average"] → Found: ("stats", "mean")
              ↓
3. Check modules["stats"].get("mean") → Found!
              ↓
Return (mean_function, arity=1)
              ↓
Execute function
```

### Prelude Precedence Example

```
User calls: sin(pi)
              ↓
evaluator.module_registry.resolve("sin", imports)
              ↓
1. Check prelude["sin"] → Found!
              ↓
Return immediately (no module lookup)
              ↓
Execute prelude sin function
```

---

**Phase 2 Status: 🔄 50% COMPLETE**

**Completed**: ✅ Module Registry Infrastructure, Tests, Integration placeholders
**In Progress**: 🔄 Organizing built-in functions
**Pending**: ❌ Evaluator integration, Import execution

**Next**: Organize all built-in functions into prelude and modules
