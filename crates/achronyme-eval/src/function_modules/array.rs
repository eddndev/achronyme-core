/// Array utility functions
///
/// This module provides essential array operations that complement
/// the existing higher-order functions (map, filter, reduce).
///
/// Tier 1 Functions:
/// - product(array) - Multiply all elements
/// - range(start, end, step?) - Generate integer sequence
/// - len(array) - Get array/vector length
/// - reverse(array) - Reverse array order
///
/// Tier 2 Functions (Predicates & Search):
/// - any(array, predicate) - Check if any element matches
/// - all(array, predicate) - Check if all elements match
/// - find(array, predicate) - Find first matching element
/// - findIndex(array, predicate) - Find index of first match
/// - count(array, predicate) - Count matching elements
/// - contains(array, value) - Check if value exists
///
/// Tier 3 Functions (Array Transformations):
/// - zip(array1, array2) - Combine two arrays into pairs
/// - flatten(nestedArray, depth?) - Flatten nested arrays
/// - take(array, n) - Take first n elements
/// - drop(array, n) - Skip first n elements
/// - slice(array, start, end?) - Extract subarray
/// - unique(array) - Remove duplicates
/// - chunk(array, size) - Split into chunks

use crate::functions::FunctionRegistry;
use achronyme_types::value::Value;
use achronyme_types::Environment;

/// Register all array utility functions
pub fn register_functions(registry: &mut FunctionRegistry) {
    // Tier 1: Essential operations
    registry.register("product", product, 1);
    registry.register("range", range, -1); // Variadic: 2 or 3 args
    registry.register("len", len, 1);
    registry.register("reverse", reverse, 1);

    // Tier 2: Predicates and search
    // Note: any, all, find, findIndex, count are handled as HOFs in handlers/function_call.rs
    // They need evaluator access to apply lambda predicates
    registry.register("contains", contains, 2);

    // Tier 3: Array transformations
    registry.register("zip", zip, 2);
    registry.register("flatten", flatten, -1); // Variadic: 1 or 2 args
    registry.register("take", take, 2);
    registry.register("drop", drop, 2);
    registry.register("slice", slice, -1); // Variadic: 2 or 3 args
    registry.register("unique", unique, 1);
    registry.register("chunk", chunk, 2);
}

// ============================================================================
// Tier 1 Implementations
// ============================================================================

/// Compute the product of all elements in an array
///
/// Examples:
/// - product([1, 2, 3, 4]) => 24
/// - product([2, 3, 4]) => 24
/// - product([]) => 1 (empty product identity)
/// - product([5]) => 5
///
/// Performance: O(n) single pass
fn product(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    match &args[0] {
        // Tensor support (optimized path)
        Value::Tensor(t) => {
            let result: f64 = t.data().iter().product();
            Ok(Value::Number(result))
        }
        // ComplexTensor support
        Value::ComplexTensor(t) => {
            let mut result = achronyme_types::complex::Complex::new(1.0, 0.0);
            for &c in t.data() {
                result = result * c;
            }
            Ok(Value::Complex(result))
        }
        // Legacy Vector support (backward compatibility)
        Value::Vector(vec) => {
            if vec.is_empty() {
                return Ok(Value::Number(1.0)); // Empty product is 1
            }

            if !Value::is_numeric_vector(vec) {
                return Err("product() requires a numeric vector".to_string());
            }

            let mut total = Value::Number(1.0);
            for val in vec {
                total = crate::handlers::binary_ops::apply(
                    &achronyme_parser::ast::BinaryOp::Multiply,
                    total,
                    val.clone()
                )?;
            }
            Ok(total)
        }
        _ => Err("product() requires a vector or tensor".to_string()),
    }
}

