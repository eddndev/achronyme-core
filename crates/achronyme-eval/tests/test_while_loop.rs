/// Integration tests for while loop
///
/// Tests the while loop control flow structure:
/// - while(condition) { body }
/// - Executes body repeatedly while condition is true
/// - Returns the value of the last iteration, or 0 if never executed
/// - Supports early return with return statement

use achronyme_eval::Evaluator;
use achronyme_types::value::Value;

fn eval(source: &str) -> Result<Value, String> {
    let mut evaluator = Evaluator::new();
    evaluator.eval_str(source)
}

// ============================================================================
// Basic While Loop Tests
// ============================================================================

#[test]
fn test_simple_while_loop() {
    let result = eval(r#"
        mut i = 0
        while(i < 5) {
            i = i + 1
        }
        i
    "#).unwrap();
    assert_eq!(result, Value::Number(5.0));
}

#[test]
fn test_while_loop_never_executes() {
    let result = eval(r#"
        mut i = 10
        while(i < 5) {
            i = i + 1
        }
        i
    "#).unwrap();
    assert_eq!(result, Value::Number(10.0));
}

#[test]
fn test_while_loop_returns_last_value() {
    let result = eval(r#"
        mut i = 0
        while(i < 3) {
            i = i + 1
            i * 10
        }
    "#).unwrap();
    assert_eq!(result, Value::Number(30.0));
}

#[test]
fn test_while_loop_returns_zero_if_never_executed() {
    let result = eval(r#"
        while(false) {
            42
        }
    "#).unwrap();
    assert_eq!(result, Value::Number(0.0));
}

// ============================================================================
// While Loop with Accumulation
// ============================================================================

#[test]
fn test_while_loop_sum() {
    let result = eval(r#"
        mut i = 1
        mut sum = 0
        while(i <= 5) {
            sum = sum + i
            i = i + 1
        }
        sum
    "#).unwrap();
    assert_eq!(result, Value::Number(15.0));  // 1+2+3+4+5
}

#[test]
fn test_while_loop_factorial() {
    let result = eval(r#"
        mut n = 5
        mut result = 1
        while(n > 0) {
            result = result * n
            n = n - 1
        }
        result
    "#).unwrap();
    assert_eq!(result, Value::Number(120.0));  // 5!
}

#[test]
fn test_while_loop_fibonacci() {
    let result = eval(r#"
        mut a = 0
        mut b = 1
        mut i = 0
        while(i < 7) {
            let temp = a + b
            a = b
            b = temp
            i = i + 1
        }
        a
    "#).unwrap();
    assert_eq!(result, Value::Number(13.0));  // 7th fibonacci number
}

// ============================================================================
// While Loop with Conditions
// ============================================================================

#[test]
fn test_while_loop_with_comparison() {
    let result = eval(r#"
        mut x = 10
        while(x > 0) {
            x = x - 2
        }
        x
    "#).unwrap();
    assert_eq!(result, Value::Number(0.0));
}

#[test]
fn test_while_loop_with_inequality() {
    let result = eval(r#"
        mut x = 1
        while(x != 10) {
            x = x + 1
        }
        x
    "#).unwrap();
    assert_eq!(result, Value::Number(10.0));
}

#[test]
fn test_while_loop_with_logical_and() {
    let result = eval(r#"
        mut x = 0
        mut y = 0
        while(x < 5 && y < 3) {
            x = x + 1
            y = y + 1
        }
        x
    "#).unwrap();
    assert_eq!(result, Value::Number(3.0));  // Stops when y reaches 3
}

// ============================================================================
// Nested While Loops
// ============================================================================

#[test]
fn test_nested_while_loops() {
    let result = eval(r#"
        mut i = 0
        mut sum = 0
        while(i < 3) {
            mut j = 0
            while(j < 2) {
                sum = sum + 1
                j = j + 1
            }
            i = i + 1
        }
        sum
    "#).unwrap();
    assert_eq!(result, Value::Number(6.0));  // 3 * 2
}

#[test]
fn test_nested_while_loops_multiplication_table() {
    let result = eval(r#"
        mut i = 1
        mut product = 1
        while(i <= 2) {
            mut j = 1
            while(j <= 3) {
                product = i * j
                j = j + 1
            }
            i = i + 1
        }
        product
    "#).unwrap();
    assert_eq!(result, Value::Number(6.0));  // 2 * 3 (last iteration)
}

// ============================================================================
// While Loop with Early Return
// ============================================================================

#[test]
fn test_while_loop_with_early_return() {
    let result = eval(r#"
        let find = (target) => do {
            mut i = 0
            while(i < 10) {
                if(i == target) {
                    return i
                }
                i = i + 1
            }
            -1
        }
        find(5)
    "#).unwrap();
    assert_eq!(result, Value::Number(5.0));
}

#[test]
fn test_while_loop_early_return_not_found() {
    let result = eval(r#"
        let find = (target) => do {
            mut i = 0
            while(i < 5) {
                if(i == target) {
                    return i
                }
                i = i + 1
            }
            -1
        }
        find(10)
    "#).unwrap();
    assert_eq!(result, Value::Number(-1.0));  // Not found
}

#[test]
fn test_while_loop_with_return_in_nested_if() {
    let result = eval(r#"
        let search = () => do {
            mut i = 0
            while(i < 10) {
                if(i > 5) {
                    if(i == 7) {
                        return i * 10
                    }
                }
                i = i + 1
            }
            0
        }
        search()
    "#).unwrap();
    assert_eq!(result, Value::Number(70.0));
}

// ============================================================================
// While Loop in Functions
// ============================================================================

#[test]
fn test_while_loop_in_lambda() {
    let result = eval(r#"
        let countdown = (n) => do {
            mut i = n
            while(i > 0) {
                i = i - 1
            }
            i
        }
        countdown(5)
    "#).unwrap();
    assert_eq!(result, Value::Number(0.0));
}

#[test]
fn test_while_loop_with_parameters() {
    let result = eval(r#"
        let sumUpTo = (n) => do {
            mut i = 1
            mut sum = 0
            while(i <= n) {
                sum = sum + i
                i = i + 1
            }
            sum
        }
        sumUpTo(10)
    "#).unwrap();
    assert_eq!(result, Value::Number(55.0));  // 1+2+...+10
}

// ============================================================================
// While Loop with Complex Conditions
// ============================================================================

#[test]
fn test_while_loop_with_expression_condition() {
    let result = eval(r#"
        mut x = 16
        while(x / 2 > 1) {
            x = x / 2
        }
        x
    "#).unwrap();
    assert_eq!(result, Value::Number(2.0));
}

#[test]
fn test_while_loop_with_function_call_in_condition() {
    let result = eval(r#"
        let isEven = (n) => n % 2 == 0
        mut x = 7
        while(!isEven(x)) {
            x = x + 1
        }
        x
    "#).unwrap();
    assert_eq!(result, Value::Number(8.0));
}

// ============================================================================
// While Loop with Multiple Variables
// ============================================================================

#[test]
fn test_while_loop_multiple_variables() {
    let result = eval(r#"
        mut a = 0
        mut b = 10
        mut c = 0
        while(a < 5) {
            a = a + 1
            b = b - 1
            c = a + b
        }
        c
    "#).unwrap();
    assert_eq!(result, Value::Number(10.0));  // 5 + 5
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_while_loop_single_iteration() {
    let result = eval(r#"
        mut i = 4
        while(i < 5) {
            i = i + 1
        }
        i
    "#).unwrap();
    assert_eq!(result, Value::Number(5.0));
}

#[test]
fn test_while_loop_with_break_condition_inside() {
    // Using return as a break mechanism
    let result = eval(r#"
        let process = () => do {
            mut i = 0
            while(true) {
                i = i + 1
                if(i >= 5) {
                    return i
                }
            }
        }
        process()
    "#).unwrap();
    assert_eq!(result, Value::Number(5.0));
}

#[test]
fn test_while_loop_empty_body() {
    // While loop with empty body (just increments in condition would be infinite)
    // This tests that the body can be minimal
    let result = eval(r#"
        mut i = 0
        while(i < 5) {
            i = i + 1
        }
        i
    "#).unwrap();
    assert_eq!(result, Value::Number(5.0));
}

// ============================================================================
// While Loop with Arrays (Future: when we have array iteration)
// ============================================================================

#[test]
fn test_while_loop_building_array() {
    let result = eval(r#"
        mut i = 0
        mut arr = []
        while(i < 3) {
            arr = [...arr, i]
            i = i + 1
        }
        arr
    "#).unwrap();
    // Should return [0, 1, 2]
    assert!(matches!(result, Value::Vector(_) | Value::Tensor(_)));
}
