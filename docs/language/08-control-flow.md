# Control Flow

Achronyme uses **functional control flow** with `if()` and `piecewise()` functions instead of traditional statements.

## The if() Function

### Basic Syntax

```javascript
if(condition, then_value, else_value)
```

The `if()` function takes **three arguments**:
1. **condition**: Boolean expression to evaluate
2. **then_value**: Value returned if condition is true
3. **else_value**: Value returned if condition is false

### Simple Examples

```javascript
// Basic if
let result = if(5 > 3, 100, 200)    // 100

// With variables
let x = 10
let category = if(x > 5, 1, 0)      // 1

// Negative numbers
let value = if(-5 < 0, true, false)  // true
```

### if() Returns a Value

Since `if()` is a function, it always returns a value:

```javascript
// Assign result directly
let status = if(score >= 60, "pass", "fail")

// Use in expressions
let total = 10 + if(bonus, 5, 0)

// Return from lambda
let classify = x => if(x < 0, "negative", "positive")
```

### Mathematical Functions with if()

```javascript
// Absolute value
let abs = x => if(x < 0, -x, x)
abs(-5)   // 5
abs(3)    // 3

// Maximum of two numbers
let max = (a, b) => if(a > b, a, b)
max(10, 5)   // 10

// Minimum of two numbers
let min = (a, b) => if(a < b, a, b)
min(10, 5)   // 5
```

### Activation Functions (ML)

```javascript
// ReLU (Rectified Linear Unit)
let relu = x => if(x > 0, x, 0)
relu(5)    // 5
relu(-3)   // 0

// Leaky ReLU
let leaky_relu = x => if(x > 0, x, 0.01 * x)
leaky_relu(5)    // 5
leaky_relu(-10)  // -0.1

// Heaviside step function
let heaviside = x => if(x >= 0, 1, 0)
heaviside(5)   // 1
heaviside(-2)  // 0
```

### Nested if() Functions

For multiple conditions, nest `if()` calls:

```javascript
// Sign function (-1, 0, or 1)
let sign = x => if(x < 0, -1, if(x > 0, 1, 0))
sign(-10)  // -1
sign(10)   // 1
sign(0)    // 0

// Grading system (4=A, 3=B, 2=C, 1=D, 0=F)
let grade = score => if(score >= 90,
                        4,
                        if(score >= 80,
                           3,
                           if(score >= 70,
                              2,
                              if(score >= 60, 1, 0))))
grade(95)  // 4 (A)
grade(75)  // 2 (C)
grade(50)  // 0 (F)
```

### Complex Conditions

Use logical operators (`&&`, `||`, `!`) for complex conditions:

```javascript
// Check if value is in range [min, max]
let in_range = (x, min, max) => if(x >= min && x <= max, true, false)
in_range(5, 0, 10)    // true
in_range(15, 0, 10)   // false

// XOR (exclusive or)
let xor = (a, b) => if((a || b) && !(a && b), true, false)
xor(true, false)   // true
xor(true, true)    // false
```

### if() with Higher-Order Functions

```javascript
// Filter positive numbers
let positives = v => filter(x => x > 0, v)
positives([1, -2, 3, -4, 5])  // [1, 3, 5]

// Clamp values to range
let clamp = (v, min_val, max_val) =>
    map(x => if(x < min_val,
                min_val,
                if(x > max_val, max_val, x)), v)
clamp([1, 5, 10, 15, 20], 5, 15)  // [5, 5, 10, 15, 15]

// Apply ReLU to vector
let relu_vec = v => map(x => if(x > 0, x, 0), v)
relu_vec([1, -2, 3, -4, 5])  // [1, 0, 3, 0, 5]
```

## The piecewise() Function

For **3 or more conditions**, `piecewise()` is cleaner than nested `if()` calls.

### Basic Syntax

```javascript
piecewise(
    [condition1, value1],
    [condition2, value2],
    [condition3, value3],
    default_value
)
```

