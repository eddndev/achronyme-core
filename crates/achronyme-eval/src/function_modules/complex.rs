use crate::functions::FunctionRegistry;
use achronyme_types::value::Value;

pub fn register_functions(registry: &mut FunctionRegistry) {
    registry.register("complex", complex, 2);
    registry.register("real", real, 1);
    registry.register("imag", imag, 1);
    registry.register("conj", conj, 1);
    registry.register("arg", arg, 1);
}

// Implementations

fn complex(args: &[Value]) -> Result<Value, String> {
    match (&args[0], &args[1]) {
        (Value::Number(re), Value::Number(im)) => {
            Ok(Value::Complex(achronyme_types::complex::Complex::new(*re, *im)))
        }
        _ => Err("complex() requires two numbers (real, imaginary)".to_string()),
    }
}

fn real(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Number(x) => Ok(Value::Number(*x)),
        Value::Complex(c) => Ok(Value::Number(c.re)),
        _ => Err("real() requires a number or complex number".to_string()),
    }
}

fn imag(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Number(_) => Ok(Value::Number(0.0)),
        Value::Complex(c) => Ok(Value::Number(c.im)),
        _ => Err("imag() requires a number or complex number".to_string()),
    }
}

fn conj(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Number(x) => Ok(Value::Number(*x)),
        Value::Complex(c) => Ok(Value::Complex(c.conjugate())),
        _ => Err("conj() requires a number or complex number".to_string()),
    }
}

fn arg(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Number(x) => Ok(Value::Number(if *x >= 0.0 { 0.0 } else { std::f64::consts::PI })),
        Value::Complex(c) => Ok(Value::Number(c.im.atan2(c.re))),
        _ => Err("arg() requires a number or complex number".to_string()),
    }
}
