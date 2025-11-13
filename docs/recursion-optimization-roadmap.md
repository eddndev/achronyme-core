# Recursion Optimization Roadmap

## Executive Summary

This document outlines a comprehensive strategy to eliminate recursion limitations in Achronyme through three complementary approaches:

1. **Tail Call Optimization (TCO)** - Unlimited depth for tail-recursive patterns
2. **Built-in Functions** - Optimized implementations of common recursive patterns
3. **Memoization/Caching** - Automatic caching for expensive recursive computations

**Current State**: ~50 call depth limit for all recursion
**Target State**: Unlimited depth for most common patterns, with graceful degradation

---

## Priority-Based Implementation Plan

### 🔴 Priority 1: Tail Call Optimization (CRITICAL)
**Timeline**: 2 weeks | **Impact**: High | **Complexity**: Medium

#### Why First?
- Solves the most common use case (accumulator patterns)
- Foundation for other optimizations
- No backward compatibility issues
- Clear implementation path

#### Benefits
- ✅ Unlimited depth for tail-recursive functions
- ✅ GCD, range generation, iterative algorithms
- ✅ Enables functional programming patterns
- ✅ 2-5x performance improvement even at low depths

#### Patterns Solved
```javascript
// Tail-recursive factorial
let factorial = n => (
    (current, acc) =>
        if(current <= 1, acc, rec(current - 1, acc * current))
)(n, 1)

// GCD
let gcd = (a, b) => if(b == 0, a, rec(b, a % b))

// Range generation
let range = n => (
    (left, current, vector) =>
        if(left == 0, vector, rec(left - 1, current + 1, [...vector, current]))
)(n, 0, [])
```

#### Implementation Tasks
1. ✅ Detection module (`tco.rs`) - 2 days
2. ✅ Iterative loop in `apply_lambda()` - 3 days
3. ✅ Environment optimization - 2 days
4. ✅ Testing suite - 3 days
5. ✅ Documentation - 2 days

**Total**: ~12 days

---

### 🟡 Priority 2: Built-in Functions Library (HIGH)
**Timeline**: 3 weeks | **Impact**: High | **Complexity**: Low-Medium

#### Why Second?
- Quick wins for users
- Low implementation complexity
- Complements TCO (handles non-tail patterns)
- Immediate value

#### Strategy
Replace recursive implementations with optimized built-ins for common patterns.

#### Phase 2A: Mathematical Functions (Week 1)

**Category: Sequences & Series**

```javascript
// 1. Fibonacci (currently exponential, O(2^n))
fibonacci(n)           // Returns nth Fibonacci number
fibonacci_range(n)     // Returns [fib(0), fib(1), ..., fib(n)]

// Implementation: Iterative O(n), or closed form O(1)
// Internal: let mut a = 0, b = 1; loop { swap }

// 2. Factorial (currently limited to ~50)
factorial(n)           // Returns n!
factorial_range(n)     // Returns [0!, 1!, 2!, ..., n!]

// Implementation: Iterative with overflow detection
// Or use gamma function for large n: gamma(n+1)

// 3. Binomial coefficients
binomial(n, k)         // Returns C(n,k) = n! / (k!(n-k)!)
pascal_row(n)          // Returns nth row of Pascal's triangle

// 4. Prime numbers
is_prime(n)            // Primality test
primes_up_to(n)        // Sieve of Eratosthenes
nth_prime(n)           // Returns nth prime

// 5. GCD/LCM (optimize existing)
gcd(a, b)              // Already tail-recursive, TCO handles it
lcm(a, b)              // gcd-based implementation
gcd_list(arr)          // GCD of array
```

**Category: Combinatorics**

```javascript
// 6. Permutations & Combinations
permutations(arr)      // All permutations
combinations(arr, k)   // All k-combinations
powerset(arr)          // All subsets

// 7. Partitions
partitions(n)          // Integer partitions
compositions(n)        // Integer compositions
```

#### Phase 2B: Data Structure Functions (Week 2)

