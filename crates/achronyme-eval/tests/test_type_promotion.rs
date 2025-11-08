use achronyme_eval::Evaluator;
use achronyme_parser::parse;
use achronyme_types::complex::Complex;
use achronyme_types::value::Value;

fn eval(source: &str) -> Result<Value, String> {
    let statements = parse(source)?;
    let mut evaluator = Evaluator::new();

    let mut result = Value::Number(0.0);
    for stmt in &statements {
        result = evaluator.evaluate(stmt)?;
    }

    Ok(result)
}

fn eval_with_evaluator(evaluator: &mut Evaluator, source: &str) -> Result<Value, String> {
    let statements = parse(source)?;

    let mut result = Value::Number(0.0);
    for stmt in &statements {
        result = evaluator.evaluate(stmt)?;
    }

    Ok(result)
}

// ========================================================================
// Type Promotion Tests
// ========================================================================

#[test]
fn test_mixed_number_complex_literal() {
    // Complex primero, luego números
    let result = eval("[1+i, 2, 3]").unwrap();

    match result {
        Value::Vector(v) => {
            assert_eq!(v.len(), 3);
            assert_eq!(v[0], Value::Complex(Complex::new(1.0, 1.0)));
            assert_eq!(v[1], Value::Complex(Complex::new(2.0, 0.0))); // Promoted
            assert_eq!(v[2], Value::Complex(Complex::new(3.0, 0.0))); // Promoted
        }
        _ => panic!("Expected Vector, got {:?}", result),
    }
}

#[test]
fn test_mixed_number_complex_middle() {
    // Número, complex en medio, número
    let result = eval("[1, 2+i, 3]").unwrap();

    match result {
        Value::Vector(v) => {
            assert_eq!(v.len(), 3);
            assert_eq!(v[0], Value::Complex(Complex::new(1.0, 0.0))); // Promoted
            assert_eq!(v[1], Value::Complex(Complex::new(2.0, 1.0)));
            assert_eq!(v[2], Value::Complex(Complex::new(3.0, 0.0))); // Promoted
        }
        _ => panic!("Expected Vector, got {:?}", result),
    }
}

#[test]
fn test_mixed_number_complex_end() {
    // Números primero, complex al final
    let result = eval("[1, 2, 3+i]").unwrap();

    match result {
        Value::Vector(v) => {
            assert_eq!(v.len(), 3);
            assert_eq!(v[0], Value::Complex(Complex::new(1.0, 0.0))); // Promoted
            assert_eq!(v[1], Value::Complex(Complex::new(2.0, 0.0))); // Promoted
            assert_eq!(v[2], Value::Complex(Complex::new(3.0, 1.0)));
        }
        _ => panic!("Expected Vector, got {:?}", result),
    }
}

#[test]
fn test_mixed_with_variables() {
    // El caso que reportó el usuario
    let mut evaluator = Evaluator::new();

    eval_with_evaluator(&mut evaluator, "let a = 3").unwrap();
    eval_with_evaluator(&mut evaluator, "let b = 4 + i").unwrap();
    let result = eval_with_evaluator(&mut evaluator, "let v = [a, b]").unwrap();

    match result {
        Value::Vector(v) => {
            assert_eq!(v.len(), 2);
            assert_eq!(v[0], Value::Complex(Complex::new(3.0, 0.0))); // a promoted
            assert_eq!(v[1], Value::Complex(Complex::new(4.0, 1.0))); // b
        }
        _ => panic!("Expected Vector, got {:?}", result),
    }
}

#[test]
fn test_mixed_with_imaginary_unit() {
    // Usando constante i
    let result = eval("[1, i, 2+i]").unwrap();

    match result {
        Value::Vector(v) => {
            assert_eq!(v.len(), 3);
            assert_eq!(v[0], Value::Complex(Complex::new(1.0, 0.0))); // Promoted
            assert_eq!(v[1], Value::Complex(Complex::new(0.0, 1.0))); // i
            assert_eq!(v[2], Value::Complex(Complex::new(2.0, 1.0)));
        }
        _ => panic!("Expected Vector, got {:?}", result),
    }
}

#[test]
fn test_all_real_numbers() {
    // Sin complejos, debería seguir siendo Vector de Numbers
    let result = eval("[1, 2, 3, 4]").unwrap();

    match result {
        Value::Vector(v) => {
            assert_eq!(v, vec![Value::Number(1.0), Value::Number(2.0), Value::Number(3.0), Value::Number(4.0)]);
        }
        _ => panic!("Expected Vector, got {:?}", result),
    }
}

