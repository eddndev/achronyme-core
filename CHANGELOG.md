# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added - Complex Numbers & Complex Vectors 🔢✨

**Complex Number System Enhanced:**

- **Complex^Complex Power Operation**
  - Full support for complex exponents: `z^w` where both base and exponent are complex
  - Implementation using formula: `a^b = e^(b * ln(a))`
  - Examples:
    ```javascript
    i^i           // → e^(-π/2) ≈ 0.2079
    (1+i)^2       // → 0 + 2i
    2^i           // → cos(ln(2)) + i*sin(ln(2))
    (2i)^2        // → -4
    (1+i)^(2+i)   // → complex result
    ```

- **Imaginary Unit Constant `i`**
  - Built-in constant `i` = `0+1i` (imaginary unit)
  - Available globally without declaration
  - Case-insensitive: `i` or `I`
  - Automatic type promotion in expressions
  - Example: `let z = 3 + 4i`

**New `ComplexVector` Type:**

- **Native Complex Vector Support**
  - First-class type for vectors of complex numbers
  - Automatic type detection from literals
  - Internal storage: interleaved format `[re0, im0, re1, im1, ...]`
  - Examples:
    ```javascript
    [i, 2+3i, 4]              // → ComplexVector
    [0+1i, 2+3i, 4+0i]       // Promoted to complex
    ```

- **Type Promotion System**
  - Vector literals auto-detect complex elements
  - Real numbers promoted to complex in mixed contexts
  - `[1, 2+i]` → `[1+0i, 2+i]` (ComplexVector)
  - Consistent behavior across all operations

- **Vector Operations**
  - Element-wise arithmetic: `+`, `-`, `*`, `/`
  - Power operations: `[1+i, 2+i]^2`
  - Examples:
    ```javascript
    let v1 = [1+2i, 3+4i]
    let v2 = [5+6i, 7+8i]
    v1 + v2  // → [6+8i, 10+12i]
    v1 * v2  // → [element-wise multiplication]
    ```

**Complex Functions Extended for Vectors:**

- **`real(z)` - Extract Real Part**
  - Works with: number, complex number, or ComplexVector
  - `real(3+4i)` → `3`
  - `real([1+i, 2+3i])` → `[1, 2]` (Vector)

- **`imag(z)` - Extract Imaginary Part**
  - Works with: number, complex number, or ComplexVector
  - `imag(3+4i)` → `4`
  - `imag([1+i, 2+3i])` → `[1, 3]` (Vector)

- **`conj(z)` - Complex Conjugate**
  - Works with: number, complex number, or ComplexVector
  - `conj(3+4i)` → `3-4i`
  - `conj([1+i, 2+3i])` → `[1-i, 2-3i]` (ComplexVector)

- **`arg(z)` - Phase/Argument**
  - Returns phase angle in radians
  - `arg(1+i)` → `π/4`

**Higher-Order Functions with Complex Support:**

- **`map()` Enhanced**
  - Now accepts both `Vector` and `ComplexVector`
  - Automatic type promotion based on inputs/outputs
  - Returns ComplexVector if any input or result is complex
  - Examples:
    ```javascript
    let v = [2+i, 3+i]
    map(z => z^2, v)           // → [3+4i, 8+6i]

    // With closure capturing
    let offset = 1+i
    map(z => z + offset, v)    // → [3+2i, 4+2i]
    ```

**DSP Functions Modernized:**

- **`fft(signal)` - Fast Fourier Transform**
  - **NEW**: Returns `ComplexVector` (was: Matrix [N×2])
  - Much cleaner API - no more matrix manipulation
  - Direct access to complex spectrum
  - Example:
    ```javascript
    let signal = [0, 0.707, 1, 0.707, 0, -0.707, -1, -0.707]
    let spectrum = fft(signal)           // ComplexVector
    let real_part = real(spectrum)       // Extract reals
    let imag_part = imag(spectrum)       // Extract imaginaries
    ```

- **`ifft(spectrum)` - Inverse FFT**
  - **NEW**: Accepts `ComplexVector` (was: Matrix [N×2])
  - Perfect roundtrip: `signal → fft → ifft → signal`
  - Example:
    ```javascript
    let spectrum = fft(signal)
    let recovered = ifft(spectrum)       // Recovers original
    ```

- **FFT Analysis Workflow**
  ```javascript
  // Modern workflow with ComplexVector
  let spectrum = fft(signal)
  let magnitudes = fft_mag(signal)      // Or: map(z => abs(z), spectrum)
  let phases = fft_phase(signal)

  // Process spectrum
  let filtered = map(z => z * filter, spectrum)
  let result = ifft(filtered)

  // Extract components
  let real_parts = real(spectrum)
  let imag_parts = imag(spectrum)
  ```

**WASM & TypeScript SDK:**

- **New WASM Bindings**
  - `createComplexVector(data)` - Create from interleaved `[re, im, ...]` format
  - `getComplexVector(handle)` - Extract to interleaved format
  - Efficient memory layout for JavaScript interop

- **TypeScript `ComplexVector` Class**
  - `.get(index)` → `{re: number, im: number}`
  - `.toComplexArray()` → `Array<{re, im}>`
  - `.toArray()` → `number[]` (interleaved)
  - Iterator support: `for (const z of complexVec)`
  - `.length` property
  - `.toString()` for display

**Closure Support Validated:**

- **Lambda Closures Work Correctly** ✓
  - Lambdas can capture variables from outer scope
  - Example:
    ```javascript
    let multiplier = 2
    map(x => x * multiplier, [1, 2, 3])  // → [2, 4, 6]

    let offset = 1+i
    let shift = (z) => z + offset
    shift(2+i)  // → 3+2i
    ```

- **Piecewise with Complex Values** ✓
  - Piecewise functions support complex return values
  - Example:
    ```javascript
    piecewise([x > 0, 1+i], [x < 0, -1+i], 0+0i)
    ```

**Implementation Details:**

- Complex vector storage: interleaved `[re0, im0, re1, im1, ...]`
- Efficient memory layout for WASM/JavaScript interop
- Full integration with existing binary operations
- CLI display format: `[a+bi, c+di]`
- Automatic negative sign handling: `3-4i` (not `3+-4i`)

### Added - Variable Shadowing & Scope System 🔄

**Stack-Based Scope System:**

- ✅ **Variable Shadowing in Lambdas**
  - Lambda parameters can now shadow outer variables
  - Example:
    ```javascript
    let z = 10
    let f = (z) => z * 2
    f(3)  // → 6 (uses parameter z=3, not outer z=10)
    ```

