# Tail Call Optimization (TCO) Implementation Plan

## Current State Analysis

### Problem: Stack Overflow at ~50 Recursive Calls

**Limitation**: Achronyme currently crashes with stack overflow at approximately 50-100 recursive calls.

**Root Causes**:
1. **Environment Cloning (O(n))**: Every recursive call clones the entire environment HashMap
2. **Function Cloning**: The function is cloned to bind to `rec` keyword
3. **Scope Management**: Each call pushes/pops scopes
4. **Rust Stack Frames**: Each recursion creates 1-2 KB stack frames
5. **Value Cloning**: Arguments and intermediate values are cloned
6. **Memory Growth**: Patterns like `[...vector, element]` grow quadratically

**Memory per Call**: ~2.5 KB (500 bytes cloning + 2 KB stack frame)
**Total at 50 calls**: ~125 KB
**Rust default stack**: 2 MB (theoretical limit ~800 calls, practical limit ~50-100)

### Current Implementation Location

**File**: `crates/achronyme-eval/src/handlers/functions.rs`
**Function**: `apply_lambda()` (lines 32-107)

**Key bottleneck (line 51)**:
```rust
let saved_env = evaluator.environment().clone();  // EXPENSIVE: O(n) deep clone
```

This clones the entire `HashMap<String, Value>` on every call.

**Recursion mechanism (line 62)**:
```rust
evaluator.environment_mut().define("rec".to_string(), Value::Function(function.clone()))?;
```

The `rec` keyword is injected into every function call's environment.

### What is Tail Call Optimization?

**Tail Call**: A function call that is the **last operation** in a function body.

**Examples**:

```javascript
// ✅ TAIL RECURSIVE (last operation is rec call)
let factorial = n => (
    (current, acc) =>
        if(current <= 1, acc, rec(current - 1, acc * current))
)(n, 1)

// ❌ NOT TAIL RECURSIVE (multiplication happens after rec returns)
let factorial = n =>
    if(n <= 1, 1, n * rec(n - 1))
```

**TCO Benefit**: Converts tail-recursive calls into loops, eliminating stack frame creation.

**Result**: Unlimited recursion depth for tail-recursive patterns.

---

## Implementation Plan

### Phase 1: Tail Call Detection (2-3 days)

**Goal**: Identify which function bodies are tail-recursive at parse/evaluation time.

**Tasks**:
1. Create new module `crates/achronyme-eval/src/tco.rs`
2. Implement `is_tail_recursive(node: &AstNode) -> bool`
3. Implement `extract_tail_call(node: &AstNode) -> Option<Vec<AstNode>>`
4. Add unit tests for detection logic

**Detection Rules**:

| Pattern | Tail Recursive? | Reason |
|---------|----------------|---------|
| `rec(n-1)` | ✅ Yes | Direct call, no operation after |
| `n * rec(n-1)` | ❌ No | Multiplication after rec returns |
| `if(cond, rec(a), rec(b))` | ✅ Yes | Both branches are tail positions |
| `do { let x = 5; rec(x) }` | ✅ Yes | Last statement is tail position |
| `rec(n-1) + 1` | ❌ No | Addition after rec returns |

**Implementation**:
```rust
pub fn is_tail_recursive(node: &AstNode) -> bool {
    match node {
        // Direct rec call
        AstNode::FunctionCall { function, .. } => {
            matches!(**function, AstNode::RecReference)
        }

        // If-expression: both branches must be tail
        AstNode::FunctionCall { function, args } if is_if_call(function) => {
            if args.len() == 3 {
                is_tail_recursive(&args[1]) && is_tail_recursive(&args[2])
            } else {
                false
            }
        }

        // Do block: last statement must be tail
        AstNode::DoBlock(statements) => {
            statements.last().map(is_tail_recursive).unwrap_or(false)
        }

        // Binary/unary ops are NOT tail (operation after call)
        AstNode::BinaryOp { .. } | AstNode::UnaryOp { .. } => false,

        _ => false,
    }
}
```

---

### Phase 2: Iterative Execution Loop (3-5 days)

