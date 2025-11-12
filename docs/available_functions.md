# Available Functions in Achronyme

This document lists all functions currently available in Achronyme, organized by category.

## How Functions Work

All registered functions are available as first-class values, meaning you can:
- Assign them to variables: `let f = sin`
- Pass them as arguments to higher-order functions: `map(sin, [0, 1, 2])`
- Store them in records: `{ method: sin }`

## Trigonometric Functions

Basic trigonometry:
- `sin(x)` - Sine
- `cos(x)` - Cosine
- `tan(x)` - Tangent
- `asin(x)` - Arc sine
- `acos(x)` - Arc cosine
- `atan(x)` - Arc tangent
- `atan2(y, x)` - Two-argument arc tangent

Hyperbolic functions:
- `sinh(x)` - Hyperbolic sine
- `cosh(x)` - Hyperbolic cosine
- `tanh(x)` - Hyperbolic tangent

## Exponential and Logarithmic Functions

- `exp(x)` - Exponential (e^x)
- `ln(x)` - Natural logarithm
- `log(x)` - Natural logarithm (alias for ln)
- `log10(x)` - Base-10 logarithm
- `log2(x)` - Base-2 logarithm
- `sqrt(x)` - Square root
- `cbrt(x)` - Cube root
- `pow(base, exp)` - Power function (base^exp)

## Rounding and Basic Math

- `floor(x)` - Round down
- `ceil(x)` - Round up
- `round(x)` - Round to nearest integer
- `abs(x)` - Absolute value
- `sign(x)` - Sign function (-1, 0, or 1)

## Complex Number Functions

- `complex(re, im)` - Create complex number
- `real(z)` - Get real part
- `imag(z)` - Get imaginary part
- `conj(z)` - Complex conjugate
- `arg(z)` - Argument (phase angle)

Note: The imaginary unit `i` is available as a constant.

## Statistical Functions

- `sum(vector)` - Sum of elements
- `mean(vector)` - Arithmetic mean
- `std(vector)` - Standard deviation

## Matrix/Vector Operations

- `dot(v1, v2)` - Dot product
- `cross(v1, v2)` - Cross product
- `norm(v)` - Vector norm (magnitude)
- `det(M)` - Matrix determinant
- `transpose(M)` - Matrix transpose
- `trace(M)` - Matrix trace

## Digital Signal Processing (DSP)

### FFT Functions
- `fft(signal)` - Fast Fourier Transform
- `ifft(spectrum)` - Inverse FFT
- `fft_mag(signal)` - FFT magnitude
- `fft_phase(signal)` - FFT phase

### Convolution
- `conv(signal, kernel)` - Direct convolution
- `conv_fft(signal, kernel)` - FFT-based convolution

### Window Functions
- `hanning(n)` - Hanning window of size n
- `hamming(n)` - Hamming window of size n
- `blackman(n)` - Blackman window of size n
- `rectangular(n)` - Rectangular window of size n

### Utility Functions
- `linspace(start, end, n)` - Generate n linearly spaced values

Example:
```javascript
let v = linspace(0, 10, 11)  // [0, 1, 2, ..., 10]
```

## Higher-Order Functions

These functions are "special forms" that require lazy evaluation:

- `map(fn, vector)` - Apply function to each element
- `filter(fn, vector)` - Filter elements by predicate
- `reduce(fn, initial, vector)` - Reduce vector to single value
- `pipe(fn1, fn2, ...)` - Function composition pipeline

Example:
```javascript
let doubled = map(x => x * 2, [1, 2, 3])  // [2, 4, 6]
let evens = filter(x => x % 2 == 0, [1, 2, 3, 4])  // [2, 4]
```

## Numerical Analysis

These are also special forms because they require lambda evaluation:

### Differentiation
- `diff(f)` - First derivative
- `diff2(f)` - Second derivative
- `diff3(f)` - Third derivative
- `gradient(f, point)` - Gradient at point
- `derivative(f, x)` - Derivative at point x

### Integration
- `integral(f, a, b)` - Numerical integration
- `trapz(f, a, b)` - Trapezoidal rule
- `simpson(f, a, b)` - Simpson's rule
- `romberg(f, a, b)` - Romberg integration
- `quad(f, a, b)` - Adaptive quadrature

