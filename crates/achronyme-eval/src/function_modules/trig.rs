use achronyme_types::value::Value;
use achronyme_types::vector::Vector;

use super::super::functions::{BuiltinFunction, FunctionRegistry};

/// Helper macro for unary functions that work on both scalars and vectors
macro_rules! unary_math_fn {
    ($name:expr, $f:expr, $arg:expr) => {
        match $arg {
            Value::Number(x) => Ok(Value::Number($f(*x))),
            Value::Vector(v) => {
                let result: Vec<f64> = v.data().iter().map(|&x| $f(x)).collect();
                Ok(Value::Vector(Vector::new(result)))
            }
            _ => Err(format!("{}() requires a number or vector", $name)),
        }
    };
}

/// Register all trigonometric functions
pub fn register_functions(registry: &mut FunctionRegistry) {
    // Basic trigonometric functions
    registry.register("sin", sin, 1);
    registry.register("cos", cos, 1);
    registry.register("tan", tan, 1);

    // Inverse trigonometric functions
    registry.register("asin", asin, 1);
    registry.register("acos", acos, 1);
    registry.register("atan", atan, 1);
    registry.register("atan2", atan2, 2);

    // Hyperbolic functions
    registry.register("sinh", sinh, 1);
    registry.register("cosh", cosh, 1);
    registry.register("tanh", tanh, 1);
}

// ============================================================================
// Trigonometric Function Implementations
// ============================================================================

fn sin(args: &[Value]) -> Result<Value, String> {
    unary_math_fn!("sin", f64::sin, &args[0])
}

fn cos(args: &[Value]) -> Result<Value, String> {
    unary_math_fn!("cos", f64::cos, &args[0])
}

fn tan(args: &[Value]) -> Result<Value, String> {
    unary_math_fn!("tan", f64::tan, &args[0])
}

fn asin(args: &[Value]) -> Result<Value, String> {
    unary_math_fn!("asin", f64::asin, &args[0])
}

fn acos(args: &[Value]) -> Result<Value, String> {
    unary_math_fn!("acos", f64::acos, &args[0])
}

fn atan(args: &[Value]) -> Result<Value, String> {
    unary_math_fn!("atan", f64::atan, &args[0])
}

fn atan2(args: &[Value]) -> Result<Value, String> {
    match (&args[0], &args[1]) {
        (Value::Number(y), Value::Number(x)) => Ok(Value::Number(y.atan2(*x))),
        _ => Err("atan2() requires two numbers".to_string()),
    }
}

fn sinh(args: &[Value]) -> Result<Value, String> {
    unary_math_fn!("sinh", f64::sinh, &args[0])
}

fn cosh(args: &[Value]) -> Result<Value, String> {
    unary_math_fn!("cosh", f64::cosh, &args[0])
}

fn tanh(args: &[Value]) -> Result<Value, String> {
    unary_math_fn!("tanh", f64::tanh, &args[0])
}
