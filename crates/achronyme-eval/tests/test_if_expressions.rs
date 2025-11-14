/// Integration tests for if expression syntax
///
/// Tests the new block-based if expression syntax with parenthesized conditions:
/// - if(expr) { block }
/// - if(expr) { block } else { block }
/// - if(expr) { block } else if(expr) { block } else { block }
///
/// This coexists with the functional if(cond, then, else) syntax

use achronyme_eval::Evaluator;
use achronyme_types::value::Value;

fn eval(source: &str) -> Result<Value, String> {
    let mut evaluator = Evaluator::new();
    evaluator.eval_str(source)
}

// ============================================================================
// Basic If Expression Tests
// ============================================================================

#[test]
fn test_if_expr_true_condition() {
    let result = eval("if(true) { 42 } else { 0 }").unwrap();
    assert_eq!(result, Value::Number(42.0));
}

#[test]
fn test_if_expr_false_condition() {
    let result = eval("if(false) { 42 } else { 0 }").unwrap();
    assert_eq!(result, Value::Number(0.0));
}

#[test]
fn test_if_expr_with_comparison() {
    let result = eval(r#"
        let x = 10;
        if(x > 5) { "large" } else { "small" }
    "#).unwrap();
    assert_eq!(result, Value::String("large".to_string()));
}

#[test]
fn test_if_expr_without_else() {
    // If without else returns 0 when condition is false
    let result = eval("if(false) { 42 }").unwrap();
    assert_eq!(result, Value::Number(0.0));
}

#[test]
fn test_if_expr_without_else_true() {
    let result = eval("if(true) { 42 }").unwrap();
    assert_eq!(result, Value::Number(42.0));
}

// ============================================================================
// Multi-Statement Blocks
// ============================================================================

#[test]
fn test_if_expr_with_sequence() {
    let result = eval(r#"
        if(true) {
            let a = 10;
            let b = 20;
            a + b
        } else {
            0
        }
    "#).unwrap();
    assert_eq!(result, Value::Number(30.0));
}

// TODO: Re-enable once mut keyword parsing issue is resolved
// #[test]
// fn test_if_expr_with_assignments() {
//     let result = eval(r#"
//         let mut x = 0;
//         if(true) {
//             x = 10;
//             x = x + 5;
//             x
//         } else {
//             x
//         }
//     "#).unwrap();
//     assert_eq!(result, Value::Number(15.0));
// }

// ============================================================================
// Else-If Chains
// ============================================================================

#[test]
fn test_else_if_chain() {
    let result = eval(r#"
        let score = 75;
        if(score >= 90) {
            "A"
        } else if(score >= 80) {
            "B"
        } else if(score >= 70) {
            "C"
        } else {
            "F"
        }
    "#).unwrap();
    assert_eq!(result, Value::String("C".to_string()));
}

#[test]
fn test_else_if_first_match() {
    let result = eval(r#"
        let x = 95;
        if(x >= 90) { "excellent" } else if(x >= 80) { "good" } else { "ok" }
    "#).unwrap();
    assert_eq!(result, Value::String("excellent".to_string()));
}

#[test]
fn test_else_if_final_else() {
    let result = eval(r#"
        let x = 50;
        if(x >= 90) { "A" } else if(x >= 70) { "B" } else { "F" }
    "#).unwrap();
    assert_eq!(result, Value::String("F".to_string()));
}

// ============================================================================
// Nested If Expressions
// ============================================================================

#[test]
fn test_nested_if_expr() {
    let result = eval(r#"
        let x = 10;
        let y = 20;
        if(x > 5) {
            if(y > 15) {
                "both large"
            } else {
                "x large, y small"
            }
        } else {
            "x small"
        }
    "#).unwrap();
    assert_eq!(result, Value::String("both large".to_string()));
}

// ============================================================================
// If Expression as Value
// ============================================================================

#[test]
fn test_if_expr_assigned_to_variable() {
    let result = eval(r#"
        let x = 10;
        let y = if(x > 5) { 100 } else { 50 };
        y + 20
    "#).unwrap();
    assert_eq!(result, Value::Number(120.0));
}

#[test]
fn test_if_expr_in_arithmetic() {
    let result = eval(r#"
        let x = 5;
        10 + if(x > 0) { x * 2 } else { 0 }
    "#).unwrap();
    assert_eq!(result, Value::Number(20.0));  // 10 + (5 * 2)
}

// ============================================================================
// Comparison: Block Syntax vs Functional Syntax
// ============================================================================

#[test]
fn test_block_syntax_vs_functional_equivalent() {
    let block_result = eval(r#"
        let x = 10;
        if(x > 5) { "yes" } else { "no" }
    "#).unwrap();

    let func_result = eval(r#"
        let x = 10;
        if(x > 5, "yes", "no")
    "#).unwrap();

    assert_eq!(block_result, func_result);
}

#[test]
fn test_functional_if_still_works() {
    // The old functional syntax should still work
    let result = eval("if(true, 42, 0)").unwrap();
    assert_eq!(result, Value::Number(42.0));
}

// ============================================================================
// Complex Conditions
// ============================================================================

#[test]
fn test_if_with_logical_and() {
    let result = eval(r#"
        let x = 10;
        let y = 20;
        if(x > 5 && y < 30) { "both true" } else { "at least one false" }
    "#).unwrap();
    assert_eq!(result, Value::String("both true".to_string()));
}

#[test]
fn test_if_with_logical_or() {
    let result = eval(r#"
        let x = 3;
        if(x < 5 || x > 10) { "outside range" } else { "in range" }
    "#).unwrap();
    assert_eq!(result, Value::String("outside range".to_string()));
}

#[test]
fn test_if_with_negation() {
    let result = eval(r#"
        let flag = false;
        if(!flag) { "not false" } else { "false" }
    "#).unwrap();
    assert_eq!(result, Value::String("not false".to_string()));
}

// ============================================================================
// If with Complex Return Types
// ============================================================================

#[test]
fn test_if_returning_array() {
    let result = eval(r#"
        let x = 5;
        if(x > 0) { [1, 2, 3] } else { [4, 5, 6] }
    "#).unwrap();
    match result {
        Value::Vector(v) => {
            assert_eq!(v.len(), 3);
            assert_eq!(v[0], Value::Number(1.0));
        }
        Value::Tensor(t) => {
            assert_eq!(t.data(), &[1.0, 2.0, 3.0]);
        }
        _ => panic!("Expected Vector or Tensor"),
    }
}

#[test]
fn test_if_returning_record() {
    let result = eval(r#"
        let condition = true;
        if(condition) {
            { x: 10, y: 20 }
        } else {
            { x: 0, y: 0 }
        }
    "#).unwrap();
    if let Value::Record(map) = result {
        assert!(matches!(map.get("x"), Some(Value::Number(10.0))));
        assert!(matches!(map.get("y"), Some(Value::Number(20.0))));
    } else {
        panic!("Expected Record");
    }
}

#[test]
fn test_if_returning_lambda() {
    let result = eval(r#"
        let selector = true;
        let f = if(selector) {
            x => x * 2
        } else {
            x => x * 3
        };
        f(10)
    "#).unwrap();
    assert_eq!(result, Value::Number(20.0));
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_if_condition_is_expression() {
    let result = eval(r#"
        let x = 10;
        if(x * 2 > 15) { "yes" } else { "no" }
    "#).unwrap();
    assert_eq!(result, Value::String("yes".to_string()));
}

#[test]
fn test_if_in_function_call() {
    let result = eval(r#"
        print(if(true) { "hello" } else { "goodbye" })
    "#).unwrap();
    assert_eq!(result, Value::String("hello".to_string()));
}

#[test]
fn test_if_with_do_block() {
    let result = eval(r#"
        if(true) {
            do {
                let x = 10;
                let y = 20;
                x + y
            }
        } else {
            0
        }
    "#).unwrap();
    assert_eq!(result, Value::Number(30.0));
}

// ============================================================================
// Error Cases
// ============================================================================

#[test]
fn test_if_missing_braces_fails() {
    // if without braces should fail (use functional syntax instead)
    let result = eval("if(true) 42 else 0");
    assert!(result.is_err());
}

#[test]
fn test_if_missing_parentheses_fails() {
    // if without parentheses around condition should fail
    let result = eval("if true { 42 } else { 0 }");
    assert!(result.is_err());
}
