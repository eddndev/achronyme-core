//! Integration tests for type checking in the evaluator
//!
//! These tests verify that type annotations are enforced at runtime.

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

// ============================================================================
// Null Type Tests
// ============================================================================

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

// ============================================================================
// Function Parameter Type Checking Tests
// ============================================================================

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

// ============================================================================
// Assignment Type Checking Tests
// ============================================================================

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

// ============================================================================
// Function Type Annotation Tests
// ============================================================================

#[test]
fn test_function_type_annotation_basic() {
    let mut eval = Evaluator::new();
    let result = eval.eval_str(r#"
        let add: (Number, Number) => Number = (a, b) => a + b;
        add(10, 20)
    "#);
    assert!(result.is_ok());
    match result.unwrap() {
        achronyme_types::value::Value::Number(n) => assert_eq!(n, 30.0),
        other => panic!("Expected Number(30), got {:?}", other),
    }
}

#[test]
fn test_function_type_annotation_enforces_param_types() {
    let mut eval = Evaluator::new();
    let result = eval.eval_str(r#"
        let add: (Number, Number) => Number = (a, b) => a + b;
        add(10, "Hola")
    "#);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("Type error"));
    assert!(err.contains("argument 2"));
    assert!(err.contains("Number"));
    assert!(err.contains("String"));
}

#[test]
fn test_function_type_annotation_enforces_return_type() {
    let mut eval = Evaluator::new();
    let result = eval.eval_str(r#"
        let getNum: () => Number = () => "not a number";
        getNum()
    "#);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("Type error"));
    assert!(err.contains("return type"));
}

#[test]
fn test_function_type_mismatch_not_a_function() {
    let mut eval = Evaluator::new();
    let result = eval.eval_str(r#"
        let add: (Number, Number) => Number = "not a function"
    "#);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("Type error"));
    assert!(err.contains("String"));
}

#[test]
fn test_function_type_with_any_param() {
    let mut eval = Evaluator::new();
    let result = eval.eval_str(r#"
        let toString: (Any) => String = x => "Value: " + x;
        toString(42)
    "#);
    assert!(result.is_ok());
}

#[test]
fn test_higher_order_function_type() {
    let mut eval = Evaluator::new();
    let result = eval.eval_str(r#"
        let curry: (Number) => ((Number) => Number) = a => b => a + b;
        let add5 = curry(5);
        add5(10)
    "#);
    assert!(result.is_ok());
    match result.unwrap() {
        achronyme_types::value::Value::Number(n) => assert_eq!(n, 15.0),
        other => panic!("Expected Number(15), got {:?}", other),
    }
}

#[test]
fn test_function_type_no_params() {
    let mut eval = Evaluator::new();
    let result = eval.eval_str(r#"
        let getAnswer: () => Number = () => 42;
        getAnswer()
    "#);
    assert!(result.is_ok());
    match result.unwrap() {
        achronyme_types::value::Value::Number(n) => assert_eq!(n, 42.0),
        other => panic!("Expected Number(42), got {:?}", other),
    }
}

#[test]
fn test_function_type_with_union_param() {
    let mut eval = Evaluator::new();
    let result = eval.eval_str(r#"
        let process: (Number | String) => Boolean = x => true;
        process(42)
    "#);
    assert!(result.is_ok());
}

// ============================================================================
// Edge Type Annotation Tests
// ============================================================================

#[test]
fn test_edge_type_annotation_valid() {
    let mut eval = Evaluator::new();
    let result = eval.eval_str(r#"
        let edge: Edge = A -> B;
        edge
    "#);
    assert!(result.is_ok());
}

#[test]
fn test_edge_type_annotation_mismatch() {
    let mut eval = Evaluator::new();
    let result = eval.eval_str(r#"
        let edge: Edge = 42
    "#);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("Type error"));
    assert!(err.contains("Edge"));
    assert!(err.contains("Number"));
}

#[test]
fn test_edge_type_undirected() {
    let mut eval = Evaluator::new();
    let result = eval.eval_str(r#"
        let edge: Edge = A <> B;
        edge
    "#);
    assert!(result.is_ok());
}

#[test]
fn test_edge_union_type() {
    let mut eval = Evaluator::new();
    // Edge can be part of union types
    let result = eval.eval_str(r#"
        let maybeEdge: Edge | null = null
    "#);
    assert!(result.is_ok());

    let result2 = eval.eval_str(r#"
        let maybeEdge2: Edge | null = A -> B
    "#);
    assert!(result2.is_ok());
}
