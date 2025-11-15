//! Tests for array utility functions

use super::core::{len, product, range, reverse};
use super::search::contains;
use super::transform::{chunk, drop, flatten, slice, take, unique, zip};
use achronyme_types::value::Value;
use achronyme_types::Environment;

// ========================================================================
// Tier 1 Tests (Core Functions)
// ========================================================================

#[test]
fn test_product_basic() {
    let mut env = Environment::new();
    let args = vec![Value::Vector(vec![
        Value::Number(2.0),
        Value::Number(3.0),
        Value::Number(4.0),
    ])];
    let result = product(&args, &mut env).unwrap();
    assert_eq!(result, Value::Number(24.0));
}

#[test]
fn test_product_empty() {
    let mut env = Environment::new();
    let args = vec![Value::Vector(vec![])];
    let result = product(&args, &mut env).unwrap();
    assert_eq!(result, Value::Number(1.0)); // Empty product is 1
}

#[test]
fn test_range_basic() {
    let mut env = Environment::new();
    let args = vec![Value::Number(0.0), Value::Number(5.0)];
    let result = range(&args, &mut env).unwrap();

    match result {
        Value::Vector(vec) => {
            assert_eq!(vec.len(), 5);
            assert_eq!(vec[0], Value::Number(0.0));
            assert_eq!(vec[4], Value::Number(4.0));
        }
        _ => panic!("Expected Vector"),
    }
}

#[test]
fn test_range_with_step() {
    let mut env = Environment::new();
    let args = vec![Value::Number(1.0), Value::Number(10.0), Value::Number(2.0)];
    let result = range(&args, &mut env).unwrap();

    match result {
        Value::Vector(vec) => {
            assert_eq!(vec.len(), 5);
            assert_eq!(
                vec,
                vec![
                    Value::Number(1.0),
                    Value::Number(3.0),
                    Value::Number(5.0),
                    Value::Number(7.0),
                    Value::Number(9.0),
                ]
            );
        }
        _ => panic!("Expected Vector"),
    }
}

#[test]
fn test_range_negative_step() {
    let mut env = Environment::new();
    let args = vec![Value::Number(5.0), Value::Number(0.0), Value::Number(-1.0)];
    let result = range(&args, &mut env).unwrap();

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
fn test_len_vector() {
    let mut env = Environment::new();
    let args = vec![Value::Vector(vec![
        Value::Number(1.0),
        Value::Number(2.0),
        Value::Number(3.0),
    ])];
    let result = len(&args, &mut env).unwrap();
    assert_eq!(result, Value::Number(3.0));
}

#[test]
fn test_len_empty() {
    let mut env = Environment::new();
    let args = vec![Value::Vector(vec![])];
    let result = len(&args, &mut env).unwrap();
    assert_eq!(result, Value::Number(0.0));
}

#[test]
fn test_reverse_vector() {
    let mut env = Environment::new();
    let args = vec![Value::Vector(vec![
        Value::Number(1.0),
        Value::Number(2.0),
        Value::Number(3.0),
    ])];
    let result = reverse(&args, &mut env).unwrap();

    match result {
        Value::Vector(vec) => {
            assert_eq!(
                vec,
                vec![
                    Value::Number(3.0),
                    Value::Number(2.0),
                    Value::Number(1.0),
                ]
            );
        }
        _ => panic!("Expected Vector"),
    }
}

#[test]
fn test_reverse_string() {
    let mut env = Environment::new();
    let args = vec![Value::String("hello".to_string())];
    let result = reverse(&args, &mut env).unwrap();
    assert_eq!(result, Value::String("olleh".to_string()));
}

// ========================================================================
// Tier 2 Tests (Search Functions)
// ========================================================================

#[test]
fn test_contains_found() {
    let mut env = Environment::new();
    let args = vec![
        Value::Vector(vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0),
        ]),
        Value::Number(2.0),
    ];
    let result = contains(&args, &mut env).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_contains_not_found() {
    let mut env = Environment::new();
    let args = vec![
        Value::Vector(vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0),
        ]),
        Value::Number(5.0),
    ];
    let result = contains(&args, &mut env).unwrap();
    assert_eq!(result, Value::Boolean(false));
}

// ========================================================================
// Tier 3 Tests (Transform Functions)
// ========================================================================

#[test]
fn test_zip_basic() {
    let mut env = Environment::new();
    let args = vec![
        Value::Vector(vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0),
        ]),
        Value::Vector(vec![
            Value::String("a".to_string()),
            Value::String("b".to_string()),
            Value::String("c".to_string()),
        ]),
    ];
    let result = zip(&args, &mut env).unwrap();
    match result {
        Value::Vector(vec) => {
            assert_eq!(vec.len(), 3);
            assert_eq!(
                vec[0],
                Value::Vector(vec![Value::Number(1.0), Value::String("a".to_string())])
            );
        }
        _ => panic!("Expected Vector"),
    }
}