#[test]
fn test_all_complex_numbers() {
    // Todos complejos
    let result = eval("[1+i, 2+2i, 3+3i]").unwrap();

    match result {
        Value::Vector(v) => {
            assert_eq!(v.len(), 3);
            assert_eq!(v[0], Value::Complex(Complex::new(1.0, 1.0)));
            assert_eq!(v[1], Value::Complex(Complex::new(2.0, 2.0)));
            assert_eq!(v[2], Value::Complex(Complex::new(3.0, 3.0)));
        }
        _ => panic!("Expected Vector, got {:?}", result),
    }
}

#[test]
fn test_mixed_with_expressions() {
    // Expresiones que evalúan a números y complejos
    let result = eval("[2*3, 1+i, 4/2]").unwrap();

    match result {
        Value::Vector(v) => {
            assert_eq!(v.len(), 3);
            assert_eq!(v[0], Value::Complex(Complex::new(6.0, 0.0)));  // 2*3 promoted
            assert_eq!(v[1], Value::Complex(Complex::new(1.0, 1.0)));
            assert_eq!(v[2], Value::Complex(Complex::new(2.0, 0.0)));  // 4/2 promoted
        }
        _ => panic!("Expected Vector, got {:?}", result),
    }
}

#[test]
fn test_mixed_complex_operations() {
    // Operaciones complejas mezcladas con números
    let result = eval("[i^2, 2, (1+i)^2]").unwrap();

    match result {
        Value::Vector(v) => {
            assert_eq!(v.len(), 3);
            // i^2 = -1
            assert_eq!(v[0], Value::Complex(Complex::new(-1.0, 0.0)));
            // 2 promoted to complex
            assert_eq!(v[1], Value::Complex(Complex::new(2.0, 0.0)));
            // (1+i)^2 = 0 + 2i
            let c = v[2].as_complex().unwrap();
            assert!(c.re.abs() < 1e-10);
            assert!((c.im - 2.0).abs() < 1e-10);
        }
        _ => panic!("Expected Vector, got {:?}", result),
    }
}

#[test]
fn test_single_element_real() {
    // Un solo elemento real
    let result = eval("[42]").unwrap();

    match result {
        Value::Vector(v) => {
            assert_eq!(v, vec![Value::Number(42.0)]);
        }
        _ => panic!("Expected Vector, got {:?}", result),
    }
}

#[test]
fn test_single_element_complex() {
    // Un solo elemento complejo
    let result = eval("[3+4i]").unwrap();

    match result {
        Value::Vector(v) => {
            assert_eq!(v.len(), 1);
            assert_eq!(v[0], Value::Complex(Complex::new(3.0, 4.0)));
        }
        _ => panic!("Expected Vector, got {:?}", result),
    }
}

#[test]
fn test_mixed_in_pipeline() {
    // Vector mixto en pipeline con shadowing
    let mut evaluator = Evaluator::new();

    eval_with_evaluator(&mut evaluator, "let x = 2").unwrap();
    eval_with_evaluator(&mut evaluator, "let y = 3+i").unwrap();
    eval_with_evaluator(&mut evaluator, "let v = [x, y]").unwrap();

    // Usar en map
    let result = eval_with_evaluator(&mut evaluator, "map(z => z^2, v)").unwrap();

    match result {
        Value::Vector(v) => {
            assert_eq!(v.len(), 2);
            // 2^2 = 4+0i
            assert_eq!(v[0], Value::Complex(Complex::new(4.0, 0.0)));
            // (3+i)^2 = 8+6i (using epsilon for floating point comparison)
            let c = v[1].as_complex().unwrap();
            assert!((c.re - 8.0).abs() < 1e-10);
            assert!((c.im - 6.0).abs() < 1e-10);
        }
        _ => panic!("Expected Vector, got {:?}", result),
    }
}

#[test]
fn test_type_promotion_preserves_order() {
    // Orden de elementos debe preservarse
    let result = eval("[5, 1+i, 10, 2+2i, 15]").unwrap();

    match result {
        Value::Vector(v) => {
            assert_eq!(v.len(), 5);
            assert_eq!(v[0], Value::Complex(Complex::new(5.0, 0.0)));
            assert_eq!(v[1], Value::Complex(Complex::new(1.0, 1.0)));
            assert_eq!(v[2], Value::Complex(Complex::new(10.0, 0.0)));
            assert_eq!(v[3], Value::Complex(Complex::new(2.0, 2.0)));
            assert_eq!(v[4], Value::Complex(Complex::new(15.0, 0.0)));
        }
        _ => panic!("Expected Vector, got {:?}", result),
    }
}
