# Built-in Functions Implementation Plan (Updated)

**Date**: 2025-11-13 (Updated after codebase investigation)
**Priority**: 2 (After TCO)
**Estimated Duration**: 1-2 weeks
**Status**: Planning Phase

---

## 🔍 Current State Analysis

After investigating the codebase, many functions are **already implemented**:

### ✅ Already Implemented (DO NOT re-implement)
- **HOFs**: `map`, `filter`, `reduce`, `pipe` (handlers/hof.rs)
- **Stats**: `sum`, `mean`, `std`, `min`, `max` (function_modules/stats.rs)
- **Vectors**: `dot`, `cross`, `norm`, `normalize` (function_modules/vector.rs)
- **Math**: Complete trig, exp, log, rounding functions
- **Utils**: `linspace` (similar to range, but for floats)
- **Strings**: `length`, `concat` (only for strings, not arrays)

### ❌ Missing Functions (IMPLEMENT THESE)
Focus on the gaps in array operations that users commonly need:

---

## 🎯 Revised Objectives

Implement **missing** built-in functions that complement existing functionality:

### Goals
1. **Fill gaps**: Add functions that are missing but commonly needed
2. **Array utilities**: Extend array operations beyond HOFs
3. **Consistency**: Match existing naming conventions and patterns
4. **Performance**: Native implementations for operations users do recursively

---

## 📊 Revised Priority Categories

### **Tier 1: Essential Missing Functions** (Week 1, Days 1-3)
Critical functions that users need but are missing:

| Function | Signature | Purpose | Status | Priority |
|----------|-----------|---------|--------|----------|
| `product` | `product(array)` | Multiply all elements | ❌ Missing | ⭐⭐⭐ |
| `range` | `range(start, end, step?)` | Integer sequence generator | ❌ Missing | ⭐⭐⭐ |
| `len` | `len(array)` | Array length (length is for strings) | ❌ Missing | ⭐⭐⭐ |
| `reverse` | `reverse(array)` | Reverse array order | ❌ Missing | ⭐⭐⭐ |

**Note:** `linspace` exists but only generates uniform float sequences. `range` should generate integer sequences like Python's `range(0, 10)`.

### **Tier 2: Predicates & Searches** (Week 1, Days 4-5)
Boolean predicates and search operations:

| Function | Signature | Purpose | Status | Priority |
|----------|-----------|---------|--------|----------|
| `any` | `any(array, predicate)` | Check if any match | ❌ Missing | ⭐⭐⭐ |
| `all` | `all(array, predicate)` | Check if all match | ❌ Missing | ⭐⭐⭐ |
| `find` | `find(array, predicate)` | Find first matching element | ❌ Missing | ⭐⭐ |
| `findIndex` | `findIndex(array, predicate)` | Find index of first match | ❌ Missing | ⭐⭐ |
| `count` | `count(array, predicate)` | Count matching elements | ❌ Missing | ⭐⭐ |
| `contains` | `contains(array, value)` | Check if value exists | ❌ Missing | ⭐⭐ |

### **Tier 3: Array Transformations** (Week 2, Days 1-3)
More specialized transformations:

| Function | Signature | Purpose | Status | Priority |
|----------|-----------|---------|--------|----------|
| `zip` | `zip(array1, array2)` | Combine two arrays | ✅ Implemented | ⭐⭐ |
| `flatten` | `flatten(nestedArray, depth?)` | Flatten nested arrays | ✅ Implemented | ⭐⭐ |
| `take` | `take(array, n)` | Take first n elements | ✅ Implemented | ⭐⭐ |
| `drop` | `drop(array, n)` | Skip first n elements | ✅ Implemented | ⭐⭐ |
| `slice` | `slice(array, start, end?)` | Extract subarray | ✅ Implemented | ⭐⭐ |
| `unique` | `unique(array)` | Remove duplicates | ✅ Implemented | ⭐ |
| `chunk` | `chunk(array, size)` | Split into chunks | ✅ Implemented | ⭐ |

### **Tier 4: Sorting & Advanced** (Week 2, Days 4-5)
Sorting and advanced operations:

