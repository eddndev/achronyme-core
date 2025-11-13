use crate::functions::FunctionRegistry;
use achronyme_types::value::Value;
use achronyme_types::Environment;

pub fn register_functions(registry: &mut FunctionRegistry) {
    registry.register("dot", dot, 2);
    registry.register("cross", cross, 2);
    registry.register("norm", norm, 1);
    registry.register("normalize", normalize, 1);
}

// Implementations

fn dot(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    match (&args[0], &args[1]) {
        // Tensor support (optimized path)
        (Value::Tensor(t1), Value::Tensor(t2)) => {
            if !t1.is_vector() || !t2.is_vector() {
                return Err("dot() requires rank-1 tensors (vectors)".to_string());
            }
            let result = t1.dot(t2).map_err(|e| e.to_string())?;
            Ok(Value::Number(result))
        }
        (Value::ComplexTensor(t1), Value::ComplexTensor(t2)) => {
            if !t1.is_vector() || !t2.is_vector() {
                return Err("dot() requires rank-1 tensors (vectors)".to_string());
            }
            let result = t1.dot(t2).map_err(|e| e.to_string())?;
            Ok(Value::Complex(result))
        }
        // Legacy Vector support (backward compatibility)
        (Value::Vector(vec1), Value::Vector(vec2)) => {
            if !Value::is_numeric_vector(vec1) || !Value::is_numeric_vector(vec2) {
                return Err("dot() requires numeric vectors".to_string());
            }

            let has_complex = vec1.iter().any(|v| matches!(v, Value::Complex(_))) || vec2.iter().any(|v| matches!(v, Value::Complex(_)));

            if has_complex {
                let t1 = Value::to_complex_tensor(vec1).map_err(|e| e.to_string())?;
                let t2 = Value::to_complex_tensor(vec2).map_err(|e| e.to_string())?;
                let result = t1.dot(&t2).map_err(|e| e.to_string())?;
                Ok(Value::Complex(result))
            } else {
                let t1 = Value::to_real_tensor(vec1).map_err(|e| e.to_string())?;
                let t2 = Value::to_real_tensor(vec2).map_err(|e| e.to_string())?;
                let result = t1.dot(&t2).map_err(|e| e.to_string())?;
                Ok(Value::Number(result))
            }
        }
        _ => Err("dot() requires two vectors or tensors".to_string()),
    }
}

fn cross(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    match (&args[0], &args[1]) {
        // Tensor support (optimized path)
        (Value::Tensor(t1), Value::Tensor(t2)) => {
            let result = t1.cross(t2).map_err(|e| e.to_string())?;
            Ok(Value::Tensor(result))
        }
        // Legacy Vector support (backward compatibility)
        (Value::Vector(vec1), Value::Vector(vec2)) => {
            if !Value::is_numeric_vector(vec1) || !Value::is_numeric_vector(vec2) {
                return Err("cross() requires numeric vectors".to_string());
            }
            let t1 = Value::to_real_tensor(vec1).map_err(|e| e.to_string())?;
            let t2 = Value::to_real_tensor(vec2).map_err(|e| e.to_string())?;
            let result = t1.cross(&t2).map_err(|e| e.to_string())?;
            Ok(Value::from_real_tensor(result))
        }
        _ => Err("cross() requires two vectors or tensors".to_string()),
    }
}

fn norm(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    match &args[0] {
        // Tensor support (optimized path)
        Value::Tensor(t) => {
            if !t.is_vector() {
                return Err("norm() requires a rank-1 tensor (vector)".to_string());
            }
            Ok(Value::Number(t.norm()))
        }
        Value::ComplexTensor(t) => {
            if !t.is_vector() {
                return Err("norm() requires a rank-1 tensor (vector)".to_string());
            }
            Ok(Value::Number(t.norm()))
        }
        // Legacy Vector support (backward compatibility)
        Value::Vector(vec) => {
            if !Value::is_numeric_vector(vec) {
                return Err("norm() requires a numeric vector".to_string());
            }
            if vec.iter().any(|v| matches!(v, Value::Complex(_))) {
                let t = Value::to_complex_tensor(vec).map_err(|e| e.to_string())?;
                Ok(Value::Number(t.norm()))
            } else {
                let t = Value::to_real_tensor(vec).map_err(|e| e.to_string())?;
                Ok(Value::Number(t.norm()))
            }
        }
        _ => Err("norm() requires a vector or tensor".to_string()),
    }
}

fn normalize(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    match &args[0] {
        // Tensor support (optimized path)
        Value::Tensor(t) => {
            if !t.is_vector() {
                return Err("normalize() requires a rank-1 tensor (vector)".to_string());
            }
            let result = t.normalize().map_err(|e| e.to_string())?;
            Ok(Value::Tensor(result))
        }
        Value::ComplexTensor(t) => {
            if !t.is_vector() {
                return Err("normalize() requires a rank-1 tensor (vector)".to_string());
            }
            let result = t.normalize().map_err(|e| e.to_string())?;
            Ok(Value::ComplexTensor(result))
        }
        // Legacy Vector support (backward compatibility)
        Value::Vector(vec) => {
            if !Value::is_numeric_vector(vec) {
                return Err("normalize() requires a numeric vector".to_string());
            }
            if vec.iter().any(|v| matches!(v, Value::Complex(_))) {
                let t = Value::to_complex_tensor(vec).map_err(|e| e.to_string())?;
                let result = t.normalize().map_err(|e| e.to_string())?;
                Ok(Value::from_complex_tensor(result))
            } else {
                let t = Value::to_real_tensor(vec).map_err(|e| e.to_string())?;
                let result = t.normalize().map_err(|e| e.to_string())?;
                Ok(Value::from_real_tensor(result))
            }
        }
        _ => Err("normalize() requires a vector or tensor".to_string()),
    }
}
