use achronyme_parser::ast::AstNode;
use achronyme_types::value::Value;

use crate::evaluator::Evaluator;

/// Evaluate an if expression
pub fn evaluate_if(
    evaluator: &mut Evaluator,
    condition: &AstNode,
    then_expr: &AstNode,
    else_expr: &AstNode,
) -> Result<Value, String> {
    // Evaluate condition
    let cond_val = evaluator.evaluate(condition)?;

    // Convert to boolean
    let cond_bool = value_to_bool(&cond_val)?;

    // Evaluate appropriate branch (short-circuit)
    if cond_bool {
        evaluator.evaluate(then_expr)
    } else {
        evaluator.evaluate(else_expr)
    }
}

/// Evaluate a while loop
pub fn evaluate_while(
    evaluator: &mut Evaluator,
    condition: &AstNode,
    body: &AstNode,
) -> Result<Value, String> {
    let mut last_value = Value::Number(0.0);

    loop {
        // Evaluate condition
        let cond_val = evaluator.evaluate(condition)?;
        let cond_bool = value_to_bool(&cond_val)?;

        // If condition is false, exit loop
        if !cond_bool {
            break;
        }

        // Execute body
        last_value = evaluator.evaluate(body)?;

        // Check for early return - propagate it immediately
        if matches!(last_value, Value::EarlyReturn(_)) {
            return Ok(last_value);
        }
    }

    Ok(last_value)
}

/// Evaluate a piecewise function
pub fn evaluate_piecewise(
    evaluator: &mut Evaluator,
    cases: &[(Box<AstNode>, Box<AstNode>)],
    default: &Option<Box<AstNode>>,
) -> Result<Value, String> {
    // Evaluate cases in order (short-circuit)
    for (condition, expression) in cases {
        let cond_val = evaluator.evaluate(condition)?;
        let cond_bool = value_to_bool(&cond_val)?;

        if cond_bool {
            return evaluator.evaluate(expression);
        }
    }

    // If no condition was true, evaluate default if present
    if let Some(default_expr) = default {
        return evaluator.evaluate(default_expr);
    }

    // No condition was true and no default provided
    Err("piecewise: no condition was true and no default value provided".to_string())
}

/// Helper to convert Value to bool
/// Boolean values map directly, numbers: 0 = false, != 0 = true
fn value_to_bool(value: &Value) -> Result<bool, String> {
    match value {
        Value::Boolean(b) => Ok(*b),
        Value::Number(n) => Ok(*n != 0.0),
        _ => Err(format!("Cannot convert {:?} to boolean", value)),
    }
}