**Goal**: Modify `apply_lambda()` to detect and execute tail calls iteratively.

**Approach**: When a tail-recursive call is detected, instead of creating a new stack frame, update the parameters and loop.

**Pseudocode**:
```rust
pub fn apply_lambda(
    evaluator: &mut Evaluator,
    function: &Function,
    mut args: Vec<Value>,
) -> Result<Value, String> {
    // Check if function body is tail-recursive
    let is_tail = is_tail_recursive(&body);

    if !is_tail {
        // Use existing recursive implementation
        return apply_lambda_recursive(evaluator, function, args);
    }

    // TCO: Iterative execution
    loop {
        // Set up environment with current args
        setup_environment(evaluator, function, &args);

        // Evaluate body
        let result = evaluator.evaluate(&body)?;

        // Check if result is a tail call
        match extract_tail_call_from_result(&result, &body) {
            Some(new_args) => {
                // Continue loop with new arguments
                args = evaluate_args(evaluator, new_args)?;
                cleanup_environment(evaluator);
                continue;
            }
            None => {
                // No tail call, return result
                cleanup_environment(evaluator);
                return Ok(result);
            }
        }
    }
}
```

**Key Changes**:
1. Check if body is tail-recursive using `is_tail_recursive()`
2. If not, use existing recursive implementation (backward compatible)
3. If yes, enter iterative loop
4. Evaluate body, check if result triggers tail call
5. If tail call, update args and continue loop
6. If no tail call, return result

**Files to Modify**:
- `crates/achronyme-eval/src/handlers/functions.rs` - Add TCO logic to `apply_lambda()`
- `crates/achronyme-eval/src/tco.rs` - Detection and extraction logic
- `crates/achronyme-eval/src/lib.rs` - Export new module

---

### Phase 3: Environment Optimization (1-2 days)

**Goal**: Reduce environment cloning overhead in the TCO loop.

**Current Problem**: Line 51 clones entire environment on every call.

**Solution**: In TCO mode, reuse the same environment and only update parameter bindings.

**Approach**:
1. Add `update_binding()` method to `Environment`
2. In TCO loop, update parameter values in place instead of cloning
3. Only clone environment for non-tail-recursive functions

**Implementation**:
```rust
// In crates/achronyme-types/src/environment.rs
impl Environment {
    /// Update an existing variable in the current scope
    /// Used by TCO to update parameters without cloning environment
    pub fn update_in_current_scope(&mut self, name: &str, value: Value) -> Result<(), String> {
        if self.variables.contains_key(name) {
            self.variables.insert(name.to_string(), value);
            Ok(())
        } else {
            Err(format!("Variable '{}' not found in current scope", name))
        }
    }
}
```

**Expected Impact**: Reduce per-call overhead from O(n) to O(1) for parameter updates.

---

### Phase 4: Testing (2-3 days)

**Goal**: Ensure TCO works correctly and doesn't break existing code.

**Test Cases**:

1. **Deep Tail Recursion (Previously Failed)**:
```javascript
let factorial = n => (
    (current, acc) =>
        if(current <= 1, acc, rec(current - 1, acc * current))
)(n, 1)

factorial(100)   // ✅ Should work with TCO
factorial(1000)  // ✅ Should work with TCO
factorial(10000) // ✅ Should work with TCO
```

2. **Non-Tail Recursion (Existing Behavior)**:
```javascript
let factorial = n =>
    if(n <= 1, 1, n * rec(n - 1))

factorial(10)  // ✅ Should still work
factorial(50)  // ❌ Should still fail (not tail-recursive)
```

3. **Fibonacci (Non-Tail, Multiple Recursion)**:
```javascript
let fib = n =>
    if(n <= 1, n, rec(n - 1) + rec(n - 2))

fib(10)  // ✅ Should work
fib(50)  // ❌ Should still fail (not tail-recursive, exponential)
```

4. **GCD (Tail Recursive)**:
```javascript
let gcd = (a, b) =>
    if(b == 0, a, rec(b, a % b))

gcd(1000000, 123456)  // ✅ Should work with TCO
```

