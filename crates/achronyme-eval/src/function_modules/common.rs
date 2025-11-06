/// Helper macro for unary functions that work on both scalars and vectors
#[macro_export]
macro_rules! unary_math_fn {
    ($name:expr, $f:expr, $arg:expr) => {
        match $arg {
            achronyme_types::value::Value::Number(x) => Ok(achronyme_types::value::Value::Number($f(*x))),
            achronyme_types::value::Value::Vector(v) => {
                let result: Vec<f64> = v.data().iter().map(|&x| $f(x)).collect();
                Ok(achronyme_types::value::Value::Vector(achronyme_types::vector::Vector::new(result)))
            }
            _ => Err(format!("{}() requires a number or vector", $name)),
        }
    };
}