/// Generate a range of integers from start to end (exclusive) with optional step
///
/// Examples:
/// - range(0, 5) => [0, 1, 2, 3, 4]
/// - range(1, 10, 2) => [1, 3, 5, 7, 9]
/// - range(5, 0, -1) => [5, 4, 3, 2, 1]
/// - range(0, 0) => []
/// - range(3, 3) => []
///
/// Differences from linspace:
/// - range generates integers, linspace generates floats
/// - range excludes end, linspace includes end
/// - range uses step, linspace uses count
///
/// Performance: O(n) where n = abs((end - start) / step)
fn range(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    // Validate argument count
    if args.len() < 2 || args.len() > 3 {
        return Err("range() expects 2 or 3 arguments: range(start, end, step?)".to_string());
    }

    // Extract start
    let start = match &args[0] {
        Value::Number(n) => *n,
        _ => return Err("range() start must be a number".to_string()),
    };

    // Extract end
    let end = match &args[1] {
        Value::Number(n) => *n,
        _ => return Err("range() end must be a number".to_string()),
    };

    // Extract step (optional, default based on direction)
    let step = if args.len() == 3 {
        match &args[2] {
            Value::Number(n) => {
                if *n == 0.0 {
                    return Err("range() step cannot be zero".to_string());
                }
                *n
            }
            _ => return Err("range() step must be a number".to_string()),
        }
    } else {
        // Auto-detect step direction
        if end >= start {
            1.0
        } else {
            -1.0
        }
    };

    // Validate step direction matches start/end
    if (end > start && step < 0.0) || (end < start && step > 0.0) {
        return Err(
            "range() step direction doesn't match start and end values".to_string()
        );
    }

    // Generate range
    let mut result = Vec::new();
    let mut current = start;

    // Avoid infinite loops by limiting iterations
    const MAX_ITERATIONS: usize = 1_000_000;
    let mut iterations = 0;

    if step > 0.0 {
        while current < end && iterations < MAX_ITERATIONS {
            result.push(Value::Number(current));
            current += step;
            iterations += 1;
        }
    } else {
        while current > end && iterations < MAX_ITERATIONS {
            result.push(Value::Number(current));
            current += step;
            iterations += 1;
        }
    }

    if iterations >= MAX_ITERATIONS {
        return Err("range() exceeded maximum iterations (1,000,000)".to_string());
    }

    Ok(Value::Vector(result))
}

/// Get the length of an array, vector, or tensor
///
/// Examples:
/// - len([1, 2, 3]) => 3
/// - len([]) => 0
/// - len([[1, 2], [3, 4]]) => 2 (outer array length)
///
/// Note: This is different from length() which only works on strings
///
/// Performance: O(1) constant time
fn len(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    let length = match &args[0] {
        Value::Vector(vec) => vec.len(),
        Value::Tensor(tensor) => tensor.data().len(),
        Value::ComplexTensor(tensor) => tensor.data().len(),
        Value::String(s) => s.len(), // Also support strings for consistency
        Value::Record(map) => map.len(), // Support records too
        _ => return Err("len() requires an array, tensor, string, or record".to_string()),
    };

    Ok(Value::Number(length as f64))
}