**Category: Array/List Operations**

```javascript
// 1. Range generation (replace recursive version)
range(start, stop, step)     // Like Python's range
linspace(start, stop, n)     // Already exists, verify optimization

// 2. Repeat/Fill
repeat(value, n)             // [value, value, ..., value] n times
zeros(n)                     // [0, 0, ..., 0]
ones(n)                      // [1, 1, ..., 1]
fill(n, fn)                  // [fn(0), fn(1), ..., fn(n-1)]

// 3. Flatten
flatten(arr)                 // Recursive flatten [[1], [2, [3]]] -> [1,2,3]
flatten_depth(arr, depth)    // Flatten to specific depth

// 4. Partition/Chunk
chunk(arr, size)             // Split into chunks [[1,2], [3,4], [5]]
partition(arr, pred)         // Split by predicate {true: [...], false: [...]}
```

**Category: String Processing**

```javascript
// 1. String generation
repeat_str(s, n)             // Repeat string n times
join(arr, sep)               // Already exists, verify
split(s, sep)                // String to array

// 2. String transformation
reverse_str(s)               // Reverse string
chars(s)                     // String to char array
from_chars(arr)              // Char array to string
```

#### Phase 2C: Algorithm Functions (Week 3)

**Category: Searching & Sorting**

```javascript
// 1. Binary search
binary_search(arr, target)            // Returns index or -1
binary_search_insert(arr, target)     // Returns insertion index

// 2. Sorting algorithms (optimized)
quicksort(arr)                        // O(n log n) iterative
mergesort(arr)                        // O(n log n) stable
heapsort(arr)                         // O(n log n) in-place

// 3. Selection algorithms
nth_element(arr, n)                   // Nth smallest element
median(arr)                           // Median value
quantile(arr, q)                      // q-th quantile
```

**Category: Tree/Graph Traversal**

```javascript
// 1. Tree operations (for nested structures)
tree_map(tree, fn)                    // Map over tree
tree_fold(tree, fn, init)            // Fold tree
tree_depth(tree)                      // Max depth
tree_size(tree)                       // Node count

// 2. Path finding (complement graph module)
all_paths(graph, start, end)          // All paths between nodes
shortest_paths(graph, start)          // Dijkstra from start
```

#### Implementation Strategy

**File Structure**:
```
crates/achronyme-builtins/src/
├── lib.rs                    # Registry
├── sequences.rs              # Fibonacci, factorial, primes
├── combinatorics.rs          # Permutations, combinations
├── arrays.rs                 # Range, repeat, flatten
├── strings.rs                # String operations
├── sorting.rs                # Sort algorithms
├── search.rs                 # Binary search
└── trees.rs                  # Tree operations
```

**Registration**:
```rust
// In function registry
pub fn register_builtin_recursion_replacements(registry: &mut FunctionRegistry) {
    // Sequences
    registry.register("fibonacci", builtin_fibonacci);
    registry.register("factorial", builtin_factorial);
    registry.register("binomial", builtin_binomial);

    // Arrays
    registry.register("range", builtin_range);
    registry.register("repeat", builtin_repeat);
    registry.register("flatten", builtin_flatten);

    // Combinatorics
    registry.register("permutations", builtin_permutations);
    registry.register("combinations", builtin_combinations);

    // ... etc
}
```

