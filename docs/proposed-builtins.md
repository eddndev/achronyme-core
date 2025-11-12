# Proposed Built-in Functions

This document lists functions that should be implemented as built-in to replace recursive patterns that are limited by stack depth (~50 calls).

## Array Construction Functions

### `zeros(n)` → Tensor
Create a 1D tensor filled with zeros.

**Signature**: `zeros(n: Number) -> Tensor`

**Examples**:
```javascript
zeros(5)        // [0, 0, 0, 0, 0]
zeros(10)       // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
zeros(100)      // Works for any size
```

**Current workaround**:
```javascript
// Limited to n < 50 due to recursion
let zeros = n => (
    (left, vector) => if(left == 0, vector, rec(left - 1, [0, ...vector]))
)(n, [])
```

**Implementation location**: `crates/achronyme-eval/src/handlers/array_construction.rs` (new file)

---

### `ones(n)` → Tensor
Create a 1D tensor filled with ones.

**Signature**: `ones(n: Number) -> Tensor`

**Examples**:
```javascript
ones(5)         // [1, 1, 1, 1, 1]
ones(10)        // [1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
ones(1000)      // Works for any size
```

**Current workaround**:
```javascript
// Limited to n < 50 due to recursion
let ones = n => (
    (left, vector) => if(left == 0, vector, rec(left - 1, [1, ...vector]))
)(n, [])
```

---

### `range(n)` → Tensor
### `range(start, end)` → Tensor
### `range(start, end, step)` → Tensor

Generate sequence of numbers.

**Signatures**:
- `range(n: Number) -> Tensor` - Generate [0, 1, 2, ..., n-1]
- `range(start: Number, end: Number) -> Tensor` - Generate [start, start+1, ..., end-1]
- `range(start: Number, end: Number, step: Number) -> Tensor` - Generate with custom step

**Examples**:
```javascript
// Single argument: 0 to n-1
range(5)                // [0, 1, 2, 3, 4]
range(10)               // [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]

// Two arguments: start to end-1
range(5, 10)            // [5, 6, 7, 8, 9]
range(0, 5)             // [0, 1, 2, 3, 4]

// Three arguments: start to end-1 with step
range(0, 10, 2)         // [0, 2, 4, 6, 8]
range(1, 10, 3)         // [1, 4, 7]
range(10, 0, -1)        // [10, 9, 8, 7, 6, 5, 4, 3, 2, 1]
range(5, 0, -2)         // [5, 3, 1]
```

**Current workaround**:
```javascript
// Limited to n < 50 due to recursion
let range = n => (
    (left, current, vector) =>
        if(left == 0, vector, rec(left - 1, current + 1, [...vector, current]))
)(n, 0, [])
```

**Note**: Python-style range (end is exclusive)

---

### `repeat(value, n)` → Vector/Tensor
Repeat a value n times.

**Signature**: `repeat(value: Any, n: Number) -> Vector`

**Examples**:
```javascript
repeat(0, 5)            // [0, 0, 0, 0, 0]
repeat(42, 3)           // [42, 42, 42]
repeat("x", 4)          // ["x", "x", "x", "x"]
repeat(true, 3)         // [true, true, true]
repeat({x: 10}, 2)      // [{x: 10}, {x: 10}]
```

**Current workaround**:
```javascript
// Limited to n < 50 due to recursion
let repeat = (value, n) => (
    (left, vector) =>
        if(left == 0, vector, rec(left - 1, [value, ...vector]))
)(n, [])
```

**Type inference**:
- If value is Number → returns Tensor
- Otherwise → returns Vector

---

### `fill(n, fn)` → Vector/Tensor
Create array by applying function to each index.

**Signature**: `fill(n: Number, fn: Function) -> Vector`

**Examples**:
```javascript
fill(5, x => x * x)         // [0, 1, 4, 9, 16]
fill(5, x => x * 2)         // [0, 2, 4, 6, 8]
fill(3, x => x * x * x)     // [0, 1, 8]
fill(5, i => i * 10)        // [0, 10, 20, 30, 40]

// Can create any pattern
fill(5, i => if(i % 2 == 0, "even", "odd"))
// ["even", "odd", "even", "odd", "even"]
```

**Current workaround**:
```javascript
// Limited to n < 50 due to recursion
let fill_with = (n, fn) => (
    (index, vector) =>
        if(index >= n, vector, rec(index + 1, [...vector, fn(index)]))
)(0, [])
```

**Alternative names considered**: `tabulate`, `generate`, `fill_with`

---

## Array Manipulation Functions

