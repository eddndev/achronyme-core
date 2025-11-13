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

### Core HOFs

These functions are "special forms" that require lazy evaluation:

- `map(fn, coll, ...)` - Apply function to collection(s)
- `filter(pred, coll)` - Filter elements by predicate
- `reduce(fn, initial, coll)` - Reduce collection to single value
- `pipe(value, f1, f2, ...)` - Function composition pipeline

### Predicate HOFs (Tier 2)

Higher-order functions for searching and testing:

- `any(coll, pred)` - Check if any element matches predicate
- `all(coll, pred)` - Check if all elements match predicate
- `find(coll, pred)` - Find first matching element (errors if not found)
- `findIndex(coll, pred)` - Find index of first match (returns -1 if not found)
- `count(coll, pred)` - Count elements matching predicate

Example:
```javascript
let doubled = map(x => x * 2, [1, 2, 3])  // [2, 4, 6]
let evens = filter(x => x % 2 == 0, [1, 2, 3, 4])  // [2, 4]
any([1, 2, 3, 4], x => x > 3)  // true
count([1, 2, 3, 4, 5], x => x > 2)  // 3
```

## Array Utility Functions

### Tier 1: Essential Operations

- `product(array)` - Multiply all elements
- `range(start, end, step?)` - Generate integer sequence (exclusive end)
- `len(array)` - Get array/vector/tensor length
- `reverse(array)` - Reverse array order
- `contains(array, value)` - Check if value exists in array (also works with strings)

Example:
```javascript
product([1, 2, 3, 4])     // 24
range(0, 5)               // [0, 1, 2, 3, 4]
len([1, 2, 3])            // 3
reverse([1, 2, 3])        // [3, 2, 1]
contains([1, 2, 3], 2)    // true
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

### Basic Functions

- `concat(s1, s2)` - Concatenate two strings (also available via `+` operator)
- `length(s)` - Get string length

### Case Conversion

- `upper(s)` - Convert to uppercase
- `lower(s)` - Convert to lowercase

### Whitespace Handling

- `trim(s)` - Remove whitespace from both ends
- `trim_start(s)` - Remove whitespace from start
- `trim_end(s)` - Remove whitespace from end

### Search Functions

- `starts_with(s, prefix)` - Check if string starts with prefix
- `ends_with(s, suffix)` - Check if string ends with suffix
- `contains(s, substring)` - Check if string contains substring

### Manipulation

- `replace(s, pattern, replacement)` - Replace all occurrences
- `split(s, delimiter)` - Split string into array
- `join(array, delimiter)` - Join array of strings

### Padding

- `pad_start(s, length, fill_char?)` - Pad at start (default: space)
- `pad_end(s, length, fill_char?)` - Pad at end (default: space)

Example:
```javascript
upper("hello")                              // "HELLO"
trim("  hello  ")                           // "hello"
split("a,b,c", ",")                         // ["a", "b", "c"]
join(["a", "b", "c"], "-")                  // "a-b-c"
pad_start("5", 3, "0")                      // "005"
"hello" + " " + "world"                     // "hello world"
```

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

## Utility Functions

Essential utility functions for output, type inspection, and string conversion:

### Output
- `print(value1, value2, ...)` - Print values to standard output (variadic, accepts 1+ arguments)
  - Prints values separated by spaces
  - Always adds newline at end
  - Returns the last value printed (useful for chaining)
  - Example: `print("Value:", x)` or `print(1, 2, 3)`

### Type Inspection
- `type(value)` - Get the type name of a value as a string
  - Returns: "Number", "Boolean", "String", "Vector", "Tensor", "ComplexTensor", "Complex", "Function", "Record", "Edge", "TailCall", or "MutableRef<T>"
  - Example: `type(42)` returns `"Number"`

### String Conversion
- `str(value)` - Convert any value to its string representation
  - Numbers: Formats integers without decimal point (42 instead of 42.0)
  - Complex: Formats as "a+bi" or "a-bi"
  - Vectors/Tensors: Formats as "[1, 2, 3]"
  - Functions: Shows "<function:name>" or "<function:lambda>"
  - Records: Shows "{key: value, ...}"
  - Example: `"The answer is " + str(42)` returns `"The answer is 42"`

Example usage:
```javascript
let x = [1, 2, 3];
print("Type:", type(x));     // Prints: Type: Tensor
print("Value:", str(x));     // Prints: Value: [1, 2, 3]

// Use in pipeline
pipe(
    5,
    x => x * 2,
    x => print(x),    // Prints 10 and returns it
    x => x + 10
)  // Returns 20
```

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
- Predicate HOFs: 5 functions (any, all, find, findIndex, count)
- Array Utilities: 5 functions (product, range, len, reverse, contains)
- Numerical: 13 functions (differentiation, integration, solvers)
- Strings: 15 functions (concat, length, upper, lower, trim, split, join, etc.)
- Records: 3 functions (keys, values, has_field)
- Graphs: 15+ functions
- PERT/CPM: 13 functions
- Utilities: 3 functions (print, type, str)
- Debug: 1 function (describe)

### How to Check Available Functions

To see all registered functions programmatically, you can iterate through the function registry (developer documentation).
