use achronyme_types::value::Value;

pub fn apply_modulo(left: Value, right: Value) -> Result<Value, String> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => {
            if b == 0.0 {
                Err("Modulo by zero".to_string())
            } else {
                Ok(Value::Number(a % b))
            }
        }
        _ => Err("Modulo operator currently only supports numbers".to_string()),
    }
}
