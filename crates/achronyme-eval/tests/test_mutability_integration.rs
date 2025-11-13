/// Integration tests for mutability system
/// Tests complete functionality including mutable variables, records, and self-reference
use achronyme_eval::Evaluator;
use achronyme_parser::parse;
use achronyme_types::value::Value;

/// Helper function to evaluate code with an evaluator
fn eval_with_evaluator(evaluator: &mut Evaluator, source: &str) -> Result<Value, String> {
    let statements = parse(source)?;
    let mut result = Value::Number(0.0);
    for stmt in &statements {
        result = evaluator.evaluate(stmt)?;
    }
    Ok(result)
}

#[test]
fn test_mutable_variable_basic() {
    let mut eval = Evaluator::new();

    // Declare mutable variable
    let result = eval_with_evaluator(&mut eval, "mut x = 10").unwrap();
    assert_eq!(result, Value::Number(10.0));

    // Reassign
    let result = eval_with_evaluator(&mut eval, "x = 20").unwrap();
    assert_eq!(result, Value::Number(20.0));

    // Verify new value
    let result = eval_with_evaluator(&mut eval, "x").unwrap();
    assert_eq!(result, Value::Number(20.0));
}

#[test]
fn test_immutable_variable_cannot_reassign() {
    let mut eval = Evaluator::new();

    // Declare immutable variable
    eval_with_evaluator(&mut eval, "let x = 10").unwrap();

    // Try to reassign - should fail
    let result = eval_with_evaluator(&mut eval, "x = 20");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Cannot assign to immutable"));
}

#[test]
fn test_mutable_record_field() {
    let mut eval = Evaluator::new();

    // Create record with mutable field
    let code = r#"
        let config = {
            mut valor: 10,
            inmutable: 20
        }
    "#;
    eval_with_evaluator(&mut eval, code).unwrap();

    // Read initial value
    let result = eval_with_evaluator(&mut eval, "config.valor").unwrap();
    assert_eq!(result, Value::Number(10.0));

    // Mutate field
    eval_with_evaluator(&mut eval, "config.valor = 30").unwrap();

    // Verify mutation
    let result = eval_with_evaluator(&mut eval, "config.valor").unwrap();
    assert_eq!(result, Value::Number(30.0));

    // Immutable field should remain unchanged
    let result = eval_with_evaluator(&mut eval, "config.inmutable").unwrap();
    assert_eq!(result, Value::Number(20.0));
}

#[test]
fn test_immutable_record_field_cannot_reassign() {
    let mut eval = Evaluator::new();

    let code = r#"
        let config = {
            valor: 10
        }
    "#;
    eval_with_evaluator(&mut eval, code).unwrap();

    // Try to reassign immutable field - should fail
    let result = eval_with_evaluator(&mut eval, "config.valor = 20");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Cannot assign to immutable"));
}

#[test]
fn test_record_method_with_mutable_self() {
    let mut eval = Evaluator::new();

    // Create record with mutable field and method
    let code = r#"
        let contador = {
            mut valor: 0,
            incrementar: () => do {
                self.valor = self.valor + 1
            },
            obtener: () => self.valor
        }
    "#;
    eval_with_evaluator(&mut eval, code).unwrap();

    // Initial value
    let result = eval_with_evaluator(&mut eval, "contador.obtener()").unwrap();
    assert_eq!(result, Value::Number(0.0));

    // Increment
    eval_with_evaluator(&mut eval, "contador.incrementar()").unwrap();

    // Verify increment
    let result = eval_with_evaluator(&mut eval, "contador.obtener()").unwrap();
    assert_eq!(result, Value::Number(1.0));

    // Increment again
    eval_with_evaluator(&mut eval, "contador.incrementar()").unwrap();

    // Verify
    let result = eval_with_evaluator(&mut eval, "contador.obtener()").unwrap();
    assert_eq!(result, Value::Number(2.0));

    // Verify direct field access also shows updated value
    let result = eval_with_evaluator(&mut eval, "contador.valor").unwrap();
    assert_eq!(result, Value::Number(2.0));
}