### `take(n, arr)` → Vector/Tensor
Take first n elements from array.

**Signature**: `take(n: Number, arr: Vector|Tensor) -> Vector|Tensor`

**Examples**:
```javascript
let data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]

take(3, data)           // [0, 1, 2]
take(5, data)           // [0, 1, 2, 3, 4]
take(0, data)           // []
take(100, data)         // [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] (clamped)
```

**Current workaround**:
```javascript
let take = (n, arr) => arr[..n]
// Issue: arr[..n] includes index n-1, not index n
// Correct: arr[..n-1] but this is awkward
```

**Note**: More intuitive than slicing with `arr[..n-1]`

---

### `drop(n, arr)` → Vector/Tensor
Drop first n elements from array.

**Signature**: `drop(n: Number, arr: Vector|Tensor) -> Vector|Tensor`

**Examples**:
```javascript
let data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]

drop(3, data)           // [3, 4, 5, 6, 7, 8, 9]
drop(5, data)           // [5, 6, 7, 8, 9]
drop(0, data)           // [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
drop(100, data)         // []
```

**Current workaround**:
```javascript
let drop = (n, arr) => arr[n..]
// This works well, but built-in would be faster
```

---

### `head(arr)` → Any
Get first element of array.

**Signature**: `head(arr: Vector|Tensor) -> Any`

**Examples**:
```javascript
head([1, 2, 3, 4, 5])       // 1
head([10, 20, 30])          // 10
head(["a", "b", "c"])       // "a"
```

**Current workaround**:
```javascript
let head = arr => arr[0]
```

**Error handling**: Should error on empty array

---

### `tail(arr)` → Vector/Tensor
Get all elements except first.

**Signature**: `tail(arr: Vector|Tensor) -> Vector|Tensor`

**Examples**:
```javascript
tail([1, 2, 3, 4, 5])       // [2, 3, 4, 5]
tail([10, 20, 30])          // [20, 30]
tail([42])                  // []
```

**Current workaround**:
```javascript
let tail = arr => arr[1..]
```

---

### `last(arr)` → Any
Get last element of array.

**Signature**: `last(arr: Vector|Tensor) -> Any`

**Examples**:
```javascript
last([1, 2, 3, 4, 5])       // 5
last([10, 20, 30])          // 30
last(["a", "b", "c"])       // "c"
```

**Current workaround**:
```javascript
let last = arr => arr[-1]
```

**Error handling**: Should error on empty array

---

### `init(arr)` → Vector/Tensor
Get all elements except last.

**Signature**: `init(arr: Vector|Tensor) -> Vector|Tensor`

**Examples**:
```javascript
init([1, 2, 3, 4, 5])       // [1, 2, 3, 4]
init([10, 20, 30])          // [10, 20]
init([42])                  // []
```

**Current workaround**:
```javascript
let init = arr => arr[..-1]
```

---

## 2D Array Construction

### `zeros_2d(rows, cols)` → Tensor
Create 2D tensor filled with zeros.

**Signature**: `zeros_2d(rows: Number, cols: Number) -> Tensor`

**Examples**:
```javascript
zeros_2d(3, 4)
// [[0, 0, 0, 0],
//  [0, 0, 0, 0],
//  [0, 0, 0, 0]]

zeros_2d(2, 3)
// [[0, 0, 0],
//  [0, 0, 0]]
```

**Current workaround**:
```javascript
// Requires zeros() which is also limited by recursion
let zeros_2d = (rows, cols) => (
    (rows_left, result) =>
        if(rows_left == 0, result, rec(rows_left - 1, [zeros(cols), ...result]))
)(rows, [])
```

---

### `ones_2d(rows, cols)` → Tensor
Create 2D tensor filled with ones.

**Signature**: `ones_2d(rows: Number, cols: Number) -> Tensor`

**Examples**:
```javascript
ones_2d(2, 3)
// [[1, 1, 1],
//  [1, 1, 1]]
```

---

### `identity(n)` → Tensor
Create n×n identity matrix.

**Signature**: `identity(n: Number) -> Tensor`

**Examples**:
```javascript
identity(3)
// [[1, 0, 0],
//  [0, 1, 0],
//  [0, 0, 1]]

identity(2)
// [[1, 0],
//  [0, 1]]

identity(4)
// [[1, 0, 0, 0],
//  [0, 1, 0, 0],
//  [0, 0, 1, 0],
//  [0, 0, 0, 1]]
```

**Alternative name**: `eye` (MATLAB/NumPy style)

---

## Utility Functions

### `reverse(arr)` → Vector/Tensor
Reverse array elements.

