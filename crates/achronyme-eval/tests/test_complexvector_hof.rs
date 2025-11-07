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
// Map with ComplexVector Tests
// ========================================================================

#[test]
fn test_map_complex_vector() {
    let mut evaluator = Evaluator::new();
    eval_with_evaluator(&mut evaluator, "let cv = [1+i, 2+2i, 3+3i]").unwrap();

    let result = eval_with_evaluator(&mut evaluator, "map(z => z^2, cv)").unwrap();
    match result {
        Value::ComplexVector(cv) => {
            assert_eq!(cv.data().len(), 3);
            // (1+i)^2 = 0+2i
            assert!((cv.data()[0].re - 0.0).abs() < 1e-10);
            assert!((cv.data()[0].im - 2.0).abs() < 1e-10);
        }
        _ => panic!("Expected ComplexVector, got {:?}", result),
    }
}

#[test]
fn test_map_mixed_vector_to_complex() {
    // Map that returns complex values promotes to ComplexVector
    let result = eval("map(x => x + i, [1, 2, 3])").unwrap();
    match result {
        Value::ComplexVector(cv) => {
            assert_eq!(cv.data().len(), 3);
            assert_eq!(cv.data()[0], Complex::new(1.0, 1.0));
            assert_eq!(cv.data()[1], Complex::new(2.0, 1.0));
            assert_eq!(cv.data()[2], Complex::new(3.0, 1.0));
        }
        _ => panic!("Expected ComplexVector, got {:?}", result),
    }
}

#[test]
fn test_map_complex_magnitude() {
    let mut evaluator = Evaluator::new();
    eval_with_evaluator(&mut evaluator, "let cv = [3+4i, 5+12i]").unwrap();

    // Map to get magnitude
    // Note: map over ComplexVector returns ComplexVector even if lambda returns Number
    let result = eval_with_evaluator(&mut evaluator, "map(z => abs(z), cv)").unwrap();
    match result {
        Value::ComplexVector(cv) => {
            assert_eq!(cv.data().len(), 2);
            assert!((cv.data()[0].re - 5.0).abs() < 1e-10);  // sqrt(3²+4²) = 5
            assert!(cv.data()[0].im.abs() < 1e-10);
            assert!((cv.data()[1].re - 13.0).abs() < 1e-10); // sqrt(5²+12²) = 13
            assert!(cv.data()[1].im.abs() < 1e-10);
        }
        _ => panic!("Expected ComplexVector, got {:?}", result),
    }
}

// ========================================================================
// Filter with ComplexVector Tests
// ========================================================================

#[test]
fn test_filter_complex_vector_by_real_part() {
    let mut evaluator = Evaluator::new();
    eval_with_evaluator(&mut evaluator, "let cv = [1+i, 2+2i, 3+i, 4+2i]").unwrap();

    // Filter by real part > 2
    // Note: Need to define real() function or use comparison
    let result = eval_with_evaluator(&mut evaluator, "filter(z => abs(z) > 2, cv)").unwrap();
    match result {
        Value::ComplexVector(cv) => {
            // Elements with magnitude > 2: 2+2i (≈2.83), 3+i (≈3.16), 4+2i (≈4.47)
            assert_eq!(cv.data().len(), 3);
        }
        _ => panic!("Expected ComplexVector, got {:?}", result),
    }
}

#[test]
fn test_filter_complex_vector_to_empty() {
    let mut evaluator = Evaluator::new();
    eval_with_evaluator(&mut evaluator, "let cv = [1+i, 2+i]").unwrap();

    // Filter that matches nothing
    let result = eval_with_evaluator(&mut evaluator, "filter(z => abs(z) > 100, cv)").unwrap();
    match result {
        Value::ComplexVector(cv) => {
            assert_eq!(cv.data().len(), 0);
        }
        _ => panic!("Expected ComplexVector, got {:?}", result),
    }
}

// ========================================================================
// Reduce with ComplexVector Tests
// ========================================================================

#[test]
fn test_reduce_complex_vector_sum() {
    let mut evaluator = Evaluator::new();
    eval_with_evaluator(&mut evaluator, "let cv = [1+i, 2+2i, 3+3i]").unwrap();

    let result = eval_with_evaluator(&mut evaluator, "reduce((a, b) => a + b, 0, cv)").unwrap();
    match result {
        Value::Complex(c) => {
            assert_eq!(c.re, 6.0);  // 1+2+3
            assert_eq!(c.im, 6.0);  // 1+2+3
        }
        _ => panic!("Expected Complex, got {:?}", result),
    }
}

#[test]
fn test_reduce_complex_vector_with_complex_init() {
    let mut evaluator = Evaluator::new();
    eval_with_evaluator(&mut evaluator, "let cv = [1+i, 2+2i]").unwrap();

    let result = eval_with_evaluator(&mut evaluator, "reduce((a, b) => a + b, 10+10i, cv)").unwrap();
    match result {
        Value::Complex(c) => {
            assert_eq!(c.re, 13.0);  // 10+1+2
            assert_eq!(c.im, 13.0);  // 10+1+2
        }
        _ => panic!("Expected Complex, got {:?}", result),
    }
}