#[test]
fn test_record_method_with_parameters() {
    let mut eval = Evaluator::new();

    let code = r#"
        let acumulador = {
            mut total: 0,
            agregar: (x) => do {
                self.total = self.total + x
            },
            reset: () => do {
                self.total = 0
            }
        }
    "#;
    eval_with_evaluator(&mut eval, code).unwrap();

    // Add values
    eval_with_evaluator(&mut eval, "acumulador.agregar(5)").unwrap();
    eval_with_evaluator(&mut eval, "acumulador.agregar(10)").unwrap();
    eval_with_evaluator(&mut eval, "acumulador.agregar(3)").unwrap();

    // Check total
    let result = eval_with_evaluator(&mut eval, "acumulador.total").unwrap();
    assert_eq!(result, Value::Number(18.0));

    // Reset
    eval_with_evaluator(&mut eval, "acumulador.reset()").unwrap();

    // Verify reset
    let result = eval_with_evaluator(&mut eval, "acumulador.total").unwrap();
    assert_eq!(result, Value::Number(0.0));
}

#[test]
fn test_nested_mutable_fields() {
    let mut eval = Evaluator::new();

    let code = r#"
        let app = {
            mut config: {
                mut debug: true,
                version: "1.0"
            }
        }
    "#;
    eval_with_evaluator(&mut eval, code).unwrap();

    // NOTE: Nested field assignment like app.config.debug = false
    // is not supported yet because it would require special handling
    // For now, we need to reassign the entire config object

    // Read nested value
    let result = eval_with_evaluator(&mut eval, "app.config.debug").unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_mutable_variable_in_lambda() {
    let mut eval = Evaluator::new();

    // All in one code block to maintain scope
    let code = r#"
        mut counter = 0;
        let increment = () => do {
            counter = counter + 1
        };
        increment();
        increment();
        increment();
        counter
    "#;
    let result = eval_with_evaluator(&mut eval, code).unwrap();
    assert_eq!(result, Value::Number(3.0));
}

#[test]
fn test_assignment_in_do_block() {
    let mut eval = Evaluator::new();

    // Define x in outer scope, mutate in do block
    let code = r#"
        mut x = 10;
        let result = do {
            x = 20;
            x = x + 5;
            x
        };
        result
    "#;
    let result = eval_with_evaluator(&mut eval, code).unwrap();
    assert_eq!(result, Value::Number(25.0));
}

#[test]
fn test_multiple_mutable_variables() {
    let mut eval = Evaluator::new();

    eval_with_evaluator(&mut eval, "mut a = 1").unwrap();
    eval_with_evaluator(&mut eval, "mut b = 2").unwrap();
    eval_with_evaluator(&mut eval, "mut c = 3").unwrap();

    eval_with_evaluator(&mut eval, "a = a + 10").unwrap();
    eval_with_evaluator(&mut eval, "b = b * 5").unwrap();
    eval_with_evaluator(&mut eval, "c = c - 1").unwrap();

    assert_eq!(eval_with_evaluator(&mut eval, "a").unwrap(), Value::Number(11.0));
    assert_eq!(eval_with_evaluator(&mut eval, "b").unwrap(), Value::Number(10.0));
    assert_eq!(eval_with_evaluator(&mut eval, "c").unwrap(), Value::Number(2.0));
}

#[test]
fn test_mutable_variable_shadowing() {
    let mut eval = Evaluator::new();

    let code = r#"
        mut x = 10;
        let inner_x = do {
            let x = 20;  // Shadows outer x with immutable
            x
        };
        let outer_x = x;
        [inner_x, outer_x]
    "#;
    let result = eval_with_evaluator(&mut eval, code).unwrap();

    // Should return [20, 10] - inner shadowed value and outer unchanged value
    match result {
        Value::Tensor(tensor) => {
            assert_eq!(tensor.shape(), &[2]);
            assert_eq!(tensor.data()[0], 20.0); // inner
            assert_eq!(tensor.data()[1], 10.0); // outer unchanged
        }
        Value::Vector(vec) => {
            assert_eq!(vec.len(), 2);
            assert_eq!(vec[0], Value::Number(20.0)); // inner
            assert_eq!(vec[1], Value::Number(10.0)); // outer unchanged
        }
        _ => panic!("Expected tensor or vector result, got {:?}", result),
    }
}
