//! Basic let/mut type checking tests

use achronyme_eval::Evaluator;

#[test]
fn test_let_number_type_valid() {
    let mut eval = Evaluator::new();
    let result = eval.eval_str("let x: Number = 42");
    assert!(result.is_ok());
}

#[test]
fn test_let_string_type_valid() {
    let mut eval = Evaluator::new();
    let result = eval.eval_str(r#"let y: String = "hello""#);
    assert!(result.is_ok());
}

#[test]
fn test_let_boolean_type_valid() {
    let mut eval = Evaluator::new();
    let result = eval.eval_str("let z: Boolean = true");
    assert!(result.is_ok());
}

#[test]
fn test_let_number_type_mismatch() {
    let mut eval = Evaluator::new();
    let result = eval.eval_str(r#"let z: Number = "oops""#);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("Type error"));
    assert!(err.contains("variable 'z'"));
    assert!(err.contains("Number"));
    assert!(err.contains("String"));
}

#[test]
fn test_let_string_type_mismatch() {
    let mut eval = Evaluator::new();
    let result = eval.eval_str("let x: String = 42");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("Type error"));
    assert!(err.contains("variable 'x'"));
}

#[test]
fn test_let_boolean_type_mismatch() {
    let mut eval = Evaluator::new();
    let result = eval.eval_str(r#"let flag: Boolean = "not a bool""#);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("Type error"));
}

#[test]
fn test_mut_boolean_type_valid() {
    let mut eval = Evaluator::new();
    let result = eval.eval_str("mut a: Boolean = true");
    assert!(result.is_ok());
}

#[test]
fn test_mut_number_type_valid() {
    let mut eval = Evaluator::new();
    let result = eval.eval_str("mut counter: Number = 0");
    assert!(result.is_ok());
}

#[test]
fn test_mut_boolean_type_mismatch() {
    let mut eval = Evaluator::new();
    let result = eval.eval_str("mut b: Boolean = 10");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("Type error"));
    assert!(err.contains("variable 'b'"));
    assert!(err.contains("Boolean"));
    assert!(err.contains("Number"));
}

#[test]
fn test_mut_string_type_mismatch() {
    let mut eval = Evaluator::new();
    let result = eval.eval_str("mut s: String = false");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("Type error"));
}

#[test]
fn test_union_type_number_or_string_with_number() {
    let mut eval = Evaluator::new();
    // Number | String should accept Number
    let result = eval.eval_str("let opt: Number | String = 42");
    assert!(result.is_ok());
}

#[test]
fn test_union_type_number_or_string_with_string() {
    let mut eval = Evaluator::new();
    // Number | String should accept String
    let result = eval.eval_str(r#"let opt: Number | String = "hello""#);
    assert!(result.is_ok());
}

#[test]
fn test_union_type_mismatch() {
    let mut eval = Evaluator::new();
    // Number | String should reject Boolean
    let result = eval.eval_str("let opt: Number | String = true");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("Type error"));
}

#[test]
fn test_complex_type_valid() {
    let mut eval = Evaluator::new();
    let result = eval.eval_str("let c: Complex = 3 + 4i");
    assert!(result.is_ok());
}

#[test]
fn test_complex_type_mismatch() {
    let mut eval = Evaluator::new();
    let result = eval.eval_str("let c: Complex = 42");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("Type error"));
}

#[test]
fn test_vector_type_valid() {
    let mut eval = Evaluator::new();
    let result = eval.eval_str("let v: Vector = [1, 2, 3]");
    assert!(result.is_ok());
}

#[test]
fn test_vector_type_mismatch() {
    let mut eval = Evaluator::new();
    let result = eval.eval_str("let v: Vector = 42");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("Type error"));
}

#[test]
fn test_any_type_accepts_everything() {
    let mut eval = Evaluator::new();
    // Any type should accept any value
    assert!(eval.eval_str("let a: Any = 42").is_ok());
    assert!(eval.eval_str(r#"let b: Any = "hello""#).is_ok());
    assert!(eval.eval_str("let c: Any = true").is_ok());
    assert!(eval.eval_str("let d: Any = [1, 2, 3]").is_ok());
}

#[test]
fn test_no_type_annotation_accepts_anything() {
    let mut eval = Evaluator::new();
    // Without type annotation, any value is accepted
    assert!(eval.eval_str("let x = 42").is_ok());
    assert!(eval.eval_str(r#"let y = "hello""#).is_ok());
    assert!(eval.eval_str("let z = true").is_ok());
}

#[test]
fn test_expression_evaluation_with_type() {
    let mut eval = Evaluator::new();
    // Type checking happens after evaluating the expression
    let result = eval.eval_str("let sum: Number = 2 + 3 * 4");
    assert!(result.is_ok());
}

#[test]
fn test_expression_type_mismatch() {
    let mut eval = Evaluator::new();
    // Boolean expression assigned to Number should fail
    let result = eval.eval_str("let x: Number = 5 > 3");
    assert!(result.is_err());
}

#[test]
fn test_multiple_union_types() {
    let mut eval = Evaluator::new();
    // String | Number | Boolean union
    assert!(eval.eval_str("let x: String | Number | Boolean = 42").is_ok());
    assert!(eval.eval_str(r#"let y: String | Number | Boolean = "test""#).is_ok());
    assert!(eval.eval_str("let z: String | Number | Boolean = false").is_ok());
}