**Example Implementation**:
```rust
// crates/achronyme-builtins/src/sequences.rs

/// Fibonacci number (iterative, O(n))
pub fn builtin_fibonacci(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("fibonacci expects 1 argument".to_string());
    }

    let n = args[0].as_number()?;
    if n < 0.0 {
        return Err("fibonacci requires non-negative integer".to_string());
    }

    let n = n as usize;

    // Handle base cases
    if n == 0 { return Ok(Value::Number(0.0)); }
    if n == 1 { return Ok(Value::Number(1.0)); }

    // Iterative computation - O(n) time, O(1) space
    let mut a = 0.0;
    let mut b = 1.0;

    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }

    Ok(Value::Number(b))
}

/// Factorial (iterative with overflow detection)
pub fn builtin_factorial(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("factorial expects 1 argument".to_string());
    }

    let n = args[0].as_number()?;
    if n < 0.0 {
        return Err("factorial requires non-negative integer".to_string());
    }

    let n = n as usize;

    // Use f64, but warn about precision loss above ~170
    if n > 170 {
        return Err(format!("factorial({}) exceeds f64 precision (max 170)", n));
    }

    let mut result = 1.0;
    for i in 2..=n {
        result *= i as f64;
    }

    Ok(Value::Number(result))
}

/// Range generation (replaces recursive version)
pub fn builtin_range(args: &[Value]) -> Result<Value, String> {
    // range(stop) or range(start, stop) or range(start, stop, step)
    let (start, stop, step) = match args.len() {
        1 => (0.0, args[0].as_number()?, 1.0),
        2 => (args[0].as_number()?, args[1].as_number()?, 1.0),
        3 => (args[0].as_number()?, args[1].as_number()?, args[2].as_number()?),
        _ => return Err("range expects 1-3 arguments".to_string()),
    };

    if step == 0.0 {
        return Err("range step cannot be zero".to_string());
    }

    let mut result = Vec::new();
    let mut current = start;

    // Handle positive and negative steps
    if step > 0.0 {
        while current < stop {
            result.push(Value::Number(current));
            current += step;
        }
    } else {
        while current > stop {
            result.push(Value::Number(current));
            current += step;
        }
    }

    Ok(Value::Array(result))
}
```

#### Testing Strategy

**Test Coverage**:
```javascript
// Test against recursive versions
let fib_recursive = n => if(n <= 1, n, rec(n-1) + rec(n-2))
let fib_builtin = n => fibonacci(n)

// Verify correctness
assert(fib_builtin(10) == fib_recursive(10))
assert(fib_builtin(20) == fib_recursive(20))

// Verify performance (builtin should be much faster)
let start = now()
fib_builtin(30)
let builtin_time = now() - start

start = now()
fib_recursive(30)
let recursive_time = now() - start

assert(builtin_time < recursive_time / 100)  // 100x faster

// Verify large values work
assert(fibonacci(100) != error())  // Should work
assert(fib_recursive(100) == error())  // Should fail
```

---

### 🟢 Priority 3: Memoization/Caching (MEDIUM-HIGH)
**Timeline**: 2-3 weeks | **Impact**: High for specific cases | **Complexity**: Medium-High

#### Why Third?
- Requires careful design
- Benefits specific patterns (Fibonacci, dynamic programming)
- Needs cache invalidation strategy
- More complex than TCO and built-ins

#### Strategy
Automatic memoization for recursive functions with pure inputs.

#### Phase 3A: Simple Function-Level Memoization (Week 1)

**Approach**: Cache results keyed by function + arguments

**Use Cases**:
- Fibonacci: `fib(5)` called multiple times
- Dynamic programming: overlapping subproblems
- Expensive pure computations

**Implementation**:

