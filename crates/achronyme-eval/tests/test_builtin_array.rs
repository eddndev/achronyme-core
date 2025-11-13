/// Integration tests for built-in array utility functions
///
/// Tests the Tier 1 array functions: product, range, len, reverse

use achronyme_eval::Evaluator;
use achronyme_types::value::Value;

// ============================================================================
// product() tests
// ============================================================================

#[test]
fn test_product_basic() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("product([2, 3, 4])").unwrap();
    assert_eq!(result, Value::Number(24.0));
}

#[test]
fn test_product_single_element() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("product([5])").unwrap();
    assert_eq!(result, Value::Number(5.0));
}

#[test]
fn test_product_empty_array() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("product([])").unwrap();
    assert_eq!(result, Value::Number(1.0)); // Empty product is 1
}

#[test]
fn test_product_with_zero() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("product([1, 2, 0, 4])").unwrap();
    assert_eq!(result, Value::Number(0.0));
}

#[test]
fn test_product_negative_numbers() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("product([-2, 3, -4])").unwrap();
    assert_eq!(result, Value::Number(24.0)); // -2 * 3 * -4 = 24
}

#[test]
fn test_product_with_variable() {
    let mut evaluator = Evaluator::new();

    evaluator.eval_str("let arr = [1, 2, 3, 4, 5]").unwrap();
    let result = evaluator.eval_str("product(arr)").unwrap();
    assert_eq!(result, Value::Number(120.0)); // 5!
}

// ============================================================================
// range() tests
// ============================================================================

#[test]
fn test_range_basic() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("range(0, 5)").unwrap();

    match result {
        Value::Vector(vec) => {
            assert_eq!(vec.len(), 5);
            assert_eq!(vec[0], Value::Number(0.0));
            assert_eq!(vec[1], Value::Number(1.0));
            assert_eq!(vec[2], Value::Number(2.0));
            assert_eq!(vec[3], Value::Number(3.0));
            assert_eq!(vec[4], Value::Number(4.0));
        }
        _ => panic!("Expected Vector, got {:?}", result),
    }
}

#[test]
fn test_range_with_step() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("range(0, 10, 2)").unwrap();

    match result {
        Value::Vector(vec) => {
            assert_eq!(vec.len(), 5);
            assert_eq!(vec, vec![
                Value::Number(0.0),
                Value::Number(2.0),
                Value::Number(4.0),
                Value::Number(6.0),
                Value::Number(8.0),
            ]);
        }
        _ => panic!("Expected Vector"),
    }
}

#[test]
fn test_range_negative_step() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("range(5, 0, -1)").unwrap();

    match result {
        Value::Vector(vec) => {
            assert_eq!(vec.len(), 5);
            assert_eq!(vec[0], Value::Number(5.0));
            assert_eq!(vec[4], Value::Number(1.0));
        }
        _ => panic!("Expected Vector"),
    }
}

#[test]
fn test_range_empty() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("range(0, 0)").unwrap();

    match result {
        Value::Vector(vec) => {
            assert_eq!(vec.len(), 0);
        }
        _ => panic!("Expected Vector"),
    }
}

#[test]
fn test_range_auto_direction() {
    let mut evaluator = Evaluator::new();

    // Auto-detects positive step
    let result = evaluator.eval_str("range(1, 4)").unwrap();
    match result {
        Value::Vector(vec) => {
            assert_eq!(vec, vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0),
            ]);
        }
        _ => panic!("Expected Vector"),
    }

    // Auto-detects negative step
    let result = evaluator.eval_str("range(4, 1)").unwrap();
    match result {
        Value::Vector(vec) => {
            assert_eq!(vec, vec![
                Value::Number(4.0),
                Value::Number(3.0),
                Value::Number(2.0),
            ]);
        }
        _ => panic!("Expected Vector"),
    }
}

#[test]
fn test_range_error_zero_step() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("range(0, 10, 0)");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("cannot be zero"));
}

#[test]
fn test_range_error_wrong_direction() {
    let mut evaluator = Evaluator::new();

    // Positive step with decreasing range
    let result = evaluator.eval_str("range(10, 0, 1)");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("direction"));
}

// ============================================================================
// len() tests
// ============================================================================

#[test]
fn test_len_basic() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("len([1, 2, 3])").unwrap();
    assert_eq!(result, Value::Number(3.0));
}

#[test]
fn test_len_empty() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("len([])").unwrap();
    assert_eq!(result, Value::Number(0.0));
}

#[test]
fn test_len_nested_array() {
    let mut evaluator = Evaluator::new();

    // Note: In Achronyme, [[1, 2], [3, 4], [5, 6]] creates a 2D tensor
    // which has 6 total elements (3x2 matrix)
    // This is expected behavior for numeric arrays
    let result = evaluator.eval_str("len([[1, 2], [3, 4], [5, 6]])").unwrap();
    // This becomes a Tensor with shape [3, 2], so total length is 6
    assert_eq!(result, Value::Number(6.0)); // Total tensor elements
}

