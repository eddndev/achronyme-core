# 🚀 Phase 2: Mathematical Functions - Implementation Plan

**Status**: 🔄 In Progress
**Start Date**: 2025-10-26
**Estimated Duration**: 2-3 hours

---

## 🎯 Objectives

Extend the arithmetic evaluator to support:
1. Mathematical constants (PI, E, PHI, etc.)
2. Trigonometric functions (sin, cos, tan, asin, acos, atan)
3. Exponential/logarithmic functions (exp, log, ln, log10, log2)
4. Other mathematical functions (sqrt, abs, floor, ceil, round, min, max)

---

## 📋 Implementation Tasks

### 1. **Constants** (30 min)

#### 1.1 Create Constants Registry
**File**: `wasm/src/core/constants.hpp`

```cpp
namespace achronyme::core::constants {
    constexpr double PI = 3.141592653589793;
    constexpr double E = 2.718281828459045;
    constexpr double PHI = 1.618033988749895;  // Golden ratio
    constexpr double SQRT2 = 1.414213562373095;
    constexpr double SQRT3 = 1.732050807568877;
    constexpr double LN2 = 0.693147180559945;
    constexpr double LN10 = 2.302585092994046;
}
```

#### 1.2 Extend Lexer for Identifiers
- Add `IDENTIFIER` token type
- Recognize letter sequences: `[a-zA-Z_][a-zA-Z0-9_]*`
- Case-insensitive matching (PI = pi = Pi)

#### 1.3 Extend Parser for Constants
- In `primary()`, check if IDENTIFIER is a constant
- Return constant value directly

**Example**:
```
"2 * PI" → AST with NUMBER(2) * NUMBER(3.14159...)
```

---

### 2. **Functions** (1.5 hours)

#### 2.1 Create Function Registry
**File**: `wasm/src/core/functions.hpp`

```cpp
namespace achronyme::core {

using MathFunction = std::function<Value(const std::vector<Value>&)>;

class FunctionRegistry {
public:
    static FunctionRegistry& instance();

    void registerFunction(const std::string& name, MathFunction func, int arity);
    bool hasFunction(const std::string& name) const;
    MathFunction getFunction(const std::string& name) const;
    int getArity(const std::string& name) const;

private:
    std::unordered_map<std::string, std::pair<MathFunction, int>> functions_;
};

} // namespace achronyme::core
```

#### 2.2 Implement Mathematical Functions
**File**: `wasm/src/core/functions.cpp`

**Categories**:

1. **Trigonometric** (radians):
   - `sin(x)`, `cos(x)`, `tan(x)`
   - `asin(x)`, `acos(x)`, `atan(x)`, `atan2(y, x)`
   - `sinh(x)`, `cosh(x)`, `tanh(x)`

2. **Exponential/Logarithmic**:
   - `exp(x)` - e^x
   - `log(x)` - natural log (ln)
   - `log10(x)` - base 10
   - `log2(x)` - base 2

3. **Power/Root**:
   - `sqrt(x)` - square root
   - `cbrt(x)` - cube root
   - `pow(x, y)` - x^y (already have ^, but as function)

4. **Rounding**:
   - `floor(x)` - round down
   - `ceil(x)` - round up
   - `round(x)` - round to nearest
   - `trunc(x)` - truncate decimals

5. **Other**:
   - `abs(x)` - absolute value
   - `sign(x)` - sign (-1, 0, 1)
   - `min(x, y, ...)` - minimum
   - `max(x, y, ...)` - maximum

#### 2.3 Extend AST for Function Calls
**File**: `wasm/src/parser/ast.hpp`

```cpp
enum class ASTNodeType {
    NUMBER,
    BINARY_OP,
    UNARY_OP,
    FUNCTION_CALL  // ← NEW
};

class FunctionCallNode : public ASTNode {
public:
    FunctionCallNode(std::string name, std::vector<std::unique_ptr<ASTNode>> args)
        : name_(std::move(name)), args_(std::move(args)) {}

    ASTNodeType type() const override { return ASTNodeType::FUNCTION_CALL; }
    const std::string& name() const { return name_; }
    const std::vector<std::unique_ptr<ASTNode>>& args() const { return args_; }

private:
    std::string name_;
    std::vector<std::unique_ptr<ASTNode>> args_;
};
```