- ✅ **Let Redeclaration (Shadowing in Same Scope)**
  - Variables can be redeclared with `let` to shadow previous definitions
  - Enables clean pipeline transformations without memory duplication
  - Example:
    ```javascript
    let v = [1,2,3,4]
    let v = map(x => x^2, v)   // [1,4,9,16]
    let v = map(x => x+1, v)   // [2,5,10,17]
    ```

- ✅ **Closures Preserved**
  - Closures continue to capture variables correctly
  - Each closure captures the value at creation time
  - Example:
    ```javascript
    let x = 5
    let f1 = (y) => x + y  // Captures x=5

    let x = 10             // Shadow x
    let f2 = (y) => x + y  // Captures x=10

    f1(1)  // → 6  (uses captured x=5)
    f2(1)  // → 11 (uses captured x=10)
    ```

**Implementation Details:**

- Environment uses stack-based scopes (Vec<HashMap>)
- Lambda parameters create new scope automatically
- Variable lookup: innermost → outermost scope
- Shadowing does not modify outer scope values
- 131 tests passing, including 11 new shadowing tests

**Benefits:**

1. **Memory Efficiency**: Reuse variable names without duplicating large vectors
2. **Clean Pipelines**: Natural data transformation workflows
3. **Functional Programming**: Standard shadowing semantics
4. **Better Ergonomics**: No need for `v2`, `v3`, `v_final` naming

### Refactoring - Evaluator Architecture 🏗️

**Code Organization Improvements:**

- Evaluator reduced from 1179 → 203 lines (83% reduction)
- Logic separated into specialized handlers:
  - `handlers/literals.rs` (94 lines): Number, boolean, complex, vector, matrix literals
  - `handlers/variables.rs` (42 lines): Variable declaration and reference
  - `handlers/control_flow.rs` (60 lines): If expressions, piecewise functions
  - `handlers/functions.rs` (66 lines): Lambda evaluation and application
- Tests moved to `tests/test_evaluator.rs` (744 lines)
- Cleaner, more maintainable codebase
- Easier to extend with new features

### Known Limitations

**Variable Scoping:**

- **No Variable Reassignment** (by design)
  - Variables cannot be reassigned without `let`
  - Example (not supported):
    ```javascript
    let x = 5
    x = 10  // Error: reassignment not supported
    ```
  - Use shadowing instead:
    ```javascript
    let x = 5
    let x = 10  // OK: shadowing with new let
    ```

**Note**: Variable reassignment (mutation without `let`) is intentionally not supported. Achronyme is designed as a functional computation engine where data transformations are expressed through shadowing, not mutation. This design choice:
- Simplifies reasoning about code
- Prevents accidental side effects
- Aligns with mathematical notation (definitions, not mutations)
- Enables future optimizations (immutable data structures)

### Planned Features (Phase 5+)
- Symbolic computation
- Units and dimensions
- Ordinary Differential Equations (ODEs)
- Nonlinear optimization (gradient descent, conjugate gradient, BFGS)
- Constrained optimization (SQP, barrier methods)

## [0.5.3] - 2025-11-06

### Added - Conditional Expressions & Piecewise Functions 🎯

**Boolean Logic & Conditionals:**

- **Boolean Type System**
  - New `Value::Boolean(bool)` variant in type system
  - Boolean literals: `true`, `false`
  - Automatic type conversion: `Number → Boolean` (0 = false, non-zero = true)

- **Logical Operators**
  - AND operator: `&&` with short-circuit evaluation
  - OR operator: `||` with short-circuit evaluation
  - NOT operator: `!` for boolean negation
  - Operator precedence: `!` > `&&` > `||`

- **Comparison Operators (Enhanced)**
  - Now return `Boolean` type instead of `Number`
  - Operators: `>`, `<`, `>=`, `<=`, `==`, `!=`
  - Example: `5 > 3` → `true` (not `1.0`)

- **`if()` Function - Conditional Expression**
  - Syntax: `if(condition, then_value, else_value)`
  - Short-circuit evaluation (only evaluates chosen branch)
  - Works with any expression type
  - Examples:
    ```javascript
    if(x > 0, x, -x)              // Absolute value
    if(x > 0, x, 0)               // ReLU activation
    if(x > 10, 2, 1)              // Conditional logic
    ```

**Piecewise Functions:**

- **`piecewise()` Function - Multi-Branch Conditionals**
  - Syntax: `piecewise([cond1, val1], [cond2, val2], ..., default)`
  - Sequential evaluation with short-circuit
  - Optional default value (last argument without `[]`)
  - Error if no condition matches and no default provided
  - Full support for multivariable lambdas

  - **Mathematical Applications:**
    ```javascript
    // Sign function
    let signo = x => piecewise([x < 0, -1], [x > 0, 1], 0)

    // Progressive tax brackets
    let tax = income => piecewise(
      [income <= 10000, income * 0.1],
      [income <= 50000, income * 0.2],
      income * 0.3
    )

    // Piecewise polynomial
    let f = x => piecewise([x < -1, x^2], [x < 1, 2*x + 1], x^3)

    // Heaviside step function
    let H = x => piecewise([x < 0, 0], 1)
    ```

**DSP & Signal Processing Integration:**

- **Classic Waveforms Implemented:**
  - Square Wave (FFT → odd harmonics)
  - Sawtooth Wave, Triangle Wave
  - Rectangular Pulse (FFT → sinc pattern)
  - Half-Wave & Full-Wave Rectifiers
  - Pulse Train (digital signals)

- **Numerical Analysis Validation:**
  - ✅ **FFT**: Spectral analysis of discontinuous signals
  - ✅ **Differentiation**: Correct derivatives at discontinuities
  - ✅ **Integration**: `trapz(relu, -1, 2, 100)` = 2.0 (exact)

**Testing & Validation:**

- **Unit Tests:** 92 total (20 new for conditionals/piecewise), all passing ✅
- **DSP Integration:** Square wave FFT, rectifiers, numerical analysis ✅
- **CLI Validation:** All example files execute correctly ✅

**Example Files:**

- `examples/soc/15-conditionals.soc` - Boolean logic and `if()` expressions
- `examples/soc/16-piecewise.soc` - 14 piecewise function examples
- `examples/soc/17-piecewise-analysis.soc` - DSP + numerical analysis integration

**Use Cases Unlocked:**

1. **Mathematical Modeling:** Discontinuous functions, piecewise polynomials
2. **Digital Signal Processing:** Waveform generation, rectification, FFT analysis
3. **Machine Learning:** Activation functions (ReLU, Leaky ReLU)
4. **Economics:** Progressive tax systems, tiered pricing
5. **Physics & Engineering:** Boundary conditions, control systems

