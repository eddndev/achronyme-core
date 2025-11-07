use crate::functions::FunctionRegistry;
use achronyme_types::value::Value;
use achronyme_types::vector::Vector;
use achronyme_types::complex_vector::ComplexVector;

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
        Value::ComplexVector(cv) => {
            let real_parts: Vec<f64> = cv.data().iter().map(|c| c.re).collect();
            Ok(Value::Vector(Vector::new(real_parts)))
        }
        _ => Err("real() requires a number, complex number, or complex vector".to_string()),
    }
}

fn imag(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Number(_) => Ok(Value::Number(0.0)),
        Value::Complex(c) => Ok(Value::Number(c.im)),
        Value::ComplexVector(cv) => {
            let imag_parts: Vec<f64> = cv.data().iter().map(|c| c.im).collect();
            Ok(Value::Vector(Vector::new(imag_parts)))
        }
        _ => Err("imag() requires a number, complex number, or complex vector".to_string()),
    }
}

fn conj(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Number(x) => Ok(Value::Number(*x)),
        Value::Complex(c) => Ok(Value::Complex(c.conjugate())),
        Value::ComplexVector(cv) => {
            let conjugates: Vec<_> = cv.data().iter().map(|c| c.conjugate()).collect();
            Ok(Value::ComplexVector(ComplexVector::new(conjugates)))
        }
        _ => Err("conj() requires a number, complex number, or complex vector".to_string()),
    }
}

fn arg(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Number(x) => Ok(Value::Number(if *x >= 0.0 { 0.0 } else { std::f64::consts::PI })),
        Value::Complex(c) => Ok(Value::Number(c.im.atan2(c.re))),
        _ => Err("arg() requires a number or complex number".to_string()),
    }
}