```rust
// crates/achronyme-eval/src/memo.rs

use std::collections::HashMap;
use std::hash::{Hash, Hasher};

/// Cache key: function identity + arguments
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct CacheKey {
    function_id: usize,  // Unique ID for each function
    args: Vec<ValueHash>, // Hashable representation of args
}

/// Hashable wrapper for Value
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum ValueHash {
    Number(u64),      // f64 bits
    Boolean(bool),
    String(String),
    // Arrays/Records: hash of contents
    Array(Vec<ValueHash>),
    // Functions: not cacheable
}

/// Global memoization cache
pub struct MemoCache {
    cache: HashMap<CacheKey, Value>,
    enabled: bool,
    max_size: usize,
    hits: usize,
    misses: usize,
}

impl MemoCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            enabled: true,
            max_size: 10_000,  // Configurable
            hits: 0,
            misses: 0,
        }
    }

    pub fn get(&mut self, key: &CacheKey) -> Option<Value> {
        if !self.enabled {
            return None;
        }

        match self.cache.get(key) {
            Some(value) => {
                self.hits += 1;
                Some(value.clone())
            }
            None => {
                self.misses += 1;
                None
            }
        }
    }

    pub fn insert(&mut self, key: CacheKey, value: Value) {
        if !self.enabled {
            return;
        }

        // LRU eviction if cache full
        if self.cache.len() >= self.max_size {
            self.evict_oldest();
        }

        self.cache.insert(key, value);
    }

    pub fn clear(&mut self) {
        self.cache.clear();
        self.hits = 0;
        self.misses = 0;
    }

    pub fn stats(&self) -> (usize, usize, f64) {
        let total = self.hits + self.misses;
        let hit_rate = if total > 0 {
            self.hits as f64 / total as f64
        } else {
            0.0
        };
        (self.hits, self.misses, hit_rate)
    }

    fn evict_oldest(&mut self) {
        // Simple: remove random entry
        // Better: LRU with linked list
        if let Some(key) = self.cache.keys().next().cloned() {
            self.cache.remove(&key);
        }
    }
}
```

**Integration with Evaluator**:

```rust
// In Evaluator
pub struct Evaluator {
    env: Environment,
    constants: ConstantsRegistry,
    functions: FunctionRegistry,
    memo_cache: MemoCache,  // NEW
}

// In apply_lambda()
pub fn apply_lambda(
    evaluator: &mut Evaluator,
    function: &Function,
    args: Vec<Value>,
) -> Result<Value, String> {
    // Check cache first
    let cache_key = CacheKey::new(function.id(), &args);

    if let Some(cached) = evaluator.memo_cache.get(&cache_key) {
        return Ok(cached);
    }

    // Compute result (existing logic)
    let result = compute_function(evaluator, function, args)?;

    // Cache result if function is pure
    if function.is_pure() {
        evaluator.memo_cache.insert(cache_key, result.clone());
    }

    Ok(result)
}
```

**Opt-in Annotation**:

```javascript
// User can opt-in to memoization
let fib = memo(n => if(n <= 1, n, rec(n-1) + rec(n-2)))

// Or automatic for rec functions
let fib = n => if(n <= 1, n, rec(n-1) + rec(n-2))
// ^^ Automatically memoized if contains rec
```

#### Phase 3B: Advanced Memoization (Week 2-3)

**Features**:
1. **LRU Cache**: Proper eviction policy
2. **Cache Statistics**: Hit rate, memory usage
3. **Configurable**: Max size, TTL, per-function limits
4. **Selective Caching**: Only cache expensive operations
5. **Cache Busting**: Clear cache on variable changes

**Configuration API**:

```javascript
// Configure memoization
set_memo_config({
    enabled: true,
    max_size: 10000,
    max_per_function: 1000,
    ttl_seconds: 3600,
    auto_memo_rec: true  // Automatically memoize rec functions
})

// Get statistics
let stats = memo_stats()
// {hits: 1234, misses: 567, hit_rate: 0.685, size: 450}

// Clear cache
clear_memo()
clear_memo_function(fib)  // Clear specific function
```

**Smart Memoization**:

```rust
// Detect patterns that benefit from memoization
fn should_memoize(function: &Function) -> bool {
    // 1. Contains rec (recursive)
    // 2. No side effects (pure)
    // 3. Not tail-recursive (TCO handles those)
    // 4. Small argument space (not infinite)

    function.contains_rec()
        && function.is_pure()
        && !function.is_tail_recursive()
        && function.param_count() <= 3
}
```

#### Performance Impact

**Fibonacci Example**:

| Approach | fib(30) Time | fib(100) |
|----------|--------------|----------|
| Naive recursion | ~2 seconds | Stack overflow |
| With memoization | ~0.001 seconds | ~0.001 seconds |
| Built-in | ~0.0001 seconds | ~0.0001 seconds |

**Speedup**: 1000-2000x for overlapping subproblems

---

## Implementation Priorities Summary

### Phase-by-Phase Timeline