### Changed

- **Comparison Operators:** Now return `Value::Boolean` instead of `Value::Number`
- **Parser Cleanup:** Removed legacy hand-written parser (lexer.rs, parser.rs, token.rs)

### Breaking Changes

- Comparison operators return `Boolean` instead of `Number` (1.0/0.0)
- Migration: `filter()` automatically handles both boolean and numeric predicates

### Performance

- Boolean operations: <1μs
- `if()` evaluation: <2μs (short-circuit)
- `piecewise()`: <5μs for 3 cases
- FFT of piecewise signals: ~1ms for 32 samples

## [0.5.3] - 2025-11-06

### Added - Parser Migration to Pest 🦀

**Major Infrastructure Upgrade:**

- **Migrated Parser from Hand-Written to Pest (PEG Parser Generator)**
  - Replaced manual recursive descent parser with declarative Pest grammar
  - ~150 lines of clean PEG grammar vs ~1000+ lines of hand-written code
  - Better error messages with precise location information
  - Easier to extend with new language features (conditionals, loops, pattern matching)
  - More robust handling of precedence and associativity

- **New Parser Module: `pest_parser.rs`**
  - Complete SOC language grammar in `grammar.pest`
  - Automatic tokenization and parsing via Pest macros
  - Clean AST generation from Pest pairs
  - Operator precedence correctly handled:
    - Power (right-associative): `2^3^2 = 2^(3^2) = 512`
    - Multiplicative: `*`, `/`, `%` (left-associative)
    - Additive: `+`, `-` (left-associative)
    - Comparison: `>`, `<`, `>=`, `<=`, `==`, `!=`

- **New Evaluator API: `eval_str()`**
  - Direct string-to-value evaluation using Pest parser
  - Replaces `Lexer → Parser → Evaluator` chain with single call
  - Example: `evaluator.eval_str("2 + 3 * 4")` → `Value::Number(14.0)`
  - Supports all SOC features: lambdas, vectors, matrices, function calls, let statements

- **Comprehensive Test Suite**
  - 8 parser-specific tests (all passing)
  - 12 evaluator integration tests (all passing)
  - Tests for comments, multi-line scripts, and SOC-style code
  - Validation against real `examples/soc/` files

### Documentation - Linear Programming Standard Form 📋

**Important Conventions for Optimization Functions:**

All LP solvers (`linprog`, `simplex`, `dual_simplex`, `revised_simplex`) use **standard form**:

```
maximize/minimize z = c^T × x
subject to: Ax ≤ b, x ≥ 0
```

**Key Points:**
- ALL constraints must be in `Ax ≤ b` form (less-than-or-equal)
- ALL values in vector `b` must be non-negative (b ≥ 0)
- User is responsible for converting problems to standard form

**Conversion Examples:**

| Constraint Type | Original | Standard Form |
|----------------|----------|---------------|
| Greater-than | `x₁ + x₂ ≥ 5` | `-x₁ - x₂ ≤ -5` |
| Equality | `x₁ + x₂ = 5` | Two constraints: `x₁ + x₂ ≤ 5` AND `-x₁ - x₂ ≤ -5` |
| Negative RHS | `x₁ ≤ -3` | Multiply by -1: `-x₁ ≤ 3` |

**Why This Design:**
- Keeps the language syntax simple and mathematical
- Follows standard LP textbook conventions
- User maintains full control over problem formulation
- Avoids domain-specific syntax complexity

**Future Consideration:**
Mixed constraint types (≤, ≥, =) may be supported in future versions, but the current design philosophy favors simplicity and mathematical clarity.

**Grammar Features Implemented:**
```pest
// Expressions with correct precedence
expr = { comparison }
comparison = { additive ~ (cmp_op ~ additive)? }
additive = { multiplicative ~ (add_op ~ multiplicative)* }
multiplicative = { unary ~ (mult_op ~ unary)* }
power = { primary ~ ("^" ~ power)? }  // Right-associative

// Complex literals
vector = { "[" ~ expr ~ ("," ~ expr)* ~ "]" }
matrix = { "[" ~ vector ~ ("," ~ vector)* ~ "]" }
complex = { number ~ "i" }

// Lambdas and functions
lambda = { lambda_params ~ "=>" ~ expr }
function_call = { identifier ~ "(" ~ (expr ~ ("," ~ expr)*)? ~ ")" }

// Statements
let_statement = { "let" ~ identifier ~ "=" ~ expr }
program = { SOI ~ statement* ~ EOI }
```

**Benefits of Pest Migration:**
1. **Maintainability**: Grammar is declarative and self-documenting
2. **Extensibility**: Easy to add new features (see `docs/roadmap.md` for planned conditionals)
3. **Robustness**: Pest handles edge cases and ambiguities automatically
4. **Error Handling**: Built-in error messages with line/column information
5. **Performance**: Pest is highly optimized with zero-copy parsing

**Backward Compatibility:**
- Old hand-written parser still available (not removed yet)
- All existing tests pass
- No breaking changes to public API
- Gradual migration path for downstream code

### Added - Linear Programming & Optimization Module 📊

**Complete Linear Programming Suite:**

- **New Rust Crate: `achronyme-solver`**
  - Pure Rust implementation of optimization algorithms
  - Modular architecture with separate solver methods
  - Zero dependencies on evaluator (clean separation of concerns)

- **Simplex Method Variants (5 implementations):**
  - **`simplex(c, A, b, sense)`** - Standard primal simplex algorithm
  - **`linprog(c, A, b, sense)`** - Auto-selection wrapper (chooses best method)
  - **`dual_simplex(c, A, b, sense)`** - Dual simplex for sensitivity analysis
  - **`two_phase_simplex(c, A, b, sense)`** - Handles equality constraints, ≥ constraints, negative RHS
  - **`revised_simplex(c, A, b, sense)`** - Memory-efficient for large problems (n > 1000)

  *Parameters:*
  - `c`: Objective coefficients vector
  - `A`: Constraint matrix (m × n)
  - `b`: Right-hand side vector
  - `sense`: 1 for maximize, -1 for minimize

- **Objective Value Calculation:**
  - **`objective_value(c, x)`** - Computes c·x for solution verification

