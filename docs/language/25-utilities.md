# Utility Functions

Utility functions provide essential capabilities for output, type inspection, and value formatting in Achronyme.

## Overview

The utility module includes three fundamental functions:
- **`print()`** - Output values to console
- **`type()`** - Inspect value types at runtime
- **`str()`** - Convert values to string representation

All utility functions are available as first-class values and can be passed to higher-order functions.

---

## Output Functions

### print - Print Values to Console

Prints one or more values to standard output, separated by spaces.

**Signature:** `print(value1, value2, ...) -> last_value`

**Parameters:**
- Variadic: Accepts 1 or more arguments of any type
- All values are formatted using the same rules as `str()`

**Returns:** The last value that was printed (useful for chaining in pipelines)

**Behavior:**
- Values are printed separated by single spaces
- A newline is always added at the end
- Numbers: Integers without decimal point (42 instead of 42.0)
- Strings: Printed as-is (without quotes)
- Complex: Formatted as "a+bi" or "a-bi"
- Vectors/Tensors: Formatted as "[1, 2, 3]"
- Records: Formatted as "{key: value, ...}"
- Functions: Shown as "<function:name>" or "<function:lambda>"

**Examples:**

```javascript
// Print a single value
print(42)
// Output: 42

// Print multiple values
print("The answer is", 42)
// Output: The answer is 42

// Print expressions
print(2 + 2, 3 * 3)
// Output: 4 9

// Print arrays
print([1, 2, 3])
// Output: [1, 2, 3]

// Print in pipeline (returns the printed value)
let result = pipe(
    5,
    x => x * 2,
    x => print(x),    // Prints 10 and returns 10
    x => x + 10
)
// Output: 10
// result = 20
```

**Use Cases:**
- Debugging: Print intermediate values in calculations
- Logging: Display computation progress
- Pipelines: Inspect values while maintaining flow

**Error Conditions:**
- Calling with no arguments raises an error: "print() requires at least 1 argument"

---

## Type Inspection Functions

### type - Get Value Type

Returns the type name of a value as a string.

**Signature:** `type(value) -> String`

**Parameters:**
- `value`: Any value

**Returns:** String representing the type name

**Type Names:**
- `"Number"` - Floating-point numbers
- `"Boolean"` - true/false values
- `"String"` - Text values
- `"Complex"` - Complex numbers (a + bi)
- `"Vector"` - Heterogeneous arrays
- `"Tensor"` - Numeric arrays
- `"ComplexTensor"` - Complex number arrays
- `"Function"` - Functions (builtin or lambda)
- `"Record"` - Key-value records
- `"Edge"` - Graph edges
- `"TailCall"` - Tail call optimization wrapper
- `"MutableRef<T>"` - Mutable references (shows inner type)

**Examples:**

```javascript
// Basic types
type(42)          // "Number"
type(3.14)        // "Number"
type(true)        // "Boolean"
type("hello")     // "String"

// Complex and collections
type(3 + 4i)      // "Complex"
type([1, 2, 3])   // "Tensor" (numeric array)
type(["a", "b"])  // "Vector" (mixed array)

// Functions and records
type(x => x + 1)  // "Function"
type(sin)         // "Function"
type({x: 10})     // "Record"

// Type checking pattern
let x = 42;
if type(x) == "Number" {
    print("x is a number")
}

// Use with map for type analysis
let values = [42, "hello", true, [1, 2, 3]];
let types = map(x => type(x), values);
// types = ["Number", "String", "Boolean", "Tensor"]

// Combining with other functions
let data = [1, "2", 3];
let numbers = filter(x => type(x) == "Number", data);
// numbers = [1, 3]
```

**Use Cases:**
- Runtime type checking
- Debugging: Verify expected types
- Type-based filtering and processing
- Dynamic dispatch based on type

---

## String Conversion Functions

### str - Convert to String

Converts any value to its string representation.

**Signature:** `str(value) -> String`

**Parameters:**
- `value`: Any value to convert

**Returns:** String representation of the value

**Formatting Rules:**

| Type | Format | Example Input | Example Output |
|------|--------|---------------|----------------|
| Number (integer) | No decimal point | `42.0` | `"42"` |
| Number (float) | With decimals | `3.14` | `"3.14"` |
| Boolean | true/false | `true` | `"true"` |
| String | As-is | `"hello"` | `"hello"` |
| Complex | a+bi or a-bi | `3 + 4i` | `"3+4i"` |
| Vector | [val1, val2, ...] | `["a", "b"]` | `"[a, b]"` |
| Tensor (1D) | [num1, num2, ...] | `[1, 2, 3]` | `"[1, 2, 3]"` |
| Tensor (multi-D) | TensorShape | 2×3 tensor | `"Tensor[2, 3]"` |
| Function (builtin) | <function:name> | `sin` | `"<function:sin>"` |
| Function (lambda) | <function:lambda> | `x => x + 1` | `"<function:lambda>"` |
| Record | {key: val, ...} | `{x: 10}` | `"{x: 10}"` |
| MutableRef | mut value | `mut 42` | `"mut 42"` |

