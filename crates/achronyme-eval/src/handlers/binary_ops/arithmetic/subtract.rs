use achronyme_types::complex::Complex;
use achronyme_types::value::Value;

pub fn apply_subtract(left: Value, right: Value) -> Result<Value, String> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
        (Value::Complex(a), Value::Complex(b)) => Ok(Value::Complex(a - b)),

        // Tensor support (optimized path)
        (Value::Tensor(a), Value::Tensor(b)) => {
            a.sub(&b).map(Value::Tensor).map_err(|e| e.to_string())
        }
        (Value::ComplexTensor(a), Value::ComplexTensor(b)) => {
            a.sub(&b).map(Value::ComplexTensor).map_err(|e| e.to_string())
        }

        // Type promotion: Number → Complex
        (Value::Number(a), Value::Complex(b)) => {
            Ok(Value::Complex(Complex::from_real(a) - b))
        }
        (Value::Complex(a), Value::Number(b)) => {
            Ok(Value::Complex(a - Complex::from_real(b)))
        }

        // Legacy Vector - Vector
        (Value::Vector(ref a), Value::Vector(ref b)) => {
            // Check if both vectors are numeric
            if Value::is_numeric_vector(a) && Value::is_numeric_vector(b) {
                // Check if any element is complex
                let has_complex_a = a.iter().any(|v| matches!(v, Value::Complex(_)));
                let has_complex_b = b.iter().any(|v| matches!(v, Value::Complex(_)));

                if has_complex_a || has_complex_b {
                    // Complex tensor subtraction
                    let tensor_a = Value::to_complex_tensor(a).map_err(|_| "Type conversion error")?;
                    let tensor_b = Value::to_complex_tensor(b).map_err(|_| "Type conversion error")?;
                    let result = tensor_a.sub(&tensor_b).map_err(|e| e.to_string())?;
                    Ok(Value::ComplexTensor(result))
                } else {
                    // Real tensor subtraction
                    let tensor_a = Value::to_real_tensor(a).map_err(|_| "Type conversion error")?;
                    let tensor_b = Value::to_real_tensor(b).map_err(|_| "Type conversion error")?;
                    let result = tensor_a.sub(&tensor_b).map_err(|e| e.to_string())?;
                    Ok(Value::Tensor(result))
                }
            } else {
                Err("Vector subtraction requires numeric vectors".to_string())
            }
        }

        // Broadcasting: Scalar - Vector
        (Value::Number(scalar), Value::Vector(ref vec)) => {
            if Value::is_numeric_vector(vec) {
                let result: Vec<Value> = vec.iter().map(|v| match v {
                    Value::Number(n) => Value::Number(scalar - n),
                    Value::Complex(c) => Value::Complex(Complex::from_real(scalar) - *c),
                    _ => unreachable!(),
                }).collect();
                Ok(Value::Vector(result))
            } else {
                Err("Broadcasting requires numeric vector".to_string())
            }
        }
        (Value::Vector(ref vec), Value::Number(scalar)) => {
            if Value::is_numeric_vector(vec) {
                let result: Vec<Value> = vec.iter().map(|v| match v {
                    Value::Number(n) => Value::Number(n - scalar),
                    Value::Complex(c) => Value::Complex(*c - Complex::from_real(scalar)),
                    _ => unreachable!(),
                }).collect();
                Ok(Value::Vector(result))
            } else {
                Err("Broadcasting requires numeric vector".to_string())
            }
        }

        // Broadcasting: Complex - Vector
        (Value::Complex(c), Value::Vector(ref vec)) => {
            if Value::is_numeric_vector(vec) {
                let result: Vec<Value> = vec.iter().map(|v| match v {
                    Value::Number(n) => Value::Complex(c - Complex::from_real(*n)),
                    Value::Complex(cv) => Value::Complex(c - *cv),
                    _ => unreachable!(),
                }).collect();
                Ok(Value::Vector(result))
            } else {
                Err("Broadcasting requires numeric vector".to_string())
            }
        }
        (Value::Vector(ref vec), Value::Complex(c)) => {
            if Value::is_numeric_vector(vec) {
                let result: Vec<Value> = vec.iter().map(|v| match v {
                    Value::Number(n) => Value::Complex(Complex::from_real(*n) - c),
                    Value::Complex(cv) => Value::Complex(*cv - c),
                    _ => unreachable!(),
                }).collect();
                Ok(Value::Vector(result))
            } else {
                Err("Broadcasting requires numeric vector".to_string())
            }
        }

        // Broadcasting: Tensor - Scalar
        (Value::Tensor(t), Value::Number(scalar)) => {
            Ok(Value::Tensor(t.sub_scalar(scalar)))
        }
        (Value::Number(scalar), Value::Tensor(t)) => {
            // scalar - tensor = -(tensor - scalar)
            Ok(Value::Tensor(t.sub_scalar(scalar).negate()))
        }

        // Broadcasting: Tensor - Complex
        (Value::Tensor(t), Value::Complex(c)) => {
            let ct = t.to_complex();
            Ok(Value::ComplexTensor(ct.sub_scalar(c)))
        }
        (Value::Complex(c), Value::Tensor(t)) => {
            // c - tensor = -(tensor - c)
            let ct = t.to_complex();
            Ok(Value::ComplexTensor(ct.sub_scalar(c).negate()))
        }

        // Broadcasting: ComplexTensor - Scalar
        (Value::ComplexTensor(ct), Value::Number(scalar)) => {
            Ok(Value::ComplexTensor(ct.sub_scalar(Complex::from_real(scalar))))
        }
        (Value::Number(scalar), Value::ComplexTensor(ct)) => {
            // scalar - tensor = -(tensor - scalar)
            Ok(Value::ComplexTensor(ct.sub_scalar(Complex::from_real(scalar)).negate()))
        }

        // Broadcasting: ComplexTensor - Complex
        (Value::ComplexTensor(ct), Value::Complex(c)) => {
            Ok(Value::ComplexTensor(ct.sub_scalar(c)))
        }
        (Value::Complex(c), Value::ComplexTensor(ct)) => {
            // c - tensor = -(tensor - c)
            Ok(Value::ComplexTensor(ct.sub_scalar(c).negate()))
        }

        _ => Err("Incompatible types for subtraction".to_string()),
    }
}
