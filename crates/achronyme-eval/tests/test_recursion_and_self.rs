use achronyme_eval::Evaluator;
use achronyme_parser::parse;
use achronyme_types::value::Value;

// Helper function to evaluate a single expression
fn eval(source: &str) -> Result<Value, String> {
    let statements = parse(source)?;
    let mut evaluator = Evaluator::new();

    let mut result = Value::Number(0.0);
    for stmt in &statements {
        result = evaluator.evaluate(stmt)?;
    }

    Ok(result)
}

// Helper function for tests that need to maintain state across multiple eval calls
fn eval_with_evaluator(evaluator: &mut Evaluator, source: &str) -> Result<Value, String> {
    let statements = parse(source)?;

    let mut result = Value::Number(0.0);
    for stmt in &statements {
        result = evaluator.evaluate(stmt)?;
    }

    Ok(result)
}

// ============================================================================
// Tests for Recursive Functions with 'rec'
// ============================================================================

#[test]
fn test_rec_factorial() {
    let mut evaluator = Evaluator::new();

    // Define recursive factorial using rec
    let result = eval_with_evaluator(&mut evaluator, "let factorial = n => if(n <= 1, 1, n * rec(n - 1))");
    assert!(result.is_ok());

    // Test factorial(5) = 120
    let result = eval_with_evaluator(&mut evaluator, "factorial(5)");
    assert!(result.is_ok());
    match result.unwrap() {
        Value::Number(n) => assert_eq!(n, 120.0),
        _ => panic!("Expected number"),
    }
}

#[test]
fn test_rec_fibonacci() {
    let mut evaluator = Evaluator::new();

    // Define recursive fibonacci using rec
    let result = eval_with_evaluator(&mut evaluator, "let fib = n => if(n <= 1, n, rec(n-1) + rec(n-2))");
    assert!(result.is_ok());

    // Test fib(10) = 55
    let result = eval_with_evaluator(&mut evaluator, "fib(10)");
    assert!(result.is_ok());
    match result.unwrap() {
        Value::Number(n) => assert_eq!(n, 55.0),
        _ => panic!("Expected number"),
    }
}

#[test]
fn test_rec_outside_function_fails() {
    // Using 'rec' outside a function should fail
    let result = eval("let x = rec");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("can only be used inside functions"));
}

#[test]
fn test_anonymous_recursive_function() {
    // Test that anonymous functions can be recursive with rec
    let result = eval("map(n => if(n <= 1, 1, n * rec(n - 1)), [1, 2, 3, 4, 5])");
    assert!(result.is_ok());

    // Verify results: [1, 2, 6, 24, 120]
    match result.unwrap() {
        Value::Vector(v) => {
            assert_eq!(v.len(), 5);
            match (&v[0], &v[1], &v[2], &v[3], &v[4]) {
                (Value::Number(a), Value::Number(b), Value::Number(c), Value::Number(d), Value::Number(e)) => {
                    assert_eq!(*a, 1.0);
                    assert_eq!(*b, 2.0);
                    assert_eq!(*c, 6.0);
                    assert_eq!(*d, 24.0);
                    assert_eq!(*e, 120.0);
                }
                _ => panic!("Expected vector of numbers"),
            }
        }
        _ => panic!("Expected vector"),
    }
}

#[test]
fn test_rec_gcd() {
    let mut evaluator = Evaluator::new();

    // Euclidean algorithm for GCD using rec
    let result = eval_with_evaluator(&mut evaluator, "let gcd = (a, b) => if(b == 0, a, rec(b, a % b))");
    assert!(result.is_ok());

    // Test gcd(48, 18) = 6
    let result = eval_with_evaluator(&mut evaluator, "gcd(48, 18)");
    assert!(result.is_ok());
    match result.unwrap() {
        Value::Number(n) => assert_eq!(n, 6.0),
        _ => panic!("Expected number"),
    }
}

#[test]
fn test_rec_sum_range() {
    let mut evaluator = Evaluator::new();

    // Sum from 1 to n using rec
    let result = eval_with_evaluator(&mut evaluator, "let sumRange = n => if(n <= 0, 0, n + rec(n - 1))");
    assert!(result.is_ok());

    // Test sumRange(10) = 55
    let result = eval_with_evaluator(&mut evaluator, "sumRange(10)");
    assert!(result.is_ok());
    match result.unwrap() {
        Value::Number(n) => assert_eq!(n, 55.0),
        _ => panic!("Expected number"),
    }
}

// ============================================================================
// Tests for Self-Reference in Records
// ============================================================================

