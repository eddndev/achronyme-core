//! Assignment Type Checking Tests

use achronyme_eval::Evaluator;

#[test]
fn test_mutable_assignment_type_valid() {
    let mut eval = Evaluator::new();
    let result = eval.eval_str(r#"
        mut x: Number = 10;
        x = 20
    "#);
    assert!(result.is_ok());
    match result.unwrap() {
        achronyme_types::value::Value::Number(n) => assert_eq!(n, 20.0),
        other => panic!("Expected Number(20), got {:?}", other),
    }
}

#[test]
fn test_mutable_assignment_type_mismatch() {
    let mut eval = Evaluator::new();
    let result = eval.eval_str(r#"
        mut id: Number = 10;
        id = "Hola mundo"
    "#);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("Type error"));
    assert!(err.contains("String"));
    assert!(err.contains("variable 'id'"));
    assert!(err.contains("Number"));
}

#[test]
fn test_mutable_union_assignment_valid() {
    let mut eval = Evaluator::new();
    // Union type allows both Number and String
    let result = eval.eval_str(r#"
        mut x: Number | String = 10;
        x = "Now a string"
    "#);
    assert!(result.is_ok());
}

#[test]
fn test_mutable_union_assignment_invalid() {
    let mut eval = Evaluator::new();
    // Union type rejects Vector
    let result = eval.eval_str(r#"
        mut x: Number | String = 10;
        x = [1, 2, 3]
    "#);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("Type error"));
    assert!(err.contains("Vector"));
}

#[test]
fn test_mutable_optional_assignment_null() {
    let mut eval = Evaluator::new();
    // Optional type can be assigned null
    let result = eval.eval_str(r#"
        mut maybeValue: Number | null = 42;
        maybeValue = null
    "#);
    assert!(result.is_ok());
}

#[test]
fn test_mutable_optional_assignment_back_to_value() {
    let mut eval = Evaluator::new();
    // Can assign back to number from null
    let result = eval.eval_str(r#"
        mut maybeValue: Number | null = null;
        maybeValue = 100
    "#);
    assert!(result.is_ok());
}

#[test]
fn test_mutable_without_type_accepts_anything() {
    let mut eval = Evaluator::new();
    // Without type annotation, assignment accepts any type
    let result = eval.eval_str(r#"
        mut dynamic = 10;
        dynamic = "now a string"
    "#);
    assert!(result.is_ok());
}

#[test]
fn test_multiple_assignments_respect_type() {
    let mut eval = Evaluator::new();
    let result = eval.eval_str(r#"
        mut counter: Number = 0;
        counter = 1;
        counter = 2;
        counter = 3
    "#);
    assert!(result.is_ok());
    match result.unwrap() {
        achronyme_types::value::Value::Number(n) => assert_eq!(n, 3.0),
        other => panic!("Expected Number(3), got {:?}", other),
    }
}
