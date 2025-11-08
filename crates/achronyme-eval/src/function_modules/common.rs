/// Helper macro for unary functions that work on both scalars and vectors
#[macro_export]
macro_rules! unary_math_fn {
    ($name:expr, $f:expr, $arg:expr) => {
        match $arg {
            achronyme_types::value::Value::Number(x) => Ok(achronyme_types::value::Value::Number($f(*x))),
            achronyme_types::value::Value::Vector(v) => {
                let mut result = Vec::new();
                for val in v {
                    if let achronyme_types::value::Value::Number(n) = val {
                        result.push(achronyme_types::value::Value::Number($f(*n)));
                    } else {
                        return Err(format!("{}() can only be applied to numeric vectors", $name));
                    }
                }
                Ok(achronyme_types::value::Value::Vector(result))
            }
            _ => Err(format!("{}() requires a number or vector", $name)),
        }
    };
}