use achronyme_types::value::Value;

// Comparison operators (return boolean values)
pub fn apply_gt(left: Value, right: Value) -> Result<Value, String> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a > b)),
        _ => Err("Comparison operators currently only support numbers".to_string()),
    }
}

pub fn apply_lt(left: Value, right: Value) -> Result<Value, String> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a < b)),
        _ => Err("Comparison operators currently only support numbers".to_string()),
    }
}

pub fn apply_gte(left: Value, right: Value) -> Result<Value, String> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a >= b)),
        _ => Err("Comparison operators currently only support numbers".to_string()),
    }
}

pub fn apply_lte(left: Value, right: Value) -> Result<Value, String> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a <= b)),
        _ => Err("Comparison operators currently only support numbers".to_string()),
    }
}

pub fn apply_eq(left: Value, right: Value) -> Result<Value, String> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a == b)),
        (Value::Boolean(a), Value::Boolean(b)) => Ok(Value::Boolean(a == b)),
        (Value::String(a), Value::String(b)) => Ok(Value::Boolean(a == b)),
        _ => Err("Comparison operators support numbers, booleans, and strings".to_string()),
    }
}

pub fn apply_neq(left: Value, right: Value) -> Result<Value, String> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a != b)),
        (Value::Boolean(a), Value::Boolean(b)) => Ok(Value::Boolean(a != b)),
        (Value::String(a), Value::String(b)) => Ok(Value::Boolean(a != b)),
        _ => Err("Comparison operators support numbers, booleans, and strings".to_string()),
    }
}
