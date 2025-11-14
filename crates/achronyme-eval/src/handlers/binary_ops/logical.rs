use achronyme_types::value::Value;

pub fn apply_and(left: Value, right: Value) -> Result<Value, String> {
    let left_bool = match left {
        Value::Boolean(b) => b,
        Value::Number(n) => n != 0.0,
        _ => return Err("Logical AND operator requires boolean or number values".to_string()),
    };
    let right_bool = match right {
        Value::Boolean(b) => b,
        Value::Number(n) => n != 0.0,
        _ => return Err("Logical AND operator requires boolean or number values".to_string()),
    };
    Ok(Value::Boolean(left_bool && right_bool))
}

pub fn apply_or(left: Value, right: Value) -> Result<Value, String> {
    let left_bool = match left {
        Value::Boolean(b) => b,
        Value::Number(n) => n != 0.0,
        _ => return Err("Logical OR operator requires boolean or number values".to_string()),
    };
    let right_bool = match right {
        Value::Boolean(b) => b,
        Value::Number(n) => n != 0.0,
        _ => return Err("Logical OR operator requires boolean or number values".to_string()),
    };
    Ok(Value::Boolean(left_bool || right_bool))
}
