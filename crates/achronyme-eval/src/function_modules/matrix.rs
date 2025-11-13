use crate::functions::FunctionRegistry;
use achronyme_types::value::Value;
use achronyme_types::Environment;

pub fn register_functions(registry: &mut FunctionRegistry) {
    registry.register("transpose", transpose, 1);
    registry.register("det", det, 1);
    registry.register("trace", trace, 1);
}

// Implementations

fn transpose(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    match &args[0] {
        // Tensor support (optimized path)
        Value::Tensor(t) => {
            if !t.is_matrix() {
                return Err("transpose() requires a rank-2 tensor (matrix)".to_string());
            }
            let result = t.transpose().map_err(|e| e.to_string())?;
            Ok(Value::Tensor(result))
        }
        Value::ComplexTensor(t) => {
            if !t.is_matrix() {
                return Err("transpose() requires a rank-2 tensor (matrix)".to_string());
            }
            let result = t.transpose().map_err(|e| e.to_string())?;
            Ok(Value::ComplexTensor(result))
        }
        _ => Err("transpose() requires a tensor".to_string()),
    }
}

fn det(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    match &args[0] {
        Value::Tensor(t) => {
            if !t.is_matrix() {
                return Err("det() requires a rank-2 tensor (matrix)".to_string());
            }
            let determinant = t.determinant()
                .map_err(|e| format!("Determinant failed: {}", e))?;
            Ok(Value::Number(determinant))
        }
        Value::ComplexTensor(t) => {
            if !t.is_matrix() {
                return Err("det() requires a rank-2 tensor (matrix)".to_string());
            }
            // TODO: Implementar determinante para matrices complejas
            Err("Complex matrix determinant not yet implemented".to_string())
        }
        _ => Err("det() requires a tensor".to_string()),
    }
}

fn trace(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    match &args[0] {
        // Tensor support (optimized path)
        Value::Tensor(t) => {
            let result = t.trace().map_err(|e| e.to_string())?;
            Ok(Value::Number(result))
        }
        Value::ComplexTensor(t) => {
            let result = t.trace().map_err(|e| e.to_string())?;
            Ok(Value::Complex(result))
        }
        _ => Err("trace() requires a tensor".to_string()),
    }
}
