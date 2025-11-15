//! Array transformation functions
//!
//! This module provides array transformation operations:
//! - zip(array1, array2) - Combine two arrays into pairs
//! - flatten(nestedArray, depth?) - Flatten nested arrays
//! - take(array, n) - Take first n elements
//! - drop(array, n) - Skip first n elements
//! - slice(array, start, end?) - Extract subarray
//! - unique(array) - Remove duplicates
//! - chunk(array, size) - Split into chunks

use achronyme_types::value::Value;
use achronyme_types::Environment;

/// Combine two arrays element-wise into an array of pairs
///
/// Examples:
/// - zip([1, 2, 3], ["a", "b", "c"]) => [[1, "a"], [2, "b"], [3, "c"]]
/// - zip([1, 2], [3, 4, 5]) => [[1, 3], [2, 4]] (shorter length wins)
/// - zip([], [1, 2]) => []
///
/// Performance: O(min(n, m))
pub fn zip(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    let (vec1, vec2) = match (&args[0], &args[1]) {
        (Value::Vector(v1), Value::Vector(v2)) => (v1, v2),
        _ => return Err("zip() requires two arrays".to_string()),
    };

    let min_len = vec1.len().min(vec2.len());
    let mut result = Vec::with_capacity(min_len);

    for i in 0..min_len {
        result.push(Value::Vector(vec![vec1[i].clone(), vec2[i].clone()]));
    }

    Ok(Value::Vector(result))
}

/// Flatten nested arrays up to a specified depth
///
/// Examples:
/// - flatten([[1, 2], [3, 4]]) => [1, 2, 3, 4]
/// - flatten([[[1]], [[2]]]) => [[1], [2]] (depth=1 by default)
/// - flatten([[[1]], [[2]]], 2) => [1, 2]
/// - flatten([1, [2, [3, 4]]]) => [1, 2, [3, 4]]
/// - flatten([1, [2, [3, 4]]], 2) => [1, 2, 3, 4]
///
/// Performance: O(n * depth) where n is total element count
pub fn flatten(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    if args.is_empty() || args.len() > 2 {
        return Err("flatten() expects 1 or 2 arguments: flatten(array, depth?)".to_string());
    }

    let depth = if args.len() == 2 {
        match &args[1] {
            Value::Number(n) => {
                if *n < 0.0 || *n != n.floor() {
                    return Err("flatten() depth must be a non-negative integer".to_string());
                }
                *n as usize
            }
            _ => return Err("flatten() depth must be a number".to_string()),
        }
    } else {
        1 // Default depth
    };

    match &args[0] {
        Value::Vector(v) => Ok(Value::Vector(flatten_recursive(v, depth))),
        Value::Tensor(t) => {
            // For tensors, flatten to 1D array of numbers
            if depth == 0 {
                // Return as-is (wrapped in array)
                let elements: Vec<Value> = t.data().iter().map(|&x| Value::Number(x)).collect();
                Ok(Value::Vector(elements))
            } else {
                // Flatten tensor to 1D
                let elements: Vec<Value> = t.data().iter().map(|&x| Value::Number(x)).collect();
                Ok(Value::Vector(elements))
            }
        }
        _ => Err("flatten() requires an array or tensor".to_string()),
    }
}

/// Helper function for recursive flattening
fn flatten_recursive(array: &[Value], depth: usize) -> Vec<Value> {
    if depth == 0 {
        return array.to_vec();
    }

    let mut result = Vec::new();
    for item in array {
        match item {
            Value::Vector(inner) => {
                let flattened = flatten_recursive(inner, depth - 1);
                result.extend(flattened);
            }
            _ => result.push(item.clone()),
        }
    }
    result
}

/// Take the first n elements from an array
///
/// Examples:
/// - take([1, 2, 3, 4, 5], 3) => [1, 2, 3]
/// - take([1, 2], 5) => [1, 2] (returns all if n > length)
/// - take([], 3) => []
/// - take([1, 2, 3], 0) => []
///
/// Performance: O(min(n, length))
pub fn take(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    let array = match &args[0] {
        Value::Vector(v) => v,
        _ => return Err("take() requires an array as first argument".to_string()),
    };

    let n = match &args[1] {
        Value::Number(num) => {
            if *num < 0.0 || *num != num.floor() {
                return Err("take() count must be a non-negative integer".to_string());
            }
            *num as usize
        }
        _ => return Err("take() count must be a number".to_string()),
    };

    let take_count = n.min(array.len());
    Ok(Value::Vector(array[..take_count].to_vec()))
}