- Each condition is a `[boolean_expr, value]` pair
- Conditions are evaluated **sequentially** (first match wins)
- The last argument (without brackets) is the **default** value
- Default is **optional** - error if no match and no default

### Simple Examples

```javascript
// Sign function
let sign = x => piecewise(
    [x < 0, -1],
    [x > 0, 1],
    0
)
sign(-10)  // -1
sign(5)    // 1
sign(0)    // 0

// Absolute value
let abs = x => piecewise([x < 0, -x], x)
abs(-5)  // 5
abs(3)   // 3
```

### Activation Functions with piecewise()

```javascript
// ReLU
let relu = x => piecewise([x > 0, x], 0)
relu(-3)  // 0
relu(5)   // 5

// Leaky ReLU
let leaky_relu = x => piecewise([x > 0, x], 0.01 * x)
leaky_relu(10)   // 10
leaky_relu(-10)  // -0.1

// Heaviside step function
let heaviside = x => piecewise([x < 0, 0], 1)
heaviside(-5)  // 0
heaviside(0)   // 1
heaviside(5)   // 1
```

### Multi-Branch Examples

```javascript
// Progressive tax brackets
let tax = income => piecewise(
    [income <= 10000, income * 0.1],
    [income <= 50000, income * 0.2],
    income * 0.3
)
tax(5000)    // 500 (10%)
tax(30000)   // 6000 (20%)
tax(100000)  // 30000 (30%)

// Grading system
let grade = score => piecewise(
    [score >= 90, 5],
    [score >= 80, 4],
    [score >= 70, 3],
    [score >= 60, 2],
    1
)
grade(95)  // 5
grade(75)  // 3
grade(50)  // 1
```

### Piecewise Functions (Mathematics)

```javascript
// f(x) = { x^2      if x < -1
//        { 2x + 1   if -1 <= x < 1
//        { x^3      if x >= 1

let f = x => piecewise(
    [x < -1, x^2],
    [x < 1, 2*x + 1],
    x^3
)
f(-2)   // 4 (from x^2)
f(0)    // 1 (from 2x+1)
f(2)    // 8 (from x^3)
```

### Multi-Variable piecewise()

```javascript
// Classify points in 2D plane
let region = (x, y) => piecewise(
    [x^2 + y^2 < 1, 1],           // Inside circle
    [abs(x) < 2 && abs(y) < 2, 2], // In square, outside circle
    0                              // Outside both
)
region(0, 0)      // 1 (inside circle)
region(1.5, 0)    // 2 (in square)
region(3, 3)      // 0 (outside)
```

### piecewise() with Higher-Order Functions

```javascript
// Classify numbers
let classify = x => piecewise(
    [x < 0, -1],
    [x > 0, 1],
    0
)
map(classify, [-5, -2, 0, 3, 7])  // [-1, -1, 0, 1, 1]

// Apply tax function to incomes
let incomes = [5000, 30000, 100000]
map(tax, incomes)  // [500, 6000, 30000]
```

## if() vs piecewise()

### When to Use if()

- **2 branches** (condition + else)
- Simple true/false decisions
- More concise for binary choices

```javascript
// Good use of if()
let abs = x => if(x < 0, -x, x)
let max = (a, b) => if(a > b, a, b)
```

### When to Use piecewise()

- **3+ branches**
- Sequential condition checking
- More readable than nested `if()`

```javascript
// Bad: nested if() for multiple conditions
let grade = score => if(score >= 90, 5,
                       if(score >= 80, 4,
                          if(score >= 70, 3,
                             if(score >= 60, 2, 1))))

// Good: piecewise() for multiple conditions
let grade = score => piecewise(
    [score >= 90, 5],
    [score >= 80, 4],
    [score >= 70, 3],
    [score >= 60, 2],
    1
)
```

## Sequential Evaluation (Short-Circuit)

Both `if()` and `piecewise()` use **short-circuit evaluation**:

```javascript
// Only evaluates the branch that's taken
let safe_divide = (a, b) => if(b == 0, 0, a / b)
safe_divide(10, 0)  // 0 (doesn't evaluate a/b)

// piecewise: first true condition wins
let classify = x => piecewise(
    [x >= 90, "A"],  // Checked first
    [x >= 80, "B"],  // Only if x < 90
    [x >= 70, "C"],  // Only if x < 80
    "F"
)
```

## No Default Value (Error)

If no condition matches and no default is provided, **error**:

```javascript
// This will error if x is outside [0, 2)
let partial = x => piecewise(
    [x >= 0 && x < 1, x^2],
    [x >= 1 && x < 2, 2*x - 1]
)
// partial(-1)  // ERROR: no condition matched

// Fix: add default
let partial_fixed = x => piecewise(
    [x >= 0 && x < 1, x^2],
    [x >= 1 && x < 2, 2*x - 1],
    0  // default value
)
partial_fixed(-1)  // 0 (uses default)
```

## Common Patterns

### Indicator/Characteristic Function

```javascript
let indicator = (x, a, b) => if(x >= a && x <= b, 1, 0)
indicator(5, 0, 10)   // 1
indicator(15, 0, 10)  // 0
```

### Clipping/Clamping

```javascript
let clip = (x, min, max) => if(x < min, min, if(x > max, max, x))
clip(5, 0, 10)    // 5
clip(-5, 0, 10)   // 0
clip(15, 0, 10)   // 10
```

### Comparison Selection

```javascript
let select_by_condition = (cond, a, b) => if(cond, a, b)
select_by_condition(true, 100, 200)   // 100
select_by_condition(false, 100, 200)  // 200
```

## Best Practices

### 1. Use Appropriate Function

```javascript
// Good: if() for 2 branches
let sign_bit = x => if(x >= 0, 0, 1)

// Good: piecewise() for 3+ branches
let grade = score => piecewise(
    [score >= 90, "A"],
    [score >= 80, "B"],
    [score >= 70, "C"],
    "F"
)
```

### 2. Order Conditions Correctly

```javascript
// Correct: most restrictive first
piecewise(
    [x >= 90, "A"],  // Must be first
    [x >= 80, "B"],
    [x >= 70, "C"],
    "F"
)

// Wrong: will always return "F" for x < 70
piecewise(
    [x >= 70, "C"],
    [x >= 80, "B"],  // Never reached!
    [x >= 90, "A"],  // Never reached!
    "F"
)
```

### 3. Always Provide Default (When Possible)

```javascript
// Good: has default
piecewise([x > 0, 1], [x < 0, -1], 0)

// Risky: no default (error if conditions don't cover all cases)
piecewise([x > 0, 1], [x < 0, -1])  // Error when x == 0!
```

### 4. Keep Conditions Simple

```javascript
// Good: clear conditions
let category = x => piecewise(
    [x < 0, "negative"],
    [x > 0, "positive"],
    "zero"
)

// Avoid: overly complex conditions
let category = x => piecewise(
    [x < 0 && x > -100 && (x % 2 == 0 || x % 3 == 0), "complex"],
    [x > 0, "positive"],
    "other"
)
```

## Summary

| Feature | `if()` | `piecewise()` |
|---------|--------|---------------|
| Arguments | 3 (condition, then, else) | Variable ([cond, val], ..., default) |
| Best for | 2 branches | 3+ branches |
| Syntax | `if(cond, then, else)` | `piecewise([c1, v1], [c2, v2], default)` |
| Nesting | Gets messy with 3+ conditions | Clean for many conditions |
| Default | Always has else | Optional (error if missing) |
| Evaluation | Short-circuit | Sequential, first match wins |

**Key Points**:
- ✅ `if()` and `piecewise()` are **functions**, not statements
- ✅ They **return values** (can be used in expressions)
- ✅ Use `if()` for **2-way** decisions
- ✅ Use `piecewise()` for **3+ way** decisions
- ✅ Both support **short-circuit evaluation**
- ✅ Always provide a **default** in `piecewise()` when possible

---

**Next**: [Arrays and Tensors](09-arrays-tensors.md)