#[test]
fn test_self_access_field() {
    let mut evaluator = Evaluator::new();

    // Record with method that accesses 'self.value'
    let result = eval_with_evaluator(&mut evaluator, r#"
        let counter = {
            value: 10,
            getValue: () => self.value
        }
    "#);
    assert!(result.is_ok());

    // Call the method
    let result = eval_with_evaluator(&mut evaluator, "counter.getValue()");
    assert!(result.is_ok());
    match result.unwrap() {
        Value::Number(n) => assert_eq!(n, 10.0),
        _ => panic!("Expected number"),
    }
}

#[test]
fn test_self_call_another_method() {
    let mut evaluator = Evaluator::new();

    // Record with methods that call each other
    let result = eval_with_evaluator(&mut evaluator, r#"
        let math = {
            square: x => x * x,
            sumOfSquares: (a, b) => self.square(a) + self.square(b)
        }
    "#);
    assert!(result.is_ok());

    // Test sumOfSquares(3, 4) = 9 + 16 = 25
    let result = eval_with_evaluator(&mut evaluator, "math.sumOfSquares(3, 4)");
    assert!(result.is_ok());
    match result.unwrap() {
        Value::Number(n) => assert_eq!(n, 25.0),
        _ => panic!("Expected number"),
    }
}

#[test]
fn test_self_recursive_in_record() {
    let mut evaluator = Evaluator::new();

    // Record with recursive method
    let result = eval_with_evaluator(&mut evaluator, r#"
        let factorial = {
            compute: n => if(n <= 1, 1, n * self.compute(n - 1))
        }
    "#);
    assert!(result.is_ok());

    // Test factorial.compute(5) = 120
    let result = eval_with_evaluator(&mut evaluator, "factorial.compute(5)");
    assert!(result.is_ok());
    match result.unwrap() {
        Value::Number(n) => assert_eq!(n, 120.0),
        _ => panic!("Expected number"),
    }
}

#[test]
fn test_self_outside_record_fails() {
    // Using 'self' outside a record should fail
    let result = eval("let x = self.value");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("can only be used inside record"));
}

#[test]
fn test_self_in_lambda_outside_record_fails() {
    let mut evaluator = Evaluator::new();

    // 'self' in a lambda not in a record should fail
    let result = eval_with_evaluator(&mut evaluator, "let f = () => self.x");
    assert!(result.is_ok()); // Lambda definition succeeds

    // But calling it should fail
    let result = eval_with_evaluator(&mut evaluator, "f()");
    assert!(result.is_err());
}

#[test]
fn test_self_multiple_fields() {
    let mut evaluator = Evaluator::new();

    // Record with multiple fields accessed via self
    let result = eval_with_evaluator(&mut evaluator, r#"
        let circle = {
            radius: 5,
            pi: 3.14159,
            area: () => self.pi * self.radius * self.radius,
            circumference: () => 2 * self.pi * self.radius
        }
    "#);
    assert!(result.is_ok());

    // Test area
    let result = eval_with_evaluator(&mut evaluator, "circle.area()");
    assert!(result.is_ok());
    match result.unwrap() {
        Value::Number(n) => assert!((n - 78.53975).abs() < 0.0001),
        _ => panic!("Expected number"),
    }

    // Test circumference
    let result = eval_with_evaluator(&mut evaluator, "circle.circumference()");
    assert!(result.is_ok());
    match result.unwrap() {
        Value::Number(n) => assert!((n - 31.4159).abs() < 0.0001),
        _ => panic!("Expected number"),
    }
}

// ============================================================================
// Combined Tests: Recursion + Self-Reference
// ============================================================================

#[test]
fn test_rec_and_self_combined() {
    let mut evaluator = Evaluator::new();

    // Standalone recursive function using rec
    let result = eval_with_evaluator(&mut evaluator, "let factorial = n => if(n <= 1, 1, n * rec(n - 1))");
    assert!(result.is_ok());

    // Record with self-referencing method
    let result = eval_with_evaluator(&mut evaluator, r#"
        let calculator = {
            factFunc: factorial,
            compute: n => self.factFunc(n)
        }
    "#);
    assert!(result.is_ok());

    // Test via record method
    let result = eval_with_evaluator(&mut evaluator, "calculator.compute(5)");
    assert!(result.is_ok());
    match result.unwrap() {
        Value::Number(n) => assert_eq!(n, 120.0),
        _ => panic!("Expected number"),
    }
}

#[test]
fn test_closure_with_self() {
    let mut evaluator = Evaluator::new();

    // Record that creates closures using self
    let result = eval_with_evaluator(&mut evaluator, r#"
        let obj = {
            base: 10,
            makeAdder: () => x => x + self.base
        }
    "#);
    assert!(result.is_ok());

    // Get the adder function
    let result = eval_with_evaluator(&mut evaluator, "let adder = obj.makeAdder()");
    assert!(result.is_ok());

    // Use the adder
    let result = eval_with_evaluator(&mut evaluator, "adder(5)");
    assert!(result.is_ok());
    match result.unwrap() {
        Value::Number(n) => assert_eq!(n, 15.0),
        _ => panic!("Expected number"),
    }
}
