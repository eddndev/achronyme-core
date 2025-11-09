use achronyme_parser::ast::BinaryOp;
use achronyme_types::complex::Complex;
use achronyme_types::value::Value;

/// Apply a binary operation to two values
pub fn apply(op: &BinaryOp, left: Value, right: Value) -> Result<Value, String> {
    match op {
        BinaryOp::Add => apply_add(left, right),
        BinaryOp::Subtract => apply_subtract(left, right),
        BinaryOp::Multiply => apply_multiply(left, right),
        BinaryOp::Divide => apply_divide(left, right),
        BinaryOp::Power => apply_power(left, right),
        BinaryOp::Modulo => apply_modulo(left, right),
        BinaryOp::Gt => apply_gt(left, right),
        BinaryOp::Lt => apply_lt(left, right),
        BinaryOp::Gte => apply_gte(left, right),
        BinaryOp::Lte => apply_lte(left, right),
        BinaryOp::Eq => apply_eq(left, right),
        BinaryOp::Neq => apply_neq(left, right),
        BinaryOp::And => apply_and(left, right),
        BinaryOp::Or => apply_or(left, right),
    }
}

fn apply_add(left: Value, right: Value) -> Result<Value, String> {
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

        _ => Err("Incompatible types for addition".to_string()),
    }
}

fn apply_subtract(left: Value, right: Value) -> Result<Value, String> {
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

fn apply_multiply(left: Value, right: Value) -> Result<Value, String> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
        (Value::Complex(a), Value::Complex(b)) => Ok(Value::Complex(a * b)),

        // Tensor support (optimized path)
        (Value::Tensor(a), Value::Tensor(b)) => {
            // If both are matrices, do matrix multiplication
            if a.is_matrix() && b.is_matrix() {
                a.matmul(&b)
                    .map(Value::Tensor)
                    .map_err(|e| e.to_string())
            } else {
                // Otherwise, do element-wise multiplication (Hadamard product)
                a.mul(&b)
                    .map(Value::Tensor)
                    .map_err(|e| e.to_string())
            }
        }
        (Value::ComplexTensor(a), Value::ComplexTensor(b)) => {
            a.mul(&b).map(Value::ComplexTensor).map_err(|e| e.to_string())
        }

        // Type promotion: Number → Complex
        (Value::Number(a), Value::Complex(b)) => {
            Ok(Value::Complex(Complex::from_real(a) * b))
        }
        (Value::Complex(a), Value::Number(b)) => {
            Ok(Value::Complex(a * Complex::from_real(b)))
        }

        // Legacy Vector * Vector
        (Value::Vector(ref a), Value::Vector(ref b)) => {
            // Check if both vectors are numeric
            if Value::is_numeric_vector(a) && Value::is_numeric_vector(b) {
                // Check if any element is complex
                let has_complex_a = a.iter().any(|v| matches!(v, Value::Complex(_)));
                let has_complex_b = b.iter().any(|v| matches!(v, Value::Complex(_)));

                if has_complex_a || has_complex_b {
                    // Complex tensor multiplication
                    let tensor_a = Value::to_complex_tensor(a).map_err(|_| "Type conversion error")?;
                    let tensor_b = Value::to_complex_tensor(b).map_err(|_| "Type conversion error")?;
                    let result = tensor_a.mul(&tensor_b).map_err(|e| e.to_string())?;
                    Ok(Value::ComplexTensor(result))
                } else {
                    // Real tensor multiplication
                    let tensor_a = Value::to_real_tensor(a).map_err(|_| "Type conversion error")?;
                    let tensor_b = Value::to_real_tensor(b).map_err(|_| "Type conversion error")?;
                    let result = tensor_a.mul(&tensor_b).map_err(|e| e.to_string())?;
                    Ok(Value::Tensor(result))
                }
            } else {
                Err("Vector multiplication requires numeric vectors".to_string())
            }
        }

        // Broadcasting: Scalar * Vector
        (Value::Number(scalar), Value::Vector(ref vec)) => {
            if Value::is_numeric_vector(vec) {
                let result: Vec<Value> = vec.iter().map(|v| match v {
                    Value::Number(n) => Value::Number(n * scalar),
                    Value::Complex(c) => Value::Complex(*c * Complex::from_real(scalar)),
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
                    Value::Number(n) => Value::Number(n * scalar),
                    Value::Complex(c) => Value::Complex(*c * Complex::from_real(scalar)),
                    _ => unreachable!(),
                }).collect();
                Ok(Value::Vector(result))
            } else {
                Err("Broadcasting requires numeric vector".to_string())
            }
        }

        // Broadcasting: Complex * Vector
        (Value::Complex(c), Value::Vector(ref vec)) => {
            if Value::is_numeric_vector(vec) {
                let result: Vec<Value> = vec.iter().map(|v| match v {
                    Value::Number(n) => Value::Complex(Complex::from_real(*n) * c),
                    Value::Complex(cv) => Value::Complex(*cv * c),
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
                    Value::Number(n) => Value::Complex(Complex::from_real(*n) * c),
                    Value::Complex(cv) => Value::Complex(*cv * c),
                    _ => unreachable!(),
                }).collect();
                Ok(Value::Vector(result))
            } else {
                Err("Broadcasting requires numeric vector".to_string())
            }
        }

        // Broadcasting: Tensor * Scalar
        (Value::Tensor(t), Value::Number(scalar)) => {
            Ok(Value::Tensor(t.mul_scalar(scalar)))
        }
        (Value::Number(scalar), Value::Tensor(t)) => {
            Ok(Value::Tensor(t.mul_scalar(scalar)))
        }

        // Broadcasting: Tensor * Complex
        (Value::Tensor(t), Value::Complex(c)) => {
            let ct = t.to_complex();
            Ok(Value::ComplexTensor(ct.mul_scalar(c)))
        }
        (Value::Complex(c), Value::Tensor(t)) => {
            let ct = t.to_complex();
            Ok(Value::ComplexTensor(ct.mul_scalar(c)))
        }

        // Broadcasting: ComplexTensor * Scalar
        (Value::ComplexTensor(ct), Value::Number(scalar)) => {
            Ok(Value::ComplexTensor(ct.mul_scalar(Complex::from_real(scalar))))
        }
        (Value::Number(scalar), Value::ComplexTensor(ct)) => {
            Ok(Value::ComplexTensor(ct.mul_scalar(Complex::from_real(scalar))))
        }

        // Broadcasting: ComplexTensor * Complex
        (Value::ComplexTensor(ct), Value::Complex(c)) => {
            Ok(Value::ComplexTensor(ct.mul_scalar(c)))
        }
        (Value::Complex(c), Value::ComplexTensor(ct)) => {
            Ok(Value::ComplexTensor(ct.mul_scalar(c)))
        }

        _ => Err("Incompatible types for multiplication".to_string()),
    }
}