/// Skip the first n elements from an array
///
/// Examples:
/// - drop([1, 2, 3, 4, 5], 2) => [3, 4, 5]
/// - drop([1, 2], 5) => [] (returns empty if n >= length)
/// - drop([], 3) => []
/// - drop([1, 2, 3], 0) => [1, 2, 3]
///
/// Performance: O(length - n)
pub fn drop(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    let array = match &args[0] {
        Value::Vector(v) => v,
        _ => return Err("drop() requires an array as first argument".to_string()),
    };

    let n = match &args[1] {
        Value::Number(num) => {
            if *num < 0.0 || *num != num.floor() {
                return Err("drop() count must be a non-negative integer".to_string());
            }
            *num as usize
        }
        _ => return Err("drop() count must be a number".to_string()),
    };

    if n >= array.len() {
        Ok(Value::Vector(vec![]))
    } else {
        Ok(Value::Vector(array[n..].to_vec()))
    }
}

/// Extract a subarray from start to end index (exclusive)
///
/// Examples:
/// - slice([1, 2, 3, 4, 5], 1, 4) => [2, 3, 4]
/// - slice([1, 2, 3, 4, 5], 2) => [3, 4, 5] (end defaults to length)
/// - slice([1, 2, 3], 0, 2) => [1, 2]
/// - slice([1, 2, 3], 1, 1) => []
/// - slice([1, 2, 3], 5, 10) => [] (out of bounds returns empty)
///
/// Performance: O(end - start)
pub fn slice(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    if args.len() < 2 || args.len() > 3 {
        return Err("slice() expects 2 or 3 arguments: slice(array, start, end?)".to_string());
    }

    let array = match &args[0] {
        Value::Vector(v) => v,
        _ => return Err("slice() requires an array as first argument".to_string()),
    };

    let start = match &args[1] {
        Value::Number(n) => {
            if *n < 0.0 || *n != n.floor() {
                return Err("slice() start index must be a non-negative integer".to_string());
            }
            *n as usize
        }
        _ => return Err("slice() start index must be a number".to_string()),
    };

    let end = if args.len() == 3 {
        match &args[2] {
            Value::Number(n) => {
                if *n < 0.0 || *n != n.floor() {
                    return Err("slice() end index must be a non-negative integer".to_string());
                }
                *n as usize
            }
            _ => return Err("slice() end index must be a number".to_string()),
        }
    } else {
        array.len() // Default to end of array
    };

    // Handle out of bounds gracefully
    if start >= array.len() {
        return Ok(Value::Vector(vec![]));
    }

    let actual_end = end.min(array.len());
    if start >= actual_end {
        return Ok(Value::Vector(vec![]));
    }

    Ok(Value::Vector(array[start..actual_end].to_vec()))
}

/// Remove duplicate values from an array (keeps first occurrence)
///
/// Examples:
/// - unique([1, 2, 2, 3, 1]) => [1, 2, 3]
/// - unique(["a", "b", "a"]) => ["a", "b"]
/// - unique([]) => []
/// - unique([1, 2, 3]) => [1, 2, 3]
///
/// Note: Uses equality comparison, so [1] != [1] for nested arrays
///
/// Performance: O(n^2) due to linear search (could be optimized with hash set)
pub fn unique(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    let array = match &args[0] {
        Value::Vector(v) => v,
        _ => return Err("unique() requires an array".to_string()),
    };

    let mut result = Vec::new();
    for item in array {
        if !result.contains(item) {
            result.push(item.clone());
        }
    }

    Ok(Value::Vector(result))
}

/// Split an array into chunks of specified size
///
/// Examples:
/// - chunk([1, 2, 3, 4, 5], 2) => [[1, 2], [3, 4], [5]]
/// - chunk([1, 2, 3, 4], 2) => [[1, 2], [3, 4]]
/// - chunk([1, 2, 3], 5) => [[1, 2, 3]]
/// - chunk([], 3) => []
///
/// Performance: O(n)
pub fn chunk(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    let array = match &args[0] {
        Value::Vector(v) => v,
        _ => return Err("chunk() requires an array as first argument".to_string()),
    };

    let size = match &args[1] {
        Value::Number(n) => {
            if *n <= 0.0 || *n != n.floor() {
                return Err("chunk() size must be a positive integer".to_string());
            }
            *n as usize
        }
        _ => return Err("chunk() size must be a number".to_string()),
    };

    if array.is_empty() {
        return Ok(Value::Vector(vec![]));
    }

    let mut result = Vec::new();
    for chunk in array.chunks(size) {
        result.push(Value::Vector(chunk.to_vec()));
    }

    Ok(Value::Vector(result))
}
