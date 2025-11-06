use crate::functions::{BuiltinFunction, FunctionRegistry};
use achronyme_types::value::Value;
use achronyme_types::vector::Vector;

pub fn register_functions(registry: &mut FunctionRegistry) {
    registry.register("sum", sum, 1);
    registry.register("mean", mean, 1);
    registry.register("std", std, 1);
}

// Implementations

fn sum(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Vector(v) => {
            let total: f64 = v.data().iter().sum();
            Ok(Value::Number(total))
        }
        _ => Err("sum() requires a vector".to_string()),
    }
}

fn mean(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Vector(v) => {
            if v.len() == 0 {
                return Err("mean() requires non-empty vector".to_string());
            }
            let total: f64 = v.data().iter().sum();
            Ok(Value::Number(total / v.len() as f64))
        }
        _ => Err("mean() requires a vector".to_string()),
    }
}

fn std(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Vector(v) => {
            if v.len() < 2 {
                return Err("std() requires vector with at least 2 elements".to_string());
            }
            let n = v.len() as f64;
            let mean_val: f64 = v.data().iter().sum::<f64>() / n;
            let variance: f64 = v.data().iter()
                .map(|x| (x - mean_val).powi(2))
                .sum::<f64>() / (n - 1.0);  // Sample standard deviation
            Ok(Value::Number(variance.sqrt()))
        }
        _ => Err("std() requires a vector".to_string()),
    }
}
