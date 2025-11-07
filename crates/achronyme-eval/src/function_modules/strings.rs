use crate::functions::FunctionRegistry;
use achronyme_types::value::Value;

pub fn register_functions(registry: &mut FunctionRegistry) {
    // Basic string operations
    registry.register("concat", concat, 2);
    registry.register("length", length, 1);
}

// Implementations

/// Concatenate two strings
fn concat(args: &[Value]) -> Result<Value, String> {
    match (&args[0], &args[1]) {
        (Value::String(s1), Value::String(s2)) => {
            Ok(Value::String(format!("{}{}", s1, s2)))
        }
        _ => Err("concat() requires two strings".to_string()),
    }
}

/// Get the length of a string
fn length(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::String(s) => Ok(Value::Number(s.len() as f64)),
        _ => Err("length() requires a string".to_string()),
    }
}
