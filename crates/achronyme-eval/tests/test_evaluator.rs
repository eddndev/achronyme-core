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

// ========================================================================
// Lambda and Closure Tests
// ========================================================================

#[test]
fn test_lambda_creation() {
    // Create a lambda
    let result = eval("x => x * 2").unwrap();

    // Should be a function value
    match result {
        Value::Function(_) => {}, // Success
        _ => panic!("Expected function value"),
    }
}

#[test]
fn test_lambda_call() {
    // Define lambda and call it
    let mut evaluator = Evaluator::new();
    eval_with_evaluator(&mut evaluator, "let f = x => x * 2").unwrap();

    // Now call it
    let result = eval_with_evaluator(&mut evaluator, "f(5)").unwrap();

    assert_eq!(result, Value::Number(10.0));
}

#[test]
fn test_lambda_closure() {
    // Lambda captures variable from outer scope
    let mut evaluator = Evaluator::new();
    eval_with_evaluator(&mut evaluator, "let x = 10").unwrap();

    // Create lambda that uses x
    eval_with_evaluator(&mut evaluator, "let f = y => x + y").unwrap();

    // Call lambda
    let result = eval_with_evaluator(&mut evaluator, "f(5)").unwrap();

    assert_eq!(result, Value::Number(15.0)); // 10 + 5
}

#[test]
fn test_lambda_multi_param() {
    // Lambda with multiple parameters
    let mut evaluator = Evaluator::new();
    eval_with_evaluator(&mut evaluator, "let add = (x, y) => x + y").unwrap();

    // Call it
    let result = eval_with_evaluator(&mut evaluator, "add(3, 4)").unwrap();

    assert_eq!(result, Value::Number(7.0));
}

#[test]
fn test_lambda_arity_check() {
    // Lambda arity mismatch should fail
    let mut evaluator = Evaluator::new();
    eval_with_evaluator(&mut evaluator, "let f = x => x * 2").unwrap();

    // Call with wrong number of args
    let result = eval_with_evaluator(&mut evaluator, "f(1, 2)");

    assert!(result.is_err());
}

#[test]
fn test_lambda_nested() {
    // Nested lambda (higher-order function)
    let mut evaluator = Evaluator::new();
    eval_with_evaluator(&mut evaluator, "let makeAdder = x => (y => x + y)").unwrap();

    // Get an adder function
    eval_with_evaluator(&mut evaluator, "let add5 = makeAdder(5)").unwrap();

    // Use it
    let result = eval_with_evaluator(&mut evaluator, "add5(3)").unwrap();

    assert_eq!(result, Value::Number(8.0)); // 5 + 3
}

// ========================================================================
// Higher-Order Functions Tests
// ========================================================================

#[test]
fn test_map_single_collection() {
    // map(x => x * 2, [1,2,3]) → [2,4,6]
    let result = eval("map(x => x * 2,[1,2,3])").unwrap();
    match result {
        Value::Vector(v) => {
            assert_eq!(v.len(), 3);
            assert_eq!(v[0], Value::Number(2.0));
            assert_eq!(v[1], Value::Number(4.0));
            assert_eq!(v[2], Value::Number(6.0));
        }
        _ => panic!("Expected vector"),
    }
}

#[test]
fn test_map_multi_collection() {
    // map((x,y) => x + y, [1,2,3], [4,5,6]) → [5,7,9]
    let result = eval("map((x,y) => x + y,[1,2,3],[4,5,6])").unwrap();
    match result {
        Value::Vector(v) => {
            assert_eq!(v.len(), 3);
            assert_eq!(v[0], Value::Number(5.0));
            assert_eq!(v[1], Value::Number(7.0));
            assert_eq!(v[2], Value::Number(9.0));
        }
        _ => panic!("Expected vector"),
    }
}

#[test]
fn test_map_truncates_to_shortest() {
    // map((x,y) => x + y, [1,2], [3,4,5,6]) → [4,6] (truncates)
    let result = eval("map((x,y) => x + y,[1,2],[3,4,5,6])").unwrap();
    match result {
        Value::Vector(v) => {
            assert_eq!(v.len(), 2);
            assert_eq!(v[0], Value::Number(4.0));
            assert_eq!(v[1], Value::Number(6.0));
        }
        _ => panic!("Expected vector"),
    }
}

#[test]
fn test_filter() {
    // filter(x => x > 2, [1,2,3,4,5]) → [3,4,5]
    let result = eval("filter(x => x > 2,[1,2,3,4,5])").unwrap();
    match result {
        Value::Vector(v) => {
            assert_eq!(v.len(), 3);
            assert_eq!(v[0], Value::Number(3.0));
            assert_eq!(v[1], Value::Number(4.0));
            assert_eq!(v[2], Value::Number(5.0));
        }
        _ => panic!("Expected vector"),
    }
}