| Function | Signature | Purpose | Status | Priority |
|----------|-----------|---------|--------|----------|
| `sort` | `sort(array, compareFn?)` | Sort array | ❌ Missing | ⭐⭐ |
| `sortBy` | `sortBy(array, keyFn)` | Sort by key function | ❌ Missing | ⭐ |
| `groupBy` | `groupBy(array, keyFn)` | Group by key function | ❌ Missing | ⭐ |
| `partition` | `partition(array, predicate)` | Split into [true, false] | ❌ Missing | ⭐ |

### **Tier 5: Statistical Extensions** (Optional)
Expand existing stats module:

| Function | Signature | Purpose | Status | Priority |
|----------|-----------|---------|--------|----------|
| `variance` | `variance(array)` | Statistical variance | ❌ Missing | ⭐ |
| `median` | `median(array)` | Middle value | ❌ Missing | ⭐ |
| `mode` | `mode(array)` | Most frequent value | ❌ Missing | ⭐ |

**Note:** `std` (standard deviation) already exists, so `variance` can be computed as `std(arr)^2`.

---

## 🏗️ Revised Implementation Strategy

### Phase 1: Create Array Utilities Module (Day 1)

#### 1.1 Create New Module for Array Operations
Following the existing pattern in `crates/achronyme-eval/src/function_modules/`:

```rust
// crates/achronyme-eval/src/function_modules/array.rs
use achronyme_parser::ast::AstNode;
use achronyme_types::value::Value;
use crate::evaluator::Evaluator;

/// Register all array utility functions
pub fn register(registry: &mut crate::functions::FunctionRegistry) {
    // Tier 1: Essential
    registry.register("product".to_string(), 1, product);
    registry.register("range".to_string(), -1, range);  // -1 = variadic (2-3 args)
    registry.register("len".to_string(), 1, len);
    registry.register("reverse".to_string(), 1, reverse);

    // Tier 2: Predicates
    registry.register("any".to_string(), 2, any);
    registry.register("all".to_string(), 2, all);
    registry.register("find".to_string(), 2, find);
    registry.register("findIndex".to_string(), 2, find_index);
    registry.register("count".to_string(), 2, count);
    registry.register("contains".to_string(), 2, contains);
}
```

#### 1.2 Update Function Registry
In `crates/achronyme-eval/src/functions.rs`, add registration:

```rust
impl FunctionRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            functions: HashMap::new(),
        };

        // Existing registrations
        Self::register_math_functions(&mut registry);
        Self::register_trig_functions(&mut registry);
        // ... other existing modules ...

        // NEW: Array utilities
        function_modules::array::register(&mut registry);

        registry
    }
}
```

#### 1.3 Update Module Exports
In `crates/achronyme-eval/src/function_modules/mod.rs`:

```rust
pub mod array;      // NEW
pub mod complex;
pub mod dsp;
pub mod exponential;
// ... existing modules ...
```

---

### Phase 2: Tier 1 Implementation (Day 3-5)

#### 2.1 Array Operations (`builtins/array.rs`)

**`sum(array)` - Sum all numeric elements**
```rust
/// Sum all elements in a numeric array
///
/// Examples:
/// - sum([1, 2, 3, 4, 5]) => 15
/// - sum([1.5, 2.5, 3.0]) => 7.0
/// - sum([]) => 0
///
/// Performance: O(n) single pass
pub fn sum(evaluator: &mut Evaluator, args: &[&AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("sum() expects 1 argument".to_string());
    }

    let array = evaluator.evaluate(args[0])?;

    match array {
        Value::Vector(vec) => {
            let mut sum = 0.0;
            for val in vec {
                match val {
                    Value::Number(n) => sum += n,
                    _ => return Err("sum() requires numeric array".to_string()),
                }
            }
            Ok(Value::Number(sum))
        }
        Value::Tensor(tensor) => {
            // Optimized tensor sum
            Ok(Value::Number(tensor.data().iter().sum()))
        }
        _ => Err("sum() requires an array".to_string()),
    }
}
```

**`length(array)` - Get array length**
```rust
/// Get the length of an array or tensor
///
/// Examples:
/// - length([1, 2, 3]) => 3
/// - length([]) => 0
/// - length("hello") => 5 (string length)
///
/// Performance: O(1)
pub fn length(evaluator: &mut Evaluator, args: &[&AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("length() expects 1 argument".to_string());
    }

    let value = evaluator.evaluate(args[0])?;

    let len = match value {
        Value::Vector(vec) => vec.len(),
        Value::Tensor(tensor) => tensor.data().len(),
        Value::String(s) => s.len(),
        _ => return Err("length() requires array or string".to_string()),
    };

    Ok(Value::Number(len as f64))
}
```

