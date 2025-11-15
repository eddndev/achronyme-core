//! Search and predicate functions for arrays
//!
//! This module provides predicates and search operations:
//! - contains(array, value) - Check if value exists
//!
//! Note: any, all, find, findIndex, count are handled as HOFs in handlers/function_call.rs
//! They need evaluator access to apply lambda predicates

use achronyme_types::value::Value;
use achronyme_types::Environment;

/// Check if an array contains a specific value
///
/// Examples:
/// - contains([1, 2, 3], 2) => true
/// - contains([1, 2, 3], 5) => false
/// - contains([], 1) => false
///
/// Performance: O(n) with short-circuit
pub fn contains(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
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
