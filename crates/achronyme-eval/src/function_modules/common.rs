/// Helper macro for unary functions that work on both scalars, vectors, and tensors
#[macro_export]
macro_rules! unary_math_fn {
    ($name:expr, $f:expr, $arg:expr) => {
        match $arg {
            achronyme_types::value::Value::Number(x) => Ok(achronyme_types::value::Value::Number($f(*x))),

            // Tensor support (optimized path)
            achronyme_types::value::Value::Tensor(t) => {
                let data: Vec<f64> = t.data().iter().map(|&x| $f(x)).collect();
                let result = achronyme_types::tensor::RealTensor::new(data, t.shape().to_vec())
                    .map_err(|e| format!("{}(): {}", $name, e))?;
                Ok(achronyme_types::value::Value::Tensor(result))
            }

            // Legacy Vector support (backward compatibility)
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

            _ => Err(format!("{}() requires a number, vector, or tensor", $name)),
        }
    };
}