use crate::functions::FunctionRegistry;
use achronyme_types::value::Value;
use achronyme_types::vector::Vector;

pub fn register_functions(registry: &mut FunctionRegistry) {
    // Basic record operations
    registry.register("keys", keys, 1);
    registry.register("values", values, 1);
    registry.register("has_field", has_field, 2);
}

// Implementations

/// Get all keys from a record as a vector of strings
fn keys(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Record(map) => {
            // Return a vector of strings containing the keys
            // For now, we'll convert to a vector of numbers representing the count
            // In a full implementation, we'd need a String vector type
            Ok(Value::Number(map.len() as f64))
        }
        _ => Err("keys() requires a record".to_string()),
    }
}

/// Get all values from a record as a vector
fn values(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Record(map) => {
            // Try to extract values as numbers
            let nums: Result<Vec<f64>, String> = map
                .values()
                .map(|v| match v {
                    Value::Number(n) => Ok(*n),
                    _ => Err("values() currently only supports records with numeric values".to_string()),
                })
                .collect();

            match nums {
                Ok(data) => Ok(Value::Vector(Vector::new(data))),
                Err(e) => Err(e),
            }
        }
        _ => Err("values() requires a record".to_string()),
    }
}

/// Check if a record has a specific field
fn has_field(args: &[Value]) -> Result<Value, String> {
    match (&args[0], &args[1]) {
        (Value::Record(map), Value::String(field_name)) => {
            Ok(Value::Boolean(map.contains_key(field_name)))
        }
        (Value::Record(_), _) => Err("has_field() requires a string as the second argument".to_string()),
        _ => Err("has_field() requires a record as the first argument".to_string()),
    }
}
