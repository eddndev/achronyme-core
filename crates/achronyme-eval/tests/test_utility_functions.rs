/// Integration tests for utility functions
///
/// Tests: print, type, str

use achronyme_eval::Evaluator;
use achronyme_types::value::Value;

// ============================================================================
// type() tests
// ============================================================================

#[test]
fn test_type_number() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("type(42)").unwrap();
    assert_eq!(result, Value::String("Number".to_string()));
}

#[test]
fn test_type_float() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("type(3.14)").unwrap();
    assert_eq!(result, Value::String("Number".to_string()));
}

#[test]
fn test_type_string() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str(r#"type("hello")"#).unwrap();
    assert_eq!(result, Value::String("String".to_string()));
}

#[test]
fn test_type_boolean_true() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("type(true)").unwrap();
    assert_eq!(result, Value::String("Boolean".to_string()));
}

#[test]
fn test_type_boolean_false() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("type(false)").unwrap();
    assert_eq!(result, Value::String("Boolean".to_string()));
}

#[test]
fn test_type_vector() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str(r#"type(["a", "b", "c"])"#).unwrap();
    assert_eq!(result, Value::String("Vector".to_string()));
}

#[test]
fn test_type_tensor() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("type([1, 2, 3])").unwrap();
    assert_eq!(result, Value::String("Tensor".to_string()));
}

#[test]
fn test_type_complex() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("type(3 + 4i)").unwrap();
    assert_eq!(result, Value::String("Complex".to_string()));
}

#[test]
fn test_type_function() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("type(x => x + 1)").unwrap();
    assert_eq!(result, Value::String("Function".to_string()));
}

#[test]
fn test_type_record() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("type({x: 10, y: 20})").unwrap();
    assert_eq!(result, Value::String("Record".to_string()));
}

#[test]
fn test_type_with_variable() {
    let mut evaluator = Evaluator::new();

    let code = r#"
        let x = 42;
        type(x)
    "#;
    let result = evaluator.eval_str(code).unwrap();
    assert_eq!(result, Value::String("Number".to_string()));
}

// ============================================================================
// str() tests
// ============================================================================

#[test]
fn test_str_number_integer() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("str(42)").unwrap();
    assert_eq!(result, Value::String("42".to_string()));
}

#[test]
fn test_str_number_float() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("str(3.14)").unwrap();
    assert_eq!(result, Value::String("3.14".to_string()));
}

#[test]
fn test_str_string() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str(r#"str("hello")"#).unwrap();
    assert_eq!(result, Value::String("hello".to_string()));
}

#[test]
fn test_str_boolean_true() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("str(true)").unwrap();
    assert_eq!(result, Value::String("true".to_string()));
}

#[test]
fn test_str_boolean_false() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("str(false)").unwrap();
    assert_eq!(result, Value::String("false".to_string()));
}

#[test]
fn test_str_vector() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str(r#"str(["a", "b", "c"])"#).unwrap();
    assert_eq!(result, Value::String("[a, b, c]".to_string()));
}

#[test]
fn test_str_tensor() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("str([1, 2, 3])").unwrap();
    assert_eq!(result, Value::String("[1, 2, 3]".to_string()));
}

#[test]
fn test_str_complex() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("str(3 + 4i)").unwrap();
    assert_eq!(result, Value::String("3+4i".to_string()));
}

#[test]
fn test_str_complex_negative_im() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("str(3 - 4i)").unwrap();
    assert_eq!(result, Value::String("3-4i".to_string()));
}

#[test]
fn test_str_record() {
    let mut evaluator = Evaluator::new();

    // Note: HashMap iteration order is not guaranteed
    let result = evaluator.eval_str("str({x: 10})").unwrap();
    match result {
        Value::String(s) => assert!(s.contains("x: 10")),
        _ => panic!("Expected string"),
    }
}

#[test]
fn test_str_concatenation() {
    let mut evaluator = Evaluator::new();

    let code = r#"
        let x = 42;
        "The answer is " + str(x)
    "#;
    let result = evaluator.eval_str(code).unwrap();
    assert_eq!(result, Value::String("The answer is 42".to_string()));
}