**Examples:**

```javascript
// Numbers
str(42)           // "42"
str(3.14)         // "3.14"

// Booleans
str(true)         // "true"
str(false)        // "false"

// Strings (no change)
str("hello")      // "hello"

// Complex numbers
str(3 + 4i)       // "3+4i"
str(3 - 4i)       // "3-4i"

// Arrays
str([1, 2, 3])    // "[1, 2, 3]"
str(["a", "b"])   // "[a, b]"

// Records
str({x: 10, y: 20})  // "{x: 10, y: 20}" (order may vary)

// String concatenation
"The answer is " + str(42)
// "The answer is 42"

// Format numbers for display
let pi = 3.14159;
print("π ≈", str(pi))
// Output: π ≈ 3.14159

// Convert and join
let numbers = [1, 2, 3, 4, 5];
let strings = map(n => str(n), numbers);
let result = join(strings, ", ");
// result = "1, 2, 3, 4, 5"

// Debugging with str
let data = {name: "Alice", age: 30};
print("Data:", str(data));
// Output: Data: {name: Alice, age: 30}
```

**Use Cases:**
- String concatenation with non-string values
- Formatting output for display
- Converting numeric results to text
- Debugging: Display complex data structures

---

## Practical Examples

### Debugging Pipeline

Use `print()` and `type()` together to debug data transformations:

```javascript
let result = pipe(
    [1, 2, 3, 4, 5],
    arr => {
        print("Input:", str(arr));
        print("Type:", type(arr));
        arr
    },
    arr => map(x => x * 2, arr),
    arr => {
        print("After doubling:", str(arr));
        arr
    },
    arr => filter(x => x > 5, arr),
    arr => {
        print("After filter:", str(arr));
        arr
    }
)
// Output:
// Input: [1, 2, 3, 4, 5]
// Type: Tensor
// After doubling: [2, 4, 6, 8, 10]
// After filter: [6, 8, 10]
```

### Type-Safe Operations

Check types before performing operations:

```javascript
let safe_divide = (a, b) => do {
    if type(a) != "Number" || type(b) != "Number" {
        print("Error: Both arguments must be numbers");
        print("Got:", type(a), "and", type(b));
        0
    } else if b == 0 {
        print("Error: Division by zero");
        0
    } else {
        a / b
    }
};

safe_divide(10, 2)        // 5
safe_divide(10, "2")      // Error: Both arguments must be numbers
```

### Dynamic Formatting

Format different types appropriately:

```javascript
let format_value = (label, value) => do {
    let t = type(value);
    if t == "Number" {
        label + ": " + str(value)
    } else if t == "String" {
        label + ": \"" + value + "\""
    } else if t == "Tensor" || t == "Vector" {
        label + " (array): " + str(value)
    } else {
        label + " (" + t + "): " + str(value)
    }
};

print(format_value("Count", 42));
// Output: Count: 42

print(format_value("Name", "Alice"));
// Output: Name: "Alice"

print(format_value("Values", [1, 2, 3]));
// Output: Values (array): [1, 2, 3]
```

### Introspection Example

Analyze a collection of mixed-type values:

```javascript
let analyze = (arr) => do {
    print("Analyzing", len(arr), "values:");

    let types = map(x => type(x), arr);
    let unique_types = reduce((acc, t) => {
        if contains(acc, t) { acc } else { acc + [t] }
    }, [], types);

    print("Unique types:", str(unique_types));

    // Count each type
    map(t => {
        let count = count(arr, x => type(x) == t);
        print("  -", t + ":", count);
    }, unique_types);

    arr
};

analyze([1, "hello", 2, true, [3, 4], "world", 5])
// Output:
// Analyzing 7 values:
// Unique types: ["Number", "String", "Boolean", "Tensor"]
//   - Number: 3
//   - String: 2
//   - Boolean: 1
//   - Tensor: 1
```

---

## Implementation Details

### Function Registration

All utility functions are registered in the function registry:

- `print` - Variadic function (arity: -1, accepts 1+ arguments)
- `type` - Unary function (arity: 1)
- `str` - Unary function (arity: 1)

### Module Location

Implementation: `crates/achronyme-eval/src/function_modules/utils.rs`

### First-Class Functions

All utilities are first-class values:

```javascript
// Assign to variables
let output = print;
output("hello");  // Prints: hello

// Pass to HOFs
let arr = [1, 2, 3];
map(print, arr);  // Prints each element

// Store in records
let utils = {
    log: print,
    typeof: type,
    format: str
};
utils.log("Using record function");
```

---

## See Also

- [Data Types](03-data-types.md) - Understanding Achronyme's type system
- [Strings](20-strings.md) - String manipulation functions
- [Higher-Order Functions](11-higher-order-functions.md) - Using utilities with map, filter, etc.
- [Control Flow](08-control-flow.md) - Using type checking in conditionals
