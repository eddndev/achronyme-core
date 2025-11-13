use crate::functions::FunctionRegistry;
use crate::unary_math_fn;
use achronyme_types::value::Value;
use achronyme_types::Environment;

pub fn register_functions(registry: &mut FunctionRegistry) {
    registry.register("floor", floor, 1);
    registry.register("ceil", ceil, 1);
    registry.register("round", round, 1);
    registry.register("trunc", trunc, 1);
    registry.register("abs", abs, 1);
    registry.register("sign", sign, 1);
    registry.register("deg", deg, 1);
    registry.register("rad", rad, 1);
    registry.register("min", min, -1);
    registry.register("max", max, -1);
}

// Implementations

fn floor(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    unary_math_fn!("floor", f64::floor, &args[0])
}

fn ceil(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    unary_math_fn!("ceil", f64::ceil, &args[0])
}

fn round(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    unary_math_fn!("round", f64::round, &args[0])
}

fn abs(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    match &args[0] {
        Value::Number(x) => Ok(Value::Number(x.abs())),
        Value::Complex(c) => {
            // For complex numbers, abs returns the magnitude as a real number
            Ok(Value::Number(c.norm()))
        }

        // Tensor support (optimized path)
        Value::Tensor(t) => {
            let data: Vec<f64> = t.data().iter().map(|&x| x.abs()).collect();
            let result = achronyme_types::tensor::RealTensor::new(data, t.shape().to_vec())
                .map_err(|e| format!("abs(): {}", e))?;
            Ok(Value::Tensor(result))
        }
        Value::ComplexTensor(t) => {
            // For complex tensor, abs returns real tensor of magnitudes
            let data: Vec<f64> = t.data().iter().map(|c| c.norm()).collect();
            let result = achronyme_types::tensor::RealTensor::new(data, t.shape().to_vec())
                .map_err(|e| format!("abs(): {}", e))?;
            Ok(Value::Tensor(result))
        }

        // Legacy Vector support (backward compatibility)
        Value::Vector(v) => {
            let mut result = Vec::new();
            for val in v {
                match val {
                    Value::Number(n) => result.push(Value::Number(n.abs())),
                    Value::Complex(c) => result.push(Value::Number(c.norm())),
                    _ => return Err("abs() can only be applied to numeric vectors".to_string()),
                }
            }
            Ok(Value::Vector(result))
        }

        _ => Err("abs() requires a number, vector, tensor, or complex number".to_string()),
    }
}

fn trunc(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    unary_math_fn!("trunc", f64::trunc, &args[0])
}

fn sign(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    unary_math_fn!("sign", |x: f64| {
        if x > 0.0 {
            1.0
        } else if x < 0.0 {
            -1.0
        } else {
            0.0
        }
    }, &args[0])
}

fn deg(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    unary_math_fn!("deg", |x: f64| x.to_degrees(), &args[0])
}

fn rad(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    unary_math_fn!("rad", |x: f64| x.to_radians(), &args[0])
}

fn min(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    if args.is_empty() {
        return Err("min() requires at least one argument".to_string());
    }

    // Single argument case - find min of elements in vector/tensor
    if args.len() == 1 {
        match &args[0] {
            Value::Tensor(t) => {
                return t.min().map(Value::Number).map_err(|e| e.to_string());
            }
            Value::Vector(v) => {
                if v.is_empty() {
                    return Err("min() requires a non-empty vector".to_string());
                }
                let mut min_val = match &v[0] {
                    Value::Number(n) => *n,
                    _ => return Err("min() on a vector requires numeric values".to_string()),
                };
                for val in v.iter().skip(1) {
                    if let Value::Number(n) = val {
                        if *n < min_val {
                            min_val = *n;
                        }
                    } else {
                        return Err("min() on a vector requires numeric values".to_string());
                    }
                }
                return Ok(Value::Number(min_val));
            }
            _ => {}
        }
    }

    // Multiple arguments case - find min across arguments
    let mut result = match &args[0] {
        Value::Number(x) => *x,
        _ => return Err("min() requires numbers".to_string()),
    };
    for arg in &args[1..] {
        match arg {
            Value::Number(x) => {
                if *x < result {
                    result = *x;
                }
            }
            _ => return Err("min() requires numbers".to_string()),
        }
    }
    Ok(Value::Number(result))
}

fn max(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    if args.is_empty() {
        return Err("max() requires at least one argument".to_string());
    }

    // Single argument case - find max of elements in vector/tensor
    if args.len() == 1 {
        match &args[0] {
            Value::Tensor(t) => {
                return t.max().map(Value::Number).map_err(|e| e.to_string());
            }
            Value::Vector(v) => {
                if v.is_empty() {
                    return Err("max() requires a non-empty vector".to_string());
                }
                let mut max_val = match &v[0] {
                    Value::Number(n) => *n,
                    _ => return Err("max() on a vector requires numeric values".to_string()),
                };
                for val in v.iter().skip(1) {
                    if let Value::Number(n) = val {
                        if *n > max_val {
                            max_val = *n;
                        }
                    } else {
                        return Err("max() on a vector requires numeric values".to_string());
                    }
                }
                return Ok(Value::Number(max_val));
            }
            _ => {}
        }
    }

    // Multiple arguments case - find max across arguments
    let mut result = match &args[0] {
        Value::Number(x) => *x,
        _ => return Err("max() requires numbers".to_string()),
    };
    for arg in &args[1..] {
        match arg {
            Value::Number(x) => {
                if *x > result {
                    result = *x;
                }
            }
            _ => return Err("max() requires numbers".to_string()),
        }
    }
    Ok(Value::Number(result))
}