5. **Range Generation (Tail Recursive with Accumulator)**:
```javascript
let range = n => (
    (left, current, vector) =>
        if(left == 0, vector, rec(left - 1, current + 1, [...vector, current]))
)(n, 0, [])

range(100)   // ✅ Should work with TCO
range(1000)  // ✅ Should work with TCO (limited by memory, not stack)
```

6. **Mutual Recursion (Not Supported Yet)**:
```javascript
// This pattern is difficult and not prioritized
```

**Test File**: `crates/achronyme-eval/tests/test_tco.rs`

**Existing Tests**: Ensure all tests in `tests/test_recursion_and_self.rs` still pass.

---

### Phase 5: Documentation Update (1 day)

**Files to Update**:

1. **`docs/language/22-recursion.md`**:
   - Add section on TCO
   - Explain which patterns benefit from TCO
   - Update examples to show unlimited depth for tail recursion
   - Add performance comparison table

2. **`docs/language/25-performance-limitations.md`**:
   - Update limitation from "~50 calls" to "~50 for non-tail, unlimited for tail"
   - Add TCO explanation
   - Update recommendations

3. **`docs/language/23-best-practices.md`**:
   - Add recommendation to use tail-recursive patterns
   - Show how to convert regular recursion to tail recursion
   - Add examples of accumulator pattern

**Example Addition to 22-recursion.md**:
```markdown
## Tail Call Optimization (NEW)

Achronyme now optimizes tail-recursive functions, allowing unlimited recursion depth.

### What Gets Optimized

✅ **Optimized (Unlimited Depth)**:
- Tail-recursive calls (last operation is rec)
- Tail calls in if/piecewise branches
- Tail calls in do blocks

❌ **Not Optimized (Still Limited to ~50)**:
- Non-tail recursion (operations after rec)
- Multiple recursion (Fibonacci-style)

### Converting to Tail Recursion

**Before (Limited to ~50 calls)**:
```javascript
let factorial = n =>
    if(n <= 1, 1, n * rec(n - 1))
```

**After (Unlimited depth)**:
```javascript
let factorial = n => (
    (current, acc) =>
        if(current <= 1, acc, rec(current - 1, acc * current))
)(n, 1)

factorial(10000)  // ✅ Works!
```
```

---

## Implementation Timeline

| Phase | Duration | Description |
|-------|----------|-------------|
| **Phase 1** | 2-3 days | Tail call detection logic |
| **Phase 2** | 3-5 days | Iterative execution loop |
| **Phase 3** | 1-2 days | Environment optimization |
| **Phase 4** | 2-3 days | Comprehensive testing |
| **Phase 5** | 1 day | Documentation updates |
| **Total** | **9-14 days** | Full TCO implementation |

---

## Technical Challenges

### Challenge 1: Detecting Tail Position

**Problem**: Need to analyze AST to determine if a call is in tail position.

**Solution**: Recursive AST traversal with tail position tracking.

**Complexity**: Medium - requires careful analysis of all AST node types.

### Challenge 2: Extracting Next Arguments

**Problem**: When a tail call is detected, need to extract the new arguments.

**Solution**: Evaluate the rec call's arguments and use them for next iteration.

**Complexity**: Medium - requires evaluating AST nodes to get values.

### Challenge 3: Environment Management

**Problem**: Need to update parameter bindings without full clone.

**Solution**: Add `update_in_current_scope()` method to Environment.

**Complexity**: Low - straightforward implementation.

### Challenge 4: Backward Compatibility

**Problem**: Must not break existing non-tail-recursive code.

**Solution**: Only apply TCO when tail recursion is detected, otherwise use existing path.

**Complexity**: Low - if/else branch in `apply_lambda()`.

### Challenge 5: Complex Control Flow

**Problem**: Tail position in if/piecewise/do blocks requires special handling.

**Solution**: Recursively check each branch for tail calls.

**Complexity**: Medium - requires careful AST analysis.

---

## Alternative Approaches Considered

### Alternative 1: Trampoline Pattern

**Approach**: Return continuation functions instead of values.