- **Sensitivity Analysis (3 functions):**
  - **`shadow_price(c, A, b, sense)`** - Returns dual variables (marginal resource values)
    - Interpretation: How much objective improves per unit increase in each constraint
    - Zero price indicates non-binding constraint (resource surplus)

  - **`sensitivity_c(c, A, b, index)`** - Returns range [c_min, c_max] for coefficient c[index]
    - Within range: Optimal solution structure remains unchanged
    - Outside range: May need to recompute optimal solution

  - **`sensitivity_b(c, A, b, index)`** - Returns range [b_min, b_max] for constraint b[index]
    - Within range: Shadow price remains valid
    - Change in objective: Δz* = shadow_price[i] × Δb[i]

- **Integer Programming (Branch & Bound):**
  - **`intlinprog(c, A, b, sense, integer_vars)`** - Solves LP with integer constraints
    - Uses Branch & Bound algorithm with LP relaxations
    - Supports mixed-integer problems (some variables integer, others continuous)
    - Optimal branching variable selection (most fractional heuristic)
    - Efficient pruning based on LP bounds

  - **`binary_linprog(c, A, b, sense, binary_vars)`** - Solves 0-1 Integer Programming
    - Specialized for binary variables (xᵢ ∈ {0, 1})
    - Optimized Branch & Bound for binary decisions
    - Applications: Knapsack, Set Covering, Assignment, Capital Budgeting

  *Algorithm Features:*
  - **Fixed Variable Substitution:** Automatically handles variables with equal bounds
  - **Binary-Aware Branching:** Direct branching on 0/1 for binary variables
  - **Smart Pruning:** Eliminates suboptimal branches early
  - **Iteration Limit:** 50,000 iterations (prevents infinite loops)

  *Common Applications:*
  - **0-1 Knapsack Problem:** Select items to maximize value within weight limit
  - **Capital Budgeting:** Choose projects to maximize NPV within budget
  - **Production Planning:** Integer units of products
  - **Set Covering/Packing:** Minimum cost coverage, maximum non-overlapping sets
  - **Assignment Problems:** Match workers to tasks optimally
  - **Facility Location:** Binary decisions on opening facilities

**Architecture Improvements:**

- **Modular Solver Structure:**
  ```
  achronyme-solver/
  ├── src/
  │   ├── lib.rs              # Public API and re-exports
  │   ├── linear/
  │   │   ├── mod.rs           # Module organization
  │   │   ├── tableau.rs       # Core Tableau structure with pivot operations
  │   │   ├── simplex.rs       # Primal simplex implementation
  │   │   ├── linprog.rs       # Auto-selection logic
  │   │   ├── dual_simplex.rs  # Dual simplex algorithm
  │   │   ├── two_phase.rs     # Two-phase simplex for difficult problems
  │   │   ├── revised_simplex.rs  # Memory-efficient variant
  │   │   └── sensitivity.rs   # Sensitivity analysis functions
  │   └── integer/
  │       ├── mod.rs           # Integer programming module
  │       └── branch_bound.rs  # Branch & Bound implementation (~650 lines)
  ```

- **Handler Architecture Integration:**
  - New `handlers/optimization.rs` module (450+ lines)
  - Central dispatcher pattern in `handlers/function_call.rs`
  - Clean separation: evaluator → dispatcher → handlers → solvers

**WASM Bindings:**

- **11 optimization function exports** in `achronyme-wasm`:
  - Linear Programming: `simplex`, `linprog`, `dualSimplex`, `twoPhaseSimplex`, `revisedSimplex`
  - Sensitivity Analysis: `objectiveValue`, `shadowPrice`, `sensitivityC`, `sensitivityB`
  - Integer Programming: `intlinprog`, `binaryLinprog` ⭐ **NEW**
- All functions use handle-based API for efficient memory management
- Proper error propagation from Rust to JavaScript
- Compiled WASM binary: 1.06 MB (optimized with wasm-opt)

**TypeScript SDK:**

- **New `OptimizationOps` module** with comprehensive documentation:
  - Complete JSDoc for all **11 functions** (9 LP + 2 IP ⭐ **NEW**)
  - Economic interpretation of sensitivity analysis
  - Real-world examples: production planning, knapsack, capital budgeting
  - Type-safe handle-based API
  - Integer Programming examples with 7+ documented applications

- **Integration with Main SDK:**
  - New `ach.optimization` namespace
  - Consistent API with existing modules (math, dsp, linalg, numerical)
  - Automatic memory cleanup with session-based resource management

**Testing & Validation:**

- **Rust Unit Tests:**
  - **30 total tests** in `achronyme-solver` (22 LP + 8 IP ⭐ **NEW**)
  - **28/30 passing** (2 edge cases for dual/two-phase documented)
  - Tests for: simplex, tableau operations, sensitivity analysis, integer programming
  - Validated against known optimal solutions

- **Integer Programming Tests (8 tests, 100% passing):**
  - `test_intlinprog_simple` - Basic integer LP (z* = 12)
  - `test_binary_linprog_knapsack` - Classic knapsack (z* = 220)
  - `test_knapsack_small_capacity` - Tight capacity (z* = 5)
  - `test_knapsack_large_instance` - 5 items (z* = 100)
  - `test_knapsack_all_items_fit` - All items feasible (z* = 30)
  - `test_knapsack_one_item_only` - Single item selection (z* = 300)
  - `test_knapsack_tight_capacity` - Tight constraint (z ≥ 35)
  - `test_intlinprog_multiple_constraints` - 2 constraints (z* = 17)

- **SOC Script Tests (6 test files):**
  ```
  examples/soc/
  ├── 07-optimization-phase1.soc           # Basic simplex (z* = 36)
  ├── 08-production-problem.soc           # Production planning (profit = 2500)
  ├── 10-sensitivity-analysis.soc         # Shadow prices [10, 0, 30]
  ├── 11-sensitivity-c-test.soc           # Coefficient range [20, 80]
  ├── 12-sensitivity-b-test.soc           # RHS range [35, 105]
  └── 09-comprehensive-optimization.soc   # All methods comparison
  ```
  - ✅ All tests passing with correct results
  - ✅ Validates simplex, revised simplex, sensitivity analysis

- **Interactive Demo Tests (15 total):**
  - **Linear Programming (10 tests):**
    - Simple LP, Production Planning, All Simplex Methods
    - Shadow Prices, Sensitivity Analysis (c & b)
    - Full Analysis with all features
  - **Integer Programming (5 tests, 100% passing):** ⭐ **NEW**
    - `opt-integer-simple` - Integer LP (x=[4,0], z*=12, ~0.4ms)
    - `opt-knapsack-01` - 0-1 Knapsack (items 2,3 selected, z*=220, ~0.3ms)
    - `opt-knapsack-large` - 5-item knapsack (z*=100, ~0.4ms)
    - `opt-integer-production` - Integer units (40A+30B=$2500, ~0.2ms)
    - `opt-capital-budgeting` - Project selection (Projects 2,4, NPV=$400M, ~0.2ms)

