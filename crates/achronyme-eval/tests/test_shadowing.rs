use achronyme_eval::Evaluator;
use achronyme_parser::parse;
use achronyme_types::value::Value;

fn eval(source: &str) -> Result<Value, String> {
    let statements = parse(source)?;
    let mut evaluator = Evaluator::new();

    // Evaluate all statements, return the last result
    let mut result = Value::Number(0.0);
    for stmt in &statements {
        result = evaluator.evaluate(stmt)?;
    }

    Ok(result)
}

/// Helper function for tests that need to maintain state across multiple eval calls
fn eval_with_evaluator(evaluator: &mut Evaluator, source: &str) -> Result<Value, String> {
    let statements = parse(source)?;

    // Evaluate all statements, return the last result
    let mut result = Value::Number(0.0);
    for stmt in &statements {
        result = evaluator.evaluate(stmt)?;
    }

    Ok(result)
}

// ========================================================================
// Variable Shadowing Tests
// ========================================================================

#[test]
fn test_shadowing_in_lambda_parameters() {
    // Lambda parameter should shadow outer variable
    let mut evaluator = Evaluator::new();
    eval_with_evaluator(&mut evaluator, "let z = 10").unwrap();

    // Lambda parameter 'z' should shadow outer 'z'
    eval_with_evaluator(&mut evaluator, "let g = (z) => z * 2").unwrap();

    // Call with argument 3, should use parameter (not outer z=10)
    let result = eval_with_evaluator(&mut evaluator, "g(3)").unwrap();
    assert_eq!(result, Value::Number(6.0)); // Should be 3*2, not 10*2

    // Outer z should still be 10
    let result = eval_with_evaluator(&mut evaluator, "z").unwrap();
    assert_eq!(result, Value::Number(10.0));
}

#[test]
fn test_shadowing_with_let_redeclaration() {
    // Should be able to redefine variables in same scope
    let mut evaluator = Evaluator::new();

    eval_with_evaluator(&mut evaluator, "let x = 5").unwrap();
    assert_eq!(eval_with_evaluator(&mut evaluator, "x").unwrap(), Value::Number(5.0));

    // Redefine x in the same scope
    eval_with_evaluator(&mut evaluator, "let x = 10").unwrap();
    assert_eq!(eval_with_evaluator(&mut evaluator, "x").unwrap(), Value::Number(10.0));

    // Redefine again
    eval_with_evaluator(&mut evaluator, "let x = 20").unwrap();
    assert_eq!(eval_with_evaluator(&mut evaluator, "x").unwrap(), Value::Number(20.0));
}

#[test]
fn test_shadowing_pipeline_transformation() {
    // Simulating: let v = [...]; let v = fft(v); let v = map(abs, v)
    let mut evaluator = Evaluator::new();

    eval_with_evaluator(&mut evaluator, "let v = [1, 2, 3, 4]").unwrap();
    assert_eq!(
        eval_with_evaluator(&mut evaluator, "v").unwrap(),
        Value::Vector(achronyme_types::vector::Vector::new(vec![1.0, 2.0, 3.0, 4.0]))
    );

    // Transform: square each element
    eval_with_evaluator(&mut evaluator, "let v = map(x => x^2, v)").unwrap();
    assert_eq!(
        eval_with_evaluator(&mut evaluator, "v").unwrap(),
        Value::Vector(achronyme_types::vector::Vector::new(vec![1.0, 4.0, 9.0, 16.0]))
    );

    // Transform again: add 1 to each
    eval_with_evaluator(&mut evaluator, "let v = map(x => x + 1, v)").unwrap();
    assert_eq!(
        eval_with_evaluator(&mut evaluator, "v").unwrap(),
        Value::Vector(achronyme_types::vector::Vector::new(vec![2.0, 5.0, 10.0, 17.0]))
    );
}

#[test]
fn test_shadowing_in_nested_lambdas() {
    // Test lambda parameter shadowing
    let mut evaluator = Evaluator::new();

    // Outer variable x
    eval_with_evaluator(&mut evaluator, "let x = 100").unwrap();

    // Test with same variable name shadowing in lambda
    eval_with_evaluator(&mut evaluator, "let g = (x) => x + 1").unwrap();
    let result = eval_with_evaluator(&mut evaluator, "g(5)").unwrap();
    assert_eq!(result, Value::Number(6.0)); // Should use parameter x=5, not global x=100

    // Global x should still be 100
    assert_eq!(eval_with_evaluator(&mut evaluator, "x").unwrap(), Value::Number(100.0));

    // Test nested lambdas where inner closes over outer parameter
    eval_with_evaluator(&mut evaluator, "let makeAdder = (y) => (z) => y + z").unwrap();
    eval_with_evaluator(&mut evaluator, "let add5 = makeAdder(5)").unwrap();
    let result = eval_with_evaluator(&mut evaluator, "add5(3)").unwrap();
    assert_eq!(result, Value::Number(8.0)); // 5 + 3
}

