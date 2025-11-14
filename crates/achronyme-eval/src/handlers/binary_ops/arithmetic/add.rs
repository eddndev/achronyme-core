use achronyme_types::complex::Complex;
use achronyme_types::value::Value;
use crate::handlers::binary_ops::utils::value_to_string;

pub fn apply_add(left: Value, right: Value) -> Result<Value, String> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
        (Value::Complex(a), Value::Complex(b)) => Ok(Value::Complex(a + b)),

        // Tensor support (optimized path)
        (Value::Tensor(a), Value::Tensor(b)) => {
            a.add(&b).map(Value::Tensor).map_err(|e| e.to_string())
        }
        (Value::ComplexTensor(a), Value::ComplexTensor(b)) => {
            a.add(&b).map(Value::ComplexTensor).map_err(|e| e.to_string())
        }

        // Type promotion: Number → Complex
        (Value::Number(a), Value::Complex(b)) => {
            Ok(Value::Complex(Complex::from_real(a) + b))
        }
        (Value::Complex(a), Value::Number(b)) => {
            Ok(Value::Complex(a + Complex::from_real(b)))
        }

        // Legacy Vector + Vector
        (Value::Vector(ref a), Value::Vector(ref b)) => {
            // Check if both vectors are numeric
            if Value::is_numeric_vector(a) && Value::is_numeric_vector(b) {
                // Check if any element is complex
                let has_complex_a = a.iter().any(|v| matches!(v, Value::Complex(_)));
                let has_complex_b = b.iter().any(|v| matches!(v, Value::Complex(_)));

                if has_complex_a || has_complex_b {
                    // Complex tensor addition
                    let tensor_a = Value::to_complex_tensor(a).map_err(|_| "Type conversion error")?;
                    let tensor_b = Value::to_complex_tensor(b).map_err(|_| "Type conversion error")?;
                    let result = tensor_a.add(&tensor_b).map_err(|e| e.to_string())?;
                    Ok(Value::ComplexTensor(result))
                } else {
                    // Real tensor addition
                    let tensor_a = Value::to_real_tensor(a).map_err(|_| "Type conversion error")?;
                    let tensor_b = Value::to_real_tensor(b).map_err(|_| "Type conversion error")?;
                    let result = tensor_a.add(&tensor_b).map_err(|e| e.to_string())?;
                    Ok(Value::Tensor(result))
                }
            } else {
                Err("Vector addition requires numeric vectors".to_string())
            }
        }

        // Broadcasting: Scalar + Vector
        (Value::Number(scalar), Value::Vector(ref vec)) => {
            if Value::is_numeric_vector(vec) {
                let result: Vec<Value> = vec.iter().map(|v| match v {
                    Value::Number(n) => Value::Number(n + scalar),
                    Value::Complex(c) => Value::Complex(*c + Complex::from_real(scalar)),
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
                    Value::Number(n) => Value::Number(n + scalar),
                    Value::Complex(c) => Value::Complex(*c + Complex::from_real(scalar)),
                    _ => unreachable!(),
                }).collect();
                Ok(Value::Vector(result))
            } else {
                Err("Broadcasting requires numeric vector".to_string())
            }
        }

        // Broadcasting: Complex + Vector
        (Value::Complex(c), Value::Vector(ref vec)) => {
            if Value::is_numeric_vector(vec) {
                let result: Vec<Value> = vec.iter().map(|v| match v {
                    Value::Number(n) => Value::Complex(Complex::from_real(*n) + c),
                    Value::Complex(cv) => Value::Complex(*cv + c),
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
                    Value::Number(n) => Value::Complex(Complex::from_real(*n) + c),
                    Value::Complex(cv) => Value::Complex(*cv + c),
                    _ => unreachable!(),
                }).collect();
                Ok(Value::Vector(result))
            } else {
                Err("Broadcasting requires numeric vector".to_string())
            }
        }

        // Broadcasting: Tensor + Scalar
        (Value::Tensor(t), Value::Number(scalar)) => {
            Ok(Value::Tensor(t.add_scalar(scalar)))
        }
        (Value::Number(scalar), Value::Tensor(t)) => {
            Ok(Value::Tensor(t.add_scalar(scalar)))
        }

        // Broadcasting: Tensor + Complex
        (Value::Tensor(t), Value::Complex(c)) => {
            // Convert real tensor to complex tensor, then add
            let ct = t.to_complex();
            Ok(Value::ComplexTensor(ct.add_scalar(c)))
        }
        (Value::Complex(c), Value::Tensor(t)) => {
            // Convert real tensor to complex tensor, then add
            let ct = t.to_complex();
            Ok(Value::ComplexTensor(ct.add_scalar(c)))
        }

        // Broadcasting: ComplexTensor + Scalar
        (Value::ComplexTensor(ct), Value::Number(scalar)) => {
            Ok(Value::ComplexTensor(ct.add_scalar(Complex::from_real(scalar))))
        }
        (Value::Number(scalar), Value::ComplexTensor(ct)) => {
            Ok(Value::ComplexTensor(ct.add_scalar(Complex::from_real(scalar))))
        }

        // Broadcasting: ComplexTensor + Complex
        (Value::ComplexTensor(ct), Value::Complex(c)) => {
            Ok(Value::ComplexTensor(ct.add_scalar(c)))
        }
        (Value::Complex(c), Value::ComplexTensor(ct)) => {
            Ok(Value::ComplexTensor(ct.add_scalar(c)))
        }

        // String concatenation (String + String)
        (Value::String(a), Value::String(b)) => {
            Ok(Value::String(format!("{}{}", a, b)))
        }

        // String broadcasting: String + Any (automatic toString conversion)
        (Value::String(s), other) => {
            Ok(Value::String(format!("{}{}", s, value_to_string(&other))))
        }
        (other, Value::String(s)) => {
            Ok(Value::String(format!("{}{}", value_to_string(&other), s)))
        }

        _ => Err("Incompatible types for addition".to_string()),
    }
}