fn apply_divide(left: Value, right: Value) -> Result<Value, String> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => {
            if b == 0.0 {
                Err("Division by zero".to_string())
            } else {
                Ok(Value::Number(a / b))
            }
        }
        (Value::Complex(a), Value::Complex(b)) => Ok(Value::Complex(a / b)),

        // Tensor support (optimized path)
        (Value::Tensor(a), Value::Tensor(b)) => {
            a.div(&b).map(Value::Tensor).map_err(|e| e.to_string())
        }
        (Value::ComplexTensor(a), Value::ComplexTensor(b)) => {
            a.div(&b).map(Value::ComplexTensor).map_err(|e| e.to_string())
        }

        // Type promotion: Number → Complex
        (Value::Number(a), Value::Complex(b)) => {
            Ok(Value::Complex(Complex::from_real(a) / b))
        }
        (Value::Complex(a), Value::Number(b)) => {
            Ok(Value::Complex(a / Complex::from_real(b)))
        }

        // Legacy Vector / Vector
        (Value::Vector(ref a), Value::Vector(ref b)) => {
            // Check if both vectors are numeric
            if Value::is_numeric_vector(a) && Value::is_numeric_vector(b) {
                // Check if any element is complex
                let has_complex_a = a.iter().any(|v| matches!(v, Value::Complex(_)));
                let has_complex_b = b.iter().any(|v| matches!(v, Value::Complex(_)));

                if has_complex_a || has_complex_b {
                    // Complex tensor division
                    let tensor_a = Value::to_complex_tensor(a).map_err(|_| "Type conversion error")?;
                    let tensor_b = Value::to_complex_tensor(b).map_err(|_| "Type conversion error")?;
                    let result = tensor_a.div(&tensor_b).map_err(|e| e.to_string())?;
                    Ok(Value::ComplexTensor(result))
                } else {
                    // Real tensor division
                    let tensor_a = Value::to_real_tensor(a).map_err(|_| "Type conversion error")?;
                    let tensor_b = Value::to_real_tensor(b).map_err(|_| "Type conversion error")?;
                    let result = tensor_a.div(&tensor_b).map_err(|e| e.to_string())?;
                    Ok(Value::Tensor(result))
                }
            } else {
                Err("Vector division requires numeric vectors".to_string())
            }
        }

        // Broadcasting: Scalar / Vector
        (Value::Number(scalar), Value::Vector(ref vec)) => {
            if Value::is_numeric_vector(vec) {
                let result: Result<Vec<Value>, String> = vec.iter().map(|v| match v {
                    Value::Number(n) => if *n == 0.0 { Err("Division by zero".to_string()) } else { Ok(Value::Number(scalar / n)) },
                    Value::Complex(c) => if c.re == 0.0 && c.im == 0.0 { Err("Division by zero".to_string()) } else { Ok(Value::Complex(Complex::from_real(scalar) / *c)) },
                    _ => unreachable!(),
                }).collect();
                result.map(Value::Vector)
            } else {
                Err("Broadcasting requires numeric vector".to_string())
            }
        }
        (Value::Vector(ref vec), Value::Number(scalar)) => {
            if scalar == 0.0 {
                return Err("Division by zero".to_string());
            }
            if Value::is_numeric_vector(vec) {
                let result: Vec<Value> = vec.iter().map(|v| match v {
                    Value::Number(n) => Value::Number(n / scalar),
                    Value::Complex(c) => Value::Complex(*c / Complex::from_real(scalar)),
                    _ => unreachable!(),
                }).collect();
                Ok(Value::Vector(result))
            } else {
                Err("Broadcasting requires numeric vector".to_string())
            }
        }

        // Broadcasting: Complex / Vector
        (Value::Complex(c), Value::Vector(ref vec)) => {
            if Value::is_numeric_vector(vec) {
                let result: Result<Vec<Value>, String> = vec.iter().map(|v| match v {
                    Value::Number(n) => if *n == 0.0 { Err("Division by zero".to_string()) } else { Ok(Value::Complex(c / Complex::from_real(*n))) },
                    Value::Complex(cv) => if cv.re == 0.0 && cv.im == 0.0 { Err("Division by zero".to_string()) } else { Ok(Value::Complex(c / *cv)) },
                    _ => unreachable!(),
                }).collect();
                result.map(Value::Vector)
            } else {
                Err("Broadcasting requires numeric vector".to_string())
            }
        }
        (Value::Vector(ref vec), Value::Complex(c)) => {
            if c.re == 0.0 && c.im == 0.0 {
                return Err("Division by zero".to_string());
            }
            if Value::is_numeric_vector(vec) {
                let result: Vec<Value> = vec.iter().map(|v| match v {
                    Value::Number(n) => Value::Complex(Complex::from_real(*n) / c),
                    Value::Complex(cv) => Value::Complex(*cv / c),
                    _ => unreachable!(),
                }).collect();
                Ok(Value::Vector(result))
            } else {
                Err("Broadcasting requires numeric vector".to_string())
            }
        }

        // Broadcasting: Tensor / Scalar
        (Value::Tensor(t), Value::Number(scalar)) => {
            t.div_scalar(scalar).map(Value::Tensor)
        }
        (Value::Number(scalar), Value::Tensor(t)) => {
            // scalar / tensor = scalar * (1 / tensor)
            // We need to compute element-wise: scalar / each_element
            let data: Vec<f64> = t.data().iter().map(|&x| {
                if x == 0.0 {
                    f64::INFINITY // Or could return error
                } else {
                    scalar / x
                }
            }).collect();
            use achronyme_types::tensor::RealTensor;
            RealTensor::new(data, t.shape().to_vec())
                .map(Value::Tensor)
                .map_err(|e| e.to_string())
        }

        // Broadcasting: Tensor / Complex
        (Value::Tensor(t), Value::Complex(c)) => {
            let ct = t.to_complex();
            ct.div_scalar(c).map(Value::ComplexTensor)
        }
        (Value::Complex(c), Value::Tensor(t)) => {
            // c / tensor: element-wise c / each_element
            let data: Vec<Complex> = t.data().iter().map(|&x| {
                if x == 0.0 {
                    Complex::new(f64::INFINITY, 0.0)
                } else {
                    c / Complex::from_real(x)
                }
            }).collect();
            use achronyme_types::tensor::ComplexTensor;
            ComplexTensor::new(data, t.shape().to_vec())
                .map(Value::ComplexTensor)
                .map_err(|e| e.to_string())
        }

        // Broadcasting: ComplexTensor / Scalar
        (Value::ComplexTensor(ct), Value::Number(scalar)) => {
            ct.div_scalar(Complex::from_real(scalar)).map(Value::ComplexTensor)
        }
        (Value::Number(scalar), Value::ComplexTensor(ct)) => {
            // scalar / tensor: element-wise scalar / each_element
            let data: Vec<Complex> = ct.data().iter().map(|c| {
                if c.re == 0.0 && c.im == 0.0 {
                    Complex::new(f64::INFINITY, 0.0)
                } else {
                    Complex::from_real(scalar) / *c
                }
            }).collect();
            use achronyme_types::tensor::ComplexTensor;
            ComplexTensor::new(data, ct.shape().to_vec())
                .map(Value::ComplexTensor)
                .map_err(|e| e.to_string())
        }

        // Broadcasting: ComplexTensor / Complex
        (Value::ComplexTensor(ct), Value::Complex(c)) => {
            ct.div_scalar(c).map(Value::ComplexTensor)
        }
        (Value::Complex(c), Value::ComplexTensor(ct)) => {
            // c / tensor: element-wise c / each_element
            let data: Vec<Complex> = ct.data().iter().map(|elem| {
                if elem.re == 0.0 && elem.im == 0.0 {
                    Complex::new(f64::INFINITY, 0.0)
                } else {
                    c / *elem
                }
            }).collect();
            use achronyme_types::tensor::ComplexTensor;
            ComplexTensor::new(data, ct.shape().to_vec())
                .map(Value::ComplexTensor)
                .map_err(|e| e.to_string())
        }

        _ => Err("Incompatible types for division".to_string()),
    }
}