#### 2.4 Extend Parser for Function Calls
**File**: `wasm/src/parser/parser.cpp`

Modify `primary()`:
```cpp
std::unique_ptr<ASTNode> Parser::primary() {
    // ... existing NUMBER and LPAREN handling ...

    // NEW: Handle identifiers (constants or function calls)
    if (match(TokenType::IDENTIFIER)) {
        std::string name = previous().lexeme;

        // Check if it's a function call
        if (check(TokenType::LPAREN)) {
            return parseFunctionCall(name);
        }

        // Otherwise, it's a constant
        return parseConstant(name);
    }

    throw std::runtime_error("Expected expression");
}

std::unique_ptr<ASTNode> Parser::parseFunctionCall(const std::string& name) {
    consume(TokenType::LPAREN, "Expected '(' after function name");

    std::vector<std::unique_ptr<ASTNode>> args;

    if (!check(TokenType::RPAREN)) {
        do {
            args.push_back(expression());
        } while (match(TokenType::COMMA));
    }

    consume(TokenType::RPAREN, "Expected ')' after arguments");

    return std::make_unique<FunctionCallNode>(name, std::move(args));
}
```

#### 2.5 Extend Evaluator for Function Calls
**File**: `wasm/src/parser/evaluator.cpp`

```cpp
Value Evaluator::evaluate(const ASTNode* node) {
    switch (node->type()) {
        // ... existing cases ...

        case ASTNodeType::FUNCTION_CALL:
            return evaluateFunctionCall(static_cast<const FunctionCallNode*>(node));
    }
}

Value Evaluator::evaluateFunctionCall(const FunctionCallNode* node) {
    auto& registry = FunctionRegistry::instance();

    if (!registry.hasFunction(node->name())) {
        throw std::runtime_error("Unknown function: " + node->name());
    }

    // Evaluate arguments
    std::vector<Value> args;
    for (const auto& argNode : node->args()) {
        args.push_back(evaluate(argNode.get()));
    }

    // Check arity
    int expectedArity = registry.getArity(node->name());
    if (expectedArity >= 0 && args.size() != expectedArity) {
        throw std::runtime_error("Function " + node->name() +
                               " expects " + std::to_string(expectedArity) +
                               " arguments, got " + std::to_string(args.size()));
    }

    // Call function
    auto func = registry.getFunction(node->name());
    return func(args);
}
```

---

### 3. **Extended Demo** (20 min)

Add complex expressions to `demo.mjs`:

```javascript
// Phase 2: Functions and Constants
{ expr: 'sin(PI/2)', desc: 'Trigonometric (sin of 90°)' },
{ expr: 'cos(0)', desc: 'Trigonometric (cos of 0°)' },
{ expr: 'tan(PI/4)', desc: 'Trigonometric (tan of 45°)' },
{ expr: 'exp(1)', desc: 'Exponential (e^1)' },
{ expr: 'log(E)', desc: 'Natural log of e' },
{ expr: 'sqrt(16)', desc: 'Square root' },
{ expr: 'abs(-5)', desc: 'Absolute value' },
{ expr: 'floor(3.7)', desc: 'Floor function' },
{ expr: 'ceil(3.2)', desc: 'Ceiling function' },
{ expr: 'round(3.5)', desc: 'Rounding' },
{ expr: 'min(5, 3, 8, 1)', desc: 'Minimum of values' },
{ expr: 'max(5, 3, 8, 1)', desc: 'Maximum of values' },

// Complex expressions
{ expr: 'sin(PI/6) + cos(PI/3)', desc: 'Trig combination' },
{ expr: 'sqrt(abs(-16))', desc: 'Nested functions' },
{ expr: 'log(exp(5))', desc: 'Inverse functions' },
{ expr: '2 * PI * sqrt(9.8 / 0.5)', desc: 'Physics formula' },
{ expr: 'abs(sin(PI/4)) ^ 2 + abs(cos(PI/4)) ^ 2', desc: 'Pythagorean identity' },
```

---

### 4. **Testing** (30 min)

#### 4.1 C++ Tests
**File**: `wasm/tests/test_functions.cpp`

