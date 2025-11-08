use achronyme_eval::Evaluator;
use achronyme_types::complex::Complex;
use achronyme_types::value::Value;

#[test]
fn test_imaginary_unit() {
    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_str("i").unwrap();

    match result {
        Value::Complex(c) => {
            assert_eq!(c.re, 0.0);
            assert_eq!(c.im, 1.0);
        }
        _ => panic!("Expected complex number"),
    }
}

#[test]
fn test_complex_power_complex() {
    let mut evaluator = Evaluator::new();

    // Test i^i
    let result = evaluator.eval_str("i^i").unwrap();
    match result {
        Value::Complex(c) => {
            // i^i = e^(i * ln(i)) = e^(i * (0 + i*π/2)) = e^(-π/2) ≈ 0.2079
            assert!((c.re - 0.2079).abs() < 0.001);
            assert!(c.im.abs() < 0.001);
        }
        _ => panic!("Expected complex number"),
    }
}

#[test]
fn test_complex_vector_literal() {
    let mut evaluator = Evaluator::new();

    // Test vector with complex elements
    let result = evaluator.eval_str("[i, 2+3i, 4]").unwrap();

    match result {
        Value::Vector(v) => {
            assert_eq!(v.len(), 3);
            assert_eq!(v[0], Value::Complex(Complex::new(0.0, 1.0)));
            assert_eq!(v[1], Value::Complex(Complex::new(2.0, 3.0)));
            assert_eq!(v[2], Value::Number(4.0));
        }
        _ => panic!("Expected vector, got {:?}", result),
    }
}

#[test]
fn test_complex_vector_addition() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("[1+2i, 3+4i] + [5+6i, 7+8i]").unwrap();

    match result {
        Value::Vector(v) => {
            assert_eq!(v.len(), 2);
            assert_eq!(v[0], Value::Complex(Complex::new(6.0, 8.0)));
            assert_eq!(v[1], Value::Complex(Complex::new(10.0, 12.0)));
        }
        _ => panic!("Expected vector"),
    }
}

#[test]
fn test_complex_vector_multiplication() {
    let mut evaluator = Evaluator::new();

    // Element-wise multiplication
    let result = evaluator.eval_str("[1+i, 2+2i] * [1-i, 1+i]").unwrap();

    match result {
        Value::Vector(v) => {
            assert_eq!(v.len(), 2);
            // (1+i)(1-i) = 1 - i + i - i^2 = 1 - (-1) = 2
            assert_eq!(v[0], Value::Complex(Complex::new(2.0, 0.0)));
            // (2+2i)(1+i) = 2 + 2i + 2i + 2i^2 = 2 + 4i - 2 = 0 + 4i
            assert_eq!(v[1], Value::Complex(Complex::new(0.0, 4.0)));
        }
        _ => panic!("Expected vector"),
    }
}

#[test]
fn test_real_to_complex_promotion() {
    let mut evaluator = Evaluator::new();

    // Test promotion of real number to complex
    let result = evaluator.eval_str("2 + 3i").unwrap();

    match result {
        Value::Complex(c) => {
            assert_eq!(c.re, 2.0);
            assert_eq!(c.im, 3.0);
        }
        _ => panic!("Expected complex number"),
    }
}

#[test]
fn test_complex_power_real() {
    let mut evaluator = Evaluator::new();

    // Test (1+i)^2
    let result = evaluator.eval_str("(1+i)^2").unwrap();

    match result {
        Value::Complex(c) => {
            // (1+i)^2 = 1 + 2i + i^2 = 1 + 2i - 1 = 2i
            assert!((c.re - 0.0).abs() < 0.0001);
            assert!((c.im - 2.0).abs() < 0.0001);
        }
        _ => panic!("Expected complex number"),
    }
}

#[test]
fn test_number_power_complex() {
    let mut evaluator = Evaluator::new();

    // Test 2^i
    let result = evaluator.eval_str("2^i").unwrap();

    match result {
        Value::Complex(c) => {
            // 2^i = e^(i * ln(2)) = cos(ln(2)) + i*sin(ln(2))
            let ln2 = 2.0_f64.ln();
            assert!((c.re - ln2.cos()).abs() < 0.0001);
            assert!((c.im - ln2.sin()).abs() < 0.0001);
        }
        _ => panic!("Expected complex number"),
    }
}
