use crate::helpers::eval;
use achronyme_types::value::Value;

#[test]
fn test_number() {
    let result = eval("42").unwrap();
    assert_eq!(result, Value::Number(42.0));
}

#[test]
fn test_addition() {
    let result = eval("2 + 3").unwrap();
    assert_eq!(result, Value::Number(5.0));
}

#[test]
fn test_precedence() {
    let result = eval("2 + 3 * 4").unwrap();
    assert_eq!(result, Value::Number(14.0)); // 2 + (3 * 4) = 2 + 12 = 14
}

#[test]
fn test_power() {
    let result = eval("2 ^ 3").unwrap();
    assert_eq!(result, Value::Number(8.0));
}

#[test]
fn test_negation() {
    let result = eval("-5").unwrap();
    assert_eq!(result, Value::Number(-5.0));
}

#[test]
fn test_function_sin() {
    let result = eval("sin(0)").unwrap();
    match result {
        Value::Number(x) => assert!(x.abs() < 1e-10),
        _ => panic!("Expected number"),
    }
}

#[test]
fn test_constant_pi() {
    let result = eval("PI").unwrap();
    match result {
        Value::Number(x) => assert!((x - std::f64::consts::PI).abs() < 1e-10),
        _ => panic!("Expected number"),
    }
}

#[test]
fn test_variable() {
    let result = eval("let x = 5").unwrap();
    assert_eq!(result, Value::Number(5.0));
}

#[test]
fn test_vector() {
    let result = eval("[1, 2, 3]").unwrap();
    match result {
        Value::Vector(v) => {
            assert_eq!(v.len(), 3);
            assert_eq!(v[0], Value::Number(1.0));
            assert_eq!(v[1], Value::Number(2.0));
            assert_eq!(v[2], Value::Number(3.0));
        }
        _ => panic!("Expected vector"),
    }
}

#[test]
fn test_comparison() {
    let result = eval("5 > 3").unwrap();
    assert_eq!(result, Value::Boolean(true));
    let result = eval("5 < 3").unwrap();
    assert_eq!(result, Value::Boolean(false));
}
