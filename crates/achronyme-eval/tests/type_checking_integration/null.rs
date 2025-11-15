//! Null Type Tests

use achronyme_eval::Evaluator;

#[test]
fn test_null_literal_parsing() {
    let mut eval = Evaluator::new();
    let result = eval.eval_str("null");
    assert!(result.is_ok());
    match result.unwrap() {
        achronyme_types::value::Value::Null => {}
        other => panic!("Expected Null, got {:?}", other),
    }
}

#[test]
fn test_null_type_annotation() {
    let mut eval = Evaluator::new();
    let result = eval.eval_str("let x: null = null");
    assert!(result.is_ok());
}

#[test]
fn test_null_type_mismatch() {
    let mut eval = Evaluator::new();
    let result = eval.eval_str("let x: null = 42");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("Type error"));
    assert!(err.contains("null"));
}

#[test]
fn test_optional_type_with_null() {
    let mut eval = Evaluator::new();
    // Number | null is an optional number
    let result = eval.eval_str("let opt: Number | null = null");
    assert!(result.is_ok());
}

#[test]
fn test_optional_type_with_value() {
    let mut eval = Evaluator::new();
    // Number | null can also hold a Number
    let result = eval.eval_str("let opt: Number | null = 42");
    assert!(result.is_ok());
}

#[test]
fn test_optional_type_mismatch() {
    let mut eval = Evaluator::new();
    // Number | null should not accept String
    let result = eval.eval_str(r#"let opt: Number | null = "hello""#);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("Type error"));
}

#[test]
fn test_multiple_optional_declarations() {
    let mut eval = Evaluator::new();
    // Chain of optional declarations
    assert!(eval.eval_str("let a: String | null = null").is_ok());
    assert!(eval.eval_str(r#"let b: String | null = "value""#).is_ok());
    assert!(eval.eval_str("let c: Boolean | null = true").is_ok());
    assert!(eval.eval_str("let d: Boolean | null = null").is_ok());
}

#[test]
fn test_mutable_optional_type() {
    let mut eval = Evaluator::new();
    // Mutable variables with optional types
    let result = eval.eval_str("mut x: Number | null = null");
    assert!(result.is_ok());
}