#[test]
fn test_filter_even_numbers() {
    // filter(x => x % 2 == 0, [1,2,3,4,5,6]) → [2,4,6]
    // Note: == returns boolean
    let result = eval("filter(x => (x % 2) == 0,[1,2,3,4,5,6])").unwrap();
    match result {
        Value::Vector(v) => {
            assert_eq!(v.len(), 3);
            assert_eq!(v[0], Value::Number(2.0));
            assert_eq!(v[1], Value::Number(4.0));
            assert_eq!(v[2], Value::Number(6.0));
        }
        _ => panic!("Expected vector"),
    }
}

#[test]
fn test_reduce_sum() {
    // reduce((acc, x) => acc + x, 0, [1,2,3,4]) → 10
    let result = eval("reduce((acc,x) => acc + x,0,[1,2,3,4])").unwrap();
    assert_eq!(result, Value::Number(10.0));
}

#[test]
fn test_reduce_product() {
    // reduce((acc, x) => acc * x, 1, [2,3,4]) → 24
    let result = eval("reduce((acc,x) => acc * x,1,[2,3,4])").unwrap();
    assert_eq!(result, Value::Number(24.0));
}

#[test]
fn test_reduce_max() {
    // reduce((acc, x) => max(acc, x), 0, [3,1,4,1,5,9]) → 9
    let result = eval("reduce((acc,x) => max(acc,x),0,[3,1,4,1,5,9])").unwrap();
    assert_eq!(result, Value::Number(9.0));
}

#[test]
fn test_pipe_simple() {
    // pipe(5, x => x * 2, x => x + 1) → 11
    let result = eval("pipe(5,x => x * 2,x => x + 1)").unwrap();
    assert_eq!(result, Value::Number(11.0));
}

#[test]
fn test_pipe_multiple_functions() {
    // pipe(2, x => x + 1, x => x * 2, x => x ^ 2) → 36
    // 2 → 3 → 6 → 36
    let result = eval("pipe(2,x => x + 1,x => x * 2,x => x ^ 2)").unwrap();
    assert_eq!(result, Value::Number(36.0));
}

#[test]
fn test_hof_composition() {
    // Test combining HOFs
    // Get squares of even numbers: filter(even) then map(square)
    let mut evaluator = Evaluator::new();
    eval_with_evaluator(&mut evaluator, "let evens = filter(x => (x % 2) == 0,[1,2,3,4,5,6])").unwrap();

    // Now map square over evens
    let result = eval_with_evaluator(&mut evaluator, "map(x => x ^ 2,evens)").unwrap();

    match result {
        Value::Vector(v) => {
            assert_eq!(v.len(), 3);
            assert_eq!(v[0], Value::Number(4.0));  // 2^2
            assert_eq!(v[1], Value::Number(16.0)); // 4^2
            assert_eq!(v[2], Value::Number(36.0)); // 6^2
        }
        _ => panic!("Expected vector"),
    }
}

#[test]
fn test_map_arity_mismatch() {
    // map with wrong function arity should fail
    let result = eval("map((x,y) => x + y,[1,2,3])");
    assert!(result.is_err());
}

#[test]
fn test_filter_non_unary_predicate() {
    // filter with non-unary predicate should fail
    let result = eval("filter((x,y) => x + y,[1,2,3])");
    assert!(result.is_err());
}

#[test]
fn test_reduce_non_binary_function() {
    // reduce with non-binary function should fail
    let result = eval("reduce(x => x * 2,0,[1,2,3])");
    assert!(result.is_err());
}

#[test]
fn test_pipe_non_unary_function() {
    // pipe with non-unary function should fail
    let result = eval("pipe(5,(x,y) => x + y)");
    assert!(result.is_err());
}

// ========================================================================
// Pest Parser Tests (New)
// ========================================================================

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
            assert_eq!(v, &vec![Value::Number(1.0), Value::Number(2.0), Value::Number(3.0)]);
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
        Value::Matrix(_) => {}, // Success
        _ => panic!("Expected matrix"),
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

// ========================================================================
// Conditional Tests (if, boolean, logical operators)
// ========================================================================

#[test]
fn test_boolean_literals() {
    assert_eq!(eval("true").unwrap(), Value::Boolean(true));
    assert_eq!(eval("false").unwrap(), Value::Boolean(false));
}

#[test]
fn test_logical_and() {
    assert_eq!(eval("true && true").unwrap(), Value::Boolean(true));
    assert_eq!(eval("true && false").unwrap(), Value::Boolean(false));
    assert_eq!(eval("false && true").unwrap(), Value::Boolean(false));
    assert_eq!(eval("false && false").unwrap(), Value::Boolean(false));
}

#[test]
fn test_logical_or() {
    assert_eq!(eval("true || true").unwrap(), Value::Boolean(true));
    assert_eq!(eval("true || false").unwrap(), Value::Boolean(true));
    assert_eq!(eval("false || true").unwrap(), Value::Boolean(true));
    assert_eq!(eval("false || false").unwrap(), Value::Boolean(false));
}