**`range(start, end, step?)` - Generate number sequence**
```rust
/// Generate a range of numbers
///
/// Examples:
/// - range(0, 5) => [0, 1, 2, 3, 4]
/// - range(1, 10, 2) => [1, 3, 5, 7, 9]
/// - range(5, 0, -1) => [5, 4, 3, 2, 1]
///
/// Performance: O(n) where n = (end - start) / step
pub fn range(evaluator: &mut Evaluator, args: &[&AstNode]) -> Result<Value, String> {
    if args.len() < 2 || args.len() > 3 {
        return Err("range() expects 2 or 3 arguments".to_string());
    }

    let start = match evaluator.evaluate(args[0])? {
        Value::Number(n) => n,
        _ => return Err("range() start must be a number".to_string()),
    };

    let end = match evaluator.evaluate(args[1])? {
        Value::Number(n) => n,
        _ => return Err("range() end must be a number".to_string()),
    };

    let step = if args.len() == 3 {
        match evaluator.evaluate(args[2])? {
            Value::Number(n) => {
                if n == 0.0 {
                    return Err("range() step cannot be zero".to_string());
                }
                n
            }
            _ => return Err("range() step must be a number".to_string()),
        }
    } else {
        if end >= start { 1.0 } else { -1.0 }
    };

    // Validate step direction
    if (end > start && step < 0.0) || (end < start && step > 0.0) {
        return Err("range() step direction doesn't match start/end".to_string());
    }

    let mut result = Vec::new();
    let mut current = start;

    if step > 0.0 {
        while current < end {
            result.push(Value::Number(current));
            current += step;
        }
    } else {
        while current > end {
            result.push(Value::Number(current));
            current += step;
        }
    }

    Ok(Value::Vector(result))
}
```

#### 2.2 Functional Operations (`builtins/functional.rs`)

**`map(array, fn)` - Transform each element**
```rust
/// Apply a function to each element of an array
///
/// Examples:
/// - map([1, 2, 3], x => x * 2) => [2, 4, 6]
/// - map([1, 2, 3], x => x^2) => [1, 4, 9]
///
/// Performance: O(n) single pass
pub fn map(evaluator: &mut Evaluator, args: &[&AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("map() expects 2 arguments".to_string());
    }

    let array = evaluator.evaluate(args[0])?;
    let func_value = evaluator.evaluate(args[1])?;

    let func = match func_value {
        Value::Function(f) => f,
        _ => return Err("map() second argument must be a function".to_string()),
    };

    match array {
        Value::Vector(vec) => {
            let mut result = Vec::new();
            for item in vec {
                let mapped = evaluator.apply_lambda(&func, vec![item])?;
                result.push(mapped);
            }
            Ok(Value::Vector(result))
        }
        Value::Tensor(tensor) => {
            // Optimized tensor mapping
            let mut result_data = Vec::new();
            for &val in tensor.data() {
                let mapped = evaluator.apply_lambda(&func, vec![Value::Number(val)])?;
                match mapped {
                    Value::Number(n) => result_data.push(n),
                    _ => return Err("map() function must return numbers for tensor".to_string()),
                }
            }
            let result_tensor = RealTensor::new(result_data, tensor.shape().to_vec())
                .map_err(|e| format!("Failed to create tensor: {}", e))?;
            Ok(Value::Tensor(result_tensor))
        }
        _ => Err("map() requires an array".to_string()),
    }
}
```