#[test]
fn test_zip_different_lengths() {
    let mut env = Environment::new();
    let args = vec![
        Value::Vector(vec![Value::Number(1.0), Value::Number(2.0)]),
        Value::Vector(vec![
            Value::Number(3.0),
            Value::Number(4.0),
            Value::Number(5.0),
        ]),
    ];
    let result = zip(&args, &mut env).unwrap();
    match result {
        Value::Vector(vec) => {
            assert_eq!(vec.len(), 2); // Shorter length
        }
        _ => panic!("Expected Vector"),
    }
}

#[test]
fn test_flatten_one_level() {
    let mut env = Environment::new();
    let args = vec![Value::Vector(vec![
        Value::Vector(vec![Value::Number(1.0), Value::Number(2.0)]),
        Value::Vector(vec![Value::Number(3.0), Value::Number(4.0)]),
    ])];
    let result = flatten(&args, &mut env).unwrap();
    match result {
        Value::Vector(vec) => {
            assert_eq!(vec.len(), 4);
            assert_eq!(vec[0], Value::Number(1.0));
            assert_eq!(vec[3], Value::Number(4.0));
        }
        _ => panic!("Expected Vector"),
    }
}

#[test]
fn test_flatten_deep() {
    let mut env = Environment::new();
    let args = vec![
        Value::Vector(vec![Value::Vector(vec![Value::Vector(vec![
            Value::Number(1.0),
        ])])]),
        Value::Number(2.0),
    ];
    let result = flatten(&args, &mut env).unwrap();
    match result {
        Value::Vector(vec) => {
            assert_eq!(vec.len(), 1);
            assert_eq!(vec[0], Value::Number(1.0));
        }
        _ => panic!("Expected Vector"),
    }
}

#[test]
fn test_take_basic() {
    let mut env = Environment::new();
    let args = vec![
        Value::Vector(vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0),
            Value::Number(4.0),
            Value::Number(5.0),
        ]),
        Value::Number(3.0),
    ];
    let result = take(&args, &mut env).unwrap();
    match result {
        Value::Vector(vec) => {
            assert_eq!(vec.len(), 3);
            assert_eq!(vec[2], Value::Number(3.0));
        }
        _ => panic!("Expected Vector"),
    }
}

#[test]
fn test_drop_basic() {
    let mut env = Environment::new();
    let args = vec![
        Value::Vector(vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0),
            Value::Number(4.0),
            Value::Number(5.0),
        ]),
        Value::Number(2.0),
    ];
    let result = drop(&args, &mut env).unwrap();
    match result {
        Value::Vector(vec) => {
            assert_eq!(vec.len(), 3);
            assert_eq!(vec[0], Value::Number(3.0));
        }
        _ => panic!("Expected Vector"),
    }
}

#[test]
fn test_slice_basic() {
    let mut env = Environment::new();
    let args = vec![
        Value::Vector(vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0),
            Value::Number(4.0),
            Value::Number(5.0),
        ]),
        Value::Number(1.0),
        Value::Number(4.0),
    ];
    let result = slice(&args, &mut env).unwrap();
    match result {
        Value::Vector(vec) => {
            assert_eq!(vec.len(), 3);
            assert_eq!(vec[0], Value::Number(2.0));
            assert_eq!(vec[2], Value::Number(4.0));
        }
        _ => panic!("Expected Vector"),
    }
}

#[test]
fn test_unique_basic() {
    let mut env = Environment::new();
    let args = vec![Value::Vector(vec![
        Value::Number(1.0),
        Value::Number(2.0),
        Value::Number(2.0),
        Value::Number(3.0),
        Value::Number(1.0),
    ])];
    let result = unique(&args, &mut env).unwrap();
    match result {
        Value::Vector(vec) => {
            assert_eq!(vec.len(), 3);
            assert_eq!(vec[0], Value::Number(1.0));
            assert_eq!(vec[1], Value::Number(2.0));
            assert_eq!(vec[2], Value::Number(3.0));
        }
        _ => panic!("Expected Vector"),
    }
}

#[test]
fn test_chunk_basic() {
    let mut env = Environment::new();
    let args = vec![
        Value::Vector(vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0),
            Value::Number(4.0),
            Value::Number(5.0),
        ]),
        Value::Number(2.0),
    ];
    let result = chunk(&args, &mut env).unwrap();
    match result {
        Value::Vector(vec) => {
            assert_eq!(vec.len(), 3);
            assert_eq!(
                vec[0],
                Value::Vector(vec![Value::Number(1.0), Value::Number(2.0)])
            );
            assert_eq!(vec[2], Value::Vector(vec![Value::Number(5.0)]));
        }
        _ => panic!("Expected Vector"),
    }
}