### Root Finding
- `solve(f, a, b)` - Find root in interval
- `bisect(f, a, b)` - Bisection method
- `newton(f, x0)` - Newton-Raphson method
- `secant(f, x0, x1)` - Secant method

## String Functions

- `concat(s1, s2)` - Concatenate two strings
- `length(s)` - Get string length

## Record Functions

- `keys(record)` - Get record keys
- `values(record)` - Get record values
- `has_field(record, key)` - Check if field exists

## Graph/Network Functions

### Network Creation
- `network(edges)` - Create network from edges

### Network Properties
- `nodes(network)` - Get all nodes
- `edges(network)` - Get all edges
- `neighbors(network, node)` - Get neighbors
- `degree(network, node)` - Get node degree

### Graph Algorithms - Traversal
- `bfs(network, start)` - Breadth-first search
- `dfs(network, start)` - Depth-first search
- `bfs_path(network, start, end)` - BFS shortest path

### Graph Algorithms - Shortest Paths
- `dijkstra(network, start, end)` - Dijkstra's algorithm

### Graph Algorithms - Cycles
- `has_cycle(network)` - Check for cycles

### Graph Algorithms - MST
- `kruskal(network)` - Kruskal's MST algorithm
- `prim(network)` - Prim's MST algorithm

### Graph Algorithms - Connectivity
- `connected_components(network)` - Find connected components
- `is_connected(network)` - Check if graph is connected

### Graph Algorithms - Topological Sort
- `topological_sort(network)` - Topological ordering

## PERT/CPM Functions

Project management functions:
- `forward_pass(network)` - Forward pass calculation
- `backward_pass(network)` - Backward pass calculation
- `calculate_slack(network)` - Calculate slack times
- `critical_path(network)` - Find critical path
- `all_critical_paths(network)` - Find all critical paths
- `project_duration(network)` - Get project duration
- `expected_time(optimistic, most_likely, pessimistic)` - PERT expected time
- `task_variance(optimistic, pessimistic)` - Task variance
- `project_variance(network)` - Project variance
- `project_std_dev(network)` - Project standard deviation
- `completion_probability(network, time)` - Probability of completion
- `time_for_probability(network, probability)` - Time for given probability
- `pert_analysis(network)` - Complete PERT analysis

## Optimization Functions

Linear programming and optimization:
- `simplex(...)` - Simplex method
- `linprog(...)` - Linear programming
- `dual_simplex(...)` - Dual simplex
- `two_phase_simplex(...)` - Two-phase simplex
- `revised_simplex(...)` - Revised simplex
- `objective_value(...)` - Get objective value
- `shadow_price(...)` - Shadow prices
- `sensitivity_c(...)` - Sensitivity analysis (c)
- `sensitivity_b(...)` - Sensitivity analysis (b)
- `reduced_costs(...)` - Reduced costs
- `basic_variables(...)` - Basic variables
- `nonbasic_variables(...)` - Non-basic variables

## Debug Functions

- `describe(value)` - Get detailed description of a value

## Notes

### Function Categories

Functions are implemented in different ways:

1. **Standard Functions**: Registered in `FunctionRegistry`
   - Can be called normally
   - Arguments are evaluated before passing
   - Available as first-class values

2. **Special Forms**: Handled specially in `function_call.rs`
   - Require lazy evaluation of arguments
   - Have access to the evaluator
   - Still available as first-class values via `is_special_form()`

### Implementation Status

Some functions mentioned in documentation may not be fully implemented yet. If you encounter an error like "Undefined variable or constant: function_name", the function may not be implemented.

Currently implemented function counts by module:
- Trigonometry: 10 functions
- Exponential/Logarithmic: 8 functions
- Rounding: 5 functions
- Complex: 5 functions
- Stats: 3 functions (sum, mean, std)
- Matrix: 6 functions (dot, cross, norm, det, transpose, trace)
- DSP: 11 functions (fft, ifft, fft_mag, fft_phase, conv, conv_fft, 4 windows, linspace)
- HOF: 4 functions (map, filter, reduce, pipe)
- Numerical: 13 functions (differentiation, integration, solvers)
- Strings: 2 functions (concat, length)
- Records: 3 functions (keys, values, has_field)
- Graphs: 15+ functions
- PERT/CPM: 13 functions
- Debug: 1 function (describe)

### How to Check Available Functions

To see all registered functions programmatically, you can iterate through the function registry (developer documentation).