| Priority | Feature | Timeline | Benefit |
|----------|---------|----------|---------|
| 🔴 **P1** | **Tail Call Optimization** | **Weeks 1-2** | Unlimited tail recursion |
| 🟡 **P2A** | **Built-ins: Math** | **Week 3** | Fast fib, factorial, primes |
| 🟡 **P2B** | **Built-ins: Arrays** | **Week 4** | Fast range, flatten, repeat |
| 🟡 **P2C** | **Built-ins: Algorithms** | **Week 5** | Fast sort, search |
| 🟢 **P3A** | **Simple Memoization** | **Week 6** | Auto-cache rec functions |
| 🟢 **P3B** | **Advanced Memo** | **Weeks 7-8** | LRU, stats, config |

**Total Timeline**: 8 weeks (2 months)

---

## Success Metrics

### Before Implementation

| Pattern | Max Depth | Time (n=30) | Status |
|---------|-----------|-------------|--------|
| Factorial (any) | ~50 | N/A | ❌ Limited |
| Fibonacci (naive) | ~50 | ~2s | ❌ Slow + Limited |
| Range(n) | ~50 | N/A | ❌ Limited |
| GCD(a,b) | ~50 | Fast | ❌ Limited |

### After P1 (TCO)

| Pattern | Max Depth | Time (n=30) | Status |
|---------|-----------|-------------|--------|
| Factorial (tail) | ∞ | < 0.001s | ✅ Fixed |
| Fibonacci (naive) | ~50 | ~2s | ⚠️ Still slow |
| Range(n) | ∞ | < 0.001s | ✅ Fixed |
| GCD(a,b) | ∞ | < 0.001s | ✅ Fixed |

### After P2 (Built-ins)

| Pattern | Max Depth | Time (n=30) | Status |
|---------|-----------|-------------|--------|
| Factorial (builtin) | ∞ | < 0.0001s | ✅ Optimized |
| Fibonacci (builtin) | ∞ | < 0.0001s | ✅ Optimized |
| Range(n) | ∞ | < 0.0001s | ✅ Optimized |
| GCD(a,b) | ∞ | < 0.0001s | ✅ Optimized |

### After P3 (Memoization)

| Pattern | Max Depth | Time (n=30) | Status |
|---------|-----------|-------------|--------|
| Factorial (any) | ∞ | < 0.001s | ✅ Auto-cached |
| Fibonacci (recursive+memo) | ∞ | < 0.001s | ✅ Auto-cached |
| Custom DP functions | ∞ | Fast | ✅ Auto-cached |

---

## Documentation Updates Needed

### 1. Update `docs/language/22-recursion.md`

Add sections:
- **TCO Support**: Which patterns get optimized
- **Built-in Alternatives**: Table of recursive → built-in mappings
- **Memoization**: How to use `memo()` and auto-memoization
- **Performance Comparison**: Before/after tables
- **Best Practices**: When to use each approach

### 2. Update `docs/language/25-performance-limitations.md`

Change from:
> Recursion limited to ~50 calls

To:
> - **Tail recursion**: Unlimited (TCO optimized)
> - **Non-tail recursion**: Use built-ins or memoization
> - **Multiple recursion**: Auto-memoized for common patterns

### 3. Create `docs/language/26-memoization.md`

New document covering:
- What is memoization
- When it's applied automatically
- How to use `memo()` function
- Configuration options
- Cache statistics and debugging
- Best practices

### 4. Update `docs/language/23-best-practices.md`

Add:
- Prefer tail-recursive patterns (get TCO)
- Use built-ins for common patterns
- Rely on auto-memoization for custom DP
- How to write memo-friendly functions

---

## Testing Strategy

### Test Suite Structure

```
crates/achronyme-eval/tests/
├── test_tco.rs                    # TCO comprehensive tests
├── test_builtins_sequences.rs     # Fibonacci, factorial, etc.
├── test_builtins_arrays.rs        # Range, repeat, flatten
├── test_builtins_algorithms.rs    # Sort, search
├── test_memoization.rs            # Cache behavior
└── test_recursion_integration.rs  # All optimizations together
```