- **Package Validation:**
  - ✅ `npm pack --dry-run` successful (428.4 kB compressed)
  - ✅ OptimizationOps included in distribution (enhanced with IP methods)
  - ✅ 59 total files ready for publication
  - ✅ WASM binary optimized and verified

**Documentation:**

- Complete JSDoc comments with mathematical notation
- Economic interpretation examples (resource allocation, production planning)
- Shadow price interpretation (marginal resource values)
- Sensitivity analysis use cases (what-if scenarios, parameter robustness)

### Changed

- **Function Registry Refactoring (continued):**
  - Optimization functions integrated into modular handler system
  - Function dispatcher updated with optimization routing
  - Consistent pattern across all function categories

### Performance

- **Simplex Algorithm:**
  - Efficient pivot operations with in-place tableau updates
  - Typical problems (n=10, m=5): <1ms
  - Revised simplex for large problems (n>1000): reduced memory footprint

- **Sensitivity Analysis:**
  - Shadow prices extracted from optimal tableau (O(m))
  - Coefficient sensitivity: Conservative ranges (instant)
  - RHS sensitivity: Conservative ranges (instant)

- **Integer Programming (Branch & Bound):** ⭐ **NEW**
  - Small instances (n=3): ~0.3-0.4ms (WASM)
  - Medium instances (n=5): ~0.2-0.4ms (WASM)
  - Binary optimization: 0/1 branching strategy (~30% faster than general integer)
  - Iteration limit: 50,000 (prevents runaway computation)
  - Memory efficient: Fixed variable substitution reduces problem size

### Technical Details

- **Linear Programming:**
  - **Tableau Structure:** Row-major storage with slack variables
  - **Pivot Selection:** Bland's rule to prevent cycling
  - **Numerical Stability:** Tolerance-based comparison (1e-10)
  - **Memory Management:** Shared handle system across all WASM operations

- **Integer Programming:** ⭐ **NEW**
  - **Algorithm:** Branch & Bound with LP relaxations
  - **Branching Strategy:** Most fractional heuristic (closest to 0.5)
  - **Binary Optimization:** Direct 0/1 branching (not floor/ceil)
  - **Fixed Variables:** Automatic substitution when lower == upper
  - **Pruning:** Bound-based elimination (LP objective vs best integer solution)
  - **Constraint Handling:** Avoids negative RHS through variable substitution
  - **Integer Tolerance:** 1e-6 for checking integrality

### Example Usage

#### Linear Programming

```typescript
const ach = new Achronyme();
await ach.init();

await ach.use(async () => {
  // Production planning problem
  // maximize z = 40x₁ + 30x₂
  // subject to: x₁ ≤ 40, x₂ ≤ 50, x₁+x₂ ≤ 70

  const c = ach.vector([40, 30]);
  const A = ach.matrix([[1, 0], [0, 1], [1, 1]]);
  const b = ach.vector([40, 50, 70]);

  // Solve with linprog
  const solution = ach.optimization.linprog(c.handle, A.handle, b.handle, 1);
  const profit = ach.optimization.objectiveValue(c.handle, solution);
  // profit = 2500

  // Shadow prices (marginal resource values)
  const shadows = ach.optimization.shadowPrice(c.handle, A.handle, b.handle, 1);
  // [10, 0, 30] = [$10/unit A, $0/unit B, $30/hour]

  // Sensitivity: how much can c[0] vary?
  const range = ach.optimization.sensitivityC(c.handle, A.handle, b.handle, 0);
  // [20, 80] = c[0] can vary between $20-$80 without changing solution structure
});
```

#### Integer Programming ⭐ **NEW**

```typescript
await ach.use(async () => {
  // 0-1 Knapsack Problem
  // maximize value within weight limit
  const values = ach.vector([60, 100, 120]);
  const weights = ach.matrix([[10, 20, 30]]);
  const capacity = ach.vector([50]);
  const binVars = ach.vector([0, 1, 2]); // All variables are binary

  const solution = ach.optimization.binaryLinprog(
    values.handle, weights.handle, capacity.handle, 1, binVars.handle
  );

  const totalValue = ach.optimization.objectiveValue(values.handle, solution);
  // solution = [0, 1, 1] → take items 2 and 3
  // totalValue = 220 (weight = 50)
});

await ach.use(async () => {
  // Capital Budgeting - Select projects within budget
  const npvs = ach.vector([100, 150, 200, 250]);
  const costs = ach.matrix([[50, 75, 100, 125]]);
  const budget = ach.vector([200]);
  const binVars = ach.vector([0, 1, 2, 3]);

  const solution = ach.optimization.binaryLinprog(
    npvs.handle, costs.handle, budget.handle, 1, binVars.handle
  );

  // solution = [0, 1, 0, 1] → Projects 2 and 4
  // Total NPV = $400M (budget used = $200M)
});
```

### Breaking Changes
- None - All new features are additions to the API

### Migration Notes
- Optimization module is opt-in via `ach.optimization` namespace
- No changes required to existing code
- Fully compatible with existing SDK architecture

## [0.5.2] - 2025-11-05

### Added

- **Comprehensive Built-in Function Reference:** Added a detailed list of all functions available in the SOC language evaluator, now organized by domain.

  - **Trigonometric:** `sin`, `cos`, `tan`, `asin`, `acos`, `atan`, `atan2`, `sinh`, `cosh`, `tanh`
  - **Exponential & Logarithmic:** `exp`, `ln`, `log` (alias for ln), `log10`, `log2`, `sqrt`, `cbrt`, `pow`
  - **Rounding & Utility:** `floor`, `ceil`, `round`, `trunc`, `abs`, `sign`, `deg`, `rad`, `min`, `max`
  - **Complex Numbers:** `complex`, `real`, `imag`, `conj`, `arg`
  - **Vector Operations:** `dot`, `cross`, `norm`, `normalize`
  - **Matrix Operations:** `transpose`, `det`, `trace`
  - **Statistics:** `sum`, `mean`, `std`
  - **Digital Signal Processing (DSP):**
    - **FFT:** `fft`, `ifft`, `fft_mag`, `fft_phase`
    - **Convolution:** `conv`, `conv_fft`
    - **Windows:** `hanning`, `hamming`, `blackman`, `rectangular`
    - **Utilities:** `linspace`
  - **Optimization & Linear Programming:**
    - **Solvers:** `simplex`, `linprog`, `dual_simplex`, `two_phase_simplex`, `revised_simplex`
    - **Analysis:** `objective_value`, `shadow_price`, `sensitivity_b`, `sensitivity_c`

