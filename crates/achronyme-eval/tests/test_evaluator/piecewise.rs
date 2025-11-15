use crate::helpers::{eval, eval_with_evaluator};
use achronyme_eval::Evaluator;
use achronyme_types::value::Value;

#[test]
fn test_piecewise_simple() {
    // piecewise([x < 0, -1], [x > 0, 1], 0)
    let mut evaluator = Evaluator::new();
    eval_with_evaluator(&mut evaluator, "let x = -5").unwrap();
    let result = eval_with_evaluator(&mut evaluator, "piecewise([x < 0, -1], [x > 0, 1], 0)").unwrap();
    assert_eq!(result, Value::Number(-1.0));

    let mut evaluator = Evaluator::new();
    eval_with_evaluator(&mut evaluator, "let x = 5").unwrap();
    let result = eval_with_evaluator(&mut evaluator, "piecewise([x < 0, -1], [x > 0, 1], 0)").unwrap();
    assert_eq!(result, Value::Number(1.0));

    let mut evaluator = Evaluator::new();
    eval_with_evaluator(&mut evaluator, "let x = 0").unwrap();
    let result = eval_with_evaluator(&mut evaluator, "piecewise([x < 0, -1], [x > 0, 1], 0)").unwrap();
    assert_eq!(result, Value::Number(0.0));
}

#[test]
fn test_piecewise_no_default_error() {
    // piecewise without default should error if no condition is true
    let result = eval("piecewise([false, 1], [false, 2])");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("no condition was true"));
}

#[test]
fn test_piecewise_abs() {
    // abs using piecewise: x => piecewise([x < 0, -x], x)
    let mut evaluator = Evaluator::new();
    eval_with_evaluator(&mut evaluator, "let abs = x => piecewise([x < 0, -x], x)").unwrap();

    let result = eval_with_evaluator(&mut evaluator, "abs(-5)").unwrap();
    assert_eq!(result, Value::Number(5.0));

    let result = eval_with_evaluator(&mut evaluator, "abs(3)").unwrap();
    assert_eq!(result, Value::Number(3.0));

    let result = eval_with_evaluator(&mut evaluator, "abs(0)").unwrap();
    assert_eq!(result, Value::Number(0.0));
}

#[test]
fn test_piecewise_tax_bracket() {
    // Progressive tax:
    // income <= 10000: 10%
    // income <= 50000: 20%
    // else: 30%
    let mut evaluator = Evaluator::new();
    eval_with_evaluator(&mut evaluator, "let tax = income => piecewise([income <= 10000, income * 0.1], [income <= 50000, income * 0.2], income * 0.3)").unwrap();

    let result = eval_with_evaluator(&mut evaluator, "tax(5000)").unwrap();
    assert_eq!(result, Value::Number(500.0)); // 10%

    let result = eval_with_evaluator(&mut evaluator, "tax(30000)").unwrap();
    assert_eq!(result, Value::Number(6000.0)); // 20%

    let result = eval_with_evaluator(&mut evaluator, "tax(100000)").unwrap();
    assert_eq!(result, Value::Number(30000.0)); // 30%
}

#[test]
fn test_piecewise_math_function() {
    // f(x) = { x^2    if x < -1
    //        { 2x+1   if -1 <= x < 1
    //        { x^3    if x >= 1
    let mut evaluator = Evaluator::new();
    eval_with_evaluator(&mut evaluator, "let f = x => piecewise([x < -1, x^2], [x < 1, 2*x + 1], x^3)").unwrap();

    // x < -1: x^2
    let result = eval_with_evaluator(&mut evaluator, "f(-2)").unwrap();
    assert_eq!(result, Value::Number(4.0));

    // -1 <= x < 1: 2x+1
    let result = eval_with_evaluator(&mut evaluator, "f(0)").unwrap();
    assert_eq!(result, Value::Number(1.0));

    let result = eval_with_evaluator(&mut evaluator, "f(-1)").unwrap();
    assert_eq!(result, Value::Number(-1.0)); // 2*(-1) + 1

    // x >= 1: x^3
    let result = eval_with_evaluator(&mut evaluator, "f(2)").unwrap();
    assert_eq!(result, Value::Number(8.0));
}

#[test]
fn test_piecewise_heaviside() {
    // Heaviside step function: H(x) = { 0 if x < 0, 1 if x >= 0 }
    let mut evaluator = Evaluator::new();
    eval_with_evaluator(&mut evaluator, "let H = x => piecewise([x < 0, 0], 1)").unwrap();

    let result = eval_with_evaluator(&mut evaluator, "H(-5)").unwrap();
    assert_eq!(result, Value::Number(0.0));

    let result = eval_with_evaluator(&mut evaluator, "H(0)").unwrap();
    assert_eq!(result, Value::Number(1.0));

    let result = eval_with_evaluator(&mut evaluator, "H(5)").unwrap();
    assert_eq!(result, Value::Number(1.0));
}

#[test]
fn test_piecewise_with_hof() {
    // Use piecewise in map
    let mut evaluator = Evaluator::new();
    eval_with_evaluator(&mut evaluator, "let classify = x => piecewise([x < 0, -1], [x > 0, 1], 0)").unwrap();
    let result = eval_with_evaluator(&mut evaluator, "map(classify, [-5, -2, 0, 3, 7])").unwrap();

    match result {
        Value::Vector(v) => {
            assert_eq!(v, vec![Value::Number(-1.0), Value::Number(-1.0), Value::Number(0.0), Value::Number(1.0), Value::Number(1.0)]);
        }
        _ => panic!("Expected vector"),
    }
}

#[test]
fn test_piecewise_multivariable() {
    // Region classifier in 2D plane: inside circle (1), in square (2), outside (0)
    let mut evaluator = Evaluator::new();
    eval_with_evaluator(&mut evaluator, "let region = (x, y) => piecewise([x^2 + y^2 < 1, 1], [abs(x) < 2 && abs(y) < 2, 2], 0)").unwrap();

    // Inside circle
    let result = eval_with_evaluator(&mut evaluator, "region(0, 0)").unwrap();
    assert_eq!(result, Value::Number(1.0));

    // In square but outside circle
    let result = eval_with_evaluator(&mut evaluator, "region(1.5, 0)").unwrap();
    assert_eq!(result, Value::Number(2.0));

    // Outside both
    let result = eval_with_evaluator(&mut evaluator, "region(3, 3)").unwrap();
    assert_eq!(result, Value::Number(0.0));
}

#[test]
fn test_piecewise_sequential_evaluation() {
    // Verify short-circuit: first true condition wins
    let mut evaluator = Evaluator::new();
    eval_with_evaluator(&mut evaluator, "let x = 5").unwrap();
    // x > 0 is true, so should return 100 (not 200)
    let result = eval_with_evaluator(&mut evaluator, "piecewise([x > 0, 100], [x > 3, 200], 0)").unwrap();
    assert_eq!(result, Value::Number(100.0));
}

#[test]
fn test_piecewise_leaky_relu() {
    // Leaky ReLU: x > 0 ? x : 0.01*x
    let mut evaluator = Evaluator::new();
    eval_with_evaluator(&mut evaluator, "let leaky_relu = x => piecewise([x > 0, x], 0.01 * x)").unwrap();

    let result = eval_with_evaluator(&mut evaluator, "leaky_relu(10)").unwrap();
    assert_eq!(result, Value::Number(10.0));

    let result = eval_with_evaluator(&mut evaluator, "leaky_relu(-10)").unwrap();
    assert_eq!(result, Value::Number(-0.1));
}
