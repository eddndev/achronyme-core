/// Helper macro for unary functions that work on both scalars and vectors
#[macro_export]
macro_rules! unary_math_fn {
    ($name:expr, $f:expr, $arg:expr) => {
        match $arg {
            Value::Number(x) => Ok(Value::Number($f(*x))),
            Value::Vector(v) => {
                let result: Vec<f64> = v.data().iter().map(|&x| $f(x)).collect();
                Ok(Value::Vector(Vector::new(result)))
            }
            _ => Err(format!("{}() requires a number or vector", $name)),
        }
    };
}