/// Reverse the order of elements in an array
///
/// Examples:
/// - reverse([1, 2, 3]) => [3, 2, 1]
/// - reverse([]) => []
/// - reverse(["a", "b", "c"]) => ["c", "b", "a"]
/// - reverse([[1, 2], [3, 4]]) => [[3, 4], [1, 2]]
///
/// Performance: O(n)
fn reverse(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    match &args[0] {
        Value::Vector(vec) => {
            let mut reversed = vec.clone();
            reversed.reverse();
            Ok(Value::Vector(reversed))
        }
        Value::Tensor(tensor) => {
            // Only support 1D tensors (vectors)
            if !tensor.is_vector() {
                return Err(
                    "reverse() only supports 1D tensors (use indexing for multi-dimensional)"
                        .to_string(),
                );
            }

            let mut data = tensor.data().to_vec();
            data.reverse();

            let reversed = achronyme_types::tensor::RealTensor::new(
                data,
                tensor.shape().to_vec(),
            )
            .map_err(|e| format!("Failed to create reversed tensor: {}", e))?;

            Ok(Value::Tensor(reversed))
        }
        Value::ComplexTensor(tensor) => {
            // Only support 1D tensors (vectors)
            if !tensor.is_vector() {
                return Err(
                    "reverse() only supports 1D tensors (use indexing for multi-dimensional)"
                        .to_string(),
                );
            }

            let mut data = tensor.data().to_vec();
            data.reverse();

            let reversed = achronyme_types::tensor::ComplexTensor::new(
                data,
                tensor.shape().to_vec(),
            )
            .map_err(|e| format!("Failed to create reversed tensor: {}", e))?;

            Ok(Value::ComplexTensor(reversed))
        }
        Value::String(s) => {
            // Bonus: also support string reversal
            let reversed: String = s.chars().rev().collect();
            Ok(Value::String(reversed))
        }
        _ => Err("reverse() requires an array, tensor, or string".to_string()),
    }
}

// ============================================================================
// Tier 2 Implementations (Predicates & Search)
// ============================================================================
//
// Note: any, all, find, findIndex, count are implemented in handlers/hof.rs
// They need evaluator access to apply lambda predicates

/// Check if an array contains a specific value
///
/// Examples:
/// - contains([1, 2, 3], 2) => true
/// - contains([1, 2, 3], 5) => false
/// - contains([], 1) => false
///
/// Performance: O(n) with short-circuit
fn contains(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    // This one we can implement directly since it doesn't need a lambda
    if args.len() != 2 {
        return Err("contains() expects 2 arguments".to_string());
    }

    let search_value = &args[1];

    match &args[0] {
        Value::Vector(vec) => {
            for item in vec {
                if item == search_value {
                    return Ok(Value::Boolean(true));
                }
            }
            Ok(Value::Boolean(false))
        }
        Value::Tensor(tensor) => {
            // Only works if searching for a number
            if let Value::Number(n) = search_value {
                for &val in tensor.data() {
                    if (val - n).abs() < f64::EPSILON {
                        return Ok(Value::Boolean(true));
                    }
                }
                Ok(Value::Boolean(false))
            } else {
                Err("contains() on tensor requires numeric search value".to_string())
            }
        }
        Value::String(s) => {
            // Bonus: string contains substring
            if let Value::String(substr) = search_value {
                Ok(Value::Boolean(s.contains(substr.as_str())))
            } else {
                Err("contains() on string requires string search value".to_string())
            }
        }
        _ => Err("contains() requires an array, tensor, or string".to_string()),
    }
}

// ============================================================================
// Tier 3 Implementations (Array Transformations)
// ============================================================================

/// Combine two arrays element-wise into an array of pairs
///
/// Examples:
/// - zip([1, 2, 3], ["a", "b", "c"]) => [[1, "a"], [2, "b"], [3, "c"]]
/// - zip([1, 2], [3, 4, 5]) => [[1, 3], [2, 4]] (shorter length wins)
/// - zip([], [1, 2]) => []
///
/// Performance: O(min(n, m))
fn zip(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
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
fn flatten(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
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
fn take(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
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
fn drop(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
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
fn slice(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
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
/// Performance: O(n²) due to linear search (could be optimized with hash set)
fn unique(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
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
fn chunk(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
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

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // Tier 1 Tests
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
                assert_eq!(vec, vec![
                    Value::Number(1.0),
                    Value::Number(3.0),
                    Value::Number(5.0),
                    Value::Number(7.0),
                    Value::Number(9.0),
                ]);
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
                assert_eq!(vec, vec![
                    Value::Number(3.0),
                    Value::Number(2.0),
                    Value::Number(1.0),
                ]);
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
    // Tier 2 Tests
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
    // Tier 3 Tests
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
}