**`filter(array, predicate)` - Filter elements**
```rust
/// Filter elements that satisfy a predicate
///
/// Examples:
/// - filter([1, 2, 3, 4, 5], x => x > 3) => [4, 5]
/// - filter([1, 2, 3, 4], x => x % 2 == 0) => [2, 4]
///
/// Performance: O(n) single pass
pub fn filter(evaluator: &mut Evaluator, args: &[&AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("filter() expects 2 arguments".to_string());
    }

    let array = evaluator.evaluate(args[0])?;
    let predicate_value = evaluator.evaluate(args[1])?;

    let predicate = match predicate_value {
        Value::Function(f) => f,
        _ => return Err("filter() second argument must be a function".to_string()),
    };

    match array {
        Value::Vector(vec) => {
            let mut result = Vec::new();
            for item in vec {
                let test_result = evaluator.apply_lambda(&predicate, vec![item.clone()])?;
                match test_result {
                    Value::Boolean(true) => result.push(item),
                    Value::Boolean(false) => {},
                    _ => return Err("filter() predicate must return boolean".to_string()),
                }
            }
            Ok(Value::Vector(result))
        }
        _ => Err("filter() requires an array".to_string()),
    }
}
```

**`reduce(array, fn, init)` - Fold/reduce operation**
```rust
/// Reduce an array to a single value using an accumulator function
///
/// Examples:
/// - reduce([1, 2, 3, 4], (acc, x) => acc + x, 0) => 10
/// - reduce([1, 2, 3, 4], (acc, x) => acc * x, 1) => 24
/// - reduce(["a", "b", "c"], (acc, x) => acc + x, "") => "abc"
///
/// Performance: O(n) single pass
pub fn reduce(evaluator: &mut Evaluator, args: &[&AstNode]) -> Result<Value, String> {
    if args.len() != 3 {
        return Err("reduce() expects 3 arguments".to_string());
    }

    let array = evaluator.evaluate(args[0])?;
    let func_value = evaluator.evaluate(args[1])?;
    let init = evaluator.evaluate(args[2])?;

    let func = match func_value {
        Value::Function(f) => f,
        _ => return Err("reduce() second argument must be a function".to_string()),
    };

    match array {
        Value::Vector(vec) => {
            let mut accumulator = init;
            for item in vec {
                accumulator = evaluator.apply_lambda(&func, vec![accumulator, item])?;
            }
            Ok(accumulator)
        }
        Value::Tensor(tensor) => {
            let mut accumulator = init;
            for &val in tensor.data() {
                accumulator = evaluator.apply_lambda(&func, vec![accumulator, Value::Number(val)])?;
            }
            Ok(accumulator)
        }
        _ => Err("reduce() requires an array".to_string()),
    }
}
```

---

### Phase 3: Tier 2 Implementation (Day 6-8)

#### 3.1 Additional Array Operations

**`reverse(array)` - Reverse array**
```rust
/// Reverse the order of elements in an array
///
/// Examples:
/// - reverse([1, 2, 3]) => [3, 2, 1]
/// - reverse(["a", "b", "c"]) => ["c", "b", "a"]
///
/// Performance: O(n)
pub fn reverse(evaluator: &mut Evaluator, args: &[&AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("reverse() expects 1 argument".to_string());
    }

    let array = evaluator.evaluate(args[0])?;

    match array {
        Value::Vector(mut vec) => {
            vec.reverse();
            Ok(Value::Vector(vec))
        }
        Value::Tensor(tensor) => {
            if !tensor.is_vector() {
                return Err("reverse() only works with 1D tensors".to_string());
            }
            let mut data = tensor.data().to_vec();
            data.reverse();
            let reversed = RealTensor::new(data, tensor.shape().to_vec())
                .map_err(|e| format!("Failed to create tensor: {}", e))?;
            Ok(Value::Tensor(reversed))
        }
        _ => Err("reverse() requires an array".to_string()),
    }
}
```

**`any(array, predicate)` - Check if any element matches**
```rust
/// Check if any element satisfies the predicate
///
/// Examples:
/// - any([1, 2, 3, 4], x => x > 3) => true
/// - any([1, 2, 3], x => x > 10) => false
///
/// Performance: O(n) short-circuits on first match
pub fn any(evaluator: &mut Evaluator, args: &[&AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("any() expects 2 arguments".to_string());
    }

    let array = evaluator.evaluate(args[0])?;
    let predicate_value = evaluator.evaluate(args[1])?;

    let predicate = match predicate_value {
        Value::Function(f) => f,
        _ => return Err("any() second argument must be a function".to_string()),
    };

    match array {
        Value::Vector(vec) => {
            for item in vec {
                let test_result = evaluator.apply_lambda(&predicate, vec![item])?;
                match test_result {
                    Value::Boolean(true) => return Ok(Value::Boolean(true)),
                    Value::Boolean(false) => continue,
                    _ => return Err("any() predicate must return boolean".to_string()),
                }
            }
            Ok(Value::Boolean(false))
        }
        _ => Err("any() requires an array".to_string()),
    }
}
```

