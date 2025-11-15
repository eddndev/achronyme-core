use achronyme_eval::Evaluator;
use achronyme_types::value::Value;

#[test]
fn test_pest_simple_arithmetic() {
    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_str("2 + 3 * 4").unwrap();
    assert_eq!(result, Value::Number(14.0)); // 2 + (3 * 4)
}

#[test]
fn test_pest_power_right_associative() {
    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_str("2^3^2").unwrap();
    assert_eq!(result, Value::Number(512.0)); // 2^(3^2) = 2^9
}

#[test]
fn test_pest_vector() {
    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_str("[1, 2, 3]").unwrap();
    match result {
        Value::Vector(v) => {
            assert_eq!(v, vec![Value::Number(1.0), Value::Number(2.0), Value::Number(3.0)]);
        }
        _ => panic!("Expected vector"),
    }
}

#[test]
fn test_pest_function_call() {
    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_str("sin(0)").unwrap();
    match result {
        Value::Number(x) => assert!(x.abs() < 1e-10),
        _ => panic!("Expected number"),
    }
}

#[test]
fn test_pest_let_and_reference() {
    let mut evaluator = Evaluator::new();
    evaluator.eval_str("let x = 42").unwrap();
    let result = evaluator.eval_str("x").unwrap();
    assert_eq!(result, Value::Number(42.0));
}

#[test]
fn test_pest_lambda() {
    let mut evaluator = Evaluator::new();
    evaluator.eval_str("let f = x => x^2").unwrap();
    // Lambda should be stored in environment
    assert!(evaluator.environment().get("f").is_ok());
}

#[test]
fn test_pest_complex_expression() {
    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_str("(2 + 3) * (4 - 1)").unwrap();
    assert_eq!(result, Value::Number(15.0)); // 5 * 3
}

#[test]
fn test_pest_matrix() {
    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_str("[[1, 2], [3, 4]]").unwrap();
    match result {
        Value::Tensor(t) if t.is_matrix() => {}, // Success - rank-2 tensor
        _ => panic!("Expected matrix (rank-2 tensor)"),
    }
}

#[test]
fn test_pest_comparison() {
    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_str("5 > 3").unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_pest_multiple_statements() {
    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_str("let x = 10\nlet y = 20\nx + y").unwrap();
    assert_eq!(result, Value::Number(30.0));
}

#[test]
fn test_pest_with_comments() {
    let mut evaluator = Evaluator::new();
    let source = "// This is a comment\nlet x = 42\n// Another comment\nx * 2";
    let result = evaluator.eval_str(source).unwrap();
    assert_eq!(result, Value::Number(84.0));
}

#[test]
fn test_pest_soc_style_script() {
    let mut evaluator = Evaluator::new();
    let source = r#"
// Test simple
let x = 10
let y = 20
x + y
"#;
    let result = evaluator.eval_str(source).unwrap();
    assert_eq!(result, Value::Number(30.0));
}
