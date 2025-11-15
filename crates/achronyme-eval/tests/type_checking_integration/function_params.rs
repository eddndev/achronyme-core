//! Function Parameter Type Checking Tests

use achronyme_eval::Evaluator;

#[test]
fn test_function_parameter_type_valid() {
    let mut eval = Evaluator::new();
    let result = eval.eval_str(r#"
        let fun = (a: Number, b: Number): Number => a + b;
        fun(10, 20)
    "#);
    assert!(result.is_ok());
    match result.unwrap() {
        achronyme_types::value::Value::Number(n) => assert_eq!(n, 30.0),
        other => panic!("Expected Number(30), got {:?}", other),
    }
}

#[test]
fn test_function_parameter_type_mismatch() {
    let mut eval = Evaluator::new();
    let result = eval.eval_str(r#"
        let fun = (a: Number, b: Number): Number => a + b;
        fun(10, "Hola")
    "#);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("Type error"));
    assert!(err.contains("argument 2"));
    assert!(err.contains("parameter 'b'"));
    assert!(err.contains("Number"));
    assert!(err.contains("String"));
}

#[test]
fn test_function_first_argument_type_mismatch() {
    let mut eval = Evaluator::new();
    let result = eval.eval_str(r#"
        let fun = (a: String, b: Number) => a;
        fun(42, 10)
    "#);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("Type error"));
    assert!(err.contains("argument 1"));
    assert!(err.contains("parameter 'a'"));
}

#[test]
fn test_function_return_type_valid() {
    let mut eval = Evaluator::new();
    let result = eval.eval_str(r#"
        let fun = (x: Number): Number => x * 2;
        fun(5)
    "#);
    assert!(result.is_ok());
    match result.unwrap() {
        achronyme_types::value::Value::Number(n) => assert_eq!(n, 10.0),
        other => panic!("Expected Number(10), got {:?}", other),
    }
}

#[test]
fn test_function_return_type_mismatch() {
    let mut eval = Evaluator::new();
    let result = eval.eval_str(r#"
        let fun = (x: Number): Number => "not a number";
        fun(5)
    "#);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("Type error"));
    assert!(err.contains("return type"));
}

#[test]
fn test_function_union_parameter_type() {
    let mut eval = Evaluator::new();
    // Function with union type parameter - accepts Number
    let result = eval.eval_str(r#"
        let process = (val: Number | String) => val;
        process(42)
    "#);
    assert!(result.is_ok());

    // Also accepts String
    let result2 = eval.eval_str(r#"
        let process2 = (val: Number | String) => val;
        process2("hello")
    "#);
    assert!(result2.is_ok());
}

#[test]
fn test_function_union_parameter_rejects_wrong_type() {
    let mut eval = Evaluator::new();
    let result = eval.eval_str(r#"
        let process = (val: Number | String) => val;
        process(true)
    "#);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("Type error"));
}

#[test]
fn test_function_optional_parameter() {
    let mut eval = Evaluator::new();
    // Nullable parameter type - accepts null
    let result = eval.eval_str(r#"
        let maybeDouble = (x: Number | null) => x;
        maybeDouble(null)
    "#);
    assert!(result.is_ok());

    // Also accepts Number
    let result2 = eval.eval_str(r#"
        let maybeDouble2 = (x: Number | null) => x;
        maybeDouble2(10)
    "#);
    assert!(result2.is_ok());
}

#[test]
fn test_function_no_type_annotation_accepts_any() {
    let mut eval = Evaluator::new();
    // Without type annotations, any value is accepted
    let result = eval.eval_str(r#"
        let identity = x => x;
        identity(42)
    "#);
    assert!(result.is_ok());

    let result2 = eval.eval_str(r#"
        let identity2 = x => x;
        identity2("hello")
    "#);
    assert!(result2.is_ok());
}

#[test]
fn test_function_mixed_typed_untyped_params() {
    let mut eval = Evaluator::new();
    // Mix of typed and untyped parameters
    let result = eval.eval_str(r#"
        let mixed = (a: Number, b) => a + b;
        mixed(10, 20)
    "#);
    assert!(result.is_ok());

    // b can be anything (string concatenation with number)
    let result2 = eval.eval_str(r#"
        let mixed2 = (a: Number, b) => a + b;
        mixed2(10, "test")
    "#);
    assert!(result2.is_ok());
}

#[test]
fn test_function_complex_type_parameter() {
    let mut eval = Evaluator::new();
    let result = eval.eval_str(r#"
        let processComplex = (c: Complex) => c;
        processComplex(3 + 4i)
    "#);
    assert!(result.is_ok());
}

#[test]
fn test_function_vector_type_parameter() {
    let mut eval = Evaluator::new();
    let result = eval.eval_str(r#"
        let processVec = (v: Vector) => v;
        processVec([1, 2, 3])
    "#);
    assert!(result.is_ok());
}

#[test]
fn test_higher_order_function_with_types() {
    let mut eval = Evaluator::new();
    // Higher-order function respects types
    let result = eval.eval_str(r#"
        let apply = (f, x: Number) => f(x);
        let double = (n: Number): Number => n * 2;
        apply(double, 5)
    "#);
    assert!(result.is_ok());
}
