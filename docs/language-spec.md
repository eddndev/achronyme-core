# Achronyme/SOC Language Specification

This document specifies the syntax and features of the Achronyme/SOC (String-Oriented Calculation) language, which is processed by the `ach.eval()` method. This language provides a powerful way to perform complex mathematical operations in a single, efficient call to the Rust-based core engine.

## Table of Contents

- [Introduction](#introduction)
- [Syntax](#syntax)
  - [Data Types & Literals](#data-types--literals)
  - [Operators](#operators)
  - [Variables](#variables)
  - [Functions (Lambdas)](#functions-lambdas)
- [Built-in Functions](#built-in-functions)
  - [Core Functions](#core-functions)
  - [Mathematical Functions](#mathematical-functions)
  - [Linear Algebra](#linear-algebra)
  - [Digital Signal Processing (DSP)](#digital-signal-processing-dsp)
  - [Statistics](#statistics)
- [Higher-Order Functions](#higher-order-functions)
- [Examples](#examples)

---

## Introduction

The SOC language is a simple, expression-based functional language designed for mathematics. It is dynamically typed and allows for the creation of variables and functions at runtime. Its primary purpose is to define a sequence of operations in a single string, which the Achronyme engine can then parse and execute in one pass, minimizing the communication overhead between JavaScript and WebAssembly.

**Example:**
```typescript
// Instead of multiple JS calls...
const v = ach.vector([1, 2, 3]);
const v2 = ach.math.sin(v);
const v3 = ach.math.abs(v2);

// You can do it in one eval call
const result = ach.eval("abs(sin([1, 2, 3]))");
```

---

## Syntax

### Data Types & Literals

The language supports the following primitive types:

- **Scalar**: A single number.
  ```
  42
  -3.14159
  1.2e3
  ```
- **Vector**: A 1D array of numbers.
  ```
  [1, 2, 3, 4, 5]
  ```
- **Matrix**: A 2D array of numbers.
  ```
  [[1, 2], [3, 4]]
  ```
- **Function**: A user-defined or built-in function.

### Operators

Operators follow standard mathematical precedence.

| Operator | Description | Example |
| :--- | :--- | :--- |
| `+`, `-` | Addition, Subtraction | `5 + 3` |
| `*`, `/` | Multiplication, Division | `10 * 2` |
| `^` | Power / Exponentiation | `2 ^ 10` |
| `%` | Modulo | `10 % 3` |
| `==`, `!=` | Equality, Inequality | `x == 5` |
| `>`, `<`, `>=`, `<=` | Comparison | `x > 0` |

### Variables

You can declare variables using the `let` keyword. The scope of these variables is the lifetime of the `Achronyme` instance or until `ach.resetEvaluator()` is called.

```
let pi = 3.14159
let v = [1, 2, 3]
let f = x => x * pi
f(10) // 31.4159
```

### Functions (Lambdas)

Custom functions can be defined using lambda syntax.

```
// Single parameter
let square = x => x ^ 2

// Multiple parameters
let add = (a, b) => a + b

// Using the functions
square(5) // 25
add(10, 20) // 30
```

---

## Built-in Functions

The language includes a rich standard library of functions. Function names are case-insensitive.

### Core Functions

- `let(name, value)`: Declares a variable.
- `if(condition, then_expr, else_expr)`: Conditional expression.
- `print(value)`: Prints a value to the console.

### Mathematical Functions

- **Trigonometric**: `sin`, `cos`, `tan`, `asin`, `acos`, `atan`, `atan2`
- **Hyperbolic**: `sinh`, `cosh`, `tanh`
- **Exponential/Logarithmic**: `exp`, `ln`, `log` (natural log), `log10`, `log2`
- **Power**: `sqrt`, `cbrt`, `pow(base, exp)`
- **Rounding/Misc**: `abs`, `sign`, `floor`, `ceil`, `round`, `trunc`

### Linear Algebra

- **Vector**: `dot(v1, v2)`, `cross(v1, v2)`, `norm(v)`
- **Matrix**: `det(m)`, `transpose(m)`, `inverse(m)`
- **Constructors**: `linspace(start, stop, num)`, `zeros(n)`, `ones(n)`, `identity(n)`

### Digital Signal Processing (DSP)

- **FFT**: `fft(signal)`, `ifft(spectrum)`, `fft_mag(signal)`, `fft_phase(signal)`
- **Convolution**: `conv(signal1, signal2)`
- **Windowing**: `hanning(n)`, `hamming(n)`, `blackman(n)`
- **Utilities**: `fftshift(spectrum)`, `ifftshift(spectrum)`

### Statistics

- `sum(v)`, `mean(v)`, `std(v)`
- `min(v)`, `max(v)`
- `median(v)`, `percentile(v, p)`

---

## Higher-Order Functions

These functions take other functions as arguments.

- `map(func, vector)`: Applies a function to each element of a vector, returning a new vector.
  ```
  map(x => x * 2, [1, 2, 3]) // [2, 4, 6]
  ```
- `filter(predicate, vector)`: Returns a new vector containing only elements that satisfy the predicate function.
  ```
  filter(x => x > 3, [1, 5, 2, 4]) // [5, 4]
  ```
- `reduce(func, initial, vector)`: Reduces a vector to a single value.
  ```
  reduce((acc, x) => acc + x, 0, [1, 2, 3, 4]) // 10
  ```
- `pipe(f1, f2, ..., initial_value)`: Passes the initial value through a pipeline of functions from left to right.
  ```
  let double = x => x * 2
  let add_one = x => x + 1
  pipe(double, add_one, 5) // add_one(double(5)) => 11
  ```

---

## Examples

### Simple Arithmetic

```
eval> 2 + 3 * (4 - 1)
"11"
```

### Vector Operations

```
eval> let v = [1, 2, 3, 4]
eval> map(x => x^2, v)
"[1, 4, 9, 16]"
```

### Defining and Using a Function

```
eval> let hypot = (a, b) => sqrt(a^2 + b^2)
eval> hypot(3, 4)
"5"
```

### DSP Signal Processing Chain

```
// 1. Create a time vector
eval> let t = linspace(0, 1, 1024)

// 2. Create a signal
eval> let signal = map(x => sin(2 * PI * 50 * x), t)

// 3. Apply a window
eval> let windowed = signal * hanning(1024)

// 4. Get the FFT magnitude
eval> let spectrum = fft_mag(windowed)
```