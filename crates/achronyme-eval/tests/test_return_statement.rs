/// Integration tests for return statement (early return)
///
/// Tests the new return statement that allows early exit from functions:
/// - return expr: Returns immediately with the given value
/// - Works in do blocks, if expressions, and nested structures
/// - Avoids deeply nested else clauses

use achronyme_eval::Evaluator;
use achronyme_types::value::Value;

fn eval(source: &str) -> Result<Value, String> {
    let mut evaluator = Evaluator::new();
    evaluator.eval_str(source)
}

// ============================================================================
// Basic Return Statement Tests
// ============================================================================

#[test]
fn test_simple_return() {
    let result = eval(r#"
        let f = () => do {
            return 42
        }
        f()
    "#).unwrap();
    assert_eq!(result, Value::Number(42.0));
}

#[test]
fn test_return_string() {
    let result = eval(r#"
        let greet = () => do {
            return "Hello"
        }
        greet()
    "#).unwrap();
    assert_eq!(result, Value::String("Hello".to_string()));
}

#[test]
fn test_return_with_computation() {
    let result = eval(r#"
        let compute = (x) => do {
            let result = x * 2 + 10
            return result
        }
        compute(5)
    "#).unwrap();
    assert_eq!(result, Value::Number(20.0));
}

// ============================================================================
// Early Return (Skip Remaining Code)
// ============================================================================

#[test]
fn test_early_return_skips_code() {
    let result = eval(r#"
        let f = () => do {
            return 10
            42  // This should never execute
        }
        f()
    "#).unwrap();
    assert_eq!(result, Value::Number(10.0));
}

#[test]
fn test_early_return_in_sequence() {
    let result = eval(r#"
        let f = () => do {
            let a = 5
            return a * 2
            let b = 100  // Never executed
            b
        }
        f()
    "#).unwrap();
    assert_eq!(result, Value::Number(10.0));
}

// ============================================================================
// Return in If Expressions
// ============================================================================

#[test]
fn test_return_in_if_true_branch() {
    let result = eval(r#"
        let validate = (x) => do {
            if(x < 0) {
                return "error: negative"
            }
            "ok"
        }
        validate(-5)
    "#).unwrap();
    assert_eq!(result, Value::String("error: negative".to_string()));
}

#[test]
fn test_return_in_if_false_continues() {
    let result = eval(r#"
        let validate = (x) => do {
            if(x < 0) {
                return "error"
            }
            "ok"
        }
        validate(10)
    "#).unwrap();
    assert_eq!(result, Value::String("ok".to_string()));
}

#[test]
fn test_multiple_early_returns() {
    let result = eval(r#"
        let classify = (x) => do {
            if(x < 0) {
                return "negative"
            }
            if(x == 0) {
                return "zero"
            }
            if(x > 100) {
                return "large"
            }
            "normal"
        }
        classify(0)
    "#).unwrap();
    assert_eq!(result, Value::String("zero".to_string()));
}

#[test]
fn test_multiple_early_returns_last_case() {
    let result = eval(r#"
        let classify = (x) => do {
            if(x < 0) {
                return "negative"
            }
            if(x == 0) {
                return "zero"
            }
            if(x > 100) {
                return "large"
            }
            "normal"
        }
        classify(50)
    "#).unwrap();
    assert_eq!(result, Value::String("normal".to_string()));
}

// ============================================================================
// Return with Different Types
// ============================================================================

#[test]
fn test_return_boolean() {
    let result = eval(r#"
        let isPositive = (x) => do {
            if(x <= 0) {
                return false
            }
            return true
        }
        isPositive(10)
    "#).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_return_array() {
    let result = eval(r#"
        let getRange = (n) => do {
            if(n <= 0) {
                return []
            }
            return [1, 2, 3]
        }
        getRange(5)
    "#).unwrap();
    assert!(matches!(result, Value::Vector(_) | Value::Tensor(_)));
}

#[test]
fn test_return_record() {
    let result = eval(r#"
        let makeError = (msg) => do {
            return {error: msg, code: 400}
        }
        makeError("bad request")
    "#).unwrap();
    assert!(matches!(result, Value::Record(_)));
}

#[test]
fn test_return_function() {
    let result = eval(r#"
        let makeAdder = (x) => do {
            return y => x + y
        }
        let add5 = makeAdder(5)
        add5(10)
    "#).unwrap();
    assert_eq!(result, Value::Number(15.0));
}

// ============================================================================
// Nested Returns
// ============================================================================

#[test]
fn test_return_in_nested_if() {
    let result = eval(r#"
        let process = (x, y) => do {
            if(x > 0) {
                if(y > 0) {
                    return "both positive"
                }
                return "x positive, y not"
            }
            "x not positive"
        }
        process(5, 10)
    "#).unwrap();
    assert_eq!(result, Value::String("both positive".to_string()));
}

#[test]
fn test_return_in_nested_do_block() {
    // return propagates through nested do blocks and exits the function
    let result = eval(r#"
        let outer = () => do {
            let result = do {
                let x = 10
                if(x > 5) {
                    return 100  // Exits outer(), not just the inner do block
                }
                x
            }
            result + 50  // This line is NOT reached
        }
        outer()
    "#).unwrap();
    assert_eq!(result, Value::Number(100.0));  // return exits the entire function
}

// ============================================================================
// Return with Parameters
// ============================================================================

#[test]
fn test_return_with_single_parameter() {
    let result = eval(r#"
        let double = (x) => do {
            return x * 2
        }
        double(21)
    "#).unwrap();
    assert_eq!(result, Value::Number(42.0));
}

#[test]
fn test_return_with_multiple_parameters() {
    let result = eval(r#"
        let sum = (a, b, c) => do {
            return a + b + c
        }
        sum(10, 20, 12)
    "#).unwrap();
    assert_eq!(result, Value::Number(42.0));
}

// ============================================================================
// Practical Use Cases
// ============================================================================

#[test]
fn test_input_validation_with_return() {
    let result = eval(r#"
        let divide = (a, b) => do {
            if(b == 0) {
                return "error: division by zero"
            }
            a / b
        }
        divide(10, 2)
    "#).unwrap();
    assert_eq!(result, Value::Number(5.0));
}

#[test]
fn test_input_validation_error_case() {
    let result = eval(r#"
        let divide = (a, b) => do {
            if(b == 0) {
                return "error: division by zero"
            }
            a / b
        }
        divide(10, 0)
    "#).unwrap();
    assert_eq!(result, Value::String("error: division by zero".to_string()));
}

#[test]
fn test_find_first_matching_with_return() {
    let result = eval(r#"
        let findFirst = (predicate, list) => do {
            // Simplified version - in real code would iterate over list
            if(predicate(list[0])) {
                return list[0]
            }
            if(predicate(list[1])) {
                return list[1]
            }
            if(predicate(list[2])) {
                return list[2]
            }
            return -1
        }
        let isEven = x => x % 2 == 0
        findFirst(isEven, [1, 3, 4, 5])
    "#).unwrap();
    assert_eq!(result, Value::Number(4.0));
}

#[test]
fn test_guard_clauses() {
    let result = eval(r#"
        let process = (value) => do {
            // Guard clauses with early returns
            if(value < 0) {
                return "negative"
            }
            if(value == 0) {
                return "zero"
            }

            // Main logic only runs if guards pass
            value * 2
        }
        process(5)
    "#).unwrap();
    assert_eq!(result, Value::Number(10.0));
}

// ============================================================================
// Return in Recursive Functions
// ============================================================================

#[test]
fn test_return_in_recursive_function() {
    let result = eval(r#"
        let factorial = (n) => do {
            if(n <= 1) {
                return 1
            }
            return n * rec(n - 1)
        }
        factorial(5)
    "#).unwrap();
    assert_eq!(result, Value::Number(120.0));
}

#[test]
fn test_return_in_tail_recursive_function() {
    let result = eval(r#"
        let sum = (n, acc) => do {
            if(n == 0) {
                return acc
            }
            return rec(n - 1, acc + n)
        }
        sum(5, 0)
    "#).unwrap();
    assert_eq!(result, Value::Number(15.0));  // 5+4+3+2+1
}

// ============================================================================
// Return Without Do Block (Should Work in Single Expression Lambdas)
// ============================================================================

#[test]
fn test_return_in_lambda_body_expression() {
    // In a single-expression lambda, we can't use return
    // This is expected - return requires a do block
    let result = eval(r#"
        let f = (x) => x * 2
        f(21)
    "#).unwrap();
    assert_eq!(result, Value::Number(42.0));
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_return_zero() {
    let result = eval(r#"
        let f = () => do {
            return 0
        }
        f()
    "#).unwrap();
    assert_eq!(result, Value::Number(0.0));
}

#[test]
fn test_return_empty_string() {
    let result = eval(r#"
        let f = () => do {
            return ""
        }
        f()
    "#).unwrap();
    assert_eq!(result, Value::String("".to_string()));
}

#[test]
fn test_return_false() {
    let result = eval(r#"
        let f = () => do {
            return false
        }
        f()
    "#).unwrap();
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_multiple_functions_with_returns() {
    let result = eval(r#"
        let f1 = (x) => do {
            if(x < 0) {
                return 0
            }
            x
        }

        let f2 = (x) => do {
            if(x > 100) {
                return 100
            }
            x
        }

        f1(-5) + f2(200)
    "#).unwrap();
    assert_eq!(result, Value::Number(100.0));  // 0 + 100
}

// ============================================================================
// Comparison: With and Without Return
// ============================================================================

#[test]
fn test_without_return_nested_if() {
    // Old style: nested if/else
    let result = eval(r#"
        let validate = (x) => do {
            if(x < 0) {
                "error: negative"
            } else {
                if(x > 100) {
                    "error: too large"
                } else {
                    "ok"
                }
            }
        }
        validate(50)
    "#).unwrap();
    assert_eq!(result, Value::String("ok".to_string()));
}

#[test]
fn test_with_return_flat_structure() {
    // New style: flat structure with early returns
    let result = eval(r#"
        let validate = (x) => do {
            if(x < 0) {
                return "error: negative"
            }
            if(x > 100) {
                return "error: too large"
            }
            "ok"
        }
        validate(50)
    "#).unwrap();
    assert_eq!(result, Value::String("ok".to_string()));
}
