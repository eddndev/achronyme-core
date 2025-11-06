use crate::functions::{BuiltinFunction, FunctionRegistry};
use achronyme_types::value::Value;

pub fn register_functions(registry: &mut FunctionRegistry) {
    registry.register("transpose", transpose, 1);
    registry.register("det", det, 1);
    registry.register("trace", trace, 1);
}

// Implementations

fn transpose(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Matrix(m) => {
            let transposed = m.transpose();
            Ok(Value::Matrix(transposed))
        }
        _ => Err("transpose() requires a matrix".to_string()),
    }
}

fn det(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Matrix(m) => {
            let determinant = achronyme_linalg::determinant_nd(m)
                .map_err(|e| format!("Determinant failed: {}", e))?;
            Ok(Value::Number(determinant))
        }
        _ => Err("det() requires a matrix".to_string()),
    }
}

fn trace(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Matrix(m) => {
            if m.rows != m.cols {
                return Err("trace() requires a square matrix".to_string());
            }
            let mut sum = 0.0;
            for i in 0..m.rows {
                sum += m.get(i, i).map_err(|e| e.to_string())?;
            }
            Ok(Value::Number(sum))
        }
        _ => Err("trace() requires a matrix".to_string()),
    }
}
