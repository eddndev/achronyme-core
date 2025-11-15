use crate::helpers::{eval, eval_with_evaluator};
use achronyme_eval::Evaluator;
use achronyme_types::value::Value;

#[test]
fn test_boolean_literals() {
    assert_eq!(eval("true").unwrap(), Value::Boolean(true));
    assert_eq!(eval("false").unwrap(), Value::Boolean(false));
}

#[test]
fn test_logical_and() {
    assert_eq!(eval("true && true").unwrap(), Value::Boolean(true));
    assert_eq!(eval("true && false").unwrap(), Value::Boolean(false));
    assert_eq!(eval("false && true").unwrap(), Value::Boolean(false));
    assert_eq!(eval("false && false").unwrap(), Value::Boolean(false));
}

#[test]
fn test_logical_or() {
    assert_eq!(eval("true || true").unwrap(), Value::Boolean(true));
    assert_eq!(eval("true || false").unwrap(), Value::Boolean(true));
    assert_eq!(eval("false || true").unwrap(), Value::Boolean(true));
    assert_eq!(eval("false || false").unwrap(), Value::Boolean(false));
}

#[test]
fn test_logical_not() {
    assert_eq!(eval("!true").unwrap(), Value::Boolean(false));
    assert_eq!(eval("!false").unwrap(), Value::Boolean(true));
    assert_eq!(eval("!!true").unwrap(), Value::Boolean(true));
    assert_eq!(eval("!!false").unwrap(), Value::Boolean(false));
}

#[test]
fn test_comparison_returns_boolean() {
    assert_eq!(eval("5 > 3").unwrap(), Value::Boolean(true));
    assert_eq!(eval("5 < 3").unwrap(), Value::Boolean(false));
    assert_eq!(eval("5 >= 5").unwrap(), Value::Boolean(true));
    assert_eq!(eval("5 <= 3").unwrap(), Value::Boolean(false));
    assert_eq!(eval("5 == 5").unwrap(), Value::Boolean(true));
    assert_eq!(eval("5 != 3").unwrap(), Value::Boolean(true));
}

#[test]
fn test_if_simple() {
    assert_eq!(eval("if(true, 1, 2)").unwrap(), Value::Number(1.0));
    assert_eq!(eval("if(false, 1, 2)").unwrap(), Value::Number(2.0));
}

#[test]
fn test_if_with_comparison() {
    assert_eq!(eval("if(5 > 3, 100, 200)").unwrap(), Value::Number(100.0));
    assert_eq!(eval("if(2 > 10, 100, 200)").unwrap(), Value::Number(200.0));
}

#[test]
fn test_if_with_logical_ops() {
    assert_eq!(eval("if(true && true, 1, 0)").unwrap(), Value::Number(1.0));
    assert_eq!(eval("if(true && false, 1, 0)").unwrap(), Value::Number(0.0));
    assert_eq!(eval("if(false || true, 1, 0)").unwrap(), Value::Number(1.0));
}

#[test]
fn test_if_nested() {
    // if(x > 0, if(x > 10, 2, 1), 0)
    let mut evaluator = Evaluator::new();
    eval_with_evaluator(&mut evaluator, "let x = 15").unwrap();
    let result = eval_with_evaluator(&mut evaluator, "if(x > 0, if(x > 10, 2, 1), 0)").unwrap();
    assert_eq!(result, Value::Number(2.0));

    let mut evaluator = Evaluator::new();
    eval_with_evaluator(&mut evaluator, "let x = 5").unwrap();
    let result = eval_with_evaluator(&mut evaluator, "if(x > 0, if(x > 10, 2, 1), 0)").unwrap();
    assert_eq!(result, Value::Number(1.0));

    let mut evaluator = Evaluator::new();
    eval_with_evaluator(&mut evaluator, "let x = -5").unwrap();
    let result = eval_with_evaluator(&mut evaluator, "if(x > 0, if(x > 10, 2, 1), 0)").unwrap();
    assert_eq!(result, Value::Number(0.0));
}

#[test]
fn test_if_in_lambda() {
    // abs function: x => if(x < 0, -x, x)
    let mut evaluator = Evaluator::new();
    eval_with_evaluator(&mut evaluator, "let abs = x => if(x < 0, -x, x)").unwrap();

    let result = eval_with_evaluator(&mut evaluator, "abs(-5)").unwrap();
    assert_eq!(result, Value::Number(5.0));

    let result = eval_with_evaluator(&mut evaluator, "abs(3)").unwrap();
    assert_eq!(result, Value::Number(3.0));
}

#[test]
fn test_relu_activation() {
    // ReLU: x => if(x > 0, x, 0)
    let mut evaluator = Evaluator::new();
    eval_with_evaluator(&mut evaluator, "let relu = x => if(x > 0, x, 0)").unwrap();

    let result = eval_with_evaluator(&mut evaluator, "relu(5)").unwrap();
    assert_eq!(result, Value::Number(5.0));

    let result = eval_with_evaluator(&mut evaluator, "relu(-3)").unwrap();
    assert_eq!(result, Value::Number(0.0));

    let result = eval_with_evaluator(&mut evaluator, "relu(0)").unwrap();
    assert_eq!(result, Value::Number(0.0));
}

#[test]
fn test_sign_function() {
    // sign: x => if(x < 0, -1, if(x > 0, 1, 0))
    let mut evaluator = Evaluator::new();
    eval_with_evaluator(&mut evaluator, "let sign = x => if(x < 0, -1, if(x > 0, 1, 0))").unwrap();

    let result = eval_with_evaluator(&mut evaluator, "sign(-10)").unwrap();
    assert_eq!(result, Value::Number(-1.0));

    let result = eval_with_evaluator(&mut evaluator, "sign(10)").unwrap();
    assert_eq!(result, Value::Number(1.0));

    let result = eval_with_evaluator(&mut evaluator, "sign(0)").unwrap();
    assert_eq!(result, Value::Number(0.0));
}
