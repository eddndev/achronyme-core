use crate::helpers::{eval, eval_with_evaluator};
use achronyme_eval::Evaluator;
use achronyme_types::value::Value;

#[test]
fn test_lambda_creation() {
    // Create a lambda
    let result = eval("x => x * 2").unwrap();

    // Should be a function value
    match result {
        Value::Function(_) => {}, // Success
        _ => panic!("Expected function value"),
    }
}

#[test]
fn test_lambda_call() {
    // Define lambda and call it
    let mut evaluator = Evaluator::new();
    eval_with_evaluator(&mut evaluator, "let f = x => x * 2").unwrap();

    // Now call it
    let result = eval_with_evaluator(&mut evaluator, "f(5)").unwrap();

    assert_eq!(result, Value::Number(10.0));
}

#[test]
fn test_lambda_closure() {
    // Lambda captures variable from outer scope
    let mut evaluator = Evaluator::new();
    eval_with_evaluator(&mut evaluator, "let x = 10").unwrap();

    // Create lambda that uses x
    eval_with_evaluator(&mut evaluator, "let f = y => x + y").unwrap();

    // Call lambda
    let result = eval_with_evaluator(&mut evaluator, "f(5)").unwrap();

    assert_eq!(result, Value::Number(15.0)); // 10 + 5
}

#[test]
fn test_lambda_multi_param() {
    // Lambda with multiple parameters
    let mut evaluator = Evaluator::new();
    eval_with_evaluator(&mut evaluator, "let add = (x, y) => x + y").unwrap();

    // Call it
    let result = eval_with_evaluator(&mut evaluator, "add(3, 4)").unwrap();

    assert_eq!(result, Value::Number(7.0));
}

#[test]
fn test_lambda_arity_check() {
    // Lambda arity mismatch should fail
    let mut evaluator = Evaluator::new();
    eval_with_evaluator(&mut evaluator, "let f = x => x * 2").unwrap();

    // Call with wrong number of args
    let result = eval_with_evaluator(&mut evaluator, "f(1, 2)");

    assert!(result.is_err());
}

#[test]
fn test_lambda_nested() {
    // Nested lambda (higher-order function)
    let mut evaluator = Evaluator::new();
    eval_with_evaluator(&mut evaluator, "let makeAdder = x => (y => x + y)").unwrap();

    // Get an adder function
    eval_with_evaluator(&mut evaluator, "let add5 = makeAdder(5)").unwrap();

    // Use it
    let result = eval_with_evaluator(&mut evaluator, "add5(3)").unwrap();

    assert_eq!(result, Value::Number(8.0)); // 5 + 3
}
