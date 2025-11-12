use achronyme_eval::Evaluator;
use achronyme_types::value::Value;

fn eval(source: &str) -> Result<Value, String> {
    let mut evaluator = Evaluator::new();
    evaluator.eval_str(source)
}

#[test]
fn test_linspace_basic() {
    let result = eval("linspace(0, 10, 11)").unwrap();
    if let Value::Tensor(t) = result {
        assert_eq!(t.data().len(), 11);
        assert_eq!(t.data()[0], 0.0);
        assert_eq!(t.data()[10], 10.0);
    } else {
        panic!("Expected Tensor, got {:?}", result);
    }
}

#[test]
fn test_linspace_fractional() {
    let result = eval("linspace(0, 1, 5)").unwrap();
    if let Value::Tensor(t) = result {
        assert_eq!(t.data().len(), 5);
        assert_eq!(t.data()[0], 0.0);
        assert_eq!(t.data()[4], 1.0);
        assert!((t.data()[1] - 0.25).abs() < 1e-10);
        assert!((t.data()[2] - 0.5).abs() < 1e-10);
        assert!((t.data()[3] - 0.75).abs() < 1e-10);
    } else {
        panic!("Expected Tensor, got {:?}", result);
    }
}

#[test]
fn test_linspace_as_variable() {
    let result = eval(r#"
        let v = linspace(0, 5, 6);
        v
    "#).unwrap();
    if let Value::Tensor(t) = result {
        assert_eq!(t.data().len(), 6);
        assert_eq!(t.data()[0], 0.0);
        assert_eq!(t.data()[5], 5.0);
    } else {
        panic!("Expected Tensor, got {:?}", result);
    }
}

#[test]
fn test_hanning_window() {
    let result = eval("hanning(8)").unwrap();
    if let Value::Tensor(t) = result {
        assert_eq!(t.data().len(), 8);
    } else {
        panic!("Expected Tensor, got {:?}", result);
    }
}

#[test]
fn test_hamming_window() {
    let result = eval("hamming(8)").unwrap();
    if let Value::Tensor(t) = result {
        assert_eq!(t.data().len(), 8);
    } else {
        panic!("Expected Tensor, got {:?}", result);
    }
}

#[test]
fn test_blackman_window() {
    let result = eval("blackman(8)").unwrap();
    if let Value::Tensor(t) = result {
        assert_eq!(t.data().len(), 8);
    } else {
        panic!("Expected Tensor, got {:?}", result);
    }
}

#[test]
fn test_rectangular_window() {
    let result = eval("rectangular(8)").unwrap();
    if let Value::Tensor(t) = result {
        assert_eq!(t.data().len(), 8);
        for &val in t.data() {
            assert_eq!(val, 1.0);
        }
    } else {
        panic!("Expected Tensor, got {:?}", result);
    }
}

#[test]
fn test_fft_basic() {
    let result = eval("fft([1, 2, 3, 4])").unwrap();
    if let Value::ComplexTensor(t) = result {
        assert_eq!(t.data().len(), 4);
    } else {
        panic!("Expected ComplexTensor, got {:?}", result);
    }
}

#[test]
fn test_fft_mag() {
    let result = eval("fft_mag([1, 2, 3, 4])").unwrap();
    if let Value::Tensor(t) = result {
        assert_eq!(t.data().len(), 4);
    } else {
        panic!("Expected Tensor, got {:?}", result);
    }
}

#[test]
fn test_fft_phase() {
    let result = eval("fft_phase([1, 2, 3, 4])").unwrap();
    if let Value::Tensor(t) = result {
        assert_eq!(t.data().len(), 4);
    } else {
        panic!("Expected Tensor, got {:?}", result);
    }
}

#[test]
fn test_conv_basic() {
    let result = eval("conv([1, 2, 3], [1, 1])").unwrap();
    if let Value::Tensor(t) = result {
        assert!(t.data().len() > 0);
    } else {
        panic!("Expected Tensor, got {:?}", result);
    }
}

#[test]
fn test_conv_fft() {
    let result = eval("conv_fft([1, 2, 3], [1, 1])").unwrap();
    if let Value::Tensor(t) = result {
        assert!(t.data().len() > 0);
    } else {
        panic!("Expected Tensor, got {:?}", result);
    }
}

#[test]
fn test_linspace_error_less_than_2() {
    let result = eval("linspace(0, 10, 1)");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("n >= 2"));
}

#[test]
fn test_window_functions_accessible() {
    let result = eval(r#"
        let h = hanning;
        let ham = hamming;
        let b = blackman;
        let r = rectangular;
        h(4)
    "#).unwrap();
    if let Value::Tensor(t) = result {
        assert_eq!(t.data().len(), 4);
    } else {
        panic!("Expected Tensor, got {:?}", result);
    }
}

#[test]
fn test_linspace_as_first_class_value() {
    let result = eval(r#"
        let f = linspace;
        f(0, 10, 11)
    "#).unwrap();
    if let Value::Tensor(t) = result {
        assert_eq!(t.data().len(), 11);
        assert_eq!(t.data()[0], 0.0);
        assert_eq!(t.data()[10], 10.0);
    } else {
        panic!("Expected Tensor, got {:?}", result);
    }
}