#[test]
fn test_logical_not() {
    assert_eq!(eval("!true").unwrap(), Value::Boolean(false));
    assert_eq!(eval("!false").unwrap(), Value::Boolean(true));
    assert_eq!(eval("!!true").unwrap(), Value::Boolean(true));
    assert_eq!(eval("!!false").unwrap(), Value::Boolean(false));
}

#[test]
fn test_comparison_returns_boolean() {
    assert_eq!(eval("5 > 3").unwrap(), Value::Boolean(true));
    assert_eq!(eval("5 < 3").unwrap(), Value::Boolean(false));
    assert_eq!(eval("5 >= 5").unwrap(), Value::Boolean(true));
    assert_eq!(eval("5 <= 3").unwrap(), Value::Boolean(false));
    assert_eq!(eval("5 == 5").unwrap(), Value::Boolean(true));
    assert_eq!(eval("5 != 3").unwrap(), Value::Boolean(true));
}

#[test]
fn test_if_simple() {
    assert_eq!(eval("if(true, 1, 2)").unwrap(), Value::Number(1.0));
    assert_eq!(eval("if(false, 1, 2)").unwrap(), Value::Number(2.0));
}

#[test]
fn test_if_with_comparison() {
    assert_eq!(eval("if(5 > 3, 100, 200)").unwrap(), Value::Number(100.0));
    assert_eq!(eval("if(2 > 10, 100, 200)").unwrap(), Value::Number(200.0));
}

#[test]
fn test_if_with_logical_ops() {
    assert_eq!(eval("if(true && true, 1, 0)").unwrap(), Value::Number(1.0));
    assert_eq!(eval("if(true && false, 1, 0)").unwrap(), Value::Number(0.0));
    assert_eq!(eval("if(false || true, 1, 0)").unwrap(), Value::Number(1.0));
}

#[test]
fn test_if_nested() {
    // if(x > 0, if(x > 10, 2, 1), 0)
    let mut evaluator = Evaluator::new();
    eval_with_evaluator(&mut evaluator, "let x = 15").unwrap();
    let result = eval_with_evaluator(&mut evaluator, "if(x > 0, if(x > 10, 2, 1), 0)").unwrap();
    assert_eq!(result, Value::Number(2.0));

    let mut evaluator = Evaluator::new();
    eval_with_evaluator(&mut evaluator, "let x = 5").unwrap();
    let result = eval_with_evaluator(&mut evaluator, "if(x > 0, if(x > 10, 2, 1), 0)").unwrap();
    assert_eq!(result, Value::Number(1.0));

    let mut evaluator = Evaluator::new();
    eval_with_evaluator(&mut evaluator, "let x = -5").unwrap();
    let result = eval_with_evaluator(&mut evaluator, "if(x > 0, if(x > 10, 2, 1), 0)").unwrap();
    assert_eq!(result, Value::Number(0.0));
}

#[test]
fn test_if_in_lambda() {
    // abs function: x => if(x < 0, -x, x)
    let mut evaluator = Evaluator::new();
    eval_with_evaluator(&mut evaluator, "let abs = x => if(x < 0, -x, x)").unwrap();

    let result = eval_with_evaluator(&mut evaluator, "abs(-5)").unwrap();
    assert_eq!(result, Value::Number(5.0));

    let result = eval_with_evaluator(&mut evaluator, "abs(3)").unwrap();
    assert_eq!(result, Value::Number(3.0));
}

#[test]
fn test_relu_activation() {
    // ReLU: x => if(x > 0, x, 0)
    let mut evaluator = Evaluator::new();
    eval_with_evaluator(&mut evaluator, "let relu = x => if(x > 0, x, 0)").unwrap();

    let result = eval_with_evaluator(&mut evaluator, "relu(5)").unwrap();
    assert_eq!(result, Value::Number(5.0));

    let result = eval_with_evaluator(&mut evaluator, "relu(-3)").unwrap();
    assert_eq!(result, Value::Number(0.0));

    let result = eval_with_evaluator(&mut evaluator, "relu(0)").unwrap();
    assert_eq!(result, Value::Number(0.0));
}

#[test]
fn test_sign_function() {
    // sign: x => if(x < 0, -1, if(x > 0, 1, 0))
    let mut evaluator = Evaluator::new();
    eval_with_evaluator(&mut evaluator, "let sign = x => if(x < 0, -1, if(x > 0, 1, 0))").unwrap();

    let result = eval_with_evaluator(&mut evaluator, "sign(-10)").unwrap();
    assert_eq!(result, Value::Number(-1.0));

    let result = eval_with_evaluator(&mut evaluator, "sign(10)").unwrap();
    assert_eq!(result, Value::Number(1.0));

    let result = eval_with_evaluator(&mut evaluator, "sign(0)").unwrap();
    assert_eq!(result, Value::Number(0.0));
}

// ========================================================================
// Piecewise Tests
// ========================================================================

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
