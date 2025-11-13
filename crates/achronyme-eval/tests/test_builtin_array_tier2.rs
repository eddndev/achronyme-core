/// Integration tests for Tier 2 array functions (predicates & search)
///
/// Tests: any, all, find, findIndex, count, contains

use achronyme_eval::Evaluator;
use achronyme_types::value::Value;

// ============================================================================
// any() tests
// ============================================================================

#[test]
fn test_any_basic_true() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("any([1, 2, 3, 4], x => x > 3)").unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_any_basic_false() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("any([1, 2, 3], x => x > 10)").unwrap();
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_any_empty_array() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("any([], x => x > 0)").unwrap();
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_any_with_range() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("any(range(1, 10), x => x == 5)").unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_any_short_circuits() {
    let mut evaluator = Evaluator::new();

    // Should find 3 without checking all elements
    let result = evaluator.eval_str("any([1, 2, 3, 4, 5], x => x == 3)").unwrap();
    assert_eq!(result, Value::Boolean(true));
}

// ============================================================================
// all() tests
// ============================================================================

#[test]
fn test_all_basic_true() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("all([2, 4, 6, 8], x => x % 2 == 0)").unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_all_basic_false() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("all([1, 2, 3], x => x > 2)").unwrap();
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_all_empty_array() {
    let mut evaluator = Evaluator::new();

    // Vacuous truth: all elements of empty set satisfy any predicate
    let result = evaluator.eval_str("all([], x => x > 0)").unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_all_with_range() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("all(range(1, 6), x => x > 0)").unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_all_short_circuits() {
    let mut evaluator = Evaluator::new();

    // Should stop at first false (element 1)
    let result = evaluator.eval_str("all([1, 2, 3, 4, 5], x => x > 1)").unwrap();
    assert_eq!(result, Value::Boolean(false));
}

// ============================================================================
// find() tests
// ============================================================================

#[test]
fn test_find_basic() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("find([1, 2, 3, 4, 5], x => x > 3)").unwrap();
    assert_eq!(result, Value::Number(4.0)); // First element > 3
}

#[test]
fn test_find_not_found() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("find([1, 2, 3], x => x > 10)");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not found"));
}

#[test]
fn test_find_first_match() {
    let mut evaluator = Evaluator::new();

    // Should return 2, not 4 or 6
    let result = evaluator.eval_str("find([1, 2, 3, 4, 5, 6], x => x % 2 == 0)").unwrap();
    assert_eq!(result, Value::Number(2.0));
}

#[test]
fn test_find_with_range() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("find(range(1, 100), x => x * x > 50)").unwrap();
    // First number where x^2 > 50 is 8 (8^2 = 64)
    assert_eq!(result, Value::Number(8.0));
}

// ============================================================================
// findIndex() tests
// ============================================================================

#[test]
fn test_find_index_basic() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("findIndex([1, 2, 3, 4, 5], x => x > 3)").unwrap();
    assert_eq!(result, Value::Number(3.0)); // Index of 4
}

#[test]
fn test_find_index_not_found() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("findIndex([1, 2, 3], x => x > 10)").unwrap();
    assert_eq!(result, Value::Number(-1.0));
}

#[test]
fn test_find_index_first_element() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("findIndex([5, 4, 3, 2, 1], x => x == 5)").unwrap();
    assert_eq!(result, Value::Number(0.0));
}

#[test]
fn test_find_index_last_element() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("findIndex([1, 2, 3, 4, 5], x => x == 5)").unwrap();
    assert_eq!(result, Value::Number(4.0));
}

// ============================================================================
// count() tests
// ============================================================================

#[test]
fn test_count_basic() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("count([1, 2, 3, 4, 5], x => x > 2)").unwrap();
    assert_eq!(result, Value::Number(3.0)); // 3, 4, 5
}

#[test]
fn test_count_zero() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("count([1, 2, 3], x => x > 10)").unwrap();
    assert_eq!(result, Value::Number(0.0));
}