#[test]
fn test_closure_captures_correct_shadowed_value() {
    // Test that closures capture the correct value when variables are shadowed
    let mut evaluator = Evaluator::new();

    eval_with_evaluator(&mut evaluator, "let x = 5").unwrap();
    eval_with_evaluator(&mut evaluator, "let f1 = (y) => x + y").unwrap(); // Captures x=5

    eval_with_evaluator(&mut evaluator, "let x = 10").unwrap(); // Shadow x
    eval_with_evaluator(&mut evaluator, "let f2 = (y) => x + y").unwrap(); // Captures x=10

    // f1 should use x=5 (captured at creation)
    let result = eval_with_evaluator(&mut evaluator, "f1(1)").unwrap();
    assert_eq!(result, Value::Number(6.0)); // 5 + 1

    // f2 should use x=10 (captured at creation)
    let result = eval_with_evaluator(&mut evaluator, "f2(1)").unwrap();
    assert_eq!(result, Value::Number(11.0)); // 10 + 1

    // Current x should be 10
    assert_eq!(eval_with_evaluator(&mut evaluator, "x").unwrap(), Value::Number(10.0));
}

#[test]
fn test_shadowing_with_map() {
    // Variable shadowing in map lambda
    let mut evaluator = Evaluator::new();

    eval_with_evaluator(&mut evaluator, "let x = 100").unwrap();

    // Map with lambda parameter x (should shadow outer x)
    let result = eval_with_evaluator(&mut evaluator, "map(x => x * 2, [1, 2, 3])").unwrap();

    match result {
        Value::Vector(v) => {
            assert_eq!(v.data(), &[2.0, 4.0, 6.0]); // Uses parameter, not outer x=100
        }
        _ => panic!("Expected vector"),
    }

    // Outer x should still be 100
    assert_eq!(eval_with_evaluator(&mut evaluator, "x").unwrap(), Value::Number(100.0));
}

#[test]
fn test_shadowing_with_filter() {
    // Variable shadowing in filter lambda
    let mut evaluator = Evaluator::new();

    eval_with_evaluator(&mut evaluator, "let threshold = 100").unwrap();

    // Filter with lambda parameter threshold (should shadow outer threshold)
    let result = eval_with_evaluator(&mut evaluator, "filter(threshold => threshold > 2, [1, 2, 3, 4])").unwrap();

    match result {
        Value::Vector(v) => {
            assert_eq!(v.data(), &[3.0, 4.0]); // Uses parameter
        }
        _ => panic!("Expected vector"),
    }

    // Outer threshold should still be 100
    assert_eq!(eval_with_evaluator(&mut evaluator, "threshold").unwrap(), Value::Number(100.0));
}

#[test]
fn test_shadowing_complex_example() {
    // Complex example from CHANGELOG
    let source = r#"
let x = 10
let f = (x) => x * 2
f(5)
"#;

    let result = eval(source).unwrap();
    assert_eq!(result, Value::Number(10.0)); // 5 * 2, not 10 * 2
}

#[test]
fn test_memory_efficiency_with_shadowing() {
    // Test that we can reuse variable names without keeping old values
    let mut evaluator = Evaluator::new();

    // Create a large vector
    eval_with_evaluator(&mut evaluator, "let v = [1, 2, 3, 4, 5]").unwrap();

    // "Transform" it (in reality, shadowing with new value)
    eval_with_evaluator(&mut evaluator, "let v = map(x => x * 2, v)").unwrap();

    // Should have the transformed value
    match eval_with_evaluator(&mut evaluator, "v").unwrap() {
        Value::Vector(vec) => {
            assert_eq!(vec.data(), &[2.0, 4.0, 6.0, 8.0, 10.0]);
        }
        _ => panic!("Expected vector"),
    }
}

#[test]
fn test_shadowing_with_complex_numbers() {
    // Test shadowing with complex numbers
    let mut evaluator = Evaluator::new();

    eval_with_evaluator(&mut evaluator, "let z = 1+2i").unwrap();
    eval_with_evaluator(&mut evaluator, "let f = (z) => z^2").unwrap();

    let result = eval_with_evaluator(&mut evaluator, "f(2+i)").unwrap();

    // (2+i)^2 = 4 + 4i - 1 = 3 + 4i
    match result {
        Value::Complex(c) => {
            assert!((c.re - 3.0).abs() < 1e-10);
            assert!((c.im - 4.0).abs() < 1e-10);
        }
        _ => panic!("Expected complex number"),
    }

    // Original z should still be 1+2i
    match eval_with_evaluator(&mut evaluator, "z").unwrap() {
        Value::Complex(c) => {
            assert_eq!(c.re, 1.0);
            assert_eq!(c.im, 2.0);
        }
        _ => panic!("Expected complex number"),
    }
}

#[test]
fn test_shadowing_preserves_closure_semantics() {
    // Verify that closures still work correctly with shadowing
    let mut evaluator = Evaluator::new();

    eval_with_evaluator(&mut evaluator, "let multiplier = 2").unwrap();
    eval_with_evaluator(&mut evaluator, "let scale = (x) => x * multiplier").unwrap();

    // Call should use captured multiplier=2
    let result = eval_with_evaluator(&mut evaluator, "scale(5)").unwrap();
    assert_eq!(result, Value::Number(10.0)); // 5 * 2

    // Shadow multiplier
    eval_with_evaluator(&mut evaluator, "let multiplier = 10").unwrap();

    // scale should still use old multiplier=2 (closure semantics)
    let result = eval_with_evaluator(&mut evaluator, "scale(5)").unwrap();
    assert_eq!(result, Value::Number(10.0)); // Still 5 * 2, not 5 * 10
}