### Changed

- **Refactored `FunctionRegistry`:** Modularized the monolithic function registry into domain-specific modules (`trig`, `dsp`, `stats`, etc.) located in the `function_modules/` directory. This greatly improves organization, scalability, and maintainability, making it easier to add new function categories in the future.

## [0.5.1] - 2025-01-05

### Added - Numerical Calculus Module 🧮

**Complete Numerical Calculus Suite:**
- **Numerical Differentiation:**
  - `diff()` - First derivative using central difference method
  - `diff2()` - Second derivative
  - `diff3()` - Third derivative
  - Configurable step sizes for precision control

- **Numerical Integration:**
  - `integral()` - Trapezoidal rule integration
  - `simpson()` - Simpson's 1/3 rule (higher accuracy)
  - `romberg()` - Romberg integration with Richardson extrapolation
  - `quad()` - Adaptive quadrature for automatic accuracy

- **Root Finding:**
  - `solve()` - Bisection method for bracketed roots
  - `newton()` - Newton's method (requires derivative)
  - `secant()` - Secant method (no derivative needed)

**Architecture Improvements:**
- **Dependency Injection Pattern** via `LambdaEvaluator` trait:
  - Decouples numerical algorithms from expression evaluator
  - Enables multiple evaluation backends (JIT, GPU, caching, etc.)
  - Simplifies testing with mock evaluators
  - Resolves Rust borrow checker conflicts elegantly

- **Multi-Parameter Lambda Support:**
  - Extended `LambdaEvaluator` with `eval_at_nd()` for N-dimensional functions
  - Foundation for future ODEs, optimization, and multivariate calculus
  - Full support for `(x, y) => x^2 + y^2` style lambdas

**WASM Bindings:**
- New `evalToHandle()` function - evaluates SOC expressions and returns handles
- 10 new numerical function exports: `numDiff`, `numDiff2`, `numDiff3`, `numIntegral`, `numSimpson`, `numRomberg`, `numQuad`, `numSolve`, `numNewton`, `numSecant`

**TypeScript SDK:**
- New `NumericalOps` module with complete JSDoc documentation
- Clean API accepting SOC expression strings: `numerical.diff('x => x^2', 2)`
- All functions return numbers or throw descriptive errors
- Example usage in interactive demo with 11 comprehensive tests

**Testing:**
- SOC script tests for all numerical functions
- Interactive demo category with visual results
- Validated against analytical solutions (derivatives, integrals, roots)
- All tests passing with 6-decimal precision

### Technical Details
- **Modular Architecture**: New `achronyme-numerical` crate (pure math, no eval dependency)
- **Zero Breaking Changes**: All existing APIs remain unchanged
- **Performance**: Numerical functions execute directly in WASM with minimal overhead

## [0.5.0] - 2025-11-04

### Added - Rust WASM Integration & SDK v2.0 🦀

**Complete Rewrite with Rust/WASM:**
- **Rust WASM Core** using `wasm-bindgen` with `target=bundler`
- **TypeScript SDK v2.0** with modern architecture:
  - Session-based resource management with automatic cleanup
  - Modular operations: `MathOps`, `DSPOps`, `LinalgOps`, `VectorOps`, `HOFOps`, `StatsOps`
  - Zero-copy value types: `Vector`, `Matrix`, `Scalar`, `Complex`
  - Handle-based memory management with garbage collection
  - Memory pool for efficient handle reuse

**SOC Language Evaluator:**
- Full expression evaluation with `eval()` method
- Lambda functions with closures support
- Higher-order functions: `map`, `filter`, `reduce`, `pipe`
- Variable declarations with `let`
- Complete parser and evaluator in Rust

**Performance Improvements:**
- **5.25x faster** than JavaScript V8 in vectorized math operations
- Optimized vector operations with direct WASM calls (fast-path)
- 10M element operations:
  - `sin()`: 625ms (79.8M ops/sec) vs V8: 3805ms
  - `cos()`: 650ms (76.8M ops/sec) vs V8: 3647ms
  - `exp()`: 754ms (66.2M ops/sec) vs V8: 3417ms

**Interactive Demo:**
- Comprehensive test suite with 8 categories
- Real-time benchmarks vs JavaScript V8 and math.js
- Stress tests (50K vectors, 100K elements, 32K DSP pipeline)
- SOC expression tests with lambdas and HOF

### Changed
- **WASM Build Target**: Changed from `target=web` to `target=bundler` for npm compatibility
- **SDK Architecture**: Refactored to use Rust WASM instead of C++ Emscripten
- **VectorOps Fast-Path**: Now uses direct WASM bindings instead of JavaScript loops (2.6x faster)
- **Import Structure**: Uses compiled artifacts from `dist/` for production

### Fixed
- **Type Safety**: Fixed Float64Array ↔ number[] conversions in RustBindings
- **Initialization**: Fixed DOMContentLoaded timing with module scripts
- **Vite Support**: Added `vite-plugin-wasm` and `vite-plugin-top-level-await` for bundler compatibility
- **Memory Management**: `gc()` now returns count of freed handles
- **Package Publishing**: Fixed `.npmignore` to include dist/ properly (55 files, 1.3MB)

### Technical Details
- Rust crates: `achronyme-wasm`, `achronyme-types`, `achronyme-parser`, `achronyme-eval`, `achronyme-dsp`, `achronyme-linalg`
- TypeScript SDK with full type definitions
- Automatic WASM initialization with bundler target
- Enhanced error handling and logging

### Validation
- ✅ All SDK tests passing
- ✅ All SOC expression tests passing (8/8)
- ✅ All benchmarks showing expected performance (5.25x vs V8)
- ✅ Package ready for npm publish (verified with `npm pack --dry-run`)
- ✅ Interactive demo fully functional

## [0.4.0] - 2025-11-01

### Added - Advanced Linear Algebra 📐
- **Matrix Decompositions**:
  - LU decomposition with partial pivoting (`lu`).
  - QR decomposition using Householder reflections (`qr`).
  - Cholesky decomposition for symmetric, positive-definite matrices (`cholesky`).
  - Singular Value Decomposition (SVD) (`svd`).
- **Eigenvalue and Eigenvector Solvers**:
  - Power Iteration method for finding the dominant eigenvalue (`power_iteration`).
  - QR Algorithm for finding all eigenvalues of a matrix (`qr_eigenvalues`).
  - Full symmetric eigenvalue solver (`eig`).