fn apply_power(left: Value, right: Value) -> Result<Value, String> {
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

fn apply_modulo(left: Value, right: Value) -> Result<Value, String> {
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

// Comparison operators (return boolean values)
fn apply_gt(left: Value, right: Value) -> Result<Value, String> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a > b)),
        _ => Err("Comparison operators currently only support numbers".to_string()),
    }
}

fn apply_lt(left: Value, right: Value) -> Result<Value, String> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a < b)),
        _ => Err("Comparison operators currently only support numbers".to_string()),
    }
}

fn apply_gte(left: Value, right: Value) -> Result<Value, String> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a >= b)),
        _ => Err("Comparison operators currently only support numbers".to_string()),
    }
}

fn apply_lte(left: Value, right: Value) -> Result<Value, String> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a <= b)),
        _ => Err("Comparison operators currently only support numbers".to_string()),
    }
}

fn apply_eq(left: Value, right: Value) -> Result<Value, String> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a == b)),
        (Value::Boolean(a), Value::Boolean(b)) => Ok(Value::Boolean(a == b)),
        (Value::String(a), Value::String(b)) => Ok(Value::Boolean(a == b)),
        _ => Err("Comparison operators support numbers, booleans, and strings".to_string()),
    }
}

fn apply_neq(left: Value, right: Value) -> Result<Value, String> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a != b)),
        (Value::Boolean(a), Value::Boolean(b)) => Ok(Value::Boolean(a != b)),
        (Value::String(a), Value::String(b)) => Ok(Value::Boolean(a != b)),
        _ => Err("Comparison operators support numbers, booleans, and strings".to_string()),
    }
}

fn apply_and(left: Value, right: Value) -> Result<Value, String> {
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

fn apply_or(left: Value, right: Value) -> Result<Value, String> {
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