use crate::functions::FunctionRegistry;
use achronyme_types::value::Value;
use achronyme_types::Environment;

pub fn register_functions(registry: &mut FunctionRegistry) {
    registry.register("complex", complex, 2);
    registry.register("real", real, 1);
    registry.register("imag", imag, 1);
    registry.register("conj", conj, 1);
    registry.register("arg", arg, 1);
}

// Implementations

fn complex(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    match (&args[0], &args[1]) {
        (Value::Number(re), Value::Number(im)) => {
            Ok(Value::Complex(achronyme_types::complex::Complex::new(*re, *im)))
        }
        _ => Err("complex() requires two numbers (real, imaginary)".to_string()),
    }
}

fn real(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    match &args[0] {
        Value::Number(x) => Ok(Value::Number(*x)),
        Value::Complex(c) => Ok(Value::Number(c.re)),

        // Tensor support (optimized path)
        Value::Tensor(t) => {
            // Real tensor - return as-is (all values are already real)
            Ok(Value::Tensor(t.clone()))
        }
        Value::ComplexTensor(t) => {
            // Extract real parts from complex tensor
            let data: Vec<f64> = t.data().iter().map(|c| c.re).collect();
            let tensor = achronyme_types::tensor::RealTensor::new(data, t.shape().to_vec())
                .map_err(|e| format!("real(): {}", e))?;
            Ok(Value::Tensor(tensor))
        }

        // Legacy Vector support (backward compatibility)
        Value::Vector(v) => {
            let mut real_parts = Vec::new();
            for val in v {
                match val {
                    Value::Number(n) => real_parts.push(Value::Number(*n)),
                    Value::Complex(c) => real_parts.push(Value::Number(c.re)),
                    _ => return Err("real() on a vector requires numeric values".to_string()),
                }
            }
            Ok(Value::Vector(real_parts))
        }

        _ => Err("real() requires a number, complex number, vector, or tensor".to_string()),
    }
}

fn imag(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    match &args[0] {
        Value::Number(_) => Ok(Value::Number(0.0)),
        Value::Complex(c) => Ok(Value::Number(c.im)),

        // Tensor support (optimized path)
        Value::Tensor(t) => {
            // Real tensor - all imaginary parts are 0
            let data = vec![0.0; t.size()];
            let tensor = achronyme_types::tensor::RealTensor::new(data, t.shape().to_vec())
                .map_err(|e| format!("imag(): {}", e))?;
            Ok(Value::Tensor(tensor))
        }
        Value::ComplexTensor(t) => {
            // Extract imaginary parts from complex tensor
            let data: Vec<f64> = t.data().iter().map(|c| c.im).collect();
            let tensor = achronyme_types::tensor::RealTensor::new(data, t.shape().to_vec())
                .map_err(|e| format!("imag(): {}", e))?;
            Ok(Value::Tensor(tensor))
        }

        // Legacy Vector support (backward compatibility)
        Value::Vector(v) => {
            let mut imag_parts = Vec::new();
            for val in v {
                match val {
                    Value::Number(_) => imag_parts.push(Value::Number(0.0)),
                    Value::Complex(c) => imag_parts.push(Value::Number(c.im)),
                    _ => return Err("imag() on a vector requires numeric values".to_string()),
                }
            }
            Ok(Value::Vector(imag_parts))
        }

        _ => Err("imag() requires a number, complex number, vector, or tensor".to_string()),
    }
}

fn conj(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    match &args[0] {
        Value::Number(x) => Ok(Value::Number(*x)),
        Value::Complex(c) => Ok(Value::Complex(c.conjugate())),
        Value::Vector(v) => {
            let mut conjugates = Vec::new();
            for val in v {
                match val {
                    Value::Number(n) => conjugates.push(Value::Number(*n)),
                    Value::Complex(c) => conjugates.push(Value::Complex(c.conjugate())),
                    _ => return Err("conj() on a vector requires numeric values".to_string()),
                }
            }
            Ok(Value::Vector(conjugates))
        }
        _ => Err("conj() requires a number, complex number, or vector".to_string()),
    }
}

fn arg(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    match &args[0] {
        Value::Number(x) => Ok(Value::Number(if *x >= 0.0 { 0.0 } else { std::f64::consts::PI })),
        Value::Complex(c) => Ok(Value::Number(c.im.atan2(c.re))),
        _ => Err("arg() requires a number or complex number".to_string()),
    }
}