- **Helper Functions**: `isSymmetric`, `isPositiveDefinite`, `identity`.

### Fixed
- **CRITICAL**: Fixed a memory corruption (dangling pointer) bug in the Handle-to-JavaScript data retrieval process. This was caused by returning a pointer to a temporary object's data. The fix ensures all handle operations are stable and memory-safe.
- Corrected intermittent failures in eigenvalue tests, which were a symptom of the memory bug.

### Changed
- Refactored the internal implementation of the QR eigenvalue algorithm for clarity and robustness.

### Validation
- ✅ All 11 linear algebra tests now pass consistently.
- ✅ No regressions introduced in existing SDK, DSP, or Handle system tests.

## [0.3.0] - 2025-11-01

### Added - Performance Revolution 🚀
- **Handles System (Fast Path)**: 10-1000x performance improvement.
  - C++ HandleManager for efficient memory management.
  - Zero-copy operations with direct references to WASM memory.
  - Automatic fast path for vectors ≥8 elements (configurable).
  - Backward-compatible slow path for small vectors and debugging.
  - Statistics tracking: `getMemoryStats()` shows fast path usage %.
  - Real-world benchmarks: Vector 100K (900x), FFT 4096 (150x), DSP Pipeline (33x).
- **Vectorized Math Functions** (native in C++):
  - `exp(vector)` - Element-wise exponential (~100x faster than map).
  - `ln(vector)` - Element-wise natural logarithm.
  - `sqrt(vector)` - Element-wise square root.
  - `abs(vector)` - Element-wise absolute value.
  - `sin(vector)`, `cos(vector)`, `tan(vector)` - Element-wise trigonometric functions.
  - **Note**: Transparent API - functions automatically accept both scalars and vectors.
- **DSP Fast Path Optimizations**:
  - `fft_fast()` - Optimized FFT with handles (direct memory access).
  - `fft_mag_fast()` - FFT magnitude without intermediate serialization.
  - `fft_phase_fast()` - FFT phase without parsing overhead.
  - `linspace()` - Optimized generation from the start with handles.
  - Full DSP pipelines without leaving WASM memory.
- **Exhaustive Test Suite** (~200 total tests):
  - `test-stability.mjs` - 20 stability tests (10K ops, 1M elements, stress).
  - `test-accuracy.mjs` - 25 math precision tests (1e-6 tolerance).
  - `test-edge-cases.mjs` - 25 edge case tests.
  - All tests: 0 memory leaks, >90% fast path usage.
  - Full validation of the handles system.
- **Complete Technical Documentation**:
  - `FAST-PATH-VS-SLOW-PATH-EXPLICACION.md` - Complete system guide.
  - `FAST-PATH-DIAGRAMS.md` - Visual diagrams of architecture and flows.
  - `LEGACY-TESTS-FIX-SUMMARY.md` - Summary of compatibility fixes.
  - `TEST-SUITE-SUMMARY.md` - Summary of test suite and results.
  - `RESUMEN-SESION.md` - Executive summary of implementation.

### Fixed - Compatibility
- **Emscripten 4.0 Compatibility**:
  - Updated WASM heap access (`HEAPF64.buffer` → `HEAPF64.subarray`).
  - Added `EXPORTED_RUNTIME_METHODS='["HEAPF64","HEAPU32"]'` to build.
  - Fixed TypeScript types to include `HEAP8`.
  - Build scripts updated for Emscripten 4.0.15.
- **Legacy Tests Updated** (10 files):
  - Fixed duplicate import paths (`sdk/sdk/` → `sdk/`).
  - Fixed incorrect API usage (`.fft_mag()` on spectra → signals).
  - Fixed relative paths in `test-npm-import.mjs` and `debug-module.mjs`.
  - All legacy tests now compatible and passing.

### Changed
- **Performance Improvements**:
  - Vector creation (100K elements): ~450ms → ~0.5ms (900x improvement).
  - FFT 4096 samples: ~180ms → ~1.2ms (150x improvement).
  - Full DSP pipeline: ~100ms → ~3ms (33x improvement).
  - Memory overhead: Drastically reduced with zero-copy.
- **API Enhancement** (no breaking changes):
  - Math functions now automatically accept vectors.
  - Handles system is completely transparent to the user.
  - Automatic fast/slow path decision based on data size.
  - Public API is 100% backward compatible.

### Technical Details
- **Fast Path Threshold**: 8 elements (configurable via `fastPathThreshold`).
- **Memory Management**: `shared_ptr<Value>` with HandleManager.
- **Zero-Copy**: Data remains in WASM memory during operations.
- **Statistics**: `fastPathOperationsCount`, `slowPathOperationsCount` tracked.
- **Fallback**: Automatic fallback to slow path if fast path fails (robust).

### Migration Guide
- **For npm package users**: No changes required.
  ```javascript
  // Existing code works without modification
  const v = ach.vector([1,2,3,4,5,6,7,8]);
  const result = v.exp();  // Now ~100x faster!
  ```
- **For developers compiling from source**:
  - Requires Emscripten 4.0+ (tested with 4.0.15).
  - Update emsdk and recompile: `npm run build`.

### Breaking Changes
- Requires Emscripten 4.0+ to compile from source.
- No breaking changes in the public API.

### Validation
- ✅ ~200 tests passing (test-stability, test-accuracy, test-edge-cases, test-sdk).
- ✅ 0 memory leaks in all critical tests.
- ✅ Fast path usage: >90% in real-world use cases (DSP pipelines).
- ✅ Backward compatibility: 100% (all legacy tests updated).

---

## [0.3.0-beta-8] - 2025-10-27

### Fixed
- **CRITICAL:** Fixed FFT spectrum frequency desynchronization bug in `fft_spectrum()`
  - **Issue:** Frequencies did not correctly correspond to magnitudes and phases after `fftshift`
  - **Cause:** `fftshift` was applied independently to frequencies and FFT results, then frequencies were sorted, breaking synchronization
  - **Solution:** Apply `fftshift` simultaneously to both frequencies and FFT spectrum, maintaining index correspondence
  - **Impact:** FFT spectrum results now 100% accurate (frequency error = 0.0000 rad/s)
  - **Validation:** Tested with 45 intensive tests using known signals (sinusoids, complex signals, impulse, DC)
  - **Affected module:** `wasm/src/core/functions_dsp.cpp:860-976`
  - **Upgrade priority:** HIGH - If you use `fft_spectrum()`, update immediately

