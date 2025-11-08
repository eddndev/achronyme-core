use crate::functions::FunctionRegistry;
use achronyme_types::value::Value;

pub fn register_functions(registry: &mut FunctionRegistry) {
    registry.register("sum", sum, 1);
    registry.register("mean", mean, 1);
    registry.register("std", std, 1);
}

// Implementations

fn sum(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Vector(vec) => {
            if !Value::is_numeric_vector(vec) {
                return Err("sum() requires a numeric vector".to_string());
            }
            let mut total = Value::Number(0.0);
            for val in vec {
                total = crate::handlers::binary_ops::apply(&achronyme_parser::ast::BinaryOp::Add, total, val.clone())?;
            }
            Ok(total)
        }
        _ => Err("sum() requires a vector".to_string()),
    }
}

fn mean(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Vector(vec) => {
            if vec.is_empty() {
                return Err("mean() requires a non-empty vector".to_string());
            }
            if !Value::is_numeric_vector(vec) {
                return Err("mean() requires a numeric vector".to_string());
            }
            let sum_val = sum(args)?;
            let len_val = Value::Number(vec.len() as f64);
            crate::handlers::binary_ops::apply(&achronyme_parser::ast::BinaryOp::Divide, sum_val, len_val)
        }
        _ => Err("mean() requires a vector".to_string()),
    }
}

fn std(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Vector(vec) => {
            if vec.len() < 2 {
                return Err("std() requires a vector with at least 2 elements".to_string());
            }
            if !Value::is_numeric_vector(vec) {
                return Err("std() requires a numeric vector".to_string());
            }
            let mean_val = mean(args)?;
            let mut variance_sum = Value::Number(0.0);
            for val in vec {
                let diff = crate::handlers::binary_ops::apply(&achronyme_parser::ast::BinaryOp::Subtract, val.clone(), mean_val.clone())?;
                let squared_diff = crate::handlers::binary_ops::apply(&achronyme_parser::ast::BinaryOp::Power, diff, Value::Number(2.0))?;
                variance_sum = crate::handlers::binary_ops::apply(&achronyme_parser::ast::BinaryOp::Add, variance_sum, squared_diff)?;
            }
            let len_minus_1 = Value::Number((vec.len() - 1) as f64);
            let variance = crate::handlers::binary_ops::apply(&achronyme_parser::ast::BinaryOp::Divide, variance_sum, len_minus_1)?;

            match variance {
                Value::Number(n) => Ok(Value::Number(n.sqrt())),
                Value::Complex(c) => Ok(Value::Complex(c.sqrt())),
                _ => Err("Cannot compute sqrt of non-numeric variance".to_string())
            }
        }
        _ => Err("std() requires a vector".to_string()),
    }
}
