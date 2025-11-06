use crate::functions::FunctionRegistry;
use achronyme_types::value::Value;
use achronyme_types::vector::Vector;

pub fn register_functions(registry: &mut FunctionRegistry) {
    registry.register("dot", dot, 2);
    registry.register("cross", cross, 2);
    registry.register("norm", norm, 1);
    registry.register("normalize", normalize, 1);
}

// Implementations

fn dot(args: &[Value]) -> Result<Value, String> {
    match (&args[0], &args[1]) {
        (Value::Vector(v1), Value::Vector(v2)) => {
            if v1.len() != v2.len() {
                return Err(format!("dot() requires vectors of same length ({} vs {})", v1.len(), v2.len()));
            }
            let result: f64 = v1.data().iter().zip(v2.data().iter()).map(|(a, b)| a * b).sum();
            Ok(Value::Number(result))
        }
        _ => Err("dot() requires two vectors".to_string()),
    }
}

fn cross(args: &[Value]) -> Result<Value, String> {
    match (&args[0], &args[1]) {
        (Value::Vector(v1), Value::Vector(v2)) => {
            if v1.len() != 3 || v2.len() != 3 {
                return Err("cross() requires two 3D vectors".to_string());
            }
            let a = v1.data();
            let b = v2.data();
            let result = vec![
                a[1] * b[2] - a[2] * b[1],
                a[2] * b[0] - a[0] * b[2],
                a[0] * b[1] - a[1] * b[0],
            ];
            Ok(Value::Vector(Vector::new(result)))
        }
        _ => Err("cross() requires two vectors".to_string()),
    }
}

fn norm(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Vector(v) => {
            let sum_squares: f64 = v.data().iter().map(|x| x * x).sum();
            Ok(Value::Number(sum_squares.sqrt()))
        }
        _ => Err("norm() requires a vector".to_string()),
    }
}

fn normalize(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Vector(v) => {
            let sum_squares: f64 = v.data().iter().map(|x| x * x).sum();
            let magnitude = sum_squares.sqrt();
            if magnitude < 1e-10 {
                return Err("normalize() cannot normalize zero vector".to_string());
            }
            let result: Vec<f64> = v.data().iter().map(|x| x / magnitude).collect();
            Ok(Value::Vector(Vector::new(result)))
        }
        _ => Err("normalize() requires a vector".to_string()),
    }
}
