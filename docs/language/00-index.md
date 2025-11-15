# Achronyme Language Reference

Welcome to the Achronyme programming language documentation. Achronyme is a functional, expression-oriented language designed for mathematical computations, numerical analysis, and scientific computing.

## What is Achronyme?

Achronyme (also known as SOC - Second-Order Calculus) is a domain-specific language that combines:
- **Functional programming** with first-class functions and closures
- **Mathematical notation** familiar to scientists and engineers
- **Powerful built-in libraries** for numerical analysis, DSP, linear algebra, and graph theory
- **Tensor operations** with support for multi-dimensional arrays
- **Interactive REPL** for rapid prototyping and exploration

## File Extension

Achronyme source files use the `.soc` extension (Second-Order Calculus).

## Quick Example

```javascript
// Import from modules
import { mean, std } from "stats"
import { sin, cos } from "math"

// Define a function
let square = x => x^2

// Use higher-order functions
let numbers = [1, 2, 3, 4, 5]
let squared = map(square, numbers)

// Numerical integration
let area = integral(x => x^2, 0, 10)

// Records with mutable fields
let counter = {
    mut value: 0,
    increment: () => do { self.value = self.value + 1 },
    get: () => self.value
}

// Statistical analysis
let data = [10, 20, 30, 40, 50]
let average = mean(data)
let stdDev = std(data)
```

## Documentation Structure

### Getting Started
- **[01. Getting Started](01-getting-started.md)** - Installation, REPL, first program
- **[02. Syntax Basics](02-syntax-basics.md)** - Comments, expressions, statements

### Core Language
- **[03. Data Types](03-data-types.md)** - Numbers, booleans, strings, complex numbers
- **[04. Operators](04-operators.md)** - Arithmetic, comparison, logical operators
- **[05. Variables](05-variables.md)** - Declaration, scope, shadowing
- **[06. Functions](06-functions.md)** - Function calls, lambdas, closures, recursion
- **[07. Records](07-records.md)** - Object-like structures, methods, OOP patterns
- **[08. Control Flow](08-control-flow.md)** - Conditionals, piecewise functions

### Arrays and Tensors
- **[09. Arrays and Tensors](09-arrays-tensors.md)** - Vectors, matrices, N-dimensional arrays
- **[10. Indexing and Slicing](10-indexing-slicing.md)** - Array access, ranges, spread operator

### Functional Programming
- **[11. Higher-Order Functions](11-higher-order-functions.md)** - map, filter, reduce, pipe

### Mathematical Computing
- **[12. Mathematical Functions](12-mathematical-functions.md)** - Trigonometry, exponentials, rounding
- **[13. Linear Algebra](13-linear-algebra.md)** - Dot product, cross product, matrix operations
- **[14. Complex Numbers](14-complex-numbers.md)** - Complex arithmetic, polar form
- **[15. Numerical Analysis](15-numerical-analysis.md)** - Differentiation, integration, root finding
- **[16. Statistics](16-statistics.md)** - Sum, mean, standard deviation

### Specialized Modules
- **[17. Digital Signal Processing](17-dsp.md)** - FFT, convolution, window functions
- **[18. Graph Theory](18-graph-theory.md)** - Networks, paths, algorithms
- **[19. Optimization](19-optimization.md)** - Linear programming, simplex method
- **[20. Strings](20-strings.md)** - String operations
- **[25. Utilities](25-utilities.md)** - Output, type inspection, string conversion

### Advanced Topics
- **[21. Do Blocks](21-do-blocks.md)** - Multi-statement blocks
- **[22. Recursion Patterns](22-recursion.md)** - Recursive functions, self-reference
- **[23. Best Practices](23-best-practices.md)** - Code style, patterns, performance
- **[24. Examples](24-examples.md)** - Complete programs and use cases
- **[26. Mutability](26-mutability.md)** - Mutable variables, mutable record fields, stateful objects
- **[27. I/O and Persistence](27-io-persistence.md)** - File I/O, environment save/restore
- **[28. Modules](28-modules.md)** - Import/export system, code organization
- **[29. While Loops](29-while-loops.md)** - Iterative loops with mutable state
- **[30. Gradual Type System](30-gradual-type-system.md)** - Type annotations, union types, type aliases

## Language Philosophy

### Expression-Oriented
Everything in Achronyme is an expression that returns a value. There are no statements that don't produce values.

```javascript
let result = if(x > 0, 1, -1)  // if() is a function that returns a value
```

### Immutable by Default, Mutable When Needed
Variables are immutable by default but can be declared mutable with `mut`:

```javascript
// Immutable (default)
let x = 10
let x = x + 5  // New binding, shadows the old one

// Mutable (explicit)
mut counter = 0
counter = counter + 1  // Reassignment allowed
```

### First-Class Functions
Functions are values that can be passed around, stored in variables, and returned from other functions:

```javascript
let operation = if(mode == "add", (a, b) => a + b, (a, b) => a * b)
operation(3, 4)
```

### Type Inference
The language automatically infers types based on usage. Arrays of numbers become tensors, supporting efficient mathematical operations.

## Feature Highlights

### ✨ Mathematical Computing
- Built-in support for complex numbers: `2 + 3i`
- Tensor operations with broadcasting
- Comprehensive math library (trig, exp, log, etc.)

### 🔧 Numerical Analysis
- Automatic differentiation
- Numerical integration (trapezoid, Simpson, Romberg)
- Root finding (bisection, Newton-Raphson, secant)

### 📊 Signal Processing
- Fast Fourier Transform (FFT)
- Convolution (direct and FFT-based)
- Window functions (Hanning, Hamming, Blackman)

### 🕸️ Graph Algorithms
- BFS, DFS, Dijkstra
- Minimum Spanning Trees (Kruskal, Prim)
- Topological sort
- PERT/CPM for project management

### 🎯 Modern Syntax
- Lambda functions: `x => x^2`
- Higher-order functions: `map`, `filter`, `reduce`
- Spread operator: `[...array1, ...array2]`
- Records with methods and `self` reference

## Community and Support

- **GitHub**: [Achronyme Repository](https://github.com/anthropics/achronyme-core)
- **Issues**: Report bugs or request features
- **Examples**: See the `examples/soc/` directory

## Next Steps

1. Start with [Getting Started](01-getting-started.md) to set up your environment
2. Learn the [Syntax Basics](02-syntax-basics.md)
3. Explore [Data Types](03-data-types.md)
4. Try the [Examples](24-examples.md)

---

**Note**: This is an evolving language. Some features may be experimental or subject to change.