#[test]
fn test_len_string() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("len(\"hello\")").unwrap();
    assert_eq!(result, Value::Number(5.0));
}

#[test]
fn test_len_with_range() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("len(range(0, 100))").unwrap();
    assert_eq!(result, Value::Number(100.0));
}

#[test]
fn test_len_record() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("len({a: 1, b: 2, c: 3})").unwrap();
    assert_eq!(result, Value::Number(3.0)); // Number of fields
}

// ============================================================================
// reverse() tests
// ============================================================================

#[test]
fn test_reverse_basic() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("reverse([1, 2, 3])").unwrap();

    match result {
        Value::Vector(vec) => {
            assert_eq!(vec, vec![
                Value::Number(3.0),
                Value::Number(2.0),
                Value::Number(1.0),
            ]);
        }
        Value::Tensor(t) => {
            // Arrays get converted to tensors
            assert_eq!(t.data(), &[3.0, 2.0, 1.0]);
        }
        _ => panic!("Expected Vector or Tensor, got {:?}", result),
    }
}

#[test]
fn test_reverse_empty() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("reverse([])").unwrap();

    match result {
        Value::Vector(vec) => {
            assert_eq!(vec.len(), 0);
        }
        _ => panic!("Expected Vector"),
    }
}

#[test]
fn test_reverse_single_element() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("reverse([42])").unwrap();

    match result {
        Value::Vector(vec) => {
            assert_eq!(vec, vec![Value::Number(42.0)]);
        }
        Value::Tensor(t) => {
            assert_eq!(t.data(), &[42.0]);
        }
        _ => panic!("Expected Vector or Tensor"),
    }
}

#[test]
fn test_reverse_string() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("reverse(\"hello\")").unwrap();
    assert_eq!(result, Value::String("olleh".to_string()));
}

#[test]
fn test_reverse_range() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("reverse(range(1, 6))").unwrap();

    match result {
        Value::Vector(vec) => {
            assert_eq!(vec, vec![
                Value::Number(5.0),
                Value::Number(4.0),
                Value::Number(3.0),
                Value::Number(2.0),
                Value::Number(1.0),
            ]);
        }
        _ => panic!("Expected Vector"),
    }
}

// ============================================================================
// Combined tests
// ============================================================================

#[test]
fn test_combined_operations() {
    let mut evaluator = Evaluator::new();

    // Create range, reverse it, and get product
    let code = r#"
        let nums = range(1, 6);
        let reversed = reverse(nums);
        product(reversed)
    "#;

    let result = evaluator.eval_str(code).unwrap();
    assert_eq!(result, Value::Number(120.0)); // 5! = 120
}

#[test]
fn test_range_and_len() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("len(range(0, 1000))").unwrap();
    assert_eq!(result, Value::Number(1000.0));
}

#[test]
fn test_product_of_range() {
    let mut evaluator = Evaluator::new();

    // Product of range(1, 6) = 1 * 2 * 3 * 4 * 5 = 120
    let result = evaluator.eval_str("product(range(1, 6))").unwrap();
    assert_eq!(result, Value::Number(120.0));
}

#[test]
fn test_reverse_twice() {
    let mut evaluator = Evaluator::new();

    let code = r#"
        let arr = [1, 2, 3];
        let once = reverse(arr);
        reverse(once)
    "#;

    let result = evaluator.eval_str(code).unwrap();

    match result {
        Value::Vector(vec) => {
            assert_eq!(vec, vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0),
            ]);
        }
        Value::Tensor(t) => {
            assert_eq!(t.data(), &[1.0, 2.0, 3.0]);
        }
        _ => panic!("Expected Vector or Tensor"),
    }
}

// ============================================================================
// Performance tests
// ============================================================================

#[test]
fn test_range_large() {
    let mut evaluator = Evaluator::new();

    // Generate large range
    let result = evaluator.eval_str("len(range(0, 10000))").unwrap();
    assert_eq!(result, Value::Number(10000.0));
}

#[test]
fn test_product_large_range() {
    let mut evaluator = Evaluator::new();

    // Product of 1..10 should be 10! = 3,628,800
    let result = evaluator.eval_str("product(range(1, 11))").unwrap();
    assert_eq!(result, Value::Number(3_628_800.0));
}

// ============================================================================
// Error handling tests
// ============================================================================

#[test]
fn test_product_wrong_arity() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("product()");
    assert!(result.is_err());
}

#[test]
fn test_range_wrong_arity() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("range(1)");
    assert!(result.is_err());

    let result = evaluator.eval_str("range(1, 2, 3, 4)");
    assert!(result.is_err());
}

#[test]
fn test_len_wrong_arity() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("len()");
    assert!(result.is_err());

    let result = evaluator.eval_str("len([1, 2], [3, 4])");
    assert!(result.is_err());
}

#[test]
fn test_reverse_wrong_arity() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("reverse()");
    assert!(result.is_err());
}