**Pros**:
- Simpler to implement
- Works with any recursion pattern

**Cons**:
- Requires changing return type of all functions
- Breaking change to API
- Performance overhead for non-recursive functions

**Decision**: ❌ Rejected - too invasive.

### Alternative 2: Increase Stack Size

**Approach**: Configure Rust to use larger stack (8 MB instead of 2 MB).

**Pros**:
- Simple one-line change
- Works for all recursion patterns

**Cons**:
- Only increases limit to ~200 calls (not unlimited)
- Wastes memory for non-recursive code
- Doesn't solve fundamental problem

**Decision**: ❌ Rejected - not a real solution.

### Alternative 3: CPS (Continuation Passing Style)

**Approach**: Transform all functions to CPS at parse time.

**Pros**:
- Eliminates all recursion
- Works with any pattern

**Cons**:
- Very complex implementation
- Requires complete rewrite of evaluator
- Performance overhead for all code

**Decision**: ❌ Rejected - too complex.

### Alternative 4: Iterative TCO (CHOSEN)

**Approach**: Detect tail calls and convert to loops at runtime.

**Pros**:
- Backward compatible
- Only applies to tail-recursive functions
- Unlimited depth for tail recursion
- No overhead for non-recursive code

**Cons**:
- Requires tail position detection
- Only helps tail-recursive patterns

**Decision**: ✅ **CHOSEN** - best balance of complexity and benefit.

---

## Success Criteria

### Must Have
- ✅ Tail-recursive functions work with unlimited depth
- ✅ Non-tail-recursive functions still work (backward compatible)
- ✅ All existing tests pass
- ✅ Factorial(10000) works with tail-recursive version
- ✅ GCD with large numbers works

### Should Have
- ✅ Documentation updated
- ✅ Performance tests showing improvement
- ✅ Examples in docs demonstrating TCO

### Nice to Have
- ✅ Warning when non-tail recursion detected
- ✅ Automatic conversion suggestions
- ✅ TCO statistics in debug mode

---

## Performance Expectations

### Before TCO

| Function | Max Depth | Reason |
|----------|-----------|--------|
| Factorial (regular) | ~50 | Stack overflow |
| Factorial (tail) | ~50 | Stack overflow |
| GCD | ~50 | Stack overflow |
| Range generation | ~50 | Stack overflow |

### After TCO

| Function | Max Depth | Reason |
|----------|-----------|--------|
| Factorial (regular) | ~50 | Not tail-recursive (still limited) |
| Factorial (tail) | **Unlimited** | TCO applies |
| GCD | **Unlimited** | TCO applies |
| Range generation | **Unlimited** | TCO applies (memory limit still applies) |

**Expected Speedup**: 2-5x for tail-recursive patterns even at low depths (due to reduced cloning).

---

## Risk Assessment

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Breaking existing code | Low | High | Extensive testing, fallback to old behavior |
| Performance regression | Low | Medium | Benchmark suite, only apply TCO when beneficial |
| Incorrect tail detection | Medium | High | Comprehensive test suite, conservative detection |
| Memory leaks | Low | Medium | Careful environment management, testing |
| Complex edge cases | Medium | Medium | Thorough AST analysis, gradual rollout |

---

## Next Steps

1. **Review this plan** with team/stakeholders
2. **Create feature branch**: `feature/tail-call-optimization`
3. **Implement Phase 1**: Detection logic
4. **Write tests** for detection
5. **Implement Phase 2**: Iterative execution
6. **Benchmark** performance improvements
7. **Update documentation**
8. **Merge to main** after thorough testing

---

## References

- **Current Implementation**: `crates/achronyme-eval/src/handlers/functions.rs`
- **Environment Code**: `crates/achronyme-types/src/environment.rs`
- **Existing Tests**: `crates/achronyme-eval/tests/test_recursion_and_self.rs`
- **Documentation**: `docs/language/22-recursion.md`
- **Performance Docs**: `docs/language/25-performance-limitations.md`

---

**Status**: 📋 Planning Complete - Ready for Implementation
**Last Updated**: 2025-11-12
**Author**: Achronyme Core Team
