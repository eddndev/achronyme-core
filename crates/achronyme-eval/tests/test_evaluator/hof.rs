use crate::helpers::{eval, eval_with_evaluator};
use achronyme_eval::Evaluator;
use achronyme_types::value::Value;

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