**`all(array, predicate)` - Check if all elements match**
```rust
/// Check if all elements satisfy the predicate
///
/// Examples:
/// - all([2, 4, 6], x => x % 2 == 0) => true
/// - all([1, 2, 3], x => x > 0) => true
/// - all([1, 2, 3], x => x > 2) => false
///
/// Performance: O(n) short-circuits on first failure
pub fn all(evaluator: &mut Evaluator, args: &[&AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("all() expects 2 arguments".to_string());
    }

    let array = evaluator.evaluate(args[0])?;
    let predicate_value = evaluator.evaluate(args[1])?;

    let predicate = match predicate_value {
        Value::Function(f) => f,
        _ => return Err("all() second argument must be a function".to_string()),
    };

    match array {
        Value::Vector(vec) => {
            for item in vec {
                let test_result = evaluator.apply_lambda(&predicate, vec![item])?;
                match test_result {
                    Value::Boolean(false) => return Ok(Value::Boolean(false)),
                    Value::Boolean(true) => continue,
                    _ => return Err("all() predicate must return boolean".to_string()),
                }
            }
            Ok(Value::Boolean(true))
        }
        _ => Err("all() requires an array".to_string()),
    }
}
```

---

### Phase 4: Testing Strategy (Day 9-10)

#### 4.1 Test Structure
Create comprehensive test files for each category:

```
crates/achronyme-eval/tests/
├── test_builtin_array.rs       # sum, length, range, reverse
├── test_builtin_functional.rs  # map, filter, reduce
├── test_builtin_stats.rs       # variance, count, any, all
└── test_builtin_performance.rs # Performance benchmarks
```

#### 4.2 Test Coverage Requirements
For each built-in function:
- ✅ Basic functionality test
- ✅ Edge cases (empty arrays, single element)
- ✅ Type checking (wrong argument types)
- ✅ Arity checking (wrong number of args)
- ✅ Error messages validation
- ✅ Performance comparison vs recursive version

#### 4.3 Example Test Template
```rust
#[test]
fn test_sum_basic() {
    let mut evaluator = Evaluator::new();

    // Test basic sum
    let result = evaluator.eval_str("sum([1, 2, 3, 4, 5])").unwrap();
    assert_eq!(result, Value::Number(15.0));

    // Test empty array
    let result = evaluator.eval_str("sum([])").unwrap();
    assert_eq!(result, Value::Number(0.0));

    // Test with tensor
    let code = "let t = [1, 2, 3]; sum(t)";
    let result = evaluator.eval_str(code).unwrap();
    assert_eq!(result, Value::Number(6.0));
}

#[test]
fn test_sum_errors() {
    let mut evaluator = Evaluator::new();

    // Wrong arity
    let result = evaluator.eval_str("sum()");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("expects 1 argument"));

    // Non-numeric array
    let result = evaluator.eval_str("sum([\"a\", \"b\"])");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("numeric"));
}

#[test]
fn test_sum_performance() {
    let mut evaluator = Evaluator::new();

    // Generate large array
    evaluator.eval_str("let arr = range(0, 10000)").unwrap();

    let start = std::time::Instant::now();
    let result = evaluator.eval_str("sum(arr)").unwrap();
    let builtin_time = start.elapsed();

    // Expected: sum(0..9999) = 9999*10000/2 = 49,995,000
    assert_eq!(result, Value::Number(49_995_000.0));

    // Should be fast (< 10ms for 10k elements)
    assert!(builtin_time.as_millis() < 10);
}
```

---

## 📋 Implementation Checklist

### Week 1: Tier 1 Functions
- [ ] Create `builtins/` module structure
- [ ] Implement `builtins/array.rs`:
  - [ ] `sum(array)`
  - [ ] `product(array)`
  - [ ] `length(array)`
  - [ ] `range(start, end, step?)`
- [ ] Implement `builtins/functional.rs`:
  - [ ] `map(array, fn)`
  - [ ] `filter(array, predicate)`
  - [ ] `reduce(array, fn, init)`
