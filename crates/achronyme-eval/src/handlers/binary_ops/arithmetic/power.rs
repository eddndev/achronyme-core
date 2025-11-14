use achronyme_types::complex::Complex;
use achronyme_types::value::Value;

pub fn apply_power(left: Value, right: Value) -> Result<Value, String> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a.powf(b))),
        (Value::Complex(a), Value::Number(b)) => Ok(Value::Complex(a.pow(b))),
        (Value::Complex(a), Value::Complex(b)) => Ok(Value::Complex(a.pow_complex(&b))),
        (Value::Number(a), Value::Complex(b)) => {
            Ok(Value::Complex(Complex::from_real(a).pow_complex(&b)))
        }

        // Broadcasting: Vector ^ Scalar
        (Value::Vector(ref vec), Value::Number(scalar)) => {
            if Value::is_numeric_vector(vec) {
                let result: Vec<Value> = vec.iter().map(|v| match v {
                    Value::Number(n) => Value::Number(n.powf(scalar)),
                    Value::Complex(c) => Value::Complex(c.pow(scalar)),
                    _ => unreachable!(),
                }).collect();
                Ok(Value::Vector(result))
            } else {
                Err("Broadcasting requires numeric vector".to_string())
            }
        }
        (Value::Number(scalar), Value::Vector(ref vec)) => {
            if Value::is_numeric_vector(vec) {
                let result: Vec<Value> = vec.iter().map(|v| match v {
                    Value::Number(n) => Value::Number(scalar.powf(*n)),
                    Value::Complex(c) => Value::Complex(Complex::from_real(scalar).pow_complex(c)),
                    _ => unreachable!(),
                }).collect();
                Ok(Value::Vector(result))
            } else {
                Err("Broadcasting requires numeric vector".to_string())
            }
        }

        // Broadcasting: Vector ^ Complex
        (Value::Vector(ref vec), Value::Complex(c)) => {
            if Value::is_numeric_vector(vec) {
                let result: Vec<Value> = vec.iter().map(|v| match v {
                    Value::Number(n) => Value::Complex(Complex::from_real(*n).pow_complex(&c)),
                    Value::Complex(cv) => Value::Complex(cv.pow_complex(&c)),
                    _ => unreachable!(),
                }).collect();
                Ok(Value::Vector(result))
            } else {
                Err("Broadcasting requires numeric vector".to_string())
            }
        }
        (Value::Complex(c), Value::Vector(ref vec)) => {
            if Value::is_numeric_vector(vec) {
                let result: Vec<Value> = vec.iter().map(|v| match v {
                    Value::Number(n) => Value::Complex(c.pow(*n)),
                    Value::Complex(cv) => Value::Complex(c.pow_complex(cv)),
                    _ => unreachable!(),
                }).collect();
                Ok(Value::Vector(result))
            } else {
                Err("Broadcasting requires numeric vector".to_string())
            }
        }
        _ => Err("Incompatible types for power".to_string()),
    }
}