### Key Test Cases

**1. TCO Tests**:
```javascript
// Deep tail recursion (>1000 calls)
let factorial_tail = n => (
    (current, acc) => if(current <= 1, acc, rec(current-1, acc*current))
)(n, 1)

test("factorial_tail(10000) should work")
test("gcd(10^9, 10^6) should work")
```

**2. Built-in Tests**:
```javascript
// Compare against recursive versions
test("fibonacci(100) == recursive_fib(10)") // Same result, different size
test("factorial(170) works") // Max f64 precision
test("range(10000) works")   // Large ranges
```

**3. Memoization Tests**:
```javascript
// Verify caching behavior
let call_count = 0
let tracked_fib = n => do {
    call_count = call_count + 1
    if(n <= 1, n, rec(n-1) + rec(n-2))
}

tracked_fib(10)
let first_count = call_count

call_count = 0
tracked_fib(10)  // Should be cached
test("call_count < first_count")  // Fewer calls due to cache
```

**4. Performance Tests**:
```javascript
// Benchmark improvements
let start = perf_now()
fibonacci(100)
let builtin_time = perf_now() - start

start = perf_now()
recursive_fib(30)  // Can't even do 100
let recursive_time = perf_now() - start

test("builtin is >1000x faster")
```

---

## Risk Assessment

| Risk | Impact | Mitigation |
|------|--------|------------|
| TCO bugs break existing code | High | Extensive testing, conservative detection |
| Memoization memory leaks | Medium | LRU eviction, max cache size |
| Built-ins have bugs | Medium | Test against recursive versions |
| Performance regressions | Low | Benchmarks, profiling |
| Cache invalidation issues | Medium | Clear cache on environment changes |

---

## Migration Guide for Users

### Before Optimization

```javascript
// Limited to ~50
let factorial = n => if(n <= 1, 1, n * rec(n-1))

// Very slow and limited
let fib = n => if(n <= 1, n, rec(n-1) + rec(n-2))

// Limited to ~50
let range = n => (
    (left, current, vector) =>
        if(left == 0, vector, rec(left-1, current+1, [...vector, current]))
)(n, 0, [])
```

### After Optimization (Recommended Patterns)

```javascript
// Option 1: Convert to tail-recursive (gets TCO)
let factorial = n => (
    (current, acc) => if(current <= 1, acc, rec(current-1, acc*current))
)(n, 1)

// Option 2: Use built-in
let factorial_value = factorial(100)

// Option 3: Keep recursive, gets auto-memoization
let fib = n => if(n <= 1, n, rec(n-1) + rec(n-2))
// ^^ Now fast and unlimited due to memoization

// Option 4: Use built-in for maximum performance
let fib_value = fibonacci(100)

// Option 5: Use built-in range
let numbers = range(0, 1000)  // Much faster than recursive
```

---

## Next Steps

### Immediate Actions

1. ✅ **Review this roadmap** with team
2. ✅ **Create feature branch**: `feature/recursion-optimization`
3. ✅ **Start P1 (TCO)**: Implement detection logic
4. ⏳ **Prepare built-ins**: Design API for common functions
5. ⏳ **Prototype memoization**: Proof of concept

### Week 1 Goals

- [ ] Complete TCO detection module
- [ ] Write TCO tests (tail vs non-tail)
- [ ] Begin iterative loop implementation

### Month 1 Goals

- [ ] TCO fully working and tested
- [ ] 15-20 built-in functions implemented
- [ ] Documentation updated with TCO guide

### Month 2 Goals

- [ ] All built-in functions complete
- [ ] Memoization working with auto-detection
- [ ] Full test coverage
- [ ] Performance benchmarks published

---

**Status**: 📋 Roadmap Complete - Ready for Implementation
**Last Updated**: 2025-11-12
**Estimated Completion**: 8 weeks from start
**Team Size Assumption**: 1-2 developers