```cpp
TEST(FunctionsTest, Constants) {
    EXPECT_NEAR(eval("PI"), 3.14159, 1e-5);
    EXPECT_NEAR(eval("E"), 2.71828, 1e-5);
    EXPECT_NEAR(eval("2 * PI"), 6.28318, 1e-5);
}

TEST(FunctionsTest, Trigonometric) {
    EXPECT_NEAR(eval("sin(0)"), 0.0, 1e-10);
    EXPECT_NEAR(eval("sin(PI/2)"), 1.0, 1e-10);
    EXPECT_NEAR(eval("cos(0)"), 1.0, 1e-10);
    EXPECT_NEAR(eval("tan(PI/4)"), 1.0, 1e-10);
}

TEST(FunctionsTest, Exponential) {
    EXPECT_NEAR(eval("exp(0)"), 1.0, 1e-10);
    EXPECT_NEAR(eval("exp(1)"), 2.71828, 1e-5);
    EXPECT_NEAR(eval("log(E)"), 1.0, 1e-10);
}

TEST(FunctionsTest, Other) {
    EXPECT_DOUBLE_EQ(eval("sqrt(16)"), 4.0);
    EXPECT_DOUBLE_EQ(eval("abs(-5)"), 5.0);
    EXPECT_DOUBLE_EQ(eval("floor(3.7)"), 3.0);
    EXPECT_DOUBLE_EQ(eval("ceil(3.2)"), 4.0);
}

TEST(FunctionsTest, Nested) {
    EXPECT_NEAR(eval("sqrt(abs(-16))"), 4.0, 1e-10);
    EXPECT_NEAR(eval("log(exp(5))"), 5.0, 1e-10);
}

TEST(FunctionsTest, MultipleArgs) {
    EXPECT_DOUBLE_EQ(eval("min(5, 3, 8, 1)"), 1.0);
    EXPECT_DOUBLE_EQ(eval("max(5, 3, 8, 1)"), 8.0);
}
```

---

## 📊 Expected Results

After Phase 2, the calculator will support:

```javascript
// Constants
soc.eval('PI')                          // 3.14159...
soc.eval('E')                           // 2.71828...
soc.eval('2 * PI')                      // 6.28318...

// Trigonometric
soc.eval('sin(PI/2)')                   // 1
soc.eval('cos(0)')                      // 1
soc.eval('tan(PI/4)')                   // 1

// Exponential/Logarithmic
soc.eval('exp(1)')                      // 2.71828...
soc.eval('log(E)')                      // 1
soc.eval('sqrt(16)')                    // 4

// Rounding
soc.eval('floor(3.7)')                  // 3
soc.eval('ceil(3.2)')                   // 4
soc.eval('round(3.5)')                  // 4

// Other
soc.eval('abs(-5)')                     // 5
soc.eval('min(5, 3, 8, 1)')            // 1
soc.eval('max(5, 3, 8, 1)')            // 8

// Complex expressions
soc.eval('sqrt(abs(-16))')              // 4
soc.eval('log(exp(5))')                 // 5
soc.eval('sin(PI/6) + cos(PI/3)')      // 1
soc.eval('abs(sin(PI/4))^2 + abs(cos(PI/4))^2')  // 1 (Pythagorean)
```

---

## 🎯 Success Criteria

- [x] All constants accessible by name
- [x] 20+ mathematical functions implemented
- [x] Function calls with 0-N arguments
- [x] Nested function calls work
- [x] All tests pass
- [x] Demo shows complex expressions

---

## 📈 Timeline

| Task | Duration | Status |
|------|----------|--------|
| 1. Constants | 30 min | ⏳ Pending |
| 2. Lexer extension | 20 min | ⏳ Pending |
| 3. Parser extension | 30 min | ⏳ Pending |
| 4. Function registry | 20 min | ⏳ Pending |
| 5. Function implementations | 40 min | ⏳ Pending |
| 6. Evaluator extension | 20 min | ⏳ Pending |
| 7. Tests | 30 min | ⏳ Pending |
| 8. Demo update | 20 min | ⏳ Pending |
| 9. Compilation & testing | 20 min | ⏳ Pending |
| **Total** | **3.5 hours** | |

---

**Let's build it! 🚀**
