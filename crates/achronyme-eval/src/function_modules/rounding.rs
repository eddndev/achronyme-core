use crate::functions::FunctionRegistry;
use crate::unary_math_fn;
use achronyme_types::value::Value;

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

fn floor(args: &[Value]) -> Result<Value, String> {
    unary_math_fn!("floor", f64::floor, &args[0])
}

fn ceil(args: &[Value]) -> Result<Value, String> {
    unary_math_fn!("ceil", f64::ceil, &args[0])
}

fn round(args: &[Value]) -> Result<Value, String> {
    unary_math_fn!("round", f64::round, &args[0])
}

fn abs(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Number(x) => Ok(Value::Number(x.abs())),
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
        Value::Complex(c) => {
            // For complex numbers, abs returns the magnitude as a real number
            Ok(Value::Number(c.norm()))
        }
        _ => Err("abs() requires a number, vector, or complex number".to_string()),
    }
}

fn trunc(args: &[Value]) -> Result<Value, String> {
    unary_math_fn!("trunc", f64::trunc, &args[0])
}

fn sign(args: &[Value]) -> Result<Value, String> {
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

fn deg(args: &[Value]) -> Result<Value, String> {
    unary_math_fn!("deg", |x: f64| x.to_degrees(), &args[0])
}

fn rad(args: &[Value]) -> Result<Value, String> {
    unary_math_fn!("rad", |x: f64| x.to_radians(), &args[0])
}

fn min(args: &[Value]) -> Result<Value, String> {
    if args.is_empty() {
        return Err("min() requires at least one argument".to_string());
    }
    if args.len() == 1 {
        if let Value::Vector(v) = &args[0] {
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
    }
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

fn max(args: &[Value]) -> Result<Value, String> {
    if args.is_empty() {
        return Err("max() requires at least one argument".to_string());
    }
    if args.len() == 1 {
        if let Value::Vector(v) = &args[0] {
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
    }
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
