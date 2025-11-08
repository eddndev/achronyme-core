use crate::functions::FunctionRegistry;
use achronyme_types::value::Value;

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
            let keys: Vec<Value> = map.keys().map(|k| Value::String(k.clone())).collect();
            Ok(Value::Vector(keys))
        }
        _ => Err("keys() requires a record".to_string()),
    }
}

/// Get all values from a record as a vector
fn values(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Record(map) => {
            let values: Vec<Value> = map.values().cloned().collect();
            Ok(Value::Vector(values))
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