### Added
- Cross-platform build system
  - Node.js cross-platform build script (`scripts/build-cross-platform.mjs`)
  - Bash scripts for Unix/Linux/Mac (`scripts/build-wasm.sh`, `scripts/build-wasm-dev.sh`)
  - Batch scripts for Windows (`scripts/build-wasm.bat`, `scripts/build-wasm-dev.bat`)
  - Development mode compilation (5-10x faster, includes debug symbols)
  - Production mode compilation (optimized -O3)
- New npm scripts
  - `npm run build:wasm:dev` - Fast development build
  - `npm run build:dev` - Full development build (WASM + TypeScript)
  - `npm run clean` - Clean all build artifacts
  - `npm run test:sdk` - Run SDK test suite
  - `npm run test:dsp` - Run intensive DSP tests
  - `npm run test:all` - Run all tests
- Comprehensive documentation
  - `BUILD-GUIDE.md` - Complete build guide
  - `QUICK-START.md` - Quick start guide (3 steps)
  - `scripts/README.md` - Build scripts documentation

### Changed
- Improved build system with better error handling and cross-platform support
- Build scripts now use `emcc` directly (simpler, more reliable)
- Updated package.json with additional build and test scripts

### Validated
- ✅ SDK tests: 30/30 passing
- ✅ FFT vs DFT cross-validation: Perfect match
- ✅ Signal analysis with known frequencies: Exact results
- ✅ Conjugate symmetry preserved for real signals

## [0.3.0] - 2025-10-26

### Added
- **Phase 3: Complex Types** 🎉🎉🎉
  - **Complex Numbers** (a + bi)
    - Full arithmetic support (+, -, *, /, ^)
    - Imaginary unit: `i`, `3i`, `2+3i`
    - Functions: `complex(real, imag)`, `real(z)`, `imag(z)`, `conj(z)`, `arg(z)`
    - Complex magnitude: `abs(3+4i)` = 5
    - Automatic type promotion: Number → Complex

  - **Vectors** ([x, y, z, ...])
    - Vector literals: `[1, 2, 3]`
    - Arithmetic: `[1,2] + [3,4]`, `[1,2,3] * 2`
    - Functions: `dot(v1, v2)`, `cross(v1, v2)`, `norm(v)`, `normalize(v)`
    - Broadcasting: `[1,2,3] + 10` → `[11, 12, 13]`
    - Expressions in vectors: `[sin(0), cos(0), PI]`

  - **Matrices** ([[a, b], [c, d], ...])
    - Matrix literals: `[[1, 2], [3, 4]]`
    - Arithmetic: matrix addition, subtraction, multiplication
    - Functions: `transpose(M)`, `det(M)`, `inverse(M)`, `trace(M)`
    - Scalar operations: `2 * [[1,2],[3,4]]`
    - Expressions in matrices: `[[PI, E], [sqrt(2), sqrt(3)]]`

  - **Type System**
    - std::variant-based Value type
    - Type checking: `isNumber()`, `isComplex()`, `isVector()`, `isMatrix()`
    - Automatic type coercion
    - Type-safe operations with runtime dispatch

  - **Lexer/Parser Extensions**
    - New tokens: `[`, `]` (LBRACKET, RBRACKET)
    - Complex literal parsing: `3i`, `i`
    - Vector/matrix literal parsing
    - Automatic matrix row validation

  - **Extended Functions**
    - `abs()` now works for complex numbers
    - 13 new functions for complex types
    - Full support for nested expressions

### Changed
- WASM bundle size increased to 234 KB (from 106 KB)
- eval() now returns string representation for all types
- Value class now uses std::variant instead of double
- Breaking change: eval() return type changed from double to string

### Performance
- Complex arithmetic: <10μs per operation
- Vector operations: <5μs for basic ops
- Matrix multiplication (2x2): <20μs
- Still 3-20x faster than JavaScript equivalents

## [0.2.0] - 2025-10-26

### Added
- **Phase 2: Mathematical Functions** 🎉
  - **Constants Registry** (7 constants)
    - Mathematical constants: PI, E, PHI (golden ratio)
    - Square roots: SQRT2, SQRT3
    - Logarithms: LN2, LN10
    - Case-insensitive lookup

  - **Function Registry** (25+ functions)
    - Trigonometric: sin, cos, tan, asin, acos, atan, atan2
    - Hyperbolic: sinh, cosh, tanh
    - Exponential/Logarithmic: exp, log, ln, log10, log2
    - Power/Root: sqrt, cbrt, pow
    - Rounding: floor, ceil, round, trunc
    - Utility: abs, sign, min, max, deg, rad

  - **Lexer Extensions**
    - IDENTIFIER token type for function/constant names
    - COMMA token for function arguments
    - scanIdentifier() method

  - **Parser Extensions**
    - Function call parsing: `name(arg1, arg2, ...)`
    - Constant parsing: `PI`, `E`, etc.
    - Variadic function support (min, max)
    - Grammar extended for function calls

  - **AST Extensions**
    - FunctionCallNode class for function calls
    - Support for N arguments

  - **Evaluator Extensions**
    - evaluateFunctionCall() method
    - Arity checking
    - Nested function evaluation

### Changed
- WASM bundle size increased to 106 KB (from 53 KB) due to math functions
- Demo updated with 34 comprehensive test expressions
- All expressions now case-insensitive (PI = pi = Pi)

### Performance
- Simple functions (sin, cos): <5μs
- Nested functions: <10μs
- Complex expressions: <15μs
- Still 10-20x faster than pure JavaScript

## [0.1.0] - 2025-10-26

### Added
- Initial release of Achronyme Core
- **Phase 1: Arithmetic Evaluator**
  - Lexer (tokenizer) implementation
  - Recursive descent parser with operator precedence
  - AST-based evaluator
  - Support for basic arithmetic operators: `+`, `-`, `*`, `/`, `^`
  - Parentheses support for precedence override
  - Unary minus operator
  - Decimal number support
  - Scientific notation support (e.g., `1e-3`, `2.5e10`)
  - Right-associative exponentiation (`2^3^2 = 512`)
- Emscripten bindings for WebAssembly export
- TypeScript/JavaScript wrapper (SOC class)
- Comprehensive test suite (C++ with Google Test, TypeScript with Vitest)
- Build scripts for WASM compilation
- Complete documentation and examples

### Technical Details
- C++20 codebase
- WebAssembly compilation via Emscripten
- Zero-copy AST evaluation
- Type-safe bindings

[Unreleased]: https://github.com/eddndev/achronyme-core/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/eddndev/achronyme-core/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/eddndev/achronyme-core/releases/tag/v0.1.0