// ============================================================================
// print() tests
// ============================================================================

#[test]
fn test_print_single_number() {
    let mut evaluator = Evaluator::new();

    // print returns the value it printed
    let result = evaluator.eval_str("print(42)").unwrap();
    assert_eq!(result, Value::Number(42.0));
}

#[test]
fn test_print_single_string() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str(r#"print("hello")"#).unwrap();
    assert_eq!(result, Value::String("hello".to_string()));
}

#[test]
fn test_print_multiple_values() {
    let mut evaluator = Evaluator::new();

    // print returns the last value
    let result = evaluator.eval_str(r#"print("hello", 42, true)"#).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_print_with_expression() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("print(2 + 2)").unwrap();
    assert_eq!(result, Value::Number(4.0));
}

#[test]
fn test_print_vector() {
    let mut evaluator = Evaluator::new();

    let code = r#"
        let arr = [1, 2, 3];
        print(arr)
    "#;
    evaluator.eval_str(code).unwrap();
}

#[test]
fn test_print_wrong_arity() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("print()");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("at least 1"));
}

// ============================================================================
// Combined tests
// ============================================================================

#[test]
fn test_type_and_str_combined() {
    let mut evaluator = Evaluator::new();

    let code = r#"
        let x = 42;
        let t = type(x);
        "Type: " + t
    "#;
    let result = evaluator.eval_str(code).unwrap();
    assert_eq!(result, Value::String("Type: Number".to_string()));
}

#[test]
fn test_type_checking_pattern() {
    let mut evaluator = Evaluator::new();

    let code = r#"
        let x = 42;
        type(x) == "Number"
    "#;
    let result = evaluator.eval_str(code).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_str_with_complex_expression() {
    let mut evaluator = Evaluator::new();

    let code = r#"
        let nums = [1, 2, 3, 4, 5];
        let doubled = map(x => x * 2, nums);
        str(doubled)
    "#;
    let result = evaluator.eval_str(code).unwrap();
    assert_eq!(result, Value::String("[2, 4, 6, 8, 10]".to_string()));
}

#[test]
fn test_print_in_pipeline() {
    let mut evaluator = Evaluator::new();

    let code = r#"
        let result = pipe(
            5,
            x => x * 2,
            x => print(x),  // print returns the value
            x => x + 10
        );
        result
    "#;
    let result = evaluator.eval_str(code).unwrap();
    assert_eq!(result, Value::Number(20.0)); // (5 * 2) + 10 = 20
}

#[test]
fn test_debugging_with_print_and_type() {
    let mut evaluator = Evaluator::new();

    let code = r#"
        let x = [1, 2, 3];
        print("Type:", type(x));
        print("Value:", str(x));
        x
    "#;
    let result = evaluator.eval_str(code).unwrap();

    // Should return the array
    match result {
        Value::Tensor(_) => (),
        Value::Vector(_) => (),
        _ => panic!("Expected tensor or vector"),
    }
}

#[test]
fn test_type_with_map() {
    let mut evaluator = Evaluator::new();

    let code = r#"
        let values = [42, "hello", true, [1, 2, 3]];
        map(x => type(x), values)
    "#;
    let result = evaluator.eval_str(code).unwrap();

    match result {
        Value::Vector(vec) => {
            assert_eq!(vec.len(), 4);
            assert_eq!(vec[0], Value::String("Number".to_string()));
            assert_eq!(vec[1], Value::String("String".to_string()));
            assert_eq!(vec[2], Value::String("Boolean".to_string()));
        }
        _ => panic!("Expected vector"),
    }
}

#[test]
fn test_str_with_join() {
    let mut evaluator = Evaluator::new();

    let code = r#"
        let numbers = [1, 2, 3, 4, 5];
        let strings = map(n => str(n), numbers);
        join(strings, ", ")
    "#;
    let result = evaluator.eval_str(code).unwrap();
    assert_eq!(result, Value::String("1, 2, 3, 4, 5".to_string()));
}