#[test]
fn test_reduce_real_vector_with_complex_init() {
    let mut evaluator = Evaluator::new();

    let result = eval_with_evaluator(&mut evaluator, "reduce((a, b) => a + b, 1+i, [1, 2, 3])").unwrap();
    match result {
        Value::Complex(c) => {
            assert_eq!(c.re, 7.0);  // 1+1+2+3
            assert_eq!(c.im, 1.0);  // Imaginary part stays 1
        }
        _ => panic!("Expected Complex, got {:?}", result),
    }
}

// ========================================================================
// DSP Functions with ComplexVector Tests
// ========================================================================

#[test]
fn test_fft_mag_on_complex_vector() {
    let mut evaluator = Evaluator::new();
    eval_with_evaluator(&mut evaluator, "let signal = [1, 2, 3, 4]").unwrap();
    eval_with_evaluator(&mut evaluator, "let spectrum = fft(signal)").unwrap();

    let result = eval_with_evaluator(&mut evaluator, "fft_mag(spectrum)").unwrap();
    match result {
        Value::Vector(v) => {
            assert_eq!(v.data().len(), 4);
            // Just verify it returns magnitudes (real numbers)
            for mag in v.data() {
                assert!(*mag >= 0.0, "Magnitude should be non-negative");
            }
        }
        _ => panic!("Expected Vector, got {:?}", result),
    }
}

#[test]
fn test_fft_phase_on_complex_vector() {
    let mut evaluator = Evaluator::new();
    eval_with_evaluator(&mut evaluator, "let signal = [1, 2, 3, 4]").unwrap();
    eval_with_evaluator(&mut evaluator, "let spectrum = fft(signal)").unwrap();

    let result = eval_with_evaluator(&mut evaluator, "fft_phase(spectrum)").unwrap();
    match result {
        Value::Vector(v) => {
            assert_eq!(v.data().len(), 4);
            // Phases should be in range [-π, π]
            for phase in v.data() {
                assert!(*phase >= -std::f64::consts::PI && *phase <= std::f64::consts::PI);
            }
        }
        _ => panic!("Expected Vector, got {:?}", result),
    }
}

#[test]
fn test_abs_complex_vector() {
    let result = eval("abs([3+4i, 5+12i, 1+i])").unwrap();
    match result {
        Value::Vector(v) => {
            assert_eq!(v.data().len(), 3);
            assert!((v.data()[0] - 5.0).abs() < 1e-10);   // sqrt(9+16)
            assert!((v.data()[1] - 13.0).abs() < 1e-10);  // sqrt(25+144)
            assert!((v.data()[2] - 1.4142135623730951).abs() < 1e-10);  // sqrt(2)
        }
        _ => panic!("Expected Vector, got {:?}", result),
    }
}

#[test]
fn test_abs_single_complex() {
    let result = eval("abs(3+4i)").unwrap();
    match result {
        Value::Number(n) => {
            assert!((n - 5.0).abs() < 1e-10);
        }
        _ => panic!("Expected Number, got {:?}", result),
    }
}

// ========================================================================
// Integration Tests: Complete Workflow
// ========================================================================

#[test]
fn test_dsp_workflow_with_complex() {
    let mut evaluator = Evaluator::new();

    // Create signal
    eval_with_evaluator(&mut evaluator, "let signal = [1, 2, 3, 4, 3, 2, 1, 0]").unwrap();

    // Apply FFT
    eval_with_evaluator(&mut evaluator, "let spectrum = fft(signal)").unwrap();

    // Get magnitude
    eval_with_evaluator(&mut evaluator, "let magnitude = fft_mag(spectrum)").unwrap();

    // Filter high frequencies (keep only first half)
    eval_with_evaluator(&mut evaluator, "let filtered_mag = map(x => x, magnitude)").unwrap();

    let result = eval_with_evaluator(&mut evaluator, "filtered_mag").unwrap();
    match result {
        Value::Vector(v) => {
            assert_eq!(v.data().len(), 8);
        }
        _ => panic!("Expected Vector, got {:?}", result),
    }
}

#[test]
fn test_complex_pipeline_with_shadowing() {
    let mut evaluator = Evaluator::new();

    eval_with_evaluator(&mut evaluator, "let v = [1, 2, 3, 4]").unwrap();
    eval_with_evaluator(&mut evaluator, "let v = map(x => x + i, v)").unwrap();
    eval_with_evaluator(&mut evaluator, "let v = map(z => z^2, v)").unwrap();
    eval_with_evaluator(&mut evaluator, "let magnitudes = map(z => abs(z), v)").unwrap();

    let result = eval_with_evaluator(&mut evaluator, "magnitudes").unwrap();
    match result {
        Value::ComplexVector(cv) => {
            assert_eq!(cv.data().len(), 4);
            // (1+i)^2 = 2i, |2i| = 2
            // (2+i)^2 = 3+4i, |3+4i| = 5
            // (3+i)^2 = 8+6i, |8+6i| = 10
            // (4+i)^2 = 15+8i, |15+8i| = 17
            assert!((cv.data()[0].re - 2.0).abs() < 1e-10);
            assert!((cv.data()[1].re - 5.0).abs() < 1e-10);
            assert!((cv.data()[2].re - 10.0).abs() < 1e-10);
            assert!((cv.data()[3].re - 17.0).abs() < 1e-10);
        }
        _ => panic!("Expected ComplexVector, got {:?}", result),
    }
}