**Signature**: `reverse(arr: Vector|Tensor) -> Vector|Tensor`

**Examples**:
```javascript
reverse([1, 2, 3, 4, 5])        // [5, 4, 3, 2, 1]
reverse([10, 20, 30])           // [30, 20, 10]
reverse(["a", "b", "c"])        // ["c", "b", "a"]
```

**Current workaround**: None (would require recursion)

---

### `flatten(arr)` → Vector/Tensor
Flatten nested arrays into 1D array.

**Signature**: `flatten(arr: Vector|Tensor) -> Vector|Tensor`

**Examples**:
```javascript
flatten([[1, 2], [3, 4], [5, 6]])
// [1, 2, 3, 4, 5, 6]

flatten([[[1, 2]], [[3, 4]]])
// [1, 2, 3, 4] (fully flattened)
```

**Depth**: Should flatten all levels or just one level?
- Option 1: Full flatten (all levels)
- Option 2: Single level flatten (keep `flatten_deep` for full)

---

### `reshape(arr, shape)` → Tensor
Reshape array to new dimensions.

**Signature**: `reshape(arr: Vector|Tensor, shape: Vector) -> Tensor`

**Examples**:
```javascript
let data = [1, 2, 3, 4, 5, 6]

reshape(data, [2, 3])
// [[1, 2, 3],
//  [4, 5, 6]]

reshape(data, [3, 2])
// [[1, 2],
//  [3, 4],
//  [5, 6]]
```

**Error handling**: Product of shape dimensions must equal array length

---

## Implementation Priority

### High Priority (Most Commonly Needed)
1. ✅ `zeros(n)` - Essential for array initialization
2. ✅ `ones(n)` - Common in mathematical operations
3. ✅ `range(n)` / `range(start, end, step)` - Replaces recursive version
4. ✅ `fill(n, fn)` - Powerful for custom array generation
5. ✅ `identity(n)` - Important for linear algebra

### Medium Priority (Convenience)
6. ✅ `repeat(value, n)` - General-purpose repetition
7. ✅ `take(n, arr)` - Clearer than slicing
8. ✅ `drop(n, arr)` - Clearer than slicing
9. ✅ `reverse(arr)` - Common operation
10. ✅ `zeros_2d(rows, cols)` - 2D array initialization

### Lower Priority (Already Have Workarounds)
11. `head(arr)` - Can use `arr[0]`
12. `tail(arr)` - Can use `arr[1..]`
13. `last(arr)` - Can use `arr[-1]`
14. `init(arr)` - Can use `arr[..-1]`
15. `ones_2d(rows, cols)` - Less common than zeros_2d
16. `flatten(arr)` - Can use reduce
17. `reshape(arr, shape)` - Advanced feature

---

## Implementation Notes

### File Organization
Create new file: `crates/achronyme-eval/src/handlers/array_construction.rs`

Functions to implement:
- `handle_zeros(evaluator, args)`
- `handle_ones(evaluator, args)`
- `handle_range(evaluator, args)`
- `handle_repeat(evaluator, args)`
- `handle_fill(evaluator, args)`
- `handle_take(evaluator, args)`
- `handle_drop(evaluator, args)`
- `handle_reverse(evaluator, args)`
- `handle_identity(evaluator, args)`
- `handle_zeros_2d(evaluator, args)`
- `handle_ones_2d(evaluator, args)`

### Registration
Add to `crates/achronyme-eval/src/functions.rs`:
```rust
registry.register("zeros", "array_construction::handle_zeros");
registry.register("ones", "array_construction::handle_ones");
// ... etc
```

### Testing
Create comprehensive tests in:
- `crates/achronyme-eval/tests/test_array_construction.rs`

### Documentation Updates
After implementation, update:
- `docs/language/09-arrays-tensors.md` - Add builtin functions section
- `docs/language/11-higher-order-functions.md` - Reference these functions
- `docs/language/README.md` - Add to quick reference
- `examples/soc/` - Create new example file showcasing builtin functions

---

## Summary

**Total proposed**: 17 functions
**High priority**: 5 functions
**Medium priority**: 5 functions
**Lower priority**: 7 functions

These built-in functions will:
- ✅ Eliminate recursion depth limitations for common operations
- ✅ Improve performance (native Rust vs interpreted recursion)
- ✅ Provide clearer, more readable code
- ✅ Align with conventions from NumPy, MATLAB, Haskell

**Next steps**:
1. Complete language documentation (files 11-23)
2. Implement high-priority built-ins
3. Update documentation with new built-ins
4. Create comprehensive examples
