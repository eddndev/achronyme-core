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

#[cfg(test)]
mod tests {
    use super::*;

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
}
