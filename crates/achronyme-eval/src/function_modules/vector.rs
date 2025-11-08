use crate::functions::FunctionRegistry;
use achronyme_types::value::Value;

pub fn register_functions(registry: &mut FunctionRegistry) {
    registry.register("dot", dot, 2);
    registry.register("cross", cross, 2);
    registry.register("norm", norm, 1);
    registry.register("normalize", normalize, 1);
}

// Implementations

fn dot(args: &[Value]) -> Result<Value, String> {
    match (&args[0], &args[1]) {
        (Value::Vector(vec1), Value::Vector(vec2)) => {
            if !Value::is_numeric_vector(vec1) || !Value::is_numeric_vector(vec2) {
                return Err("dot() requires numeric vectors".to_string());
            }

            let has_complex = vec1.iter().any(|v| matches!(v, Value::Complex(_))) || vec2.iter().any(|v| matches!(v, Value::Complex(_)));

            if has_complex {
                let v1 = Value::to_complex_vector(vec1).map_err(|e| e.to_string())?;
                let v2 = Value::to_complex_vector(vec2).map_err(|e| e.to_string())?;
                 if v1.len() != v2.len() {
                    return Err(format!("dot() requires vectors of same length ({} vs {})", v1.len(), v2.len()));
                }
                let result = v1.dot(&v2).map_err(|e| e.to_string())?;
                Ok(Value::Complex(result))
            } else {
                let v1 = Value::to_real_vector(vec1).map_err(|e| e.to_string())?;
                let v2 = Value::to_real_vector(vec2).map_err(|e| e.to_string())?;
                 if v1.len() != v2.len() {
                    return Err(format!("dot() requires vectors of same length ({} vs {})", v1.len(), v2.len()));
                }
                let result = v1.dot(&v2).map_err(|e| e.to_string())?;
                Ok(Value::Number(result))
            }
        }
        _ => Err("dot() requires two vectors".to_string()),
    }
}

fn cross(args: &[Value]) -> Result<Value, String> {
    match (&args[0], &args[1]) {
        (Value::Vector(vec1), Value::Vector(vec2)) => {
            if !Value::is_numeric_vector(vec1) || !Value::is_numeric_vector(vec2) {
                return Err("cross() requires numeric vectors".to_string());
            }
            let v1 = Value::to_real_vector(vec1).map_err(|e| e.to_string())?;
            let v2 = Value::to_real_vector(vec2).map_err(|e| e.to_string())?;

            if v1.len() != 3 || v2.len() != 3 {
                return Err("cross() requires two 3D vectors".to_string());
            }
            let result = v1.cross(&v2).map_err(|e| e.to_string())?;
            Ok(Value::from_real_vector(result))
        }
        _ => Err("cross() requires two vectors".to_string()),
    }
}

fn norm(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Vector(vec) => {
            if !Value::is_numeric_vector(vec) {
                return Err("norm() requires a numeric vector".to_string());
            }
            if vec.iter().any(|v| matches!(v, Value::Complex(_))) {
                let v = Value::to_complex_vector(vec).map_err(|e| e.to_string())?;
                Ok(Value::Number(v.norm()))
            } else {
                let v = Value::to_real_vector(vec).map_err(|e| e.to_string())?;
                Ok(Value::Number(v.norm()))
            }
        }
        _ => Err("norm() requires a vector".to_string()),
    }
}

fn normalize(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Vector(vec) => {
            if !Value::is_numeric_vector(vec) {
                return Err("normalize() requires a numeric vector".to_string());
            }
            if vec.iter().any(|v| matches!(v, Value::Complex(_))) {
                let v = Value::to_complex_vector(vec).map_err(|e| e.to_string())?;
                let norm = v.norm();
                if norm < 1e-10 {
                    return Err("Cannot normalize a zero vector".to_string());
                }
                let result = v.normalize().map_err(|e| e.to_string())?;
                Ok(Value::from_complex_vector(result))
            } else {
                let v = Value::to_real_vector(vec).map_err(|e| e.to_string())?;
                let norm = v.norm();
                if norm < 1e-10 {
                    return Err("Cannot normalize a zero vector".to_string());
                }
                let result = v.normalize().map_err(|e| e.to_string())?;
                Ok(Value::from_real_vector(result))
            }
        }
        _ => Err("normalize() requires a vector".to_string()),
    }
}