- [ ] Write tests for Tier 1
- [ ] Documentation for Tier 1

### Week 2: Tier 2 Functions
- [ ] Implement additional array operations:
  - [ ] `reverse(array)`
  - [ ] `any(array, predicate)`
  - [ ] `all(array, predicate)`
  - [ ] `find(array, predicate)`
  - [ ] `count(array, predicate)`
- [ ] Expand `builtins/stats.rs`:
  - [ ] `variance(array)`
  - [ ] `quantile(array, q)`
- [ ] Implement `sort(array, compareFn?)`
- [ ] Write tests for Tier 2
- [ ] Documentation for Tier 2

### Week 3: Tier 3 & Polish
- [ ] Implement advanced transformations:
  - [ ] `zip(array1, array2)`
  - [ ] `flatten(array, depth?)`
  - [ ] `chunk(array, size)`
  - [ ] `take(array, n)`
  - [ ] `drop(array, n)`
  - [ ] `unique(array)`
- [ ] Performance benchmarks
- [ ] Complete documentation
- [ ] Update language docs

---

## 🎯 Success Criteria

### Must Have
- [ ] All Tier 1 functions implemented and tested
- [ ] 10-100x performance improvement vs recursive versions
- [ ] Comprehensive test coverage (>90%)
- [ ] All functions documented with examples
- [ ] Zero breaking changes to existing code

### Should Have
- [ ] All Tier 2 functions implemented
- [ ] Performance benchmarks comparing builtin vs recursive
- [ ] Error messages are clear and helpful
- [ ] Functions work with both Vector and Tensor types

### Nice to Have
- [ ] Tier 3 advanced transformations
- [ ] Tier 4 mathematical sequences with memoization
- [ ] Automatic type coercion where sensible
- [ ] Pipeline-friendly design (works well with `|>`)

---

## 📊 Expected Performance Gains

| Operation | Recursive (interpreted) | Built-in (native) | Speedup |
|-----------|------------------------|-------------------|---------|
| `sum(10k elements)` | ~50ms | ~0.5ms | **100x** |
| `map(10k elements)` | ~100ms | ~5ms | **20x** |
| `filter(10k elements)` | ~80ms | ~4ms | **20x** |
| `range(0, 10k)` | ~200ms | ~2ms | **100x** |
| `reduce(10k elements)` | ~120ms | ~6ms | **20x** |

---

## 🔄 Integration with Existing Code

### Function Registry Extension
All built-in functions will be registered in `FunctionRegistry::new()`:

```rust
impl FunctionRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            functions: HashMap::new(),
        };

        // Existing functions
        Self::register_math_functions(&mut registry);
        Self::register_trig_functions(&mut registry);
        Self::register_stats_functions(&mut registry);

        // NEW: Built-in array/functional operations
        builtins::array::register(&mut registry);
        builtins::functional::register(&mut registry);

        registry
    }
}
```

### No Breaking Changes
- All existing code continues to work
- Users can still define their own `sum`, `map`, etc. (will shadow built-ins)
- Built-ins follow existing conventions for error handling
- Compatible with TCO for user-defined recursive functions

---

## 📝 Documentation Updates Required

1. **`docs/language/20-functions.md`**
   - Add section on built-in array operations
   - Add examples of `map`, `filter`, `reduce`

2. **`docs/language/24-arrays-and-tensors.md`**
   - Update with new array operations
   - Show performance benefits

3. **`docs/language/23-best-practices.md`**
   - Recommend built-ins over manual recursion
   - Show idiomatic patterns

4. **New: `docs/language/26-builtin-functions.md`**
   - Complete reference for all built-in functions
   - Organized by category
   - Performance notes

---

## 🚀 Next Steps After Completion

Once built-in functions are complete:

1. **Performance Profiling**
   - Benchmark all built-ins vs recursive equivalents
   - Identify any remaining bottlenecks

2. **Phase 3: Memoization**
   - Implement automatic memoization for non-tail recursive functions
   - Add `@memo` decorator or similar

3. **User Feedback**
   - Gather feedback on which functions are most useful
   - Identify gaps in the standard library

---

**Author**: Claude (Achronyme Core Team)
**Status**: 📋 Planning Phase - Ready to Begin Implementation