#[test]
fn test_count_all() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("count([1, 2, 3, 4, 5], x => x > 0)").unwrap();
    assert_eq!(result, Value::Number(5.0));
}

#[test]
fn test_count_even_numbers() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("count(range(1, 11), x => x % 2 == 0)").unwrap();
    assert_eq!(result, Value::Number(5.0)); // 2, 4, 6, 8, 10
}

#[test]
fn test_count_empty_array() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("count([], x => x > 0)").unwrap();
    assert_eq!(result, Value::Number(0.0));
}

// ============================================================================
// contains() tests
// ============================================================================

#[test]
fn test_contains_true() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("contains([1, 2, 3, 4, 5], 3)").unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_contains_false() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("contains([1, 2, 3], 10)").unwrap();
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_contains_empty_array() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("contains([], 1)").unwrap();
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_contains_first_element() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("contains([1, 2, 3], 1)").unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_contains_last_element() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("contains([1, 2, 3], 3)").unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_contains_string() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("contains(\"hello world\", \"world\")").unwrap();
    assert_eq!(result, Value::Boolean(true));

    let result = evaluator.eval_str("contains(\"hello\", \"bye\")").unwrap();
    assert_eq!(result, Value::Boolean(false));
}

// ============================================================================
// Combined tests
// ============================================================================

#[test]
fn test_any_and_all_together() {
    let mut evaluator = Evaluator::new();

    let code = r#"
        let nums = range(1, 10);
        let has_even = any(nums, x => x % 2 == 0);
        let all_positive = all(nums, x => x > 0);
        has_even && all_positive
    "#;

    let result = evaluator.eval_str(code).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_find_and_contains() {
    let mut evaluator = Evaluator::new();

    let code = r#"
        let nums = [1, 2, 3, 4, 5];
        let found = find(nums, x => x > 3);
        contains(nums, found)
    "#;

    let result = evaluator.eval_str(code).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_count_vs_len() {
    let mut evaluator = Evaluator::new();

    let code = r#"
        let nums = range(1, 11);
        let even_count = count(nums, x => x % 2 == 0);
        let total_len = len(nums);
        even_count < total_len
    "#;

    let result = evaluator.eval_str(code).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_filter_then_any() {
    let mut evaluator = Evaluator::new();

    let code = r#"
        let nums = range(1, 20);
        let evens = filter(x => x % 2 == 0, nums);
        any(evens, x => x > 10)
    "#;

    let result = evaluator.eval_str(code).unwrap();
    assert_eq!(result, Value::Boolean(true)); // 12, 14, 16, 18 are > 10
}

#[test]
fn test_find_index_then_access() {
    let mut evaluator = Evaluator::new();

    // Find index, then access that element
    let code = r#"
        let nums = [10, 20, 30, 40, 50];
        let idx = findIndex(nums, x => x == 30);
        nums[idx]
    "#;

    let result = evaluator.eval_str(code).unwrap();
    assert_eq!(result, Value::Number(30.0));
}

// ============================================================================
// Performance & edge cases
// ============================================================================

#[test]
fn test_any_large_range() {
    let mut evaluator = Evaluator::new();

    // Should short-circuit quickly
    let result = evaluator.eval_str("any(range(0, 10000), x => x == 100)").unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_count_large_range() {
    let mut evaluator = Evaluator::new();

    // Count multiples of 5 in range 0-100
    let result = evaluator.eval_str("count(range(0, 100), x => x % 5 == 0)").unwrap();
    assert_eq!(result, Value::Number(20.0)); // 0, 5, 10, ..., 95
}

// ============================================================================
// Error handling
// ============================================================================

#[test]
fn test_any_wrong_arity() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("any([1, 2, 3])");
    assert!(result.is_err());
}

#[test]
fn test_contains_wrong_arity() {
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval_str("contains([1, 2, 3])");
    assert!(result.is_err());
}
